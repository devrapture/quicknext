use std::{env, error::Error, path::PathBuf};

use crate::{constants, installers::installer::PackageInstallerMap};

use super::{copy_file, PackagesEnum};

struct FileConfig {
    template_dir: PathBuf,
    dest_path: PathBuf,
    file_name: String,
}

impl FileConfig {
    fn new(
        project_dir: &PathBuf,
        template_path: &str,
        dest_path: &str,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            template_dir: env::current_dir()?.join(template_path),
            dest_path: project_dir.join(dest_path),
            file_name: String::from("base.tsx"),
        })
    }

    // for select layout file
    fn determine_file_name_layout(&mut self, packages: &PackageInstallerMap) {
        let using_tailwind = packages.get(&PackagesEnum::Tailwind).unwrap().in_use;
        let using_trpc = packages.get(&PackagesEnum::Trpc).unwrap().in_use;

        self.file_name = match (using_tailwind, using_trpc) {
            (true, true) => "with-trpc-tw.tsx",
            (true, false) => "with-trpc.tsx",
            (false, true) => "with-tw.tsx",
            (false, false) => "base.tsx",
        }
        .to_owned();
    }

    fn determine_file_name(&mut self, packages: &PackageInstallerMap) {
        let using_tailwind = packages.get(&PackagesEnum::Tailwind).unwrap().in_use;
        let using_trpc = packages.get(&PackagesEnum::Trpc).unwrap().in_use;
        let using_auth = packages.get(&PackagesEnum::NextAuth).unwrap().in_use;

        self.file_name = match (using_trpc, using_tailwind, using_auth) {
            (true, true, true) => "with-auth-trpc-tw.tsx",
            (true, false, true) => "with-auth-trpc.tsx",
            (true, true, false) => "with-trpc-tw.tsx",
            (true, false, false) => "with-trpc.tsx",
            (false, true, false) => "with-tw.tsx",
            _ => "base.tsx",
        }
        .to_owned();
    }

    fn copy(&self) -> Result<(), Box<dyn Error>> {
        copy_file::run(&self.template_dir.join(&self.file_name), &self.dest_path)?;
        Ok(())
    }
}

pub fn select_layout_file( project_dir: &PathBuf,packages: &PackageInstallerMap) -> Result<(), Box<dyn Error>> {
    let mut file_config = FileConfig::new(project_dir, constants::LAYOUT_FILE_TEMPLATE_DIR, "src/app/layout.tsx")?;
    file_config.determine_file_name_layout(&packages);
    file_config.copy()?;
    Ok(())
}

pub fn select_page_file(project_dir: &PathBuf,packages: &PackageInstallerMap) -> Result<(), Box<dyn Error>> {
    let mut file_config = FileConfig::new(project_dir, "template/extras/src/pages/index", "src/app/page.tsx")?;
    file_config.determine_file_name(packages);
    file_config.copy()?;
    Ok(())
}

pub fn select_index_file( project_dir: &PathBuf,packages: &PackageInstallerMap) -> Result<(), Box<dyn Error>> {
    let mut file_config = FileConfig::new(project_dir, "template/extras/src/pages/index", "src/pages/index.tsx")?;
    file_config.determine_file_name(packages);
    file_config.copy()?;
    Ok(())
}

