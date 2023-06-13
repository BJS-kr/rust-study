use std::{env, error::Error};
pub struct Config {
    pub file_path: String,
    pub query: String
}

impl Config {
    pub fn build() -> Result<Config, Box<dyn Error>> {
        let argv:Vec<String> = env::args().collect();
        let file_path = argv.get(1).unwrap_or(&"./src/test.txt".to_string()).to_string();
        let query =argv.get(2).unwrap_or(&"test".to_string()).to_string();

        Ok(Config { file_path, query })
    }
}



