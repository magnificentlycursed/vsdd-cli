# crosslink

**Status:** approved (CI-installed tool binary) for the pinned commit, with items owed — the Platform Engineer and Security reviews are recorded below; Solution Owner approval is the operator's ratification of the vsdd-cli#894 amendment
**Registered:** 2026-10-07, first registration under the Dependency approval member as amended by vsdd-cli#894 (tool binaries CI installs are covered).
**Reviewed on:** first registration and **every re-pin** — the pin is a commit, which has no version component to move, so any re-pin pulls new upstream code; the owed move to a release tag re-enters the review too. The pin's posture, an upstream commit rather than a release, is carried in the deviation register; this record points there rather than repeating it.
**Reviewed at:** commit 875066ad66b868c63cfde09d1b0de6f6d8228f05

## What it is

[`crosslink`](https://github.com/Corvidae-Coding-Projects/crosslink) — the issue tracker and agent-orchestration tool this estate runs on: the tracker, sessions, locks, the hub, kickoff, and the hooks it deploys into `.claude/` and `.crosslink/`.

## Why we need it

The routing gate (`vsdd gate`) reads the live crosslink tracker, so CI installs crosslink and hydrates the tracker before running it. Locally, every session and every tracker record goes through it.

## The boundary

vsdd consumes the crosslink **binary** and its tracker data, never a library; crosslink is not in `Cargo.lock`.

## Pin and install sites

- CI: `.github/workflows/routing-gate.yml` checks out `Corvidae-Coding-Projects/crosslink` at commit `875066ad66b868c63cfde09d1b0de6f6d8228f05` (the merge of crosslink PR #108) and builds the nested `crosslink/crosslink` crate with `cargo install --path . --locked`.
- The pin posture — a commit, because no upstream release tag carries the fixes the gate depends on — is the `crosslink-develop-consumption` entry in `.vsdd/registry/deviation-registry.yaml` (vsdd-cli#892), with a date retest of 2026-11-30.
- Toolchain: the job builds crosslink with the runner image's default Rust, not the 1.88 that crosslink declares (owed below).
- Locally: the developer's installed binary, which may differ from the CI pin; the handoff records note when it does.

## Supply chain

- Source: built from a pinned git commit of the upstream repository, not a registry. The commit hash fixes the source exactly; `--locked` fixes the transitive set to the nested crate's committed `Cargo.lock`.
- Release state: upstream tags exist (the newest, v0.9.0-beta.1, predates the pin), but none carries the pinned commit. No release signature or provenance attestation is published. The pinned merge commit carries a GitHub signature header, not verified here.
- The crate has a build script; at the pin it runs only local `git rev-parse` and `git status` and generates rule files.
- License: MIT.

## Retest trigger

The register entry's: on 2026-11-30, or earlier on an upstream release that carries the pinned fixes, re-pin to the release tag — which re-enters the review below, as every re-pin does.

## Solution Owner, Platform Engineer and Security review

**Solution Owner (scope):** approved by the operator's ratification of the vsdd-cli#894 amendment.

**Platform Engineer (supply chain):** approved for the current pin, with items owed (vsdd-cli#894 cold review, 2026-10-07). The source is fixed exactly by the commit (the crosslink PR #108 merge, on upstream develop); `--locked` builds the nested crate against its committed lockfile; the license is MIT; the commit-not-release posture is the `crosslink-develop-consumption` register entry. Risks checked: source pinning (exact), lockfile (committed and honoured), license (MIT), release state (tags exist, none carries the pin), deviation coverage (entry present and current). Owed: every re-pin re-enters this review; the build runs third-party build-script code while the job token is persisted into the checkouts, so the workflow should declare `permissions: contents: read` and set `persist-credentials: false` on the crosslink checkout; the build uses the runner's default toolchain rather than 1.88.

**Security (CVEs, license, threat):** approved for the pinned commit, with items owed (vsdd-cli#894 cold review, 2026-10-07). An offline `cargo audit` of the pinned commit's `crosslink/Cargo.lock`, against the local RustSec database dated 2026-10-03, reports no vulnerabilities and no warnings across 417 crates; upstream runs no audit or license-deny check of its own, so this review is the only one. License: MIT, in the pinned commit's manifest and LICENSE file (two copyright notices, the original project and the crosslink contributors); no linking, no redistribution. Threat: crosslink runs in the routing gate with the checkout's persisted token and writes the tracker data the gate reads; a compromised upstream commit pulled in by a future re-pin could run at build time, read the token and, if the token can write, forge the hub branch the gate treats as its oracle. The commit pin protects against upstream compromise after the pin, not at re-pin. Owed: every re-pin, including the move to a release tag, re-enters this review; the routing gate declares `contents: read` and `issues: read`.
