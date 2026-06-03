---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-06-02
phase: phase-3
scope: >-
  Crosslink #12 bundled changes per docs/refactor/phase-1-codes-and-dsl/DESIGN.md
  — reserved-code drift fix (E0001/E0002/E0050/E0070/E0080), DSL Field-access
  symmetry, defined() empty-string carve-out drop. Cross-process / install /
  audit surface touched: mdatron-core code emissions, .githooks/pre-commit,
  .github/workflows/mdatron-verify.yml, vsdd-core/schemas/phase-primer.json +
  domain-prompt.json revert candidates, the lint surface
  `all_emitted_codes_are_reserved`.
lens: Platform Engineer — dim 1 (reproducible builds / version-pin discipline), dim 2 (dependency approval / cross-repo contract), dim 4 (CI workflow discipline + hook discipline), dim 7 (observability of CI itself), with cross-process implications focus.
source: operator-directive
session_note: >-
  Cold-session Phase 3 review of the bundled DESIGN per crosslink #12.
  No prior-session memory. Subject is the design as-written (mdatron-core
  implementation lives in a sibling repo — not on disk in vsdd-cli), so this
  review reasons against the contract surface + the consumer-side artifacts
  on disk (pre-commit hook; CI workflow; phase-primer / domain-prompt schemas;
  the prior PE briefs in review-log/2026-06-02-platform-engineer-*).
model: claude-opus-4-7
execution_method: >-
  Single-domain cold-session review (PE primary). No code modifications.
  Operator-supplied prompt named the six PE-lens questions; this entry
  answers each with a classified finding.
sycophancy_compensation: >-
  PE-lens bias is "more pins, more attestations, more gates." The v0.1
  counter-bias is "every gate is overhead until it bites." I am resisting
  both by grading WHICH of the six raised questions reach the v0.1 ship-blocker
  bar versus which are Phase 5 / v0.2 hardening; not every Yes is correct,
  and not every No is permissive-drift.
supplements_loaded: []
---

# Platform Engineer Review — Phase 1 codes-and-dsl bundle (crosslink #12)

## Headline

The bundle's wire-contract claim ("mdatron_output_version stays 1.0.0") is correct under the Phase 0 versioning rule — but only if `is_reserved_mdatron_code()` is treated as cross-process private; the bundle should land with a CI lint gate (not just `cargo test`), a hook-side mdatron-version sanity check, and explicit "reserved-future" annotations on the gap ranges E0060-E0069 / E0090-E0099. The schema-tightening revert is safe because the on-disk schemas in this checkout never enforced the required fields in the first place.

## Per-question findings

### F1 — Output-version bump: NOT needed; the DESIGN's reasoning holds

**Question:** does renaming `MDATRON-E0001 → E0050` (and renumbering the parse-failure emission to take E0001) require bumping `mdatron_output_version` from 1.0.0 to 1.1.0 or 2.0.0?

**PE rationale (dim 1, reproducibility-of-contract):** the Phase 0 output-format DESIGN names the versioning rule explicitly (`docs/refactor/phase-0-output-format/DESIGN.md:323-329`): "Add an optional field: no version bump. Remove a field, add a required field, change a field's type or semantics: bump." The `code` field's `type` is unchanged (still `string` matching `^MDATRON-[EWL][0-9]{4}$`); its `semantics` are unchanged (still "an identifier for the diagnostic class"); the specific numeric value emitted for a given diagnostic class is NOT part of the wire-format contract. The Phase 1 DESIGN's claim at line 87-89 ("`mdatron_output_version` stays `1.0.0` because adding a new code numeric within reserved ranges is an additive change") is contract-correct.

**Caveat that escalates this:** any consumer that pattern-matches on the literal `MDATRON-E0001` to mean "frontmatter-schema-violation" (the prior meaning) IS broken — but the right framing is "consumers must not bind to specific code numbers without consulting the explain catalog," not "this is a wire-format break." The DESIGN should call this out explicitly in `## Behavioral contracts` so a future reviewer doesn't re-litigate.

**Classification:** Resolved-pending (1-line spec addition in Phase 1 DESIGN naming the consumer-side discipline).

---

### F2 — Pre-commit hook needs a mdatron version sanity check; current hook is "any mdatron will do"

**PE rationale (dim 4 + dim 1):** `.githooks/pre-commit:6-10` skips if mdatron is absent but does NOT pin or check version when present. After this bundle lands, an operator with a pre-bundle mdatron on PATH (still emitting old codes) will run the hook against the new patterns / new explain-catalog expectations; the hook will pass-or-fail on STALE code emissions. Operators experiencing CI green / local-hook red (or vice versa) get told "discipline held" by exit code 0 — the PE sycophancy failure mode "CI green is the only signal."

