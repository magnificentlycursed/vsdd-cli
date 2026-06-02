---
schema_class: review-entry
schema_version: 1.0.0
review_number: 2
date: 2026-06-02
phase: phase-1c
scope: >-
  Design opinion on vsdd init drift / collision / upgrade UX for v0.1 —
  flag surface (--update vs implicit re-init), CI-vs-local-dev hard-fail
  asymmetry, multi-substrate composition with crosslink, missing-or-corrupt
  manifest disposition. Surface under review: vsdd-core/src/init.rs:79-202,
  vsdd/src/main.rs:25-82, DESIGN-METHODOLOGY.md:816-862 +
  crosslink/crosslink/src/commands/init/manifest.rs +
  crosslink/crosslink/src/commands/init/mod.rs:417-683.
lens: Platform Engineer (primary) + Sanity Check (baseline) — PE dim 1 (Reproducible builds), dim 4 (CI workflow discipline), dim 7 (Observability of CI itself); Sanity dim 3 (rubber-ducking the install-a-year-from-now question).
source: operator-directive
session_note: >-
  Cold-session opinion work — no prior context on the init drift discussion;
  citations grounded in current vsdd-core/src/init.rs line-numbers + crosslink
  manifest.rs line-numbers as inspected this session. The DESIGN-METHODOLOGY.md
  spec at 818-870 names flags (--keep-operator-edits / --accept-managed-defaults)
  that the crosslink substrate does NOT use; the prompt frames that as a spec
  hallucination, and the opinion threads that explicitly.
model: claude-opus-4-7
execution_method: >-
  single-domain cold-session opinion (PE primary lens + Sanity-Check baseline);
  no code modifications; review-log entry only.
sycophancy_compensation: >-
  PE-lens bias is "explicit + observable wins"; the v0.1-ergonomic counter-bias
  is "any flag surface is friction." I am resisting the second by naming the
  hard-error-on-drift current behavior as operator-hostile in the discoverability
  axis even though it is observable. The opinion grades the trade-off rather
  than declaring one path obviously correct.
supplements_loaded: []
---

# Platform Engineer Opinion — vsdd init drift / collision / upgrade (Phase 1c)

## Headline

v0.1 should ship crosslink's three-way model verbatim: `vsdd init` is first-run-only; `vsdd init --update` is the only safe-upgrade path; `--force` overwrites unconditionally; hard-error-on-drift WITHOUT `--update` is fine and adopter-friendly because the error names the next command. CI-mode is `--update --no-prompt` (refuse to write on conflict, exit non-zero). The spec's `--keep-operator-edits` / `--accept-managed-defaults` flags are unimplementable as written and should be retired from DESIGN-METHODOLOGY.md:854 in favor of crosslink's verb-pair.

## Mechanical citations

- vsdd current impl (two-way, hard-error): `vsdd-core/src/init.rs:113-125` — compares `prior_entry.sha256` vs `actual_sha`; on mismatch returns `InitError::ManagedFileDrifted` with the hallucinated flag names from the spec.
- Corrupt-manifest silent path: `vsdd-core/src/init.rs:260-272` (`load_manifest`) — `serde_json::from_str` failure returns `Ok(None)`; the comment at 264-266 names the discipline ("Corrupt manifest: treat as no prior init") but the operator-visible surface is zero.
- ci_mode currently a no-op: `vsdd-core/src/init.rs:80` (`let _ = options;`) — the flag parses (`vsdd/src/main.rs:31-33`) and threads through `InitOptions { ci_mode }` (main.rs:64-66) but does not branch any behavior.
- Crosslink three-way classifier: `crosslink/.../init/manifest.rs:32-48` (`UpdateAction` enum) + `manifest.rs:136-152` (`classify_update`). Five real outcomes: UpToDate / AutoUpdate / TemplateUnchanged / Conflict / Deleted (+ NewFile set out-of-band).
- Crosslink corrupt-manifest disposition: `manifest.rs:77-81` returns `None` silently; the warn-surface is at `init/mod.rs:443-450` — `--update` flow prints "No init-manifest.json found — treating all managed files as potentially modified" + a recovery hint. The WARN is in the orchestrator, not the loader.
- Crosslink interactive-prompt: `init/mod.rs:607-635` — `Conflict` files prompt `y/N`; `!is_tty` skips silently with detail-line.
- Crosslink `--update` refusal-when-uninitialized: `init/mod.rs:427-431` — `bail!("Project not initialized. Run \`crosslink init\` first...")`. Inverse of the first-run path.
- Spec's hallucinated flags: `DESIGN-METHODOLOGY.md:854` — `--keep-operator-edits` / `--accept-managed-defaults`. Neither exists in crosslink's surface. The vsdd error at `init.rs:55-58` already cites them in the error string.

## Findings

### F1 — Adopt crosslink's verb-pair (`init` first-run / `--update` safe-upgrade) verbatim; reject the implicit-detection path

