# mdatron

**Status:** approved (CI-installed tool binary), with items owed — the Platform Engineer and Security reviews are recorded below; Solution Owner approval is the operator's ratification of the vsdd-cli#894 amendment
**Registered:** 2026-10-07, first registration under the Dependency approval member as amended by vsdd-cli#894 (tool binaries CI installs are covered). The binary has been consumed since the conformance checks first ran; this record closes that gap.
**Reviewed on:** first registration, and on each semver-incompatible move of the pin — for this 0.x tool, a change in the minor component (0.7 to 0.8 re-enters review); patch moves within 0.7 do not.
**Reviewed at:** 0.7.0

## What it is

[`mdatron`](https://github.com/magnificentlycursed/mdatron) — the conformance engine for the methodology's markdown artifacts: schemas, routes, pins, vocabulary, links, markers and section rules, run as `mdatron verify`. A sibling project in this estate.

## Why we need it

The contract's Conformance at action time member makes mdatron the checker of every governed markdown artifact, at action time through the pre-commit hook and at boundary time through CI. Without it the governed corpus has no shape checks.

## The boundary

vsdd consumes the mdatron **binary**, never a library (`Cargo.toml` comment; the vsdd-cli#739 boundary): mdatron is not in `Cargo.lock`, and nothing in vsdd links its code. It is reached as a subprocess and through its versioned `verify --json` envelope.

## Pin and install sites

- CI: `cargo install mdatron --version 0.7.0 --locked --force` from crates.io in `.github/workflows/mdatron-verify.yml` and `.github/workflows/vsdd-test.yml`. Both cache the built binary; on a cache hit the job verifies the restored binary against the sha256 recorded when it was built as well as its version string (vsdd-cli#896), so the registry checksum below is checked on a miss and the restored bytes on a hit.
- Adopters: `templates/.github/workflows/vsdd-verify.yml` installs 0.7.0 with `--locked` (without `--force`).
- Developers: `README.md` and the pre-commit hook's install hints; the hook refuses a version outside 0.7.x.
- The consumed envelope is pinned at `.mdatron/envelope-3.1.0.schema.json` and asserted by exact equality in CI.
- Toolchain: pinned to 1.88 by running the install from inside the vsdd-cli checkout, with `rust1.88` in the cache keys so no binary built under another toolchain is restored (vsdd-cli#896).

## Supply chain

- Source: crates.io. crates.io versions are immutable (yank only), and cargo checks each downloaded crate against the registry index checksum. The 0.7.0 crate's sha256, matching the index, is `fc35de8268f92488b235049697f6ac06c85d3b6e3a8a1c4e8c1ad76e5af7c0f7`.
- `--locked` builds mdatron's transitive set exactly as the `Cargo.lock` packaged in the crate declares. The crate has no build script.
- The v0.7.0 tag is annotated and unsigned; no release signature or provenance attestation is published (raised upstream as mdatron GitHub issue #73).
- License: MIT.

## Retest trigger

The next mdatron release: the CI envelope assertion fails on any envelope version change, which forces a re-pin and an adoption issue (the vsdd-cli#877 and vsdd-cli#893 precedent). A minor-component move re-enters the review below.

## Solution Owner, Platform Engineer and Security review

**Solution Owner (scope):** approved by the operator's ratification of the vsdd-cli#894 amendment.

**Platform Engineer (supply chain):** approved, with items owed (vsdd-cli#894 cold review, 2026-10-07). The pin is exact (0.7.0) at every CI site and in the adopter template; the source is the immutable crates.io artifact whose sha256 matches the registry index; `--locked` honours the packaged lockfile; the license is MIT; the envelope pin's exact-equality assertion is a CI-backed block that forces a re-pin on any envelope change. Risks checked: version floating (none), lockfile honoured (yes), checksum (verified against the local registry cache and index), license (MIT), signature and provenance (absent, recorded). Owed items, all met by vsdd-cli#896 (2026-10-08): the cache-hit check now verifies the restored binary against its install-time sha256 (an integrity check on the restored blob, not provenance: the digest travels in the same cache entry, and a mismatch fails the job); the install runs under 1.88 from inside the vsdd-cli checkout; both workflows declare `contents: read` and persist no credential.

**Security (CVEs, license, threat):** approved for 0.7.0, with items owed (vsdd-cli#894 cold review, 2026-10-07). An offline `cargo audit` of the lockfile packaged in the 0.7.0 crate, against the local RustSec database dated 2026-10-03, reports no vulnerabilities and no warnings across 152 crates; mdatron's own CI runs cargo-audit and cargo-deny on each change. License: MIT, in the crate's manifest and LICENSE file, compatible with use as an installed binary that is never linked. Threat: the realistic attacker is a hijacked crates.io publisher, given no provenance attestation. In this repository's CI that attacker reaches the job's GitHub token, persisted by checkout, and the governed tree whose verdict it reports; in the adopter template it also reaches the adopter's API key and telemetry token and pull-request and code-scanning write. Owed items, met by vsdd-cli#896 (2026-10-08): the adopter template scopes its API key to the one step that claims it and runs mdatron and every install with no secret in reach; the repository workflows declare read-only token permissions and persist no credential; the cached binary is checked against its install-time sha256. Still open: the adopter template cannot run until the vsdd crate is published (vsdd-cli#664); reserving the crate name is the operator's act.
