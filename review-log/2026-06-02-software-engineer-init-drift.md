---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-06-02
phase: phase-1c
scope: vsdd init drift / collision / upgrade — v0.1 path selection
lens: software-engineer
source: operator-directive
session_note: Cold-session SE opinion on Path A/B/C/D for vsdd init v0.1. SE+sanity-check baseline. No code modified.
model: claude-opus-4-7
execution_method: claude-code-agent
sycophancy_compensation: SE-lens-bias-toward-cleanest-code; cost compared against operator's documented need; minimal-implementation discipline applied per phase-1c.
supplements_loaded: []
---

# SE opinion: vsdd init drift handling for v0.1

## Headline

**Recommend Path B** (two flags wired against the existing two-way model, ~30-50 LoC) — *and* clean up the four code smells in `vsdd-core/src/init.rs` named below. Path C/D are correct for v1+; they are speculative complexity now.

## Why not the others (SE lens)

**Path A (rename in error text only).** The spec (`DESIGN-METHODOLOGY.md:854`) already names the two flags as the operator's escape hatch. Putting them in the error message without wiring them is a documented dead-end: operator reads the flag name, types it, gets "unknown argument". That violates SE Dimension 3 (error specificity = operator's recovery action must be real). Reject.

**Path C (mirror crosslink's three-way + `UpdateAction` + prompts, ~200-400 LoC).** Crosslink's three-way is load-bearing because crosslink ships per-file *merge strategies* (`.gitignore` markers, `.mcp.json` object merge, `allowedTools` union). vsdd v0.1 ships **only whole-file artifacts** — schemas, primers, domain prompts, supplements (see `build_deployment_plan` at init.rs:207). With no per-file merge to feed, three of crosslink's six `UpdateAction` variants (`TemplateUnchanged`, `Conflict`, `NewFile`) collapse to "overwrite or refuse". The `UpdateAction` enum without merge strategies is the premature-abstraction sycophancy failure mode named in the SE domain prompt. Defer.

**Path D (Path C + managed-section markers + parser, ~400-700 LoC).** vsdd v0.1 emits zero files needing managed sections — `.gitignore`, `.claude/mcp.json`, `.claude/settings.json`, `.github/CODEOWNERS`, `.pre-commit-config.yaml` are all in the spec's per-file collision matrix (`DESIGN-METHODOLOGY.md:822-828`) but **none are in the current `build_deployment_plan`**. A markdown-managed-section parser without any markdown files needing one is dead code. Reject for v0.1; revisit when the deployment plan grows the first file that needs it.

**Path B (wire the flags, ~30-50 LoC).** Spec-aligned (the two flag names appear verbatim in the error text). Honors the two-way model the code already has. Surfaces operator recovery actions truthfully. Cost matches what the operator's actual upgrade-friction surface is at v0.1 (small fleet, no field reports of three-way drift).

## InitError variants under Path B

Path B adds two new variants and clarifies one existing:

- `ManagedFileDrifted` — unchanged (already names both flags + both hashes).
- `ConflictingFlags { flag_a, flag_b }` — `--keep-operator-edits` and `--accept-managed-defaults` are mutually exclusive; flag conflict is a config error, not a drift error.
- `UpgradeNoOpWithoutFlags { drifted_count }` — when drift is detected and *neither* flag is set in non-interactive context, today's behavior (`return Err(ManagedFileDrifted{..})` for the *first* drifted file) silently masks the rest of the fleet. Variant should carry the count + a path to a written drift-report file so operator sees the full surface before choosing a flag.

Exhaustiveness check: with these three the universe is {not-initialized, not-git, drifted-and-resolvable, flag-conflict, io}. That covers v0.1.

## Test surface (Red Gate, Path B)

Mirror crosslink's `test_update_*` shape but at vsdd's altitude:

- `init_with_keep_operator_edits_preserves_drifted_file` — drift exists, flag set, file is left as-is, manifest is updated to record the operator's hash (so next run is clean).
- `init_with_accept_managed_defaults_overwrites_drifted_file` — drift exists, flag set, file is overwritten, manifest reflects source hash.
- `init_rejects_conflicting_resolution_flags` — both flags set → exit 2 + `ConflictingFlags` error before any I/O.
- `init_without_flags_on_drift_lists_all_drifted_files` — currently returns on first drift; the test asserts the full set is reported.
- `init_idempotent_after_keep_operator_edits` — second run with no flags is a no-op (proves manifest update closed the loop).

Crosslink runs ~14 tests for the comparable surface (`mod.rs:2192-2486`). vsdd at Path B needs ~5 — that's the right ratio for v0.1 scope.

## Idiomatic Rust smells in current `vsdd-core/src/init.rs`

1. **init.rs:264-267** — `load_manifest` swallows JSON parse errors and treats corrupt manifest as "no prior init". This is the silent-failure sycophancy mode. An operator whose manifest got `sed`-mangled will get their drift detection silently turned off. Should be `InitError::ManifestCorrupt { path, parse_error }` with a recovery hint (`delete + re-run init`).
2. **init.rs:80** — `let _ = options;` reads as "options are deliberately ignored". `ci_mode` is in the struct but never threaded. Either route it or drop the field until step-7 needs it (YAGNI per minimal-implementation Dimension 2).
3. **init.rs:164, 189** — Two `.expect("...serializes...")` panics. These are reachable in principle (e.g., non-UTF-8 sneaks into a path string future-someone adds to `Manifest.files`). Use `serde_json::to_string_pretty(..).map_err(...)?` returning a new `InitError::SerializationDefect` variant; the panic-message comment already names this as a build-time invariant, which is exactly what a typed error preserves better than `expect`.
4. **init.rs:114-125** — drift check returns on first hit. Operator gets one drifted file at a time, has to re-run after each fix. Collect-then-report (Vec<(path, expected, actual)>) is the same LoC and removes a Felicific-cycle papercut.

None of these are blockers for Path B; all four are 1-line-to-5-line fixes that should ride with the Path B PR.

## Two-impl maintainability flag

Crosslink and vsdd will both grow this surface. Crosslink's `manifest.rs::classify_update` (`crosslink/src/commands/init/manifest.rs:136-152`) is *literally* the function vsdd will need when it adopts Path C. **Flag to Solution Architect** (validator pair): extracting a `init-manifest` crate shared by both is out-of-scope for v0.1 but should land before Path C. Don't fork the three-way model in two places.

## Coordination

- **Quality Engineer**: test surface above belongs in `vsdd-core/tests/init.rs`; the missing-drift-multiplicity case is a Phase 2a Red Gate gap right now.
- **Solution Architect**: shared `init-manifest` crate decision before v1.

## Recommendation, restated

Path B + the four init.rs hygiene fixes. ~50-80 LoC total. Spec-aligned, minimal, exhaustively-typed errors, falsifiable test gate. Path C waits until the first file in `build_deployment_plan` needs managed-section semantics; Path D waits until the markdown surface needs an in-place merge.
