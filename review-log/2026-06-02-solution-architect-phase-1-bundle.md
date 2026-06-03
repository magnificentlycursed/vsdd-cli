---
schema_class: review-entry
schema_version: 1.0.0
review_number: 3
date: 2026-06-02
phase: phase-3
scope: >-
  Crosslink #12 / mdatron commits ebf6320 + 13429b9 — three bundled changes:
  (1) reserved-code drift fix (E0001/E0002 rename + new ranges E0050-E0059 /
  E0070-E0079 / E0080-E0089 + new mdatron-core/src/codes.rs + cross-file lint),
  (2) DSL Field-on-Object-missing-key returns Ok(Null) (M4 PE F1 root cause),
  (3) defined() empty-string carve-out drop, plus vsdd-cli schema reverts to
  drop supplements_in_scope / supplements_applied from required. Files
  inspected: mdatron-core/src/codes.rs, mdatron-core/src/verify.rs (L15, 240-251,
  273-285), mdatron-core/src/dsl/expr.rs (L177-203, L221-234, L320-327, L592-607),
  mdatron-core/tests/phase_1_contracts.rs, mdatron/DESIGN-MDATRON.md (L506-514,
  L584, L592, L814), vsdd-core/schemas/phase-primer.json,
  vsdd-core/schemas/domain-prompt.json.
lens: >-
  Solution Architect primary (hard-to-undo decisions; module-surface design;
  reserved-range strategy; DSL composition with every()/some(); lint evasions;
  schema-revert future failure modes). Sanity Check baseline — rubber-ducking
  the bundle's spec-vs-code coherence and naming where one of the three changes
  contradicts the artifact it claims to align with.
source: domain-raised
session_note: >-
  Cold-session single-domain composition (SA-primary + Sanity-Check baseline).
  No prior session memory. Read fresh: vsdd-phase-3.md,
  vsdd-domain-solution-architect.md, vsdd-domain-sanity-check.md,
  review-entry.json, DESIGN.md (Phase 1 bundle). Then mechanically inspected
  the diff surfaces named in the operator brief. Sycophancy compensation:
  Claude authored every line; I held findings to file:line citations.
model: claude-opus-4-7
execution_method: >-
  cold-session sub-agent dispatched from main session; primer + SA + sanity-check
  prompts + review-entry schema + DESIGN.md (Phase 1) loaded fresh. No prior
  same-domain review of this bundle exists.
sycophancy_compensation: >-
  The cheap path is "looks clean — typed contract, unit tests, lint, design
  doc, all three concerns landed coherently." I pressure-tested where the bundle
  ships internal coherence but breaks external coherence with the spec it
  claims to amend.
filename_note: >-
  phase-1-bundle slug per the DESIGN.md filename convention
  (phase-1-codes-and-dsl). Plural-domain disambiguation not needed; prior
  SA reviews used different scope slugs.
supplements_loaded: []
---

# Solution Architect — Phase 3 review of crosslink #12 bundle

## SA-F1 — Spec-vs-code reserved-range collision (CRITICAL, dim 5 + 5)

`mdatron-core/src/codes.rs:43` reserves `E0050-E0059` for frontmatter schema
validation and `E0070-E0079` for IO failures. The spec the codes are claimed
to align with — `DESIGN-MDATRON.md` — still emits **conflicting** assignments
that were never amended:

- `DESIGN-MDATRON.md:584` — `MDATRON-E0050: code-unknown` (explain command)
- `DESIGN-MDATRON.md:814` — `MDATRON-E0070: no-git-repo` (preflight)
- `DESIGN-MDATRON.md:592` — `MDATRON-E0060: no-config-file` (sits in the
  E0060-E0069 gap the new table leaves unreserved)

