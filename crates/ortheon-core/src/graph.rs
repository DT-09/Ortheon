use crate::GraphError;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// Stable identifier for a vertex.
pub type NodeId = u64;

/// An edge represented by its endpoint identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edge {
    /// Source endpoint for directed graphs; first endpoint otherwise.
    pub from: NodeId,
    /// Target endpoint for directed graphs; second endpoint otherwise.
    pub to: NodeId,
}

/// A finite labeled graph with deterministic storage and iteration.
///
/// Undirected edges are normalized so that from <= to. Parallel edges are
/// not represented: adding an existing edge is an idempotent operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph {
    directed: bool,
    nodes: BTreeMap<NodeId, String>,
    edges: BTreeSet<(NodeId, NodeId)>,
}

impl Graph {
    /// Create an empty directed graph.
    #[must_use]
    pub fn directed() -> Self {
        Self::new(true)
    }

    /// Create an empty undirected graph.
    #[must_use]
    pub fn undirected() -> Self {
        Self::new(false)
    }

    fn new(directed: bool) -> Self {
        Self {
            directed,
            nodes: BTreeMap::new(),
            edges: BTreeSet::new(),
        }
    }

    /// Return whether this graph is directed.
    #[must_use]
    pub const fn is_directed(&self) -> bool {
        self.directed
    }

    /// Number of vertices.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Number of edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Iterate over vertices in ascending identifier order.
    pub fn nodes(&self) -> impl Iterator<Item = (NodeId, &str)> {
        self.nodes.iter().map(|(id, label)| (*id, label.as_str()))
    }

    /// Iterate over edges in canonical order.
    pub fn edges(&self) -> impl Iterator<Item = Edge> + '_ {
        self.edges.iter().map(|(from, to)| Edge {
            from: *from,
            to: *to,
        })
    }

    /// Insert a vertex with a stable identifier and label.
    pub fn add_node(&mut self, id: NodeId, label: impl Into<String>) -> Result<(), GraphError> {
        if self.nodes.contains_key(&id) {
            return Err(GraphError::DuplicateNode(id));
        }
        self.nodes.insert(id, label.into());
        Ok(())
    }

    /// Insert an edge. Re-inserting an existing edge has no effect.
    pub fn add_edge(&mut self, from: NodeId, to: NodeId) -> Result<(), GraphError> {
        if !self.nodes.contains_key(&from) {
            return Err(GraphError::MissingNode(from));
        }
        if !self.nodes.contains_key(&to) {
            return Err(GraphError::MissingNode(to));
        }
        let edge = if self.directed || from <= to {
            (from, to)
        } else {
            (to, from)
        };
        self.edges.insert(edge);
        Ok(())
    }

    /// Validate representation invariants.
    pub fn validate(&self) -> Result<(), GraphError> {
        for (from, to) in &self.edges {
            if !self.nodes.contains_key(from) {
                return Err(GraphError::InvalidRepresentation(format!(
                    "edge source {from} is absent"
                )));
            }
            if !self.nodes.contains_key(to) {
                return Err(GraphError::InvalidRepresentation(format!(
                    "edge target {to} is absent"
                )));
            }
            if !self.directed && from > to {
                return Err(GraphError::InvalidRepresentation(
                    "undirected edge is not normalized".into(),
                ));
            }
        }
        Ok(())
    }

    /// Return a SHA-256 digest of the canonical labeled graph encoding.
    ///
    /// This is an identity fingerprint, not an isomorphism-invariant hash:
    /// vertex identifiers and labels are significant.
    #[must_use]
    pub fn fingerprint(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"ORTHEON-GRAPH\0");
        hasher.update(crate::FINGERPRINT_FORMAT_VERSION.to_be_bytes());
        hasher.update([u8::from(self.directed)]);
        hasher.update((self.nodes.len() as u64).to_be_bytes());
        for (id, label) in &self.nodes {
            hasher.update(id.to_be_bytes());
            hasher.update((label.len() as u64).to_be_bytes());
            hasher.update(label.as_bytes());
        }
        hasher.update((self.edges.len() as u64).to_be_bytes());
        for (from, to) in &self.edges {
            hasher.update(from.to_be_bytes());
            hasher.update(to.to_be_bytes());
        }
        format!("{:x}", hasher.finalize())
    }

    /// Return nodes reachable from start, including start.
    ///
    /// Directed graphs follow edge direction. Undirected graphs traverse
    /// either endpoint. Results are returned in ascending identifier order.
    pub fn reachable_from(&self, start: NodeId) -> Result<Vec<NodeId>, GraphError> {
        if !self.nodes.contains_key(&start) {
            return Err(GraphError::UnknownNode(start));
        }
        let mut seen = BTreeSet::from([start]);
        let mut queue = VecDeque::from([start]);
        while let Some(current) = queue.pop_front() {
            for (from, to) in &self.edges {
                let next = if self.directed {
                    if *from == current { Some(*to) } else { None }
                } else if *from == current {
                    Some(*to)
                } else if *to == current {
                    Some(*from)
                } else {
                    None
                };
                if let Some(next) = next {
                    if seen.insert(next) {
                        queue.push_back(next);
                    }
                }
            }
        }
        Ok(seen.into_iter().collect())
    }

    /// Return a deterministic topological ordering of a directed acyclic graph.
    ///
    /// Kahn's algorithm is used. Returns CycleDetected if a directed cycle
    /// exists and RequiresDirectedGraph for an undirected graph.
    pub fn topological_order(&self) -> Result<Vec<NodeId>, GraphError> {
        if !self.directed {
            return Err(GraphError::RequiresDirectedGraph);
        }
        let mut indegree: BTreeMap<NodeId, usize> = self.nodes.keys().map(|id| (*id, 0)).collect();
        let mut outgoing: BTreeMap<NodeId, Vec<NodeId>> = BTreeMap::new();
        for (from, to) in &self.edges {
            *indegree.get_mut(to).expect("validated edge endpoint") += 1;
            outgoing.entry(*from).or_default().push(*to);
        }
        let mut ready: BTreeSet<NodeId> = indegree
            .iter()
            .filter_map(|(id, degree)| (*degree == 0).then_some(*id))
            .collect();
        let mut order = Vec::with_capacity(self.nodes.len());
        while let Some(node) = ready.pop_first() {
            order.push(node);
            if let Some(neighbors) = outgoing.get(&node) {
                for neighbor in neighbors {
                    let degree = indegree.get_mut(neighbor).expect("validated edge endpoint");
                    *degree -= 1;
                    if *degree == 0 {
                        ready.insert(*neighbor);
                    }
                }
            }
        }
        if order.len() == self.nodes.len() {
            Ok(order)
        } else {
            Err(GraphError::CycleDetected)
        }
    }
}
