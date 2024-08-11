use std::{env, fs::File, io::Read};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use serde::Serialize;

use crate::types::args::{ConfigurationOperation, ConfigurationOption};
use crate::types::config::Configuration;

#[derive(Debug, Serialize)]
pub(super) struct ConfigHandler {
    config_path: PathBuf,
}

impl ConfigHandler {
    /// load config from configuration file, return the configuration and file location.
    pub fn load_config() -> (Configuration, PathBuf) {
        let exe_path = env::current_exe().expect("Failed to locate execute path");

        let parent_path = exe_path.parent().unwrap_or(&exe_path).parent().unwrap_or(&exe_path);

        let config_path: PathBuf = parent_path.join("conf").join("config.toml");

        // read config file
        let mut config_file = File::open(&config_path).expect("Failed to open file");
        let mut contents = String::new();

        config_file.read_to_string(&mut contents).expect("Failed to read file");

        (toml::from_str(&contents).expect("Failed to parse TOML"), config_path)
    }

    /// change configuration and sync to configuration file.
    fn write_config(args: ConfigurationOperation, mut config: Configuration, config_path: PathBuf) -> String {
        // change config option
        let mut previous_value: String = String::new();
        let mut new_value: String = String::new();
        let mut changed_field: String = String::new();
        match args.command {
            None => {}
            Some(ConfigurationOption::Set(args)) => {
                changed_field = args.field.clone();
                new_value = args.value.clone();
                match args.field.as_str() {
                    "from" => {
                        previous_value = config.default.from;
                        config.default.from = args.value.into();
                    },
                    "to" => {
                        previous_value = config.default.to;
                        config.default.to = args.value.into();
                    },
                    "api_version" => {
                        previous_value = config.app_info.api_version;
                        config.app_info.api_version = args.value.into();
                    },
                    _ => {
                        // TODO 报错
                    }
                }
            }
        }

        let toml = toml::to_string_pretty(&config).expect("Failed to parse from string");

        let mut file = OpenOptions::new().write(true).truncate(true).open(config_path).expect("Failed to open configuration file.");
        file.write_all(toml.as_bytes()).expect("Failed to write into configuration file.");

        format!("changed field `{}`: {} => {}", changed_field, previous_value, new_value)
    }
}

pub(crate) fn configuration(args: ConfigurationOperation) -> String {

    // load configuration
    let (config, config_path) = ConfigHandler::load_config();

    // list the configuration options
    if args.list {
        return config.to_str();
    }

    // change config option
    ConfigHandler::write_config(args, config, config_path)
}