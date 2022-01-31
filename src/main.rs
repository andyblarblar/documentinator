use crate::cli::commands::{Cli, Commands};
use anyhow::{Context, Result};
use std::fmt::format;
use std::fs::File;
use std::io::Write;

use crate::doc_types::Doc;
use crate::parsers::toml::TomlParser;
use crate::parsers::ConfigParser;

use clap::Parser;

mod cli;
mod doc_types;
mod error;
mod generators;
mod parsers;

const DEMO_CONFIG_STR: &str = include_str!("../assets/demo.doctor.toml");

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Gen(comm) => {}
        Commands::Init { node_name } => {
            let mut file = File::create(format!("./{node_name}.doctor.toml"))
                .context(format!("Failed to create file ./{node_name}.doctor.toml"))?;

            log::debug!("Created config file: {node_name}.doctor.toml");

            file.write_all(DEMO_CONFIG_STR.as_bytes())?;

            log::debug!("Wrote demo config file");

            println!("Successfully created {node_name}.doctor.toml");
        }
        Commands::Verify { .. } => {}
    }

    //TODO make do thing

    Ok(())
}
