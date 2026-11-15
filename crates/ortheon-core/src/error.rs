use std::fmt;

/// Errors returned by graph construction, validation and analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphError {
    /// A node identifier already exists.
    DuplicateNode(u64),
    /// An edge references a node that is not present.
    MissingNode(u64),
    /// A requested node is not present.
    UnknownNode(u64),
    /// The operation requires a directed graph.
    RequiresDirectedGraph,
    /// The operation requires an undirected graph.
    RequiresUndirectedGraph,
    /// A directed cycle prevents topological ordering.
    CycleDetected,
    /// The stored representation violates an invariant.
    InvalidRepresentation(String),
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateNode(id) => write!(f, "node {id} already exists"),
            Self::MissingNode(id) => write!(f, "edge references missing node {id}"),
            Self::UnknownNode(id) => write!(f, "unknown node {id}"),
            Self::RequiresDirectedGraph => write!(f, "operation requires a directed graph"),
            Self::RequiresUndirectedGraph => write!(f, "operation requires an undirected graph"),
            Self::CycleDetected => write!(f, "directed cycle detected; topological order does not exist"),
            Self::InvalidRepresentation(message) => write!(f, "invalid graph representation: {message}"),
        }
    }
}

impl std::error::Error for GraphError {}
