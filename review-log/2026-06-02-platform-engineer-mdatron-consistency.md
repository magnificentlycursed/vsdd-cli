---
schema_class: review-entry
schema_version: 1.0.0
review_number: 3
date: 2026-06-02
phase: phase-1a
scope: >-
  Platform Engineer opinion on mdatron CLI surface + config layout + error-handling style
  for consistency with crosslink as substrate baseline, in pursuit of the "absorbability"
  goal. Surfaces under review: mdatron-cli/src/main.rs (binary surface), mdatron-core/src/
  {error.rs, diagnostic.rs, verify.rs, schema.rs}, mdatron/Cargo.toml + workspace,
  mdatron/rust-toolchain.toml, DESIGN-MDATRON.md install + adoption story, mdatron/.github/
  workflows/ (NONE present). Baseline: crosslink/crosslink/Cargo.toml + main.rs CLI surface
  + .github/workflows/{ci.yml, publish.yml, release-builds.yml} + CHANGELOG.md.
lens: >-
  Platform Engineer (primary) + Sanity Check (baseline) — PE dim 1 (Reproducible builds),
  dim 2 (Dependency approval discipline), dim 3 (Supply-chain attestation), dim 4 (CI
  workflow discipline), dim 6 (Build performance — cargo install timing), dim 7
  (Observability of CI itself), dim 8 (Cross-platform binary builds). Sanity dim 3
  (rubber-ducking the "consume mdatron a year from now" question).
source: operator-directive
session_note: >-
  Cold-session opinion work — no prior context on mdatron's design decisions. Citations
  grounded in current mdatron-cli/src/main.rs line-numbers + crosslink baseline line-numbers
  as inspected this session. Operator directive in effect: mdatron is standalone; vsdd
  consumes it as a dependency (already wired — vsdd-cli/Cargo.toml:14 + vsdd-core/Cargo.toml:12
  consume mdatron-core via local path-dep workspace edge; vsdd-cli/.githooks/pre-commit
  consumes mdatron CLI via PATH). Dual library + CLI consumption is the load-bearing fact.
model: claude-opus-4-7
execution_method: >-
  single-domain cold-session opinion (PE primary lens + Sanity-Check baseline);
  no code modifications; review-log entry only.
sycophancy_compensation: >-
  PE-lens bias is "explicit + observable + supported across substrates wins"; cheap-path
  counter-bias is "rely on cargo + git, ship faster." I am resisting the cheap path by
  naming concrete pre-0.1.0 publish blockers (missing LICENSE file, missing README,
  zero CI) rather than treating "it builds locally" as discipline-held. I am NOT
  recommending the heavyweight crosslink-shape (release-builds matrix + cosign + SLSA)
  for v0.1 — that is v1.0-ship-criteria territory and over-specifying it now is
  ergonomics-traded-for-imagined-discipline.
supplements_loaded: []
---

# Platform Engineer Opinion — mdatron consistency with crosslink baseline (Phase 1a)

## Headline

mdatron's 0.1.0 publish-readiness gap is **substantial but mechanical**: no LICENSE file at repo-root, no README, zero CI workflows, no `--locked` discipline visible at the install boundary, and a CLI surface missing two crosslink-baseline globals (`--quiet`, `--json`) that downstream consumers (vsdd-cli's hook, future agent-loop) will need within the first release. Fix-list below is ordered by what blocks `cargo publish` vs what blocks "absorbability". `mdatron init` for v0.1 should ship empty-dirs-plus-config.yaml-stub, NOT starter content — starter content is the adopter's vocabulary, not mdatron's.

## Mechanical citations

