use regex::{escape, Regex};
use std::env;
use std::io;
use std::io::prelude::Write;

pub fn display_path(path: &str) -> Result<String, String> {
    match env::var("HOME") {
        Ok(home_dir) => {
            let re = Regex::new(&format!("^{}", escape(&home_dir))).unwrap();
            Ok(re.replace(path, "~").to_string())
        }
        Err(_) => Err("Environment variable HOME does not set".to_owned()),
    }
}

pub fn show_prompt() -> Result<(), String> {
    match env::current_dir() {
        Ok(current_dir_pathbuf) => match current_dir_pathbuf.into_os_string().into_string() {
            Ok(current_dir) => {
                let display_path = display_path(&current_dir)?;
                print!("{} $ ", display_path);
            }
            Err(_) => return Err("".to_owned()),
        },
        Err(e) => return Err(e.to_string()),
    }
    io::stdout().flush().ok();
    Ok(())
}
