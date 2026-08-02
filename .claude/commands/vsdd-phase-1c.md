---
schema_class: phase-primer
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

Plus the core always-on quartet. Skill mode.

## Dispatch & conformance discipline

Phase 1c's agent-work — authoring **and** implementation — runs as a **dispatched, conformance-audited agent**, never in the orchestrator session (the phases-dispatched keystone; supersedes the 2026-07-20 attended/autonomous split: human-judgment work is attended, all phase agent-work is dispatched). The composed governing context — this primer, the composed domains, and the supplements in scope — is delivered by **injection** at dispatch and **audited as skill invocations** (the skill-invocation audit: invocation is the activation signal; a recorded Read is the weaker signal; a paraphrase in the prompt is nonconformance). For a build-phase dispatch the composition SHOULD is this phase's matrix entry — the phase primer, its composed domains, the **core always-on quartet** (SO + SA + SE + QE; PE + PerfE when the project ships code), and the axis-activated product domains — which the conformance verifier audits as `WAS ⊇ SHOULD`. The full **process-governing set** (the eleven process-governing domains) is the audited SHOULD for **review compositions** (Phase 3), not for every build dispatch: wiring the whole set into a per-build gate would force every build dispatch to load all of it, against the efficiency thesis and cold-review independence.

## Phase-specific discipline

Phase 1c decomposes the spec into **milestones** — independently-buildable, independently-verifiable units of work. Each milestone:

- Names its **acceptance criteria** (a subset of DESIGN.md § Behavioral contracts that this milestone closes)
- Names its **manual-tests/layer-N.md** checklist obligation (the checklist itself is operator-authored at Phase 2a entry — the operator authors the oracle; no scaffold mechanism exists)
- Names its **Phase 2a Red Gate** seed (failing-by-default test stubs derived from acceptance criteria; the stubs are authored in the Phase 2a act itself)
- Names its **dependencies on earlier milestones** (milestone N may depend on the preceding milestone's behavior, not on its internals)
- Names its **Exit Signal pointer** (the `ExitSignaled` fact recorded in the harness run record and the crosslink session breadcrumb when the milestone closes)

The Exacting Mentor stance applies: "milestone that depends on a future milestone's internals" is the failure mode; "milestone whose acceptance criteria don't cover the behaviors it claims to close" is the failure mode; "milestone that bundles too many behaviors so the Red Gate is unfalsifiable in aggregate" is the failure mode.

## Pre-phase composition declaration template

```yaml
phase: phase-1c
composed_domains: [solution-architect, solution-owner, documentation-reviewer]
# audited SHOULD for this build dispatch = the phase-matrix entry (these composed_domains + the core always-on quartet + the supplements in scope); the full process-governing set is the audited SHOULD only for review compositions (Phase 3)
invoked_skills: [<the skills actually invoked — the skill-invocation-audit manifest>]
always_on_supplements: [claude-code-cli, bash, rust]
composition_mode: skill-interactive
operator_confirmation: confirmed
declared_at: <ISO 8601 timestamp>
```

## Phase-completion criteria

Phase 1c closes when:

- DESIGN.md § Decomposition lists all milestones with acceptance criteria per milestone
- Each milestone's acceptance criteria are a non-empty subset of DESIGN.md § Behavioral contracts
- DR's cold-reader pass produces no Open findings on decomposition completeness
- SO has signed off on the spec-gate close (per the "Raise to SO" routing discipline at the spec contract § Solution Owner change authority)
- Each milestone names its `manual-tests/layer-N.md` checklist obligation (the checklist itself is operator-authored at Phase 2a entry — the operator authors the oracle)

Record the phase transition (`PhaseExited{phase: phase-1c, exit_status: complete}`) in the crosslink session breadcrumb and the harness run record at the closing commit. The decomposition opens Phase 2a per-milestone sessions.

## Cross-references

- [Phase 1a primer](./vsdd-phase-1a.md) — Behavioral Specification (consumed)
- [Phase 1b primer](./vsdd-phase-1b.md) — Verification Architecture (consumed)
- [Phase 2a primer](./vsdd-phase-2a.md) — Test Suite Generation (opens per-milestone after Phase 1c closes)
- [the spec contract § Per-milestone PR discipline](../../.design/agent-first-vsdd-toolkit.md#per-milestone-pr-discipline) — draft PR opens at Phase 2a; PR template auto-generation
