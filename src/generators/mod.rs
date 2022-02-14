//! Documentation generators.

pub mod markdown;

use anyhow::Result;

use crate::Doc;

pub trait Generator {
    /// Generates docs as a string for each node. The resulting doc vector has the same node order
    /// as the doc passed in.
    ///
    /// Each vector element is (node name, doc)
    fn generate_string(&self, nodes: Doc) -> Result<Vec<(String, String)>>;

    /// Adds the file extension of the document type this generator creates to the passed filename.
    fn add_file_extension(&self, filename: &str) -> String;
}