**Concrete v0.1 wiring:** the hook should call `mdatron --version` and compare against a pin in `.mdatron/manifest.toml` (or equivalent); on mismatch, emit `pre-commit: mdatron version X.Y.Z does not match pinned A.B.C — run cargo install --path ../mdatron/mdatron-cli to upgrade` and exit non-zero. The CI workflow already pins via `mdatron/Cargo.lock` (`mdatron-verify.yml:33-40`); the local hook does not.

**Classification:** Accepted-pending (Phase 4 → new milestone; not a Phase 1 ship-blocker but routes Phase 5 hardening).

---

### F3 — `all_emitted_codes_are_reserved` MUST run in CI, not just `cargo test`

**PE rationale (dim 4 + dim 7):** the DESIGN names this as a `cargo test`-shaped lint (lines 84-86 + 227-229). PR merges that add a new emission with an unreserved code would land if the test isn't part of the CI-required-check set. Current CI workflow (`mdatron-verify.yml`) runs only `mdatron verify --project-root .` against vsdd-cli's corpus; it does NOT run mdatron-core's own test suite. The lint catches additions-during-merges only if the mdatron repo's CI gates it as a required check; vsdd-cli's CI does not, and cannot (lint runs against mdatron-core source which lives in the sibling repo).

**Concrete fix:** mdatron-core's CI (in the sibling repo, not this one) must mark `cargo test --test all_emitted_codes_are_reserved` (or whatever name) as a required-status-check. This is a cross-repo PE finding — the lint is necessary but not sufficient unless the CI gate is wired. The DESIGN should name this explicitly under "acceptance criteria" as "CI required-check entry in mdatron-cli."

**Classification:** Accepted-pending. Routes Phase 4 → cross-repo coordination with mdatron-cli's CI config. Cross-process implication: the lint is the static-check side of the namespace-separation contract from Phase 0 DESIGN:295-299; both sides must run in CI for the contract to hold.

---

### F4 — Schema-tightening revert is safe IN THIS CHECKOUT (already not enforced); broader downstream impact is bounded

**PE rationale (dim 1, reproducibility-of-spec-to-impl):** verified inline — `vsdd-core/schemas/phase-primer.json:7-15` does NOT list `supplements_in_scope` in the `required` array; `vsdd-core/schemas/domain-prompt.json:7-14` does NOT list `supplements_applied` in `required`. So the "revert" in the DESIGN (line 116-119) reverts a tightening that, in this on-disk checkout, was never applied to the schemas. EITHER the DESIGN is talking about a tightening that lives somewhere else (the .mdatron-mirrored schemas? the patterns? the pattern DSL?), OR the tightening lived briefly and was already rolled back.

**Sub-finding:** the DESIGN's claim "two earlier reactive schema-tightenings can be reverted" is, at minimum, ambiguous about WHERE the tightening lives. If it's in `.mdatron/schemas/phase-primer.json` (the mdatron-mirrored copy), that's a different file from `vsdd-core/schemas/phase-primer.json`, and a downstream adopter who composed the vsdd-core schema verbatim wouldn't see the tightening at all. The DESIGN should cite the file path of the tightening it's reverting.

**Downstream impact assessment:** for ANY downstream that enumerated `required: [..., supplements_in_scope, ...]` in their own adopter validator (e.g., a fork that hardened the schema), this revert is non-breaking-for-them (they still validate; we just emit a wider-acceptable shape). For downstreams that wrote test fixtures asserting the field's presence: those test fixtures stay green (the field is still emitted in vsdd-core's own files); only fixtures asserting "required-field-error fires when absent" would regress. Adopter risk: low. PE-sycophancy-resistance: this is a Resolved, not a Hallucinated.

**Classification:** Resolved-pending (DESIGN clarification: cite the file path of the tightening being reverted).

---

### F5 — Gap ranges E0060-E0069 / E0090-E0099 need explicit "reserved-future" annotation, not silence

**PE rationale (dim 1, audit-trail of reserved namespace):** the DESIGN adds three contiguous-but-non-adjacent ranges (E0050-E0059, E0070-E0079, E0080-E0089). The gaps E0060-E0069 and E0090-E0099 are unspecified — a future contributor adding a new emission class might allocate into a gap without realizing it was deliberately left for a future class. The PE-failure-mode here is "Build pinned to 'latest' — reproducibility traded for ergonomics; bites at release time": a year-from-now contributor allocates E0065 for "validation-cache-stale" without realizing E0060-E0069 was meant for "schema-resolution failures."

