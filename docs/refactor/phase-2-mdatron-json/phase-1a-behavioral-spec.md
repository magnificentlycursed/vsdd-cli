# Phase 1a — Behavioral Specification

**Issue:** crosslink #13 (Phase 2 of binary-first plan).
**Parent plan:** [`../binary-first-plan.md`](../binary-first-plan.md).
**Consumes:** [Phase 0 output-format DESIGN](../phase-0-output-format/DESIGN.md);
[Phase 1 codes + DSL](../phase-1-codes-and-dsl/phase-1a-behavioral-spec.md).

## Pre-phase composition declaration

```yaml
phase: phase-1a
composed_domains: [solution-owner, solution-architect, software-engineer, technical-writer, documentation-reviewer, sanity-check]
composition_mode: skill-interactive
supplements_loaded: [rust]
operator_confirmation: confirmed
declared_at: 2026-06-07T00:00:00Z
```

## Scope

Four discrete changes that tighten mdatron's operator-facing surface around
the `--json` flag already shipped by Phase 0 (crosslink #11). All four ride
the same milestone per § Decomposition rationale in Phase 1c.

1. **`mdatron verify --json` finalization.** The Phase 0 surface emits the
   output object; Phase 2 closes the open ends — render the `= explain:`
   diagnostic line at the CLI layer (today only `Finding::format_tty()`
   renders it; `mdatron-cli/src/main.rs:170-183`'s `print_finding` does
   not), and align the CLI's TTY diagnostic with the contract documented
   at `phase-0-output-format/DESIGN.md:229-244`.
2. **`mdatron explain <code>`.** Implement the subcommand against an
   embedded per-code catalog (today returns the placeholder
   `"extended docs not yet implemented at v0.1.0"` at
   `mdatron-cli/src/main.rs:189-192`). v0.1.0 baseline catalog covers the
   five reserved + emitted codes (`MDATRON-E0001`, `E0002`, `E0050`,
   `E0070`, `E0080`); subsequent code introductions extend the catalog at
   emission-site time.
3. **mdatron README.** First-author the repo-root `README.md` per the
   DR-F1 finding (install + first run + schema/pattern example +
   relationship to vsdd). Three audiences explicitly addressed: vsdd
   downstream user, mdatron-only adopter (non-VSDD project), tool-author
   composing mdatron as a substrate. Per [[feedback-no-premature-version-bumps]]
   no version bump on the unpublished crate; README cites
   `cargo install --path ...` for the bootstrap period and notes the
   Phase 6 cutover to crates.io.
4. **`tests/cli_integration.rs`.** Add a CLI integration test file
   distinct from `tests/output_format.rs` (which covers the BC-1..BC-8
   output-format-contract surface). The new file covers the surfaces
   newly added in Phase 2 — `explain` subcommand behavior, the rendered
   `= explain:` line under TTY mode, README presence + structure
   discoverability, and drive-by coverage of `--quiet --json` flag
   combination per the Phase 0 stream-contract matrix.

## Behavioral contracts

### `mdatron verify --json` finalization

**Current state:** Phase 0 implementation at
`mdatron-cli/src/main.rs:75-168` emits the JSON output object on stdout
under `--json`. The TTY diagnostic block (`print_finding` at
`mdatron-cli/src/main.rs:170-183`) renders code, summary, file:line,
message, and an optional `= help:` line — but does **not** render the
`= explain: mdatron explain <code>` line that the contract documents
at `mdatron-core/src/diagnostic.rs:70-99`. The Phase 0 DESIGN's open
question #2 (SO disposition 2026-06-02) settled this: the line is
retained because crosslink #13 (this phase) implements the surface it
promises.

**Change:** Render the `= explain:` line whenever the finding's
`explain_ref` is `Some`. Use `Finding::format_tty()` directly rather
than the open-coded format in `print_finding` — single source of truth
for TTY rendering.

**Observable assertions:**

- A verify run that emits at least one finding with `explain_ref ==
  Some(_)` emits a stderr block containing the substring `= explain:
  mdatron explain <code>` on a line of its own per finding
- The `= explain:` line is suppressed under `--quiet`
- The `= explain:` line is suppressed when `explain_ref` is `None`
  (e.g., the pipeline-error block in `print_pipeline_error`)
- Under `--json`, the JSON output object's `findings[*].explain_ref`
  field is unchanged from the Phase 0 contract — this change is
  TTY-surface only

**Edge cases:**

- A finding with `help` AND `explain_ref` both `Some`: both lines emit,
  `= help:` before `= explain:`, matching rustc convention
- A finding with `explain_ref` pointing at a code not in the embedded
  catalog: the line still emits (the link is advisory; the catalog miss
  surfaces only when the operator actually runs `mdatron explain` —
  see § `mdatron explain` below)

**Falsification path:** integration test asserts the stderr substring
on a fixture that produces a finding with `explain_ref` set; regression
that drops the line fires the test.

### `mdatron explain <code>`

**Current state:** `cmd_explain` at `mdatron-cli/src/main.rs:189-192`
prints `"mdatron explain <code>: extended docs not yet implemented at
v0.1.0"` and exits 2.

**Change:** Implement the subcommand against an embedded per-code
catalog. Catalog scope per Phase 0 DESIGN open question #2 SO
disposition: one paragraph of prose per emitted code at v0.1.0 baseline
(`MDATRON-E0001`, `E0002`, `E0050`, `E0070`, `E0080`). Catalog grows by
one entry per newly-emitted code thereafter; emission without a catalog
entry is a code-allocation lint failure (Phase 5 candidate; not v0.1.0
blocking).

Embedded as `include_str!` constants in `src/embedded.rs` (the
post-Phase-4 location) or `mdatron-cli/src/explain.rs` for the
bootstrap-period two-crate shape. v0.1.0 ships the two-crate shape;
Phase 4 of the binary-first refactor collapses to single crate; the
explain-catalog files survive the move unchanged.

**Per-code explain page format:**

```markdown
# <code> — <one-line-summary>

**Severity:** error | warning | lint
**Status:** accepted
**Introduced in:** 0.1.0

## What this means

<one paragraph: what condition fired this code; the user's view of "why
this diagnostic exists">

## How to fix

<one paragraph: the corrective pattern in Mentor voice; what to do, not
what was wrong>

## Related codes

- (optional bullet list of adjacent codes)
```

The four required headings are normative; per-page extra sections are
permitted at the author's discretion (matches rustc's `--explain`
convention where some pages have additional context).

