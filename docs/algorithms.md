# Algorithm contracts

## Graph representation

Finite labeled directed and undirected graphs use stable unsigned 64-bit vertex identifiers, UTF-8 labels, unique edges and deterministic ordered storage.

### Reachability

Breadth-first traversal from a supplied vertex. Directed graphs follow outgoing edges; undirected graphs traverse either endpoint. Results include the start vertex and are sorted by identifier. The current edge-set scan implementation is O(VE) worst case.

### Components

Undirected connected components and directed weakly connected components are computed by breadth-first traversal. Weak components ignore edge direction. Strongly connected components are not currently implemented. The current edge-set scan implementation is O(VE) worst case.

### Degree statistics

The current summary reports minimum, maximum and total incident degree. An undirected self-loop contributes two. Directed in-degree and out-degree sums are reported separately.

### Topological ordering

Kahn's algorithm returns a deterministic ordering for a directed acyclic graph. A cycle returns an explicit error. The smallest available vertex identifier is selected first. Complexity is O((V + E) log V) with the ordered ready set.

### Graph fingerprint

A versioned canonical encoding includes directedness, ordered vertex identifiers and labels, and ordered edges. SHA-256 is used. The fingerprint is not invariant under vertex renaming and is not a graph-isomorphism algorithm.

## Hypergraph representation

A hypergraph is a finite labeled vertex set plus uniquely identified, non-empty hyperedges. Each hyperedge is a set of vertices and can represent an n-ary relationship directly. Multiple hyperedges may have identical member sets when their identifiers differ.

### Incidence and degree

Incidence count is the sum of hyperedge cardinalities. Vertex degree is the number of hyperedges containing that vertex.

### Connected components

Two vertices are connected when they are linked by a chain of shared hyperedges. Isolated vertices remain singleton components. The algorithm builds a vertex-to-hyperedge incidence index and visits each hyperedge at most once; complexity is O(V + I), where I is the number of incidences.

### Hypergraph fingerprint

The versioned canonical encoding includes ordered vertex identifiers and labels, hyperedge identifiers and ordered member identifiers. It is identity-sensitive, not isomorphism-invariant.

## Finite binary relations

A finite relation is defined over an explicit carrier set. Insertion rejects elements outside that carrier.

Implemented predicates:
- Reflexivity over the entire carrier.
- Symmetry.
- Transitivity.

The inverse relation swaps every ordered pair. Reflexive transitive closure uses a finite Warshall algorithm in O(n^3) time and O(n^2) memory.

The relation type does not assume that its input is an order, equivalence relation or function. Those properties must be checked explicitly.

## Complexity notation

V is the number of graph or hypergraph vertices, E the number of graph edges, I the number of hypergraph incidences, and n the size of a finite relation's carrier. Complexity statements describe the current implementation, not benchmark results.


## Strongly connected components

Directed strongly connected components use iterative Kosaraju traversal. The implementation avoids recursive call-stack growth and returns deterministically sorted components. Complexity is O(V + E).

## Bipartition

Undirected bipartition uses breadth-first two-coloring. Odd cycles and self-loops are rejected by returning no partition. Isolated vertices are retained. Complexity is O(V + E).

## Induced subgraphs and complements

An induced subgraph retains selected vertices, their labels, directedness and every edge internal to the selected set. Unknown vertex identifiers are errors. The simple-graph complement is defined for undirected graphs and excludes self-loops.

## Reconstruction and local inference

Adjacency-matrix reconstruction requires a square matrix; undirected matrices must be symmetric. Vertex IDs are matrix indices. Common-neighbor inference ranks absent undirected edges by shared-neighbor count. This is a transparent structural heuristic, not causal discovery or a calibrated probability.

## Minimum spanning forest

Kruskal's algorithm consumes a caller-supplied map of canonical endpoint pairs to nonnegative integer weights. Missing edge weights are errors. Disconnected inputs produce a minimum spanning forest. Complexity is O(E log E + V α(V)).