**PE rationale (dim 4 + dim 7):** implicit upgrade detection ("re-running init magically does the right thing") is the wrong default for the "operator runs `vsdd init` a year from now after 3 toolkit upgrades" scenario. The implicit path conflates two operator intents (fresh-deploy vs upgrade-in-place) under one verb; the failure mode when the intent guess is wrong is silent over-write of operator edits OR refuse-to-proceed-with-no-recovery-path. Crosslink's verb-pair makes the intent declarative: `init` says "I expect a clean substrate"; `--update` says "I expect a prior manifest + a possibly-edited tree." The error message routes between them.

**Sanity-check rubber-duck:** "Operator runs `vsdd init` a year from now; toolkit shipped 3 upgrades; one managed file edited." Current impl: `ManagedFileDrifted` error citing hallucinated flag names + no path forward. Adopting crosslink's model: error says "Project already initialized; use `--update` for safe upgrade or `--force` to overwrite." Operator runs `--update`; gets the conflict report at `init/mod.rs:497-558`; resolves via y/N. The verb-pair makes the recovery path trivially discoverable.

**Classification:** Accepted (load-bearing v0.1 design).

---

### F2 — `--ci-mode` should be a behavior modifier on `--update`, not a top-level mode; current parse-but-no-op is a foot-loaded gun

**PE rationale (dim 4):** CI's correct disposition on drift is hard-fail-non-zero — exactly the current default. But CI rarely runs first-init (the runner clones a repo that already has `.vsdd/` from the operator's local init); the CI path is always upgrade-shaped. So `--ci-mode` is operationally `--update --no-prompt --refuse-conflict`. Currently the flag parses (`main.rs:31-33`) + threads through (`main.rs:64-66`) + does nothing (`init.rs:80`); that is the PE sycophancy-failure-mode "CI green is the only signal — CI exit code 0 conflated with discipline-held." The flag claims to enforce a discipline; the discipline isn't wired.

**Concrete v0.1 wiring:** in `--ci-mode`, refuse-on-any-conflict-or-deleted (exit 1 with VSDD-E0230); auto-apply AutoUpdate + NewFile classes; report counts as JSONL event for the observability path. NEVER prompt. NEVER silent-skip. The CI auth-method-conditional steps named in the PE prompt (dim 4) need a non-zero exit to fire the SARIF-output path.

**Classification:** Accepted-pending implementation. Routes Phase 4 → Phase 1b.

---

### F3 — Hallucinated flag names in `DESIGN-METHODOLOGY.md:854` should be retired; the error string at `init.rs:55-58` already propagates the hallucination to operator-visible surface

**PE rationale (dim 1, reproducibility-of-spec-to-impl):** the spec asserts a flag surface that crosslink (the named substrate of the discipline) does not implement. The current vsdd impl synthesizes that surface in the error string. A year-from-now operator googles `--keep-operator-edits`; zero results because no tool implements it. This is the "Build pinned to 'latest' — reproducibility traded for ergonomics" failure-mode inverted — the spec is pinned to a vocabulary the substrate doesn't speak.

**Recommendation:** spec change at DESIGN-METHODOLOGY.md:854 to name crosslink's actual verb-pair (`--update`, `--force`, `--dry-run`, `--no-prompt`) + the resolution-path semantics ("conflict surfaces a y/N prompt; --no-prompt + conflict = refuse"). Error string at `init.rs:55-58` rewrites to: `"managed file drifted at {path}; use \`vsdd init --update\` for an interactive resolution or \`--force\` to overwrite (expected sha256 {expected}, got {actual})"`.

**Classification:** Resolved-pending (spec edit + error-string edit; mechanical).

---

### F4 — Corrupt-manifest silent path is wrong for PE-dim-7 (observability); mirror crosslink's warn-with-recovery-hint

