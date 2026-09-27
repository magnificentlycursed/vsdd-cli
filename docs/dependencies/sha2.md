# sha2

**Status:** approved (runtime dependency, vsdd-core — promoted from dev-dependency; see Scope update)
**Approved:** 2026-06-02
**Approved by:** operator-directive (vsdd-cli Phase 2a Red Gate for vsdd-core::init)

## What it is

[`sha2`](https://crates.io/crates/sha2) — pure-Rust implementations of the SHA-2 family of hash functions, maintained by `RustCrypto`.

## Why we need it

The Phase 2a Red Gate for `vsdd_core::init` asserts that `init-manifest.json` records a correct SHA-256 hash for every deployed file. The test helper computes the actual SHA-256 of each deployed file and compares it against the manifest entry. Without a SHA-256 implementation the test cannot make this assertion.

## Why this crate

- `RustCrypto` is the canonical Rust home for cryptographic primitives; widely used in the supply chain audit-relevant ecosystem (rustls, x509-parser, ed25519-dalek all use it).
- No `unsafe` blocks reachable from the SHA-256 path under default features.
- Pure-Rust; no C dependencies; no build-script network access; no `proc-macro` codegen.
- Active maintenance; tagged releases with semver discipline.

## Scope

- Dev-dependency in `vsdd-core` only. Not added to the runtime dependency tree at this commit.
- Phase 2b implementation of `vsdd_core::init` will also need SHA-256 (for the manifest write path). That promotes `sha2` to a runtime dependency in Phase 2b and is noted here so the upgrade doesn't require a separate approval cycle.

## Alternatives considered

- `ring` — broader cryptographic surface than we need; brings in a C dependency.
- Hand-rolled SHA-256 — out of scope; cryptographic primitives should not be in-house.
- `sha1` only — insufficient collision resistance for a content-integrity check.

## Scope update (2026-09-27, vsdd-cli #872)

The record above was written when `sha2` was a dev-dependency of the Phase 2a Red Gate. The promotion it anticipated happened: `vsdd-core/Cargo.toml` lists `sha2 = "0.10"` under `[dependencies]`, and `vsdd_core::init` (`vsdd-core/src/init.rs`, `use sha2::{Digest, Sha256}`) computes each deployed file's SHA-256 at run time to compare against the init manifest's recorded hash before `--accept-managed-defaults` overwrites a file. That is the same computation the Red Gate test asserts, now performed by the product on the operator's behalf. Locked at 0.10.9; RustCrypto's `sha2` carries documented `unsafe` in its SIMD backends (29 hits in 0.10.9), all behind CPU-feature detection, and has no advisory against it (`cargo audit` 2026-09-27). Scope: one runtime consumer, `vsdd_core::init`; the review recorded above stands for the promoted scope.
