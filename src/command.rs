use crate::bin::get_bin_path;
use crate::parse::parse_input;

#[derive(Clone, Debug, PartialEq)]
pub enum Command<'a> {
    Pwd,
    Cd(std::path::PathBuf),
    Exit(i32),
    Echo(Vec<&'a str>),
    Type(&'a str),
    Bin(std::path::PathBuf, Vec<&'a str>),
}

pub fn run_command(input: String) {
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