**Observable assertions:**

- `mdatron explain MDATRON-E0001` exits 0 and writes the catalog page
  for `MDATRON-E0001` on stdout
- `mdatron explain MDATRON-E0002` / `MDATRON-E0050` / `MDATRON-E0070` /
  `MDATRON-E0080` likewise
- `mdatron explain <code-not-in-catalog>` exits 2 and writes a
  rustc-shaped diagnostic on stderr — `error[MDATRON-Exxxx]: explain
  page not found for <code>` plus a hint that the catalog grows per
  emitted code
- `mdatron explain --json MDATRON-E0001` is **not** in v0.1.0 scope;
  the global `--json` flag applies to `verify` only per Phase 0
  global-flag table (the flag is rejected at clap parse time for
  `explain` in v0.1.0; v0.1.x candidate)

**Edge cases:**

- Empty code argument (`mdatron explain ""`): clap-level non-zero exit
  with descriptive error
- Code in lowercase (`mdatron explain mdatron-e0001`): exits 2 with
  message naming the case-sensitivity of codes — adopters paste codes
  verbatim from diagnostic output, so case-normalization is
  intentionally NOT performed
- VSDD-namespace code (`mdatron explain VSDD-E0207`): exits 2;
  rationale message names that mdatron's explain catalog covers
  `MDATRON-Exxxx` codes only and references `vsdd explain` for the
  VSDD namespace (the latter being a vsdd-cli responsibility per
  namespace-separation in `phase-0-output-format/DESIGN.md:281-299`)

**Falsification path:** integration test asserts exit 0 + non-empty
stdout for each of the five baseline codes; regression that drops a
catalog entry fires the test. Test fixture for the not-in-catalog
case asserts exit 2 + the named-not-found behavior.

