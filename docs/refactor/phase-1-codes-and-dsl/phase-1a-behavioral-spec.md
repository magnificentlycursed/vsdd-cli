# Phase 1a — Behavioral Specification

**Issue:** crosslink #12 (Phase 1 of binary-first plan).
**Parent plan:** [`../binary-first-plan.md`](../binary-first-plan.md).

## Pre-phase composition declaration

```yaml
phase: phase-1a
composed_domains: [solution-owner, solution-architect, software-engineer, quality-engineer, sanity-check]
composition_mode: skill-interactive
supplements_loaded: [rust]
operator_confirmation: confirmed
declared_at: 2026-06-02T21:30:00Z
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

**Spec amendments needed in DESIGN-MDATRON.md:506-514** — three new rows for
the new ranges + two "Reserved for future use" placeholder rows for the
explicit gaps.

**Observable assertions:**

- Every emission in the workspace maps to a reserved range per the amended
  table
- Code-allocation lint enforces: any literal code in `.rs`/`.yaml`/`.json`/
  `.toml` carriers must match a reserved range
- No behavior change for adopters of the output-format envelope beyond the
  numeric values of the codes emitted (the `mdatron_output_version` field is
  unchanged because adding a new code within a reserved range is additive)

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

## Phase 1a exit signal

```yaml
event: PhaseExited
phase: phase-1a
exit_status: complete
layer: phase-1-codes-and-dsl
declared_at: 2026-06-02T21:30:00Z
next_phase: phase-1b
```
