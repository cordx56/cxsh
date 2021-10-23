use std::collections::HashMap;
use nix::unistd::{fork, ForkResult};
use crate::parser::{command_all_consuming, Command};
use crate::builtin::{cd};

pub struct Executor {
    builtin_commands: HashMap<String, fn(&[String]) -> Result<(), String>>,
}

impl Executor {
    pub fn new() -> Self {
        let mut builtin_commands: HashMap<String, fn(&[String]) -> Result<(), String>> = HashMap::new();
        builtin_commands.insert("cd".to_owned(), cd);
        Executor {
            builtin_commands: builtin_commands,
        }
    }

    pub fn execute(&self, command: &Command) -> Result<(), String> {
        match command {
            Command::Command(commands) => self.execute_command(commands),
            Command::Pipe(commands, pipe_command) => {
                Err("Not implemented".to_owned())
            }
            Command::Redirect(commands, redirect_command) => {
                Err("Not implemented".to_owned())
            }
        }
    }

    pub fn execute_command(&self, command: &[String]) -> Result<(), String> {
        if 0 < command.len() {
            if self.builtin_commands.contains_key(&command[0]) {
                self.builtin_commands[&command[0]](command)
            } else {
                self.execute_new_process(command)
            }
        } else {
            Ok(())
        }
    }

    pub fn execute_new_process(&self, command: &[String]) -> Result<(), String> {
        match unsafe { fork() } {
            Ok(ForkResult::Parent { .. }) => {
                Err("Not implemented".to_owned())
            },
            Ok(ForkResult::Child) => {
                Err("Not implemented".to_owned())
            },
            Err(_) => Err("fork failed".to_owned()),
        }
    }
}
