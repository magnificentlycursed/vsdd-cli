---
schema_class: phase-primer
primer_id: vsdd-phase-2c
phase: phase-2c
version: 0.1.0
frequency: per-milestone (optional — runs when Phase 2b surfaced refactor opportunities)
governing_skill: true
relevant_domains: [software-engineer, solution-architect]
supplements_in_scope: []
---

# Phase 2c Primer: Refactor

## Composition

You are entering Phase 2c (Refactor). Per the phase-domain composition matrix, load:

- **Software Engineer** — owns implementation changes
- **Solution Architect** — owns the architectural lens for refactor decisions

Plus the core always-on quartet. Skill mode.

## Dispatch & conformance discipline

Phase 2c's agent-work — authoring **and** implementation — runs as a **dispatched, conformance-audited agent**, never in the orchestrator session (the phases-dispatched keystone; supersedes the 2026-07-20 attended/autonomous split: human-judgment work is attended, all phase agent-work is dispatched). The composed governing context — this primer, the composed domains, and the supplements in scope — is delivered by **injection** at dispatch and **audited as skill invocations** (the skill-invocation audit: invocation is the activation signal; a recorded Read is the weaker signal; a paraphrase in the prompt is nonconformance). For a build-phase dispatch the composition SHOULD is this phase's matrix entry — the phase primer, its composed domains, the **core always-on quartet** (SO + SA + SE + QE; PE + PerfE when the project ships code), and the axis-activated product domains — which the conformance verifier audits as `WAS ⊇ SHOULD`. The full **process-governing set** (the eleven process-governing domains) is the audited SHOULD for **review compositions** (Phase 3), not for every build dispatch: wiring the whole set into a per-build gate would force every build dispatch to load all of it, against the efficiency thesis and cold-review independence.

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
# audited SHOULD for this build dispatch = the phase-matrix entry (these composed_domains + the core always-on quartet + the supplements in scope); the full process-governing set is the audited SHOULD only for review compositions (Phase 3)
invoked_skills: [<the skills actually invoked — the skill-invocation-audit manifest>]
always_on_supplements: [claude-code-cli, bash, rust]
composition_mode: skill-interactive
operator_confirmation: confirmed
declared_at: <ISO 8601 timestamp>
```

## Phase-completion criteria

Phase 2c closes when:

- `cargo test` returns 0 at every commit
- No externally-observable behavior changed (any behavior change routes to Phase 1a via Phase 4)
- The refactor's stated goal is met (extract-method / naming / dep-reduction / etc.) or explicitly skipped

Record the phase transition (`PhaseExited{phase: phase-2c, exit_status: complete | skipped-no-refactor-surface, layer: <N>}`) in the crosslink session breadcrumb and the harness run record at the closing commit. The milestone reaches **implementation-MVR-ready** — the milestone's Phase 3 session opens.

## Cross-references

- [Phase 2b primer](./vsdd-phase-2b.md) — Minimal Implementation (consumed)
- [Phase 3 primer](./vsdd-phase-3.md) — Adversarial Refinement (next; opens after Phase 2c closes)
- [Solution Architect domain](./vsdd-domain-solution-architect.md) — architecture lens
- [Software Engineer domain](./vsdd-domain-software-engineer.md)
