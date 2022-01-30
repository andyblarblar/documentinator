use anyhow::Result;

use crate::doc_types::Doc;
use crate::parsers::Parser;
use crate::parsers::toml::TomlParser;

mod doc_types;
mod error;
mod generators;
mod parsers;

fn main() -> Result<()> {
    //TODO impliment CLI

    Ok(())
}
