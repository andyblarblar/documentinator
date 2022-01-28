use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;

use crate::doc_types::{Doc, LaunchInfo, Node, Topic};
use crate::parsers::Parser;
use crate::parsers::toml_parser::TomlParser;

mod doc_types;
mod error;
mod parsers;

fn main() -> Result<()> {

    Ok(())
}
