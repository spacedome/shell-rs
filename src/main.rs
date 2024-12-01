#[allow(unused_imports)]
use std::io::{self, Write};

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

fn run_command(command: String) -> bool {
    // command not found
    match command.trim() {
        "exit 0" => true,
        x => {
            println!("{}: command not found", x);
            false
        }
    }
}
