pub use nom::bytes::complete::tag;
pub use nom::IResult;
use nom::{
    branch::alt,
    bytes::complete::take_till1,
    character::is_space,
    combinator::{rest, value},
    multi::many0,
    sequence::{preceded, separated_pair},
};
#[allow(unused_imports)]
use std::io::{self, Write};

#[derive(Clone)]
enum Command {
    Exit,
    Echo,
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
            (_, Command::Exit) => std::process::exit(0),
            (rem, Command::Echo) => {
                println!("{}", rem)
            }
            (_, Command::Type(s)) => {
                println!("type {}", s)
            }
        },
        Err(_) => {
            println!("{}: command not found", input.trim())
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Command> {
    alt((
        value(Command::Exit, tag("exit 0")),
        value(Command::Echo, tag("echo ")),
        parse_type,
    ))(input)
}

fn parse_type(input: &str) -> IResult<&str, Command> {
    let (remaining, _) = tag("type ")(input)?;
    let (remaining, _) = many0(tag(" "))(remaining)?;
    Ok((remaining, Command::Type(remaining.to_string())))
}
