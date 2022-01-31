pub mod markdown;

use anyhow::Result;

use crate::Doc;

pub trait Generator {
    /// Generates docs as a string for each node. The resulting doc vector has the same node order
    /// as the doc passed in.
    ///
    /// Each vector element is (node name, doc)
    fn generate_string(&mut self, nodes: Doc) -> Result<Vec<(String, String)>>;
}
