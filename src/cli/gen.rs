//! Logic for the `gen` command.

use std::collections::{HashMap, VecDeque};
use std::fs::{DirEntry, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};

use crate::{ConfigParser, Doc, GenCommand, GenTypes, Generator, MarkdownGenerator, TomlParser};

/// Generates files through recursion
pub fn generate_files(comm: GenCommand) -> Result<GenerationResults> {
    log::debug!("Beginning single threaded recursive search");

    //Some stats
    let mut file_count: u128 = 0;
    let mut dir_count: u128 = 0;
    let start_time = Instant::now();
    let mut nodes_processed = Vec::new();

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
            process_file(
                &comm,
                &mut dir_queue,
                &*parser,
                &*generator,
                file?,
                &mut nodes_processed,
            )?;
            file_count += 1;
        }
    }

    log::trace!("queue size: {}", dir_queue.len());

    //Generate readme if user asked to
    if comm.readme && !nodes_processed.is_empty() {
        log::trace!("Creating README");
        println!("Generating NODES_README.md");

        generate_readme(&comm, &nodes_processed, &*generator)
            .context("Error while writing NODES_README.md")?;
    }

    Ok(GenerationResults {
        files_read: file_count,
        dir_read: dir_count,
        duration: Instant::now().duration_since(start_time),
        nodes_processed,
    })
}

pub struct GenerationResults {
    pub files_read: u128,
    pub dir_read: u128,
    pub duration: Duration,
    pub nodes_processed: Vec<Doc>,
}

/// Handles a single file in the generator.
///
/// Returns true if a document was created.
fn process_file(
    comm: &GenCommand,
    dir_queue: &mut VecDeque<PathBuf>,
    parser: &dyn ConfigParser,
    generator: &dyn Generator,
    file: DirEntry,
    nodes_processed: &mut Vec<Doc>,
) -> Result<bool> {
    //If we find another dir to search, add to queue, then finish current dir
    if file.path().is_dir() && comm.recurse {
        log::trace!("adding dir to recurse: {:?}", file.path());
        dir_queue.push_back(file.path());
        return Ok(false);
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
        nodes_processed.push(config.clone());
        let docs = generator.generate_string(config)?;
        log::debug!("Generated Doc");

        //Write doc
        for (name, doc) in docs {
            let mut path = PathBuf::from(comm.dest_dir.clone());
            path.push(generator.add_file_extension(&name));

            log::debug!("Writing to: {path:?}");

            let mut file = File::create(path)?;
            file.write_all(doc.as_bytes())?;

            println!("Created doc for {name}");
        }

        return Ok(true);
    }

    Ok(false)
}

/// Generates a readme file in the dest dir that contains a table of contents to the other node docs.
fn generate_readme(comm: &GenCommand, docs: &[Doc], generator: &dyn Generator) -> Result<()> {
    //Create file
    let mut path = PathBuf::from(comm.dest_dir.clone());
    path.push("NODES_README.md");
    let mut file = File::create(path)?;

    // Map all docs in the same package together, so we can print them in the same place
    let mut fused_docs: HashMap<&str, Vec<&Doc>> = HashMap::new();
    docs.iter().for_each(|doc| {
        fused_docs.entry(&doc.package_name).or_default().push(doc);
    });

    // Add a block for each package
    for (package, docs) in fused_docs.iter() {
        file.write_all(format!("# Nodes in package {}\n", package).as_bytes())?;

        // Link all nodes in package
        for doc in docs {
            for node in &doc.nodes {
                let mut file_path = PathBuf::from(comm.dest_dir.clone());
                file_path.push(generator.add_file_extension(&node.node_name));

                file.write_all(format!("- [{}](", &node.node_name).as_bytes())?;
                file.write_all(
                    file_path
                        .to_str()
                        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::Other))?
                        .as_bytes(),
                )?;
                file.write_all(&[b')', b'\n'])?;
            }
        }

        file.write_all(&[b'\n'])?;
    }
    Ok(())
}
