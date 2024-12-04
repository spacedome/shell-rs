pub fn get_bin_path(input: &str) -> Result<std::path::PathBuf, String> {
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
