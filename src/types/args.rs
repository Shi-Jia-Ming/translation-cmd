use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub(crate) struct Arg {
    #[structopt(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, StructOpt)]
pub(crate) enum Command {
    /// execute translation command
    Run(TransArgs),
    /// configuration manage
    Config(ConfigurationOperation),
}

/// Args of translation
#[derive(Debug, StructOpt)]
pub(crate) struct TransArgs {
    /// origin language
    #[structopt(short, long, default_value = "default")]
    pub(crate) from: String,
    /// target language
    #[structopt(short, long, default_value = "default")]
    pub(crate) to:  String,
    /// the word to translate
    pub(crate) target: String
}


#[derive(Debug, StructOpt)]
pub(crate) struct ConfigurationOperation {
    /// list the configuration options
    #[structopt(short, long, conflicts_with= "command")]
    pub(crate) list: bool,
    /// set configuration option
    #[structopt(subcommand)]
    pub(crate) command: Option<ConfigurationOption>
}

/// Args of configuration
#[derive(Debug, StructOpt)]
pub(crate) enum ConfigurationOption {
    /// set configuration options
    Set(ConfigurationArgs),
}


#[derive(Debug, StructOpt)]
pub(crate) struct ConfigurationArgs {
    /// key of the configuration
    #[structopt(long, short)]
    pub(crate) field: String,
    /// value of the configuration
    pub(crate) value: String,
}