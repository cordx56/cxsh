use std::io;
use std::io::prelude::{Write};
use std::env;

pub fn show_prompt() -> Result<(), String> {
    match env::current_dir() {
        Ok(current_dir) => {
            print!("{} $ ", current_dir.display());
        },
        Err(e) => {
            return Err(e.to_string())
        },
    }
    io::stdout().flush().ok();
    Ok(())
}
