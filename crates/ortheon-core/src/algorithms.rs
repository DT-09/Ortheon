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
    let mut adjacency: BTreeMap<NodeId, Vec<NodeId>> =
        graph.nodes().map(|(id,_)| (id,Vec::new())).collect();
    for edge in graph.edges() {
        adjacency.get_mut(&edge.from).expect("known endpoint").push(edge.to);
        if !graph.is_directed() && edge.from != edge.to {
            adjacency.get_mut(&edge.to).expect("known endpoint").push(edge.from);
        }
    }
    if !adjacency.contains_key(&start) { return Err(GraphError::UnknownNode(start)); }
    if !adjacency.contains_key(&goal) { return Err(GraphError::UnknownNode(goal)); }
    let mut predecessor = BTreeMap::new();
    let mut seen = BTreeSet::from([start]);
    let mut queue = VecDeque::from([start]);
    while let Some(current) = queue.pop_front() {
        if current == goal { break; }
        for &next in &adjacency[&current] {
            if seen.insert(next) {
                predecessor.insert(next,current);
                queue.push_back(next);
            }
        }
    }
    if !seen.contains(&goal) { return Ok(None); }
    let mut path = vec![goal];
    let mut current = goal;
    while current != start { current = predecessor[&current]; path.push(current); }
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
    let mut adjacency: BTreeMap<NodeId, Vec<NodeId>> =
        graph.nodes().map(|(id,_)| (id,Vec::new())).collect();
    for edge in graph.edges() {
        adjacency.get_mut(&edge.from).expect("known endpoint").push(edge.to);
        if edge.from != edge.to {
            adjacency.get_mut(&edge.to).expect("known endpoint").push(edge.from);
        }
    }
    let mut unseen: BTreeSet<NodeId> = adjacency.keys().copied().collect();
    let mut result = Vec::new();
    while let Some(start) = unseen.pop_first() {
        let mut component = BTreeSet::from([start]);
        let mut queue = VecDeque::from([start]);
        while let Some(current) = queue.pop_front() {
            for &next in &adjacency[&current] {
                if unseen.remove(&next) {
                    component.insert(next);
                    queue.push_back(next);
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
        visited.insert(start);
        let mut stack = vec![(start, 0_usize)];
        while !stack.is_empty() {
            let next = {
                let (node, index) = stack.last_mut().expect("non-empty traversal stack");
                if *index < forward[node].len() {
                    let next = forward[node][*index];
                    *index += 1;
                    Some(next)
                } else {
                    None
                }
            };
            if let Some(next) = next {
                if visited.insert(next) { stack.push((next, 0)); }
            } else {
                finish.push(stack.pop().expect("non-empty traversal stack").0);
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

/// Two disjoint vertex sets forming a bipartition.
pub type Bipartition = (Vec<NodeId>, Vec<NodeId>);

/// Return a bipartition of an undirected graph, or None when it is not bipartite.
pub fn bipartition(graph: &Graph) -> Result<Option<Bipartition>, GraphError> {
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


/// Candidate edge inferred from local common-neighbor evidence.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InferredEdge {
    /// First endpoint (always smaller than second).
    pub from: NodeId,
    /// Second endpoint.
    pub to: NodeId,
    /// Number of shared neighbors supporting the candidate.
    pub common_neighbors: usize,
}

/// Infer absent undirected edges supported by at least the requested number of common neighbors.
///
/// This is a structural heuristic, not a causal or probabilistic claim.
pub fn infer_missing_edges(graph: &Graph, minimum_common_neighbors: usize) -> Result<Vec<InferredEdge>, GraphError> {
    if graph.is_directed() { return Err(GraphError::RequiresUndirectedGraph); }
    graph.validate()?;
    let mut neighbors: BTreeMap<NodeId, BTreeSet<NodeId>> =
        graph.nodes().map(|(id,_)| (id,BTreeSet::new())).collect();
    for edge in graph.edges() {
        if edge.from != edge.to {
            neighbors.get_mut(&edge.from).expect("known node").insert(edge.to);
            neighbors.get_mut(&edge.to).expect("known node").insert(edge.from);
        }
    }
    let ids: Vec<NodeId> = neighbors.keys().copied().collect();
    let mut candidates = Vec::new();
    for i in 0..ids.len() {
        for j in i+1..ids.len() {
            let a = ids[i];
            let b = ids[j];
            if neighbors[&a].contains(&b) { continue; }
            let count = neighbors[&a].intersection(&neighbors[&b]).count();
            if count >= minimum_common_neighbors {
                candidates.push(InferredEdge { from:a, to:b, common_neighbors:count });
            }
        }
    }
    candidates.sort_by(|a,b| b.common_neighbors.cmp(&a.common_neighbors)
        .then_with(|| a.from.cmp(&b.from)).then_with(|| a.to.cmp(&b.to)));
    Ok(candidates)
}

/// Reconstruct a graph from a square adjacency matrix.
///
/// Vertex identifiers are matrix indices and labels are their decimal forms.
/// Undirected matrices must be symmetric. Diagonal true entries create self-loops.
pub fn from_adjacency_matrix(matrix: &[Vec<bool>], directed: bool) -> Result<Graph, GraphError> {
    let n = matrix.len();
    if matrix.iter().any(|row| row.len() != n) {
        return Err(GraphError::InvalidRepresentation("adjacency matrix must be square".into()));
    }
    if !directed {
        for i in 0..n {
            for j in 0..n {
                if matrix[i][j] != matrix[j][i] {
                    return Err(GraphError::InvalidRepresentation("undirected adjacency matrix must be symmetric".into()));
                }
            }
        }
    }
    let mut graph = if directed { Graph::directed() } else { Graph::undirected() };
    for id in 0..n { graph.add_node(id as NodeId, id.to_string())?; }
    for (i,row) in matrix.iter().enumerate() {
        for (j,&present) in row.iter().enumerate() {
            if present && (directed || i <= j) { graph.add_edge(i as NodeId,j as NodeId)?; }
        }
    }
    Ok(graph)
}

/// Return the simple-graph complement of an undirected graph.
pub fn complement(graph: &Graph) -> Result<Graph, GraphError> {
    if graph.is_directed() { return Err(GraphError::RequiresUndirectedGraph); }
    graph.validate()?;
    let mut result = Graph::undirected();
    let ids: Vec<(NodeId, String)> =
        graph.nodes().map(|(id, label)| (id, label.to_owned())).collect();
    for (id,label) in &ids { result.add_node(*id,label)?; }
    for i in 0..ids.len() {
        for j in i+1..ids.len() {
            let (a,b) = (ids[i].0,ids[j].0);
            if !graph.edges().any(|e| e.from == a && e.to == b) {
                result.add_edge(a,b)?;
            }
        }
    }
    Ok(result)
}

/// A minimum spanning forest represented by selected edges and total weight.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpanningForest {
    /// Edges selected by Kruskal's algorithm, in deterministic selection order.
    pub edges: Vec<crate::Edge>,
    /// Sum of selected edge weights.
    pub total_weight: u64,
}

/// Compute a minimum spanning forest with Kruskal's algorithm.
///
/// The weight map uses canonical (smaller, larger) endpoint pairs. Every graph
/// edge must have a weight. Negative weights are not supported by this u64 API.
pub fn minimum_spanning_forest(
    graph: &Graph,
    weights: &BTreeMap<(NodeId,NodeId),u64>,
) -> Result<SpanningForest,GraphError> {
    if graph.is_directed() { return Err(GraphError::RequiresUndirectedGraph); }
    graph.validate()?;
    let mut weighted = Vec::new();
    for edge in graph.edges() {
        let key = (edge.from.min(edge.to),edge.from.max(edge.to));
        let weight = *weights.get(&key).ok_or_else(|| GraphError::InvalidRepresentation(
            format!("missing weight for edge {}-{}",key.0,key.1)))?;
        weighted.push((weight,key.0,key.1));
    }
    weighted.sort();
    let mut parent: BTreeMap<NodeId,NodeId> = graph.nodes().map(|(id,_)| (id,id)).collect();
    let mut rank: BTreeMap<NodeId,usize> = graph.nodes().map(|(id,_)| (id,0)).collect();
    fn root(parent: &mut BTreeMap<NodeId,NodeId>, mut node: NodeId) -> NodeId {
        while parent[&node] != node {
            let grand = parent[&parent[&node]];
            parent.insert(node,grand);
            node = grand;
        }
        node
    }
    let mut selected = Vec::new();
    let mut total_weight = 0_u64;
    for (weight,a,b) in weighted {
        if a == b { continue; }
        let ra = root(&mut parent,a);
        let rb = root(&mut parent,b);
        if ra == rb { continue; }
        let (low,high) = if rank[&ra] < rank[&rb] {(ra,rb)} else {(rb,ra)};
        parent.insert(low,high);
        if rank[&ra] == rank[&rb] {
            *rank.get_mut(&high).expect("known root") += 1;
        }
        selected.push(crate::Edge {from:a,to:b});
        total_weight = total_weight.checked_add(weight).ok_or_else(|| GraphError::InvalidRepresentation("spanning forest weight overflow".into()))?;
    }
    Ok(SpanningForest {edges:selected,total_weight})
}


/// Compute a minimum-weight path using Dijkstra's algorithm.
///
/// Directed weights use ordered (from, to) keys. Undirected weights use
/// canonical (min, max) keys. Every traversed edge must have a weight.
/// Zero weights are supported; negative weights are excluded by the u64 type.
pub fn shortest_weighted_path(
    graph: &Graph,
    start: NodeId,
    goal: NodeId,
    weights: &BTreeMap<(NodeId, NodeId), u64>,
) -> Result<Option<(Vec<NodeId>, u64)>, GraphError> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut adjacency: BTreeMap<NodeId, Vec<(NodeId, u64)>> =
        graph.nodes().map(|(id, _)| (id, Vec::new())).collect();
    if !adjacency.contains_key(&start) { return Err(GraphError::UnknownNode(start)); }
    if !adjacency.contains_key(&goal) { return Err(GraphError::UnknownNode(goal)); }
    for edge in graph.edges() {
        let key = if graph.is_directed() { (edge.from, edge.to) }
            else { (edge.from.min(edge.to), edge.from.max(edge.to)) };
        let weight = *weights.get(&key).ok_or_else(|| GraphError::InvalidRepresentation(
            format!("missing weight for edge {}-{}", key.0, key.1)))?;
        adjacency.get_mut(&edge.from).expect("known node").push((edge.to, weight));
        if !graph.is_directed() && edge.from != edge.to {
            adjacency.get_mut(&edge.to).expect("known node").push((edge.from, weight));
        }
    }
    let mut distance = BTreeMap::from([(start, 0_u64)]);
    let mut previous = BTreeMap::new();
    let mut queue = BinaryHeap::from([Reverse((0_u64, start))]);
    while let Some(Reverse((cost, node))) = queue.pop() {
        if distance.get(&node) != Some(&cost) { continue; }
        if node == goal { break; }
        for &(next, weight) in &adjacency[&node] {
            let candidate = cost.checked_add(weight).ok_or_else(|| GraphError::InvalidRepresentation(
                "weighted path cost overflow".into()))?;
            if distance.get(&next).is_none_or(|&known| candidate < known) {
                distance.insert(next, candidate);
                previous.insert(next, node);
                queue.push(Reverse((candidate, next)));
            }
        }
    }
    let Some(&cost) = distance.get(&goal) else { return Ok(None); };
    let mut path = vec![goal];
    let mut current = goal;
    while current != start { current = previous[&current]; path.push(current); }
    path.reverse();
    Ok(Some((path, cost)))
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
    fn adjacency_matrix_reconstructs_graph_and_rejects_invalid_shapes() {
        let graph = from_adjacency_matrix(&[vec![false,true],vec![true,false]], false).unwrap();
        assert_eq!((graph.node_count(),graph.edge_count()),(2,1));
        assert!(from_adjacency_matrix(&[vec![false,true],vec![false]],true).is_err());
        assert!(from_adjacency_matrix(&[vec![false,true],vec![false,false]],false).is_err());
    }

    #[test]
    fn complement_contains_exactly_missing_simple_edges() {
        let mut graph = Graph::undirected();
        for id in 1..=3 { graph.add_node(id,"").unwrap(); }
        graph.add_edge(1,2).unwrap();
        let inverse = complement(&graph).unwrap();
        assert_eq!(inverse.edge_count(),2);
        assert!(inverse.edges().any(|e| e.from==1 && e.to==3));
        assert!(inverse.edges().any(|e| e.from==2 && e.to==3));
    }

    #[test]
    fn common_neighbor_inference_ranks_triangle_closure() {
        let mut graph = Graph::undirected();
        for id in 1..=3 { graph.add_node(id,"").unwrap(); }
        graph.add_edge(1,2).unwrap(); graph.add_edge(2,3).unwrap();
        assert_eq!(infer_missing_edges(&graph,1).unwrap(),
            vec![InferredEdge{from:1,to:3,common_neighbors:1}]);
    }

    #[test]
    fn kruskal_returns_minimum_spanning_forest() {
        let mut graph = Graph::undirected();
        for id in 1..=4 { graph.add_node(id,"").unwrap(); }
        for (a,b) in [(1,2),(2,3),(1,3)] { graph.add_edge(a,b).unwrap(); }
        let weights = BTreeMap::from([((1,2),4),((2,3),2),((1,3),9)]);
        let forest = minimum_spanning_forest(&graph,&weights).unwrap();
        assert_eq!(forest.total_weight,6);
        assert_eq!(forest.edges.len(),2);
        assert!(minimum_spanning_forest(&graph,&BTreeMap::new()).is_err());
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
