---
schema_class: review-entry
schema_version: 1.0.0
review_number: 3
date: 2026-06-02
phase: phase-3
scope: >-
  vsdd-cli crosslink #12 Phase 1 bundle (codes-and-dsl). TW lens over
  docs/refactor/phase-1-codes-and-dsl/DESIGN.md prose + DESIGN-MDATRON.md
  reserved-codes table delta + commit-body convention surfaces.
lens: >-
  Technical Writer — vocabulary registry conformance + load-bearing-term
  discipline + operator-prose ergonomics. Weighting Consistency (5) >
  Maintainability (3) > Usability (2) > Edge (1) > Attacker (0).
source: director-raised
session_note: >-
  Cold-session adversarial review of the three-change bundle (reserved-code
  drift fix, DSL Field-symmetry, defined() carve-out drop). TW lens is the
  prose coherence + vocabulary guardrail; DR is the validator pair.
model: claude-opus-4-7
execution_method: >-
  Cold-session sub-agent, worktree-no-memory, Phase 3 primer + TW + sanity-check
  prompts loaded fresh.
sycophancy_compensation: >-
  All prose under review authored by Claude; TW bias is to defend coinages as
  load-bearing. Compensation — every rename / retire / new-coinage finding
  cites a structural binding (CLI output / catalog / event variant / schema
  field) or its absence, per the 2026-06-01 TW filter.
supplements_loaded: []
---

# Technical Writer Phase 3 Review — Crosslink #12 Bundle

## TW-F1 — "frontmatter schema validation failures" vs "schema load failures": three-way split is sound, table labels conflate

**Classification:** resolved-pending. **Routing:** TW (self).
**Lens:** Consistency / Dim 2 (vocabulary registry).

The amendment introduces three sibling classes against `DESIGN-MDATRON.md:506-514`:
parsing failures (E0001-E0009, pre-schema YAML-malformed), schema load failures
(E0040-E0049, the schema file itself is broken), and the new schema validation
failures (E0050-E0059, document violates a loaded valid schema). The carve-out
is genuinely distinct — parse vs. load vs. validate is canonical. But the new
row's prose neighborhood shares "schema" without naming *whose* schema. A cold
reader scanning the table sees "Schema load failures" → "Frontmatter schema
validation failures" and parses the second as a sub-case of the first.

Better: amend row labels to name the artifact — e.g., "Schema-file load
failures" (E0040-E0049) and "Document-vs-schema validation failures"
(E0050-E0059). Same split; prose now disambiguates the target.

## TW-F2 — "reactive fix" is unregistered shorthand

**Classification:** resolved-pending. **Routing:** TW (self).
**Lens:** Consistency / Dim 2 + Maintainability.

`binary-first-plan.md:117` and `DESIGN.md:116` are the corpus's only two uses
of "reactive" — not a methodology term. Either coin it (recurrence evidence
+ glossary entry) or rewrite descriptively: "the two schema-tightenings
landed to silence DSL crashes on absent optional fields, not because the
spec requires those fields, can be reverted." This is the
scaffolding-term-proliferation failure mode in the TW prompt: shorthand that
hasn't earned registry entry.

## TW-F3 — DESIGN.md "Why" sub-sections asymmetric across the three changes

**Classification:** resolved-pending. **Routing:** TW (self).
**Lens:** Consistency / Dim 5.

The three change sub-sections are uneven: Reserved-code drift gets table +
assertions but *no* "Why"; Field-symmetry gets "Why this matters"; `defined()`
gets "Why." A cold reader landing in § Behavioral contracts directly sees
the mechanical change without the rationale. The Reserved-code-drift "Why"
exists in `2026-06-02-solution-architect-binary-first-refactor.md:73` —
wire-format-break stakes post-1.0. Consolidate into DESIGN.md as a
one-paragraph "Why this matters" under § Reserved-code drift fix.

## TW-F4 — "Code-allocation lint" — "lint" is overloaded; "code-allocation check" is more accurate

**Classification:** deferred. **Routing:** Raise-to-SO.
**Lens:** Consistency / Dim 2.

`MDATRON-L0001` — `L0099` is the reserved range for "Engine-level lints" —
adopter-facing code-smell findings. The new mechanism (DESIGN.md:84,
codes.rs:6) is not that — it is a build-time / test-time enforcement that
every emitted `MDATRON-Exxxx` string matches a reserved range. It never
produces an `L####` finding; it fails the build or a test. Two paths:

1. Rename to "code-allocation check" or "reserved-range enforcement test" —
   discharges the L-code conflation. Term ships in `binary-first-plan.md:84`
   + `codes.rs:6`; rename has propagation cost but is contained.
2. Keep "lint" and broaden the methodology gloss. Cheaper edit, broader drift.

SO call needed. Recommendation: path (1) — the alternative dilutes the
reserved-`L####` semantic.

## TW-F5 — `OperatorDirectiveApplied`-in-commit-body convention has no published anchor

**Classification:** deferred. **Routing:** Raise-to-SO + DR.
**Lens:** Usability / Dim 3 (discoverability) + Consistency.

`binary-first-plan.md:23,40` cites the convention: directives "land as
`OperatorDirectiveApplied` events" at the point of operator-input. SO-F1
(this date) treats it as load-bearing for spec amendments. But README.md
mentions the variant only as a bare name (L332) and a specific instance
(L718) — never the convention itself. A cold operator inspecting
`events.jsonl` cannot discover *where* the directive text lives (commit body?
PR description? `.vsdd/`?).

Better: README § "Event variants" or methodology.md gains one sentence —
"`OperatorDirectiveApplied` events carry the directive text in the emitting
commit's body (or originating PR description for crosslink flows); the
variant + commit SHA is the audit-trail anchor."

## TW-F6 — E0001 ↔ E0050 swap leaves explain-page anchors stale

**Classification:** resolved-pending. **Routing:** TW + DR.
**Lens:** Consistency / Dim 5.

The two coinings ("frontmatter-schema-violation", "frontmatter-parse-failed")
stay stable as strings — only their code identifiers swap. Vocabulary
registry unaffected. But per `DESIGN-MDATRON.md:518-520`, every code has a
per-code explain page at `docs/error-codes/<code>.md`. If
`MDATRON-E0001.md` / `MDATRON-E0002.md` weren't moved in lockstep, the
prose-vs-code anchor inverts silently. DESIGN.md does not list this.

Better: add an "Explain-page migration" bullet under § Reserved-code drift
fix observable assertions — old `E0001.md` content moves to `E0050.md`;
`E0002.md` content moves to `E0001.md`; old paths 404 or carry deprecation
pointers.

---

## Disposition summary

| ID | Classification | Routing | Lens |
|---|---|---|---|
| TW-F1 | resolved-pending | TW | Consistency |
| TW-F2 | resolved-pending | TW | Consistency |
| TW-F3 | resolved-pending | TW | Consistency |
| TW-F4 | deferred | Raise-to-SO | Consistency |
| TW-F5 | deferred | Raise-to-SO + DR | Usability |
| TW-F6 | resolved-pending | TW + DR | Consistency |

No findings hallucinated. Sanity-check on TW-F4 + TW-F5 — both surface a
registry/methodology decision TW cannot terminate alone; routing is sound,
not abdication.
