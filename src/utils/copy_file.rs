use std::{error::Error, fs, path::PathBuf};

pub fn run(src: &PathBuf, dest: &PathBuf) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(&src, &dest)?;

    Ok(())
}
