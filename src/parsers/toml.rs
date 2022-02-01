use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;

use crate::doc_types::Doc;
use crate::parsers::ConfigParser;

/// Parses configs from toml markup files.
#[derive(Default)]
pub struct TomlParser {}

impl ConfigParser for TomlParser {
    fn parse_str(&self, string: String) -> Result<Doc> {
        toml::from_str(&string).context("Failed to parse TOML string.")
    }

    fn parse_path(&self, path: PathBuf) -> Result<Doc> {
        let file =
            File::open(path.clone()).context(format!("Cannot open File at path {:?}", path))?;

        self.parse_file(file)
    }

    fn parse_file(&self, mut file: File) -> Result<Doc> {
        let mut string = String::new();
        file.read_to_string(&mut string)
            .context("Error while reading file".to_string())?;

        self.parse_str(string)
    }

    fn match_filename(&self, filename: &str) -> bool {
        filename.ends_with("doctor.toml")
    }
}

#[cfg(test)]
mod test {
    use crate::{ConfigParser, TomlParser};

    #[test]
    fn test_toml_parse() {
        let wld = include_str!("../../test_assets/white_line_detection.doctor.toml");
        let parser = TomlParser::default();

        parser.parse_str(wld.to_string()).unwrap();
    }
}
