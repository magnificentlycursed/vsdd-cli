# serde_yaml_ng

**Status:** approved (runtime dependency, vsdd-core and vsdd; dev-dependency, vsdd) — retrofit record
**Approved:** 2026-09-27 (retrofit). The crate entered when the YAML surface moved off the deprecated `serde_yaml` (requirement `0.10`, declared directly in both crates rather than through the workspace table); no record and no three-lens review accompanied it — the `VSDD-E0100` condition the contract's Dependency approval member names. This record closes the verification debt; the process breach stands as recorded on vsdd-cli #872.
**Approved by:** retrofit under vsdd-cli #872 (Platform Engineer owner, Security validator), the three lenses recorded below

## What it is

[`serde_yaml_ng`](https://crates.io/crates/serde_yaml_ng) — YAML 1.2 for serde, the maintained fork of David Tolnay's `serde_yaml` after its author archived that crate in March 2024 (RUSTSEC-2024-0320, informational). Same API surface, same `unsafe-libyaml` backend (a machine translation of libyaml into Rust).

## Why we need it

The governed data lives in YAML: the registry data sets, the deviation register, `state.yaml`, the repo-set configuration for multi-repo status. Every one of them is read (and `state.yaml` written) through `serde_yaml_ng` into the typed structs the schema check then validates.

## Why this crate

- Drop-in successor to the crate the toolkit already used; the migration was a rename.
- Of the post-deprecation forks, the one with a single identifiable maintainer, tagged releases and no history of yanked versions (`serde_yml` was the other candidate and was set aside on maintenance signals).

## Scope

- Runtime dependency of `vsdd-core` and of `vsdd`, and a dev-dependency of `vsdd`; requirement `0.10`, locked at 0.10.0.
- Consumers: seven modules in `vsdd-core/src` (`answer/deviations`, `diagnostics`, `registry`, `registry/sets`, `schema_check`, `state/read`, `state/write`), `vsdd/src/status/multi.rs`, and tests in both crates.
- The declaration is repeated in both `Cargo.toml` files instead of living in `[workspace.dependencies]`; lifting it to the workspace table is a hygiene follow-up, not a condition of approval.
- The deprecated `serde_yaml` (0.9) was still declared in the workspace table and in `vsdd/Cargo.toml` with no consumer; this record's PR removes both declarations.

## Three-lens review (retrofit)

**Solution Owner — scope.** In scope: the contract fixes YAML for the governed data. The crate choice was the migration the deprecation forced; consolidating the two direct declarations into the workspace table is the only open hygiene item.

**Platform Engineer — supply chain.** Subtree: 15 unique crates, chiefly `unsafe-libyaml`, `indexmap`/`hashbrown`/`equivalent`, `itoa`, `ryu`, `serde`. Build: no build scripts with network access. Maintenance: a single maintainer with tagged releases; a smaller bus factor than the rest of the serde family, which is the reason to keep the crate's version pinned by the lockfile and to re-check it at each dependency update.

**Security — CVE, licence, threat.** `cargo audit` on 2026-09-27 against the current lockfile: no advisory against this crate or its subtree. The workspace's one allowed warning is RUSTSEC-2026-0190 (unsound `Error::downcast_mut` in the transitive `anyhow`, reached only through `jsonschema`). Licence: MIT. Threat: the parser reads repository-controlled files only. `unsafe`: 59 hits in the crate's own `src/` (0.10.0), inherited from `serde_yaml`'s libyaml bridge — the largest `unsafe` surface among the direct dependencies, mitigated by the input being the repository's own data and by upstream's fuzzing of the same code in `serde_yaml`. The predecessor's advisory RUSTSEC-2024-0320 is informational (unmaintained), not a vulnerability, and does not apply to this fork.

## Alternatives considered

- `serde_yaml` — archived by its author; the crate this one replaced. Its dead declarations are removed in the same PR as this record.
- `serde_yml` — fork with a contested maintenance history; set aside.
- `yaml-rust2` — maintained YAML parser without serde; would need a hand-written mapping layer for every struct.
