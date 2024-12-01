pub use nom::bytes::complete::tag;
pub use nom::IResult;
use nom::{branch::alt, combinator::value};
#[allow(unused_imports)]
use std::io::{self, Write};

#[derive(Clone)]
enum Command {
    Exit,
    Echo,
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

        if run_command(input) {
            break;
        }
    }
}

fn run_command(input: String) -> bool {
    // command not found
    let maybe_command = parse_input(&input.trim());
    match maybe_command {
        Ok(command) => match command {
            (_, Command::Exit) => true,
            (rem, Command::Echo) => {
                println!("{}", rem);
                false
            }
        },
        Err(_) => {
            println!("{}: command not found", input.trim());
            false
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Command> {
    //  note that this is really creating a function, the parser for abc
    //  vvvvv
    //         which is then called here, returning an IResult<&str, &str>
    //         vvvvv
    alt((
        value(Command::Exit, tag("exit 0")),
        value(Command::Echo, tag("echo ")),
    ))(input)
}
