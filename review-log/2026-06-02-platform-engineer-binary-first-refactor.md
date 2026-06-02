---
schema_class: review-entry
schema_version: 1.0.0
review_number: 4
date: 2026-06-02
phase: phase-1a
scope: >-
  Platform Engineer proposal for refactoring mdatron + vsdd to a binary-first,
  crosslink-cued structure. Operator directive settled: both binary-consumed;
  vsdd shells out to `mdatron verify` rather than depending on mdatron-core as
  library; mdatron owns `mdatron init`. Surfaces in scope: mdatron/Cargo.toml +
  mdatron-cli/src/main.rs (current dual-crate workspace), vsdd-cli/Cargo.toml +
  vsdd/src/{main.rs, preflight.rs} + vsdd-core/src/init.rs (current path-dep on
  mdatron-core), vsdd-cli/.github/workflows/mdatron-verify.yml (current
  sibling-checkout CI), crosslink/.github/workflows/{publish.yml, ci.yml}
  (target shape baseline), crosslink/crosslink/Cargo.toml (single-crate
  lib+bin baseline).
lens: >-
  Platform Engineer primary — PE dim 1 (Reproducible builds), dim 3
  (Supply-chain attestation), dim 4 (CI workflow discipline), dim 6 (Build
  performance), dim 7 (Observability of CI itself), dim 8 (Cross-platform
  binary builds). Sanity Check baseline — rubber-ducking "operator installs
  both tools fresh a year from now" + "vsdd CI runs in an org pinning specific
  mdatron version".
source: operator-directive
session_note: >-
  Cold-session single-domain composition (PE-primary + Sanity-Check baseline).
  Required referents re-read fresh — vsdd-domain-platform-engineer.md,
  vsdd-domain-sanity-check.md, review-entry.json, prior 2026-06-02
  platform-engineer-mdatron-consistency.md (library-era opinion) +
  platform-engineer-init-drift.md (sibling format example). Operator directive
  inverts the prior library-link assumption; subprocess invocation introduces
  new cross-process trust boundary this proposal pressure-tests against
  current library-link reliability.
model: claude-opus-4-7
execution_method: >-
  single-domain cold-session proposal (PE primary + Sanity-Check baseline);
  no code modifications; review-log entry only.
sycophancy_compensation: >-
  PE-lens bias is "observable + supportable wins"; the cheap path under
  binary-first is "subprocess invocation is fine, mdatron is just a tool". I
  resist by naming four new failure modes subprocess introduces (spawn-failure,
  malformed-stdout, runtime-version-skew, log-propagation-gap) the current
  library-link does not have, and by NOT recommending cross-platform
  release-binaries-via-GitHub-Releases at v0.1 (that remains v1.0 ship-criteria).
  Verdict: subprocess IS a net PE win for release-decoupling, but the contract
  surface (wire format + exit codes + version negotiation) must be
  load-bearing-treated at 0.1.0 or the refactor trades one supply-chain
  coupling for a worse one.
filename_note: >-
  Slug `binary-first-refactor` mirrors the same-day SA opinion under the same
  operator directive; distinguishes from prior same-day `mdatron-consistency`
  (library-era PE opinion) + `init-drift` (vsdd-init spec-drift PE opinion).
supplements_loaded: []
---

# Platform Engineer Proposal — Binary-First Crosslink-Cued Refactor (Phase 1a)

## Headline

Binary-first is a net PE win on three axes (release-decoupling, supply-chain audit-scope, MSRV independence) but introduces four runtime failure modes the library-link traded at compile time. The hard-to-undo decision is the wire-format contract — JSON-on-stdout, human-text-on-stderr, rustc-shaped exit codes — and it MUST ship at mdatron 0.1.0 before vsdd's subprocess client calcifies. Collapse both repos to single binary-publishing crates (drop `-core` halves from the published surface); consumer CI replaces sibling-checkout with `cargo install mdatron --locked --version "0.1.x"`.

---

## 1. Boundary design (operational)

