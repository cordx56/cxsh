use std::env;

pub fn show_prompt() {
    print!("{} $ ", env::current_dir().unwrap().display());
}
