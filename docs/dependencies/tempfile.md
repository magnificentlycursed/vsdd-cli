# tempfile

**Status:** approved (runtime dependency, vsdd-core; dev-dependency, vsdd) — retrofit record
**Approved:** 2026-09-27 (retrofit). The crate entered as a test fixture crate and became a runtime dependency when `state/write.rs` adopted write-then-rename; no record and no three-lens review accompanied it — the `VSDD-E0100` condition the contract's Dependency approval member names. This record closes the verification debt; the process breach stands as recorded on vsdd-cli #872.
**Approved by:** retrofit under vsdd-cli #872 (Platform Engineer owner, Security validator), the three lenses recorded below

## What it is

[`tempfile`](https://crates.io/crates/tempfile) — temporary files and directories with secure creation (`O_EXCL`, random names) and a `persist` operation that renames a temporary file into place. Maintained by Steven Allen.

## Why we need it

Two uses. At run time, `vsdd_core::state::write` writes `state.yaml` atomically: it creates a `NamedTempFile` in the target directory and `persist`s it over the destination, so a crash mid-write leaves the previous state intact rather than a truncated file. In tests (eight `vsdd-core` test files and the `vsdd` integration tests), `TempDir` gives each test an isolated repository root.

## Why this crate

- The standard for the write-then-rename idiom; `persist` is a rename on the same filesystem, which is the atomicity guarantee the state writer relies on. Creating the temp file in the destination's own directory (`new_in`) is what keeps the rename on one filesystem.
- Secure creation by default (exclusive create, random name); no reliance on `TMPDIR`.

## Scope

- Runtime dependency of `vsdd-core`, requirement `3`, locked at 3.27.0; dev-dependency of `vsdd` (integration test fixtures).
- Runtime consumer: `vsdd-core/src/state/write.rs` (`NamedTempFile::new_in` + `persist`). A second runtime consumer re-enters review.

## Three-lens review (retrofit)

**Solution Owner — scope.** In scope: the contract requires state writes that a failed run cannot corrupt; write-then-rename is the narrowest correct mechanism and `tempfile` is its idiomatic carrier.

**Platform Engineer — supply chain.** Subtree: 8 unique crates (`fastrand`, `getrandom`, `rustix`/`libc`, `once_cell`, `cfg-if`, and the `windows-sys` shim). Build: no build-script network access. Maintenance: active, semver-stable `3.x`.

**Security — CVE, licence, threat.** `cargo audit` on 2026-09-27 against the current lockfile: no advisory against this crate or its subtree. The workspace's one allowed warning is RUSTSEC-2026-0190 (unsound `Error::downcast_mut` in the transitive `anyhow`, reached only through `jsonschema`). Licence: MIT OR Apache-2.0. Threat: the runtime path creates a file in a directory the toolkit already writes to and renames it; the temp name is random and created exclusively, so a pre-placed file or symlink cannot be followed. `unsafe`: four hits in the crate's own `src/` (3.27.0), around the platform file-creation syscalls. No historical advisories against `tempfile 3`.

## Alternatives considered

- `std::fs::write` directly — not atomic; a crash leaves a partial `state.yaml`.
- `atomicwrites` — thin wrapper over the same idiom with a smaller maintenance base; `tempfile` was already a dev-dependency.
- Hand-rolled temp name + rename — reinvents the secure-creation details `tempfile` gets right.
