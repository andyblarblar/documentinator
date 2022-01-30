use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;

use crate::doc_types::Doc;

pub mod toml;

pub trait Parser: Default {
    fn parse_str(&mut self, string: String) -> Result<Doc>;
    fn parse_path(&mut self, path: PathBuf) -> Result<Doc>;
    fn parse_file(&mut self, file: File) -> Result<Doc>;
}
