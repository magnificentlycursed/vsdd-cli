# Phase 1b — Verification Architecture

**Issue:** crosslink #12.
**Consumes:** [Phase 1a behavioral specification](./phase-1a-behavioral-spec.md).

## Pre-phase composition declaration

```yaml
phase: phase-1b
composed_domains: [solution-owner, solution-architect, quality-engineer, sanity-check]
composition_mode: skill-interactive
supplements_loaded: [rust]
operator_confirmation: confirmed
declared_at: 2026-06-02T21:35:00Z
```

## Pure functions

All three changes touch pure functions; Phase 5 property-testing surface:

| Function | Purity grounds |
|---|---|
| `evaluate(Expr::Field(...), ctx)` | Pure given non-side-effecting ctx; deterministic |
| `call_function("defined", [arg], ctx)` | Pure value → bool |
| Code-emission sites in verify.rs | Pure: input findings shape determines output code string |

## Automatable vs manual

All assertions are automatable via existing test infrastructure
(mdatron-core's existing test surface; verify.rs's in-module tests;
new tests in `mdatron-core/src/dsl/expr.rs#[cfg(test)]`).

No manual-test surface; all three changes are internal-only behavior.

## Phase 5 candidates

- Property test: `defined(x)` = `!matches!(x, Value::Null)` for all `Value`
  variants (proptest `Arbitrary` impl over `Value`)
- Property test: `Field(Object(o), k)` returns `Ok(Null)` when `!o.contains_key(k)`
  for all `(Object, String)` pairs
- Property test: every emitted code in the workspace matches the
  `is_reserved_mdatron_code` predicate (the lint is currently a static test;
  could be reframed as a fuzz target over file content)

## Trust boundaries

Unchanged. All three changes are internal to mdatron-core; no new
input-from-outside surface.

## Phase 1b exit signal

```yaml
event: PhaseExited
phase: phase-1b
exit_status: complete
layer: phase-1-codes-and-dsl
declared_at: 2026-06-02T21:35:00Z
next_phase: phase-1c
```
