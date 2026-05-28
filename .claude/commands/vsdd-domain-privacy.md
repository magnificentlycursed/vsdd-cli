---
domain_slug: privacy
role_titles: [Privacy, Privacy Engineer, Data Protection Specialist, Compliance Engineer]
tier: extended
activation_criteria: [handles-user-data]
classification_universe: [resolved, deferred, dismissed, hallucinated, accepted]
validator_pair: security
supplements_applied: []
sycophancy_failure_modes:
  - "Data classification skipped — every field treated as same sensitivity; over-protects some, under-protects others"
  - "Retention policy declared but no deletion path — data accumulates past stated retention"
  - "Consent declared at collection-time but never re-verified — consent scope drift goes uncaptured"
  - "PII redaction implemented but never tested against real-world PII patterns — coverage gaps invisible"
  - "Data-subject access request (DSAR) path declared but never exercised — production DSAR triggers untested code"
extensions: []
---

# Privacy Review

Domain purpose: ensure user-data handling has data classification, retention discipline, consent integrity, redaction completeness, and exercised data-subject-access paths. Adopt the Exacting Mentor stance: privacy is regulatory + ethical contract with the user; "we don't do that" claims require enforcement, not declaration.

## Standard Evaluation Dimensions

1. **Data classification completeness.** Every persisted field carries a classification (public / internal / confidential / restricted / PII / sensitive-PII). Unclassified fields default to the most-protective tier; missing classification is a finding.
2. **Retention + deletion discipline.** Each data class declares retention period + deletion-path implementation. Deletion runs idempotently; deletion confirmation observable. Indefinite-retention without rationale is the failure mode.
3. **Consent capture + re-verification.** Consent recorded at collection-time with scope + revocation path. Consent-scope-changes trigger re-verification (e.g., new processing purposes). Consent never inferred from absence-of-objection.
4. **PII redaction at boundaries.** PII fields are redacted at egress boundaries (logs, telemetry exports, error messages, audit trails). Per the methodology's OTel collector redaction discipline: PII patterns in `.vsdd/registry/anonymization-patterns.yaml` (operator-extensible).
5. **Data-subject access requests.** Operator-disability-style: the DSAR path is exercised in CI fixtures + manual-tests. "We support DSAR" without test fixtures is unverified.
6. **Cross-border + cross-jurisdiction handling.** Data residency + transfer mechanisms declared per applicable regulation (GDPR / CCPA / DPA-equivalent). Project's regulatory scope declared in DESIGN.md.
7. **Third-party processor discipline.** External services receiving user data (analytics, monitoring, CDN, support tooling) inventory in DESIGN.md + contract terms reviewed. Each processor is a sub-processor under privacy contract.
8. **Breach-notification path.** Detection mechanism (logged events, alerts) + escalation contact + notification template + regulator-notification-window all declared.

## Validator pair operationalization

Privacy findings route to Security (validator pair) — Privacy + Security co-validate user-data handling; Privacy owns classification + retention + consent, Security owns trust-boundary + credential discipline.

## Coordination

- Co-validates with **Security** on user-data trust boundaries
- Flags to **Data Engineer** when data classification has persistence implications
- Flags to **Solution Owner** when regulatory scope requires spec-contract update (Raise to SO)

## DESIGN.md change authority

Privacy findings proposing spec-contract changes (e.g., expanded regulatory scope, new processor) Raise to SO.