**PE rationale (dim 7):** `init.rs:260-272` treats a corrupt manifest as "no prior init" silently. The comment names the intent ("Corrupt manifest: treat as no prior init. Operator can recover by running init fresh") but the operator never sees this — they observe `vsdd init` quietly proceeding as if first-run and writing fresh files. If those writes overwrite operator edits (because they're not in the manifest, no drift check fires), data is lost without a warn-trail.

Crosslink's discipline (`init/mod.rs:443-450`) prints two lines when the manifest is missing-or-corrupt under `--update`: the warn ("treating all managed files as potentially modified") + the recovery hint ("Use `crosslink init --force` instead to overwrite all managed files"). That two-line output is the entire observability surface — load-bearing under PE-dim-7 because the CI's audit trail captures it.

**v0.1 fix:** distinguish "manifest absent" (first-init, no warn needed) from "manifest present + corrupt" (warn + emit a `ManifestCorrupted` event + suggest `--force`). One-line at `init.rs:267` to differentiate the Err branch from the NotFound branch in `load_manifest`.

**Sanity-check rubber-duck:** "What does a CI run looking at events.jsonl learn from the silent path?" Nothing — the ProjectInitialized event fires (because `is_first_init` resolves true on corrupt manifest) as if it were genuinely first-init, polluting the audit trail. PE-sycophancy-failure-mode: "Deployment artifact built once, never reproduced — supply-chain attestation gap." A `ProjectInitialized` event lying about its triggering condition IS an attestation gap.

**Classification:** Resolved-pending (1-line + event variant). Routes Phase 4 → Phase 1b.

---

### F5 — Do NOT wrap `crosslink init --update`; vsdd + crosslink are parallel disciplines that compose at file-level, not command-level

**PE rationale (dim 4 + dim 8, cross-platform binary builds):** vsdd manages `.vsdd/`, `.claude/commands/vsdd-*`, `.mdatron/`, supplements; crosslink manages `.crosslink/`, `.claude/settings.json` (merged), `.claude/hooks/*`, `.mcp.json` (merged). The composition surface is the merge-into-existing files (`.claude/settings.json` allowedTools union; `.mcp.json` mcpServers; `.gitignore` managed-section). Wrapping `crosslink init --update` would couple vsdd's release cadence to crosslink's command surface; a crosslink breaking change to `--update` semantics would break `vsdd init`. That's the "dependency added because it's convenient — supply-chain risk unaccounted" sycophancy failure mode.

**v0.1 disposition:** vsdd's preflight (`preflight.rs:33`) already detects crosslink-on-PATH. Add a post-init hint when crosslink IS present but `.crosslink/` is absent: "crosslink detected on PATH; consider \`crosslink init\` to compose the issue-tracking discipline." Do NOT auto-invoke. The composition is at the merge-discipline level (vsdd's `.claude/settings.json` merge respects crosslink's hook entries; `DESIGN-METHODOLOGY.md:826` already names this).

**Classification:** Accepted (separation of concerns).

---

### F6 — `--dry-run` should ship in v0.1 (matches crosslink `--update --dry-run` at init/mod.rs:560-565); current `--check` is the preflight, not the deployment plan

**PE rationale (dim 4):** crosslink's `--update --dry-run` reports the classification matrix (auto-update / new / conflict / deleted) without writing. vsdd currently has `--check` (`main.rs:28-29`) but it short-circuits at preflight (`main.rs:59-62`) — operator sees "git/crosslink/mdatron/cargo present" but NOT the deployment plan. The discoverability gap is "what would `vsdd init` actually write?" — currently unanswerable without running it.

**v0.1 fix:** `--check` becomes "preflight only" (current behavior); add `--dry-run` (or `--plan`) that runs preflight + builds the deployment plan + classifies each file against any existing manifest + reports counts. No writes. Matches crosslink's verb shape; the spec at DESIGN-METHODOLOGY.md:856-858 already names this as `vsdd init --check`. Rename consideration: keep `--check` = preflight; add `--dry-run` = preflight + classification report. OR consolidate under `--check` with verbose output. Either is fine; the LOAD-bearing requirement is the classification report.

**Classification:** Resolved-pending. Routes Phase 4 → Phase 1b.

---

## Round-close summary

| Finding | Domain | Classification | Routing |
|---|---|---|---|
| F1 | PE + Sanity | Accepted | v0.1 design — verb-pair adoption |
| F2 | PE | Accepted-pending | Phase 4 → Phase 1b (wire ci_mode) |
| F3 | PE | Resolved-pending | Spec + error-string edit |
| F4 | PE + Sanity | Resolved-pending | Phase 4 → Phase 1b (1-line + event) |
| F5 | PE | Accepted | Composition discipline (no wrapping) |
| F6 | PE | Resolved-pending | Phase 4 → Phase 1b (dry-run report) |

**Sycophancy-compensation reflection:** the bias I resisted was treating "hard-error on any drift" as PE-correct because it's observable + safe. It IS observable + safe, but it stops at "safe for the toolkit" — it does not pass "discoverable for the operator a year from now." Crosslink's verb-pair makes the recovery path declarative + the error string self-documenting. That is the load-bearing PE win, not the hard-error.

**Cross-references:**
- `vsdd-core/src/init.rs:55-67, 80, 113-125, 260-272` (current impl surface)
- `vsdd/src/main.rs:25-82` (CLI surface; ci_mode threading)
- `vsdd/src/preflight.rs:33, 79` (crosslink detection)
- `crosslink/crosslink/src/commands/init/manifest.rs:32-48, 77-81, 136-152` (three-way classifier; corrupt-manifest)
- `crosslink/crosslink/src/commands/init/mod.rs:217-229, 417-683` (`InitOpts` + `run_update`)
- `DESIGN-METHODOLOGY.md:816-862` (collision-handling spec; hallucinated flags at 854)
