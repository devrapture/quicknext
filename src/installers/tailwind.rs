use crate::{
    constants,
    utils::{add_package_dependency, PackageJson, PathConfig},
};
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
    pub fn new(project_name: &String) -> Result<Self, Box<dyn Error>> {
        let current_dir = env::current_dir()?;
        let project_path = PathConfig::new(&project_name)?;
        Ok(Self {
            template_root: current_dir.join(constants::EXTRAS_TEMPLATE_DIR),
            project_root: project_path,
        })
    }
    pub fn add_package_dependency(&self) -> Result<(), Box<dyn Error>> {
        let dependencies = vec![
            "tailwindcss",
            "postcss",
            "prettier",
            "prettier-plugin-tailwindcss",
        ];
        add_package_dependency::add_package_dependency(&dependencies, true, &self.project_root)?;
        Ok(())
    }

    pub fn add_format_scripts(&self) -> Result<(), Box<dyn Error>> {
        let package_json_path = self.project_root.join("package.json");
        let content = fs::read_to_string(&package_json_path)?;
        let mut package_json: PackageJson = serde_json::from_str(&content)?;
        package_json.scripts.insert(
            "format:write".to_string(),
            r#"prettier --write "**/*.{ts,tsx,js,jsx,mdx}" --cache"#.to_string(),
        );
        package_json.scripts.insert(
            "format:check".to_string(),
            r#"prettier --check "**/*.{ts,tsx,js,jsx,mdx}" --cache"#.to_string(),
        );
        fs::write(
            &package_json_path,
            serde_json::to_string_pretty(&package_json)?,
        )?;
        Ok(())
    }

    pub fn copy_file(
        &self,
        src_relative_path: &str,
        dest_relative_path: &str,
    ) -> Result<(), Box<dyn Error>> {
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

pub fn install(project_name: &String) -> Result<(), Box<dyn Error>> {
    let config_styling = TailwindConfig::new(project_name)?;
    config_styling.add_package_dependency()?;
    config_styling.add_format_scripts()?;
    for (src, dest) in constants::TAILWIND_CONFIGS {
        config_styling.copy_file(src, dest)?;
    }

    Ok(())
}
