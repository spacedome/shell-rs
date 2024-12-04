use std::io::{self, Write};

mod bin;
mod command;
mod parse;

use crate::command::run_command;

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
