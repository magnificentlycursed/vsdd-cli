---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-06-02
phase: phase-3
scope: >-
  mdatron milestone — CLI wiring + 2 pipeline robustness fixes. Commits 7bff7f3
  (mdatron-cli wires verify subcommand), fe8c1c7 (MDATRON-E0002 finding on
  frontmatter parse failure), de7be3d (canonicalize project root before joining
  globs). Primary files — mdatron-core/src/verify.rs + mdatron-cli/src/main.rs.
  Surface — `mdatron verify --project-root . [--schemas DIR] [--patterns DIR]
  [--files GLOB...]` with rustc-shaped diagnostics + exit codes (0 clean / 1
  errors / 2 pipeline failure).
lens: >-
  Primary Software Engineer (error handling, argument parsing, exit-code
  discipline, idiomatic clap). Supporting Platform Engineer (install story,
  substrate, portability), Documentation Reviewer (operator-facing message
  actionability, schema-class dispatch discoverability), Sanity Check (scope
  drift, novel coinages). 5-lens application weighted Consistency (5) +
  Maintainability (4) + Edge cases (4) + Usability (3).
source: director-raised
session_note: >-
  Cold-session cluster-batched review per Phase 3 primer. Subject is the
  mdatron worktree at /Users/claire.celesterra/Documents/Source/magnificentlycursed/mdatron;
  review-log filed under vsdd-cli per the operator-directive scope split (mdatron
  consumes from vsdd-cli/DESIGN-MDATRON.md but review evidence is methodology-
  governed). Composition — single agent multi-domain (SE primary; PE/DR/SC
  supporting), inline-no-worktree-isolation, per the operator directive that this
  is a sub-milestone review rather than a layer-close gate. The deviation from
  the canonical 4-cluster shape is acknowledged; F1 + F4 + F5 are mechanically
  evidenced and not judgment-dependent.
model: claude-opus-4-7
execution_method: >-
  inline main session (single-agent multi-domain composition; vsdd-phase-3 +
  SE + PE + DR + SC primers loaded sequentially)
