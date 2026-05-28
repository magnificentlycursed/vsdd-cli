---
primer_id: vsdd-phase-1c
phase: phase-1c
version: 0.1.0
frequency: per-project (re-runs per substantive scope change)
governing_skill: true
relevant_domains: [solution-architect, solution-owner, documentation-reviewer]
supplements_in_scope: []
---

# Phase 1c Primer: Spec Review Gate (Decomposition)

## Composition

You are entering Phase 1c (Spec Review Gate / Decomposition). Per the phase-domain composition matrix, load:

- **Solution Architect** — primary; owns decomposition + architectural layering
- **Solution Owner** — co-stewards spec-gate close; spec-contract authority validation
- **Documentation Reviewer** — cold-reader pass on the decomposition for clarity + completeness

Plus the always-on baseline. Skill mode.

## Phase-specific discipline

Phase 1c decomposes the spec into **layers** — independently-buildable, independently-verifiable units of work. Each layer:

- Names its **acceptance criteria** (a subset of DESIGN.md § Behavioral contracts that this layer closes)
- Names its **manual-tests/layer-N.md** checklist (auto-scaffolded by the `post-design-md-modification.py` hook)
- Names its **Phase 2a Red Gate** seed (failing-by-default test stubs derived from acceptance criteria; also auto-scaffolded)
- Names its **dependencies on earlier layers** (Layer N may depend on Layer N-1's behavior, not on Layer N-1's internals)
- Names its **Exit Signal pointer** (the `ExitSignaled` event reference produced when the layer closes)

The Exacting Mentor stance applies: "layer that depends on a future layer's internals" is the failure mode; "layer whose acceptance criteria don't cover the behaviors it claims to close" is the failure mode; "layer that bundles too many behaviors so the Red Gate is unfalsifiable in aggregate" is the failure mode.

## Pre-phase composition declaration template

```yaml
phase: phase-1c
composed_domains: [solution-architect, solution-owner, documentation-reviewer]
composition_mode: skill-interactive
operator_confirmation: confirmed
declared_at: <ISO 8601 timestamp>
```

## Phase-completion criteria

Phase 1c closes when:

- DESIGN.md § Decomposition lists all layers L1..LN with acceptance criteria per layer
- Each layer's acceptance criteria are a non-empty subset of DESIGN.md § Behavioral contracts
- DR's cold-reader pass produces no Open findings on decomposition completeness
- SO has signed off on the spec-gate close (per the "Raise to SO" routing discipline at methodology.md § Domain change authority)
- `manual-tests/layer-N.md` stubs auto-scaffolded for each layer per the `post-design-md-modification.py` hook

Emit `PhaseExited{phase: phase-1c, exit_status: complete}` at the closing commit. The decomposition opens Phase 2a per-layer cycles.

## Cross-references

- [Phase 1a primer](./vsdd-phase-1a.md) — Behavioral Specification (consumed)
- [Phase 1b primer](./vsdd-phase-1b.md) — Verification Architecture (consumed)
- [Phase 2a primer](./vsdd-phase-2a.md) — Test Suite Generation (opens per-layer after Phase 1c closes)
- [methodology.md § Layer-cycle PR discipline](../../README.md#layer-cycle-pr-discipline) — draft PR opens at Phase 2a; PR template auto-generation
