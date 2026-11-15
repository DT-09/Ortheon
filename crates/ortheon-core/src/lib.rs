//! Deterministic structural representations and algorithms.
//!
//! The initial kernel models finite directed and undirected labeled graphs.

mod algorithms;
mod error;
mod graph;
mod hypergraph;
mod relation;

pub use algorithms::{DegreeStats, components, degree_stats, weak_components};
pub use error::GraphError;
pub use graph::{Edge, Graph, NodeId};
pub use hypergraph::{HyperedgeId, Hypergraph, HypergraphError};
pub use relation::{FiniteRelation, RelationError};

/// Version of the canonical graph encoding used by Graph::fingerprint.
pub const FINGERPRINT_FORMAT_VERSION: u16 = 1;
