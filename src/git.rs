use crate::logger::Logger;
use std::process::Command;

pub fn initialize_git() {
    Logger::info("Initializing Git...");
    // println!();
    let output = Command::new("git").arg("version").status().is_ok();
    println!("{:?}", output);
}
