# Research agenda

ORTHEON's long-term ambition is a common computational research architecture for multiple structural domains. The following topics are research directions, not implemented features.

## Representation theory
Which abstractions can be shared across relations, graphs, hypergraphs and algebraic structures? How can semantic information be preserved during conversion? Which canonical forms are useful and computable?

## Structural comparison
Which equivalence relation is relevant to a domain? When is exact equivalence tractable? Which invariants are useful filters, and which are complete certificates?

## Reconstruction and inference
Which structures are consistent with incomplete observations? How should ambiguity be represented rather than hidden? Which assumptions make an inferred relationship identifiable?

## Dynamics
How can discrete structural transitions be composed? Which properties remain invariant? How can stability be characterized for a specified model?

## Optimization
Which structural objectives admit exact algorithms? Which decompositions reduce practical search complexity? How should multi-objective trade-offs be represented?

## Formal foundations
Begin with the graph model, its invariants and selected algorithms. Proofs must correspond to executable definitions or clearly state the refinement relationship.

## Research discipline
Every proposed result requires a precise problem statement, definitions and assumptions, related work, reproducible methods where relevant, counterexamples, limitations and evidence for independent review.


## Implemented finite-kernel increment (2026-11-17)

The executable baseline now includes unweighted shortest paths by breadth-first search, finite relation domain and range, inverse and composition, additional relation predicates, and distinct transitive versus reflexive-transitive closure operations. These additions deepen the graph and relation substrate; they do not resolve the open research questions above.


## Implemented research increment (2026-11-17, continued)

The graph kernel now includes iterative strongly connected components, undirected bipartition, induced-subgraph extraction, adjacency-matrix reconstruction, simple-graph complement, common-neighbor missing-edge ranking, and minimum spanning forests using Kruskal's algorithm. Abstract relation-algebra lemmas are specified in Coq under `formal/RelationAlgebra.v`. These proofs concern the abstract relational definitions; they do not yet prove refinement of the Rust implementation.


## Implemented weighted optimization increment

The graph layer now includes Dijkstra minimum-cost paths for nonnegative integer weights, alongside Kruskal minimum spanning forests. Both algorithms require explicit edge-weight inputs and reject missing weights rather than inventing costs.
