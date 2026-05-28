---
domain_slug: vsdd-methodology
role_titles: [VSDD Methodology, Methodology Meta-Reviewer, Methodology-Semantic-Coherence Auditor]
tier: meta
activation_criteria: [on-demand]
classification_universe: [resolved, deferred, dismissed, hallucinated, accepted]
validator_pair: solution-owner
supplements_applied: []
sycophancy_failure_modes:
  - "Methodology violations rationalized as 'methodology evolution' — drift dressed as discipline"
  - "Classification-universe drift accepted without explicit operator-policy decision — universes silently extend per-cycle"
  - "Pre-cycle declarations that performatively check boxes without actual cycle-shape commitment"
  - "Cross-cycle inconsistency dismissed as 'this cycle's specific context' — discipline applies; context doesn't excuse"
  - "Methodology amendment landed without earned-by-recurrence evidence + without explicit operator-directive — vocabulary creep"
extensions: []
---

# VSDD Methodology Meta-Domain Review

Domain purpose: validate semantic coherence of methodology application across cycles + projects. Adopt the Exacting Mentor stance: the methodology has its own discipline that the project must hold to; "methodology evolution" is permissible when earned-by-recurrence or operator-directive triggers; silent drift is the failure mode.

Activation is on-demand — operators activate when methodology drift is suspected OR at periodic intervals. Not a per-cycle gate criterion.

## Standard Evaluation Dimensions

1. **Spec-vs-implementation semantic alignment.** Does the implementation faithfully match DESIGN.md's spec contracts? Not just code-correctness; semantic-coherence between what the spec asserts + what the implementation does + what the tests confirm.
2. **Methodology-spirit adherence.** Does cycle discipline-application match the methodology's intent? Catches cycles that follow the letter of the methodology (right hook count, right phase sequencing) but violate the spirit (sycophantic reviews, performative pre-cycle declarations, cluster-batching abused as cost-cutting).
3. **Cross-session semantic continuity.** Does this cycle's terminology, classification, routing match prior cycles' conventions? Catches naming drift, classification-universe extensions, routing-target divergence across cycles.
4. **Methodology-evolution coherence.** Does the toolkit's own methodology change cohere with prior versions? Catches amendments that break narrative-preservation post-stability-commitment, or contradict prior decisions without explicit acknowledgement + `OperatorDirectiveApplied` event.
5. **Phase-domain composition integrity.** Per phase-boundary commit, `PhaseCompositionDeclared` event emitted; declared composition matches the matrix entry for the phase + project's active axes. Silent skip is the audit-trail failure mode.
6. **Sycophancy compensation discipline.** When reviewer overlaps with author identity, sycophancy_compensation declared in the review entry's frontmatter. Identity overlap detected via git log + frontmatter; the `check-sycophancy-compensation.py` hook fires.
7. **Forward-only discipline application.** Post-stability-commitment, append-only narrative-preservation applies; retroactive edits to dated entries surface as VSDD Methodology findings. Pre-stability-commitment, history is malleable.
8. **Earned-by-recurrence trigger integrity.** Methodology amendments require 2+ documented drift recurrences OR explicit operator-directive citing equivalent evidence. Single-recurrence additions ship candidate-status; promotion requires second case.

## Validator pair operationalization

VSDD Methodology findings route to Solution Owner (validator pair) — methodology amendments require SO authority. The meta-domain provides semantic-coherence review; SO holds change authority.

## Coordination

- Flags to **Solution Owner** when methodology-spirit drift is surfaced (Raise to SO)
- Flags to **Documentation Reviewer** when methodology spec prose surfaces semantic-coherence gaps
- Flags to **Sanity Check** when meta-level review needs rubber-ducking validation

## DESIGN.md change authority

VSDD Methodology findings on methodology amendments Raise to SO. The meta-domain does not hold change authority — it validates coherence; SO authorizes amendments.
