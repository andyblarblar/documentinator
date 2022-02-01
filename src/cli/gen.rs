//! Logic for the `gen` command.

use crate::{ConfigParser, GenCommand, GenTypes, Generator, MarkdownGenerator, TomlParser};
use anyhow::{Context, Result};
use std::collections::VecDeque;
use std::fs::{DirEntry, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Generates files through recursion
pub fn generate_files(comm: GenCommand) -> Result<GenerationResults> {
    log::debug!("Beginning single threaded recursive search");

    //Some stats
    let mut file_count: u128 = 0;
    let mut dir_count: u128 = 0;
    let start_time = Instant::now();

    //To recurse, build a queue of dir paths
    let mut dir_queue = VecDeque::new();
    dir_queue.push_back(PathBuf::from(comm.source_dir.clone()));

    let parser: Box<dyn ConfigParser> = Box::new(TomlParser::default());
    let generator: Box<dyn Generator> = match comm.doc_type {
        GenTypes::Markdown => Box::new(MarkdownGenerator::default()),
    };

    //Iteratively recurse using queue to buffer future directories to crawl.
    while let Some(path) = dir_queue.pop_front() {
        log::trace!("recurring new directory: {:?}", path);
        dir_count += 1;

        for file in std::fs::read_dir(path).context("Failure while opening directory")? {
            process_file(&comm, &mut dir_queue, &*parser, &*generator, file?)?;
            file_count += 1;
        }
    }

    log::trace!("queue size: {}", dir_queue.len());

    Ok(GenerationResults {
        files_read: file_count,
        dir_read: dir_count,
        duration: Instant::now().duration_since(start_time),
    })
}

pub struct GenerationResults {
    pub files_read: u128,
    pub dir_read: u128,
    pub duration: Duration,
}

/// Handles a single file in the generator.
pub fn process_file(
    comm: &GenCommand,
    dir_queue: &mut VecDeque<PathBuf>,
    parser: &dyn ConfigParser,
    generator: &dyn Generator,
    file: DirEntry,
) -> Result<()> {
    //If we find another dir to search, add to queue, then finish current dir
    if file.path().is_dir() && comm.recurse {
        log::trace!("adding dir to recurse: {:?}", file.path());
        dir_queue.push_back(file.path());
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
