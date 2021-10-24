mod parser;
mod prompt;
mod execute;
mod builtin;
use std::io;
use execute::{Executor,ExecutionResult};

fn main() {
    let executor = Executor::new();
    loop {
        if let Err(e) = prompt::show_prompt() {
            println!("Prompt error: {}", e);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        match parser::command(&input) {
            Ok(parse_result) => {
                match executor.execute(&parse_result.1) {
                    Ok(result) => {
                        match result {
                            ExecutionResult::Normal => {},
                            ExecutionResult::Exit => {
                                break;
                            },
                        }
                    },
                    Err(e) => println!("Execution error: {}", e),
                }
            },
            Err(e) => println!("{}", e),
        }
    }
}
