//! Config file parsers.

use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;

use crate::doc_types::Doc;

pub mod toml;

/// Interface for structs that can parse docTor configs.
pub trait ConfigParser {
    /// Parses config from string.
    fn parse_str(&self, string: String) -> Result<Doc>;
    /// Parses config from file at path.
    fn parse_path(&self, path: PathBuf) -> Result<Doc>;
    /// Parses config from file.
    fn parse_file(&self, file: File) -> Result<Doc>;
    /// Returns true if passed filename is a valid config for this parser.
    fn match_filename(&self, filename: &str) -> bool;
}
