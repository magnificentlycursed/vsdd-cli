# Phase 1 — Reserved-Code Drift + DSL Fixes DESIGN

**Status:** Phase 1 design.
**Issue:** crosslink #12.
**Parent plan:** [`docs/refactor/binary-first-plan.md`](../binary-first-plan.md) Phase 1.

## Pre-phase composition declarations

```yaml
phase: phase-1a
composed_domains: [solution-owner, solution-architect, software-engineer, quality-engineer, sanity-check]
composition_mode: skill-interactive
supplements_loaded: [rust]
operator_confirmation: confirmed
declared_at: 2026-06-02T21:30:00Z
context: |
  Three concrete changes for crosslink #12. The behavioral specs are already
  documented in binary-first-plan.md, review-log 2026-06-02-platform-engineer-init-drift
  (M4 PE F1), and review-log 2026-06-02-software-engineer-mdatron-dsl-catchup (M7 F6).
  This DESIGN consolidates them with explicit acceptance criteria so the
  Phase 2a Red Gate can be authored against a single artifact.

phase: phase-1b
composed_domains: [solution-owner, solution-architect, quality-engineer, sanity-check]
composition_mode: skill-interactive
supplements_loaded: [rust]
operator_confirmation: confirmed
declared_at: 2026-06-02T21:35:00Z

phase: phase-1c
composed_domains: [solution-architect, solution-owner, documentation-reviewer, sanity-check]
composition_mode: skill-interactive
supplements_loaded: [rust]
operator_confirmation: confirmed
declared_at: 2026-06-02T21:40:00Z
```

## Scope

Three discrete internal changes to mdatron-core. All are pure-Rust; none
change the operator-facing CLI surface (verify still verifies; explain still
explains). The output-format contract from crosslink #11 is unaffected
except that some emitted codes change number.

1. **Reserved-code drift fix.** Current impl emits codes that violate the
   reserved-codes table at `DESIGN-MDATRON.md:506-514`. Rename emissions to
   match the spec; amend the spec where new ranges are needed (IO failures,
   pipeline orchestration, frontmatter schema validation).
2. **DSL `Field`-access symmetry.** `mdatron-core/src/dsl/expr.rs:221-236`
   has `Field-on-Null` returning `Null` while `Field-on-Object-missing-key`
   raises `FieldNotFound`. Make both return `Null`. This is the M4 PE F1
   root cause.
3. **Drop `defined()` empty-string carve-out.** `mdatron-core/src/dsl/expr.rs:322-330`
   treats `defined("")` as `false`. Drop the carve-out; `defined()` becomes
   strict not-`Null`. Per Phase 4 disposition #10 (2026-06-02).

## Behavioral contracts

### Reserved-code drift fix

**Current emissions vs spec mapping:**

| Code emitted | Currently emitted for | Per spec range | Disposition |
|---|---|---|---|
| `MDATRON-E0001` | `frontmatter-schema-violation` (verify.rs:264) | E0001-E0009 = parsing failures | Rename emission to `MDATRON-E0050` |
| `MDATRON-E0002` | `frontmatter-parse-failed` (verify.rs:231) | E0001-E0009 = parsing failures | Rename emission to `MDATRON-E0001` |
| `MDATRON-E0070` | `io: cannot resolve project root` (main.rs) | Unspecified | Reserve new range E0070-E0079 = IO failures |
| `MDATRON-E0080` | `verify pipeline failed` (main.rs) | Unspecified | Reserve new range E0080-E0089 = Pipeline orchestration failures |

**Spec amendments needed in DESIGN-MDATRON.md:506-514:**

Add three rows:

| Range | Class |
|---|---|
| `MDATRON-E0050` — `E0059` | Frontmatter schema validation failures |
| `MDATRON-E0070` — `E0079` | IO failures during verify pipeline |
| `MDATRON-E0080` — `E0089` | Pipeline orchestration failures |

**Observable assertions:**

- Every emission in mdatron-core / mdatron-cli source maps to a reserved
  range per the amended table
- Code-allocation lint enforces: any `"MDATRON-Exxxx"` string literal must
  match a reserved range
- No behavior change for adopters of the wire output format beyond the
  numeric values of the codes emitted (`mdatron_output_version` stays
  `1.0.0` because adding a new code numeric within reserved ranges is an
  additive change, not a wire-format break)

### DSL `Field`-access symmetry

**Current implementation** (`mdatron-core/src/dsl/expr.rs:221-236`):

```rust
Expr::Field(inner, name) => {
    let v = evaluate(inner, ctx)?;
    match v {
        Value::Object(o) => o.get(name).cloned().ok_or_else(|| {
            EvalError::FieldNotFound { field: name.clone(), on: "object" }
        }),
        Value::Null => Ok(Value::Null),  // Null propagates
        other => Err(EvalError::TypeMismatch { ... }),
    }
}
```

**Change:** When the object exists but the key is missing, return `Value::Null`
(matching the `Null` propagation branch above).

**Why this matters:** Today, an optional frontmatter field requires
schema-tightening (mark it required-but-empty-allowed) to avoid pattern
crashes. After this change, patterns can naturally reference an optional
field; a missing field flows as `Null` through `defined()` / equality / etc.

