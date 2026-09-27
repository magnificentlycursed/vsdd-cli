# serde

**Status:** approved (runtime dependency, vsdd-core and vsdd) — retrofit record
**Approved:** 2026-09-27 (retrofit). The crate has been a runtime dependency since the first data structure was serialised (workspace pin `1`, feature `derive`); no record and no three-lens review accompanied it — the `VSDD-E0100` condition the contract's Dependency approval member names. This record closes the verification debt; the process breach stands as recorded on vsdd-cli #872.
**Approved by:** retrofit under vsdd-cli #872 (Platform Engineer owner, Security validator), the three lenses recorded below

## What it is

[`serde`](https://crates.io/crates/serde) — the Rust serialisation framework: `Serialize`/`Deserialize` traits and (with the `derive` feature) the proc-macro derives. Maintained by David Tolnay under the serde-rs organisation; the de facto standard, used by essentially every Rust program that reads or writes structured data.

## Why we need it

Every structured artefact the toolkit reads or writes — the registry data sets, the deviation register, `state.yaml`, snapshot JSON, the machine-form status envelope, the init manifest — is a typed Rust struct with derived `Deserialize`/`Serialize`. The typed boundary is what lets the schema check, the answer layer and the status envelope share one definition instead of hand-parsing maps.

## Why this crate

- There is no alternative at this layer; `serde_json`, `serde_yaml_ng`, `jsonschema` and `clap` all build on it.
- `derive` is the only feature enabled: no `rc`, no `alloc`-only configuration, no unstable features.

## Scope

- Runtime dependency of `vsdd-core` and `vsdd`, workspace requirement `1`, locked at 1.0.228.
- Consumers: eight modules in `vsdd-core/src` (`init`, `snapshot/*`, `answer/*`, `state/schema`, `registry/*`, `diagnostics`) and two in `vsdd/src`; tests throughout.

## Three-lens review (retrofit)

**Solution Owner — scope.** In scope by construction: the contract's data plumbing and machine-form requirements presume typed (de)serialisation, and the derive macros are the narrowest way to keep the type and its wire form in one place.

**Platform Engineer — supply chain.** Subtree: 8 unique crates (`serde_core`, `serde_derive`, `proc-macro2`, `quote`, `syn`, `unicode-ident`), all from the same maintainer group or the Rust project's orbit. Build: proc-macro codegen at compile time (the derives); no build-script network access. Maintenance: continuous, semver-stable on the `1.x` line since 2017.

**Security — CVE, licence, threat.** `cargo audit` on 2026-09-27 against the current lockfile: no advisory against this crate or its subtree. The workspace's one allowed warning is RUSTSEC-2026-0190 (unsound `Error::downcast_mut` in the transitive `anyhow`, reached only through `jsonschema`). Licence: MIT OR Apache-2.0. Threat: `serde` itself performs no I/O and holds no parser; the attack surface lives in the format crates (`serde_json`, `serde_yaml_ng`, their own records). The 2023 precompiled-binary episode in `serde_derive` (1.0.172–1.0.184) was reverted upstream; the locked 1.0.228 builds the derive from source.

## Alternatives considered

- None at this layer. `miniserde`/`nanoserde` trade the derive ecosystem for size and would cut off every format crate the toolkit uses.
