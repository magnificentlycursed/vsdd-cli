---
schema_class: review-entry
schema_version: 1.0.0
review_number: 3
date: 2026-06-02
phase: phase-1c
subject_project: mdatron
scope: >-
  Solution Owner opinion on mdatron v0.1 CLI surface (mdatron-cli/src/main.rs:24-50)
  + config layout (.mdatron/{schemas,patterns}/ per mdatron-core/src/verify.rs:46-57)
  + error-handling style (MDATRON-Exxxx codes + rustc-shaped Finding rendering per
  mdatron-core/src/diagnostic.rs:68-99 + mdatron-core/src/error.rs:12-25) against
  crosslink as substrate baseline (crosslink/crosslink/src/main.rs:80-527 — 30+
  subcommand surface). Operator framing: pursue "absorbability" — mdatron as a
  standalone tool that vsdd consumes — without inheriting crosslink-shape that
  doesn't fit a single-purpose validator.
lens: >-
  Solution Owner primary (scope-discipline + project-boundary authority + adopter-shape
  reasoning). Sanity Check baseline (cross-finding coherence with prior round's
  CLI-divergence + error-code-range findings). Phase 1c framing —
  acceptance-criteria scoping for mdatron v0.1.
source: operator-directive
session_note: >-
  Cold-session single-domain composition. Operator directive settled prior to this
  session: mdatron is a standalone project; vsdd consumes mdatron as a dependency;
  mdatron has its own init (pending Phase 1a). The SO opinion sought is on what
  consistency-with-crosslink-as-substrate means for a tool whose surface is
  dramatically smaller and whose adopter shape diverges from crosslink's.
model: claude-opus-4-7
execution_method: >-
  Cold-session single-agent dispatched from main session; SO + Sanity Check domain
  prompts loaded fresh; mechanical citation against mdatron-cli/src/main.rs,
  mdatron-core/src/{error.rs, diagnostic.rs, verify.rs, schema.rs, dsl/expr.rs} +
  crosslink/crosslink/src/main.rs as the inheritance baseline + DESIGN-MDATRON.md
  § CLI surface / § Adopter onboarding / § Co-evolution with vsdd-cli as the
  spec-side surface; no prior-cycle context beyond explicitly-named read-first set.
sycophancy_compensation: >-
  Operator pre-named the bias to guard against: "SO-lens guards against scope-
  bloat; the cheap path is 'do nothing major; keep mdatron minimal' — pressure-test
  whether that protects adopters or just delays inevitable work." The disposition
  below adopts the minimal-surface path for v0.1 but tightens the inheritance
  story: the v0.1 surface is intentionally NOT crosslink-shaped, because the
  adopter shape is not crosslink-shaped; the consistency claim is narrowed to
  diagnostic-shape (rustc convention) + error-code grammar, NOT subcommand
  topology. That cut is testable against the v0.2 adopter shape: if a second
  external adopter materializes who wants crosslink-style subcommand discoverability
  (e.g., `mdatron config show`, `mdatron explain --list`), the cut was wrong AND
  recoverable.
