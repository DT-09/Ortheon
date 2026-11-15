# Architecture

## Dependency direction

~~~text
ortheon-cli
    |
    v
ortheon-core
    |
    +-- graph representation
    +-- typed errors
    +-- deterministic algorithms
~~~

The core must not depend on the CLI, UI, network services or research notebooks. Future domain crates may depend on the core, but the core must not depend on those extensions.

## Current kernel

The initial kernel is a finite labeled graph:
- Vertex identifiers are unsigned 64-bit integers.
- Vertex labels are UTF-8 strings.
- Directedness is a graph-level property.
- Edges are unique.
- Undirected edges are stored in normalized endpoint order.
- Ordered maps and sets provide deterministic iteration.

This is a deliberately explicit first representation, not a universal representation for every mathematical object.

## Extension boundaries

A hypergraph must not be represented as an ordinary graph by silently losing n-ary incidence. Algebraic structures must retain their operations and axioms. Temporal structures must model time explicitly.

Common traits should be introduced only when at least two real implementations demonstrate a stable shared contract.
