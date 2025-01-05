use std::{env, error::Error, path::PathBuf};

pub struct PathConfig;

impl PathConfig {
    pub fn new(app_name: &String) -> Result<PathBuf, Box<dyn Error>> {
        let current_dir = env::current_dir()?;
        let path = match app_name.as_str() {
            "." => &current_dir,
            _ => &current_dir.join(&app_name),
        };
        Ok(path.to_path_buf())
    }
}
