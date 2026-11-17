# Architecture

## Current computational substrate

ORTHEON currently supports three distinct finite structural models:

1. **Graph:** directed or undirected binary edges over labeled vertices.
2. **Hypergraph:** n-ary hyperedges over labeled vertices, represented as incidence sets.
3. **Finite binary relation:** ordered pairs over an explicit finite carrier.

These models share a Rust workspace and deterministic conventions. They do not share a forced universal representation. This is intentional: converting an n-ary relation to pairwise edges or treating an arbitrary relation as an order can destroy semantics.

## Dependency direction

~~~text
ortheon-cli
    |
    v
ortheon-core
    +-- graph
    +-- hypergraph
    +-- finite relation
    +-- typed errors and deterministic algorithms
~~~

The core must not depend on the CLI, UI, network services or research notebooks.

## Graph model

- Vertex identifiers are unsigned 64-bit integers.
- Vertex labels are UTF-8 strings.
- Directedness is a graph-level property.
- Edges are unique.
- Undirected edges are normalized by endpoint order.

## Hypergraph model

- Vertex identifiers and hyperedge identifiers are unsigned 64-bit integers.
- Each hyperedge is a non-empty set of vertices.
- Hyperedges have stable identities.
- Distinct hyperedges may have equal member sets.
- Incidence is represented directly.

## Finite relation model

- The carrier set is explicit and finite.
- Every pair must contain two carrier elements.
- Pairs are ordered; symmetry is not assumed.
- Relation properties are predicates, not implicit type guarantees.

## Extension boundaries

Future algebraic structures must retain their operations and axioms. Temporal structures must model time explicitly. Common traits should be introduced only when multiple implementations demonstrate a stable shared contract.


## Algorithms added to the finite kernel

- Breadth-first shortest path returns a minimum-edge path, follows direction for directed graphs, and returns `None` when no path exists.
- Finite relation composition is defined only for identical carrier sets.
- Relation predicates include reflexivity, irreflexivity, symmetry, antisymmetry, asymmetry, transitivity, equivalence-relation recognition and partial-order recognition.
- Transitive closure and reflexive-transitive closure are separate operations. The former does not add identity pairs except where a non-empty cycle implies them.

These are deterministic reference implementations. They are not yet formally verified or benchmark-optimized.
