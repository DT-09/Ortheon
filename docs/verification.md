# Verification and evidence

## Current validation
- Unit tests for local algorithm behavior.
- Integration tests for public API contracts.
- CI checks for formatting, compilation, tests and Clippy.
- Deterministic ordered data structures.
- Explicit error cases for invalid operations and directed cycles.

## Required checks for new algorithms
- Mathematical contract and preconditions.
- Complexity analysis.
- Boundary-case and property-based tests where useful.
- Differential tests against an independent implementation when available.
- Determinism tests when deterministic behavior is promised.
- Known limitations and counterexamples.

## Formal verification
Coq is a planned research dependency, not currently part of the executable build. Formal verification should be introduced after a concrete specification and refinement target exist.

## Benchmark policy
No performance claim should be published without versioned benchmark code, input methodology, dataset sizes and distributions, hardware and toolchain details, baselines, repeated measurements and uncertainty reporting.


## Formal relation algebra

`formal/RelationAlgebra.v` specifies abstract relations and proves inverse involution, composition associativity and identity laws, plus reflexivity, edge inclusion and transitivity of reflexive-transitive closure. CI compiles this file with Coq. These are algebra-level proofs, not end-to-end proofs of Rust code correctness.
