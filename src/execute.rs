use crate::builtin::cd;
use crate::parser::{command_all_consuming, Command};
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::{execvp, fork, ForkResult};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::process::exit;

pub enum ExecutionResult {
    Normal,
    Exit,
}

pub struct Executor {
    builtin_commands: HashMap<String, fn(&[String]) -> Result<ExecutionResult, String>>,
}

impl Executor {
    pub fn new() -> Self {
        let mut builtin_commands: HashMap<
            String,
            fn(&[String]) -> Result<ExecutionResult, String>,
        > = HashMap::new();
        builtin_commands.insert("cd".to_owned(), cd);
        builtin_commands.insert("exit".to_owned(), |_| Ok(ExecutionResult::Exit));
        Executor {
            builtin_commands: builtin_commands,
        }
    }

    pub fn execute(&self, command: &Command) -> Result<ExecutionResult, String> {
        match command {
            Command::Command(commands) => self.execute_command(commands),
            Command::Pipe(commands, pipe_command) => Err("Not implemented".to_owned()),
            Command::Redirect(commands, redirect_command) => Err("Not implemented".to_owned()),
        }
    }

    pub fn execute_command(&self, command: &[String]) -> Result<ExecutionResult, String> {
        if 0 < command.len() {
            if self.builtin_commands.contains_key(&command[0]) {
                self.builtin_commands[&command[0]](command)
            } else {
                self.execute_new_process(command)
            }
        } else {
            Ok(ExecutionResult::Normal)
        }
    }

    pub fn execute_new_process(&self, command: &[String]) -> Result<ExecutionResult, String> {
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => {
                match waitpid(child, Some(WaitPidFlag::WUNTRACED)) {
                    Ok(status) => match status {
                        WaitStatus::Exited(_, _) => Ok(ExecutionResult::Normal),
                        WaitStatus::Stopped(_, _) => Ok(ExecutionResult::Normal),
                        WaitStatus::Signaled(_, _, _) => Ok(ExecutionResult::Normal),
                        _ => Ok(ExecutionResult::Normal),
                    },
                    Err(_) => Err("Process wait error".to_owned()),
                }
            }
            Ok(ForkResult::Child) => {
                let mut args = Vec::new();
                for arg_string in command {
                    let string: &str = &arg_string;
                    match CString::new(string) {
                        Ok(cstring) => args.push(cstring),
                        Err(_) => {
                            println!("String to CStr failed");
                            exit(-1);
                        }
                    }
                }

                unsafe {
                    let string: &str = &command[0];
                    let cstring = CString::new(string).unwrap();
                    let cstr = CStr::from_bytes_with_nul_unchecked(cstring.to_bytes_with_nul());
                    match execvp(cstr, &args) {
                        Ok(_) => {
                            exit(0);
                        }
                        Err(_) => {
                            println!("cxsh: command not found: {}", command[0]);
                            exit(-1);
                        }
                    }
                }
            }
            Err(_) => Err("fork failed".to_owned()),
        }
    }
}
