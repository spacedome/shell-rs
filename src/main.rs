pub use nom::bytes::complete::tag;
pub use nom::IResult;
use nom::{branch::alt, multi::many0};
#[allow(unused_imports)]
use std::io::{self, Write};

#[derive(Clone)]
enum Command {
    Exit(i32),
    Echo(String),
    Type(String),
}

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
                println!("{}", rem)
            }
            (_, Command::Type(s)) => run_type(&s),
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
        "exit" | "echo" | "type" => {
            println!("{} is a shell builtin", input)
        }
        _ => match get_bin_path(input) {
            Ok(s) => println!("{} is {}", input, s.display()),
            Err(s) => println!("{}", s),
        },
    }
}

fn parse_input(input: &str) -> IResult<&str, Command> {
    alt((parse_exit, parse_echo, parse_type))(input)
}

fn parse_exit(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("exit ")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, status) = nom::character::complete::i32(input)?;
    Ok((input, Command::Exit(status)))
}

fn parse_echo(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("echo ")(input)?;
    Ok((input, Command::Echo(input.to_string())))
}

fn parse_type(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("type ")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    Ok((input, Command::Type(input.to_string())))
}
