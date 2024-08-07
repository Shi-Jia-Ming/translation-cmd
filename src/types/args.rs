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