- `vsdd init` spawns `mdatron init --project-root .` for `.mdatron/`; vsdd does NOT replicate substrate logic.
- `vsdd verify` spawns `mdatron verify --project-root . --format json`, parses stdout, aggregates with vsdd-native findings.
- **Discovery: PATH-first, config-overridable.** `vsdd/src/preflight.rs:99-114` already probes `mdatron --version`; init pins resolved path + version into `.vsdd/config.yaml`; runtime honors `mdatron_path` override before PATH.
- **Version compat: declared, not derived.** vsdd carries `pub const MDATRON_MIN_VERSION: &str = "0.1.0";` at compile time; init refuses on `< MIN` or major mismatch; pinned-version in config drives `MdatronVersionDrift` event when `vsdd verify` detects post-init replacement.
- Install-time AND runtime detection (NOT either-or) — install-only misses long-tail upgrade-broke-it.

## 2. Install + adoption picture

```
cargo install mdatron --locked --version "0.1.0"
cargo install vsdd     --locked --version "0.1.0"
git init && vsdd init
```

mdatron MUST install before `vsdd init` runs; `cargo install vsdd` itself does NOT depend on mdatron (no link-time dep). PE-strict improvement: org can ship vsdd to workstations without mdatron on every workstation's install path. Failure modes:
- missing → preflight refuses (current behavior at `preflight.rs:111` unchanged).
- `< vsdd_min` → `VSDD-E0240: mdatron {x} < required {y}; upgrade via cargo install mdatron --version "{y}" --locked --force`.
- major-mismatch → `VSDD-E0241: incompatible major; upgrade vsdd or pin mdatron`.
- replaced post-init → config-pinned version detects; `MdatronVersionDrift`; soft-warn minor / refuse major.

## 3. File-tree + Cargo.toml shape

**mdatron (post-refactor, single crate):**
```
mdatron/
├── Cargo.toml             # name="mdatron"; [[bin]]+[lib]; lib is in-crate-only, not stable public
├── LICENSE / README.md    # currently absent — F1 carry-over
├── rust-toolchain.toml    # channel = "1.88"
├── .github/workflows/{ci.yml, publish.yml}
└── src/
    ├── main.rs
    ├── lib.rs             # internal re-exports; no public contract
    ├── commands/{mod.rs, init.rs, verify.rs, explain.rs}
    ├── diagnostic.rs, schema.rs, verify.rs, frontmatter.rs, error.rs
    └── dsl/
```

**vsdd-cli (post-refactor, single crate):**
```
vsdd-cli/
├── Cargo.toml             # name="vsdd"; [[bin]]+[lib]; no mdatron-core dep
├── LICENSE / README.md
├── rust-toolchain.toml
├── .github/workflows/{ci.yml, publish.yml, vsdd-verify.yml}
└── src/
    ├── main.rs, lib.rs, preflight.rs
    ├── mdatron_client.rs  # NEW — subprocess wrapper: spawn, parse JSON, map errors
    ├── commands/{init.rs, verify.rs, observe.rs}
    ├── schemas/, patterns/, artifacts/   # include_str!-embedded
```

**Cargo.toml (mdatron, target):** single `[package]` (no `[workspace]`), `[[bin]] name = "mdatron"; path = "src/main.rs"`, `[lib] name = "mdatron"; path = "src/lib.rs"` (in-crate-only, doc-comment says so), explicit `include = ["src/**/*", "LICENSE", "README.md"]`. vsdd Cargo.toml is the same shape minus the mdatron-core dep entirely.

**CI workflows (post-refactor):**
- `mdatron/.github/workflows/ci.yml` — fmt + clippy + cargo test on ubuntu-22.04 (single OS/arch at 0.1.0).
- `mdatron/.github/workflows/publish.yml` — dry-run on PR, publish on tag; mirror `crosslink/publish.yml:1-70` verbatim, name-swap.
- `vsdd-cli/.github/workflows/vsdd-verify.yml` (replaces `mdatron-verify.yml`) — drops sibling-checkout (`mdatron-verify.yml:24-28`); runs `cargo install mdatron --version "0.1.x" --locked` + `cargo install vsdd --version "0.1.x" --locked` + `vsdd verify --ci-mode`.
- `vsdd-cli/.github/workflows/ci.yml` — fmt + clippy + cargo test; installs mdatron in workflow for integration tests.

**Release artifacts:** `cargo install` only at 0.1.0; GH-Releases matrix stays v1.0 ship-criteria. **MSRV:** both repos pin 1.88 in both `rust-toolchain.toml` + `Cargo.toml`; CI lint enforces agreement (F4 carry-over).

