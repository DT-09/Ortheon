# ORTHEON

**Universal Structural Computation**

The science of representing, analyzing and verifying structure.

ORTHEON is a Rust-first research framework. Its initial executable foundation is a deterministic graph model with validation, traversal, connectivity analysis, topological ordering and canonical fingerprints. The broader programme includes relations, hypergraphs, transformations, dynamics, inference, optimization and formal verification; those extensions are not claimed as implemented.

## Quick start

```bash
cargo test --workspace
cargo run -p ortheon-cli -- demo
```

## Current capabilities

- Directed and undirected labeled graphs.
- Typed errors and representation validation.
- Deterministic reachability and component analysis.
- Degree statistics and Kahn topological ordering.
- Versioned SHA-256 fingerprint of canonical labeled graph encoding.
- Rust CLI demonstration and automated CI.

A fingerprint is not a graph-isomorphism certificate. Directed weak connectivity is not strong connectivity. See `docs/` for algorithm contracts, architecture, verification policy and research boundaries.

## Research principle

A capability is complete only when its semantics, limitations, tests and evidence are documented. Planned research is kept separate from executable functionality.

MIT License. See LICENSE.