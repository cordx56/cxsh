use std::fmt;
use std::io;
use std::io::{stdin, stdout, Read, Stdin, Stdout, Write};
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn string_slice(s: &str, start: usize, end: usize) -> String {
    let mut result = String::new();
    let mut i = 0;
    for c in s.chars() {
        if i < start {
            i += 1;
            continue;
        }
        if end <= i {
            break;
        }
        result.push(c);
        i += 1;
    }
    result
}

pub struct TermIO {
    buffer: String,
    cursor_pos: usize,
}

impl TermIO {
    pub fn new() -> Self {
        TermIO {
            buffer: String::new(),
            cursor_pos: 0,
        }
    }
    pub fn read(&mut self) -> String {
        let stdin = stdin();
        let stdin = stdin.lock();
        let stdout = stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();
        self.buffer = String::new();
        self.cursor_pos = 0;
        for key in stdin.keys() {
            match key {
                Ok(Key::Char(c)) => {
                    if c == '\n' {
                        write!(stdout, "\r\n");
                        stdout.flush().ok();
                        return self.buffer.clone();
                    } else if c == '\t' {
                    } else {
                        self.clear_command(&mut stdout);
                        let cursor_left = string_slice(&self.buffer, 0, self.cursor_pos);
                        let cursor_right =
                            string_slice(&self.buffer, self.cursor_pos, self.buffer.len());
                        self.buffer = format!("{}{}{}", cursor_left, c, cursor_right);
                        write!(stdout, "{}", self.buffer);
                        self.cursor_pos += 1;
                        let move_left = (self.buffer.len() - self.cursor_pos) as u16;
                        self.cursor_to_left(&mut stdout, move_left);
                        stdout.flush().ok();
                    }
                }
                Ok(Key::Ctrl(c)) => {
                    if c == 'c' {
                        write!(stdout, "\r\n");
                        stdout.flush().ok();
                        return String::new();
                    }
                }
                Ok(Key::Backspace) => {
                    if 0 < self.cursor_pos {
                        self.clear_command(&mut stdout);
                        let cursor_left = string_slice(&self.buffer, 0, self.cursor_pos - 1);
                        let cursor_right =
                            string_slice(&self.buffer, self.cursor_pos, self.buffer.len());
                        self.buffer = format!("{}{}", cursor_left, cursor_right);
                        write!(stdout, "{}", self.buffer);
                        self.cursor_pos -= 1;
                        let move_left = (self.buffer.len() - self.cursor_pos) as u16;
                        self.cursor_to_left(&mut stdout, move_left);
                        stdout.flush().ok();
                    }
                }
                Ok(Key::Delete) => {
                    if self.cursor_pos < self.buffer.len() {
                        self.clear_command(&mut stdout);
                        let cursor_left = string_slice(&self.buffer, 0, self.cursor_pos);
                        let cursor_right =
                            string_slice(&self.buffer, self.cursor_pos + 1, self.buffer.len());
                        self.buffer = format!("{}{}", cursor_left, cursor_right);
                        write!(stdout, "{}", self.buffer);
                        let move_left = (self.buffer.len() - self.cursor_pos) as u16;
                        self.cursor_to_left(&mut stdout, move_left);
                        stdout.flush().ok();
                    }
                }
                Ok(Key::Left) => {
                    if 0 < self.cursor_pos {
                        self.cursor_pos -= 1;
                        write!(stdout, "{}", cursor::Left(1));
                        stdout.flush().ok();
                    }
                }
                Ok(Key::Right) => {
                    if self.cursor_pos < self.buffer.len() {
                        self.cursor_pos += 1;
                        write!(stdout, "{}", cursor::Right(1));
                        stdout.flush().ok();
                    }
                }
                Ok(_) => {}
                Err(_) => {}
            }
        }
        self.buffer.clone()
    }

    pub fn clear_command(&mut self, stdout: &mut dyn Write) {
        if 0 < self.cursor_pos {
            write!(stdout, "{}", cursor::Left(self.cursor_pos as u16));
        }
        for _ in 0..self.buffer.len() {
            write!(stdout, " ");
        }
        if 0 < self.buffer.len() {
            write!(stdout, "{}", cursor::Left(self.buffer.len() as u16));
        }
    }

    fn cursor_to_left(&mut self, stdout: &mut dyn Write, n: u16) {
        if 0 < n {
            write!(stdout, "{}", cursor::Left(n));
        }
    }
}
