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

/// Compute a shortest path by edge count using breadth-first search.
///
/// Directed graphs follow edge direction; undirected graphs traverse either
/// endpoint. Returns None when the target is unreachable.
pub fn shortest_path(graph: &Graph, start: NodeId, goal: NodeId) -> Result<Option<Vec<NodeId>>, GraphError> {
    let known: BTreeSet<NodeId> = graph.nodes().map(|(id, _)| id).collect();
    if !known.contains(&start) { return Err(GraphError::UnknownNode(start)); }
    if !known.contains(&goal) { return Err(GraphError::UnknownNode(goal)); }
    let mut predecessor = BTreeMap::new();
    let mut seen = BTreeSet::from([start]);
    let mut queue = VecDeque::from([start]);
    while let Some(current) = queue.pop_front() {
        if current == goal { break; }
        for edge in graph.edges() {
            let next = if graph.is_directed() {
                (edge.from == current).then_some(edge.to)
            } else if edge.from == current {
                Some(edge.to)
            } else if edge.to == current {
                Some(edge.from)
            } else { None };
            if let Some(next) = next {
                if seen.insert(next) {
                    predecessor.insert(next, current);
                    queue.push_back(next);
                }
            }
        }
    }
    if !seen.contains(&goal) { return Ok(None); }
    let mut path = vec![goal];
    let mut current = goal;
    while current != start {
        current = predecessor[&current];
        path.push(current);
    }
    path.reverse();
    Ok(Some(path))
}

/// Compute connected components of an undirected graph.
pub fn components(graph: &Graph) -> Result<Vec<Vec<NodeId>>, GraphError> {
    if graph.is_directed() { return Err(GraphError::RequiresUndirectedGraph); }
    component_walk(graph)
}

/// Compute weakly connected components of a directed graph.
///
/// Edge direction is ignored for component membership. This is not a strong
/// connected-components algorithm.
pub fn weak_components(graph: &Graph) -> Result<Vec<Vec<NodeId>>, GraphError> {
    if !graph.is_directed() { return Err(GraphError::RequiresDirectedGraph); }
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
                let next = if edge.from == current { Some(edge.to) }
                    else if edge.to == current { Some(edge.from) } else { None };
                if let Some(next) = next {
                    if unseen.remove(&next) { component.insert(next); queue.push_back(next); }
                }
            }
        }
        result.push(component.into_iter().collect());
    }
    Ok(result)
}


/// Compute strongly connected components of a directed graph with Kosaraju's algorithm.
pub fn strongly_connected_components(graph: &Graph) -> Result<Vec<Vec<NodeId>>, GraphError> {
    if !graph.is_directed() { return Err(GraphError::RequiresDirectedGraph); }
    graph.validate()?;
    let mut forward: BTreeMap<NodeId, Vec<NodeId>> = graph.nodes().map(|(id,_)| (id,Vec::new())).collect();
    let mut reverse = forward.clone();
    for edge in graph.edges() {
        forward.get_mut(&edge.from).expect("known endpoint").push(edge.to);
        reverse.get_mut(&edge.to).expect("known endpoint").push(edge.from);
    }
    let mut visited = BTreeSet::new();
    let mut finish = Vec::new();
    for &start in forward.keys() {
        if visited.contains(&start) { continue; }
        let mut stack = vec![(start, false)];
        while let Some((node, exiting)) = stack.pop() {
            if exiting { finish.push(node); continue; }
            if !visited.insert(node) { continue; }
            stack.push((node, true));
            for &next in forward[&node].iter().rev() {
                if !visited.contains(&next) { stack.push((next, false)); }
            }
        }
    }
    visited.clear();
    let mut result = Vec::new();
    for start in finish.into_iter().rev() {
        if !visited.insert(start) { continue; }
        let mut component = BTreeSet::from([start]);
        let mut stack = vec![start];
        while let Some(node) = stack.pop() {
            for &next in reverse[&node].iter().rev() {
                if visited.insert(next) { component.insert(next); stack.push(next); }
            }
        }
        result.push(component.into_iter().collect());
    }
    result.sort();
    Ok(result)
}

/// Return a bipartition of an undirected graph, or None when it is not bipartite.
pub fn bipartition(graph: &Graph) -> Result<Option<(Vec<NodeId>, Vec<NodeId>)>, GraphError> {
    if graph.is_directed() { return Err(GraphError::RequiresUndirectedGraph); }
    graph.validate()?;
    let mut adjacency: BTreeMap<NodeId, Vec<NodeId>> = graph.nodes().map(|(id,_)| (id,Vec::new())).collect();
    for edge in graph.edges() {
        adjacency.get_mut(&edge.from).expect("known endpoint").push(edge.to);
        if edge.from != edge.to { adjacency.get_mut(&edge.to).expect("known endpoint").push(edge.from); }
    }
    let mut colors = BTreeMap::new();
    for &start in adjacency.keys() {
        if colors.contains_key(&start) { continue; }
        colors.insert(start, false);
        let mut queue = VecDeque::from([start]);
        while let Some(node) = queue.pop_front() {
            let color = colors[&node];
            for &next in &adjacency[&node] {
                match colors.get(&next) {
                    Some(&other) if other == color => return Ok(None),
                    Some(_) => {},
                    None => { colors.insert(next, !color); queue.push_back(next); }
                }
            }
        }
    }
    let mut left = Vec::new();
    let mut right = Vec::new();
    for (node, color) in colors { if color { right.push(node); } else { left.push(node); } }
    Ok(Some((left, right)))
}