## 4. Init + config (PE lens)

`.mdatron/config.yaml` (v0.1, mdatron-owned): `schema_class_routing` + optional dir overrides + `log_level`. Deferred to v0.2: per-class glob overrides, ignore patterns, exit-code custom mapping.

`.vsdd/config.yaml` (vsdd-owned, composes by file-location convention):
```yaml
schema_version: "1.0.0"
vsdd_version: "0.1.0"
mdatron:
  required_version: ">=0.1.0,<0.2.0"
  pinned_version: "0.1.0"
  pinned_path: "/Users/x/.cargo/bin/mdatron"  # optional; PATH-fallback
log_level: "warn"
ci_mode: false
```

Configs sibling under project root; vsdd does NOT edit `.mdatron/config.yaml`, mdatron does NOT know `.vsdd/` exists. PE-clean: independently audit-able + independently versionable. Discovery project-local only at v0.1 (XDG_CONFIG_HOME globals = v0.2 ergonomics). **Migration v0.1→v0.2:** `schema_version: "1.0.0"` in both; both refuse-with-recovery-hint on unrecognized; migration commands ship at v0.2 bump.

**CI-mode under binary-first:** PE-init-drift F2 carries — `--ci-mode` = `--update --no-prompt --refuse-conflict`. NEW requirement: `--ci-mode` cascades across the process boundary (vsdd passes through to spawned mdatron). SARIF-output path needs non-zero exit on any conflict; enforceable only when `ci_mode` threads end-to-end.

## 5. Cross-process IPC reliability

**Four new failure modes subprocess introduces (library-link did not have):**

1. **Spawn failure** — binary present at init, gone at verify. `VSDD-E0250: spawn failed at '{path}': {err}; reinstall via cargo install mdatron --locked`. Distinct from preflight-missing.
2. **Process crash mid-stream** — partial stdout. `VSDD-E0251: subprocess crashed (exit {n}): {stderr_tail}`. Library-link gave a stack trace; subprocess loses the stack but gains process-isolation (vsdd survives a mdatron panic).
3. **Malformed stdout** — JSON parse fail. `VSDD-E0252: non-JSON output (first 200B: {prefix}); check version compat`. Recovery hint surfaces config-pinned version.
4. **Hang** — DSL infinite loop, FS stall. Configurable timeout (default 60s verify / 30s init); SIGTERM + 5s + SIGKILL; `VSDD-E0253: timed out after {n}s`.

**Wire-format contract (the load-bearing piece):**
- **stdout = JSON only under `--format json`** (mirror crosslink `--json` at `main.rs:84-105`); no progress lines. Under `--format text`, stdout is rustc-shaped TTY output.
- **stderr = human diagnostics always** — tracing, progress, panics. vsdd captures separately + propagates with `[mdatron]` prefix at info+; suppresses to logfile under warn default.
- **Exit codes (mdatron):** 0=clean / 1=error findings / 2=pipeline failure / 64+ reserved internal. Matches current `mdatron-cli/src/main.rs:74-95`; codify in DESIGN-MDATRON.md.
- **vsdd exit mapping:** `max(vsdd_native, mdatron)`; exit 2 on any spawn/parse/timeout regardless of finding count.

## 6. Observability + logging

Both tools need `--log-level` + `--log-format` globals at 0.1.0. PE-mdatron-consistency F7 already required this for mdatron; binary-first makes it load-bearing for vsdd's subprocess triage. Mirror crosslink (`main.rs:84-105, 1856-1870`): `tracing` + `tracing-subscriber` + `EnvFilter`. Env vars: `MDATRON_LOG`, `VSDD_LOG`.

**Cascade:** vsdd exports `MDATRON_LOG=$VSDD_LOG` into subprocess; single `VSDD_LOG=debug vsdd verify` propagates triage through mdatron without per-tool flag-juggling. vsdd inlines mdatron stderr with `[mdatron]` prefix at info+; warn default suppresses to logfile.

