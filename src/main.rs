use std::{error::Error, process};

mod banner;
mod cli;
mod constants;
mod git;
mod install_packages;
pub mod installers;
mod scafold_project;
pub mod utils;

use cli::Config;
use installers::installer::PackageInstaller;
use utils::{rename_project, Logger};

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
        let use_packages = PackageInstaller::build_pkg_installer_map(&self.config.packages);
        self.scaffold_project()?;
        rename_project(&self.config.project_name)?;
        install_packages::run(&use_packages, &self.config.project_name)?;
        // self.add_styling()?;
        if self.config.initialize_git {
            git::initialize_git();
        }

        Ok(())
    }

    fn scaffold_project(&self) -> AppResult<()> {
        scafold_project::run(&self.config.project_name)
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
