---
schema_class: review-entry
schema_version: 1.0.0
review_number: 2
date: 2026-06-02
phase: phase-1c
scope: >-
  vsdd init v0.1 drift / collision / upgrade handling — disposition opinion on the
  spec/implementation mismatch surfaced by review-log/2026-06-02-solution-owner.md
  F3 (drift error names flags `--keep-operator-edits` / `--accept-managed-defaults`
  that don't exist in the CLI). Subject — DESIGN-METHODOLOGY.md:818-870 (Adoption
  into existing projects — collision handling) + vsdd-core/src/init.rs:51-67
  (ManagedFileDrifted error variant) + crosslink reference impl at
  crosslink/src/commands/init/{manifest,merge,mod}.rs as the inheritance source.
lens: >-
  Solution Owner primary (scope-discipline + spec-contract authority + Raise-to-SO
  routing). Sanity Check baseline (cross-finding coherence with prior round's F3 +
  F5 + F10). Phase 1c framing — acceptance-criteria scoping, NOT Phase 3
  adversarial finding-raising.
source: operator-directive
session_note: >-
  Cold-session single-domain composition (SO + Sanity Check baseline). Director
  asked for an SO opinion on the v0.1 drift-handling disposition surfaced by the
  prior round's F3. Scope is acceptance-criteria-shaped (what does "init done"
  include for v0.1 vs v0.2) — explicitly NOT a new finding round. Operator named
  the implementer-bias to pressure-test ("simplest disposition is to delete the
  flag references + defer drift handling — is that scope discipline or elision?").
model: claude-opus-4-7
execution_method: >-
  Cold-session single-agent dispatched from main session; SO + Sanity Check
  domain prompts loaded fresh; mechanical-citation discipline (line numbers +
  cross-references against crosslink/src/commands/init/ as the inheritance
  baseline); no prior-cycle context beyond the explicitly-named read-first set.
sycophancy_compensation: >-
  Operator pre-named the implementer-bias path ("delete the flag references +
  call drift deferred"). The pressure-test: does that protect scope-discipline
  or elide a real adopter requirement? The SO position below adopts neither the
  operator-named easy path nor the maximalist "implement crosslink-mirror"
  path — it splits on the v0.1 adopter-shape question (vsdd-cli itself is the
  only realistic adopter; v0.1 is developer-test-ready, not production-adopter-
  ready) and recommends a third disposition the operator did not name. That
  third disposition is testable against the v0.2 adopter shape: if a non-
  vsdd-cli adopter materializes before drift discipline ships, the cut was wrong.
filename_note: >-
  Filed `<date>-solution-owner-init-drift.md` (suffixed to disambiguate from
  same-date `2026-06-02-solution-owner.md` Phase 3 entry). Per the existing
  same-date precedent (2026-06-02-software-engineer.md + -mdatron-dsl-catchup.md),
  suffix-distinguishes scope when one domain produces multiple entries in a day.
supplements_loaded: []
---

# Solution Owner Opinion — vsdd init v0.1 drift handling disposition

**Phase:** 1c (acceptance-criteria scoping). Not Phase 3 adversarial.

## The question being opined on

Prior round's F3 (review-log/2026-06-02-solution-owner.md:100-113) surfaced: vsdd-core/src/init.rs:55-58 names recovery flags `--keep-operator-edits` / `--accept-managed-defaults` that don't exist in the CLI; DESIGN-METHODOLOGY.md:854 names the same flags as the documented discipline; crosslink's actual three-way classification (crosslink/src/commands/init/manifest.rs:32-48 — UpToDate/AutoUpdate/TemplateUnchanged/Conflict/Deleted/NewFile) uses interactive prompts (crosslink/src/commands/init/mod.rs:605-611), NOT named flags. The spec inherited from a baseline that doesn't have what the spec claims.

Three candidate dispositions: (a) amend spec to match crosslink (drop flag names; use interactive prompts); (b) implement spec's flags faithfully (build what crosslink doesn't have); (c) keep both honest at v0.1 — neither — and document the v0.1 limitation.

## SO opinion: disposition (c), with a tightening

**Recommendation: ship v0.1 with refusal-on-drift as the only drift behavior. Delete the named flags from BOTH the error message AND the spec at :854. Add explicit v0.2 acceptance criteria for drift resolution. Test refusal as the contract.**

This is NOT "operator-bias path (a)" (which is delete-from-error-only + leave spec inconsistent). It tightens the spec at the same time so the spec is honest about what v0.1 ships.

### Rationale

**Adopter shape (load-bearing argument).** vsdd-cli is currently the only realistic v0.1 adopter. v0.1's job is developer-test-ready — proving the toolkit can deploy itself into its own substrate, idempotently, with a manifest. Drift in v0.1 happens only when the implementer hand-edits a deployed artifact between toolkit runs; the implementer can always blow away `.vsdd/init-manifest.json` and re-init. **No external adopter exists yet who would hit drift as an unrecoverable state.** Refusing-on-drift IS the v0.1 contract; the recovery is "delete the manifest entry or revert the file." That's not elegant, but it's honest, and it does not block any v0.1 adopter shape.

**Crosslink-mirror cost-benefit (asymmetric).** Crosslink's surface includes `.gitignore` (managed-section markers), `.mcp.json` (JSON merge), `.claude/settings.json` (UNION merge of `allowedTools`) — files where partial user edits ARE the expected case (merge/manifest.rs + merge/merge.rs:91-228). The vsdd-cli corpus is 4 schemas + 1 pattern + 42 markdown — fixed canonical artifacts where user-edits-in-managed-files is not the expected case; it's an exception. The cost of implementing three-way classification + interactive prompts (~200 LoC + interactive test harness + non-`--ci-mode` path) is the same; the benefit is dramatically lower for a fixed-canonical-artifact corpus. **DESIGN-METHODOLOGY.md:818 inherits the discipline name from crosslink but inherits a use-case shape that doesn't match.** This is the load-bearing observation: inheritance was named generally; it should have been named per-file-class.

**Scope-discipline test.** The original session ordering ("vsdd init done → mdatron usage finished → tech debt") sets "init done" as a milestone boundary. Implementing the full crosslink-mirror collision discipline now expands the milestone beyond what the v0.1 adopter (vsdd-cli itself) actually needs. That IS scope creep — defensible only if a Phase 1c acceptance-criteria reading says drift discipline is gate-in for "init done." It is not. The DESIGN-METHODOLOGY.md:818-870 section names what `vsdd init` SHOULD eventually do; it does not gate the v0.1 cut.

**Hallucination disposition.** DESIGN-METHODOLOGY.md:854 names flags that do not exist in crosslink (the claimed inheritance source) AND do not exist in vsdd (the implementation). That's a two-sided hallucination — the spec invented flag names while claiming inheritance from a source that has different recovery shape. Disposition (a) "amend spec to match crosslink" propagates crosslink's interactive prompt shape into vsdd-cli, which (per cost-benefit above) is a worse fit than refusal-on-drift for a fixed-canonical-artifact corpus. Disposition (b) "implement the spec's flags faithfully" builds infrastructure for a v0.2 adopter who doesn't exist yet. **Disposition (c) is the only path that doesn't bake in a wrong-shape decision at v0.1.**

### Acceptance criteria for "vsdd init done" at v0.1 (Phase 1c)

1. Refusal-on-drift is the contract. `InitError::ManagedFileDrifted` fires with: drifted path, expected SHA, actual SHA, and a recovery hint pointing at manifest-deletion OR file-revert as the v0.1 recovery (no flag names).
2. DESIGN-METHODOLOGY.md:854 amended to: "Re-running `vsdd init` after toolkit upgrade replaces managed sections + merges new entries; operator-edits inside managed sections detected via manifest-SHA mismatch + **vsdd-init refuses at v0.1 with `VSDD-E0223: managed-file-drifted` — operator reverts the file or removes the manifest entry to re-deploy. Drift-resolution flags (`--keep-operator-edits` / `--accept-managed-defaults`) deferred to milestone v0.2; tracked at <issue>.**"
3. Test #7 (init.rs:280-282 string-assertion) replaced with: assert refusal IS the behavior + assert the named recovery procedure (file-revert OR manifest-entry-removal) actually works as recovery.
4. v0.2 acceptance criteria booked NOW: three-way classification (manifest hash vs disk hash vs source hash) + at least one of {interactive prompt, `--keep-operator-edits` flag, `--accept-managed-defaults` flag}. Choose between crosslink-interactive vs flag-driven AT v0.2 design, when the first external adopter shape is concrete enough to inform the choice.

### What this disposition is NOT

- NOT "drift handling is fine as-is." It isn't — F3's defect (error names non-existent flags) is real. The fix is to drop the flag names from both error AND spec at :854.
- NOT "we don't need crosslink-style collision handling." We do — but at v0.2, scoped to a concrete adopter shape, not pre-built against a phantom one.
- NOT amending the spec silently. This is an explicit `OperatorDirectiveApplied{directive: spec-contract-amended}` event per SO Standard Evaluation Dim 0; the spec change is part of the disposition and lands in the event log.

### Routing

- Phase 4 → Phase 1b (spec amendment at :854) + Phase 2b (implementation: rewrite error message + drop the named flags + add the manifest-revert recovery hint + replace Test #7).
- Composes with prior-round F2 (error-catalog allocation: `VSDD-E0223: managed-file-drifted` needs to be allocated in the init-time range chosen by F2's resolution).
- Composes with prior-round F5 (templates deployment scope decision): both are "did v0.1 deliver what an adopter needs day-one?" questions; both surface the same load-bearing observation that vsdd-cli-is-the-only-adopter changes the cost-benefit of "ship the full discipline now."
- Does NOT block prior-round F1 + F4 + F7 (audit-trail bundle) — orthogonal concerns.

### Sanity Check (validator-pair) baseline

Rubber-ducked: is "vsdd-cli is the only realistic v0.1 adopter" load-bearing or convenient? Load-bearing. If a second external adopter existed today, the cost-benefit shifts (more file-shapes; more user-edit patterns; refusal-only would block real workflows). The cut is testable: if the v0.2 milestone discovers a real external adopter who hit drift refusal as an unrecoverable state, the v0.1 cut was wrong AND it's recoverable (ship the v0.2 drift-resolution discipline; document the migration). The cut is NOT testable if v0.1 ships the speculative full discipline AND it turns out the v0.2 adopter shape needed different mechanics — that's worse, because it bakes in mechanics chosen against a phantom.

Cross-finding coherence: this disposition is consistent with prior-round F10 (manifest schema version) — both adopt "don't bake in v0.2 decisions at v0.1 without versioning + migration discipline named." The v0.2 drift-resolution mechanics will become a manifest-format change; tying that to the manifest_schema_version bump policy is the discipline F10 names.

### What pressure-tested the implementer-bias path

Operator named the easy disposition: "delete the flag references + call drift handling deferred." The pressure-test is: does that change the spec? Operator's framing did not include amending the spec at :854 — only changing the error message. That's silent divergence (spec says flags; impl says no flags; nothing in the event log records the disposition). That's exactly the SO sycophancy_failure_mode "Spec amended silently to match implementation — the spec moves to fit the code rather than the code being fixed against the spec." This disposition rejects that path. The spec is the contract; if the contract is wrong, the contract changes explicitly with an `OperatorDirectiveApplied` event, not by quietly mutating the error message.

## Cross-references

- DESIGN-METHODOLOGY.md:818-870 (collision-handling section; subject of spec amendment)
- DESIGN-METHODOLOGY.md:854 (specific flag-naming hallucination; amendment target)
- vsdd-core/src/init.rs:51-67 (`InitError::ManagedFileDrifted` variant; error message rewrite target)
- vsdd-core/tests/init.rs:280-282 (Test #7 string-assertion; replacement target)
- crosslink/src/commands/init/manifest.rs:32-48 (`UpdateAction` enum — the actual crosslink three-way classification, no flag names)
- crosslink/src/commands/init/mod.rs:605-611 (crosslink's interactive prompt path; NOT mirrored at v0.1)
- review-log/2026-06-02-solution-owner.md F3 (the finding that surfaced this disposition question)
- review-log/2026-06-02-solution-owner.md F2 (error-code range allocation; composes with `VSDD-E0223` allocation here)
- review-log/2026-06-02-solution-owner.md F5 (templates deployment scope; same v0.1-adopter-shape cost-benefit reasoning)
- review-log/2026-06-02-solution-owner.md F10 (manifest schema versioning; v0.2 migration discipline this disposition relies on)
