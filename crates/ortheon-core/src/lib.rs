//! Deterministic structural representations and algorithms.

mod algorithms;
mod error;
mod graph;
mod hypergraph;
mod relation;

pub use algorithms::{DegreeStats, bipartition, components, degree_stats, induced_subgraph, shortest_path, strongly_connected_components, weak_components};
pub use error::GraphError;
pub use graph::{Edge, Graph, NodeId};
pub use hypergraph::{HyperedgeId, Hypergraph, HypergraphError};
pub use relation::{FiniteRelation, RelationError};

/// Version of the canonical graph encoding used by Graph::fingerprint.
pub const FINGERPRINT_FORMAT_VERSION: u16 = 1;
