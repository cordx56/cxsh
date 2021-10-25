mod parser;
mod prompt;
mod execute;
mod input;
mod builtin;
use std::io;
use execute::{Executor,ExecutionResult};

fn main() {
    let executor = Executor::new();
    loop {
        if let Err(e) = prompt::show_prompt() {
            println!("Prompt error: {}", e);
        }
        let mut termio = input::TermIO::new();
        let input = termio.read();
        match parser::command(&input) {
            Ok(parse_result) => {
                match executor.execute(&parse_result.1) {
                    Ok(result) => {
                        match result {
                            ExecutionResult::Normal(_) => {},
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
