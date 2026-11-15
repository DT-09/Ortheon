# Contributing

ORTHEON is a research-oriented computational project. Contributions should improve correctness, clarity or demonstrated capability.

## Before proposing an algorithm
1. State the mathematical problem.
2. Define input and output semantics.
3. Identify preconditions and failure cases.
4. Provide a complexity analysis.
5. Identify an established baseline or explain why none applies.

## Development checks
~~~bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
~~~

## Code expectations
- Keep core APIs small and explicit.
- Avoid hidden nondeterminism.
- Do not silently coerce between distinct mathematical domains.
- Return typed errors for invalid operations.
- Document public APIs and add tests with every behavior change.
- Never describe a planned feature as implemented.

Research novelty requires literature review and reproducible evidence.
