---
schema_class: review-entry
schema_version: 1.0.0
review_number: 2
date: 2026-06-02
phase: phase-3
scope: >-
  Crosslink #12 bundle (reserved-code drift fix, DSL Field-on-Object-missing-key
  symmetry, defined() empty-string carve-out drop). Subject: the Phase 2a Red
  Gate (mdatron/mdatron-core/tests/phase_1_contracts.rs, 9 tests) + Phase 2b
  implementation (codes.rs reserved-range table with 8 unit tests; expr.rs
  updated missing_field_returns_null and defined_is_strictly_not_null tests).
lens: >-
  QE dims 1 (falsifiability), 3 (edge enumeration), 6 (mutation readiness),
  7 (property-test surface). Cold-session adversarial — what stub passes the
  test but doesn't deliver the behavior?
source: director-raised
session_note: >-
  Cold-session reviewer per Phase 3 primer; no prior-session memory of crosslink
  #12 design discussions. Findings asked for each Red Gate test: "what is the
  smallest mutation that survives this assertion?"
model: claude-opus-4-7
execution_method: >-
  inline single-domain QE cold reviewer; read DESIGN.md + phase_1_contracts.rs
  + codes.rs + expr.rs test block + verify.rs emission sites; cross-referenced
  observable assertions against test bodies for delta surface.
sycophancy_compensation: >-
  No author-overlap; reviewer is cold to the implementation. Discipline applied:
  every "test looks fine" instinct converted into a named mutation survivor.
---

# Findings

## F1 — `schema_violation_emits_e0050` is a tautology (PROBE constants masquerade as observation)
**domain:** quality-engineer **dim:** 1 (falsifiability) **classification:** resolved-pending **routing:** software-engineer
**source:** domain-raised

The Red Gate's `current_emitted_code_for_schema_violation()` returns
`PROBE_SCHEMA_VIOLATION_CODE`, a `const &str = "MDATRON-E0050"` defined in
the same test file. The assertion compares the constant to itself. **Any
mutation of `verify.rs:273`** (e.g., a stub that emits `"MDATRON-E9999"`,
`"FOO"`, or `""`) survives this test. Same for the parse-failure twin.

Real falsifying probes exist downstream: `verify.rs:542` already asserts
`findings[0].code == "MDATRON-E0050"` via an actual `verify()` call. The
Red Gate should either (a) call into `verify()` directly with a known
schema-violating fixture and assert the emitted code, or (b) drop these
two tests and rely on `verify.rs`'s in-module tests as the contract.

The "Red Gate test that fails before Phase 2b and passes after" property
is only true here because the operator hand-edited the constants when
landing 2b — not because behavior was observed.

## F2 — `all_emitted_codes_are_reserved` lint has named gaps; runtime construction defeats it
**domain:** quality-engineer **dim:** 3 (edge enumeration) **classification:** deferred **routing:** software-engineer
**source:** domain-raised

`extract_mdatron_code_literals` is a substring scan. Survivors:
- Runtime-constructed codes: `format!("MDATRON-E{:04}", n)` produces no
  `"MDATRON-Exxxx"` literal — emission of an unreserved code escapes.
- Multi-line / concatenated strings: `"MDATRON-" + variant_suffix` is not
  matched (the scanner stops at the first non-`[A-Za-z0-9-]` char).
- Comments + docstrings: scanned identically to code, so a stale doc
  comment referencing a now-unallocated `MDATRON-E0061` produces a false
  positive (lint failure for a comment).
- `#[cfg(test)]` fixtures: scanned. Any test fixture emitting `TEST-E0001`
  is fine, but a test using `"MDATRON-E0099"` to assert rejection would
  itself fail the lint.
- The `codes.rs` allowlist is a hard-coded suffix match (`entry.ends_with("codes.rs")`),
  meaning **any file named `codes.rs` anywhere in the scan tree bypasses
  the lint**. Path-based allowlist should be exact-path.

Routing: SE to (1) document these holes as known-coverage gaps in the
DESIGN.md, OR (2) tighten the extractor to skip comments + docstrings and
the allowlist to exact-path. Phase 5 surface candidate.

## F3 — Field-symmetry has no nested-missing test
**domain:** quality-engineer **dim:** 3 (edge enumeration) **classification:** resolved-pending **routing:** quality-engineer (test-only)
**source:** domain-raised

Three tests cover Field-on-Object-missing, Field-on-Null, Field-on-Int.
**Nested missing (`a.b` where `a` is an Object lacking `b`, then `.b.c`)
is not tested.** Critical because the M4 PE F1 root-cause repro is exactly
`every(s in $self.optional_field, ...)` — the missing-field result feeds
into another expression. A stub returning `Ok(Null)` only at the top level
but `FieldNotFound` on a nested miss passes all three tests yet fails the
DESIGN.md observable: "vsdd's pattern files containing `every(...)`
evaluate without error when `$self.optional_field` is absent."