sycophancy_compensation: >-
  Claude authored the code under review. Compensation: every finding cites
  file:line and includes a falsifiability clause ("what would have to change for
  this no longer to apply"). No dismissal_rationale carried by author identity;
  Phase 4 routing required for every Resolved-pending finding. The bias I am
  resisting is reading "the code does what the commit message says" as closure
  evidence — the milestone commits frame the change as robustness fixes but the
  spec/code drift this review surfaces (F1) is invisible from the commit-diff
  surface alone.
filename_note: >-
  Filed under software-engineer per Phase 3 cluster-batched naming; SE is
  primary domain. PE + DR + SC findings labeled inline.
---

# Software Engineer Review 1 — 2026-06-02 (mdatron milestone)

## Pre-phase composition declaration

```yaml
phase: phase-3
composed_domains: [software-engineer, platform-engineer, documentation-reviewer, sanity-check]
composition_mode: inline-single-agent-multi-domain
memory_isolation: NONE (single main-session; no worktree isolation)
operator_confirmation: confirmed (director-raised, sub-milestone scope)
cluster_shape: deviation-from-4-cluster-default (sub-milestone-justified)
declared_at: 2026-06-02T00:00Z
```

---

## Findings

### F1 — MDATRON error-code allocations contradict DESIGN-MDATRON.md spec (SE + DR; spec-implementation alignment) — Open

**Evidence:**
- `mdatron/DESIGN-MDATRON.md:116-117`: spec assigns `MDATRON-E0001: frontmatter-parse-failed` and `MDATRON-E0002: schema-class-unknown`.
- `mdatron/mdatron-core/src/verify.rs:231` emits `MDATRON-E0002` with summary `"frontmatter-parse-failed"` — this is the spec's E0001 semantic under the E0002 code.
- `mdatron/mdatron-core/src/verify.rs:264` emits `MDATRON-E0001` with summary `"frontmatter-schema-violation"` — neither E0001 nor any other allocated range in `DESIGN-MDATRON.md:506-514` names this semantic.
- `mdatron/mdatron-cli/src/main.rs:74` emits `MDATRON-E0070` for "cannot resolve project root", but `DESIGN-MDATRON.md:814` reserves `MDATRON-E0070: no-git-repo`.
- `mdatron/mdatron-cli/src/main.rs:141` emits `MDATRON-E0080: verify pipeline failed`; the E0080 range is unallocated in the spec's reserved-codes table (`DESIGN-MDATRON.md:506-514`).

**Lens:** Consistency (5) + Maintainability (4). The error catalog is the operator's contract; spec/code disagreement at the catalog layer means `mdatron explain MDATRON-E0001` will document one thing and the binary will emit a different semantic under that code. The PR cover message ("emit MDATRON-E0002 finding on frontmatter parse failure (was previously a fatal pipeline error)") names the change as semantic improvement but does not flag that the chosen code contradicts the spec.

**Falsifiability:** This no longer applies if DESIGN-MDATRON.md § Reserved mdatron codes is amended to match the implementation OR the implementation is rewritten to use the spec's allocation (E0001 = parse-failed; E0002 = schema-class-unknown; new code for schema-violation; reallocate E0070/E0080 outside reserved ranges or extend the table).

**Routing:** Phase 4 → Raise to SO (spec amendment) OR Phase 1a (code amendment). Mechanical sweep either direction; SO disposition required on which is canonical.

**Classification:** Resolved-pending.

---

### F2 — `canonicalize()` failure mode on `--project-root` produces opaque error + symlink semantics undocumented (SE + PE; error specificity + portability) — Open

**Evidence:**
- `mdatron/mdatron-core/src/verify.rs:104-107`: `config.project_root.canonicalize()` returns `VerifyError::Io` with the bare `std::io::Error` string.
- Two failure modes collapse onto one message: (a) path does not exist (most common; `ENOENT`); (b) path exists but symlink chain cannot be resolved (`ELOOP`, permission). Both render as `"io error at '<path>': <oserror>"` via `print_pipeline_error` (`main.rs:141`).
- Windows portability: `canonicalize()` on Windows returns UNC-form paths (`\\?\C:\...`). Subsequent `project_root.join(glob_pattern)` (`verify.rs:118`) produces a UNC path; `glob::glob()` (`verify.rs:119`) behavior against UNC patterns is unspecified in the glob 0.3 docs. Macros/CI may report path-mismatch findings the operator cannot reconcile.
- Symlink semantics: the commit message for `de7be3d` says "canonicalize before joining globs (relative --project-root . silently matched zero files)", but `canonicalize()` ALSO resolves symlinks. An operator who has `.mdatron/` as a symlink to a sibling directory (a reasonable monorepo pattern) will see globs matched against the symlink target, which may not contain their `.md` files.

**Lens:** Edge cases (4) + Maintainability (3) + Attacker's mindset (2: a symlinked project-root + a TOCTOU between canonicalize and glob walk is a low-severity vector, but worth naming).

**Falsifiability:** This no longer applies if (a) the `Io` variant is split into `ProjectRootMissing` vs `ProjectRootResolutionFailed` with specific messages, AND (b) DESIGN-MDATRON.md documents the symlink-resolution semantic + the Windows UNC-path behavior, OR `dunce::canonicalize` (or equivalent) is used to keep paths in non-UNC form.

**Routing:** Phase 4 → Phase 1a (error-variant split) + Phase 1b (DESIGN-MDATRON.md portability section).

**Classification:** Resolved-pending.

---

### F3 — `--files` flag accepts glob strings but spec § Behaviors says `--files <file>` (DR; spec-vs-CLI surface drift) — Open

**Evidence:**
- `mdatron/mdatron-cli/src/main.rs:40-42`: `--files` is declared as `value_name = "GLOB"`, accepts `Vec<String>`, fed directly to `config.file_globs` (`main.rs:87`) which feeds `glob::glob()`.
- `mdatron/DESIGN-MDATRON.md:555`: "`mdatron verify --files <file>...          # selective execution`" — value_name `<file>`, behavior described at `:583` as "validates only the listed files; useful for hook integration where the caller knows what changed".
- A hook that passes a literal path like `docs/foo.md` works (glob-with-no-metacharacters matches the literal), but the CLI surface does not communicate to the operator that the value is interpreted as a glob. An operator who passes a Windows-style path with backslashes or a path containing `[` (e.g., `docs/[draft]-notes.md`) will see glob-meta interpretation applied with no warning.

**Lens:** Usability (4) + Consistency (3). Cold reader of `--help` sees `--files <GLOB>...` but spec promises `<file>`.

**Falsifiability:** This no longer applies if (a) DESIGN-MDATRON.md is updated to document the glob semantic for `--files`, OR (b) the flag is split into `--files <FILE>...` (literal) and `--glob <GLOB>...` (pattern), with help text disambiguating.

**Routing:** Phase 4 → Phase 1a (spec/help-text reconciliation). Mechanical.

**Classification:** Resolved-pending.

---

### F4 — Warning-only runs return ExitCode::SUCCESS but stderr says "N error(s), M warning(s)" — surface inconsistency (SE; exit-code discipline) — Open

**Evidence:**
- `mdatron/mdatron-cli/src/main.rs:109-122`: the `else` branch is taken whenever `errors > 0 || warnings > 0`. It always prints `"mdatron verify: {errors} error(s), {warnings} warning(s) across {N} finding(s)"` to stderr, then returns `ExitCode::SUCCESS` when `errors == 0 && warnings > 0`.
- The operator sees a stderr summary that looks like a failure ("0 error(s), 3 warning(s) across 3 finding(s)") followed by a `$? == 0`. A CI pipeline that scrapes stderr for "warning(s)" will interpret that as a problem; a CI pipeline that scrapes exit codes will not.
- The spec (`DESIGN-MDATRON.md`) does not name an exit-code table for the verify subcommand. The closest is `:589-594` which says "exit code 2" for config-load failures but does not enumerate warning-only behavior.

**Lens:** Consistency (5) + Usability (3). Exit-code discipline is the pre-commit-hook contract; ambiguity here means downstream tools have to choose between two signals and choose differently.

**Falsifiability:** This no longer applies if (a) DESIGN-MDATRON.md adds an explicit exit-code table for `mdatron verify` that names the warning-only case as exit 0 by deliberate design, AND (b) the stderr summary line is suppressed (or rewritten as "0 errors, N warnings — clean") on the SUCCESS path, OR (c) a `--warnings-as-errors` flag is added so the operator can opt into the stricter semantic.

**Routing:** Phase 4 → Phase 1a (spec exit-code table) + Phase 2b (CLI behavior reconciliation).

**Classification:** Resolved-pending.

---

### F5 — `cmd_explain` is a v0.1.0 stub that exits 2; clap surface advertises it as a verb (PE + DR; install story coherence) — Open

**Evidence:**
- `mdatron/mdatron-cli/src/main.rs:46-49` declares `Explain { code: String }` as a top-level subcommand; help text reads "Show extended documentation for an error code (rustc --explain pattern)".
- `mdatron/mdatron-cli/src/main.rs:144-147`: body is `eprintln!("mdatron explain {code}: extended docs not yet implemented at v0.1.0"); ExitCode::from(2)`.
- The user has no `cargo install mdatron` path — they must `cargo install --path mdatron-cli`. The first error-code operator sees in the wild (say, a CI failure with `MDATRON-E0001`) will invite them to run `mdatron explain MDATRON-E0001`; they will get exit-2 + a "not yet implemented" stub.
- The PE install story is also unaddressed: there is no top-level README at `mdatron/` (the repo has `BOOTSTRAP-MITIGATION.md`, `BOUNDARY-PREAMBLE.md`, `DESIGN-MDATRON.md`, `V1-SHIP-CRITERIA.md`, `STEP-2-SCOPE.md`, `CHANGELOG.md`, `dsl-falsifiability-report.md` — none named `README.md`). A new operator landing on the repo cannot derive the install command without grepping.

**Lens:** Usability (4) + Consistency (3). The clap surface promises a contract the implementation does not honor; the install story is undocumented at the directory the install user will land on.

**Falsifiability:** This no longer applies if (a) `Explain` is gated behind a `#[cfg(feature = "explain")]` or removed from the v0.1.0 surface until implemented, AND (b) a top-level README.md is added with the `cargo install --path mdatron-cli` instruction + the v0.1.0 scope disclaimer.

**Routing:** Phase 4 → Phase 1a (README.md + scope-gating decision). Composes with V1-SHIP-CRITERIA.md scoping.

**Classification:** Resolved-pending.

---

### F6 — `MDATRON-E0080` coinage introduced for "pipeline failure" without spec or catalog entry (SC; scope drift + novel-coinage) — Open

**Evidence:**
- `mdatron/mdatron-cli/src/main.rs:141`: `error[MDATRON-E0080]: verify pipeline failed`.
- The code `MDATRON-E0080` does not appear in `DESIGN-MDATRON.md` (full-file grep: only E0001, E0002, E0003, E0010–E0015, E0020, E0021, E0030, E0040, E0050, E0060, E0061, E0070 are referenced).
- The reserved-codes table (`DESIGN-MDATRON.md:506-514`) ends at E0040–E0049 (schema load failures); the next allocated range is `MDATRON-W0030`. E0070 + E0080 are both outside the reserved-range table and not enumerated as exceptions.
- The summary in print_pipeline_error has no `mdatron explain MDATRON-E0080` page (and the spec/catalog does not declare one); the operator who sees this error has no `explain` path.

**Lens:** Maintainability (4) + Consistency (3). Per the SC dim "scope drift": this milestone introduces a new error-code namespace allocation (E0070, E0080) as a side-effect of the CLI-wiring commit. The novel-coinage was earned by necessity (the CLI needed *some* code), but the discipline that codes live in the spec's reserved-table was bypassed.

**Falsifiability:** This no longer applies if DESIGN-MDATRON.md § Reserved mdatron codes is amended to allocate the E0070–E0089 range (CLI surface errors) with E0070 + E0080 entries, AND the corresponding `docs/error-codes/MDATRON-E0080.md` explain page exists.

**Routing:** Phase 4 → Raise to SO (spec amendment). Composes with F1.

**Classification:** Deferred-pending-SO.

---

### F7 — Schema-class dispatch model is operator-load-bearing but undocumented at the operator-landing surface (DR; cold-context discoverability) — Open

**Evidence:**
- `mdatron/mdatron-core/src/verify.rs:253-278`: Layer 1 dispatch is via the `schema_class:` frontmatter field. A file with no `schema_class` silently skips Layer 1 (`verify.rs:260-278` is gated on `Some(schema_class)`). The behavior is tested at `verify.rs:651-673` ("file_without_schema_class_skips_layer_one_runs_layer_two").
- `mdatron/DESIGN-MDATRON.md:112-113` documents the routing but assumes the reader has reached the DESIGN doc.
- The CLI `--help` text (`main.rs:26-27`) says `"Validate markdown documents against configured schemas and patterns"`. No mention of `schema_class:` as the dispatch key, no mention that files lacking it are partially validated.
- The cli's `after_help` (`main.rs:17-18`) carries the disambiguation copy ("Descended from Schematron...") but no operator-facing note about the schema-class dispatch model.
- An operator who runs `mdatron verify` on a project with markdown files that have YAML frontmatter but no `schema_class:` will see "mdatron verify: clean" + zero findings, and have no signal that Layer 1 was skipped on every file. This is a silent-success-by-default surface.

**Lens:** Cold-context discoverability (5) + Usability (3). The DR primer dim "Cold-context discoverability" asserts: "What requires reconstruction the doc doesn't enable? Discoverability gaps are findings." This is one.

**Falsifiability:** This no longer applies if EITHER (a) the verify output includes a one-line summary `"N files scanned (M with schema_class, K without)"` so the operator sees the dispatch outcome, OR (b) a `--strict` flag fires `MDATRON-W<code>: file-missing-schema-class` per file that has frontmatter but no `schema_class:`, OR (c) README.md surfaces the schema-class dispatch model in its first 20 lines.

**Routing:** Phase 4 → Phase 1a (README + DESIGN cross-reference) + Phase 2b (verify-output summary). Composes with F5.

**Classification:** Resolved-pending.

---

## Round-close summary

**7 findings raised. None Hallucinated; none Dismissed. Round MUST continue per Phase 3 round-trigger (real findings produced).**

| Finding | Domain | Classification | Routing |
|---|---|---|---|
| F1 | SE + DR | Resolved-pending | Phase 4 → SO or Phase 1a |
| F2 | SE + PE | Resolved-pending | Phase 4 → Phase 1a + 1b |
| F3 | DR | Resolved-pending | Phase 4 → Phase 1a |
| F4 | SE | Resolved-pending | Phase 4 → Phase 1a + 2b |
| F5 | PE + DR | Resolved-pending | Phase 4 → Phase 1a |
| F6 | SC | Deferred-pending-SO | Phase 4 → SO |
| F7 | DR | Resolved-pending | Phase 4 → Phase 1a + 2b |

**MVR signal:** NOT YET. 6 Resolved-pending + 1 Deferred-pending-SO; zero Hallucinated. Phase 3 cycle continues.

**Cross-finding coherence (SC dim 2):** F1 + F6 are the same defect class at two altitudes — error-code allocations drift from the spec. F2 + F4 are the same class — error surfaces collapse failure modes onto opaque catch-all variants. F5 + F7 are the same class — operator-facing discoverability gaps at the v0.1.0 install surface. The mechanical bundle (F1 + F6 spec amendment) is single-PR-able; the install-story bundle (F5 + F7) requires README authoring.

**Sycophancy-compensation reflection:** I resisted dismissing F1 as "Claude knows the spec; the code-spec mismatch is just velocity." The discipline says code that contradicts the spec is a real finding regardless of authorial good faith. F1 is the load-bearing finding; the milestone's robustness fixes are real improvements but the spec-allocation drift they carry forward is the larger gap.

---

## Cross-references

- `mdatron/DESIGN-MDATRON.md § Reserved mdatron codes` (lines 500-516; surface for F1 + F6)
- `mdatron/DESIGN-MDATRON.md § CLI surface` (lines 549-578; surface for F3 + F4)
- `mdatron/mdatron-cli/src/main.rs` (surface for F2-F7)
- `mdatron/mdatron-core/src/verify.rs` (surface for F1-F2, F7)
- `mdatron/V1-SHIP-CRITERIA.md` (composes with F5 explain-stub scoping)