**Concrete fix:** Phase 1 DESIGN should append two rows to the reserved-ranges table:

| Range | Class |
|---|---|
| `MDATRON-E0060` — `E0069` | Reserved for future use (no current emissions) |
| `MDATRON-E0090` — `E0099` | Reserved for future use (no current emissions) |

OR explicitly name them as "available for next-class allocation; document the class in the DESIGN amendment that introduces it." Either makes the discipline declarative; silence makes it discoverable only by reading the diff that allocated the previous class.

**Classification:** Resolved-pending (DESIGN amendment; mechanical).

---

### F6 — `is_reserved_mdatron_code()` should NOT be public mdatron-core API at v0.1; revisit at v0.2

**PE rationale (dim 2, dependency approval discipline + dim 4, CI workflow discipline):** exposing `is_reserved_mdatron_code()` as public API creates a cross-process contract where vsdd (the named consumer) would import mdatron-core as a library, not just spawn it as a subprocess. That's the opposite of the binary-first refactor's stated direction (`docs/refactor/binary-first-plan.md`). Per Phase 0 DESIGN:282-299, the cross-process boundary is JSON-over-stdout, NOT a library API. A `pub fn is_reserved_mdatron_code()` in mdatron-core would invite vsdd to depend on mdatron-core as a Rust dep — re-creating the supply-chain coupling the binary-first refactor was designed to break.

**The function's correct scope at v0.1:** crate-private (`pub(crate)`) — it's the implementation detail behind the lint. Downstream tooling (e.g., a third-party syntax-highlighter for mdatron codes) doesn't need it; the reserved-range table is published in `DESIGN-MDATRON.md:506-514` (post-amendment) and a downstream tool can scrape the table or copy it locally without taking a Rust dep.

**v0.2 revisit:** if mdatron ships a published-on-crates.io `mdatron-codes` mini-crate (just the table + the predicate), THAT can be public — and would be a cleaner cross-process contract than today's "scrape the spec doc." But that's a v0.2 / v0.3 surface, not Phase 1 of crosslink #12.

**Classification:** Accepted (keep crate-private at v0.1; route v0.2 consideration to the binary-first plan's later phases).

---

## Round-close summary

| Finding | Lens | Classification | Routing |
|---|---|---|---|
| F1 — output-version stays 1.0.0 | dim 1 | Resolved-pending | DESIGN clarification line |
| F2 — pre-commit version pin | dim 1 + dim 4 | Accepted-pending | Phase 4 → new milestone |
| F3 — lint must be CI required-check | dim 4 + dim 7 | Accepted-pending | Cross-repo CI coordination |
| F4 — schema-revert path citation | dim 1 | Resolved-pending | DESIGN clarification |
| F5 — gap-range annotation | dim 1 | Resolved-pending | DESIGN amendment |
| F6 — is_reserved_mdatron_code crate-private | dim 2 + dim 4 | Accepted | v0.1 ships crate-private |

**Sycophancy-compensation reflection:** the bias I resisted was treating every raised question as a ship-blocker requiring a Phase 4 routing. F1, F4, F5 are mechanical DESIGN edits (Resolved); F6 is a "no change needed but say so" (Accepted); only F2 and F3 are genuinely Accepted-pending with downstream work. PE-lens reflex would have ranked all six as "needs Phase 4 wiring"; the more honest grade is "three are spec hygiene, one is a non-decision, two are real cross-process wiring gaps."

**Cross-references:**
- `docs/refactor/phase-1-codes-and-dsl/DESIGN.md:60-90` (reserved-code drift fix; the wire-version claim)
- `docs/refactor/phase-0-output-format/DESIGN.md:323-329` (versioning rule)
- `docs/refactor/phase-0-output-format/DESIGN.md:282-299` (namespace separation contract)
- `.githooks/pre-commit:6-10` (no version pin)
- `.github/workflows/mdatron-verify.yml:33-44` (CI pins via Cargo.lock; vsdd-cli's CI does not run mdatron-core tests)
- `vsdd-core/schemas/phase-primer.json:7-15` + `vsdd-core/schemas/domain-prompt.json:7-14` (supplements_* NOT in required — revert is no-op in this checkout)
- Prior PE briefs: `review-log/2026-06-02-platform-engineer-init-drift.md`, `review-log/2026-06-02-platform-engineer-mdatron-consistency.md`
