# serde_json

**Status:** approved (runtime dependency, vsdd-core and vsdd) — retrofit record
**Approved:** 2026-09-27 (retrofit). The crate has been a runtime dependency since the first JSON surface (workspace pin `1`); no record and no three-lens review accompanied it — the `VSDD-E0100` condition the contract's Dependency approval member names. This record closes the verification debt; the process breach stands as recorded on vsdd-cli #872.
**Approved by:** retrofit under vsdd-cli #872 (Platform Engineer owner, Security validator), the three lenses recorded below

## What it is

[`serde_json`](https://crates.io/crates/serde_json) — the JSON data format for serde: parser, `Value` tree, and serialiser. Maintained by David Tolnay under serde-rs.

## Why we need it

The machine forms are JSON: `vsdd status --machine`, the snapshot, the init manifest with its SHA-256 entries, the diagnostics envelope, the tracker's `--json` output the acquisition layer reads. The schema check validates `serde_json::Value` instances, and the JSON Schema files themselves are parsed with it.

## Why this crate

- The canonical serde JSON implementation; `jsonschema` takes `serde_json::Value`, so any other JSON crate would need a conversion layer.
- Default features only (`std`); no `preserve_order`, no `arbitrary_precision`, no `float_roundtrip`.

## Scope

- Runtime dependency of `vsdd-core` and `vsdd`, workspace requirement `1`, locked at 1.0.150.
- Consumers: six modules in `vsdd-core/src` (`text`, `schema_check`, `diagnostics`, `init`, `snapshot/acquire`, `registry`), four in `vsdd/src`, and the integration tests of both crates.

## Three-lens review (retrofit)

**Solution Owner — scope.** In scope: the contract fixes JSON as the machine form and JSON Schema as the data-set schema language; this is their implementation.

**Platform Engineer — supply chain.** Subtree: 4 unique crates (`serde`, `itoa`, `memchr`, and the float formatter). Build: no build-script network access, no proc-macros of its own. Maintenance: continuous, semver-stable `1.x` since 2017.

**Security — CVE, licence, threat.** `cargo audit` on 2026-09-27 against the current lockfile: no advisory against this crate or its subtree. The workspace's one allowed warning is RUSTSEC-2026-0190 (unsound `Error::downcast_mut` in the transitive `anyhow`, reached only through `jsonschema`). Licence: MIT OR Apache-2.0. Threat: the parser reads tracker output (`crosslink issue list --json`) and repository files; it is written in safe Rust with a recursion limit (default 128) that bounds deeply nested input, and the acquisition layer treats what it parses as data, never as instructions. Historical advisories against `serde_json` are none; the crate is among the most exercised parsers in the ecosystem.

## Alternatives considered

- `simd-json` — faster, larger, `unsafe`-heavy; the toolkit parses kilobytes, not gigabytes.
- `json`/`tinyjson` — no serde integration; would force a conversion layer for `jsonschema`.