**Side effect:** Two earlier reactive schema-tightenings can be reverted —
`supplements_in_scope` in `phase-primer.json` and `supplements_applied` in
`domain-prompt.json` were promoted to required because the previous DSL
behavior crashed when they were absent. Revertable after this fix.

**Observable assertions:**

- `evaluate(Expr::Field(obj, "missing_key"), ctx)` returns `Ok(Value::Null)`
  when `obj` is an `Object` that doesn't contain `missing_key`
- The `EvalError::FieldNotFound` variant is no longer emitted from the
  Field-on-Object-missing-key path; existing tests that match on it
  need updating (or removal if the variant becomes dead)
- vsdd's pattern files containing `every(s in $self.optional_field, ...)`
  evaluate without error when `$self.optional_field` is absent

### `defined()` empty-string carve-out drop

**Current implementation** (`mdatron-core/src/dsl/expr.rs:322-330`):

```rust
"defined" => {
    let v = evaluate(&args[0], ctx)?;
    let is_defined = match &v {
        Value::Null => false,
        Value::Str(s) => !s.is_empty(),  // ← the carve-out
        _ => true,
    };
    Ok(Value::Bool(is_defined))
}
```

**Change:** Drop the `Value::Str` branch. `defined(x)` becomes strict
not-`Null`:

```rust
"defined" => {
    let v = evaluate(&args[0], ctx)?;
    Ok(Value::Bool(!matches!(v, Value::Null)))
}
```

**Why:** Asymmetry today — `defined([])` returns `true` while `defined("")`
returns `false`. Adopters using `defined()` import XPath / JSON Schema / Jsonnet
mental models where `defined` means "not Null." For the genuinely-want-non-empty
case, `$self.field != ""` is one extra character.

**Observable assertions:**

- `defined("")` returns `true` (was: `false`)
- `defined(Value::Null)` returns `false` (unchanged)
- `defined(Value::Array([]))` returns `true` (unchanged)
- No corpus pattern depends on the carve-out (audited prior; confirmed
  via grep over vsdd-core/patterns/)

## Verification architecture (Phase 1b)

### Pure functions

All three changes touch pure functions; Phase 5 property-testing surface:

| Function | Purity grounds |
|---|---|
| `evaluate(Expr::Field(...), ctx)` | Pure given non-side-effecting ctx; deterministic |
| `call_function("defined", [arg], ctx)` | Pure value -> bool |
| Code-emission sites in verify.rs | Pure: input findings shape determines output code string |

### Automatable vs manual

All assertions are automatable via existing test infrastructure
(mdatron-core's existing test surface; verify.rs's in-module tests;
new tests in `mdatron-core/src/dsl/expr.rs#[cfg(test)]`).

No manual-test surface; all three changes are internal-only behavior.

### Phase 5 candidates

- Property test: `defined(x)` = `!matches!(x, Value::Null)` for all `Value`
  variants (proptest Arbitrary impl over Value)
- Property test: `Field(Object(o), k)` returns `Ok(Null)` when `!o.contains_key(k)`
  for all `(Object, String)` pairs

### Trust boundaries

Unchanged. All three changes are internal to mdatron-core; no new
input-from-outside surface.

## Decomposition (Phase 1c)

Single milestone (all three changes ship together as crosslink #12). They
share the same Phase 2a/2b cycle, the same Phase 3 review, the same release.

Rationale for single milestone: the three are individually independent
(none requires another to land first) but each is too small for its own
milestone — splitting would produce three Red Gates of 2-4 tests each,
which is bookkeeping overhead. Bundling lets Phase 2a Red Gate cover all
three contracts in one file, Phase 2b lands one cohesive commit, Phase 3
review covers the bundle as one architectural change.

### Phase 2a Red Gate seeds

**Reserved-code drift:**
- emit_schema_violation_emits_E0050 (currently emits E0001)
- emit_parse_failure_emits_E0001 (currently emits E0002)
- code_allocation_lint_rejects_unreserved_code (covers E0070 / E0080 reservation)

**DSL Field-access symmetry:**
- field_on_missing_key_returns_null (currently raises FieldNotFound)
- every_over_missing_optional_field_does_not_panic (real-world repro)

**`defined()` carve-out drop:**
- defined_empty_string_returns_true (currently returns false)
- defined_null_returns_false (unchanged regression check)

### Acceptance criteria

- Every emitted MDATRON-Exxxx code in the codebase maps to a reserved range
  per the amended DESIGN-MDATRON.md table
- `Field`-on-Object-missing-key returns `Ok(Value::Null)`; the
  `EvalError::FieldNotFound` variant is unused (can be removed or retained
  for the Object/non-Object case)
- `defined("")` returns `true`; `defined(Value::Null)` returns `false`
- vsdd corpus passes `mdatron verify` after the schema-tightening reverts
  on `supplements_in_scope` / `supplements_applied`
- mdatron-core test count goes up (new tests for each contract); no
  pre-existing tests regress

## Phase exit signals

```yaml
event: PhaseExited
phase: phase-1c
exit_status: complete
layer: phase-1-codes-and-dsl
declared_at: 2026-06-02T21:50:00Z
next_phase: phase-2a
milestones_opened: [m1-codes-and-dsl-bundle]
```
