---
schema_class: phase-primer
primer_id: vsdd-phase-1a
phase: phase-1a
version: 0.1.0
frequency: per-milestone
governing_skill: true
relevant_domains: [solution-owner, ux, accessibility, privacy, localization]
supplements_in_scope: []
---

# Phase 1a Primer: Behavioral Specification

## Composition

You are entering Phase 1a (Behavioral Specification). Per the phase-domain composition matrix, load the following domain skills:

- **Solution Owner** (`/vsdd-domain-solution-owner`) — primary; owns spec-contract authority + scope
- **UX** (`/vsdd-domain-ux`) when `ui-surface: yes` axis is declared
- **Accessibility** (`/vsdd-domain-accessibility`) when `ui-surface: yes` axis is declared
- **Privacy** (`/vsdd-domain-privacy`) when `handles-user-data: yes` axis is declared
- **Localization** (`/vsdd-domain-localization`) when `localized: yes` axis is declared

Plus the core always-on quartet (SO + SA + SE + QE; PE + PerfE join when the project ships code). Skill mode (operator-interactive); not reviewer mode.

## Dispatch & conformance discipline

Phase 1a's agent-work — authoring **and** implementation — runs as a **dispatched, conformance-audited agent**, never in the orchestrator session (the phases-dispatched keystone; supersedes the 2026-07-20 attended/autonomous split: human-judgment work is attended, all phase agent-work is dispatched). The composed governing context — this primer, the composed domains, and the supplements in scope — is delivered by **injection** at dispatch and **audited as skill invocations** (the skill-invocation audit: invocation is the activation signal; a recorded Read is the weaker signal; a paraphrase in the prompt is nonconformance). For a build-phase dispatch the composition SHOULD is this phase's matrix entry — the phase primer, its composed domains, the **core always-on quartet** (SO + SA + SE + QE; PE + PerfE when the project ships code), and the axis-activated product domains — which the conformance verifier audits as `WAS ⊇ SHOULD`. The full **process-governing set** (the eleven process-governing domains) is the audited SHOULD for **review compositions** (Phase 3), not for every build dispatch: wiring the whole set into a per-build gate would force every build dispatch to load all of it, against the efficiency thesis and cold-review independence.

## Phase-specific discipline

Phase 1a authors the **behavioral contracts** for the layer in `DESIGN.md`. Behavioral contracts are observable-from-outside assertions about what the system does — input → output transitions, error conditions, invariants, edge-case behaviors. They are NOT implementation details. A behavioral contract is testable from the layer's external surface without inspecting internal state.

The Exacting Mentor stance applies: every behavior named must be specific (no "handles input gracefully"); every edge case enumerated (empty / null / max-size / unicode / concurrent / partial-failure); every error condition has a stated handling; every invariant has a named falsification path. Vague spec is the failure mode; "the cold reviewer can construct an adversarial example you didn't think of" is the test.

Operators may author Phase 1a and Phase 1b in a single session if the verification architecture surfaces naturally; the methodology lists them as distinct phases per the whitepaper. If authoring jointly, follow the Phase 1b primer's verification-architecture section after the behavioral contracts close.

## Pre-phase composition declaration template

Record at phase-entry commit:

```yaml
phase: phase-1a
composed_domains: [solution-owner, ...axes-activated]
# audited SHOULD for this build dispatch = the phase-matrix entry (these composed_domains + the core always-on quartet + the supplements in scope); the full process-governing set is the audited SHOULD only for review compositions (Phase 3)
invoked_skills: [<the skills actually invoked — the skill-invocation-audit manifest>]
always_on_supplements: [claude-code-cli, bash, rust]
composition_mode: skill-interactive
operator_confirmation: confirmed
declared_at: <ISO 8601 timestamp>
```

The declaration is recorded in the crosslink session breadcrumb and the harness run record as the `PhaseCompositionDeclared` fact — not in the retired `.vsdd/events.jsonl` events store, which is decommissioned. With the events store retired, the methodology lifecycle facts (phase transitions, composition declarations, finding routings, exit signals) all live in those surviving homes. `VSDD-E0050: phase-composition-not-declared` checks the declaration's presence in that surviving home and fires when it is absent at the phase-boundary commit.

## Phase-completion criteria

Phase 1a closes when:

- DESIGN.md § Behavioral contracts is non-empty for the layer
- Every behavior is specific + testable + has named edge cases
- Per-feature-axes-activated domains have surfaced their lens on the spec (UX, A11y, Privacy, L10n as applicable)
- The cold-reader (DR) can produce a falsifying example for any vague behavior; iterate until they cannot

Record the phase transition (`PhaseExited{phase: phase-1a, exit_status: complete, layer: <N>}`) in the crosslink session breadcrumb and the harness run record at the closing commit.

## Cross-references

- [Phase 1b primer](./vsdd-phase-1b.md) — Verification Architecture (often co-authored)
- [Phase 1c primer](./vsdd-phase-1c.md) — Spec Review Gate (decomposition; closes the spec phase)
- [Solution Owner domain](./vsdd-domain-solution-owner.md) — spec-contract authority
- [methodology.md § Domain change authority](../../methodology.md#domain-change-authority) — Raise-to-SO routing
