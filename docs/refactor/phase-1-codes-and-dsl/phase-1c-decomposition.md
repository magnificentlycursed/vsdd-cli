# Phase 1c — Decomposition + Acceptance Criteria

**Issue:** crosslink #12.
**Consumes:** [Phase 1a behavioral specification](./phase-1a-behavioral-spec.md),
[Phase 1b verification architecture](./phase-1b-verification-architecture.md).

## Pre-phase composition declaration

```yaml
phase: phase-1c
composed_domains: [solution-architect, solution-owner, documentation-reviewer, sanity-check]
composition_mode: skill-interactive
supplements_loaded: [rust]
operator_confirmation: confirmed
declared_at: 2026-06-02T21:40:00Z
```

## Decomposition

Single milestone (all three changes ship together as crosslink #12). They
share the same Phase 2a/2b cycle, the same Phase 3 review, the same release.

Rationale for single milestone: the three are individually independent
(none requires another to land first) but each is too small for its own
milestone — splitting would produce three Red Gates of 2-4 tests each,
which is bookkeeping overhead. Bundling lets Phase 2a Red Gate cover all
three contracts in one file, Phase 2b lands one cohesive commit, Phase 3
review covers the bundle as one architectural change.

## Phase 2a Red Gate seeds

**Reserved-code drift:**
- `schema_violation_emits_e0050` (currently emits E0001)
- `frontmatter_parse_failure_emits_e0001` (currently emits E0002)
- `all_emitted_codes_are_reserved` (workspace-wide lint over `.rs`/`.yaml`/
  `.json`/`.toml` carriers)

**DSL Field-access symmetry:**
- `field_on_object_missing_key_returns_null` (currently raises FieldNotFound)
- `field_on_nested_missing_returns_null` (catches deeply-nested case;
  added during Phase 4 per QE + SE convergence)
- `field_on_null_still_returns_null` (regression check)
- `field_on_non_object_value_still_errors` (regression check)

**`defined()` carve-out drop:**
- `defined_empty_string_returns_true` (currently returns false)
- `defined_null_returns_false` (unchanged regression check)
- `defined_empty_array_remains_true` (regression check)

## Acceptance criteria

- Every emitted `MDATRON-Exxxx` code in the workspace maps to a reserved
  range per the amended `DESIGN-MDATRON.md` table
- `Field`-on-Object-missing-key returns `Ok(Value::Null)`; nested case
  also returns `Null`
- `defined("")` returns `true`; `defined(Value::Null)` returns `false`
- vsdd corpus passes `mdatron verify` after the schema-tightening reverts
  on `supplements_in_scope` / `supplements_applied`
- mdatron-core test count goes up (new tests for each contract); no
  pre-existing tests regress

## Phase 1c exit signal

```yaml
event: PhaseExited
phase: phase-1c
exit_status: complete
layer: phase-1-codes-and-dsl
declared_at: 2026-06-02T21:50:00Z
next_phase: phase-2a
milestones_opened: [m1-codes-and-dsl-bundle]
```
