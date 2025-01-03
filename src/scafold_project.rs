use console::Style;
use std::{
    env::{self},
    error::Error,
    fs,
    path::PathBuf,
    process,
};

use dialoguer::{theme::ColorfulTheme, Select};
use owo_colors::OwoColorize;

const OVERWRITE_OPTIONS: [&str; 2] = ["abort", "clear"];
const CONFIRM_OPTIONS: [&str; 2] = ["Yes", "No"];

pub struct ProjectConfig {
    pub path: PathBuf,
    pub name: String,
    pub theme: ColorfulTheme,
}

impl ProjectConfig {
    fn new(app_name: &String) -> Result<Self, Box<dyn Error>> {
        let current_dir = env::current_dir()?;
        let path = match app_name.as_str() {
            "." => &current_dir,
            _ => &current_dir.join(&app_name),
        };
        let name = path
            .file_name()
            .map_or(String::from(app_name), |n| n.to_string_lossy().to_string());
        let theme = ColorfulTheme {
            values_style: Style::new().cyan().dim(),
            ..ColorfulTheme::default()
        };
        Ok(Self {
            path: path.to_path_buf(),
            name,
            theme,
        })
    }

    fn handle_existing_directory(&self) -> Result<(), Box<dyn Error>> {
        if self.path.is_dir() {
            if fs::read_dir(&self.path)?.next().is_none() {
                if self.name != "." {
                    println!(
                        "\n{} exists but is empty, continuing...\n",
                        self.name.bright_red().bold()
                    );
                }
            } else {
                match self.prompt_overwrite_action()? {
                    0 => self.abort(),
                    1 => self.handle_clear_directory()?,
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }

    fn prompt_overwrite_action(&self) -> Result<usize, Box<dyn Error>> {
        Select::with_theme(&self.theme)
            .with_prompt(format!(
                "\n\n {} {} already exists and isn't empty. How would you like to proceed?",
                format!("{}", String::from("Warning:").red().bold()),
                &self.name.cyan().bold()
            ))
            .items(&OVERWRITE_OPTIONS)
            .default(0)
            .interact()
            .map_err(Into::into)
    }

    fn handle_clear_directory(&self) -> Result<(), Box<dyn Error>> {
        println!("Clear the directory and continue installation");
        match Select::with_theme(&self.theme)
            .with_prompt("Are you sure you want to clear the directory?")
            .items(&CONFIRM_OPTIONS)
            .default(1) // Default to the first option ("No")
            .interact()?
        {
            // user selected "yes"
            0 => {
                println!("Emptying {} and creating app..", self.name.cyan().bold());
                fs::remove_dir_all(&self.path)?;
            }
            // user selected "no"
            1 => self.abort(),
            _ => unreachable!(),
        }
        Ok(())
    }

    fn abort(&self) {
        println!("{}", "Aborting installation...".red().bold());
        process::exit(0);
    }
}

pub fn run(app_name: &String) -> Result<(), Box<dyn Error>> {
    let config = ProjectConfig::new(app_name)?;
    config.handle_existing_directory()?;
    Ok(())
}