- mdatron CLI surface: `mdatron-cli/src/main.rs:15-50` — two subcommands (`verify`, `explain`); flags scoped per-subcommand; no top-level globals.
- mdatron exit codes: `main.rs:74-95` — exit 2 for pipeline/IO error, exit 1 for findings-with-errors, exit 0 otherwise. Matches rustc convention; consistent.
- mdatron error catalog at CLI: `main.rs:74, 141, 145` — `MDATRON-E0070` (project root unresolvable), `MDATRON-E0080` (pipeline failure). These are coined at print-site, not declared in a registry.
- mdatron workspace deps + MSRV: `mdatron/Cargo.toml:1-20` — MSRV `1.88`, edition 2021, 7 workspace dependencies, no `[workspace.metadata]` for `cargo-deny` / `cargo-audit` / dependency-investigation discipline.
- mdatron toolchain: `mdatron/rust-toolchain.toml:1-4` — channel `1.88`, components rustfmt + clippy, profile minimal.
- mdatron LICENSE file: ABSENT at repo root. `mdatron/Cargo.toml:9` declares `license = "MIT"` but `cargo publish` will warn-or-fail without LICENSE in the published tarball (per crates.io guidance) + downstream consumers cannot satisfy MIT clause without the text.
- mdatron README: ABSENT. CHANGELOG.md present; DESIGN-MDATRON.md present; no top-level README.
- mdatron CI: ABSENT. No `.github/` directory at all.
- mdatron VerifyError shape: `mdatron-core/src/verify.rs:62-95` — 8 thiserror variants, structured with named fields (path/error pairs). Idiomatic. Distinct from internal `Error` enum at `error.rs:12-25` (4 variants); the verify/internal split is the right separation.
- mdatron diagnostic surface: `mdatron-core/src/diagnostic.rs:76-99` — `Finding::format_tty()` produces rustc-shape output deterministically. Tested at `diagnostic.rs:129-219`.
- mdatron logging: ABSENT. No `tracing` dep in `Cargo.toml`. No `--log-level` flag. `eprintln!` only.

- Crosslink CLI globals: `crosslink/src/main.rs:84-105` — `--quiet`, `--json`, `--log-level` (env `CROSSLINK_LOG`), `--log-format` (env `CROSSLINK_LOG_FORMAT`); all four declared `global = true`.
- Crosslink tracing init: `crosslink/src/main.rs:1856-1870` + `2304-2308` — `EnvFilter::try_new` with safe default; `Serve` subcommand overrides text→json (operational-context conditional).
- Crosslink Cargo.toml MSRV: `crosslink/Cargo.toml:5` — `rust-version = "1.87"`. Crosslink is at 1.87; mdatron is at 1.88. **One-version delta**, both ahead of debian-stable + bookworm-backports.
- Crosslink publish workflow: `crosslink/.github/workflows/publish.yml:1-50` — dry-run on PR, publish on tag-push, `cargo publish --locked` discipline.
- Crosslink release-builds: `release-builds.yml:34-50` — matrix over ubuntu/macos/windows x x86_64-only; `--locked --release` discipline. No aarch64 yet despite PE-prompt dim 8 calling for both arches.
- Crosslink Cargo.toml package metadata: `crosslink/Cargo.toml:7-21` — `description`, `documentation`, `homepage`, `keywords`, `categories`, `include` (explicit allowlist). mdatron has description + keywords + categories but no `include` allowlist.

## Findings

### F1 — 0.1.0 publish-readiness blockers: LICENSE + README + cargo-publish-dry-run-in-CI

**PE rationale (dim 1 + dim 3):** publishing to crates.io is one-way for a given version-number; a 0.1.0 with no LICENSE-in-tarball is a compliance gap that you cannot fix without yanking + republishing as 0.1.1. The cheap fix is mechanical: drop MIT LICENSE at repo root (matches `Cargo.toml:9` `license = "MIT"`), author a README starting with the TRON-blockchain disambiguation per DESIGN-MDATRON.md:63 + TW-F3, then wire a publish-dry-run CI job mirroring `crosslink/.github/workflows/publish.yml:31-47` (dry-run on PR; gated on tag-push for actual publish). The publish-readiness pass IS the supply-chain-attestation pass at 0.1.0 — you don't need cosign + SLSA yet, you need "the tarball that crates.io serves has a license file in it."

