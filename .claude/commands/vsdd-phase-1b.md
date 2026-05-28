---
primer_id: vsdd-phase-1b
phase: phase-1b
version: 0.1.0
frequency: per-layer
governing_skill: true
relevant_domains: [solution-owner, solution-architect, quality-engineer]
supplements_in_scope: []
---

# Phase 1b Primer: Verification Architecture

## Composition

You are entering Phase 1b (Verification Architecture). Per the phase-domain composition matrix, load:

- **Solution Owner** — spec-contract authority + scope alignment
- **Solution Architect** — architecture lens + purity boundary identification
- **Quality Engineer** — test strategy + falsifiability check

Plus the always-on baseline. Skill mode.

## Phase-specific discipline

Phase 1b authors the **verification architecture** for the layer in `DESIGN.md`. The verification architecture answers four questions:

1. **Which functions are pure?** (deterministic, no I/O, formally verifiable in principle) — these are the purity-boundary candidates for Phase 5 property-based testing + Proof Execution
2. **Which behaviors are automatable?** (testable via standard test infrastructure) vs. **manual-test-only** (requires human + environment-specific verification) — drives the Phase 2a Red Gate test surface vs. the `manual-tests/layer-N.md` checklist
3. **Which behaviors are Phase 5 candidates?** (mutation-testing scope; fuzz-testing scope; proof-execution scope) — drives the project's `**Phase 5 strategy:**` declaration
4. **Where are the trust boundaries?** (input from outside the process: file parsers, network protocol decoders, CLI argument parsers, deserialization entrypoints) — drives Phase 5 Fuzz Testing + Security review scope

The Exacting Mentor stance applies: "purity claim without verification path" is the failure mode. Every pure-function declaration must be verifiable at Phase 5; every trust boundary must be named explicitly; every Phase 5 candidate gets a stated property or invariant.

## Pre-phase composition declaration template

```yaml
phase: phase-1b
composed_domains: [solution-owner, solution-architect, quality-engineer]
composition_mode: skill-interactive
operator_confirmation: confirmed
declared_at: <ISO 8601 timestamp>
```

## Phase-completion criteria

Phase 1b closes when:

- DESIGN.md § Verification architecture lists the layer's pure functions, automatable-vs-manual classification per behavior, Phase 5 candidate behaviors (or explicit `Phase 5 strategy: not applicable — <rationale>`), and named trust boundaries
- `**Phase 5 strategy:**` line is committed at DESIGN.md § Project intent (verbatim per the project's intent calibration)
- SA + QE concur on the purity-boundary list (no silent "pure but takes a clock as parameter" exceptions without explicit named-refinement)

Emit `PhaseExited{phase: phase-1b, exit_status: complete, layer: <N>}` at the closing commit.

## Cross-references

- [Phase 1a primer](./vsdd-phase-1a.md) — Behavioral Specification (typically co-authored)
- [Phase 1c primer](./vsdd-phase-1c.md) — Spec Review Gate
- [Phase 5 primer](./vsdd-phase-5.md) — Formal Hardening (consumes the Phase 1b purity-boundary list)
- [Solution Architect domain](./vsdd-domain-solution-architect.md) — architecture lens
- [Quality Engineer domain](./vsdd-domain-quality-engineer.md) — test strategy
