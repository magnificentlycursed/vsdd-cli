# regex

**Status:** approved (runtime dependency, vsdd-core) — retrofit record
**Approved:** 2026-09-27 (retrofit). The crate entered with the shell-side integrity checks (`integrity_shell`), requirement `1`; no record and no three-lens review accompanied it — the `VSDD-E0100` condition the contract's Dependency approval member names. This record closes the verification debt; the process breach stands as recorded on vsdd-cli #872.
**Approved by:** retrofit under vsdd-cli #872 (Platform Engineer owner, Security validator), the three lenses recorded below

## What it is

[`regex`](https://crates.io/crates/regex) — the Rust project's regular-expression engine: finite-automata based, guaranteed linear time in the input, no backtracking. Maintained by Andrew Gallant under rust-lang.

## Why we need it

The shell-side integrity checks (`vsdd status`, vsdd-cli #880) verify that reference handles in the governed corpus resolve. The handle forms come from a data set, each with its own pattern, and `vsdd_core::integrity_shell::refs` compiles that pattern at check time (`regex::Regex::new(&form.pattern)`). The patterns are data, so a real regex engine with a safety guarantee is required rather than a fixed hand-written matcher.

## Why this crate

- The linear-time guarantee is the reason: because the patterns come from a data file, a backtracking engine could be made pathologically slow by a bad pattern; `regex` cannot.
- The Rust project's own engine, used by `ripgrep`; default features include Unicode support the handle forms need.
- Compile-time limits (`size_limit`, `dfa_size_limit`) default to sane bounds, so a bloated pattern fails to compile rather than exhausting memory.

## Scope

- Runtime dependency of `vsdd-core`, requirement `1`, locked at 1.12.3.
- One consumer: `vsdd-core/src/integrity_shell/refs.rs`. A second consumer re-enters review.
- `regex` is also in the lockfile transitively through `jsonschema`; the direct edge added no crate.

## Three-lens review (retrofit)

**Solution Owner — scope.** In scope: the Status process-integrity requirement (#880) needs the handle forms checked against their declared grammar; compiling the data set's own pattern is the narrowest implementation and keeps grammar and check in one place.

**Platform Engineer — supply chain.** Subtree: 5 unique crates (`regex-automata`, `regex-syntax`, `aho-corasick`, `memchr`), all by the same maintainer. Build: no build scripts, no proc-macros. Maintenance: continuous, semver-stable `1.x` since 2017.

**Security — CVE, licence, threat.** `cargo audit` on 2026-09-27 against the current lockfile: no advisory against this crate or its subtree. The workspace's one allowed warning is RUSTSEC-2026-0190 (unsound `Error::downcast_mut` in the transitive `anyhow`, reached only through `jsonschema`). Licence: MIT OR Apache-2.0. Threat: patterns are repository data; the engine's linear-time guarantee and compile-size limits make a hostile or careless pattern a compile error or a slow-but-bounded match, never a hang. `unsafe`: one hit in the crate's own `src/` (1.12.3); the automata crates beneath carry more, all audited as part of the Rust project.

## Alternatives considered

- `fancy-regex` (already transitive via `jsonschema`) — backtracking, so no linear-time guarantee; rejected precisely because the patterns are data.
- Fixed matchers per handle form in Rust — would move the grammar out of the data set it documents.
