use std::{env, error::Error};

use owo_colors::OwoColorize;

use crate::{
    constants,
    installers::installer::PackageInstallerMap,
    utils::{
        copy_file,
        select_boiler_plate::{
            select_app_file, select_index_file, select_layout_file, select_page_file,
        },
        Logger, PackagesEnum, PathConfig,
    },
};

pub fn run(packages: &PackageInstallerMap, project_name: &String) -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let project_path = PathConfig::new(project_name)?;
    Logger::info("Adding boilerplate...");
    for (k, v) in packages {
        if v.in_use {
            let installer_fn = &v.installer;
            installer_fn(&project_name)?;
            println!(
                "{} {:?}",
                String::from("✅ Successfully setup boilerplate for").green(),
                k.green().bold()
            );
        }
    }

    // If no tailwind, select use css modules
    if !packages.get(&PackagesEnum::Tailwind).unwrap().in_use {
        let index_module_css = current_dir.join(constants::INDEX_MODULE_CSS_TEMPLATE_DIR);
        let index_module_css_dest = project_path
            .join("src")
            .join(if packages.get(&PackagesEnum::AppRouter).unwrap().in_use {
                "app"
            } else {
                "pages"
            })
            .join("index.module.css");
        copy_file::run(&index_module_css, &index_module_css_dest)?;
        // if let Some(parent) =  index_module_css_dest.parent(){
        //     fs::create_dir_all(parent)?;
        // }
        // fs::copy(&index_module_css, &index_module_css_dest)?;
    }

    // Select necessary _app,index / layout,page files
    if packages.get(&PackagesEnum::AppRouter).unwrap().in_use {
        select_layout_file(&project_path, &packages)?;
        select_page_file(&project_path, &packages)?;
    } else {
        select_app_file(&project_path, &packages)?;
        select_index_file(&project_path, &packages)?;
    }

    Ok(())
}
