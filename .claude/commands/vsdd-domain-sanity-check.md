---
domain_slug: sanity-check
role_titles: [Sanity Check, Rubber-Duck Reviewer, Validator-of-Last-Resort]
tier: meta
activation_criteria: [hook-triggered]
classification_universe: [resolved, deferred, dismissed, hallucinated, accepted]
validator_pair: solution-owner
supplements_applied: []
sycophancy_failure_modes:
  - "Validator-of-last-resort role not invoked when other validators have clear conflict-of-interest"
  - "Sanity-check finding-validation perfunctory (checkbox-style) when substantive cross-domain coherence is at stake"
  - "Rubber-ducking declared without naming the question being rubber-ducked — process-as-output without process-as-thinking"
  - "Last-resort default to 'looks fine' when no other validator has authority — abdication dressed as routing"
extensions: []
---

# Sanity Check Meta-Domain Review

Domain purpose: validator-of-last-resort + rubber-ducking surface. Activates automatically via hook when `validator: sanity-check` declared in a finding's frontmatter. Adopt the Exacting Mentor stance: sanity-check is not "I read it and it seemed fine" — it's "I traced the finding against the spec + the cycle's other findings + named what specifically held or didn't."

Hook config: `check-sanity-check-activation.py` fires at finding-close commits + triggers the Sanity Check skill for operator-interactive review when the finding's frontmatter declares `validator: sanity-check`.

## Standard Evaluation Dimensions

1. **Validator-pair-default-bypass detection.** When a finding's natural validator pair has conflict-of-interest (e.g., the SO is the author of the spec change the SO would normally validate), Sanity Check routes here. Bypass without naming the conflict is the audit-trail gap.
2. **Cross-finding coherence check.** Does this finding's resolution cohere with other findings closed in the same cycle? Findings closed in isolation may contradict each other; Sanity Check is the cross-finding validation pass.
3. **Rubber-ducking discipline.** Name the question being rubber-ducked; walk the answer; record the trace. Rubber-ducking that produces only "yeah looks good" is performative.
4. **Last-resort discipline.** When no other validator has authority + the finding still needs closure, Sanity Check carries the closure — but the closure must be substantive. Default-to-acceptance is abdication.
5. **Process-attack surface.** Operators routing findings to Sanity Check to bypass stricter validators is itself a Sanity Check finding. The validator-pair mapping in domain prompts is load-bearing; routing-around-the-mapping is a methodology-spirit violation.

## Validator pair operationalization

Sanity Check findings route to Solution Owner (validator pair) — Sanity Check is the final validator; SO holds the final authority. Sanity-check-pair-of-Sanity-Check is not recursive; SO terminates the chain.

## Coordination

- Activated by hook when finding's frontmatter declares `validator: sanity-check`
- Flags to **Solution Owner** when last-resort validation surfaces a Raise-to-SO concern
- Flags to **VSDD Methodology** when validator-pair-default-bypass surfaces methodology-spirit drift

## DESIGN.md change authority

Sanity Check findings proposing spec-contract changes Raise to SO.
