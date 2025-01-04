use std::{error::Error, process};

mod add_styling;
mod banner;
mod cli;
mod constants;
mod create_project;
mod git;
pub mod logger;
mod project_path;
mod scafold_project;

use cli::Config;
use logger::Logger;

type AppResult<T> = Result<T, Box<dyn Error>>;

struct App {
    config: Config,
}

impl App {
    fn new() -> AppResult<Self> {
        banner::display_banner();
        let config = Config::run()?;
        Ok(Self { config })
    }

    fn run(&self) -> AppResult<()> {
        self.scaffold_project()?;
        Logger::info("Adding boilerplate...");
            self.add_styling()?;
        if self.config.initialize_git {
            git::initialize_git();
        }

        Ok(())
    }

    fn scaffold_project(&self) -> AppResult<()> {
        scafold_project::run(&self.config.project_name)
    }

    fn add_styling(&self) -> AppResult<()> {
        add_styling::run(&self.config.project_name,&self.config.styling_with_tailwind)
    }
}

fn main() {
    if let Err(e) = try_main() {
        Logger::error(&format!("Error: {e:?}"));
        process::exit(1);
    }
}

fn try_main() -> AppResult<()> {
    let app = App::new()?;
    app.run()
}
