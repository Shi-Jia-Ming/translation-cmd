use std::{env, fs::File, io::Read};

use anyhow::Result;

use crate::types::config::Configuration;

pub(crate) fn read_config() -> Result<Configuration, std::io::Error> {
    let exe_path = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get the path of the current executable: {}", e);
            return Err(e);
        }
    };
    
    let parent_path = exe_path.parent().unwrap_or(&exe_path).parent().unwrap_or(&exe_path);
    let config_path = parent_path.join("conf").join("config.toml");
    
    // read config file
    let mut config_file = File::open(config_path).expect("Failed to open file");
    let mut contents = String::new();
    
    config_file.read_to_string(&mut contents).expect("Failed to read file");
    let config: Configuration = toml::from_str(&contents).expect("Failed to parse TOML");
    
    Ok(config)
}