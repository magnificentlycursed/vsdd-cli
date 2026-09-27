# jsonschema

**Status:** approved (runtime dependency, vsdd-core) — retrofit record
**Approved:** 2026-09-27 (retrofit). The crate has been a runtime dependency since the schema validation surface was built (workspace pin `0.18`, `default-features = false`, feature `draft202012`); no record and no three-lens review accompanied it — the `VSDD-E0100` condition the contract's Dependency approval member names. This record closes the verification debt; the process breach stands as recorded on vsdd-cli #872.
**Approved by:** retrofit under vsdd-cli #872 (Platform Engineer owner, Security validator), the three lenses recorded below

## What it is

[`jsonschema`](https://crates.io/crates/jsonschema) — a JSON Schema validator (drafts 4 through 2020-12) by Dmitry Dygalo, compiled once into a `JSONSchema` and applied to `serde_json::Value` instances.

## Why we need it

The toolkit ships its data sets with JSON Schema pairs (`vsdd-core/schemas/*.json`) and refuses malformed data at the boundary rather than deep in a consumer. `vsdd_core::schema_check` compiles each bundled schema under draft 2020-12 and validates the parsed YAML/JSON before any registry or state code sees it. Hand-rolled validation would re-implement a standard badly.

## Why this crate

- The only maintained Rust validator with complete draft 2020-12 support at the time of adoption; `boon` has since matured and is the alternative to evaluate if the subtree cost becomes a problem.
- `default-features = false` drops the crate's defaults `resolve-http` (which pulls `reqwest`), `resolve-file` and `cli` (which pulls `clap`): no network capability and no file-system resolver is compiled in, matching the closed-world posture of the rest of the toolkit. Only `draft202012` is enabled.
- Deterministic: schemas are compiled from bytes bundled in the binary; the validator holds no I/O.

## Scope

- Runtime dependency of `vsdd-core`, workspace requirement `0.18`, locked at 0.18.3.
- One consumer: `vsdd_core::schema_check` (`vsdd-core/src/schema_check.rs`). A second consumer re-enters review.
- The `0.18` line is superseded upstream (the crate's API was reworked from 0.20 onward: `Validator`, new options builder). Moving off `0.18` is a major-version bump and re-enters review on its own.

## Three-lens review (retrofit)

**Solution Owner — scope.** In scope: data sets carry schema pairs by contract (the Data plumbing requirement), and validating them at load is the narrowest correct implementation. The crate is the largest single subtree in the workspace, which is the cost accepted for a standards-complete validator; a Solution Owner call is owed only if `boon` or a slimmer feature set can hold the same guarantee.

**Platform Engineer — supply chain.** Subtree: 95 unique crates under the normal edge (the largest in the workspace), including `fancy-regex`/`regex` (pattern keywords), `url`/`idna` and the 13-crate ICU4X tree (`format: uri`/`idn-*` — the path that made `icu_properties` transitive before #813), `num-*` and `fraction` (multipleOf), `time`/`iso8601` (date-time formats), `uuid`, `ahash`, `parking_lot`, `anyhow`. Build: no build scripts with network access; proc-macro codegen only through `serde_derive`/`syn`. Maintenance: active, frequent releases; the `0.18` line itself receives no further releases.

**Security — CVE, licence, threat.** `cargo audit` on 2026-09-27 against the current lockfile: no advisory against this crate or its subtree. The workspace's one allowed warning is RUSTSEC-2026-0190 (unsound `Error::downcast_mut` in the transitive `anyhow`, reached only through `jsonschema`). Licence: MIT. Threat: the validator consumes schema and instance bytes that ship inside the binary or come from the repository's own data files, never from the network (the remote resolver is compiled out); the relevant risk is pathological patterns in `pattern`/`patternProperties` — `fancy-regex` permits backtracking, so a hostile schema could be slow, but every schema the toolkit compiles is one it bundles. `unsafe`: none in the crate's own `src/` (grep over 0.18.3).

## Alternatives considered

- `boon` — draft 2020-12 support, smaller subtree; not yet stable when the surface was built. The candidate if the subtree cost is revisited.
- `valico` — unmaintained, incomplete draft coverage.
- Hand-written structural checks — re-implementing a standard; rejected.
