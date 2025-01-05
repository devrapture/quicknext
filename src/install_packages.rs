use std::error::Error;

use owo_colors::OwoColorize;

use crate::{installers::installer::PackageInstallerMap, utils::Logger};


pub fn run(packages:&PackageInstallerMap,project_name:&String) -> Result<(), Box<dyn Error>> {
    Logger::info("Adding boilerplate...");
    for (k,v) in  packages {
        if v.in_use  {
            let installer_fn = &v.installer;
            installer_fn(&project_name)?;
            println!("{} {:?}",String::from("✅ Successfully setup boilerplate for").green(), k.green().bold());
        }
    }
    Ok(())
}