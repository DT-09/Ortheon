//! Command-line interface for running deterministic ORTHEON demonstrations.

use ortheon_core::{Graph, degree_stats, shortest_path, weak_components};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let command = std::env::args().nth(1).unwrap_or_else(|| "demo".to_owned());
    match command.as_str() {
        "demo" => demo(),
        "inspect" => inspect(),
        "analyze" => analyze_file(),
        "help" | "--help" | "-h" => {
            println!(
                "ORTHEON — universal structural computation\n\nUSAGE:\n  ortheon-cli [demo|inspect|analyze <edge-list.txt> [directed|undirected]|help]\n\nCOMMANDS:\n  demo     Run a deterministic structural analysis example\n  inspect  Print the example graph's structural summary"
            );
            Ok(())
        }
        other => Err(format!("unknown command '{other}'; run with --help").into()),
    }
}

fn example_graph() -> Result<Graph, Box<dyn Error>> {
    let mut graph = Graph::directed();
    graph.add_node(10, "observation")?;
    graph.add_node(20, "model")?;
    graph.add_node(30, "analysis")?;
    graph.add_node(40, "result")?;
    graph.add_node(50, "independent")?;
    graph.add_edge(10, 20)?;
    graph.add_edge(20, 30)?;
    graph.add_edge(30, 40)?;
    Ok(graph)
}

fn demo() -> Result<(), Box<dyn Error>> {
    let graph = example_graph()?;
    println!("ORTHEON / STRUCTURAL COMPUTATION");
    println!("--------------------------------");
    println!("vertices: {}", graph.node_count());
    println!("edges:    {}", graph.edge_count());
    println!("fingerprint (SHA-256): {}", graph.fingerprint());
    println!("topological order: {:?}", graph.topological_order()?);
    println!("reachable from 10:   {:?}", graph.reachable_from(10)?);
    println!("shortest path 10→40: {:?}", shortest_path(&graph, 10, 40)?);
    println!("weak components:    {:?}", weak_components(&graph)?);
    println!("degree statistics:  {:?}", degree_stats(&graph));
    Ok(())
}

fn inspect() -> Result<(), Box<dyn Error>> {
    let graph = example_graph()?;
    graph.validate()?;
    println!("directed: {}", graph.is_directed());
    println!("nodes:");
    for (id, label) in graph.nodes() {
        println!("  {id}: {label}");
    }
    println!("edges:");
    for edge in graph.edges() {
        println!("  {} -> {}", edge.from, edge.to);
    }
    println!("fingerprint: {}", graph.fingerprint());
    Ok(())
}


fn analyze_file() -> Result<(), Box<dyn Error>> {
    use std::collections::BTreeSet;
    use std::fs;

    let mut args = std::env::args().skip(2);
    let path = args.next().ok_or("usage: ortheon-cli analyze <edge-list.txt> [directed|undirected]")?;
    let directed = match args.next().as_deref().unwrap_or("undirected") {
        "directed" => true,
        "undirected" => false,
        _ => return Err("mode must be directed or undirected".into()),
    };
    let input = fs::read_to_string(&path)?;
    let mut ids = BTreeSet::new();
    let mut edges = Vec::new();
    for (index, raw) in input.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() { continue; }
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() != 2 {
            return Err(format!("{}:{}: expected two vertex IDs", path, index + 1).into());
        }
        let from: u64 = fields[0].parse()?;
        let to: u64 = fields[1].parse()?;
        ids.insert(from);
        ids.insert(to);
        edges.push((from, to));
    }
    let mut graph = if directed { Graph::directed() } else { Graph::undirected() };
    for id in ids { graph.add_node(id, id.to_string())?; }
    for (from, to) in edges { graph.add_edge(from, to)?; }
    graph.validate()?;
    println!("vertices: {}", graph.node_count());
    println!("edges: {}", graph.edge_count());
    println!("fingerprint: {}", graph.fingerprint());
    println!("degree statistics: {:?}", degree_stats(&graph));
    if directed {
        println!("weak components: {:?}", weak_components(&graph)?);
        println!("strong components: {:?}", ortheon_core::strongly_connected_components(&graph)?);
        println!("topological order: {:?}", graph.topological_order());
    } else {
        println!("components: {:?}", ortheon_core::components(&graph)?);
        println!("bipartition: {:?}", ortheon_core::bipartition(&graph)?);
        println!("inferred missing edges: {:?}", ortheon_core::infer_missing_edges(&graph, 1)?);
    }
    Ok(())
}
