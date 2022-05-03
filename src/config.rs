use std::env;
use std::fmt;
use std::path::Path;

pub struct Config {
    data_directory: String,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "node '{}'", self.data_directory)
    }
}

impl Config {
    pub fn new() -> Config {
        let mut dir = "".to_string();
        let env_key = "MOTIF_DATA";
        match env::var(env_key) {
            Ok(val) => dir = val.to_owned().to_string(),
            Err(e) => println!("could not read var {}, {}", env_key, e)
        }
        if !Path::new(&dir).is_dir() {
            println!("config {} directory does not exist", dir);
        }
        Config {
            data_directory: dir.to_string(),
        }
    }
}