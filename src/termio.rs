use std::fmt;
use std::io;
use std::io::{stderr, stdin, stdout, Write};
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

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
    for key_result in stdin.keys() {
        match key_result {
            Ok(key) => match key {
                Key::Char('\n') => {
                    write!(stdout, "\r\n");
                    stdout.flush().ok();
                    return buffer;
                }
                Key::Char('\t') => {}
                Key::Char(c) => {
                    clear_command(&mut stdout, &buffer, cursor_pos);
                    let cursor_left = string_slice(&buffer, 0, cursor_pos);
                    let cursor_right = string_slice(&buffer, cursor_pos, buffer.len());
                    buffer = format!("{}{}{}", cursor_left, c, cursor_right);
                    write!(stdout, "{}", buffer);
                    cursor_pos += 1;
                    let move_left = UnicodeWidthStr::width(&cursor_right as &str);
                    cursor_to_left(&mut stdout, move_left);
                    stdout.flush().ok();
                }
                Key::Ctrl(c) => {
                    if c == 'c' {
                        write!(stdout, "\r\n");
                        stdout.flush().ok();
                        return String::new();
                    }
                }
                Key::Backspace => {
                    if 0 < cursor_pos {
                        clear_command(&mut stdout, &buffer, cursor_pos);
                        let cursor_left = string_slice(&buffer, 0, cursor_pos - 1);
                        let cursor_right = string_slice(&buffer, cursor_pos, buffer.len());
                        buffer = format!("{}{}", cursor_left, cursor_right);
                        write!(stdout, "{}", buffer);
                        cursor_pos -= 1;
                        let move_left = UnicodeWidthStr::width(&cursor_right as &str);
                        cursor_to_left(&mut stdout, move_left);
                        stdout.flush().ok();
                    }
                }
                Key::Delete => {
                    if cursor_pos < buffer.len() {
                        clear_command(&mut stdout, &buffer, cursor_pos);
                        let cursor_left = string_slice(&buffer, 0, cursor_pos);
                        let cursor_right = string_slice(&buffer, cursor_pos + 1, buffer.len());
                        buffer = format!("{}{}", cursor_left, cursor_right);
                        write!(stdout, "{}", buffer);
                        let move_left = UnicodeWidthStr::width(&cursor_right as &str);
                        cursor_to_left(&mut stdout, move_left);
                        stdout.flush().ok();
                    }
                }
                Key::Left => {
                    if 0 < cursor_pos {
                        cursor_pos -= 1;
                        let move_left = UnicodeWidthChar::width(buffer.chars().nth(cursor_pos).unwrap()).unwrap();
                        write!(stdout, "{}", cursor::Left(move_left as u16));
                        stdout.flush().ok();
                    }
                }
                Key::Right => {
                    if cursor_pos < buffer.len() {
                        let move_right = UnicodeWidthChar::width(buffer.chars().nth(cursor_pos).unwrap()).unwrap();
                        cursor_pos += 1;
                        write!(stdout, "{}", cursor::Right(move_right as u16));
                        stdout.flush().ok();
                    }
                }
                _ => {}
            },
            Err(_) => {
                write!(stderr, "stdin read error\r\n");
            }
        }
    }
    buffer
}

fn clear_command(stdout: &mut dyn Write, buffer: &str, cursor_pos: usize) {
    if 0 < cursor_pos {
        let cursor_left = string_slice(&buffer, 0, cursor_pos);
        write!(stdout, "{}", cursor::Left(UnicodeWidthStr::width(&cursor_left as &str) as u16));
    }
    for _ in 0..UnicodeWidthStr::width(buffer) {
        write!(stdout, " ");
    }
    if 0 < buffer.len() {
        write!(stdout, "{}", cursor::Left(UnicodeWidthStr::width(buffer) as u16));
    }
}

fn cursor_to_left(stdout: &mut dyn Write, n: usize) {
    if 0 < n {
        write!(stdout, "{}", cursor::Left(n as u16));
    }
}
