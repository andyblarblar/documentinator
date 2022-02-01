use std::fs::{DirEntry, File, ReadDir};
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use concurrent_queue::ConcurrentQueue;

use crate::cli::commands::{Cli, Commands, GenCommand, GenTypes};
use crate::doc_types::Doc;
use crate::generators::markdown::MarkdownGenerator;
use crate::generators::Generator;
use crate::parsers::toml::TomlParser;
use crate::parsers::ConfigParser;

mod cli;
mod doc_types;
mod generators;
mod parsers;

const DEMO_CONFIG_STR: &str = include_str!("../assets/demo.doctor.toml");

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Gen(comm) => {
            env_logger::Builder::new()
                .filter_level(comm.verbose.log_level_filter())
                .init();

            generate_files(comm)?;

            println!("done.")
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

//TODO move these to another file and cleanup

/// Generates files using rayon, for a small performance boost with huge directory sizes.
#[cfg(feature = "multi_thread")]
fn generate_files(comm: GenCommand) -> Result<()> {
    use rayon::prelude::*;
    use std::sync::Arc;

    let iter = std::fs::read_dir(comm.source_dir.clone())?;

    log::debug!("Beginning parallel recursive search");

    //To recurse, build a queue of dir iterators. Need to sync across threads
    let dir_queue = Arc::new(ConcurrentQueue::unbounded());
    dir_queue.push(iter)?;

    //Dyn dispatch to allow for CLI flags to choose the doc types
    let parser: Arc<dyn ConfigParser + Send + Sync> = Arc::new(TomlParser::default());
    let generator: Arc<dyn Generator + Send + Sync> = match comm.doc_type {
        GenTypes::Markdown => Arc::new(MarkdownGenerator::default()),
    };

    //Iteratively recurse using queue to buffer future directories to crawl.
    while let Ok(iter) = dir_queue.pop() {
        log::trace!("recurring new directory");

        //Clone to send to thread pool
        let parser = parser.clone();
        let generator = generator.clone();
        let dir_queue = dir_queue.clone();

        //Generate each doc in parallel
        iter.par_bridge()
            .map(|file| {
                process_file(&comm, &*dir_queue, &*parser, &*generator, file?)?;
                Ok(())
            })
            .collect::<Result<Vec<()>>>()?; //Pool all the errors into one
    }

    Ok(())
}

/// Generates files without rayon in case the environment does not support it.
#[cfg(not(feature = "multi_thread"))]
fn generate_files(comm: GenCommand) -> Result<()> {
    let iter = std::fs::read_dir(comm.source_dir.clone())?;
    //Non threaded recursion
    log::debug!("Beginning single threaded recursive search");

    //To recurse, build a queue of dir iterators
    let dir_queue = ConcurrentQueue::unbounded();
    dir_queue.push(iter)?;

    let parser: Box<dyn ConfigParser> = Box::new(TomlParser::default());
    let generator: Box<dyn Generator> = match comm.doc_type {
        GenTypes::Markdown => Box::new(MarkdownGenerator::default()),
    };

    //Iteratively recurse using queue to buffer future directories to crawl.
    while let Ok(iter) = dir_queue.pop() {
        log::trace!("recurring new directory");

        for file in iter {
            process_file(&comm, &dir_queue, &*parser, &*generator, file?)?;
        }
    }

    log::trace!("queue size: {}", dir_queue.len());

    Ok(())
}

/// Handles a single file in the generator.
fn process_file(
    comm: &GenCommand,
    dir_queue: &ConcurrentQueue<ReadDir>,
    parser: &dyn ConfigParser,
    generator: &dyn Generator,
    file: DirEntry,
) -> Result<()> {
    //If we find another dir to search, add to queue, then finish current dir
    if file.path().is_dir() && comm.recurse {
        log::trace!("adding dir to recurse");
        dir_queue.push(std::fs::read_dir(file.path())?)?;
        return Ok(());
    }

    //Read any config files
    if parser.match_filename(
        file.file_name()
            .to_str()
            .context("Filename contains invalid chars")?,
    ) {
        let path = file.path();
        log::debug!("Reading config: {path:?}");

        //Parse and gen doc
        let config = parser.parse_path(path)?;
        let docs = generator.generate_string(config)?;
        log::debug!("Generated Doc");

        //Write doc
        for (name, doc) in docs {
            let mut path = PathBuf::from(comm.dest_dir.clone());
            path.push(generator.add_file_extension(name.clone()));

            log::debug!("Writing to: {path:?}");

            let mut file = File::create(path)?;
            file.write_all(doc.as_bytes())?;

            println!("Created doc for {name}")
        }
    }

    Ok(())
}
