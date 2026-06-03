---
schema_class: review-entry
schema_version: 1.0.0
review_number: 11
date: 2026-06-02
phase: phase-3
scope: >-
  Crosslink #12 Phase 1 bundle per docs/refactor/phase-1-codes-and-dsl/DESIGN.md
  — reserved-code rename (E0001/E0002/E0050; reservations E0070/E0080),
  DSL Field-access symmetry (Field-on-Object-missing-key returns Null),
  defined() empty-string carve-out drop, plus revert of two reactive schema
  tightenings (phase-primer.json supplements_in_scope, domain-prompt.json
  supplements_applied).
lens: >-
  UX cold-read. Operator's hands-on-keyboard experience hitting a
  rustc-shaped diagnostic on stderr; adopter authoring a DSL rule against
  the new Field / defined() semantics; adopter authoring a new domain prompt
  against the loosened Layer 1 schema. Discoverability + recovery weighted
  over aesthetics.
source: director-raised
session_note: >-
  Cold single-agent UX pass. Six operator-named prompts: code rename
  discoverability, diagnostic line-shape readability, defined()-drop silent
  semantic change, Field-symmetry silent-pass debugging, internal-only
  is_reserved_mdatron_code() (no UX surface), Layer 1 schema-tightening
  revert + omitted-field code-review discoverability.
model: claude-opus-4-7
execution_method: >-
  inline main session (single-domain cold-session); under-800-word cap.
sycophancy_compensation: >-
  UX failure mode is "spec is complete, code is correct, operator still
  fails." Pressure-tested each prompt against "would a cold operator
  notice / recover" — answered honestly where the answer is "no, and the
  recovery is doc-reading," which the domain prompt names as itself the gap.
supplements_loaded: []
---

# Findings

## F1 — Code rename has no operator-discoverable bridge (Resolved-pending, raise to SO)

**Dim:** operator-task tracing + recovery path ergonomics.

Operator saw `MDATRON-E0001: frontmatter-schema-violation` yesterday; today sees `MDATRON-E0050: frontmatter-schema-violation` for the same finding. The message body is identical. The operator's natural recovery — search prior tickets / chat logs / runbooks for `MDATRON-E0001` — surfaces correct-but-stale guidance. There is no in-toolkit bridge: `mdatron explain MDATRON-E0001` either errors ("unknown code") or silently routes to the new code with no rename trace. CHANGELOG / release-notes mention is necessary but insufficient — operators hit the diagnostic before reading notes.

Corrective: `mdatron explain MDATRON-E0001` should emit `note: MDATRON-E0001 was renamed to MDATRON-E0050 in v0.1.x; see CHANGELOG`. One row in a rename-table in codes.rs; one branch in explain. Discoverable at the failure site, not via doc-search.

## F2 — Rustc-shaped output: note-after-location is the right order, but `=` glyph buries the message (Resolved-pending)

**Dim:** error message specificity + consistency across surfaces.

Cold scan of the three-line shape:

```
error[MDATRON-E0050]: frontmatter-schema-violation
  --> path/to/file.md:1
   = note: <message>
```

Line 1 carries label + code + slug — operator reads it as "an error of this class." Line 2 is path:line — operator's eye jumps here for the where. Line 3 carries the actual problem. Order matches rustc; that is correct. The defect is the `= note:` glyph: rustc reserves `= note:` for supplementary context and uses unprefixed wrapped prose for the primary message. Here, the message IS the primary; `note:` mis-signals it as auxiliary. Operators trained on rustc will skim past it.

Corrective: drop `= note:` for the primary message; indent two spaces and let the prose carry. Reserve `= note:` / `= help:` for true supplementary lines (e.g., the rename bridge in F1).

## F3 — defined() carve-out drop is a silent-semantic-change for adopters (Resolved-pending, raise to SO)

**Dim:** affordance discoverability + operator-time-binding visibility.

Adopter with `defined($self.title)` today: rule fires when title is `""`. After the bundle: rule no longer fires for empty-string. No diagnostic. No deprecation cycle. The rule still validates, still runs, returns a different verdict. Cold-read discoverability: zero — the operator notices when downstream content slips through verify.

The DESIGN names "$self.field != \"\"" as the one-character replacement, but the operator must KNOW to look. Minimum acceptable: CHANGELOG section titled "Behavior change: defined() on empty string" + a migration grep snippet. Better: a one-shot lint pass on adopter pattern files at first run after upgrade, flagging `defined()` call sites with a "review-required" advisory. Best (Raise to SO): deprecation cycle — one minor version emitting `MDATRON-W00xx: defined() empty-string semantics changing in next release`.

## F4 — Field-symmetry silent-pass is a debugging-tax (Deferred)

**Dim:** error message specificity + recovery path ergonomics.

Before: rule referencing `$self.missing_field` crashed loudly with `FieldNotFound`. The operator knew where to look. After: `Null` propagates; the rule silently passes (or silently catches the wrong thing). The operator debugging "why isn't my rule catching X" must know the optional-field semantics to even form the hypothesis.

This is the intended trade-off — the DESIGN names the asymmetry-with-Null branch as the bug — and the corrective is the same as F3: documentation. Defer the implementation hardening but require: a Patterns section in DSL docs titled "Optional fields and Null propagation," and at least one worked example in the explain corpus where Null-propagation is the answer to a "why didn't my rule fire" question. Don't ship the symmetry without the discoverability text alongside.

## F5 — is_reserved_mdatron_code() — no operator surface (Hallucinated)

Internal lint predicate; contributor-facing only; F6 in the DR review already covers the contributor-facing diagnostic-shape gap. No UX finding.

## F6 — Schema-tightening revert: omitted supplements_applied is invisible in code review (Resolved-pending, raise to SO)

**Dim:** affordance discoverability.

Layer 1 schema now allows `supplements_applied` to be absent. An adopter authoring a new domain prompt who omits it gets no schema error. A reviewer scanning the diff sees a clean YAML block with no missing-field marker. The omission is invisible — the field's absence is semantically "no supplements applied," but the absence-vs-empty-list distinction is reader-discretion.

Corrective: require empty list `supplements_applied: []` as a convention even though schema allows absence; lint or template-scaffold the empty list into new domain-prompt files. The schema permissiveness is correct (per DESIGN); the convention closes the cold-review gap.

# Cross-finding coherence

F1 + F3 + F6 are the same defect class: a silent change to operator-observable behavior with no in-toolkit bridge. The bundle is correct-in-the-code and incomplete-on-the-operator-path. F2 is a separate diagnostic-prose call. F4 is the intended trade-off requiring discoverability scaffolding.

Word count: ~770.
