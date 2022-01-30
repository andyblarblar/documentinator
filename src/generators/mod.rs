mod markdown;

use anyhow::Result;

use crate::Doc;

pub trait Generator: Default {
    /// Generates docs as a string for each node. The resulting doc vector has the same node order
    /// as the doc passed in.
    fn generate_string(&mut self, nodes: Doc) -> Result<Vec<String>>;
}
