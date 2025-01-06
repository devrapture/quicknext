use std::{collections::HashMap, error::Error};

use crate::utils::PackagesEnum;

use super::{app_router, tailwind};


pub type PackageInstallerMap = HashMap<PackagesEnum, PackageInstaller>;
pub type InstallerFn = fn(project_name:&String) -> Result<(), Box<dyn Error>>;
#[derive(Debug)]
pub struct PackageInstaller{
    pub in_use:bool,
    pub installer: InstallerFn
}

impl PackageInstaller {
    pub fn build_pkg_installer_map(packages:&Vec<PackagesEnum>) -> PackageInstallerMap{
        let mut map = HashMap::new();
        let installers:[(PackagesEnum, bool, InstallerFn); 2] = [
            (PackagesEnum::Tailwind, packages.contains(&PackagesEnum::Tailwind), tailwind::install),
            (PackagesEnum::AppRouter, packages.contains(&PackagesEnum::AppRouter), app_router::install),
        ];
        for (package, in_use, installer) in installers {
            map.insert(package, Self{
                in_use,
                installer:installer
            });
        }
        map

    }
}