---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-06-02
phase: phase-3
scope: >-
  Crosslink #12 bundled changes per docs/refactor/phase-1-codes-and-dsl/DESIGN.md
  — reserved-code drift fix (E0001/E0002/E0050/E0070/E0080), DSL Field-access
  symmetry (Field-on-Object-missing-key returns Null), defined() empty-string
  carve-out drop. Privacy lens reviews the data-flow surface: mdatron verify
  JSON envelope, schema-violation error payloads, the reserved-code lint scan,
  and the output_format envelope's version fingerprinting fields.
lens: >-
  Privacy — dim 1 (data classification completeness for envelope fields),
  dim 4 (PII redaction at egress boundaries; in particular error-message
  egress to CI / PR comments), dim 6 (cross-jurisdiction implications for
  operator-private filesystem path leakage), dim 7 (fingerprinting surface
  of the version envelope as a third-party-processor concern).
source: operator-directive
session_note: >-
  Cold-session Phase 3 review of the bundled DESIGN per crosslink #12.
  No prior-session memory. Privacy lens applied to a tool that operates on
  operator-authored markdown corpora; the trust boundary of interest is
  "what mdatron emits leaves the operator's machine" (CI logs, PR comments,
  downstream consumers of the wire format).
model: claude-opus-4-7
execution_method: >-
  Single-domain cold-session review (Privacy primary). No code modifications.
  Operator-supplied prompt enumerated five privacy questions; this entry
  dismisses with rationale per the prompt's "nothing load-bearing → dismiss"
  directive.
sycophancy_compensation: >-
  Privacy-lens bias is to over-classify and demand redaction everywhere;
  resisted by anchoring to "user data" per activation criterion. mdatron's
  operator is the developer, not a third-party data subject; the corpus is
  the developer's own source repo. Treating frontmatter values as PII would
  be sycophantic over-protection.
filename_note: 2026-06-02-privacy-phase-1-bundle.md
supplements_loaded: []
---

# Privacy review — crosslink #12 bundle (DISMISSED)

## Disposition

**Dismissed** — no load-bearing privacy findings.

## Rationale (the five questions, walked)

1. **Filesystem-path leakage in verify JSON.** The envelope reports paths
   relative to project root (per crosslink #11 contract). Even if absolute
   paths slipped, the data subject is the operator running the tool against
   their own repo; the consumer of the JSON is the same operator (or their
   CI, which the operator owns). No third-party data subject; activation
   criterion `handles-user-data` is not met by tool-internal diagnostics.
   Out of scope.

2. **Schema-violation error payloads echoing frontmatter values
   (e.g., `"Launch Day!"`).** Echoing the offending value is a usability
   contract — operators need to see what they wrote to fix it. The "leaks
   to PR comments" framing inverts the threat model: the frontmatter is
   already in the PR diff (markdown source is the PR's content). The error
   message cannot leak data that is not already in-band on the same PR.
   No privacy delta vs. the diff itself.

3. **`defined()` carve-out drop + Field-symmetry change.** Both are pure
   value-level evaluation semantics in the DSL evaluator. Neither changes
   what data flows into diagnostics, error messages, or the JSON envelope.
   Out of scope.

4. **Reserved-code lint.** Scans source code for `MDATRON-Exxxx` literals.
   The scan target is mdatron's own source, not operator corpora. Zero
   user-data surface.

5. **`mdatron_version` + `mdatron_output_version` envelope fingerprinting.**
   Both fields are intentional contract: consumers must be able to detect
   wire-format compatibility. Operator-controlled emission to operator-owned
   downstream consumers. "Malicious downstream consumer" is out-of-threat-model
   — that consumer is the operator's own CI / tooling.

## Cross-domain hand-offs

None. Security validator pair has no co-validation surface here (no
trust-boundary crossings introduced).

## Methodology note

Per the privacy domain prompt's sycophancy failure modes, the failure mode
I avoided is "every field treated as same sensitivity" — recognizing that
a developer-tool's diagnostics about a developer's own source code are not
the same data class as a user-facing system's PII handling.
