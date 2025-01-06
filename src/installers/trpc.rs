// TODO fix
use std::{env, error::Error, fs, path::PathBuf};

use crate::{constants, utils::PathConfig};

pub struct TrpcConfig {
    pub template_root: PathBuf,
    pub project_root: PathBuf,
}

impl TrpcConfig {
    pub fn new(project_name:&String) -> Result<Self, Box<dyn Error>> {
        let current_dir = env::current_dir()?;
        let project_path = PathConfig::new(&project_name)?;
        Ok(Self {
            template_root: current_dir.join(constants::APP_ROUTER_TEMPLATE_DIR),
            project_root: project_path.join("next.config.js"),
        })
    }

    pub fn copy_file(&self,src_relative_path: &PathBuf, dest_relative_path: &PathBuf) -> Result<(), Box<dyn Error>> {
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
println!("{}",project_name);
    Ok(())
}