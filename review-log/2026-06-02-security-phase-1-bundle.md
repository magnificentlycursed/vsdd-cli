---
schema_class: review-entry
schema_version: 1.0.0
review_number: 4
date: 2026-06-02
phase: phase-3
scope: >-
  vsdd-cli crosslink #12 Phase 1 bundle (codes-and-dsl). Subject —
  mdatron-core reserved-code rename + lint, DSL Field-on-Object-missing-key
  symmetry, defined() empty-string carve-out drop, plus the vsdd-cli schema
  reverts (supplements_in_scope / supplements_applied dropped from required).
lens: >-
  Security — trust-boundary placement, input-validation completeness,
  forensic-trail integrity, defense-in-depth weakening. Validator pair:
  Red Team (on a different cluster).
source: director-raised
session_note: >-
  Cold-session adversarial probe. Threats named against the specific
  trust-boundary model of vsdd-cli (operator-local config + human-authored
  frontmatter; no network surface).
model: claude-opus-4-7
execution_method: >-
  Cold-session sub-agent, worktree-no-memory, Phase 3 primer + Security +
  Sanity Check prompts loaded fresh; no prior-cycle context retained.
sycophancy_compensation: >-
  Bundle authored by Claude in a prior session; Security lens treats the
  author as anonymous and the spec as the only authority.
supplements_loaded: []
---

# Security — Phase 1 bundle adversarial review

## F1 — Field-symmetry null-propagation as rule-masking vector [Dismissed]

domain: security / dim: trust-boundary-placement / routing: n/a

Threat: an operator authors YAML omitting a key a pattern depends on; the
missing key flows as `Null`, causing a defect-catching rule to short-circuit.

Trust-boundary analysis: frontmatter is **inside** the trust boundary.
Operators are not adversaries to themselves; patterns are the contract the
operator opted into. The remaining adversary class — hostile contributor
landing frontmatter in a shared repo — is governed by code review, not by
DSL semantics. The contributor could equally omit any required field today.

Dismissal: no named threat actor maps here. Reroute if a
frontmatter-as-untrusted-input threat is later added.

## F2 — defined() carve-out drop weakens classification-label discipline [Accepted with remediation]

domain: security / dim: defense-in-depth /
routing: documentation-reviewer + solution-architect

Threat: a downstream adopter writes `defined($self.privacy_classification)`
intending "must declare a non-empty label." Today the rule rejects
`privacy_classification: ""`; post-change, empty-string passes silently.

This is **real** defense-in-depth weakening for any adopter who imported
the carve-out as load-bearing. DESIGN names the mitigation
(`$self.field != ""`) but does not name the migration surface for adopters
who don't read this DESIGN. Audit-trail gap: the classification-coverage
check passing empty-string is invisible — rule reports satisfied, empty
label flows through, no event emitted.

Remediation:
- DESIGN must declare this a **breaking DSL semantic**, not a "behavior
  reconciliation." Bump the DSL semantic version (the wire-format version
  is irrelevant here) and ship a migration note calling out the carve-out
  drop + recommended replacement pattern.
- Phase 5 candidate: emit a one-time deprecation event when a pattern uses
  `defined()` against a field whose observed corpus value is empty-string.

## F3 — Reserved-code lint suffix-slice fragility [Resolved-pending]

domain: security / dim: input-validation-completeness /
routing: software-engineer

`suffix[1..]` is safe given the `chars().next()` precondition (first byte is
ASCII letter, slice index on a char boundary), but the safety is implicit.
A future refactor loosening the precondition silently breaks the reasoning.

Remediation: replace with `suffix.get(1..).unwrap_or("")`. One line; bounds
reasoning becomes local and refactor-proof.

## F4 — is_reserved_mdatron_code() unbounded input [Dismissed]

domain: security / dim: resource-exhaustion / routing: n/a

A 1GB "MDATRON-" input wastes one `parse::<u32>()`. The function is internal
to the compile-time lint surface — no network, IPC, or operator-string path
reaches it without upstream bounded length (source-file size).

Dismissal: not on a trust-boundary surface. Future caller adding a
network-reachable path carries the obligation to add a length cap at the
new ingress point.

## F5 — format!()-evasion is documented-and-accepted [Accepted]

domain: security / dim: bypass-marker-discipline /
routing: red-team (validator pair)

The lint provides a discipline signal, not a security guarantee. Red Team's
F1 has the same shape and routes the remediation.

Security-specific add: this matches the domain-prompt failure mode —
"validation inside the boundary against attacks from outside it." The lint
runs in CI on code the contributor wrote; a contributor with merge authority
shapes the input to evade. Defense lives at review-author discipline, not
at the lint.

## F6 — Schema-tightening revert silently degrades downstream consumers [Resolved-pending]

domain: security / dim: forensic-trail-integrity /
routing: data-engineer + documentation-reviewer

Removing the required-field enforcement on `supplements_in_scope` /
`supplements_applied` is necessary given the field-symmetry fix, but the
revert is silent: a downstream tool that consumes these schemas will accept
documents that omit the fields where it previously rejected them.

Forensic-trail consequence: audit replay over historical primers will show
documents passing under the new schema that would have failed under the old.
`schema_version` is unchanged; the replay has no signal to distinguish.

Remediation: bump `schema_version` on `phase-primer.json` and
`domain-prompt.json` (minor — additive relaxation). Record the revert in
the CHANGELOG with a migration note.

## Session summary

6 findings: 2 Dismissed (F1, F4 — no named threat), 1 Accepted with
remediation (F2 — DSL semantic version + adopter migration note required),
2 Resolved-pending (F3, F6 — concrete code/schema changes), 1 Accepted
(F5 — routed to Red Team's parallel finding).

Cold-session isolation preserved; validator-pair-separation preserved.
