mod parser;
mod prompt;
mod execute;
mod builtin;
use std::io;
use std::io::prelude::{Write};
use execute::Executor;

fn main() {
    let mut exit = false;
    let executor = Executor::new();
    while !exit {
        prompt::show_prompt();
        let mut input = String::new();
        io::stdout().flush().ok();
        io::stdin().read_line(&mut input).ok();
        match parser::command(&input) {
            Ok(parse_result) => {
                executor.execute(&parse_result.1).ok();
            },
            Err(e) => println!("{}", e),
        }
    }
}
