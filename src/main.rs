mod parser;
mod prompt;
mod execute;
mod termio;
mod builtin;
mod messages;
use execute::{Executor,ExecutionResult};

fn main() {
    let executor = Executor::new();

    messages::welcome_message();
    loop {
        if let Err(e) = prompt::show_prompt() {
            println!("Prompt error: {}", e);
        }
        let input = termio::read();
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
