---
schema_class: review-entry
schema_version: 1.0.0
review_number: 6
date: 2026-06-02
phase: phase-3
scope: >-
  Crosslink #12 bundle from a Data Engineer lens. Subjects: the schema reverts
  in vsdd-core/schemas/phase-primer.json (supplements_in_scope -> optional) and
  vsdd-core/schemas/domain-prompt.json (supplements_applied -> optional), the
  reserved-code drift fix in mdatron-core (codes.rs / verify.rs / DESIGN-MDATRON.md
  spec table at 506-514), and the implicit JSON output object emitted by
  `mdatron verify --json` per docs/refactor/phase-0-output-format/DESIGN.md.
  Also: docs/refactor/phase-1-codes-and-dsl/DESIGN.md.
lens: >-
  Data Engineer (schema design + data integrity + evolution discipline +
  single-source-of-truth for the reserved-codes table + data-lineage Schema ->
  Verify -> Output JSON). Five-lens weighting: Maintainability 5, Consistency 5,
  Edge-cases 4, Attacker 1, Usability 2. Cold-session reviewer; sycophancy
  compensation active (DE was operator-requested specifically because schema
  work is load-bearing here, which is the exact prompt-shape under which a
  reviewer is most tempted to validate the request rather than the artifact).
source: director-raised
session_note: >-
  Cold-session per Phase 3 primer; no prior-session memory of this bundle.
  Read fresh: phase-3 primer, data-engineer + sanity-check domain prompts,
  review-entry schema, Phase 1 DESIGN.md, both reverted schemas, Phase 0
  output-format DESIGN.md (lines 80-180, 295-345). Cross-checked against
  prior phase-1-bundle review entries (SA-F1, SA-F6, QE-F5) to avoid
  duplicating their findings; DE findings below are the schema-discipline
  lens not raised by those reviewers.
model: claude-opus-4-7
execution_method: claude-code-cli-task-tool-cold-session
sycophancy_compensation: >-
  Operator named the schema work as the reason DE was invited. The cheap path
  is "yes, schema reverts look clean, semver discipline holds." Pressure-tested
  the inverse: does the revert silently lose a data-integrity signal, and does
  the schema-class dispatch model survive an absent optional field where the
  Phase 1 DSL fix promises symmetry?
---

# Data Engineer — Phase 3 review of crosslink #12 bundle

## DE-F1 — Absence-vs-empty-array distinction is now ambiguous and unrecoverable (Consistency, Maintainability, dim 1+6)
**Classification:** resolved-pending **Routing:** SO via Raise-to-SO **Source:** domain-raised

Post-revert, `supplements_in_scope: []` and absence-of-field both validate.
For an analytic process answering "how many primers declare zero supplements
vs how many forgot to declare," these are not the same answer. The schema
no longer carries the distinction. Per the DE domain prompt sycophancy
failure mode "data validation at write-time skipped because 'the source is
trusted'," the trust here is implicit: future primer authors are trusted to
distinguish absence-as-intent from absence-as-omission, with no schema
support. SA-F6 named this from the architectural surface; the DE finding
is the data-integrity restatement — once a primer is committed without the
field, the lineage is permanently ambiguous, because nothing in the
artifact records the author's intent. Corrective pattern: either keep the
field required-with-`[]`-allowed (the original schema-tightening was load-
bearing for this exact reason), OR add a sibling `supplements_declared:
boolean` whose presence asserts "author considered." The first is cheaper.

## DE-F2 — JSON Schema 2020-12 hygiene: properties block is correct; one missing convention (Consistency, dim 1)
**Classification:** dismissed (with note) **Routing:** DE-internal **Source:** domain-raised

Both reverts correctly leave `supplements_in_scope` / `supplements_applied`
in the `properties` block — draft 2020-12 idiom is "describe every known
field; required vs optional is a separate concern." Good. Missing convention:
neither field carries a `description` explaining the absence-vs-empty
distinction (or lack thereof, per DE-F1). For schema as documentation, the
optional fields are now the ones that most need a description string —
because absence is now valid, the schema is the only place a future author
learns what absence means. One-line `"description": "Empty array and absent
both mean 'no supplements'; see DE-F1 routing for the intent-signal gap"`
on each field. Dismissed-as-blocker; flagged-as-schema-debt.

## DE-F3 — `mdatron_output_version` semver discipline: Phase 1 rename is wire-stable but the contract is ambiguous about code-numbering changes (Consistency, dim 1)
**Classification:** resolved-pending **Routing:** SO **Source:** domain-raised

