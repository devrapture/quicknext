use crate::{constants, logger::Logger, project_path::PathConfig};
use std::{
    env::{self},
    error::Error,
    fs,
    path::{Path, PathBuf},
};

struct StylingConfig {
    template_root: PathBuf,
    project_root: PathBuf,
}

impl StylingConfig {
    fn new(project_name: &String) -> Result<Self, Box<dyn Error>> {
        let current_dir = env::current_dir()?;
        let project_path = PathConfig::new(project_name)?;
        Ok(Self {
            template_root: current_dir.join(constants::EXTRAS_DIR),
            project_root: project_path,
        })
    }

    fn copy_file(&self,src_relative_path: &str, dest_relative_path: &str) -> Result<(), Box<dyn Error>> {
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

pub fn run(project_name:&String, is_tailwind_select:&bool) -> Result<(), Box<dyn Error>> {
    let config_styling = StylingConfig::new(project_name)?;
    if *is_tailwind_select {
        for (src,dest) in constants::TAILWIND_CONFIGS {
            config_styling.copy_file(src, dest)?;
        }
        Logger::info("✅ Successfully setup boilerplate for tailwind");   
    }else{
        config_styling.copy_file(
            "src/index.module.css",
            "src/app/index.module.css",
        )?;
    }

    Ok(())
}




