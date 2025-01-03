use regex::Regex;

use std::error::Error;

use console::Style;
use dialoguer::{theme::ColorfulTheme, Input, Select};

const YES_NO_OPTIONS: [&str; 2] = ["Yes", "No"];
const APP_NAME_PATTERN: &str = r"^[a-z0-9_-]+$|^\.$";
const DEFAULT_IMPORT_ALIAS: &str = "~/";

#[derive(Debug)]
pub struct Config {
    pub project_name: String,
    pub styling_with_tailwind: bool,
    pub set_up_shadcn: bool,
    pub initialize_git: bool,
    pub import_alias: String,
}

impl Config {
    pub fn run() -> Result<Config, Box<dyn Error>> {
        let theme = Self::create_theme();
        let project_name = Self::get_project_name(&theme)?;
        let styling_with_tailwind =
            Self::prompt_yes_no(&theme, "Will you be using Tailwind CSS for styling?")?;
        let set_up_shadcn = if styling_with_tailwind {
            Self::prompt_yes_no(&theme, "Will you be using Shadcn?")?
        } else {
            false
        };
        let initialize_git = Self::prompt_yes_no(
            &theme,
            "Should we initialize a Git repository and stage the changes?",
        )?;

        let import_alias = Self::get_import_alias(&theme)?;
        Ok(Config {
            project_name,
            styling_with_tailwind,
            initialize_git,
            set_up_shadcn,
            import_alias,
        })
    }

    fn create_theme() -> ColorfulTheme {
        ColorfulTheme {
            values_style: Style::new().cyan().dim(),
            ..ColorfulTheme::default()
        }
    }

    fn get_project_name(theme: &ColorfulTheme) -> Result<String, Box<dyn Error>> {
        Input::with_theme(theme)
                    .with_prompt("What's the name of your project? \n")
                    .validate_with(|input: &String| -> Result<(), &str> {
                        if is_valid_app_name(input) {
                            Ok(())
                        }else {
                            Err("App name must consist of only lowercase alphanumeric characters, '-', and '_'")
                        }
                    })
                    .interact_text().map_err(Into::into)
    }

    fn prompt_yes_no(theme: &ColorfulTheme, prompt: &str) -> Result<bool, Box<dyn Error>> {
        Ok(Select::with_theme(theme)
            .with_prompt(prompt)
            .items(&YES_NO_OPTIONS)
            .default(0) // Default to the first option ("Yes")
            .interact()?
            == 0)
    }

    fn get_import_alias(theme: &ColorfulTheme) -> Result<String, Box<dyn Error>> {
        Input::with_theme(theme)
            .with_prompt(" What import alias would you like to use? \n")
            .with_initial_text(DEFAULT_IMPORT_ALIAS)
            .interact_text()
            .map_err(Into::into)
    }
}

fn is_valid_app_name(app_name: &str) -> bool {
    let re = Regex::new(APP_NAME_PATTERN).unwrap();
    re.is_match(app_name)
}
