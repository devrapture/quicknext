use std::{env, error::Error, fs};

use super::PackageJson;

pub fn rename_project(app_name: &String) -> Result<(), Box<dyn Error>> {
    let project_dir = env::current_dir()?.join(&app_name);
    let package_json_path = project_dir.join("package.json");
    let content = fs::read_to_string(&package_json_path)?;
    let mut package_json: PackageJson = serde_json::from_str(&content)?;
    package_json.name = app_name.to_string();
    fs::write(
        &package_json_path,
        serde_json::to_string_pretty(&package_json)?,
    )?;
    Ok(())
}
