use std::env;
use crate::execute::ExecutionResult;

pub fn cd(command: &[String]) -> Result<ExecutionResult, String> {
    if command.len() < 2 {
        match env::var("HOME") {
            Ok(val) => match env::set_current_dir(&val) {
                Ok(_) => Ok(ExecutionResult::Normal),
                Err(_) => Err(format!("cd: The directory {} does not exist", val)),
            },
            Err(_) => Err("Environment variable HOME does not set".to_owned()),
        }
    } else if command.len() == 2 {
        match env::set_current_dir(&command[1]) {
            Ok(_) => Ok(ExecutionResult::Normal),
            Err(_) => Err(format!("cd: The directory {} does not exist", command[1])),
        }
    } else {
        Err("cd: cd [DIRECTORY]".to_owned())
    }
}
