---
primer_id: vsdd-phase-2c
phase: phase-2c
version: 0.1.0
frequency: per-layer (optional — runs when Phase 2b surfaced refactor opportunities)
governing_skill: true
relevant_domains: [software-engineer, solution-architect]
supplements_in_scope: []
---

# Phase 2c Primer: Refactor

## Composition

You are entering Phase 2c (Refactor). Per the phase-domain composition matrix, load:

- **Software Engineer** — owns implementation changes
- **Solution Architect** — owns the architectural lens for refactor decisions

Plus the always-on baseline. Skill mode.

## Phase-specific discipline

Phase 2c **re-shapes the implementation while keeping tests green**. Every commit during Phase 2c maintains the Phase 2a Red Gate's green status — `cargo test` returns 0 throughout. The refactor surfaces:

- Extract-method / extract-trait opportunities surfaced by Phase 2b's minimal implementation
- Naming improvements (per the naming + coinage governance — descriptive at point-of-use)
- Dependency reduction (where Phase 2b added a dep that became unnecessary post-impl)
- Purity-boundary refinements (per the Phase 1b verification architecture)

The Exacting Mentor stance applies: "refactor that breaks tests; re-fixes tests" is the failure mode (the refactor is at the wrong altitude); "refactor that adds complexity to make a single test prettier" is the failure mode (the test was probably wrong, not the implementation); "refactor that changes externally-observable behavior" is a Phase 1a finding routed via Phase 4, not a Phase 2c change.

Phase 2c is **optional** — if Phase 2b produced clean minimal implementation, Phase 2c may close immediately as `PhaseExited{phase: phase-2c, exit_status: skipped-no-refactor-surface}`.

## Pre-phase composition declaration template

```yaml
phase: phase-2c
composed_domains: [software-engineer, solution-architect]
composition_mode: skill-interactive
operator_confirmation: confirmed
declared_at: <ISO 8601 timestamp>
```

## Phase-completion criteria

Phase 2c closes when:

- `cargo test` returns 0 at every commit
- No externally-observable behavior changed (any behavior change routes to Phase 1a via Phase 4)
- The refactor's stated goal is met (extract-method / naming / dep-reduction / etc.) or explicitly skipped

Emit `PhaseExited{phase: phase-2c, exit_status: complete | skipped-no-refactor-surface, layer: <N>}` at the closing commit. The layer reaches **implementation-MVR-ready** — the layer's Phase 3 cycle opens.

## Cross-references

- [Phase 2b primer](./vsdd-phase-2b.md) — Minimal Implementation (consumed)
- [Phase 3 primer](./vsdd-phase-3.md) — Adversarial Refinement (next; opens after Phase 2c closes)
- [Solution Architect domain](./vsdd-domain-solution-architect.md) — architecture lens
- [Software Engineer domain](./vsdd-domain-software-engineer.md)