**Concrete pre-0.1.0 checklist (ordered):**
1. `mdatron/LICENSE` (MIT text, dual-author per workspace Cargo.toml:10)
2. `mdatron/README.md` — TRON disambiguation in line 1; 5-minute install + verify + explain walkthrough; pointer to DESIGN-MDATRON for depth
3. `mdatron/Cargo.toml` + sub-crate Cargo.toml additions: `readme = "../README.md"` mirroring `crosslink/Cargo.toml:12` pattern; `include = [...]` allowlist (mirror `crosslink/Cargo.toml:15-21`) — keeps the tarball lean + auditable
4. `mdatron/.github/workflows/ci.yml` — fmt + clippy + test on PR; matches crosslink `ci.yml:1-50` shape but trimmed to mdatron's smaller surface
5. `mdatron/.github/workflows/publish.yml` — dry-run on PR; publish on tag (mirror `crosslink/publish.yml` verbatim with crate-name swap)

**Classification:** Accepted (load-bearing for 0.1.0 publish).

---

### F2 — Adopt crosslink's global-flag triad (`--quiet`, `--json`, `--log-level`) BEFORE downstream consumers calcify on the current surface

**PE rationale (dim 4 + dim 7):** `vsdd-cli/.githooks/pre-commit:18` already calls `exec mdatron verify --project-root .`; the next pre-commit iteration will want JSON output for compact agent-loop integration. The DESIGN-MDATRON.md:649-657 compact format is the load-bearing agent-loop surface — it MUST be a global flag (declared `global = true` like crosslink does at `main.rs:84-105`), not a verify-only flag, because future subcommands (`registry`, `explain` with structured output) will need it. Coining the flag at 0.1.0 is cheap; adding it after consumers depend on the parse-shape is a breaking-change cliff.

`--log-level` is the more interesting cut. Currently mdatron has zero observability surface — `verify.rs:104-107` returns an `Io` variant with path + error string but no operator-visible audit-trail when validation pipelines silently succeed on misconfigured paths. Crosslink's pattern (`main.rs:1856-1870`) is sound: `tracing` crate, `EnvFilter` for `MDATRON_LOG=...`, `--log-format text|json`. Cost is one dep (`tracing` + `tracing-subscriber`) + ~15 LoC of init.

**Sanity-check rubber-duck:** "Adopter pipes `mdatron verify --json` into `jq`, three months from now." Currently impossible — `print_finding` at `main.rs:125-138` is hardcoded TTY. The compact + JSON + SARIF surfaces named in DESIGN-MDATRON.md:600-657 are v1.0 ship-criteria; 0.1.0 should at minimum ship `--format text|json` so the parse-shape is stable before adopters start scripting against TTY-text.

**Classification:** Accepted-pending. Routes Phase 1a → Phase 2 (CLI surface + observability wiring). Resolve BEFORE 0.1.0 to avoid breaking-change risk on 0.2.

---

### F3 — `mdatron init` v0.1 scope: empty `.mdatron/` skeleton + `config.yaml` stub; DO NOT ship starter schemas/patterns

**PE rationale (dim 4 + Sanity-dim-4 last-resort-discipline):** crosslink's `init` at `crosslink/src/commands/init/mod.rs` is 2506 LoC because it manages a substantial substrate (gitignore-managed-section + mcp servers + claude settings + hook-config + per-language rules). mdatron's substrate is two directories of adopter-authored content (`.mdatron/schemas/`, `.mdatron/patterns/`) plus a config file. **Adopter-authored is the load-bearing constraint:** schemas + patterns encode the adopter's vocabulary; shipping VSDD-shaped starters with `mdatron init` would couple mdatron to VSDD semantics, violating DESIGN-MDATRON.md:54 ("methodology-agnostic engine").

