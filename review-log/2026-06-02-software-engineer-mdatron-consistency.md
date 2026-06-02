---
schema_class: review-entry
schema_version: 1.0.0
review_number: 2
date: 2026-06-02
phase: phase-3
scope: mdatron CLI surface + config layout + error-handling style — consistency vs crosslink substrate baseline (absorbability)
lens: software-engineer
source: operator-directive
session_note: Cold-session SE opinion on whether mdatron should converge on crosslink's anyhow + global-args + subcommand-module patterns. SE+sanity-check baseline. No code modified.
model: claude-opus-4-7
execution_method: claude-code-agent
sycophancy_compensation: SE-lens-bias-toward-aesthetic-consistency; LoC estimates demanded; weighed migration cost against named maintainability benefit per minimal-implementation Dimension 2.
supplements_loaded: []
---

# SE opinion: mdatron consistency with crosslink substrate baseline

## Headline

**Keep thiserror at the library boundary, do NOT migrate `mdatron-cli` to anyhow, DO add a `CommonArgs` struct (~20 LoC), DO fix the reserved-code violations now via spec amendment + a debug-assert lint, defer subcommand-module refactor until the third verb lands, and skip fuzz/proptest at v0.1.** Two consistency-with-crosslink concessions are net-positive; the rest are aesthetic and would burn LoC against no named benefit.

## thiserror vs anyhow — keep the split

mdatron-core's typed enums (`Error`, `VerifyError`, `EvalError`, `IndexError`, `ParseError`) are doing real work the downstream-consumer needs:

- `verify.rs:62-95` — `VerifyError` variants carry `pattern_id`, `rule_id`, `field` — vsdd (the downstream) wraps these in its own diagnostics and surfaces them at primer-load time. anyhow erases the structure that lets vsdd dispatch on `ExprParse` vs `Eval` vs `IndexBuild`.
- `expr.rs:178-203` — `EvalError::TypeMismatch { expected, got }` lets the message interpolator at `verify.rs:354-361` re-attribute the failure to the rule's `message:` field. anyhow's `Display`-only model breaks that re-attribution chain.

Migrating mdatron-cli (147 LoC, one file) to anyhow would touch `cmd_verify` + `print_pipeline_error` only, saving maybe 5 LoC of explicit `Err` matching, costing the typed `print_pipeline_error(&VerifyError)` dispatch surface. Net: **not worth it.** mdatron-cli is the binary boundary that *does* want typed errors — they're what fund the eventual `mdatron explain <code>` extended-doc dispatch (main.rs:144).

Conversely, retrofitting crosslink's `init/mod.rs` (2,506 LoC, 9 `Context` import, dozens of `.with_context()`) with thiserror is a multi-hundred-LoC project against zero adopter need. **crosslink owns its binary; mdatron-core is a library.** The split is correct.

Sycophancy check: an SE-aesthetic "they should match" finding here is the premature-abstraction failure mode. Reject.

## CLI argument convention parity — partial adoption

crosslink declares `--quiet/-q`, `--json`, `--log-level`, `--log-format` as `global = true` on the top-level `Cli` struct (main.rs:84-104), and they propagate to every subcommand automatically. mdatron has none.

**Cost to add:** ~20 LoC. One `CommonArgs` struct on `Cli`, four `global = true` attrs, then thread `--json` through `cmd_verify` to switch between `print_finding` and a SARIF/JSON emitter. The CLI already declares `Severity::Error/Warning/Lint` counts at main.rs:98-107; `--quiet` collapses the per-finding output to just the summary line; `--json` emits the existing `Finding` (already `Serialize`) as a JSON array.

**Recommendation: add `--quiet` and `--json` now, defer `--log-level` and `--log-format` until mdatron grows a tracing surface.** The CLI's exit-code semantics (0 = clean, 1 = errors, 2 = pipeline failure) are already script-friendly; `--json` is the actual ergonomic win for adopters parsing findings. ~15 LoC for `--quiet` + `--json` alone.

Sycophancy check: copying all four because crosslink has all four is aesthetic. mdatron doesn't emit tracing today. **Take the two that have a present user; defer the two that don't.**

