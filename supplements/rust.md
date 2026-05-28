---
supplement_slug: rust
languages_or_interfaces: [Rust]
domains_in_scope: [software-engineer, quality-engineer, platform-engineer, performance-engineer, security, solution-architect]
extensions: []
---

# Rust Supplement

Per-domain extensions for Rust projects. Loads alongside the domain prompts in scope when the project declares Rust as a primary language.

## Software Engineer extensions

- **Ownership + lifetime discipline.** Lifetimes named explicitly when elided forms surprise the reader. Lifetime parameters in public API are part of the contract; semver-major bump required to change them.
- **Error-as-value.** Functions return `Result<T, E>` over panic-on-error. `unwrap` / `expect` reserved for "this cannot fail per the type system" — every use is justified in a comment OR the function returns `Result`.
- **Type-driven design.** `Option` over null-ish patterns; `Result` over error-codes; enum-with-variants over flag-tuples. Make illegal states unrepresentable.
- **Build profiles.** `[profile.release]` settings declared (codegen-units, lto, opt-level) when ship-binary performance matters. Profile choices are part of the build contract.

## Quality Engineer extensions

- **`cargo test` discipline.** Per-function unit tests in `#[cfg(test)]` modules + integration tests in `tests/`. Naming convention: `test_<function>_<scenario>` makes failing-test name read like a behavioral claim.
- **`proptest` / `quickcheck` for property-based testing.** Phase 5 surface A. Properties express DESIGN.md invariants; default 1000-case budget.
- **`cargo-mutants` for Mutation Testing.** Phase 5 surface B. Per-mutant disposition required for accepted-code paths.
- **`cargo-fuzz` for Fuzz Testing.** Phase 5 surface C. Requires nightly toolchain; libFuzzer-based.
- **`kani` for Proof Execution.** Phase 5 surface D (optional; capstone+ intent only). Bounded model checking; verifies properties for all inputs up to configured bound.
- **`criterion` for performance regression testing.** Per-benchmark history tracked; regression-against-baseline blocks merge above declared threshold.

## Platform Engineer extensions

- **`Cargo.toml` workspace discipline.** Single-crate-single-binary or multi-crate workspace per project shape. Workspace-level `[workspace.package]` for shared metadata.
- **`Cargo.lock` committed.** Reproducible builds; pinned versions including transitive deps.
- **`rust-toolchain.toml` pinned.** Toolchain version + components (`clippy`, `rustfmt`) declared. CI installs from this file.
- **`cargo audit` + `cargo deny`.** Supply-chain audit at every PR; CVE detection; license compliance.
- **Cross-platform builds.** Per-platform targets at GitHub Release time (Linux x86_64 + aarch64, macOS x86_64 + aarch64). Reproducible via pinned toolchain.
- **`cargo clippy --deny warnings`.** CI gate; clippy lints surface idiomatic-Rust drift.

## Performance Engineer extensions

- **Allocator selection.** Default system allocator vs `jemalloc` / `mimalloc` for allocation-heavy workloads — declared in `Cargo.toml` when load-bearing.
- **`flamegraph` + `perf` profiling.** Linux-side profiling; macOS uses Instruments. Per-hot-path profile committed when load-bearing.
- **Inlining hints.** `#[inline]` + `#[inline(always)]` reserved for measured cases. Compiler usually chooses correctly; manual hints justify themselves with benchmark deltas.
- **`async`/`await` runtime choice.** `tokio` vs `async-std` vs `smol` — declared in `Cargo.toml`; runtime choice is part of architecture.

## Security extensions

- **`unsafe` block discipline.** Every `unsafe` block carries a `SAFETY:` comment naming the invariants the unsafe code relies on. `unsafe` in public API surface routes via Phase 4 to Phase 1a+1b for explicit declaration.
- **`cargo audit` integration.** CI-side CVE detection. Per the dependency-approval discipline, new entries in `Cargo.toml` require investigation; transitive-dep CVEs trigger Phase 4 routing.
- **Memory-safety wins are not security-completeness wins.** Rust eliminates memory-safety bugs by default; logic bugs, supply-chain attacks, side-channel attacks remain in scope.

## Solution Architect extensions

- **Module + crate boundaries.** Per-crate API surface is the architectural seam. Re-exports (`pub use`) make seams explicit; deep re-exports across crate boundaries are the maintainability gap.
- **Trait + generic discipline.** Traits define behavioral contracts; generic bounds make requirements visible. `dyn Trait` reserved for cases where monomorphization is wasteful or impractical.
