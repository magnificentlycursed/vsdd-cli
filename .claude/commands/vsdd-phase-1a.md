---
primer_id: vsdd-phase-1a
phase: phase-1a
version: 0.1.0
frequency: per-layer
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

Plus the always-on baseline (SE + QE + SA + SO; PE + PerfE if the project ships code). Skill mode (operator-interactive); not reviewer mode.

## Phase-specific discipline

Phase 1a authors the **behavioral contracts** for the layer in `DESIGN.md`. Behavioral contracts are observable-from-outside assertions about what the system does — input → output transitions, error conditions, invariants, edge-case behaviors. They are NOT implementation details. A behavioral contract is testable from the layer's external surface without inspecting internal state.

The Exacting Mentor stance applies: every behavior named must be specific (no "handles input gracefully"); every edge case enumerated (empty / null / max-size / unicode / concurrent / partial-failure); every error condition has a stated handling; every invariant has a named falsification path. Vague spec is the failure mode; "the cold reviewer can construct an adversarial example you didn't think of" is the test.

Operators may author Phase 1a and Phase 1b in a single session if the verification architecture surfaces naturally; the methodology lists them as distinct phases per the whitepaper. If authoring jointly, follow the Phase 1b primer's verification-architecture section after the behavioral contracts close.

## Pre-phase composition declaration template

Emit at phase-entry commit:

```yaml
phase: phase-1a
composed_domains: [solution-owner, ...axes-activated]
composition_mode: skill-interactive
operator_confirmation: confirmed
declared_at: <ISO 8601 timestamp>
```

The declaration emits a `PhaseCompositionDeclared` observability event. Absent declaration at phase-boundary commit fires `VSDD-E0050: phase-composition-not-declared`.

## Phase-completion criteria

Phase 1a closes when:

- DESIGN.md § Behavioral contracts is non-empty for the layer
- Every behavior is specific + testable + has named edge cases
- Per-feature-axes-activated domains have surfaced their lens on the spec (UX, A11y, Privacy, L10n as applicable)
- The cold-reader (DR) can produce a falsifying example for any vague behavior; iterate until they cannot

Emit `PhaseExited{phase: phase-1a, exit_status: complete, layer: <N>}` at the closing commit.

## Cross-references

- [Phase 1b primer](./vsdd-phase-1b.md) — Verification Architecture (often co-authored)
- [Phase 1c primer](./vsdd-phase-1c.md) — Spec Review Gate (decomposition; closes the spec phase)
- [Solution Owner domain](./vsdd-domain-solution-owner.md) — spec-contract authority
- [methodology.md § Domain change authority](../../methodology.md#domain-change-authority) — Raise-to-SO routing
