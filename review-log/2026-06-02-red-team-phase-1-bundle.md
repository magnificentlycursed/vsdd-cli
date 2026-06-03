---
schema_class: review-entry
schema_version: 1.0.0
review_number: 3
date: 2026-06-02
phase: phase-3
scope: >-
  vsdd-cli crosslink #12 Phase 1 bundle (codes-and-dsl). Subject —
  mdatron-core reserved-code rename + lint, DSL Field-on-Object-missing-key
  symmetry, defined() empty-string carve-out drop, plus the vsdd-cli schema
  reverts (supplements_in_scope / supplements_applied dropped from required).
lens: >-
  Red Team — attacker model, abuse vectors, contract evasion. Five-lens
  weighting Attacker (5) > Consistency (3) > Edge (2) > Maintainability (1)
  > Usability (1). Validator pair: Security (on a different cluster).
source: director-raised
session_note: >-
  Cold-session adversarial probe. Threats are operationalized into named
  exploit paths; threats lacking exploit paths are themselves the finding.
  Validator-pair-separation invariant preserved (Security on another cluster).
model: claude-opus-4-7
execution_method: >-
  Cold-session sub-agent, worktree-no-memory, Phase 3 primer + Red Team +
  Sanity Check prompts loaded fresh; no prior-cycle context retained.
sycophancy_compensation: >-
  Bundle authored entirely by Claude; Red Team lens treats the author as
  a hostile contributor for the purpose of probing evasion paths.
supplements_loaded: []
---

# Red Team — Phase 1 bundle adversarial review

## F1 — Reserved-code lint is a string-grep; trivially evadable [Resolved-pending]

domain: red-team / dim: hook-circumvention / source: domain-raised /
routing: software-engineer + platform-engineer

Exploit path: a hostile contributor wanting to ship a non-reserved code
has three documented bypasses, none of which the lint catches:

1. **Runtime construction.** `format!("MDATRON-E{:04}", 9999)` — the lint
   scans for literal `"MDATRON-Exxxx"` substrings. The Phase 2a fixture
   already documents this with the `VSDD-` prefix trick. A malicious PR
   can wear the same disguise as a "legitimate test fixture."
2. **Non-`.rs` carrier.** Code embedded in a `.yaml` / `.md` / `.json`
   under `vsdd-core/patterns/` or in a build.rs `include_str!`-loaded
   asset. The lint's file-extension filter is the trust boundary; it
   does not match the emission boundary.
3. **Homoglyph.** `MDATRОN-E9999` with a Cyrillic `О` (U+041E). Passes
   any naive `starts_with("MDATRON-")` check. Defender-detection-path:
   a NFKC-normalize-then-compare pass, or restrict to `[A-Z0-9-]` ASCII
   via byte-class regex.

Better-version: lint becomes a two-pass check — (a) AST-level walk over
all `&str` literals in the crate's `cargo expand` output (catches
`format!`), (b) ASCII-byte-class regex against the reserved table
(catches homoglyphs), (c) `walkdir` over the workspace narrowed to
extension-allowlist + emission-call-site allowlist (catches carrier-swap).
Document the residual: anything constructed from operator-controlled
input at runtime is out of scope and routes to schema validation instead.

## F2 — Lint crate-list is hardcoded; sibling-crate addition silently leaks codes [Resolved-pending]

domain: red-team / dim: supply-chain / source: domain-raised /
routing: platform-engineer

Exploit path: contributor adds `mdatron-tools` (or `mdatron-mcp`, or
any future sibling) to the workspace, emits `MDATRON-E0999` from it,
the lint test (configured to scan only `mdatron-core` + `mdatron-cli`)
returns green. The reserved-code contract is silently broken on the
next release.

This is a hook-circumvention by addition rather than by edit — the
"add new crate" PR is the attack surface, not the "edit the lint" PR.
Better-version: lint discovers scan targets from the workspace
`Cargo.toml` `members = […]` list at test time and asserts the
member-set is a subset of an allowlist, OR scans every crate whose
name matches `^mdatron-` and fails-closed if a new prefix appears.
Either way, the discovery rule lives in the lint, not in operator
memory.

