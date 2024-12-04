use crate::bin::get_bin_path;
use crate::command::Command;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, i32, multispace0};
use nom::combinator::{all_consuming, value};
use nom::multi::{fold_many1, many0};
use nom::sequence::{delimited, terminated};
use nom::{branch::alt, IResult};
use std::str::FromStr;

pub fn parse_input(input: &str) -> IResult<&str, Command> {
    alt((
        parse_exit,
        parse_echo,
        parse_type,
        parse_cd,
        value(Command::Pwd, all_consuming(tag("pwd"))),
        parse_bin,
    ))(input)
}

fn parse_exit(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("exit ")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, status) = i32(input)?;
    Ok((input, Command::Exit(status)))
}

fn parse_arg(input: &str) -> IResult<&str, &str> {
    let (input, arg) = terminated(
        alt((
            delimited(char('\''), is_not("'"), char('\'')),
            // NOTE: actual double quoting in POSIX shell is more complex
            // and requires special handling of \ $ and backtick `
            delimited(char('"'), is_not("\""), char('"')),
            is_not(" \t\n\r"),
        )),
        multispace0,
    )(input)?;
    Ok((input, arg))
}

fn parse_args(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, args) = fold_many1(parse_arg, Vec::new, |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
    })(input)?;
    let args = Vec::from_iter(args);
    Ok((input, args))
}

fn parse_echo(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("echo ")(input)?;
    let (input, args) = parse_args(input)?;
    Ok((input, Command::Echo(args)))
}

fn parse_type(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("type ")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    Ok((input, Command::Type(input)))
}

fn parse_bin(input: &str) -> IResult<&str, Command> {
    let (input, cmd) = is_not(" \t\n\r")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (_, args) = parse_args(input)?;
    let path = get_bin_path(cmd);
    match path {
        Ok(p) => Ok((input, Command::Bin(p, args))),
        Err(_) => Err(nom::Err::Error(nom::error::Error {
            input: "bin",
            code: nom::error::ErrorKind::Tag,
        })),
    }
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("cd ")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    match std::path::PathBuf::from_str(input) {
        Ok(p) => Ok((input, Command::Cd(p))),
        Err(_) => Err(nom::Err::Error(nom::error::Error {
            input: "cd",
            code: nom::error::ErrorKind::Tag,
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_echo() {
        let (_, cmd) = parse_echo("echo hello").unwrap();
        assert_eq!(cmd, Command::Echo(vec!["hello"]));
    }

    #[test]
    fn test_parse_args() {
        let (_, args) = parse_args("--hello world '1 2 3'").unwrap();
        assert_eq!(args, vec!["--hello", "world", "1 2 3"]);
        let (_, args) = parse_args("\"'x' 'y'\"").unwrap();
        assert_eq!(args, vec!["'x' 'y'"]);
    }
}