Phase 1 DESIGN.md line 87 declares `mdatron_output_version` stays `1.0.0`
because "adding a new code numeric within reserved ranges is an additive
change, not a wire-format break." The Phase 0 output-format DESIGN.md at
326-328 codifies the versioning rule: "Add an optional field: no version
bump… Remove a field, add a required field, change a field's type or
semantics: bump." Renaming a code's numeric value is neither addition nor
removal of a field — it is a change to the **value domain** of an existing
field (`code: "MDATRON-E0001"` now means parse-failed where it used to mean
schema-violation). A consumer who pinned on the numeric value to drive
remediation logic now sees a behavior change at the same wire version.
The DESIGN.md called this out implicitly ("some emitted codes change
number") but did not extend the versioning rule to value-domain shifts.
SO-F (prior solution-owner review) flagged the same concern with a
1.0.0 -> 1.1.0 disposition. DE concurs: either tick to 1.1.0 with a
CHANGELOG entry naming the rename, OR amend the Phase 0 versioning rule
to explicitly carve out reserved-range internal reshuffles as
"transparent." Pick one; current state is undeclared.

## DE-F4 — Single-source-of-truth: codes.rs vs DESIGN-MDATRON.md table is drift-prone by construction (Maintainability, dim 1+6)
**Classification:** accepted-with-remediation **Routing:** SE + SO **Source:** domain-raised

`codes.rs` is the lint authority (executable; CI-enforced). DESIGN-MDATRON.md
:506-514 is the prose authority (hand-maintained). SA-F1 already named that
the spec edits never landed for E0050 / E0070 / E0080 — that's the data-
integrity loss in flight today. The deeper DE concern: two authorities for
the same data, no generation step linking them. Per the DE domain prompt
dim 1 (schema discipline + semver), the canonical source must be named;
drift is a data-integrity failure mode. Corrective pattern: declare codes.rs
the canonical source, mark the .md table as derived, generate the .md
fragment from codes.rs (or assert in CI that the .md table matches a known
projection of codes.rs ranges). This is bigger than the milestone — but
naming the canonical source in DESIGN-MDATRON.md ("the rust constants in
mdatron-core/src/codes.rs are canonical; this table is a derived view")
is a one-line edit that prevents the next reviewer from raising SA-F1
again. Acceptance criterion: any future code addition must update codes.rs;
the .md update is documentation-of-record, not the contract.

## DE-F5 — Schema-class dispatch on absent optional field: the path generator's behavior is undeclared at the data layer (Edge-cases, dim 1+4)
**Classification:** deferred **Routing:** SE (data-flow audit) **Source:** domain-raised

Operator brief flagged: when a frontmatter has `schema_class: phase-primer`
and `supplements_in_scope` is absent, does the JSON path generator handle
the missing-field case correctly? The Phase 1 DSL fix promises
`Field-on-Object-missing-key` returns `Null`. SA-F4 already named that
`expect_array` rejects Null with TypeMismatch, breaking `every()` /
`some()` composition. The DE restatement: the schema-class dispatch model
treats schemas as a registry keyed by `schema_class`; the data lineage
flowing through dispatch -> validate-against-schema -> emit-finding has
three layers (1) schema-class lookup, (2) JSON Schema validation of the
frontmatter against that schema, (3) DSL pattern evaluation against the
same frontmatter. Step 2 now accepts absent `supplements_in_scope`; step
3 then evaluates patterns like `every(s in $self.supplements_in_scope,
...)` against absent-field-as-Null, which per SA-F4 still errors. The
data integrity claim "schema permits absence; pipeline handles absence"
is **partially** true: validation permits, evaluation fails. No test in
phase_1_contracts.rs locks the full lineage. Defer to SE for an end-to-end
test that runs an actual `phase-primer` frontmatter with the field
omitted through the full verify pipeline.

## DE-F6 — Data lineage audit trail: three layers, no end-to-end witness (Maintainability, dim 1+8)
**Classification:** accepted **Routing:** Phase 5 surface **Source:** domain-raised

The data flows: (1) JSON Schema in vsdd-core/schemas/*.json, (2) `codes.rs`
reserved-range enforcement, (3) Findings emission shape (Phase 0 output-format
contract). Each layer has its own test surface (the schema files have JSON
Schema $schema validation; codes.rs has 8 unit tests; output shape has
implicit fixtures). What is missing per DE dim 8 (append-only patterns +
audit trail) is a single observable that proves the layers agree on a
known input. A worked example: feed a synthetic primer-with-known-defect
through the verify pipeline, capture the JSON output, assert each layer's
contribution (schema validation passed/failed at layer 1; pattern
evaluation emitted code at layer 2; output-shape conformance at layer 3).
Today the layers are tested independently; integration is tested only by
running mdatron against the vsdd corpus (QE-F5 named the absence of an
automated corpus regression). Acceptance: one golden fixture per
schema-class flowing through all three layers, with the JSON output
diff'd against a checked-in expected. Phase 5 surface; non-blocking
for the milestone.

# Cross-finding cohesion

DE-F1 + DE-F2 cluster on the revert (data-integrity at the schema layer).
DE-F3 + DE-F4 cluster on semver + single-source-of-truth for the code-table
(data-integrity at the lint + wire-format layer). DE-F5 + DE-F6 cluster on
end-to-end lineage (data-integrity across layers). DE-F1 + SA-F6 are the
same finding from two lenses; DE phrases it as data integrity, SA as
hard-to-undo architectural drift. DE-F4 reframes SA-F1: the immediate fix
(land the .md edit) closes the bug; the structural fix (name canonical
source) closes the bug class.

# Dispositions

- DE-F1: **Resolved-pending** — Raise-to-SO on schema-vs-intent signal
- DE-F2: **Dismissed** (with schema-debt note)
- DE-F3: **Resolved-pending** — Raise-to-SO on output-version semver rule
- DE-F4: **Accepted-with-remediation** — name canonical source in DESIGN-MDATRON.md
- DE-F5: **Deferred** — SE end-to-end dispatch test
- DE-F6: **Accepted** — Phase 5 lineage-audit fixture

6 findings: 2 resolved-pending (Raise-to-SO), 1 accepted-with-remediation,
2 deferred/accepted (Phase 5), 1 dismissed. No hallucinated findings.
Bundle's internal data integrity is sound; external integrity (schema
intent, wire-version semantics, cross-authority drift) carries the load.
