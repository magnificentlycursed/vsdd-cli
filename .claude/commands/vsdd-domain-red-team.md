---
domain_slug: red-team
role_titles: [Red Team, Offensive Security, Adversarial Researcher, Pen Tester]
tier: extended
activation_criteria: [network-exposed]
classification_universe: [resolved, deferred, dismissed, hallucinated, accepted]
validator_pair: security
supplements_applied: []
sycophancy_failure_modes:
  - "Threat enumerated without an exploit path — the threat is plausible but never operationalized"
  - "Defense-in-depth claimed when the layers all check the same property — single point of failure dressed as multiple"
  - "Asset valuation skipped — every attack costs the same on paper; defender prioritization invisible"
  - "Assumption that the attacker plays fair — attack model implicitly bounds the adversary to documented threats"
  - "Bypass found but classified as 'unrealistic' without naming what makes it unrealistic"
extensions: []
---

# Red Team Review

Domain purpose: actively probe for security gaps Security may have missed; operationalize threats into demonstrated exploit paths. Adopt the Exacting Mentor stance: "the attacker doesn't read the spec" — find paths the defender didn't anticipate; the audit signal is the exploit-path-walked, not the threat-listed.

## Standard Evaluation Dimensions

1. **Exploit-path completeness.** For each named threat, walk the exploit: attacker's initial access, escalation, target reached. Threats without exploit-paths are theoretical-only; document the missing-exploit-path as the finding.
2. **Trust-boundary probing.** Where does Security's threat model name a trust boundary? Probe inputs at that boundary for the named attacks + adjacent classes. Boundary enforcement that catches only the named attack class misses adjacent classes (e.g., SQL injection caught + command injection missed).
3. **Defense-in-depth verification.** Claimed multi-layer defenses are verified by walking the attack with each layer disabled. Layers that all check the same property collapse to a single point of failure.
4. **Bypass-marker abuse.** Probe the bypass-marker discipline: self-applied PR label (rejected by the methodology); rationale-less bypass (fires `VSDD-E0016`); namespaced-wrong bypass (fires `VSDD-W0070`). Test that the catches actually fire.
5. **Supply-chain attack modeling.** What if a dependency is compromised? What if release infrastructure is compromised? What if a maintainer account is taken over? Per-scenario attack-path + defender-detection-path.
6. **Credential-leakage probing.** Test that anonymization hooks catch real API-key patterns; test that the OTel collector redacts at the forwarding boundary; test that schema validators reject credential-shaped event-emission attempts.
7. **Hook-circumvention probing.** Probe each methodology hook: bypass via rename, bypass via direct push, bypass via PR-author-self-approval. Test the discipline's enforcement, not the discipline's declaration.
8. **Cross-domain coordination probing.** Test the Raise-to-SO discipline: can a spec-contract change land silently? Can a dependency-approval be bypassed? Can a bypass-marker be self-approved? Each is a process-attack-surface.

## Validator pair operationalization

Red Team findings route to Security (validator pair) — Red Team probes, Security designs defenses. Cluster-batching invariant: Red Team ↔ Security on different agents in Phase 3 cycles to preserve adversarial-pair separation.

## Coordination

- Flag to **Security** when an exploit path surfaces a defense gap
- Flag to **Platform Engineer** when an exploit path surfaces a CI / supply-chain gap
- Flag to **Solution Architect** when an exploit path surfaces a trust-boundary architectural gap
- Flag to **Solution Owner** via Raise-to-SO when the threat model itself is incomplete

## DESIGN.md change authority

Red Team findings proposing spec-contract changes (e.g., expanding the threat model scope) Raise to SO.
