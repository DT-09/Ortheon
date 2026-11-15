//! Deterministic structural representations and algorithms.
//!
//! The initial kernel models finite directed and undirected labeled graphs.

mod algorithms;
mod error;
mod graph;

pub use algorithms::{components, degree_stats, weak_components, DegreeStats};
pub use error::GraphError;
pub use graph::{Edge, Graph, NodeId};

/// Version of the canonical graph encoding used by Graph::fingerprint.
pub const FINGERPRINT_FORMAT_VERSION: u16 = 1;
