
use crate::{constants, utils::PathConfig};
use std::{
    env::{self},
    error::Error,
    fs,
    path::PathBuf,
};

pub struct TailwindConfig {
    pub template_root: PathBuf,
    pub project_root: PathBuf,
}

impl TailwindConfig {
    pub fn new(project_name:&String) -> Result<Self, Box<dyn Error>> {
        let current_dir = env::current_dir()?;
        let project_path = PathConfig::new(&project_name)?;
        Ok(Self {
            template_root: current_dir.join(constants::EXTRAS_TEMPLATE_DIR),
            project_root: project_path,
        })
    }

    pub fn copy_file(&self,src_relative_path: &str, dest_relative_path: &str) -> Result<(), Box<dyn Error>> {
        let source = self.template_root.join(src_relative_path);
        if !source.exists() {
            return Err("Source doesn't exist".into());
        }
        let destination = self.project_root.join(dest_relative_path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&source, &destination)?;
        Ok(())
    }
}

pub fn install(project_name:&String) -> Result<(), Box<dyn Error>> {
    let config_styling = TailwindConfig::new(project_name )?;
        for (src,dest) in constants::TAILWIND_CONFIGS {
            config_styling.copy_file(src, dest)?;
        }
    Ok(())
}
