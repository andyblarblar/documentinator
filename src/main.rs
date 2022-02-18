use std::fs::File;
use std::io::Write;

use anyhow::{Context, Result};
use clap::Parser;

use crate::cli::commands::{Cli, Commands, GenCommand, GenTypes};
use crate::cli::gen::generate_files;
use crate::doc_types::Doc;
use crate::generators::markdown::MarkdownGenerator;
use crate::generators::Generator;
use crate::parsers::toml::TomlParser;
use crate::parsers::ConfigParser;

mod cli;
mod doc_types;
mod generators;
mod parsers;

/// Text of file used in init
const DEMO_CONFIG_STR: &str = include_str!("../assets/demo.doctor.toml");

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Gen(comm) => {
            env_logger::Builder::new()
                .filter_level(comm.verbose.log_level_filter())
                .init();

            let stats = generate_files(comm)?;

            if !stats.nodes_processed.is_empty() {
                println!("\nGenerated docs for: ");
                stats.nodes_processed.iter().for_each(|doc| {
                    for node in doc.nodes.iter() {
                        println!("{}", node.node_name);
                    }
                });
                println!()
            }

            println!(
                "done. \n Processed: \n {} files \n {} directories \n in {}s",
                stats.files_read,
                stats.dir_read,
                stats.duration.as_secs_f64()
            );
        }
        Commands::Init { node_name, .. } => {
            let mut file = File::create(format!("./{node_name}.doctor.toml"))
                .context(format!("Failed to create file ./{node_name}.doctor.toml"))?;

            log::debug!("Created config file: {node_name}.doctor.toml");

            file.write_all(DEMO_CONFIG_STR.as_bytes())?;

            log::debug!("Wrote demo config file");

            println!("Successfully created {node_name}.doctor.toml");
        }
        Commands::Verify { .. } => {
            todo!()
        }
    }

    Ok(())
}
