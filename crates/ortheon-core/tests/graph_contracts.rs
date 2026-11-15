//! Integration tests for the public graph API and its behavioral contracts.

use ortheon_core::{Graph, GraphError};

#[test]
fn fingerprint_is_stable_for_identical_labeled_models() {
    fn build() -> Graph {
        let mut graph = Graph::directed();
        graph.add_node(1, "alpha").unwrap();
        graph.add_node(2, "beta").unwrap();
        graph.add_edge(1, 2).unwrap();
        graph
    }
    assert_eq!(build().fingerprint(), build().fingerprint());
}

#[test]
fn undirected_edge_insertion_is_order_independent() {
    let mut left = Graph::undirected();
    let mut right = Graph::undirected();
    for graph in [&mut left, &mut right] {
        graph.add_node(1, "a").unwrap();
        graph.add_node(2, "b").unwrap();
    }
    left.add_edge(1, 2).unwrap();
    right.add_edge(2, 1).unwrap();
    assert_eq!(left.fingerprint(), right.fingerprint());
}

#[test]
fn invalid_edge_endpoint_is_rejected() {
    let mut graph = Graph::directed();
    graph.add_node(1, "a").unwrap();
    assert_eq!(graph.add_edge(1, 9), Err(GraphError::MissingNode(9)));
}

#[test]
fn duplicate_node_is_rejected() {
    let mut graph = Graph::directed();
    graph.add_node(1, "a").unwrap();
    assert_eq!(graph.add_node(1, "b"), Err(GraphError::DuplicateNode(1)));
}

#[test]
fn topological_order_rejects_cycles() {
    let mut graph = Graph::directed();
    graph.add_node(1, "a").unwrap();
    graph.add_node(2, "b").unwrap();
    graph.add_edge(1, 2).unwrap();
    graph.add_edge(2, 1).unwrap();
    assert_eq!(graph.topological_order(), Err(GraphError::CycleDetected));
}

#[test]
fn topological_order_is_deterministic() {
    let mut graph = Graph::directed();
    for id in [4, 2, 3, 1] {
        graph.add_node(id, "").unwrap();
    }
    graph.add_edge(1, 3).unwrap();
    graph.add_edge(2, 3).unwrap();
    graph.add_edge(3, 4).unwrap();
    assert_eq!(graph.topological_order().unwrap(), vec![1, 2, 3, 4]);
}

#[test]
fn reachability_respects_direction() {
    let mut graph = Graph::directed();
    for id in 1..=3 {
        graph.add_node(id, "").unwrap();
    }
    graph.add_edge(1, 2).unwrap();
    graph.add_edge(3, 2).unwrap();
    assert_eq!(graph.reachable_from(1).unwrap(), vec![1, 2]);
}

#[test]
fn empty_graph_has_valid_zero_degree_statistics() {
    let graph = Graph::undirected();
    graph.validate().unwrap();
    assert_eq!(ortheon_core::degree_stats(&graph).total, 0);
}
