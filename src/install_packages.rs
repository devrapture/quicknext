use std::{env, error::Error, fs};

use owo_colors::OwoColorize;

use crate::{
    constants,
    installers::installer::PackageInstallerMap,
    utils::{Logger, PackagesEnum, PathConfig},
};

pub fn run(packages: &PackageInstallerMap, project_name: &String) -> Result<(), Box<dyn Error>> {
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
        let current_dir = env::current_dir()?;
        let project_path = PathConfig::new(project_name)?;
        let index_module_css = current_dir.join(constants::INDEX_MODULE_CSS_DIR);
        let index_module_css_dest =  project_path.join("src").join(
            if packages.get(&PackagesEnum::AppRouter).unwrap().in_use {
                "app"
            }else{
                "pages"
            }
        ).join("index.module.css");
        if let Some(parent) =  index_module_css_dest.parent(){
            fs::create_dir_all(parent)?;
        }
        fs::copy(&index_module_css, &index_module_css_dest)?;
    }

    Ok(())
}


// if (!packages.tailwind.inUse) {
//     const indexModuleCss = path.join(
//       PKG_ROOT,
//       "template/extras/src/index.module.css"
//     );
//     const indexModuleCssDest = path.join(
//       projectDir,
//       "src",
//       appRouter ? "app" : "pages",
//       "index.module.css"
//     );
//     fs.copyFileSync(indexModuleCss, indexModuleCssDest);
//   }