## F3 — Null-propagation chain laundering via attacker-controlled frontmatter [Deferred]

domain: red-team / dim: trust-boundary / source: domain-raised /
routing: software-engineer + solution-architect

Exploit path: a corpus contributor authors frontmatter where
`$self.author.affiliation.disclosure_status` is absent at the
`author` level. Pre-fix this raised `FieldNotFound`, surfacing the
gap. Post-fix the entire chain returns `Null`, and a pattern like
`$self.author.affiliation.disclosure_status != "undisclosed"` evaluates
to `Null != "undisclosed"` → which (per current comparison semantics)
likely yields `Null` → propagates → finding never fires. The
attacker hides a defect by omitting an upstream key, not by lying.

Defender-detection-path candidates: a `required()` DSL builtin that
forces a non-Null assertion at chain head; pattern-author convention
to anchor optional chains with `defined()` at each non-leaf hop;
DESIGN-MDATRON.md addendum naming Null-propagation as a documented
threat with the named mitigations. Deferred because the carve-out
drop is intentional spec; the mitigation is pattern-corpus guidance
plus a builtin, not a code reversion. Route to SA for builtin design.

## F4 — defined() drop widens the empty-string laundry channel [Resolved-pending]

domain: red-team / dim: trust-boundary / source: domain-raised /
routing: software-engineer + quality-engineer

Exploit path: a frontmatter contributor sets `dismissal_rationale: ""`
to satisfy a `defined(dismissal_rationale)` predicate that previously
caught absent rationales. Pre-fix the empty-string returned `false`
and the lint fired `VSDD-E0016` (rationale-less bypass). Post-fix
`defined("")` returns `true` and the bypass marker passes audit.

This is the bypass-marker abuse dimension explicit in the Red Team
prompt. Defender-detection-path: any pattern that today reads
`defined(x)` to mean "non-empty meaningful value" must migrate to
`defined(x) && x != ""`. Grep the corpus, surface every such site
as a Phase 4 routing target, ship the migration in the same MR as
the carve-out drop. The carve-out drop without the corpus sweep is
the contract-evasion surface.

## F5 — Schema revert weakens trust-boundary asymmetrically [Accepted]

domain: red-team / dim: supply-chain / source: domain-raised /
routing: solution-owner

`supplements_in_scope` / `supplements_applied` no longer required.
A hostile primer / domain-prompt author can omit the field. Pre-fix
the schema rejected the document. Post-fix verify is permissive
where it was once strict — every downstream consumer that branches
on the field's presence (e.g., supplement-gating logic) is now
exposed to a `None` it did not previously have to handle.

Accepted because the revert is the DESIGN-declared intent (the
required-promotion was a reactive workaround). Mitigation routes
to consumer-side: any code path that reads these fields must treat
absence as the documented default ("no supplements in scope") and
must not branch on `field.is_some()` as a security gate. SO holds
spec-contract authority on the absence-default text.

## F6 — Reserved-code rename is a supply-chain breakage masquerading as a refactor [Deferred]

domain: red-team / dim: supply-chain / source: domain-raised /
routing: platform-engineer + solution-owner

`MDATRON-E0001` / `E0002` swap meaning; any external tool pinning
to the old numbers — log aggregators, alerting rules, downstream
remediation playbooks — breaks silently. If `mdatron-core` is
imported as a library, the constant strings move under SemVer-minor
because the code-table is "internal." The wire-format invariant
(`mdatron_output_version` stays `1.0.0`) papers over a real
break in consumer integrations.

Deferred: SO authority on whether code-number changes are a wire-
format-major signal. Mitigation candidates: bump
`mdatron_output_version` to `1.1.0` with a "code-table revision"
note in the changelog; emit `MDATRON-W00XX` warnings for one release
when the old code is encountered in a corpus-replay; document the
rename in DESIGN-MDATRON.md as a one-time correction.

## Closing note

Validator-pair routing: F1, F2, F4 route to Security on a separate
cluster per the adversarial-pair-separation invariant. F3 / F6 carry
Raise-to-SO surface. No findings hallucinated.