## Error-code allocation discipline — spec amendment + lint, not rename

`DESIGN-MDATRON.md:506-510` reserves:
- `E0001-E0009` = frontmatter parsing
- `E0010-E0019` = path-confinement
- `E0020-E0029` = DSL eval
- `E0030-E0039` = delegate
- `E0040-E0049` = schema load

Current code emissions:
- `MDATRON-E0001` (verify.rs:264) = "frontmatter-schema-violation" — **violates spec.** Spec says E0001 = "frontmatter-parse-failed" (which is correctly used at verify.rs:231); E0001 is double-booked.
- `MDATRON-E0002` (verify.rs:231) = "frontmatter-parse-failed" — **violates spec.** Spec says E0002 = "schema-class-unknown".
- `MDATRON-E0070` (main.rs:74) = "cannot resolve project root" — **partially violates spec.** Spec line 814 reserves E0070 specifically for "no-git-repo" in init context. Re-using it for "cwd unresolvable" steals the slot.
- `MDATRON-E0080` (main.rs:141) = "verify pipeline failed" — **outside the allocation table entirely.** E0050-E0099 has no class assignment in the table on line 504-514.

The right fix is **not** rename-the-codes-to-match-the-spec, because the spec table itself has gaps (E0050-E0069, E0080+) the implementation needed to fill *and* a double-booking the spec author created (E0001 means both "parse-failed" prose-line and the frontmatter-class range). **Path forward:**

1. **Amend `DESIGN-MDATRON.md:506-514` to add `E0050-E0059 = config/cwd resolution` and `E0080-E0089 = pipeline-orchestration failures`.** That gives E0070 + E0080 a legitimate home and clears the spec-implementation drift surfaced in M1/M2/M6 Phase 3 reviews. Routes to **Solution Owner** via Raise-to-SO (SE has no spec-change authority per the domain prompt).
2. **Reallocate E0001/E0002 in code: rename current E0001 emission (frontmatter-schema-violation) to E0005, leaving E0001 = frontmatter-parse-failed exclusive.** This is a one-line edit at verify.rs:264 + the matching explain-ref string at :274 + any test asserting the literal "MDATRON-E0001" on a schema-violation case. ~6 LoC.
3. **Introduce a code-allocation `debug_assert!` lint in `Finding::new` (or a constructor that validates the prefix-range mapping).** Tests assert `Finding::new("MDATRON-E0005", ...)` succeeds and `Finding::new("MDATRON-E0070", ...)` rejects when severity = error and the class doesn't match path-confinement-allocated range. ~30 LoC, prevents future drift mechanically.

The lint matters because the spec-amendment path alone repeats the original failure mode in six months. **Spec + lint, not spec or lint.** Routes the test to **Quality Engineer**.

## Diagnostic shape — already convergent, don't share a crate

`diagnostic.rs` implements `Finding { code, severity, summary, message, help, location, explain_ref }` with rustc-style `format_tty`. crosslink's `InitUI::step_start/step_ok/step_skip` (init/mod.rs:119-155) is a *progress-output* surface, not a diagnostic surface. They have **fundamentally different shapes**: mdatron emits structured-finding-per-violation; crosslink emits one-step-per-operation.

A shared `diagnostic` crate would force one of them to compromise:
- crosslink would have to wrap every `step_ok` in a `Finding{severity: Lint}` — ceremony with no operator-facing benefit.
- mdatron would have to invent a "step lifecycle" concept it doesn't need (verify is single-pass, not multi-step).

**Recommendation: do not extract a shared diagnostic crate.** The shapes are convergent at the *surface convention* level (severity labels, rustc-arrow location format) without sharing types. The convention is the load-bearing part; the type is incidental. Routes to **Solution Architect** for the "shared crate?" architectural question; SE position is no.

## Subcommand scaffolding — defer the refactor

mdatron's main.rs handles two verbs inline (`Verify`, `Explain`), 147 LoC total. crosslink's `commands/{init,issue,timer,...}/mod.rs` pattern earns its keep when each verb has hundreds of LoC of supporting logic (manifest manipulation, walkthrough TUI, etc.).

