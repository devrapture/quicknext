use owo_colors::OwoColorize;

pub struct Logger;

impl Logger {
    pub fn info(args: &str) {
        println!("{}", args.cyan().bold());
    }
    pub fn error(args: &str) {
        eprintln!("{}", args.red().bold());
    }
}
