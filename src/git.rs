use std::process::Command;

use crate::utils::Logger;

pub fn initialize_git() {
    Logger::info("Initializing Git...");
    // println!();
    let output = Command::new("git").arg("version").status().is_ok();
    println!("{:?}", output);
}
