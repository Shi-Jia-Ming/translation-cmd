use structopt::StructOpt;

use crate::service::translate::translate;
use crate::service::config::configuration;

use crate::types::args::{Arg, Command};

mod types;
mod service;
mod utils;

fn main() {
    let args: Arg = Arg::from_args();
    
    let result: String = match args.command {
        Command::Run(args) => translate(args),
        Command::Config(args) => configuration(args),
    };
    
    println!("{}", result);
}
