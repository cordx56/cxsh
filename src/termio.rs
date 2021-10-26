use std::fmt;
use std::io;
use std::io::{stderr, stdin, stdout, Write};
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

pub fn read() -> String {
    let stdin = stdin();
    let stdin = stdin.lock();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stderr = stderr();
    let mut stderr = stderr.lock().into_raw_mode().unwrap();

    let mut buffer = String::new();
    let mut cursor_pos: usize = 0;
    for key in stdin.keys() {
        match key {
            Ok(Key::Char(c)) => {
                if c == '\n' {
                    write!(stdout, "\r\n");
                    stdout.flush().ok();
                    return buffer;
                } else if c == '\t' {
                } else {
                    clear_command(&mut stdout, &buffer, cursor_pos);
                    let cursor_left = string_slice(&buffer, 0, cursor_pos);
                    let cursor_right = string_slice(&buffer, cursor_pos, buffer.len());
                    buffer = format!("{}{}{}", cursor_left, c, cursor_right);
                    write!(stdout, "{}", buffer);
                    cursor_pos += 1;
                    let move_left = (buffer.len() - cursor_pos) as u16;
                    cursor_to_left(&mut stdout, move_left);
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
                if 0 < cursor_pos {
                    clear_command(&mut stdout, &buffer, cursor_pos);
                    let cursor_left = string_slice(&buffer, 0, cursor_pos - 1);
                    let cursor_right = string_slice(&buffer, cursor_pos, buffer.len());
                    buffer = format!("{}{}", cursor_left, cursor_right);
                    write!(stdout, "{}", buffer);
                    cursor_pos -= 1;
                    let move_left = (buffer.len() - cursor_pos) as u16;
                    cursor_to_left(&mut stdout, move_left);
                    stdout.flush().ok();
                }
            }
            Ok(Key::Delete) => {
                if cursor_pos < buffer.len() {
                    clear_command(&mut stdout, &buffer, cursor_pos);
                    let cursor_left = string_slice(&buffer, 0, cursor_pos);
                    let cursor_right = string_slice(&buffer, cursor_pos + 1, buffer.len());
                    buffer = format!("{}{}", cursor_left, cursor_right);
                    write!(stdout, "{}", buffer);
                    let move_left = (buffer.len() - cursor_pos) as u16;
                    cursor_to_left(&mut stdout, move_left);
                    stdout.flush().ok();
                }
            }
            Ok(Key::Left) => {
                if 0 < cursor_pos {
                    cursor_pos -= 1;
                    write!(stdout, "{}", cursor::Left(1));
                    stdout.flush().ok();
                }
            }
            Ok(Key::Right) => {
                if cursor_pos < buffer.len() {
                    cursor_pos += 1;
                    write!(stdout, "{}", cursor::Right(1));
                    stdout.flush().ok();
                }
            }
            Ok(_) => {}
            Err(_) => {
                write!(stderr, "stdin read error\r\n");
            }
        }
    }
    buffer
}

fn clear_command(stdout: &mut dyn Write, buffer: &str, cursor_pos: usize) {
    if 0 < cursor_pos {
        write!(stdout, "{}", cursor::Left(cursor_pos as u16));
    }
    for _ in 0..buffer.len() {
        write!(stdout, " ");
    }
    if 0 < buffer.len() {
        write!(stdout, "{}", cursor::Left(buffer.len() as u16));
    }
}

fn cursor_to_left(stdout: &mut dyn Write, n: u16) {
    if 0 < n {
        write!(stdout, "{}", cursor::Left(n));
    }
}