### mdatron README

**Current state:** mdatron repo root contains no `README.md` file (per
the `find` audit at the time of this spec; only `CHANGELOG.md`,
`DESIGN-MDATRON.md`, `BOUNDARY-PREAMBLE.md`, `BOOTSTRAP-MITIGATION.md`,
`V1-SHIP-CRITERIA.md`, `dsl-falsifiability-report.md`, `STEP-2-SCOPE.md`).

**Change:** Author `mdatron/README.md` per the DR-F1 finding (recorded
in `binary-first-plan.md` row 12). Length target: ~200-400 lines —
short enough to read in one sitting; long enough to ground each
audience.

**Required structural sections** (in order; each non-empty):

| Section | Audience-served | Why required |
|---|---|---|
| 1. One-line positioning + Schematron lineage + TRON-blockchain disambiguation | All | DESIGN-MDATRON.md:47-63 sets the disambiguation discipline (TW-F3) |
| 2. Install | All | First action any new adopter takes |
| 3. First run (5-minute walkthrough) | All | Hello-world equivalent; covers `mdatron init`-ish + `mdatron verify` |
| 4. Schema example (Layer 1) | mdatron-only adopter + tool-author | Shows JSON Schema dispatch concretely |
| 5. Pattern example (Layer 2) | mdatron-only adopter + tool-author | Shows the DSL's value-add over plain JSON Schema |
| 6. Relationship to vsdd | vsdd downstream user | Names that vsdd is one adopter; mdatron is methodology-agnostic |
| 7. Where to go next | All | Pointers to DESIGN-MDATRON.md, `mdatron explain CODE`, the vsdd-cli adopter's docs |

**Observable assertions:**

- `mdatron/README.md` exists at the repo root
- The seven section headings appear in declared order (heading-text
  match: each section names the listed responsibility, but the exact
  heading text is author-discretion — the DR-F1 contract is the
  audience-coverage shape, not heading wording)
- The TRON-blockchain disambiguation sentence appears in section 1
  (per TW-F3)
- `cargo install --path mdatron-cli --locked` appears in the install
  section (bootstrap-period install command; Phase 6 of the
  binary-first plan switches this to `cargo install mdatron --locked`
  on first crates.io publish)
- The README does **not** assert a version above 0.1.0 anywhere in
  prose (per [[feedback-no-premature-version-bumps]] — the README
  ships alongside an unpublished 0.1.0 crate; version language is
  forward-looking only)

**Edge cases:**

- Examples that diverge from the actual DSL: the README cites
  `DESIGN-MDATRON.md` as the canonical DSL reference; examples are
  illustrative + must round-trip against `mdatron verify` (validated
  in tests/cli_integration.rs — see below)
- Adopter pasting a snippet from the README: the snippet must produce
  the documented diagnostic on the first try (no
  "modify this slightly" prose)

**Falsification path:** integration test reads `mdatron/README.md`
and asserts the seven required heading topics + the disambiguation
sentence + the install command + the no-version-overclaim invariant.
A drive-by snippet round-trip test runs the README's schema/pattern
example against `mdatron verify` and asserts the expected diagnostic.

### `tests/cli_integration.rs`

**Current state:** `mdatron-cli/tests/output_format.rs` exists and
covers BC-1..BC-8 from Phase 0. No second integration-test file.

**Change:** Add `mdatron-cli/tests/cli_integration.rs` distinct from
output_format.rs. The split: `output_format.rs` owns the
output-format-contract surface (Phase 0); `cli_integration.rs` owns the
CLI-as-product surface (Phase 2 — explain, README, the rendered TTY
explain line, `--quiet --json` orthogonality from Phase 0 BC-5 that
output_format.rs did not yet cover).

**Test surface scope:**

1. `mdatron explain MDATRON-E0001` exits 0 with non-empty stdout
   containing the required heading "What this means"
2. Same for `MDATRON-E0002`, `MDATRON-E0050`, `MDATRON-E0070`,
   `MDATRON-E0080`
3. `mdatron explain MDATRON-E9999` exits 2 with stderr naming the
   not-found behavior
