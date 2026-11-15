use crate::{Graph, GraphError, NodeId};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// Degree summary for a graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DegreeStats {
    /// Minimum degree, or zero for an empty graph.
    pub minimum: usize,
    /// Maximum degree, or zero for an empty graph.
    pub maximum: usize,
    /// Sum of all vertex degrees.
    pub total: usize,
    /// Number of vertices.
    pub vertices: usize,
    /// In-degree and out-degree sums for directed graphs.
    pub directed_totals: Option<(usize, usize)>,
}

/// Compute degree statistics in O(V + E).
pub fn degree_stats(graph: &Graph) -> DegreeStats {
    let mut degrees: BTreeMap<NodeId, usize> = graph.nodes().map(|(id, _)| (id, 0)).collect();
    let mut in_total = 0;
    let mut out_total = 0;
    for edge in graph.edges() {
        if graph.is_directed() {
            *degrees.get_mut(&edge.from).expect("edge endpoint exists") += 1;
            *degrees.get_mut(&edge.to).expect("edge endpoint exists") += 1;
            out_total += 1;
            in_total += 1;
        } else if edge.from == edge.to {
            *degrees.get_mut(&edge.from).expect("edge endpoint exists") += 2;
        } else {
            *degrees.get_mut(&edge.from).expect("edge endpoint exists") += 1;
            *degrees.get_mut(&edge.to).expect("edge endpoint exists") += 1;
        }
    }
    let values: Vec<usize> = degrees.values().copied().collect();
    DegreeStats {
        minimum: values.iter().copied().min().unwrap_or(0),
        maximum: values.iter().copied().max().unwrap_or(0),
        total: values.iter().sum(),
        vertices: values.len(),
        directed_totals: graph.is_directed().then_some((in_total, out_total)),
    }
}

/// Compute connected components of an undirected graph.
pub fn components(graph: &Graph) -> Result<Vec<Vec<NodeId>>, GraphError> {
    if graph.is_directed() {
        return Err(GraphError::RequiresUndirectedGraph);
    }
    component_walk(graph)
}

/// Compute weakly connected components of a directed graph.
///
/// Edge direction is ignored for component membership. This is not a strong
/// connected-components algorithm.
pub fn weak_components(graph: &Graph) -> Result<Vec<Vec<NodeId>>, GraphError> {
    if !graph.is_directed() {
        return Err(GraphError::RequiresDirectedGraph);
    }
    component_walk(graph)
}

fn component_walk(graph: &Graph) -> Result<Vec<Vec<NodeId>>, GraphError> {
    graph.validate()?;
    let mut unseen: BTreeSet<NodeId> = graph.nodes().map(|(id, _)| id).collect();
    let mut result = Vec::new();
    while let Some(start) = unseen.pop_first() {
        let mut component = BTreeSet::from([start]);
        let mut queue = VecDeque::from([start]);
        while let Some(current) = queue.pop_front() {
            for edge in graph.edges() {
                let next = if edge.from == current {
                    Some(edge.to)
                } else if edge.to == current {
                    Some(edge.from)
                } else {
                    None
                };
                if let Some(next) = next {
                    if unseen.remove(&next) {
                        component.insert(next);
                        queue.push_back(next);
                    }
                }
            }
        }
        result.push(component.into_iter().collect());
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undirected_components_include_isolated_vertices() {
        let mut graph = Graph::undirected();
        for id in 1..=4 {
            graph.add_node(id, id.to_string()).unwrap();
        }
        graph.add_edge(1, 2).unwrap();
        graph.add_edge(2, 3).unwrap();
        assert_eq!(components(&graph).unwrap(), vec![vec![1, 2, 3], vec![4]]);
    }

    #[test]
    fn directed_weak_components_ignore_direction() {
        let mut graph = Graph::directed();
        for id in 1..=3 {
            graph.add_node(id, "").unwrap();
        }
        graph.add_edge(2, 1).unwrap();
        assert_eq!(weak_components(&graph).unwrap(), vec![vec![1, 2], vec![3]]);
    }

    #[test]
    fn degree_stats_count_undirected_self_loop_twice() {
        let mut graph = Graph::undirected();
        graph.add_node(1, "a").unwrap();
        graph.add_node(2, "b").unwrap();
        graph.add_edge(1, 1).unwrap();
        graph.add_edge(1, 2).unwrap();
        let stats = degree_stats(&graph);
        assert_eq!((stats.minimum, stats.maximum, stats.total), (1, 3, 4));
    }
}
