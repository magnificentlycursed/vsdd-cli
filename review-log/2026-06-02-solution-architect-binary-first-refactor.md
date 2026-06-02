---
schema_class: review-entry
schema_version: 1.0.0
review_number: 2
date: 2026-06-02
phase: phase-1a
scope: >-
  Refactor proposal — collapse mdatron-workspace (mdatron-core lib +
  mdatron-cli bin) and vsdd-cli-workspace (vsdd-core lib + vsdd bin) into
  crosslink-shaped single-binary crates, with vsdd consuming mdatron as a
  subprocess (mdatron verify --format=json) rather than a library. Re-clusters
  the 20+ in-flight dispositions accumulated this session under the new
  directive. Files in scope: mdatron/{mdatron-core,mdatron-cli}/**,
  vsdd-cli/{vsdd-core,vsdd}/**, plus DESIGN-MDATRON.md +
  DESIGN-OBSERVABILITY.md spec amendments. No code modification; design
  draft only.
lens: >-
  Solution Architect primary (decomposition coherence + abstraction altitude +
  hard-to-undo decisions + cross-process trust boundary + cross-cutting
  observability + reversibility framing). Sanity Check baseline — rubber-ducking
  the directive against the prior 2026-06-02 SA opinion (which assumed
  library-consumed mdatron) and naming where conclusions invert.
source: operator-directive
session_note: >-
  Cold-session single-domain composition (SA-primary + Sanity-Check baseline).
  No prior session memory; required referent files re-read fresh —
  vsdd-domain-solution-architect.md, vsdd-domain-sanity-check.md,
  review-entry.json, prior 2026-06-02-solution-architect-mdatron-consistency.md.
  Operator directive: binary-first, subprocess-consumed, crosslink-shaped.
  Prior SA opinion's "thiserror at core, anyhow at CLI" verdict is superseded
  by "anyhow throughout" — the lib/CLI split no longer exists. Filed as
  phase-1a because this is the implementation-shape proposal that gates
  mdatron init authoring + the vsdd-side subprocess client.
model: claude-opus-4-7
execution_method: >-
  cold-session sub-agent dispatched from main session; SA + Sanity Check
  prompts + review-entry schema + prior same-domain sibling loaded fresh.
sycophancy_compensation: >-
  The cheap path is to mirror crosslink's shape verbatim and declare the
  refactor done. I pressure-test below where mdatron's needs diverge — its
  output is machine-consumed (vsdd parses it) where crosslink's is
  human-consumed (operator TUI). The cross-process contract is the
  hard-to-undo dimension this proposal must name explicitly, not absorb
  under "we'll just follow crosslink".
filename_note: >-
  Slug `binary-first-refactor` to distinguish from prior same-day
  `mdatron-consistency` (library-era opinion) and `init-drift` (vsdd init
  spec-drift opinion).
supplements_loaded: [rust, cli]
---

# Solution Architect Proposal — Binary-First Crosslink-Cued Refactor

**Phase 1a design draft. No code modification. SO disposes; SE consumes for implementation.**

---

## Headline

Collapse both workspaces into single binary crates. mdatron exposes a versioned `mdatron verify --format=json` contract; vsdd shells out and parses stdout. The 20 dispositions re-cluster into seven work-clusters; cluster A (cross-process contract) is the load-bearing, hard-to-undo one and must land before vsdd's subprocess client.

---

## 1. Re-clustered work-clusters

**Cluster A — Cross-process contract (load-bearing, hard-to-undo).**
NEW (subprocess refactor) + #5 (DESIGN-MDATRON.md::concat + ProjectInitialized payload) + #13 (`--quiet`/`--json`/`--log-level` globals — now load-bearing for vsdd's consumption) + #14 (`= explain:` line — strip; not in JSON contract) + #19 (vocabulary discipline — JSON field names ARE the public surface). Defines mdatron's stdout/stderr/exit-code shape as a versioned wire format. Everything else waits on this.

**Cluster B — Workspace collapse + structure.**
Folds prior #16 (anyhow throughout, no longer "at CLI boundary"). Includes the `mdatron-core` rename/absorption, the `src/commands/<verb>/mod.rs` reorg, include_str!-embedded artifacts, and #11 (DSL style cleanups landing as collateral during file moves).

**Cluster C — Error-code & catalog hygiene.**
#1 (E-code reallocation per DESIGN-MDATRON.md:116-117) + #20 (E0001/E0002 rename + code-allocation lint + spec amendment). Becomes more urgent: codes ship in the JSON contract, so renumbering is a wire-format break post-1.0. Land before any tagged release.

**Cluster D — init + config formalization.**
#8 (vsdd templates: implement) + #17 (mdatron init v0.1 scope) + #3b (vsdd init drift error text). Detailed in §4. mdatron init lays `.mdatron/`; vsdd init layers `.vsdd/` on top and *invokes* `mdatron init` as a subprocess for the substrate half.

**Cluster E — Test & spec hardening.**
#3a (should-fire cross-ref fixtures) + #9 (stdlib undertest: join, set-ops, concat edges) + #10 (`defined()` empty-string carve-out) + #15 (mdatron-cli/tests/cli_integration.rs — rewritten as subprocess-output golden tests, the new contract test surface).

**Cluster F — Audit & methodology.**
#4 (Phase 3 cluster-batched amendment) + #6 (events.jsonl truncation, OperatorDirectiveApplied retroactive, branch-protection) + #2 (DSL field-access symmetry — unaffected by refactor; lands as a leaf fix).

**Cluster G — Release & docs.**
#7 (CI/install reconciliation) + #12 (mdatron README) + #18 (publish mdatron 0.1.0 to crates.io — gated on Cluster A frozen contract).

**Retired:** thiserror-vs-anyhow debate (directive settled it — anyhow throughout; the prior SA opinion's split-paradigm verdict is null).

---

## 2. Absorbed file trees

### mdatron (single crate `mdatron`)

```
mdatron/
  Cargo.toml                       # [package] mdatron; no [workspace]
  src/
    main.rs                        # ~80 LoC — clap dispatch, anyhow::Result
    cli.rs                         # clap derives, global flags (--json/--quiet/--log-level)
    commands/
      mod.rs
      init/
        mod.rs                     # `mdatron init` orchestrator (~400 LoC)
        scaffold.rs                # writes .mdatron/{config.yaml,schemas/,patterns/}
        templates.rs               # include_str! constants for starter schemas
      verify/
        mod.rs                     # `mdatron verify` orchestrator
        runner.rs                  # walk files, dispatch to dsl::evaluate
        emit.rs                    # rustc-shape stderr OR JSON stdout
      explain/
        mod.rs                     # `mdatron explain MDATRON-E0011`
        catalog.rs                 # E-code -> description table
    dsl/
      mod.rs                       # parser + evaluator + index — internal module
      expr.rs                      # (was mdatron-core/src/dsl/expr.rs)
      expr_parser.rs
      parser.rs
      types.rs
      index.rs
      stdlib.rs                    # join, concat, set-ops (split out for #9 coverage)
    schema/
      mod.rs                       # frontmatter schema dispatch (was schema.rs)
      frontmatter.rs               # (was frontmatter.rs)
    diagnostic/
      mod.rs                       # Finding type + format_tty + format_json
      tty.rs                       # rustc-shape rendering
      json.rs                      # versioned JSON envelope (see §3)
    config/
      mod.rs                       # .mdatron/config.yaml loader + precedence
      schema.rs                    # typed Config struct, JSON-Schema-self-validating
    error_catalog.rs               # MDATRON-E0001..E0080 — single source (Cluster C)
    embedded/
      mod.rs                       # include_str! exports
      starter_schema.yaml          # for `mdatron init`
      config_template.yaml
  tests/
    cli_integration.rs             # golden tests against JSON contract (Cluster E)
    fixtures/                      # .md files + expected JSON output
```

**Diverges from crosslink intentionally:** `dsl/` and `schema/` stay as internal modules, NOT a workspace-private `mdatron-dsl` crate. Reason: ~1500 LoC of evaluator does not justify a separate crate when no other consumer exists. If a third consumer appears (e.g., a Tauri preview app), revisit. Documented as a reversibility note in DESIGN-MDATRON.md.

**Why not preserve mdatron-core for proptest/fuzz isolation?** Proptest and cargo-fuzz both target internal modules fine; `pub(crate)` + `#[cfg(test)]` carry the discipline. The workspace existed to publish a library API; the directive killed that purpose.

### vsdd (single crate `vsdd`)

```
vsdd/
  Cargo.toml                       # [package] vsdd; no [workspace]; NO mdatron dep
  src/
    main.rs
    cli.rs
    commands/
      mod.rs
      init/
        mod.rs                     # vsdd init — invokes mdatron init then layers vsdd content
        mdatron_bridge.rs          # spawns `mdatron init` subprocess
        templates.rs               # include_str! VSDD templates (Cluster D #8)
        drift.rs                   # drift detection (corrects #3b — no --keep-operator-edits)
      verify/
        mod.rs                     # vsdd verify = spawn `mdatron verify --format=json` + interpret
        mdatron_client.rs          # THE subprocess client (§3)
        interpret.rs               # vsdd-side semantic layer over mdatron findings
      observe/
        mod.rs                     # events.jsonl tail + ProjectInitialized emission (#5, #6)
        events.rs
    config/
      mod.rs                       # .vsdd/config.yaml
    embedded/
      mod.rs
      design_methodology.md        # include_str! of methodology.md
      design_schema.md
      review_entry_schema.json
  tests/
    cli_integration.rs
    fixtures/
```

---

## 3. Cross-process boundary design (Cluster A — the load-bearing one)

**Invocation.** vsdd resolves mdatron via: `MDATRON_BIN` env > `.vsdd/config.yaml::mdatron_path` > `PATH` lookup of `mdatron`. No bundled vendoring in v0.1; vsdd reports a fixable error if mdatron is absent. Documented as trust boundary in DESIGN-VERIFICATION.md (SA dim 4 — subprocess output is untrusted-until-parsed; JSON envelope is the validation gate).

**Wire format.** mdatron emits both, gated by `--format`:
- `--format=tty` (default): rustc-shape stderr findings + human summary on stdout. Operator-facing.
- `--format=json`: machine-readable envelope on stdout, NOTHING on stderr except fatal errors. vsdd-facing.

JSON envelope (versioned, `wire_version` field — separate from mdatron's SemVer):

```json
{
  "wire_version": "1.0",
  "mdatron_version": "0.1.0",
  "run_id": "uuid",
  "findings": [{"code": "MDATRON-E0011", "severity": "error", "file": "...", "span": {...}, "message": "...", "schema_class": "..."}],
  "summary": {"files_checked": 42, "findings": 3, "duration_ms": 187}
}
```

**Exit codes (versioned):** `0` clean, `1` findings present, `2` mdatron internal error (panic/IO/config), `3` invocation error (bad flags). vsdd matches on these; renumbering is a wire-format break.

**SemVer discipline for the contract.** Two version numbers: `mdatron_version` (crate SemVer; tracks any change) and `wire_version` (only bumps on field-add/remove/rename in the JSON envelope or exit-code remap). vsdd's `mdatron_client.rs` asserts `wire_version` major-match at startup; on mismatch, emits actionable error. Codified in DESIGN-MDATRON.md § Public contracts.

**#13 becomes load-bearing.** `--json` is no longer a convenience — it IS vsdd's only supported consumption path. `--quiet` suppresses summary; `--log-level` controls mdatron's own diagnostics (separate from findings). All three ship in v0.1 of the contract.

**#14 (`= explain:`).** Strip from tty output; the catalog is the source of truth via `mdatron explain CODE`. Not in JSON contract.

---

## 4. init + config cluster

**mdatron init v0.1 — disposition B.2 + B.3 (empty-skeleton + formalized config.yaml).** Not B.1 (starter-scaffold) — starter content belongs in `mdatron init --with-examples`, not the default. Reason: vsdd init will call `mdatron init` and does NOT want starter clutter polluting the vsdd-managed directory.

`.mdatron/config.yaml` v0.1 shape:
```yaml
version: "0.1"                    # required, enables migration
schemas_dir: .mdatron/schemas     # path, default shown
patterns_dir: .mdatron/patterns
file_globs: ["**/*.md"]
severity_overrides: {}            # reserved namespace, empty in v0.1
```

**vsdd init.** Spawns `mdatron init` first (substrate), then layers `.vsdd/` + design docs. The "substrate concern" shrinks per the directive. `.vsdd/config.yaml` v0.1:
```yaml
version: "0.1"
mdatron_path: null                # null = PATH lookup
methodology_pin: "1.0.0"          # which DESIGN-METHODOLOGY.md version this project agrees to
review_log_dir: review-log
```

**#3b (drift error text).** Fix as part of Cluster D landing — drop the hallucinated `--keep-operator-edits` flag; the actual mechanism is operator-edits-detected → prompt or fail. Amend DESIGN-METHODOLOGY.md.

---

## 5. Order of operations

1. **Cluster A first.** Specify the JSON wire format + exit codes + flag surface in DESIGN-MDATRON.md. Nothing else can land coherently until the contract is named.
2. **Cluster C second.** E-code renumbering must happen BEFORE Cluster A freezes — codes ship in the contract.
3. **Cluster B third.** Workspace collapse + structure reorg. Mechanical; large diff but low semantic risk if A+C are settled.
4. **Cluster D fourth.** init implementations consume the post-collapse structure.
5. **Cluster E parallel with D.** Golden tests for the JSON contract become the regression net.
6. **Cluster F parallel with anything** (audit/methodology — independent).
7. **Cluster G last.** Release + crates.io publish — gated on A frozen, E passing.

**Sequencing risk:** if Cluster B lands before A, the subprocess client gets rewritten twice. If Cluster G publishes before A is frozen, the wire contract is locked at the wrong shape and downstream consumers depend on it.

---

## 6. Hard-to-undo dimensions

**Lock-in (treat as 1.0 commitments):**
- JSON envelope field names + types (Cluster A) — once vsdd ships against them, breaking changes cost a coordinated release.
- Exit code semantics (0/1/2/3) — same.
- E-code numbers (Cluster C) — embedded in user-written rules + cited in vsdd findings.
- `.mdatron/config.yaml` top-level key names + `version` field semantics.
- The decision that vsdd consumes mdatron via subprocess (the directive itself) — reversing requires re-introducing the library dep and re-splitting workspaces.

**Reversible (defer rigor):**
- Internal module layout under `src/` — can be reorganized any time.
- Whether `dsl/` becomes its own crate later — internal refactor.
- TTY rendering shape (non-JSON path) — human-facing; can iterate.
- `--with-examples` and other init sub-flags — additive.
- mdatron README content (#12).

**Pressure point (SA dim 5):** the wire contract IS the architecture now. Crosslink doesn't have this problem — its only "consumer" is the operator. mdatron's binary-first identity is structurally different from crosslink's because mdatron's output is machine-consumed. Mirroring crosslink's shape for the *organization* of code is right; mirroring it for the *output discipline* would be wrong. Name this divergence in DESIGN-MDATRON.md so future-architect-six-months-out sees the seam.

---

## Routing

- **Raise to SO** for DESIGN-MDATRON.md amendments: JSON wire format spec, exit-code table, `wire_version` SemVer discipline, `.mdatron/config.yaml` schema, "binary-first but machine-consumed" divergence note from crosslink.
- **Raise to SO** for DESIGN-METHODOLOGY.md: drop hallucinated `--keep-operator-edits`; document vsdd-init-invokes-mdatron-init substrate handoff.
- **To SE** for Cluster A implementation gating + Cluster B workspace-collapse mechanics.
- **To QE** for Cluster E golden-test scaffold against JSON contract.
- **To PE** for Cluster G CI/install reconciliation (mdatron-on-PATH discovery in vsdd CI).

---

## Cross-references

- Prior superseded opinion: `vsdd-cli/review-log/2026-06-02-solution-architect-mdatron-consistency.md` (library-era split-paradigm verdict — now null per directive)
- `crosslink/crosslink/src/commands/init/mod.rs` (2506 LoC reference for `src/commands/<verb>/mod.rs` shape)
- `crosslink/crosslink/src/main.rs` (anyhow + clap dispatch baseline)
- `mdatron/mdatron-core/src/dsl/` (~1500 LoC absorbed as `src/dsl/` internal module)
- `mdatron/mdatron-cli/src/main.rs` (140 LoC absorbed into `src/main.rs` + `src/commands/`)
- `vsdd-cli/vsdd-core/src/init.rs` (absorbed into `vsdd/src/commands/init/`)
- `vsdd-cli/vsdd/src/main.rs` (120 LoC; expands with subprocess client)
- DESIGN-MDATRON.md:116-117 (E-code spec — Cluster C target)
- DESIGN-MDATRON.md:243 (missing concat() — Cluster A spec patch)
- DESIGN-OBSERVABILITY.md:270 (ProjectInitialized payload — Cluster A reconciliation)
