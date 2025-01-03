use std::{env, io::Result, path::PathBuf};

pub fn new(project_name: &str) -> Result<PathBuf> {
    let path = env::current_dir()?;
    println!("{project_name}");
    println!("path:{}", path.display());
    let new_path = path.join(project_name);
    Ok(new_path)
}