Add: `nested_field_on_missing_intermediate_returns_null` —
`Expr::Field(Expr::Field(obj_without_a, "a"), "b")` returns `Ok(Null)`.
Also recommend the DESIGN.md-named `every_over_missing_optional_field_does_not_panic`
seed (Red Gate § Phase 2a seeds line 223) — it was specified but not
implemented in the Red Gate file.

## F4 — `defined()` edge enumeration is shallow
**domain:** quality-engineer **dim:** 3 (edge enumeration) **classification:** deferred **routing:** quality-engineer
**source:** domain-raised

Three tests (Null, empty string, empty array). Untested edges that the
DESIGN-stated contract ("strict not-Null") commits to:
- `defined(0)` (Int zero — historically truthy/falsy confusion)
- `defined({})` (empty Object)
- `defined(" ")` (whitespace-only string — distinguishes "not-Null" from "non-blank")
- `defined(false)` (Bool false)
- Arity errors: `defined()` (zero args), `defined(a, b)` (two args) — the
  contract is silent on whether these are TypeMismatch / ArityError / panic.
  A stub `fn defined(_args) -> Bool(true)` passes 2/3 current tests.

Phase 5 surface (already named in DESIGN.md): proptest over
`Arbitrary<Value>` asserting `defined(x) == !matches!(x, Value::Null)`.
This is the mutation-kill move; the unit tests are insufficient on their
own.

## F5 — Schema-tightening revert has no corpus regression test
**domain:** quality-engineer **dim:** 2 (acceptance criteria coverage) **classification:** resolved-pending **routing:** software-engineer
**source:** domain-raised

DESIGN.md § Acceptance criteria line 237: "vsdd corpus passes
`mdatron verify` after the schema-tightening reverts on
`supplements_in_scope` / `supplements_applied`." **No test in the Red
Gate asserts this.** The Red Gate is mdatron-side; the corpus is
vsdd-side. The acceptance criterion is therefore unverifiable by the
Phase 2b green run alone.

Mutation survivor: a Phase 2b that lands the Field-symmetry fix in expr.rs
but leaves the schema revert undone in `phase-primer.json` /
`domain-prompt.json` passes all 9 Red Gate tests and still leaves the
corpus in a brittle state. Conversely, a botched revert (removing
`supplements_in_scope` from `required` but breaking the schema syntax)
passes the Red Gate and breaks the corpus.

Routing: SE to add a vsdd-cli-side integration test that runs
`mdatron verify` against the vsdd corpus post-revert; OR Phase 4 to route
this as a process gap (corpus regression check must be a CI gate). The
"audited prior; confirmed via grep" line at DESIGN.md:167-168 is not an
automated regression and should be promoted to one.

## F6 — `is_reserved_mdatron_code` accepts leading-zero-stripped codes
**domain:** quality-engineer **dim:** 3 (edge enumeration) **classification:** deferred **routing:** software-engineer
**source:** domain-raised

`number_part.parse::<u32>()` accepts `"1"`, `"01"`, `"001"`, `"0001"` — all
return `1`. So `is_reserved_mdatron_code("MDATRON-E1")` returns true.
The wire-format contract (per crosslink #11) commits to 4-digit codes.
A code emission of `"MDATRON-E1"` would pass the lint but violate the
output-format contract. The 8 unit tests in `codes.rs` use only 4-digit
canonical forms; no test asserts that `"MDATRON-E1"` is **rejected**.

Add: `non_four_digit_codes_are_rejected` — assert
`!is_reserved_mdatron_code("MDATRON-E1")` and
`!is_reserved_mdatron_code("MDATRON-E00001")`.

## F7 — `other_prefixes_are_not_mdatron_codes` is a load-bearing comment
**domain:** quality-engineer **dim:** 1 (falsifiability) **classification:** dismissed **routing:** none
**source:** domain-raised

The `format!("{}{}-E0001", "VS", "DD")` construction is documented as
working around a cross-repo namespace-separation lint. This is fine, but
note that the lint itself is **not in this repo** (the file is in mdatron,
not vsdd-cli). If the lint is removed or renamed cross-repo, this test
becomes a needless obfuscation. Recommend a `// SAFETY:` style comment
naming the upstream lint by ID. **Dismissed** as a real defect — the
construction is correct — but flagged as maintainability drift surface.

# Summary

- **2 resolved-pending** (F1 test-tautology; F3 missing nested test) — block
  Phase 3 closure; route to SE/QE for Phase 2a augmentation.
- **1 resolved-pending** (F5 corpus regression) — routes to SE for the
  vsdd-side gate.
- **2 deferred** (F2 lint gaps; F4 defined() edges) — Phase 5
  property-test surface; named but not blocking.
- **1 deferred** (F6 digit-width) — Phase 5 / quick add.
- **1 dismissed** (F7).

Headline mutation survivor: a Phase 2b that emits the wrong code from
`verify.rs:273` still passes `schema_violation_emits_e0050` because the
test asserts a probe constant equal to itself. The actual behavior probe
is in `verify.rs:542`, not in the Red Gate. The Red Gate's title promises
contract coverage; the implementation delegates to in-module tests
without naming the delegation.
