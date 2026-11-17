//! Deterministic structural representations and algorithms.

mod algorithms;
mod error;
mod graph;
mod hypergraph;
mod relation;

pub use algorithms::{Bipartition, DegreeStats, InferredEdge, SpanningForest, bipartition, complement, components, degree_stats, from_adjacency_matrix, induced_subgraph, infer_missing_edges, minimum_spanning_forest, shortest_path, shortest_weighted_path, strongly_connected_components, weak_components};
pub use error::GraphError;
pub use graph::{Edge, Graph, NodeId};
pub use hypergraph::{HyperedgeId, Hypergraph, HypergraphError};
pub use relation::{FiniteRelation, RelationError};

/// Version of the canonical graph encoding used by Graph::fingerprint.
pub const FINGERPRINT_FORMAT_VERSION: u16 = 1;
