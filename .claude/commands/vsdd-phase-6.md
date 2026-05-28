---
primer_id: vsdd-phase-6
phase: phase-6
version: 0.1.0
frequency: per project (once at project-terminal close; once per re-open)
governing_skill: true
relevant_domains: [vsdd-methodology]
supplements_in_scope: []
---

# Phase 6 Primer: Convergence (The Exit Signal)

## Composition

Phase 6 is **operator-orchestrated attestation** — no specific domain composition. The operator (or a single agent with the convergence primer loaded) walks the four dimensions and produces the convergence record.

Always-on baseline applies for coordination but no domain owns Phase 6 work itself. The VSDD Methodology meta-domain validates methodology-semantic-coherence at the close.

## Phase-specific discipline

Phase 6 evaluates **four independent MVR signals** and the **cross-dimension consistency check**. Phase 6 is not a build phase — nothing is implemented in Phase 6; the project-level record demonstrates that the four dimensions converged. Phase 6 sits AFTER every layer's Phase 5 closes (or declares not-applicable per project intent).

### Dimension 1: Spec MVR

**Established by:** Solution Owner cold-batch reviews across final 2+ layers produced only Hallucinated findings, AND Phase 4 routing across final 2+ layers produced no `route:phase-1a+1b` destinations.

**Anti-signal:** SO final-layer review closed cleanly but earlier rounds surfaced spec gaps that were Resolved by silent DESIGN.md amendments (no subsequent SO re-pass verified the amendment).

### Dimension 2: Test MVR

**Established by:** Phase 5 Mutation Testing produced per-layer kill rates with **every surviving mutant having a recorded disposition** in the QE log's per-layer Mutation Testing round.

**Verification step (required):** open each cited Mutation Testing round + confirm a per-mutant disposition table exists (rows: mutant location, mutation kind, disposition, rationale). Aggregate-only kill rate fails Dimension 2 regardless of the rate.

**Not-applicable alternative:** when DESIGN.md declares `**Phase 5 strategy:** not applicable — <rationale>`, Dimension 2's signal is QE's final-round attestation against Dim 2 (test falsifiability) without mutation-tool evidence — explicitly weaker; named in the convergence record as such.

### Dimension 3: Implementation MVR

**Established by:** Phase 3 final-round summaries per active domain across the final 2+ layers all read "only Hallucinated findings" or "no findings" — and cold-session-isolation discipline was preserved across the active domain set.

**Anti-signal:** Round closed with "no findings" but cold-session isolation relaxed (one fresh chat reused for multiple domains; context bleed).

### Dimension 4: Formal-verification MVR

**Established by:** Phase 5 Proof Execution harnesses each have recorded outcomes in the SA log's per-layer Proof Execution round.

**Not-applicable alternative:** `**Phase 5 strategy:** Proof Execution not applicable — <rationale>` — convergence closes on three of four dimensions; formal-verification dimension explicitly out of scope; named in convergence record.

## Cross-dimension consistency check

For each of the spec's named behaviors:

| Behavior (DESIGN.md ref) | Spec assertion | Test (file:line) | Implementation (file:fn) | Formal verification (harness) | Consistent? |

Inconsistent rows route via Phase 4 BEFORE convergence is declared. Inconsistency types:
- **Spec asserts X; tests don't** → route to Phase 2a
- **Tests assert X; spec doesn't** → route to Phase 1a+1b (or Phase 2a if test is over-specific)
- **Implementation does X; spec + tests don't say** → route to Phase 1a+1b (or Phase 2b if dead behavior)
- **Proof contradicts spec or tests** → route to Phase 1a+1b (property was wrong) or Phase 2b (implementation violates spec)

## Pre-phase composition declaration template

```yaml
phase: phase-6
composed_domains: [vsdd-methodology]
composition_mode: operator-orchestrated
operator_confirmation: confirmed
project_intent: <intent-level per DESIGN.md>
phase_5_strategy: <verbatim from DESIGN.md>
phase_6_strategy: <verbatim from DESIGN.md>
declared_at: <ISO 8601 timestamp>
```

## Phase-completion criteria

Phase 6 closes when:

1. New round titled `Phase 6 four-dimensional convergence (project-terminal)` exists in `vsdd-suite/review-log/<close-date>-vdd-iar-alignment.md` with all four dimensions populated (or three populated + the fourth explicitly declared out of scope)
2. Cross-dimension consistency check table has zero inconsistent rows
3. Every inconsistency surfaced during the check has been routed via Phase 4 + the routed work has landed
4. Convergence attestation is signed and dated in the round's closing block (anonymized-project posture uses commit-sha as the signature)
5. CHANGELOG.md final entry references the Phase 6 convergence round by link

Emit `ExitSignaled{project: <name>, attestation_commit: <sha>, attested_by: <fingerprint-or-handle>, per_dimension: {...}, cross_dimension_consistency_check: pass}` at the closing commit. The project is at the Exit Signal — converged.

## Cross-references

- [Phase 3 primer](./vsdd-phase-3.md) — implementation-MVR signal (Dimension 3)
- [Phase 5 primer](./vsdd-phase-5.md) — Test + Formal-verification MVR signals (Dimensions 2 + 4)
- [Solution Owner domain](./vsdd-domain-solution-owner.md) — Spec MVR signal (Dimension 1)
- [VSDD Methodology meta-domain](./vsdd-domain-vsdd-methodology.md) — methodology-semantic-coherence at the close
- [methodology.md § MVR and Exit Signal convergence](../../methodology.md#mvr-and-exit-signal-convergence) — convergence discipline overview
