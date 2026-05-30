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

Phase 1c decomposes the spec into **milestones** — independently-buildable, independently-verifiable units of work. Each milestone:

- Names its **acceptance criteria** (a subset of DESIGN.md § Behavioral contracts that this milestone closes)
- Names its **manual-tests/layer-N.md** checklist (auto-scaffolded by the `post-design-md-modification.py` hook)
- Names its **Phase 2a Red Gate** seed (failing-by-default test stubs derived from acceptance criteria; also auto-scaffolded)
- Names its **dependencies on earlier milestones** (milestone N may depend on milestone N-1's behavior, not on milestone N-1's internals)
- Names its **Exit Signal pointer** (the `ExitSignaled` event reference produced when the milestone closes)

The Exacting Mentor stance applies: "milestone that depends on a future milestone's internals" is the failure mode; "milestone whose acceptance criteria don't cover the behaviors it claims to close" is the failure mode; "milestone that bundles too many behaviors so the Red Gate is unfalsifiable in aggregate" is the failure mode.

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

- DESIGN.md § Decomposition lists all milestones M1..MN with acceptance criteria per milestone
- Each milestone's acceptance criteria are a non-empty subset of DESIGN.md § Behavioral contracts
- DR's cold-reader pass produces no Open findings on decomposition completeness
- SO has signed off on the spec-gate close (per the "Raise to SO" routing discipline at methodology.md § Domain change authority)
- `manual-tests/layer-N.md` stubs auto-scaffolded for each milestone per the `post-design-md-modification.py` hook

Emit `PhaseExited{phase: phase-1c, exit_status: complete}` at the closing commit. The decomposition opens Phase 2a per-milestone sessions.

## Cross-references

- [Phase 1a primer](./vsdd-phase-1a.md) — Behavioral Specification (consumed)
- [Phase 1b primer](./vsdd-phase-1b.md) — Verification Architecture (consumed)
- [Phase 2a primer](./vsdd-phase-2a.md) — Test Suite Generation (opens per-milestone after Phase 1c closes)
- [methodology.md § Per-milestone PR discipline](../../README.md#per-milestone-pr-discipline) — draft PR opens at Phase 2a; PR template auto-generation
