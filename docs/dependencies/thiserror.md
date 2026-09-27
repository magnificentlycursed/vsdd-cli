# thiserror

**Status:** approved (runtime dependency, vsdd-core) — retrofit record
**Approved:** 2026-09-27 (retrofit). The crate has been a runtime dependency since the first typed error enum in `vsdd_core::init` (workspace pin `1`); no record and no three-lens review accompanied it — the `VSDD-E0100` condition the contract's Dependency approval member names. This record closes the verification debt; the process breach stands as recorded on vsdd-cli #872.
**Approved by:** retrofit under vsdd-cli #872 (Platform Engineer owner, Security validator), the three lenses recorded below

## What it is

[`thiserror`](https://crates.io/crates/thiserror) — a derive macro for `std::error::Error` that generates `Display` and `source()` from attributes on an error enum. Maintained by David Tolnay.

## Why we need it

The toolkit's diagnostics are typed: `vsdd_core::init`, `vsdd_core::diagnostics` and `vsdd_core::answer::deviations` each define an error enum whose variants carry the code, the message and the cause the machine form reports. `thiserror` keeps the message next to the variant and generates the boilerplate `Display`/`Error` impls that would otherwise drift.

## Why this crate

- The standard derive for library error types; `anyhow` is the application-side complement and is deliberately not a direct dependency (typed codes are the point).
- Zero runtime footprint: the crate is a proc-macro plus a tiny runtime shim; the generated code is what a hand-written impl would be.

## Scope

- Runtime dependency of `vsdd-core`, workspace requirement `1`, locked at 1.0.69.
- Consumers: `vsdd-core/src/init.rs`, `vsdd-core/src/diagnostics.rs`, `vsdd-core/src/answer/deviations.rs`.
- `thiserror 2.x` exists (and is already in the lockfile transitively). Moving the direct edge to `2` is a major-version bump and re-enters review; the `1.x` line still receives releases.

## Three-lens review (retrofit)

**Solution Owner — scope.** In scope: the contract's diagnostics requirements call for stable codes and messages; a derive that ties message to variant is the narrowest way to keep them in one place.

**Platform Engineer — supply chain.** Subtree: 7 unique crates (`thiserror-impl`, `proc-macro2`, `quote`, `syn`, `unicode-ident`). Build: compile-time proc-macro only; no build-script network access. Maintenance: continuous.

**Security — CVE, licence, threat.** `cargo audit` on 2026-09-27 against the current lockfile: no advisory against this crate or its subtree. The workspace's one allowed warning is RUSTSEC-2026-0190 (unsound `Error::downcast_mut` in the transitive `anyhow`, reached only through `jsonschema`). Licence: MIT OR Apache-2.0. Threat: none at run time — the crate contributes no code path that touches input; the proc-macro runs at build time on the toolkit's own source. `unsafe`: none in the crate's own `src/` (grep over 1.0.69).

## Alternatives considered

- Hand-written `Display`/`Error` impls — the pre-derive state; drifted from the variants and was replaced.
- `snafu` — heavier, context-selector style; more than the diagnostics need.
- `anyhow` — erases the type; the machine form needs the code.
