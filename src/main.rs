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
        // Uncomment this block to pass the first stage
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
            (_, Command::Type(s)) => match s.as_str() {
                "exit" | "echo" | "type" => {
                    println!("{} is a shell builtin", s)
                }
                _ => println!("{}: not found", s),
            },
        },
        Err(_) => {
            println!("{}: command not found", input.trim())
        }
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
