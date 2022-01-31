use crate::cli::commands::{Cli, Commands, GenTypes};
use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::doc_types::Doc;
use crate::parsers::toml::TomlParser;
use crate::parsers::ConfigParser;

use crate::generators::markdown::MarkdownGenerator;
use crate::generators::Generator;
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
        Commands::Gen(comm) => {
            let iter = std::fs::read_dir(".")?;

            //Do single directory if not recursing
            if !comm.recurse {
                log::debug!("Beginning non-recursive search");

                let mut parser = TomlParser::default();
                let mut generator: Box<dyn Generator> = match comm.doc_type {
                    GenTypes::Markdown => Box::new(MarkdownGenerator::default()),
                    _ => Box::new(MarkdownGenerator::default()),
                };

                for file in iter {
                    let file = file?;
                    //Read any *.doctor.toml files
                    if file
                        .file_name()
                        .to_str()
                        .context("Filename contains invalid chars")?
                        .ends_with("doctor.toml")
                    {
                        let path = file.path();
                        log::debug!("Reading config: {path:?}");

                        //Parse and gen doc
                        let config = parser.parse_path(path)?;
                        let md = generator.generate_string(config)?;
                        log::debug!("Generated Doc");

                        //Write doc
                        for (name, doc) in md {
                            let mut path = PathBuf::from(comm.dest_dir.clone());
                            path.push(format!("{name}.md"));

                            log::debug!("Writing to: {path:?}");

                            let mut file = File::create(path)?;
                            file.write_all(doc.as_bytes())?;

                            println!("Created doc for {name}")
                        }
                    }
                }
            }
            else if cfg!(feature = "multi_thread") {
                //TODO rayon
            }
            else {
                log::debug!("Beginning recursive search");

                let mut parser = TomlParser::default();
                let mut generator: Box<dyn Generator> = match comm.doc_type {
                    GenTypes::Markdown => Box::new(MarkdownGenerator::default()),
                    _ => Box::new(MarkdownGenerator::default()),
                };

                for file in iter {
                    let file = file?;

                    if file.path().is_dir() {
                        //TODO recusion is hard
                    }

                    //Read any *.doctor.toml files
                    if file
                        .file_name()
                        .to_str()
                        .context("Filename contains invalid chars")?
                        .ends_with("doctor.toml")
                    {
                        let path = file.path();
                        log::debug!("Reading config: {path:?}");

                        //Parse and gen doc
                        let config = parser.parse_path(path)?;
                        let md = generator.generate_string(config)?;
                        log::debug!("Generated Doc");

                        //Write doc
                        for (name, doc) in md {
                            let mut path = PathBuf::from(comm.dest_dir.clone());
                            path.push(format!("{name}.md"));

                            log::debug!("Writing to: {path:?}");

                            let mut file = File::create(path)?;
                            file.write_all(doc.as_bytes())?;

                            println!("Created doc for {name}")
                        }
                    }
                }
            }
        }
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