filename_note: >-
  Filed `<date>-solution-owner-mdatron-consistency.md` (suffixed to disambiguate
  from same-date `2026-06-02-solution-owner.md` (Phase 3 init review) +
  `2026-06-02-solution-owner-init-drift.md` (Phase 1c drift disposition). Per the
  existing same-date precedent, suffix-distinguishes scope when one domain
  produces multiple entries in a day.
supplements_loaded: []
---

# Solution Owner Opinion — mdatron CLI/config/error-handling consistency with crosslink baseline

**Phase:** 1c (acceptance-criteria scoping for mdatron v0.1). Not Phase 3 adversarial.
**Subject project:** mdatron (standalone). **Baseline cited:** crosslink (substrate inheritance source).

## Headline

**Narrow the consistency claim. Diagnostic-shape (rustc convention) + error-code grammar inherit from crosslink/rustc; subcommand topology does NOT. v0.1 stays at 2 commands. v0.1.0 publishes to crates.io to seize the name + force semver-1.0-discipline-deferral honestly. M2/M4 tech-debt is v0.1.x-acceptable, not v0.1-blocking. mdatron's own init is starter-scaffold-only at v0.1.**

## Adopter scope realism (load-bearing argument)

Operator named three candidate v0.1 adopter shapes:
- (a) VSDD methodology user reached via `vsdd init`
- (b) non-VSDD developer wanting markdown frontmatter validation in a docs site
- (c) CI integrator wanting a single-purpose validator

**SO position: (a) is the only realistic v0.1 adopter; (c) is the realistic v0.1.x adopter; (b) is v0.2+.**

DESIGN-MDATRON.md § Co-evolution with vsdd-cli (lines 887-921) explicitly names "Bootstrap-period coordination" with vsdd-cli switching from `mdatron-core` local-path-dep to published-crate-dep at "Step 5." The bootstrap period is the v0.1 window. The realistic v0.1 adopter is vsdd-cli itself (transitively, methodology users reached via `vsdd init`). (b) requires the two worked examples DESIGN-MDATRON.md § Adopter onboarding (lines 820-831) names — those are V1-ship-criteria, NOT v0.1.

This matters because **crosslink's adopter shape (multi-agent issue-tracker; 30+ subcommands; interactive TUI; daemon; signing; locks) is fundamentally different from mdatron's (single-purpose validator invoked from hooks + CI)**. Consistency-with-crosslink means consistency-with-substrate-conventions (rustc diagnostic shape, error-code grammar `<PREFIX>-{E,W,N}NNNN`, exit-code semantics), NOT consistency-with-subcommand-topology.

## Scope of "consistency" — where it ends

**Inherits from crosslink baseline (must keep):**
- Rustc-shaped diagnostic format (`error[CODE]: summary\n  --> file:line` per `mdatron-core/src/diagnostic.rs:76-98`) — already correct
- Error-code prefix grammar (`MDATRON-E0001`, `VSDD-E0200`) — already correct per Finding.code field
- Exit-code semantics (0 = clean, 1 = findings present, 2 = pipeline error) — already correct per `mdatron-cli/src/main.rs:74,95,118`
- `cargo install` distribution convention — DESIGN-MDATRON.md:795 declares it
- `clap` derive + `#[command(version, about)]` shape — already correct

**Anti-themic to mirror (do NOT inherit):**
- Subcommand groups (Issue/Timer/Knowledge/Swarm/...). mdatron has 2 commands; parity would be cargo-cult.
- Global flags `--quiet`/`--json`/`--log-level`/`--log-format` (crosslink/crosslink/src/main.rs:85-104). v0.1 mdatron has `--format <tty|sarif|json|compact>` per spec at DESIGN-MDATRON.md:556; that's the right shape for a validator. `--quiet` + `--json` as separate globals would conflict.
- Hidden top-level shortcut aliases (crosslink/crosslink/src/main.rs:364-526). mdatron's 2 commands don't need shortcuts.
- TUI / daemon / serve commands. Out of scope.
- `find_crosslink_dir()` walk-up-to-root discovery (crosslink/crosslink/src/main.rs:1872-1896). mdatron uses explicit `--project-root` flag with cwd fallback (`mdatron-cli/src/main.rs:71`); that's simpler and correct for v0.1.

**Narrow the consistency claim to: diagnostic-shape + error-code grammar + exit-code semantics + distribution-via-cargo-install.** The substrate baseline is what produces those four — NOT crosslink-the-application's surface shape. The realistic substrate is rustc + cargo, not crosslink.

## Versioning + semver risk — publish at v0.1.0, NOT v1.0

**Recommendation: publish mdatron + mdatron-core to crates.io NOW at v0.1.0 (NOT v1.0.0).**

Cost of NOT publishing soon: path-dep convention in vsdd-cli is fragile (operator-named). vsdd-cli cannot honestly emit a `MdatronVersionPinned` event (DESIGN-MDATRON.md:912-914) against a path-dep. The bootstrap-period coordination story (DESIGN-MDATRON.md:919-921) explicitly anticipates "Step 5" as the crates.io switch — that step is gated by mdatron being publishable.

Cost of publishing too early at v1.0: forces semver discipline before the DSL is stable. `dsl/expr.rs:7-11` explicitly lists deferred operators (arithmetic, ordered comparisons, indexing). Pattern files include `mdatron_dsl_version: 1` (verify.rs test fixtures line 542). Locking v1.0 makes every DSL extension a breaking-change discussion.

**v0.1.0 is the right cut.** v0.x explicitly signals "API may break"; crates.io name is reserved; vsdd-cli can pin `mdatron-core = "0.1"` and get a real version-pin event; `cargo install mdatron` works for the v0.1.x adopter (CI integrator). Promotion to v1.0.0 lands when DESIGN-MDATRON.md § V1-SHIP-CRITERIA is satisfied (two worked examples + DSL feature-complete) — NOT a v0.1 deliverable.

Note the package-name asymmetry: `mdatron-cli/Cargo.toml` declares `name = "mdatron"` (the CLI binary) while `mdatron-core/Cargo.toml` is `name = "mdatron-core"`. Publish both. vsdd-cli depends on `mdatron-core` (library API); `mdatron` (binary) is for adopters' `cargo install`.

## M2 + M4 carryover — v0.1.x acceptable, NOT v0.1-blocking

Prior reviews surfaced:
- M1 F1 + M2 F1 + M6 F2: error-code reserved-range violations (`MDATRON-E0070`, `MDATRON-E0080` in main.rs:74,141 — adopter codes leaking into engine ranges, or vice versa per the catalog discipline at DESIGN-MDATRON.md § Error catalog).
- M4 F2: CI install-command divergence.

**SO disposition: defer to v0.1.1 (NOT v0.1.0). Cut a clean v0.1.0 with the consistency narrowing above + named v0.1.1 work-item for these two consistency-surface defects.**

Rationale: v0.1.0 to crates.io needs to ship in a clean window — publishing it WITH the tech-debt fixes bundled risks "we'll do all of it at once and never ship." The Phase 1c discipline (this entry) names the cut: v0.1.0 = current CLI + current config + current error-handling style + crates.io publish; v0.1.1 = M1/M2/M6 + M4 fixes. v0.1.x semver allows breaking changes to error codes between 0.1.0 → 0.1.1 (per the v0.x convention).

**Caveat: the error-code range violations ARE real spec violations.** They're v0.1.x-acceptable only because v0.x explicitly signals instability AND the fix lands within v0.1.x. If they were left for v0.2+, that would be silent spec-drift — a sycophancy_failure_mode hit.

## The pending mdatron init — starter-scaffold-only at v0.1

DESIGN-MDATRON.md:790-817 spec'd the full `mdatron init`. crosslink's `init` is huge (interactive walkthrough; preset selection; Python prefix detection; cpitd install; signing key setup — crosslink/crosslink/src/main.rs:117-148, 10+ flags). Mirroring it is anti-thematic.

**Recommendation for mdatron v0.1 init: starter-scaffold-only.** Deploy `.mdatron/{schemas,patterns,catalogs,registries}/` (empty dirs + `mdatron.yaml` catalog file). Append `.gitignore` managed section. Emit `MDATRON-E0220: existing-file-malformed-refuse-to-overwrite` on conflict (already spec'd at DESIGN-MDATRON.md:815). NO interactive walkthrough; NO presets; NO git-repo pre-flight beyond a warning.

**Out of scope for v0.1 init:**
- `--check` dry-run JSON plan (DESIGN-MDATRON.md:569) → v0.1.1
- Manifest-tracked three-way merge (the crosslink `init --update` shape) → v0.2 (with a real second adopter to inform the choice, per the prior-cycle `solution-owner-init-drift` finding's logic at lines 64-83)
- `--reconfigure` / `--force` flags (crosslink-shaped) → not needed for fixed-canonical-artifact corpus

This composes with prior `solution-owner-init-drift` disposition: vsdd-cli's init refuses-on-drift at v0.1 because vsdd-cli IS the only realistic v0.1 adopter. Same load-bearing logic applies here for mdatron's own init.

## Acceptance criteria for "mdatron v0.1 done"

1. **CLI surface: 2 commands only** (`verify`, `explain`). `explain` may still stub to "not yet implemented at v0.1.0" (per mdatron-cli/src/main.rs:144-146) provided MDATRON-E codes ship with the catalog file deployed by `mdatron init`.
2. **Config: `.mdatron/{schemas,patterns}/` discovered relative to `--project-root` (or cwd).** No global config file required for v0.1. (`.mdatron/config.yaml` per DESIGN-MDATRON.md:805 is deferred to v0.2 when phases land.)
3. **Error-handling: rustc-shaped Finding rendering (current `diagnostic.rs:format_tty` is correct).** Internal `Error` enum (`error.rs:12-25`) stays internal; never surfaces to operators raw.
4. **Distribution: published to crates.io at v0.1.0.** Both `mdatron-core` and `mdatron` crates. vsdd-cli switches off path-dep onto `mdatron-core = "0.1"`.
5. **Init: starter-scaffold-only.** Refuse-on-conflict. No interactive paths.
6. **NOT shipped at v0.1.0:** error-code-range fixes (→ v0.1.1), `--format sarif`/`json`/`compact` (→ v0.1.2), `mdatron registry` subcommand group (→ v0.2), the three-way-merge init upgrade (→ v0.2), agent-loop PostToolUse compact integration (→ v0.2 with vsdd-cli co-evolution).

## What this disposition is NOT

- NOT "mdatron should mirror crosslink to be consistent." Anti-thematic; rejected.
- NOT "skip the M1/M2/M6/M4 fixes." They land at v0.1.1, named NOW so they don't drift.
- NOT "defer publishing." Publishing at v0.1.0 IS the v0.1 deliverable; the path-dep convention is fragile and the bootstrap-period coordination story breaks without a real version-pin.
- NOT "mdatron init is the same as vsdd init." vsdd init refuses-on-drift; mdatron init refuses-on-conflict. Both deploy starter scaffolds; neither does interactive flows at v0.1.

## Routing

- Phase 4 → Phase 1b (DESIGN-MDATRON.md amendment: § CLI surface narrowed to v0.1 scope; § Adopter onboarding amended to clarify starter-scaffold-only at v0.1; § Co-evolution with vsdd-cli amended to set Step 5 = crates.io publish at v0.1.0 not v1.0.0).
- Phase 4 → Phase 2b (mdatron init implementation — starter-scaffold-only per criteria above).
- Phase 4 → Phase 2c (crates.io publish workflow + v0.1.0 release).
- Composes with prior-round `solution-owner-init-drift` disposition (vsdd-cli init): same load-bearing logic — only-adopter-is-vsdd-cli changes the cost-benefit of pre-building speculative discipline.
- Composes with M1/M2/M6 F2 (error-code ranges) + M4 F2 (CI install commands): defers to v0.1.1 with explicit acceptance criterion.

## Sanity Check (validator-pair) baseline

Rubber-ducked: "is the cheap path (keep mdatron minimal at v0.1) protecting adopters or delaying inevitable work?" Protecting. The work crosslink-mirror would build (subcommand topology; global flags; init walkthrough) is NOT what an absorbable single-purpose validator needs; it's what a multi-agent issue-tracker needs. The v0.1.x defects (error-code ranges, CI commands) are explicitly named with v0.1.1 disposition — not silently deferred. Publishing at v0.1.0 (not waiting for v1.0 polish) IS the inevitable work, brought forward.

Cross-finding coherence: this disposition is consistent with prior `solution-owner-init-drift` (vsdd-cli has the only realistic v0.1 adopter → cut speculative discipline). Same adopter-shape reasoning. Both findings refuse the "build the full crosslink shape now" path; both name v0.2 as where the second-adopter-shape informs the choice.

## Cross-references

- mdatron-cli/src/main.rs:24-50 (current 2-command surface — keep)
- mdatron-cli/src/main.rs:74,141 (MDATRON-E0070, E0080 — engine-range codes leaking; v0.1.1 fix target)
- mdatron-core/src/diagnostic.rs:76-99 (rustc-shape format_tty — correct, inherits from baseline)
- mdatron-core/src/error.rs:12-25 (internal Error enum — never surfaces to operators raw)
- mdatron-core/src/verify.rs:46-57 (VerifyConfig — `.mdatron/{schemas,patterns}/` discovery; v0.1 keeps this shape)
- mdatron/DESIGN-MDATRON.md:549-595 (§ CLI surface — amendment target: narrow to v0.1 scope)
- mdatron/DESIGN-MDATRON.md:788-817 (§ Adopter onboarding — amendment target: starter-scaffold-only at v0.1)
- mdatron/DESIGN-MDATRON.md:887-921 (§ Co-evolution with vsdd-cli — amendment target: Step 5 = v0.1.0 publish)
- mdatron/Cargo.toml:1-12 (workspace.package version 0.1.0 — already correctly versioned; needs publish action)
- crosslink/crosslink/src/main.rs:80-527 (baseline — explicitly NOT mirrored for subcommand topology)
- crosslink/CLAUDE.md (30+ subcommand surface — explicitly NOT mirrored)
- review-log/2026-06-02-solution-owner-init-drift.md (sibling Phase 1c entry; load-bearing-logic precedent)