**Audit-trail across boundary:** `vsdd init`'s `ProjectInitialized` fires AFTER spawned `mdatron init` returns + after vsdd's own substrate emission completes. Sequence: preflight → spawn `mdatron init` → mdatron emits its own `.mdatron/events.jsonl` `MdatronInitialized` (subprocess-local) → vsdd emits `.vsdd/events.jsonl` `ProjectInitialized` referencing mdatron version + events-log path by file-path (NOT embedded copy). Two logs independently audit-able; no atomic transaction across boundary (mitigation: vsdd writes only after subprocess success; partial-init recovery via `vsdd init --update`).

## 7. Carryover prioritization (pre-0.1.0 blockers)

1. **mdatron LICENSE + README** (PE-mdatron-consistency F1).
2. **mdatron wire-format contract in DESIGN-MDATRON.md** — JSON stdout schema, exit-code semantics, stderr discipline. NEW pre-publish blocker; calcifies at 0.1.0.
3. **mdatron `ci.yml` + `publish.yml`** mirroring crosslink (F5 carry-over).
4. **mdatron `--format json` + `--quiet` + `--log-level` globals** (F2 + F7 carry-over).
5. **vsdd `mdatron_client.rs`** with the four §5 failure modes handled + golden-output integration tests against real spawned binary.
6. **vsdd-verify.yml replaces mdatron-verify.yml** — version-pinned `cargo install` pattern; sibling-checkout retired.

**Release sequencing:** mdatron tags v0.1.0 first → CI publish → vsdd validates `mdatron_min_version` against live entry → vsdd tags v0.1.0 → publish. Document in `docs/release-engineering.md` per repo.

**Container packaging:** NOT v0.1; no adopter justifies it. **Cross-platform release matrix:** stays v1.0 ship-criteria — both tools pure-Rust zero-system-deps; single-OS-single-arch CI suffices at 0.1.0.

---

## Round-close summary

| Cluster | PE verdict | Pre-0.1.0 floor | Phase routing |
|---|---|---|---|
| Boundary design | Net-win on three axes; four new failure modes | Wire-format documented; mdatron_client.rs spawn-tested | Phase 1a → 1b |
| Install + adoption | Strict improvement; cargo install resolves independently | Error catalog VSDD-E0240/0241 | Phase 1b |
| File-tree + Cargo.toml | Single binary crate per repo | LICENSE+README for mdatron; crosslink-shape publish.yml | Phase 1a |
| Init + config | Independent configs composed at file-level | schema_version field both configs | Phase 2b |
| IPC reliability | Robust IF contract is load-bearing-treated | JSON stdout / text stderr / rustc exit codes | Phase 1a |
| Observability | Env-var cascade; stderr inlined with prefix | --log-level + --log-format both tools | Phase 1b |
| Carryover | Six items enumerated; v1.0 deferrals named | Pre-publish blockers ordered | Phase 1a–1b |

**Sycophancy-compensation reflection:** the bias I resisted was treating subprocess as inherently safer because "decoupled". Decoupled at COMPILE time, yes; but four new runtime failure modes library-link did not have. Library-link's worst case is "vsdd panics with mdatron's stack trace"; subprocess's worst case is "vsdd silently misclassifies because stdout was eaten by a buffer-flush bug". Verdict still favors subprocess — release-decoupling + audit-scope + MSRV independence outweigh the new modes — but ONLY when the wire-format contract is publish-blocker at 0.1.0. Defer to 0.2 and the refactor delivers worse coupling than the path-dep it replaced.

**Cross-references:**
- `mdatron/Cargo.toml:1-20`, `mdatron-cli/src/main.rs:15-50, 74-95, 125-145`
- `vsdd-cli/Cargo.toml:14`, `vsdd-core/Cargo.toml:12`
- `vsdd-cli/vsdd/src/preflight.rs:79-114`, `vsdd/src/main.rs:36-82`, `vsdd-core/src/init.rs:51-67, 78-100`
- `vsdd-cli/.github/workflows/mdatron-verify.yml:1-49` (to retire)
- `crosslink/crosslink/Cargo.toml:1-58`, `crosslink/src/main.rs:84-105, 1856-1870`
- `crosslink/.github/workflows/publish.yml:1-70`
- Prior same-day siblings: `review-log/2026-06-02-platform-engineer-mdatron-consistency.md` (F1/F2/F4/F5/F7 carry forward), `2026-06-02-platform-engineer-init-drift.md` (verb-pair + ci_mode threading), `2026-06-02-solution-architect-binary-first-refactor.md` (clusters A+B+D align)
