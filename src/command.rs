#[derive(Clone, Debug, PartialEq)]
pub enum Command<'a> {
    Pwd,
    Cd(std::path::PathBuf),
    Exit(i32),
    Echo(Vec<&'a str>),
    Type(&'a str),
    Bin(std::path::PathBuf, Vec<&'a str>),
}