**Cost to refactor now:** ~50 LoC of module-scaffolding churn; net zero behavioral change; one new directory tree to learn. **Trigger to refactor:** when the third verb lands. v0.1 will grow `init` (per operator directive). Two verbs (`verify`, `init` once built) still fit inline. The third verb (likely `schema generate` or `pattern test` per the v1-ship-criteria namespace reservations) is the right break point.

`init` belongs in `mdatron-cli/src/init.rs` initially — single file, sibling to `main.rs`'s dispatch arm. When verb #3 arrives, *that's* when `commands/init/mod.rs`, `commands/verify/mod.rs`, `commands/<verb-3>/mod.rs` becomes worth the directory ceremony.

Sycophancy check: refactoring now to "match crosslink's pattern" before the second verb is even implemented is the speculative-complexity sycophancy mode named in the SE domain prompt. Defer.

## Testing discipline — fuzz/proptest premature for v0.1

mdatron-core has 130 `#[test]` annotations across 8 files (no `tests/` integration dir, all inline `#[cfg(test)] mod tests`). crosslink ships fuzz harness (`fuzz/fuzz_targets/fuzz_*.rs`, 5 targets) + proptest (`tests/smoke/tui_proptest.rs`).

**Are these load-bearing for v0.1?**

- **proptest for the DSL evaluator:** would actually pay off. `EvalError::TypeMismatch` and `FieldNotFound` are the variants property-tests find unhandled paths in. Cost: ~150 LoC of strategies + asserting evaluator-never-panics across random `Expr` trees. **Defer to v0.2** — the 130-test surface plus the failures-as-findings model (panics get caught by the verify-pipeline) is sufficient v0.1 coverage. Re-evaluate when the DSL grows arithmetic or string-manipulation builtins.
- **fuzz for the expr_parser:** would pay off when adopters write rules. Cost: ~50 LoC of `cargo-fuzz` setup + one target. **Defer to v0.2** — the expr_parser at 130-test surface is finite enough to enumerate by hand at v0.1 grammar size.
- **Integration tests in `mdatron-cli/tests/cli_integration.rs`:** *missing* and *important*. The CLI has zero end-to-end tests asserting exit-code semantics. ~80 LoC, ~5 test cases (clean / warnings-only / errors / pipeline-failure / unknown-explain-code). **Add now**, routes to **Quality Engineer**.

Sycophancy check: copying crosslink's fuzz+proptest surface because "mature projects have them" is the premature-discipline form of speculative complexity. The CLI integration tests are the actual gap. Take the gap; defer the rest.

## Two-impl maintainability flag

vsdd will consume mdatron as a dependency (operator directive). vsdd's `init` lands separately; mdatron's `init` lands here. Both will need the same artifact-deployment-plan logic. **Flag to Solution Architect:** the `init-manifest` extraction concern from the prior SE review (2026-06-02-software-engineer-init-drift.md) applies here too — when mdatron's `init` ships, the manifest pattern is the natural shared abstraction surface, but **only after** both impls exist. Don't pre-extract.

## Coordination

- **Solution Owner** (Raise-to-SO): amend `DESIGN-MDATRON.md:506-514` reserved-codes table to add `E0050-E0059` (config/cwd) and `E0080-E0089` (pipeline orchestration). Disambiguate E0001 vs E0005.
- **Quality Engineer**: code-allocation `debug_assert!` lint test (~30 LoC); CLI integration test suite at `mdatron-cli/tests/cli_integration.rs` (~80 LoC, ~5 cases).
- **Solution Architect**: no shared diagnostic crate; no init-manifest pre-extraction; subcommand-module refactor at third-verb trigger.

## Recommendation, restated

Keep the thiserror/anyhow split; add `--quiet` + `--json` global args (~15 LoC); amend the reserved-codes spec + introduce a code-allocation lint + rename E0001/E0002 emission collision (~40 LoC + spec edit); defer subcommand-module refactor to third verb; add CLI integration tests now, defer fuzz/proptest to v0.2. Net mdatron-side LoC: ~85 LoC code + ~80 LoC tests + one spec amendment. Falsifiable, spec-aligned, no aesthetic-consistency tax.
