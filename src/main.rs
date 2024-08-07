mod types;
mod service;

use structopt::StructOpt;
use service::translate::translate;
use crate::service::config::read_config;
use crate::types::args::{Arg, Command};
use crate::types::config::Configuration;

fn main() {
    let config: Configuration = match read_config() {
        Ok(config) => config,
        Err(_) => { return; }
    };
    
    let args: Arg = Arg::from_args();
    
    let result: String = match args.command {
        Command::Run(args) => translate(&args.from, &args.to, &args.target, &config.app_info.app_id, &config.app_info.app_secret, &config.default.from, &config.default.to),
        Command::Config(configuration) => {config.to_str()},
    };
    
    println!("{}", result);
}
