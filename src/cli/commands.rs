use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[non_exhaustive]
#[derive(clap::ArgEnum, Copy, Clone)]
pub enum GenTypes {
    /// Generates a markdown (.md) document.
    Markdown,
}

#[derive(clap::Args)]
pub struct GenCommand {
    /// Directory to search for *.doctor.toml files.
    pub source_dir: String,
    /// Directory to place generated docs in.
    #[clap(short, long, default_value_t = String::from("."))]
    pub dest_dir: String,
    /// Recursively search directories for more configs.
    #[clap(long, short)]
    pub recurse: bool,
    /// Creates a NODES_README.md that links to the other documents, if any.
    /// Useful for git repos.
    #[clap(long)]
    pub readme: bool,
    /// Type of document to generate.
    #[clap(arg_enum, long, default_value_t = GenTypes::Markdown)]
    pub doc_type: GenTypes,
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate documentation for 1-many nodes.
    Gen(GenCommand),

    /// Creates an empty config file
    Init {
        /// Node name to be used with config
        node_name: String,
        #[clap(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    }
}
