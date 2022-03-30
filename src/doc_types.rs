//! Serializable documentation structures.

use serde_derive::{Deserialize, Serialize};

/// Top level Doc representation
#[derive(Deserialize, Serialize, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct Doc {
    /// Name of the package containing all of the nodes in this doc.
    pub package_name: String,
    /// Optional link to the repo containing this package.
    pub repo: Option<String>,
    /// The nodes in this package.
    pub nodes: Vec<Node>,
}

#[derive(Deserialize, Serialize, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct Node {
    /// Name of the node
    pub node_name: String,
    /// Source file(s) of the node
    pub source_file: Vec<String>,
    /// Summary of functionality
    pub summary: String,
    /// Description of potential improvements to the node
    pub potential_improvements: Option<String>,
    /// Anything else to be included in the documentation
    pub misc: Option<String>,
    /// Topics published to, if any
    pub publishes: Option<Vec<Topic>>,
    /// Topics subscribed to, if any
    pub subscribes: Option<Vec<Topic>>,
    /// Node Parameters
    pub params: Option<Vec<Param>>,
    /// Launchfile information
    pub launch: Option<Vec<LaunchInfo>>,
}

#[derive(Deserialize, Serialize, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct Topic {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct Param {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Default)]
pub struct LaunchInfo {
    pub file_path: String,
    pub usage: String,
    pub args: Option<Vec<Param>>,
}
