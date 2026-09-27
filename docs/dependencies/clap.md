# clap

**Status:** approved (runtime dependency, vsdd binary) — retrofit record
**Approved:** 2026-09-27 (retrofit). The crate has been the CLI parser since the `vsdd` binary's first subcommand (workspace pin `4.5`, feature `derive`); no record and no three-lens review accompanied it — the `VSDD-E0100` condition the contract's Dependency approval member names. This record closes the verification debt; the process breach stands as recorded on vsdd-cli #872.
**Approved by:** retrofit under vsdd-cli #872 (Platform Engineer owner, Security validator), the three lenses recorded below

## What it is

[`clap`](https://crates.io/crates/clap) — the command-line argument parser: derive macros turn an annotated struct/enum into the parser, help, version output and error messages.

## Why we need it

`vsdd` is a subcommand CLI (`status`, `gate`, `init`, …) with per-command flags and `--machine` forms. `clap`'s derive keeps the command tree in one Rust enum (`vsdd/src/main.rs`) that is also the documentation the `--help` surface prints; hand-parsing `std::env::args` would duplicate that in prose and drift.

## Why this crate

- The ecosystem standard; `crosslink` and `mdatron`, the sibling tools the operator drives alongside `vsdd`, use it too, so the option grammar is uniform across the desk.
- `derive` is the only added feature; the defaults (`std`, `color`, `help`, `usage`, `error-context`, `suggestions`) are kept because the terminal output they govern is the operator's first contact with a failure.

## Scope

- Runtime dependency of the `vsdd` binary only (not `vsdd-core`), workspace requirement `4.5`, locked at 4.6.1.
- One consumer: `vsdd/src/main.rs` (`Parser`/`Subcommand` derives).

## Three-lens review (retrofit)

**Solution Owner — scope.** In scope: the contract specifies the `vsdd` command surface; this is its parser. Terminal output from `clap` (help, errors) goes through `clap`'s own escaping, not the toolkit's terminal cleaner — acceptable because that text is authored in the source, never sourced from the tracker.

**Platform Engineer — supply chain.** Subtree: 18 unique crates (`clap_builder`, `clap_derive`, `clap_lex`, `anstream`/`anstyle*`/`colorchoice`, `strsim`, `heck`, `proc-macro2`, `quote`, `syn`, `unicode-ident`, and the `is_terminal`/`windows-sys` shims). All from the clap-rs organisation or the Rust CLI working group. Build: compile-time proc-macro; no build-script network access. Maintenance: continuous, semver-stable `4.x`.

**Security — CVE, licence, threat.** `cargo audit` on 2026-09-27 against the current lockfile: no advisory against this crate or its subtree. The workspace's one allowed warning is RUSTSEC-2026-0190 (unsound `Error::downcast_mut` in the transitive `anyhow`, reached only through `jsonschema`). Licence: MIT OR Apache-2.0. Threat: the parser reads the process's own argument vector; the only input is what the operator or a dispatching agent typed. No historical advisories against `clap 4`.

## Alternatives considered

- `argh`, `pico-args`, `lexopt` — smaller, but no derive-generated help of comparable quality, and the sibling tools already standardise on `clap`.
- Hand-rolled parsing — rejected for the drift reason above.
