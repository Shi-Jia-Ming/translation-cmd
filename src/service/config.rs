use std::{env, fs::File, io::Read};
use std::fs::OpenOptions;
use std::io::Write;

use crate::types::args::{ConfigurationOperation, ConfigurationOption};
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

pub(crate) fn configuration(args: ConfigurationOperation, mut config: Configuration) -> String {
    // list the configuration options
    if args.list {
        return config.to_str();
    }

    // change config option
    let mut previous_value: String = String::new();
    let mut new_value: String = String::new();
    let mut changed_field: String = String::new();
    match args.command {
        None => {}
        Some(ConfigurationOption::Set(args)) => {
            changed_field = args.key.clone();
            new_value = args.value.clone();
            match args.key.as_str() {
                "from" => {
                    previous_value = config.default.from;
                    config.default.from = args.value.into();
                }
                "to" => {
                    previous_value = config.default.to;
                    config.default.to = args.value.into();
                }
                _ => {
                    // TODO 报错
                }
            }
        }
    }

    let exe_path = env::current_exe().expect("Failed to locate binary file");

    let parent_path = exe_path.parent().unwrap_or(&exe_path).parent().unwrap_or(&exe_path);
    let config_path = parent_path.join("conf").join("config.toml");

    let toml = toml::to_string_pretty(&config).expect("Failed to parse from string");

    let mut file = OpenOptions::new().write(true).truncate(true).open(config_path).expect("Failed to open configuration file.");
    file.write_all(toml.as_bytes()).expect("Failed to write into configuration file.");

    let mut result = String::new();
    result += &*format!("{}: {} => {}", changed_field, previous_value, new_value);

    result
}