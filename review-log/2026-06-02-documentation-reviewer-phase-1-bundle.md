---
schema_class: review-entry
schema_version: 1.0.0
review_number: 4
date: 2026-06-02
phase: phase-3
scope: >-
  Crosslink #12 Phase 1 bundle as it stands in tree on 2026-06-02. Subject:
  docs/refactor/phase-1-codes-and-dsl/DESIGN.md (the consolidated 1a/1b/1c
  design); DESIGN-MDATRON.md:116-117 + 500-520 (the reserved-codes table the
  DESIGN claims to amend); mdatron-core/src/codes.rs (is_reserved_mdatron_code
  + doctring + tests); mdatron-core/tests/phase_1_contracts.rs:41-72
  (all_emitted_codes_are_reserved static lint); commit 86bb890 body
  (operator-described as ~50-line prose for next-reader bisect).
lens: >-
  DR cold-read in adversarial-cluster mode. Five DR dims weighted: cold-context
  discoverability (1), cross-reference resolution (2), stale-claim suspicion
  (6), naming-discipline cold-read (7), prose-surface composition discipline
  (8). Stance: an adopter reading DESIGN-MDATRON.md today + running mdatron
  today should reach a coherent model without external context. Vocabulary
  discipline applied to the three new range-class labels coined in the DESIGN.
source: director-raised
session_note: >-
  Cold single-domain single-session DR pass. No prior-cycle memory of this
  bundle. Operator named six lens-prompts; each addressed in the findings.
  Note: the operator-cited commit SHA "ebf6320" does not exist in this
  repo; the commit matching the described body (~50 lines, crosslink #12,
  codes + DSL + defined()) is 86bb890. Treated 86bb890 as the subject.
model: claude-opus-4-7
execution_method: >-
  inline main session (single-agent DR with sanity-check baseline; no worktree
  isolation; under-800-word opinion shape per the operator cap)
sycophancy_compensation: >-
  DR bias toward "fewer load-bearing terms" pressure-tested against operator's
  prompt explicitly asking whether the new labels are noise — answered both
  ways and named the call.
---

# Findings

## F1 — Spec-vs-impl contradiction is live on main (Resolved-pending, raise to SO)

**Dim:** stale-claim suspicion + cold-context discoverability.

DESIGN.md:70-78 says "Spec amendments needed in DESIGN-MDATRON.md:506-514." Cold-read of DESIGN-MDATRON.md:500-516: the table is unamended. The implementation already ships the new ranges — `codes.rs:43` matches `E0050-E0059 | E0070-E0079 | E0080-E0089`; the doctring at `codes.rs:14-21` documents them as "(v0.1.x)"; the static lint `all_emitted_codes_are_reserved` (phase_1_contracts.rs:41-72) is green on the new shape.

An adopter reading DESIGN-MDATRON.md sees the OLD table and would in good faith file an issue when mdatron emits `MDATRON-E0050`. Classic "doc says X, code does Y." Corrective: amend the table in the same commit as the rename; if deferred, the DESIGN must name the actor + phase, not "needed" passive-voice.

## F2 — Three new range-class labels are load-bearing — keep but tighten (Resolved-pending)

**Dim:** naming-discipline cold-read.

Operator-prompt: noise or load-bearing? Test against `E0040-E0049 = Schema load failures`. Schema-load = the schema file itself is broken at config-load time; schema-validation = a document's frontmatter violates a well-formed schema at verify time. Real distinction; different user-time, different failed-thing. Same for IO (verify-pipeline filesystem, distinct from E0010 path-confinement) and pipeline-orchestration (the orchestrator wrapping verify, distinct from any single stage). Keep.

Tighten: "frontmatter schema validation failures" parallels "frontmatter parsing failures" at E0001-E0009 but the parallelism is invisible. The DESIGN should name the parse-vs-validate cut in one sentence under the table; today the cold reader must triangulate from `codes.rs` comments.

## F3 — `is_reserved_mdatron_code()` naming is clean (Hallucinated)

"Reserved" matches DESIGN-MDATRON.md:500's "Reserved mdatron codes" header; prefix is namespace-explicit; bool predicate shape; doctring traces the table. Cold-reader pass.

## F4 — DESIGN sub-section template is inconsistent (Resolved-pending)

**Dim:** prose-surface composition discipline.

The three Phase 1a sub-sections drift. `defined()` adds a "Why" (155-160); `Field`-access adds "Why this matters" (110) + "Side effect" (116); the reserved-code section has neither. Cold-read: when only one sibling explains its rationale, the reader infers the others have none. Pick one template — {Current / Change / Why / Side effects / Observable assertions} — and apply across all three.

## F5 — Commit 86bb890 body is discoverable; headline buries the lede (Accepted)

**Dim:** cold-context discoverability for the bisector.

Body is well-structured: three numbered sections matching DESIGN sub-sections, file paths + line ranges, cross-refs to M4 PE F1 + Phase 4 disposition #10. Six-months-later bisector reaches the right model. Headline "phase-1a/1b/1c (crosslink #12): codes + DSL Field + defined() design" leads with phase numbering; "design" is misleading because the lint test lands too. Accept this commit; future pattern: lead with verb + artifact, not phase.

(Note: operator named SHA `ebf6320` — it does not exist in tree; subject is 86bb890.)

## F6 — Reserved-code lint error message is operator-hostile (Resolved-pending, route to SE)

**Dim:** cold-context discoverability for the contributor.

`phase_1_contracts.rs:67-71` panics with `violations: {violations:?}` — that Debug-prints a `Vec<(PathBuf, String)>`. A contributor who writes `MDATRON-E0099` in a doc-comment sees a file path + code, no line number. Fix: track line numbers in `extract_mdatron_code_literals` (split on `\n`; emit `(PathBuf, usize, String)`); format as `path:line: code`. That matches the rustc-shaped diagnostic style mdatron uses elsewhere.

# Cross-finding coherence

F1 + F4 are the same defect class at different scales: the DESIGN doc did not finish its own composition discipline before code landed. F2 + F6 each name a cold-reader gap the solo-author cycle did not surface. F5 is lowest-priority; F3 is a clean rubber-duck.