/// Construct the vertex-induced subgraph on the supplied vertex identifiers.
pub fn induced_subgraph(graph: &Graph, vertices: impl IntoIterator<Item=NodeId>) -> Result<Graph, GraphError> {
    graph.validate()?;
    let selected: BTreeSet<NodeId> = vertices.into_iter().collect();
    let labels: BTreeMap<NodeId, &str> = graph.nodes().collect();
    for &node in &selected {
        if !labels.contains_key(&node) { return Err(GraphError::UnknownNode(node)); }
    }
    let mut result = if graph.is_directed() { Graph::directed() } else { Graph::undirected() };
    for &node in &selected { result.add_node(node, labels[&node])?; }
    for edge in graph.edges() {
        if selected.contains(&edge.from) && selected.contains(&edge.to) {
            result.add_edge(edge.from, edge.to)?;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn strongly_connected_components_find_cycles_and_singletons() {
        let mut graph = Graph::directed();
        for id in 1..=6 { graph.add_node(id, "").unwrap(); }
        for (a,b) in [(1,2),(2,1),(2,3),(3,4),(4,3),(4,5)] { graph.add_edge(a,b).unwrap(); }
        assert_eq!(strongly_connected_components(&graph).unwrap(),
            vec![vec![1,2], vec![3,4], vec![5], vec![6]]);
    }

    #[test]
    fn bipartition_accepts_even_cycle_and_rejects_odd_cycle() {
        let mut graph = Graph::undirected();
        for id in 1..=4 { graph.add_node(id, "").unwrap(); }
        for (a,b) in [(1,2),(2,3),(3,4),(4,1)] { graph.add_edge(a,b).unwrap(); }
        assert!(bipartition(&graph).unwrap().is_some());
        graph.add_edge(1,3).unwrap();
        assert_eq!(bipartition(&graph).unwrap(), None);
    }

    #[test]
    fn induced_subgraph_preserves_labels_and_internal_edges() {
        let mut graph = Graph::directed();
        graph.add_node(1, "one").unwrap(); graph.add_node(2, "two").unwrap();
        graph.add_node(3, "three").unwrap();
        graph.add_edge(1,2).unwrap(); graph.add_edge(2,3).unwrap();
        let sub = induced_subgraph(&graph, [1,2]).unwrap();
        assert_eq!((sub.node_count(), sub.edge_count()), (2,1));
        assert_eq!(sub.nodes().map(|(id,label)| (id,label.to_string())).collect::<Vec<_>>(),
            vec![(1,"one".into()),(2,"two".into())]);
    }

    #[test]
    fn undirected_components_include_isolated_vertices() {
        let mut graph = Graph::undirected();
        for id in 1..=4 { graph.add_node(id, id.to_string()).unwrap(); }
        graph.add_edge(1, 2).unwrap(); graph.add_edge(2, 3).unwrap();
        assert_eq!(components(&graph).unwrap(), vec![vec![1, 2, 3], vec![4]]);
    }

    #[test]
    fn directed_weak_components_ignore_direction() {
        let mut graph = Graph::directed();
        for id in 1..=3 { graph.add_node(id, "").unwrap(); }
        graph.add_edge(2, 1).unwrap();
        assert_eq!(weak_components(&graph).unwrap(), vec![vec![1, 2], vec![3]]);
    }

    #[test]
    fn degree_stats_count_undirected_self_loop_twice() {
        let mut graph = Graph::undirected();
        graph.add_node(1, "a").unwrap(); graph.add_node(2, "b").unwrap();
        graph.add_edge(1, 1).unwrap(); graph.add_edge(1, 2).unwrap();
        let stats = degree_stats(&graph);
        assert_eq!((stats.minimum, stats.maximum, stats.total), (1, 3, 4));
    }

    #[test]
    fn shortest_path_respects_direction_and_finds_minimum_edges() {
        let mut graph = Graph::directed();
        for id in 1..=4 { graph.add_node(id, "").unwrap(); }
        graph.add_edge(1, 2).unwrap(); graph.add_edge(2, 4).unwrap();
        graph.add_edge(1, 3).unwrap(); graph.add_edge(3, 4).unwrap();
        assert_eq!(shortest_path(&graph, 1, 4).unwrap(), Some(vec![1, 2, 4]));
        assert_eq!(shortest_path(&graph, 4, 1).unwrap(), None);
    }

    #[test]
    fn shortest_path_returns_singleton_for_same_vertex() {
        let mut graph = Graph::undirected();
        graph.add_node(9, "only").unwrap();
        assert_eq!(shortest_path(&graph, 9, 9).unwrap(), Some(vec![9]));
    }
}
