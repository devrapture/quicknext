use std::{error::Error, path::PathBuf};

pub struct AppRouterConfig {
    pub template_root: PathBuf,
    pub project_root: PathBuf,
}

impl AppRouterConfig {
    
}

pub fn install(project_name:&String) -> Result<(), Box<dyn Error>> {
  println!("{}",project_name);
    Ok(())
}