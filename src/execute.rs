use crate::builtin::cd;
use crate::parser::{command_all_consuming, Command};
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::{close, dup2, execvp, fork, pipe, ForkResult};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::unix::io::RawFd;
use std::process::exit;

pub enum ExecutionResult {
    Normal(Option<i32>),
    Exit,
}

pub struct Executor {
    builtin_commands: HashMap<String, fn(&[String]) -> Result<ExecutionResult, String>>,
    pipes: Vec<(RawFd, RawFd)>,
}

impl Executor {
    pub fn new() -> Self {
        let mut builtin_commands: HashMap<
            String,
            fn(&[String]) -> Result<ExecutionResult, String>,
        > = HashMap::new();
        builtin_commands.insert("cd".to_owned(), cd);
        builtin_commands.insert("help".to_owned(), |_| {
            crate::messages::help();
            Ok(ExecutionResult::Normal(None))
        });
        builtin_commands.insert("exit".to_owned(), |_| Ok(ExecutionResult::Exit));
        Executor {
            builtin_commands: builtin_commands,
            pipes: Vec::new(),
        }
    }

    pub fn execute(&mut self, command: &Command) -> Result<ExecutionResult, String> {
        match command {
            Command::Command(commands) => self.execute_command(commands, true),
            Command::Pipe(commands, pipe_command) => match pipe() {
                Ok(pipe_fds) => {
                    self.pipes.push(pipe_fds);
                    self.execute_command(commands, false);
                    let result = self.execute(pipe_command);
                    self.pipes.pop();
                    result
                }
                Err(_) => return Err("pipe failed".to_owned()),
            },
            Command::Redirect(commands, redirect_command) => Err("Not implemented".to_owned()),
        }
    }

    pub fn execute_command(&self, command: &[String], pipe_end: bool) -> Result<ExecutionResult, String> {
        if 0 < command.len() {
            if self.builtin_commands.contains_key(&command[0]) {
                self.builtin_commands[&command[0]](command)
            } else {
                self.execute_new_process(command, pipe_end)
            }
        } else {
            Ok(ExecutionResult::Normal(None))
        }
    }

    pub fn execute_new_process(&self, command: &[String], pipe_end: bool) -> Result<ExecutionResult, String> {
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => {
                if pipe_end && 0 < self.pipes.len() {
                    let pipe = self.pipes[self.pipes.len() - 1];
                    if let Err(_) = close(pipe.0) {
                        return Err("pipe close failed".to_owned());
                    }
                    if let Err(_) = close(pipe.1) {
                        return Err("pipe close failed".to_owned());
                    }
                } else if 1 < self.pipes.len() {
                    let pipe = self.pipes[self.pipes.len() - 2];
                    if let Err(_) = close(pipe.0) {
                        return Err("pipe close failed".to_owned());
                    }
                    if let Err(_) = close(pipe.1) {
                        return Err("pipe close failed".to_owned());
                    }
                }
                match waitpid(child, Some(WaitPidFlag::WUNTRACED)) {
                    Ok(status) => match status {
                        WaitStatus::Exited(_, status) => Ok(ExecutionResult::Normal(Some(status))),
                        WaitStatus::Stopped(_, _) => Ok(ExecutionResult::Normal(None)),
                        WaitStatus::Signaled(_, _, _) => Ok(ExecutionResult::Normal(None)),
                        _ => Ok(ExecutionResult::Normal(None)),
                    },
                    Err(_) => Err("Process wait error".to_owned()),
                }
            }
            Ok(ForkResult::Child) => {
                if !pipe_end && 0 < self.pipes.len() {
                    let pipe = self.pipes[self.pipes.len() - 1];
                    if let Err(_) = close(pipe.0) {
                        return Err("pipe close failed".to_owned());
                    }
                    if let Err(_) = dup2(pipe.1, 1) {
                        return Err("dup2 failed".to_owned());
                    }
                    if let Err(_) = close(pipe.1) {
                        return Err("pipe close failed".to_owned());
                    }
                }
                if pipe_end && 0 < self.pipes.len() {
                    let pipe = self.pipes[self.pipes.len() - 1];
                    if let Err(_) = close(pipe.1) {
                        return Err("pipe close failed".to_owned());
                    }
                    if let Err(_) = dup2(pipe.0, 0) {
                        return Err("dup2 failed".to_owned());
                    }
                    if let Err(_) = close(pipe.0) {
                        return Err("pipe close failed".to_owned());
                    }
                }
                if !pipe_end && 1 < self.pipes.len() {
                    let pipe = self.pipes[self.pipes.len() - 2];
                    if let Err(_) = close(pipe.1) {
                        return Err("pipe close failed".to_owned());
                    }
                    if let Err(_) = dup2(pipe.0, 0) {
                        return Err("dup2 failed".to_owned());
                    }
                    if let Err(_) = close(pipe.0) {
                        return Err("pipe close failed".to_owned());
                    }
                }

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
                            eprintln!("cxsh: command not found: {}", command[0]);
                            exit(127);
                        }
                    }
                }
            }
            Err(_) => Err("fork failed".to_owned()),
        }
    }
}