4. `mdatron explain VSDD-E0207` exits 2 with stderr message pointing
   at namespace separation (per Phase 0 BC § namespace-separation)
5. A verify run on a fixture that emits a finding with
   `explain_ref == Some(_)` emits a stderr block containing
   `= explain: mdatron explain MDATRON-E0050` (or equivalent code)
6. `mdatron verify --quiet` produces no stderr output even on findings
7. `mdatron verify --quiet --json` produces JSON on stdout AND no
   stderr (Phase 0 BC-5 stream contract row 3)
8. `mdatron/README.md` exists and contains the seven required topic
   headings + the TRON-blockchain disambiguation
9. The README's pattern/schema example round-trips against
   `mdatron verify` (drive-by; ensures README does not rot)

**Observable assertions:** each named test passes; collectively they
cover the four bundled changes.

**Edge cases:**

- Test fixture for assertion #5 needs a real schema + a failing
  markdown file. `output_format.rs` already has the `TempProject`
  helper shape; reuse a similar shape in `cli_integration.rs` rather
  than refactoring out a shared helper (single-milestone scope; the
  shared-helper refactor is a Phase 4 binary-first-plan candidate
  when the workspace collapses to single crate)
- Assertion #9 (README round-trip): the README's example must compile
  against the running mdatron in the test process — the example is
  the source-of-truth for adopter copy-paste correctness

**Falsification path:** absence of any one assertion above is itself
the falsification; the test file is the surface that catches
regressions in Phase 2's scope after Phase 3 cold-session review.

## Cross-references to Phase 0 + Phase 1

This Phase 2 spec consumes:

- **Phase 0 output-format DESIGN** (`phase-0-output-format/DESIGN.md`)
  for the BC-1..BC-8 contract that `--json` already honors; Phase 2
  extends to the CLI-as-product surface around it
- **Phase 0 open question #2 SO disposition** (DESIGN.md:566-575) for
  the reversal of DR-F2's "strip the line" decision
- **Phase 1 codes + DSL spec** (`phase-1-codes-and-dsl/phase-1a-behavioral-spec.md`)
  for the reserved-codes table that bounds the v0.1.0 explain catalog
  (E0001, E0002, E0050, E0070, E0080)

This Phase 2 spec produces (forward references):

- Phase 1b verification architecture (purity boundary + automation
  classification for the four bundled changes)
- Phase 1c decomposition + acceptance criteria
- Phase 2a Red Gate (failing tests for the four bundled changes
  per `tests/cli_integration.rs`)
- Phase 2b implementation (TTY-line render switch, explain catalog
  embed, README author, CLI integration tests)
- Phase 3 cluster-batched cold-session review

## Operator-directive housekeeping (M5 F3+F9 audit-trail discipline)

The Phase 0 DESIGN's open question #2 SO disposition is dated
2026-06-02 but was not captured as `OperatorDirectiveApplied` at the
time; the resolution survives only in the DESIGN.md prose. Phase 2's
opening commit MUST emit:

```yaml
event: OperatorDirectiveApplied
directive: phase-2-explain-disposition-honored
rationale: |
  Phase 0 DESIGN open question #2 (SO disposition 2026-06-02:
  implement explain for v0.1.0; reverse DR-F2's "strip the line"
  finding) is honored by Phase 2. The binary-first plan's row 14 text
  ("Strip = explain: line") was authored before the SO disposition
  landed and remains in the plan as residual drift; Phase 2 closing
  commit amends row 14 to reflect "Implement explain catalog +
  retain line" with a footnote naming the 2026-06-02 SO disposition.
declared_at: 2026-06-07T00:00:00Z
issue: crosslink-13
```

This is the M5 F3+F9 directive-emission discipline operationalized
forward (per the binary-first-plan.md § Operator directives recorded
methodology amendment owed): the disposition is captured AT the point
the directive is acted on, not in retrospect.

## Phase 1a exit signal

```yaml
event: PhaseExited
phase: phase-1a
exit_status: complete
layer: phase-2-mdatron-json
declared_at: 2026-06-07T00:00:00Z
next_phase: phase-1b
```
