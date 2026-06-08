# Phase 1c — Decomposition + Acceptance Criteria

**Issue:** crosslink #13.
**Consumes:** [Phase 1a behavioral specification](./phase-1a-behavioral-spec.md),
[Phase 1b verification architecture](./phase-1b-verification-architecture.md).

## Pre-phase composition declaration

```yaml
phase: phase-1c
composed_domains: [solution-architect, solution-owner, documentation-reviewer, sanity-check]
composition_mode: skill-interactive
supplements_loaded: [rust]
operator_confirmation: confirmed
declared_at: 2026-06-07T00:10:00Z
```

## Decomposition

**Single milestone** (crosslink #13 ships all four bundled changes
together). Operator disposition (recorded at the head of this Phase 2
cycle, 2026-06-07): "Single milestone (matches #12)".

Rationale (mirrors Phase 1's decomposition rationale):

- The four changes share a verification surface (`cli_integration.rs`
  is the integration test file for all four)
- They share a release boundary (Phase 2's deliverable is the README +
  explain subcommand + tightened TTY rendering; partial-Phase-2
  releases would ship a non-coherent operator surface, e.g., a README
  citing `mdatron explain` without the subcommand implemented)
- They share a Phase 3 review surface (the cluster-batched
  cold-session covers the bundle; splitting would multiply Phase 3
  invocations without proportional value)
- Each individual change is small (~50-200 LOC); splitting would
  produce four Red Gates with ~1-3 tests each — bookkeeping overhead
  exceeds isolation value

Counterargument considered + rejected: the README is a prose artifact
while the other three are code; a "code vs prose" split would let
Phase 3 DR review parallelize with SE/QE. Rejected because the README's
embedded example round-trips against the code surface (`mdatron verify`
on the README's pattern/schema example); coupling is structural and
splitting would create cross-milestone test dependencies.

## Phase 2a Red Gate seeds

The Red Gate adds `mdatron-cli/tests/cli_integration.rs` with failing
tests for each contract. Test names follow Phase 1's pattern (snake_case
behavior-descriptive).

**`--json` finalization (TTY explain line):**
- `verify_tty_renders_explain_line_for_finding_with_explain_ref`
- `verify_tty_suppresses_explain_line_under_quiet`
- `verify_tty_does_not_render_explain_line_when_explain_ref_is_none`

**`mdatron explain CODE`:**
- `explain_e0001_emits_catalog_page_with_what_this_means`
- `explain_e0002_emits_catalog_page`
- `explain_e0050_emits_catalog_page`
- `explain_e0070_emits_catalog_page`
- `explain_e0080_emits_catalog_page`
- `explain_unknown_code_exits_two_with_not_found_message`
- `explain_vsdd_namespace_code_points_at_namespace_separation`

**README presence + round-trip:**
- `readme_exists_at_repo_root`
- `readme_contains_seven_required_topic_headings`
- `readme_contains_tron_disambiguation_sentence`
- `readme_cites_cargo_install_path_for_bootstrap_period`
- `readme_does_not_overclaim_version`
- `readme_pattern_example_round_trips_against_mdatron_verify`

**Drive-by `--quiet --json` coverage from Phase 0 BC-5 row 3:**
- `verify_quiet_json_combination_outputs_json_on_stdout_only`
- `verify_quiet_alone_suppresses_all_stderr_output`

Total Red Gate test count: **18 tests** in `cli_integration.rs`. All
fail-by-default at Phase 2a (the catalog, the README, and the
TTY-line render switch each don't exist yet).

## Acceptance criteria

This milestone closes when:

- **`Finding::format_tty()` is the single source of truth for TTY
  diagnostic rendering at the CLI layer.** `mdatron-cli/src/main.rs`'s
  `print_finding` either delegates to `format_tty()` or is removed in
  favor of direct `format_tty()` calls
- **The five baseline explain pages exist** as embedded constants
  (`include_str!` against `mdatron-cli/src/explain/MDATRON-E0001.md`
  etc.); each contains the four required headings; `mdatron explain
  CODE` returns the page on stdout + exits 0
- **mdatron explain rejects unknown codes** with exit 2 + a
  named-not-found stderr message; explicit handling for the
  VSDD-namespace case (per `phase-0-output-format/DESIGN.md`
  namespace-separation contract)
- **`mdatron/README.md` exists** with the seven required topic
  sections, the TRON-blockchain disambiguation sentence in section 1,
  the bootstrap-period install command, and no version-overclaim
- **The README's pattern/schema example round-trips** against
  `mdatron verify` (drive-by anti-rot guarantee)
- **`mdatron-cli/tests/cli_integration.rs`** exists with the 18
  Red Gate tests all passing; `cargo test --workspace --locked`
  green
- **`binary-first-plan.md` row 14** updated to reflect the SO
  disposition reversal — old text "Strip" replaced with
  "Implement explain catalog + retain line"; footnote names the
  2026-06-02 SO disposition + the `OperatorDirectiveApplied` event
  emission discipline applied here
- **vsdd corpus still passes** `mdatron verify` after Phase 2 lands
  (no regression in the load-bearing pre-commit hook surface)

## Drive-by hygiene (in-scope but not blocking)

Spotted during Phase 1a authoring; resolve if cost is trivial. Refactor-
shaped items may land in **Phase 2b OR Phase 2c** at the implementer's
discretion; the wording below names landing-phase as guidance, not
mandate (per crosslink #13 SA F3: Phase 2c is the refactor phase, so
moving a refactor item there is methodology-correct):

- `mdatron-cli/src/main.rs:170-183` `print_finding` has open-coded
  TTY rendering that drifts from `Finding::format_tty`; the Phase 2b/2c
  consolidation closes the drift
- `mdatron-cli/src/main.rs:189-192` `cmd_explain` placeholder text
  exits 2 even for valid codes — Phase 2b's catalog implementation
  flips this to exit 0 for known codes

Both are removed by Phase 2c at the latest; not separate milestones.

## Phase 1c exit signal

```yaml
event: PhaseExited
phase: phase-1c
exit_status: complete
layer: phase-2-mdatron-json
declared_at: 2026-06-07T00:15:00Z
next_phase: phase-2a
milestones_opened: [m1-mdatron-json-and-explain-bundle]
```