**v0.1 scope (concrete):**
- `mdatron init` creates: `.mdatron/{schemas,patterns,catalogs,registries}/.gitkeep` + `.mdatron/config.yaml` with a 5-line commented stub naming the schema_class routing keys + which globs validate. NO content under the dirs.
- `mdatron init --check` (dry-run): emits the deployment plan as JSON (mirror crosslink `init/mod.rs:560-565` `--update --dry-run` semantics shape).
- `mdatron init` refuses-with-recovery-hint if `.mdatron/config.yaml` already exists (mirror crosslink's `init/mod.rs:427-431` refusal pattern with the operator-facing "use `--update` or `--force`" hint).
- DO NOT scaffold a pre-commit hook script — that's adopter-concern (vsdd-cli's `.githooks/pre-commit` is the right shape; mdatron's job is to be on PATH and have a stable CLI). Reference the pattern in README; do not generate.
- DO NOT detect project type. Project-type detection is a v0.2+ ergonomics concern; at 0.1.0 the adopter knows what they're validating.

**The mdatron-examples library** (DESIGN-MDATRON.md:783-784) — 4 generalized classes (DESIGN doc, manual-test, PR template, CHANGELOG) — is the right home for starter content. Operators run `mdatron init` for skeleton + browse mdatron-examples + copy-paste what fits. Two-step adoption flow > one-step-with-baked-in-vocabulary.

**Classification:** Accepted (load-bearing v0.1 scope).

---

### F4 — MSRV 1.88 is fine but pin discipline at workspace + sub-crate + rust-toolchain.toml must agree

**PE rationale (dim 1):** mdatron workspace declares `rust-version = "1.88"` at `Cargo.toml:8`; `rust-toolchain.toml:2` declares channel `"1.88"`. The two-source-of-truth must always agree, else the build-environment-pinned axis breaks (`cargo` reads `rust-version`; rustup reads `rust-toolchain.toml`; CI runners reading neither will produce silent surprise on bump). Crosslink is one version behind at `1.87` (`crosslink/Cargo.toml:5`) but ships no `rust-toolchain.toml`, relying on `dtolnay/rust-toolchain@stable` in CI (`crosslink/ci.yml:40`). Both are valid disciplines; mdatron's pin-everywhere is **more** PE-correct (reproducibility-of-bytes axis), but introduces upkeep cost when bumping (must edit both files in lockstep).

**Recommendation:** keep both pins. Add a CI lint job (mirror crosslink `ci.yml` pattern) that fails when `rust-toolchain.toml` channel ≠ workspace `rust-version`. ~5-line check; closes the dual-source-of-truth drift hole. Document the bump procedure in `mdatron/docs/dependencies/` or a new `mdatron/docs/release-engineering.md`.

**Tradeoff acknowledged:** raising MSRV to 1.88 cuts off debian-stable users (which ships rust 1.63) and any organization on rustup-managed-pinned-environment that doesn't track stable. mdatron's audience is developer-tooling — `cargo install` adopters are already on rustup; the portability hit is theoretical. **No change recommended;** 1.88 is fine. Just make the agreement enforceable.

**Classification:** Resolved-pending (CI check + bump-procedure doc). Routes Phase 4 → Phase 1b.

---

### F5 — CI shape for mdatron itself: minimal-three (fmt/clippy/test) + publish-dry-run; defer release-builds matrix to v1.0

**PE rationale (dim 4 + dim 6 + dim 8):** crosslink's CI is 4 workflows (ci.yml + ci-feature.yml + publish.yml + release-builds.yml + fuzz-nightly.yml + docs.yml). For mdatron 0.1.0 with ~3000 LoC and ~50 tests, that's over-engineering. The minimum-viable CI:
- `ci.yml` — fmt + clippy + cargo test on ubuntu-latest (single-arch sufficient at 0.1.0 since mdatron is pure-Rust with no system deps; cross-platform-discovery cost can defer until v1.0)
- `publish.yml` — dry-run on PR (`cargo publish --locked --dry-run`); publish on tag (`cargo publish --locked`), mirror `crosslink/publish.yml:30-47`

The release-builds matrix (DESIGN-MDATRON.md mentions Linux x86_64+aarch64 + macOS x86_64+aarch64) is **v1.0 ship-criteria** per `V1-SHIP-CRITERIA.md:47`. At 0.1.0, `cargo install mdatron --locked` from crates.io is sufficient; the pre-built-binaries-via-GH-Releases ergonomics gap is real but not 0.1.0-blocking.

**For downstream projects consuming mdatron (the absorbability question):** the right CI shape is exactly what `vsdd-cli/.github/workflows/mdatron-verify.yml:42-48` does **once mdatron is published** — drop the sibling-repo checkout (`mdatron-verify.yml:24-28`) and switch to `cargo install mdatron-cli --locked --version "0.1.x"`. Pin the version. The current sibling-repo-checkout is a bootstrap-period workaround per BOOTSTRAP-MITIGATION; treating it as the steady state would couple downstream CI to the mdatron repo URL, which is the "Build pinned to 'latest' — reproducibility traded for ergonomics" PE sycophancy-failure-mode at the consumer level.

**Classification:** Accepted (CI shape for mdatron) + Accepted-pending (consumer-side CI guidance lands in README at 0.1.0 publish).

---

### F6 — "Stands alone" + "consumed by vsdd" is a dual-consumption pattern; document the version-pinning contract

**PE rationale (dim 1 + dim 2):** vsdd-cli already consumes mdatron in two shapes:
- Library: `vsdd-cli/Cargo.toml:14` + `vsdd-core/Cargo.toml:12` consume `mdatron-core` via Rust workspace path-dep
- CLI: `vsdd-cli/.githooks/pre-commit:6-19` consumes `mdatron` from PATH

These are independent supply-chain axes; **"stands alone" means each axis has its own version-pin**. The DESIGN-MDATRON.md:909-921 `MdatronVersionPinned` event surface is the right shape for the CLI axis (vsdd-cli reads `.vsdd/config.yaml` pinned-version; emits event at `vsdd init`); the library axis pins via `Cargo.lock` + the semver constraint in `vsdd-core/Cargo.toml:12`.

**Concrete recommendation (for mdatron 0.1.0):**
- mdatron-core ships with a `pub const MDATRON_CORE_VERSION: &str` exposed at the library boundary so downstream code can match against expected version at runtime — covers the library-API-stability axis
- mdatron CLI ships `mdatron --version` (already wired via clap; `main.rs:16` `#[command(...version...)]`) — covers the CLI-surface-stability axis
- mdatron 0.1.x semver pact: 0.1.x patches do NOT change CLI flag shape, mdatron-core public API, or finding-code names; 0.2.0 may. Codify in `mdatron/CHANGELOG.md:8` ("Unreleased" heading with `### Stability` subsection per Keep-a-Changelog convention).

The upgrade story for downstream: "`cargo update -p mdatron-core` for the library axis; `cargo install mdatron --version 0.1.x --locked` for the CLI axis; both can move independently within 0.1.x." That IS the dual-consumption discipline; making it operator-visible is what closes the absorbability gap.

**Classification:** Accepted (load-bearing for absorbability discipline).

---

### F7 — Observability gap: no logging today is "not yet built", not "well-tested". Wire `tracing` BEFORE 0.1.0

**PE rationale (dim 7):** the operator-visible audit-trail for a `mdatron verify` run is currently: "clean" or "N errors, M warnings" + per-finding rustc-text. There is no audit trail for: which schemas got loaded; which patterns matched zero files (silent dead-code); which glob expansions returned zero entries (typo); how long the pipeline took. The PE sycophancy-failure-mode "CI green is the only signal — CI exit code 0 conflated with discipline-held" maps directly: a `mdatron verify` exit 0 with **zero schemas loaded** (because `.mdatron/schemas/` was empty or misconfigured) is the silent-pass case. Currently invisible.

**Concrete 0.1.0 wiring (mirror crosslink `main.rs:1856-1870`):**
- Add `tracing` + `tracing-subscriber` to workspace deps (dim 2: investigation entry in `docs/dependencies/tracing.md` + `tracing-subscriber.md`; both are dtolnay-shape so investigation is mechanical)
- Wire `init_tracing(log_level, log_format)` in `main.rs` per crosslink's pattern
- `verify.rs:99-100` `tracing::info!("loaded {} schemas, {} patterns", schemas.len(), patterns.len())` + `tracing::debug!` on glob expansion + per-rule evaluation timing
- Default `MDATRON_LOG=warn` matches crosslink default at `main.rs:94`; operators raise to `info` or `debug` for triage

**Sanity-check rubber-duck:** "Adopter sets up `.mdatron/`, runs `mdatron verify`, gets `clean`, ships broken vocabulary to prod." That outcome is currently possible — clean output when zero rules ran. With `info`-level logging, the adopter sees `loaded 0 schemas, 0 patterns; matched 47 markdown files` and the discrepancy is operator-visible. Closing the silent-pass case is dim-7-load-bearing.

**Classification:** Resolved-pending (wire tracing + dep investigations). Routes Phase 4 → Phase 1b. Should land in 0.1.0; if cut, retire the observability claims in DESIGN-MDATRON.md:600+ ("rustc-shaped diagnostic surface" implies operator-visible audit trail) for honesty.

---

## Round-close summary

| Finding | Domain | Classification | Routing |
|---|---|---|---|
| F1 | PE | Accepted | Pre-0.1.0 publish blockers (LICENSE, README, CI skeleton) |
| F2 | PE + Sanity | Accepted-pending | Phase 1a → Phase 2 (global flags before consumers calcify) |
| F3 | PE + Sanity | Accepted | v0.1 init scope locked to empty-skeleton + config-stub |
| F4 | PE | Resolved-pending | CI lint for MSRV/toolchain agreement |
| F5 | PE | Accepted | Minimal-3 CI for mdatron; consumer-CI guidance in README |
| F6 | PE | Accepted | Dual-consumption version-pinning contract documented |
| F7 | PE + Sanity | Resolved-pending | Wire tracing before 0.1.0 to close silent-pass gap |

**Sycophancy-compensation reflection:** the bias I resisted was "ship 0.1.0 fast — defer everything to v1.0 ship-criteria." That bias would have produced a tarball published-but-uninstallable-into-an-orgs-license-audit (no LICENSE), a CLI surface that breaks on 0.2 (no `--json` global), and a verify pipeline that lies about its work (no logging). None of those are over-engineering; they are 0.1.0 floor. The over-engineering line is at release-builds-matrix + cosign + SLSA — those I am NOT recommending for 0.1.0 because they are v1.0-ship-criteria material per V1-SHIP-CRITERIA.md:40-58 and pulling them forward is the cheap-path-inverted (over-investing in ceremony before the product has adopters).

**Cross-references:**
- `mdatron-cli/src/main.rs:15-50, 74-95, 125-145` (current CLI surface)
- `mdatron-core/src/{error.rs:12-25, verify.rs:62-95, diagnostic.rs:76-99, schema.rs:36-78}` (engine surfaces)
- `mdatron/Cargo.toml:1-20, rust-toolchain.toml:1-4` (workspace + toolchain pins)
- `mdatron/DESIGN-MDATRON.md:52, 63, 568-587, 600-657, 783-784, 909-921` (install + init + format + version-pinning narrative)
- `mdatron/V1-SHIP-CRITERIA.md:40-58` (v1.0 ship-criteria boundary)
- `crosslink/crosslink/Cargo.toml:1-58` (baseline package metadata + dep story)
- `crosslink/crosslink/src/main.rs:84-105, 1856-1870, 2304-2308` (global flags + tracing init)
- `crosslink/.github/workflows/{ci.yml:1-50, publish.yml:1-50, release-builds.yml:1-80}` (CI shape baseline)
- `vsdd-cli/Cargo.toml:14, vsdd-core/Cargo.toml:12, .githooks/pre-commit:1-19, .github/workflows/mdatron-verify.yml:1-49` (dual-consumption surfaces)
