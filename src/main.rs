pub use nom::bytes::complete::tag;
pub use nom::IResult;
use nom::{branch::alt, multi::many0};
#[allow(unused_imports)]
use std::io::{self, Write};
use std::str::FromStr;

mod command;
use crate::command::Command;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        run_command(input);
    }
}

fn run_command(input: String) {
    // command not found
    let maybe_command = parse_input(&input.trim());
    match maybe_command {
        Ok(command) => match command {
            (_, Command::Exit(status)) => std::process::exit(status),
            (_, Command::Echo(rem)) => {
                println!("{}", rem.join(" "))
            }
            (_, Command::Type(s)) => run_type(&s),
            (_, Command::Bin(s, args)) => run_bin(s, args),
            (_, Command::Pwd) => run_pwd(),
            (_, Command::Cd(p)) => run_cd(p),
        },
        Err(_) => {
            println!("{}: command not found", input.trim())
        }
    }
}

fn get_bin_path(input: &str) -> Result<std::path::PathBuf, String> {
    let path = match std::env::var("PATH") {
        Ok(p) => p,
        Err(_) => return Err("Error finding PATH".to_string()),
    };

    // You can split the PATH into individual directories
    for dir in std::env::split_paths(&path) {
        if dir.is_dir() {
            match std::fs::read_dir(dir) {
                Ok(entries) => {
                    for entry in entries {
                        if let Ok(item) = entry {
                            if item.file_name() == input {
                                return Ok(item.path().canonicalize().unwrap());
                            }
                        }
                    }
                }
                Err(_) => (),
            }
        }
    }
    Err(format!("{}: not found", input))
}

fn run_type(input: &str) {
    match input {
        "exit" | "echo" | "type" | "pwd" | "cd" => {
            println!("{} is a shell builtin", input)
        }
        _ => match get_bin_path(input) {
            Ok(s) => println!("{} is {}", input, s.display()),
            Err(s) => println!("{}", s),
        },
    }
}

fn run_bin(path: std::path::PathBuf, args: Vec<&str>) {
    let _status = std::process::Command::new(path).args(args).status();
}

fn run_pwd() {
    match std::env::current_dir() {
        Ok(pwd) => println!("{}", pwd.display()),
        Err(_) => println!("error: could not get PWD"),
    };
}

fn run_cd(path: std::path::PathBuf) {
    let path = if path.display().to_string() == "~" {
        #[allow(deprecated)]
        std::env::home_dir().unwrap()
    } else {
        path
    };
    if path.exists() && path.is_dir() {
        match std::env::set_current_dir(path) {
            Ok(_) => (),
            Err(_) => println!("Could not chdir"),
        }
    } else {
        println!("cd: {}: No such file or directory", path.display());
    }
}

fn parse_input(input: &str) -> IResult<&str, Command> {
    alt((
        parse_exit,
        parse_echo,
        parse_type,
        parse_cd,
        nom::combinator::value(Command::Pwd, nom::combinator::all_consuming(tag("pwd"))),
        parse_bin,
    ))(input)
}

fn parse_exit(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("exit ")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, status) = nom::character::complete::i32(input)?;
    Ok((input, Command::Exit(status)))
}

fn parse_arg(input: &str) -> IResult<&str, &str> {
    let (input, arg) = nom::sequence::terminated(
        alt((
            nom::sequence::delimited(
                nom::character::complete::char('\''),
                nom::bytes::complete::is_not("'"),
                nom::character::complete::char('\''),
            ),
            // NOTE: actual double quoting in POSIX shell is more complex
            // and requires special handling of \ $ and backtick `
            nom::sequence::delimited(
                nom::character::complete::char('"'),
                nom::bytes::complete::is_not("\""),
                nom::character::complete::char('"'),
            ),
            nom::bytes::complete::is_not(" \t\n\r"),
        )),
        nom::character::complete::multispace0,
    )(input)?;
    Ok((input, arg))
}

fn parse_args(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, args) = nom::multi::fold_many1(parse_arg, Vec::new, |mut acc: Vec<_>, item| {
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
    let (input, cmd) = nom::bytes::complete::is_not(" \t\n\r")(input)?;
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
