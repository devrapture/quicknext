use console::Style;
use std::{
    env::{self},
    error::Error,
    fs,
    path::{Path, PathBuf},
    process,
};

use crate::{
    constants,
    utils::{Logger, PathConfig},
};
use dialoguer::{theme::ColorfulTheme, Select};
use owo_colors::OwoColorize;

struct ProjectConfig {
    path: PathBuf,
    name: String,
    theme: ColorfulTheme,
    template_dir: PathBuf,
}

impl ProjectConfig {
    fn new(app_name: &String) -> Result<Self, Box<dyn Error>> {
        let current_dir = env::current_dir()?;
        let project_path = PathConfig::new(app_name)?;
        let name = project_path
            .file_name()
            .map_or(String::from(app_name), |n| n.to_string_lossy().to_string());
        let theme = ColorfulTheme {
            values_style: Style::new().cyan().dim(),
            ..ColorfulTheme::default()
        };

        let template_dir = current_dir.join(constants::TEMPLATE_DIR);
        Ok(Self {
            path: project_path.to_path_buf(),
            name,
            theme,
            template_dir,
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
            .items(&constants::OVERWRITE_OPTIONS)
            .default(0)
            .interact()
            .map_err(Into::into)
    }

    fn handle_clear_directory(&self) -> Result<(), Box<dyn Error>> {
        println!("Clear the directory and continue installation");
        match Select::with_theme(&self.theme)
            .with_prompt("Are you sure you want to clear the directory?")
            .items(&constants::CONFIRM_OPTIONS)
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

    fn copy_directory(&self, source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
        if !destination.try_exists().unwrap() {
            fs::create_dir(destination)?
        }
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let entry_path = entry.path();
            let dist_path = destination.join(entry.file_name());

            if entry_path.is_dir() {
                self.copy_directory(&entry_path, &dist_path)?;
            } else {
                fs::copy(&entry_path, &dist_path)?;
            }
        }
        Ok(())
    }
    fn rename_gitignore_file(&self, old: &str, new: &str) -> Result<(), Box<dyn Error>> {
        let old_file = self.path.join(old);
        let new_file = self.path.join(new);
        fs::rename(&old_file, &new_file)?;
        Ok(())
    }
}

pub fn run(app_name: &String) -> Result<(), Box<dyn Error>> {
    let config = ProjectConfig::new(app_name)?;
    Logger::info(format!("Scafolding into {:?}", config.path).as_str());
    config.handle_existing_directory()?;
    config.copy_directory(&config.template_dir, &config.path)?;
    config.rename_gitignore_file("_gitignore",".gitignore")?;
    println!(
        "{} {}",
        config.name.cyan().bold(),
        String::from("scaffolded successfully!").green()
    );
    Ok(())
}
