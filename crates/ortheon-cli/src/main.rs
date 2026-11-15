use ortheon_core::{Graph, degree_stats, weak_components};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let command = std::env::args().nth(1).unwrap_or_else(|| "demo".to_owned());
    match command.as_str() {
        "demo" => demo(),
        "inspect" => inspect(),
        "help" | "--help" | "-h" => {
            println!(
                "ORTHEON — universal structural computation\n\nUSAGE:\n  ortheon-cli [demo|inspect|help]\n\nCOMMANDS:\n  demo     Run a deterministic structural analysis example\n  inspect  Print the example graph's structural summary"
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