`codes.rs:34-42` doc-comments the "as amended" ranges; the amendment never
landed in the .md spec. The DESIGN.md (Phase 1) at line 70-78 explicitly
declares the spec amendment as required ("Spec amendments needed in
DESIGN-MDATRON.md:506-514"), but a grep of DESIGN-MDATRON.md shows the table
at 506-514 unchanged. The bundle is **internally consistent** (code + tests +
codes.rs agree) but **externally incoherent** with the artifact it cites as
its authority. Hard-to-undo dimension: future v0.2 will allocate a real
`code-unknown` and `no-git-repo` emission, hitting either (a) the now-taken
slot or (b) a fresh slot, in which case the spec's L584/L814 prose silently
references nothing. Routes Raise-to-SO: Phase 1c was supposed to land both
the codes module AND the DESIGN-MDATRON spec edit; only the former shipped.

## SA-F2 — Lint scans .rs only; spec drift can re-enter (HIGH, dim 5)

`phase_1_contracts.rs:46-50` walks `mdatron-core/src` and `mdatron-cli/src`
exclusively. Any `MDATRON-Exxxx` literal in a .md file, .yaml fixture,
explain-page, README, or `.mdatron/` config is invisible to the lint. SA-F1
is the live example: the DESIGN-MDATRON.md collisions exist precisely
because the lint scans the wrong directory tree. The DESIGN.md L86 acceptance
criterion was "Every emitted MDATRON-Exxxx code in the codebase maps to a
reserved range" — "codebase" silently became "src/". Recommend the lint scan
also include `*.md` + `.mdatron/` fixtures with an allowlist for the spec's
own enumeration tables.

## SA-F3 — Lint string-extractor false-positive surface (MEDIUM, dim 2)

`phase_1_contracts.rs:226-241` `extract_mdatron_code_literals` greps for the
substring "MDATRON-" anywhere — code, string, comment, doc-comment — and
extracts the alphanumeric+hyphen run. A doc comment like `//! see MDATRON-E0060`
(intentional documentation of a not-yet-implemented code, or even a TODO
referencing a planned code) will fail the lint with no escape hatch beyond
"add it to a reserved range." There is no `// codes-lint:ignore` mechanism.
Evasion in the other direction: a code constructed at runtime
(`format!("MDATRON-E{:04}", n)`) is invisible to the lint — the codes.rs
test at line 92-93 uses exactly this trick to dodge the namespace-separation
lint. Same trick dodges this lint. Maintainability: future operator who
needs to discuss a code in a doc-comment hits the lint with no workaround.

## SA-F4 — Field-symmetry composes correctly with every()/some(); one gap (LOW, dim 2)

`expr.rs:273-294` — `Every` / `Some_` apply `expect_array` to the collection
expression. After the Field-symmetry fix, `every(s in $self.optional, ...)`
where `optional` is absent now evaluates the collection to `Value::Null`,
which `expect_array` rejects with `TypeMismatch`. The DESIGN.md L114
implied "missing field flows as Null through every()/some()" — it does NOT;
it flows as a TypeMismatch error from `expect_array`. The Red Gate test
`every_over_missing_optional_field_does_not_panic` named in DESIGN.md L224
is **not present** in `phase_1_contracts.rs` (only the unit-level
`field_on_object_missing_key_returns_null` made it in). The downstream
ergonomic the schema-revert relies on — patterns that quantify over an
optional field — was asserted but not tested. Recommend either: relaxing
`expect_array` to accept Null-as-empty, or shipping the missing Red Gate
test so the failure mode is named.

## SA-F5 — `EvalError::FieldNotFound` variant kept but unreachable (LOW, dim 4)

`expr.rs:183` retains the `FieldNotFound { field, on }` variant; the only
production emission site (the Object-missing-key branch) was removed at
`expr.rs:227`. DESIGN.md L234 explicitly named this: "the
`EvalError::FieldNotFound` variant is unused (can be removed or retained
for the Object/non-Object case)." It was retained without a use site —
dead variant. Future-developer reading the enum has no way to know
whether this is reserved-for-future-use or actually unreachable. Either
delete it or add `#[doc = "Reserved; not currently emitted"]`.

## SA-F6 — Schema revert leaves a future-failure mode unnamed (LOW, dim 5)

`phase-primer.json` + `domain-prompt.json` reverted `supplements_in_scope` /
`supplements_applied` to optional. The justification (DESIGN.md L116-119)
is "DSL no longer crashes on absent fields." True for current corpus, but
the revert removes the schema-level signal that a primer/prompt author
**considered** supplements. A primer authored post-revert with no
`supplements_in_scope` is now ambiguous between "no supplements apply" and
"author forgot to declare." A `defined()`-based pattern can no longer tell
these apart since `defined($self.supplements_in_scope)` is `false` in both
cases. The reactive tightening was load-bearing for that distinction.
Recommend a pattern that catches "primer with no `supplements_in_scope`
field at all" as a `MDATRON-W01xx` warning (not error), preserving the
revert while restoring the missing signal.

## Dispositions

- SA-F1: **Resolved-pending** — route to Phase 4 → DESIGN-MDATRON.md amend
- SA-F2: **Resolved-pending** — Phase 4 lint-scope extension
- SA-F3: **Deferred** — escape hatch + runtime-construction evasion
- SA-F4: **Resolved-pending** — missing Red Gate test or expect_array relax
- SA-F5: **Resolved-pending** — delete or doc the variant
- SA-F6: **Accepted** — name the gap in DESIGN.md; revisit if it bites
