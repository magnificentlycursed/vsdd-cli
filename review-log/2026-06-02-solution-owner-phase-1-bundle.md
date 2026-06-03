---
schema_class: review-entry
schema_version: 1.0.0
review_number: 2
date: 2026-06-02
phase: phase-3
scope: >-
  vsdd-cli crosslink #12 Phase 1 bundle (codes-and-dsl). Subject — mdatron-core
  codes.rs / verify.rs / dsl/expr.rs / tests/phase_1_contracts.rs reflecting the
  three bundled changes (reserved-code rename, DSL Field-on-Object-missing-key
  symmetry, defined() empty-string carve-out drop) plus the vsdd-cli schema
  reverts that drop `supplements_in_scope` / `supplements_applied` from required.
lens: >-
  Solution Owner — spec-contract authority + scope discipline + Raise-to-SO
  routing integrity. Five-lens weighting Consistency (5) > Maintainability (3)
  > Edge (2) > Usability (1) > Attacker (1).
source: director-raised
session_note: >-
  Cold-session adversarial review of a three-change bundle authored entirely by
  Claude. SO lens is the primary spec-contract guardrail; Sanity Check is the
  validator-pair terminator on any Raise-to-SO disposition surfaced here.
model: claude-opus-4-7
execution_method: >-
  Cold-session sub-agent, worktree-no-memory, Phase 3 primer + SO + SC prompts
  loaded fresh; no prior-cycle context retained.
sycophancy_compensation: >-
  Claude authored every line under review (DESIGN, Red Gate, implementation, and
  the schema reverts). SO is the spec-contract guardrail by role; findings below
  hold the bundle to spec-as-contract regardless of author identity.
supplements_loaded: []
---

# Solution Owner Phase 3 Review — Crosslink #12 Bundle

## SO-F1 — DESIGN-MDATRON.md table amendment not authored (spec-vs-code drift)

**Classification:** deferred. **Routing:** Raise-to-SO. **Lens:** Consistency / Dim 0.

`codes.rs:14-18,40-42` declare three new reserved ranges (`E0050-E0059`,
`E0070-E0079`, `E0080-E0089`) citing "v0.1.x amendments." The sub-DESIGN
names them. But `DESIGN-MDATRON.md:506-514` — the canonical table the lint
references — is not edited by this bundle that I can verify. If unamended in
the same commit, the source-of-truth spec contradicts the implementation. The
lint becoming "the canonical replacement" is the SO failure mode "spec amended
silently to match implementation."

Better: DESIGN-MDATRON.md:506-514 carries the three new rows in the same
commit and the bundle emits `OperatorDirectiveApplied{directive:
spec-contract-amended}`. The lint is enforcement, not contract.

## SO-F2 — Pre-publish contract break: correct window, undeclared stability stance

**Classification:** accepted with remediation. **Routing:** SO. **Lens:** Consistency / Dim 3.

`MDATRON-E0001` semantics flip (was schema-violation, now parse-failure) and
`E0002` semantics flip (parse-failure no longer emitted). DESIGN frames this as
"no behavior change... beyond the numeric values" and asserts
`mdatron_output_version` stays `1.0.0` because reserved-range additions are
additive. But renaming an emitted code is not additive — it breaks any consumer
pinning on `E0001 == schema-violation`. The pre-publish window is the right
time to ship the break; the DESIGN claim that the wire-format version doesn't
tick is the category error.

Better: DESIGN declares the "pre-1.0 codes are reassignable" stability stance
explicitly, OR ticks `mdatron_output_version` to `1.1.0` and notes the rename
as a pre-1.0 carve-out. Pick one.

## SO-F3 — Schema-tightening revert undoes an applied directive without audit event

**Classification:** deferred. **Routing:** Raise-to-SO. **Lens:** Consistency / Dim 5.

`phase-primer.json` and `domain-prompt.json` previously required
`supplements_in_scope` / `supplements_applied`; the bundle reverts both to
optional. DESIGN frames this as "two earlier reactive schema-tightenings can
be reverted." That elides audit-trail discipline: the original tightening was
an operator-applied fix surfaced by M4 PE F1 — an `OperatorDirectiveApplied`
event. Reverting it is undoing an applied directive. Per the SO failure mode
list, this requires an explicit `OperatorDirectiveRevoked` (or
`...Superseded`) event citing the M4 PE F1 directive being unwound and the
root-cause fix that supersedes it.

Better: bundle commit message emits the methodology event, naming the prior
directive by review-log path + finding ID. Without it, a future cold reader
sees only the schema diff and cannot reconstruct why required-ness flipped.

## SO-F4 — defined() carve-out drop: no future-contract breakage

**Classification:** hallucinated. **Routing:** n/a. **Lens:** Consistency / Dim 1.

The carve-out was never formalized in DESIGN-MDATRON.md as a behavioral
contract; DESIGN confirms a `vsdd-core/patterns/` audit found no dependency.
No external adopter exists. Dropping it tightens orthogonality (defined = not
Null; emptiness via `!= ""`). Reflected in `dsl/expr.rs:767-789` and Red Gate
`phase_1_contracts.rs:139-167`. Clean.

## SO-F5 — Bundle vs split: defensible, but rollback strategy undeclared

**Classification:** accepted. **Routing:** SO. **Lens:** Consistency / Dim 3.

DESIGN § Decomposition addresses this: three independent changes too small
for individual milestones, bundled so Phase 2a covers them in one Red Gate.
`tests/phase_1_contracts.rs` partitions cleanly into three labeled sections,
preserving per-contract falsifiability.

The concern: if Phase 3 forces rework on (say) the code rename, the bundle
commit cannot be reverted without dragging the DSL fixes back too. The
declared-up-front scope sidesteps "scope creep one finding at a time"; the
inverse risk (bundled-rollback-friction) is real. Phase 1c DESIGN should
state: "if any contract regresses post-merge, the bundle is reverted whole and
re-decomposed."

## Cross-finding coherence

SO-F1 + SO-F3 are the same failure-mode-class — spec/event-log silence on
applied changes. Both should be resolved in the same commit since both belong
to the same methodology-amendment cycle. SO-F2 is independent (consumer
contract, not spec-vs-code drift).

## Recommended dispositions

- SO-F1: Resolved-pending — amend DESIGN-MDATRON.md:506-514 in bundle commit.
  Block Phase 4 close until verified.
- SO-F2: Accepted — DESIGN declares pre-1.0 codes-reassignable stance OR tick
  `mdatron_output_version`.
- SO-F3: Resolved-pending — `OperatorDirectiveRevoked` event in commit
  message citing M4 PE F1.
- SO-F4: Hallucinated; no action.
- SO-F5: Accepted — Phase 1c DESIGN adds rollback-strategy paragraph.

Three Resolved-pending + two Accepted; Phase 3 continues per primer
swarm-invocation triggers.
