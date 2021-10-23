use nom::{
    branch::{alt, permutation},
    bytes::complete::{escaped_transform, is_not, tag, take_while_m_n},
    character::complete::{char, line_ending, multispace0, none_of, space0, space1},
    combinator::{all_consuming, map, opt, value},
    error::VerboseError,
    multi::{many0, many1},
    number::complete::double,
    sequence::{delimited, tuple},
    IResult,
};
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::u16;

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Command(Vec<String>),
    Pipe(Vec<String>, Box<Command>),
    Redirect(Vec<String>, Box<Command>),
}

pub fn command_all_consuming(s: &str) -> IResult<&str, Command, VerboseError<&str>> {
    all_consuming(command)(s)
}

pub fn command(s: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((
            many1(map(tuple((multispace0, atom)), |(_, atom)| atom)),
            multispace0,
            opt(tuple((alt((tag("|"), tag(">"))), command))),
        )),
        |(atoms, _, pipe_or_redirect_opt)| {
            if let Some(pipe_or_redirect) = pipe_or_redirect_opt {
                if pipe_or_redirect.0 == "|" {
                    Command::Pipe(atoms, Box::new(pipe_or_redirect.1))
                } else {
                    Command::Redirect(atoms, Box::new(pipe_or_redirect.1))
                }
            } else {
                Command::Command(atoms)
            }
        },
    )(s)
}

pub fn atom(s: &str) -> IResult<&str, String, VerboseError<&str>> {
    alt((
        string,
        map(many1(none_of(" \t\r\n|><")), |char_vec| {
            let mut atom = String::new();
            for c in char_vec {
                atom.push(c);
            }
            atom
        }),
    ))(s)
}

pub fn string(s: &str) -> IResult<&str, String, VerboseError<&str>> {
    alt((string_content, string_empty))(s)
}
pub fn string_content(s: &str) -> IResult<&str, String, VerboseError<&str>> {
    delimited(
        char('"'),
        escaped_transform(
            none_of("\"\\"),
            '\\',
            alt((
                value('\\', char('\\')),
                value('\"', char('\"')),
                value('\'', char('\'')),
                value('\r', char('r')),
                value('\n', char('n')),
                value('\t', char('t')),
                map(
                    permutation((
                        char('u'),
                        take_while_m_n(4, 4, |c: char| c.is_ascii_hexdigit()),
                    )),
                    |(_, code): (char, &str)| -> char {
                        decode_utf16(vec![u16::from_str_radix(code, 16).unwrap()])
                            .nth(0)
                            .unwrap()
                            .unwrap_or(REPLACEMENT_CHARACTER)
                    },
                ),
            )),
        ),
        char('"'),
    )(s)
}
pub fn string_empty(s: &str) -> IResult<&str, String, VerboseError<&str>> {
    map(tuple((char('"'), char('"'))), |(_, _)| String::new())(s)
}
