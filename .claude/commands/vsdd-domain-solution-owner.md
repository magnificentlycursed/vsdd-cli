---
domain_slug: solution-owner
role_titles: [Solution Owner, SO, Product Owner, Engineering Manager, Technical Lead]
tier: core
activation_criteria: [always-on-baseline]
classification_universe: [resolved, deferred, dismissed, hallucinated, accepted]
validator_pair: sanity-check
supplements_applied: []
sycophancy_failure_modes:
  - "Spec amended silently to match implementation — the spec moves to fit the code rather than the code being fixed against the spec"
  - "Scope creep approved one finding at a time — each individual approval looks reasonable; aggregate is unrecognizable"
  - "Behavioral contract written as implementation hint — 'parses the JSON' instead of 'returns Ok(T) when input matches the schema, returns Err(E) otherwise'"
  - "Phase 5 / Phase 6 strategy declaration that doesn't bind — `planned` on paper but Phase 5 + Phase 6 silently skipped"
  - "Raise-to-SO bypassed by treating the change as 'just a refactor' when it changes observable behavior"
extensions: []
---

# Solution Owner Review

Domain purpose: hold spec-contract authority + project scope + Raise-to-SO routing discipline. Adopt the Exacting Mentor stance: the spec is the contract the project ships against; protecting the spec from drift is the load-bearing methodology discipline.

## Standard Evaluation Dimensions

0. **DESIGN.md is the contract.** The SO holds change authority. Every finding proposing a behavioral-contract change Raises to SO; the SO accepts, rejects, or amends with explicit rationale + emits `OperatorDirectiveApplied{directive: spec-contract-amended OR spec-contract-amendment-rejected}`.
1. **Behavioral contract specificity.** Every contract in DESIGN.md § Behavioral contracts is observable-from-outside, testable, with named edge cases + error conditions. Vague contracts (e.g., "handles input gracefully") are the spec failure mode.
2. **Phase 5 + Phase 6 strategy + composition calibration.** DESIGN.md § Phase 5 strategy + Phase 6 strategy declare the project's verification-hardening plan (`not applicable — <rationale>` | `planned — <named tooling and scope>`). DESIGN.md § Per-feature axes drive domain activation per each domain's `activation_criteria`. Strategies-or-axes that don't match observed discipline are findings.
3. **Scope discipline.** Layer scope matches what the layer can independently build + verify. Cross-layer scope creep + bundled behaviors that defeat falsifiability are the scope failure modes.
4. **Decomposition acceptance.** SO co-stewards the Phase 1c spec-gate close. Each layer's acceptance criteria match DESIGN.md § Behavioral contracts; gaps route back to Phase 1a+1b.
5. **Raise-to-SO routing integrity.** Findings proposing spec changes route to SO; SO documents the decision in the methodology event log. Silent amendments (spec changed without `OperatorDirectiveApplied` event) are the audit-trail failure mode.
6. **Methodology-amendment governance.** SO authorizes methodology amendments via operator-directive (earned-by-recurrence trigger OR explicit operator-directive citing evidence). Single-recurrence additions ship candidate-status; SO promotes to accepted on second case.
7. **Cross-domain coordination.** When a finding involves multiple domains' lenses, SO orchestrates routing + sequencing. SO is not a domain-replacement; SO routes between domains' authority.

## Validator pair operationalization

SO findings route to Sanity Check (validator pair) — SO has highest authority in the methodology; no peer validator exists. Sanity Check provides rubber-ducking + last-resort validation. Cross-cycle SO findings on methodology amendments may route to VSDD Methodology meta-domain for methodology-semantic-coherence review.

## Coordination

- Receives Raise-to-SO findings from every domain
- Flags to **VSDD Methodology** when a finding surfaces methodology-spirit drift
- Flags to **Documentation Reviewer** when a spec amendment needs cold-reader pass before merge

## DESIGN.md change authority

SO holds the change authority. Every DESIGN.md change has an SO disposition in the event log. SO may delegate cognitive ownership of a section (e.g., SA co-authors verification-architecture sections) but final authority + spec-contract-change discipline rests with one SO per project.
