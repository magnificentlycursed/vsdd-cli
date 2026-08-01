---
title: "Finding-query join (the unrouted-findings query's live acquisition)"
tags: ["design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-07-29
updated: 2026-08-01
---

# Feature: Finding-query join (the unrouted-findings query's live acquisition)

## Summary

Make the unrouted-findings process-integrity query run against live tracker
data by having `acquire_snapshot` populate `snapshot.findings` from crosslink,
instead of the current bootstrap `findings: Vec::new()`. This is the Slice-1
(vsdd-cli #820) tracker-join increment: the pure query already exists
(`vsdd-core/src/answer/integrity.rs`) and the pure acquisition-side mappers are
green (`vsdd-core/src/snapshot/acquire.rs`); the missing piece is the effectful
join that feeds them, plus a field-readiness discipline so populating findings
does not mis-fire the sibling checks that read fields Slice 1 defers.

## Requirements

- REQ-1: `acquire_snapshot` (`vsdd-core/src/snapshot/acquire.rs:57`) populates
  `Snapshot.findings` with a `FindingRecord` per crosslink finding in the
  forward-only universe, replacing the hardcoded `findings: Vec::new()`
  (`acquire.rs:111`). The existing pure query (`integrity.rs:72`) then fires
  against live data with no change to its body.
- REQ-2: A finding is discriminated by the acquisition, not the query: only an
  issue whose parent is a review-round issue (parent carries the `review`
  label) becomes a `FindingRecord`. The join evaluates `is_finding` over the
  parent's labels; the query keeps operating over an already-filtered
  `snapshot.findings` set (it does not itself call `is_finding`).
- REQ-3: The join acquires findings by a bounded walk, keyed on child
  `parent_id` (verified against the live tracker: `issue show`'s `subissues`
  field is never populated, so the parent-side child list is not a usable
  linkage; the child's `parent_id` is): (1) `crosslink issue list --label review
  -s all --json` for the review-round issue ids; (2) one `crosslink issue list -s
  all --json` for every issue's `parent_id`, `status`, and `closed_at`, filtered
  to issues whose `parent_id` is a review-round id (the `is_finding` predicate)
  and which fall in the forward-only universe (REQ-5); (3) `crosslink issue show
  --json` per surviving finding for its `labels` (disposition, REQ-9) and
  `comments` (routing). Every subprocess call goes through `run_bounded`
  (`acquire.rs:41`, `vsdd-core/src/subprocess.rs`) — bounded and timed like the
  existing legs. The two `issue list` calls are fixed-count; only step (3) is
  per-finding, and it runs only over the forward-only universe.
- REQ-4: The walk is bounded against the "one bounded acquisition per
  invocation" contract by (a) restricting the finding universe to the
  forward-only set (REQ-5 below) and (b) a declared hard cap on findings
  queried per acquisition; when the cap is reached the acquisition records a
  worded truncation marker rather than silently dropping findings (the
  no-silent-caps discipline).
- REQ-5: The universe is forward-only per the re-sequence-enforcement-spine
  amendment (REQ-5; the design doc was removed under vsdd-cli #826 — the
  amendment survives as a crosslink knowledge page): a finding
  closed strictly before the routing amendment's ratification boundary is
  outside the universe; a finding open at ratification, or closed at/after it,
  or reopened after it, is inside. The boundary value is the routing amendment's
  ratification, 2026-07-27 (vsdd-cli #810), per the contract's Status
  requirement (`.design/agent-first-vsdd-toolkit.md:173`,
  "its universe the findings at or reopened after the amendment's ratification
  boundary"). The join supplies this boundary to `closed_before_ratification`.
- REQ-6: The join populates only the enforcement-spine fields a live finding can
  currently source — `handle`, `status`, `disposition`, `routing_present`,
  `closed_before_ratification`. The lifecycle-role fields (`owner`,
  `validator`) and `evidence_reference_present` are deferred to Slice 5 and are
  NOT populated by this join.
- REQ-7: Because REQ-6 leaves the deferred fields unset, the snapshot carries
  which finding-field groups the join actually acquired, and the sibling
  integrity checks that read the deferred fields
  (`findings-missing-owner-or-validator` at `integrity.rs:46`;
  `closed-findings-missing-evidence` at `integrity.rs:56`) run only when their
  input group was acquired. Against a live spine-only snapshot they stay
  dormant; against the convergence fixtures (which declare full acquisition)
  they run unchanged.
- REQ-8: A finding-leg subprocess failure (spawn failure, timeout, oversize,
  refused, or unparseable output during the walk) leaves the snapshot
  `Acquired` with `findings` empty — findings are a join that can be absent
  without invalidating the milestone and session legs. The finding-leg failure
  never maps to `Unusable` (contrast the milestone/session legs at
  `acquire.rs:71,89`).
- REQ-9: `disposition` is sourced from a disposition label-carry: a finding
  closed by disposition carries a `dismissed`, `hallucinated`, or `consolidated`
  label, read from `issue show` `labels` and mapped to `FindingRecord.disposition`
  (an unlabelled closed finding maps to `None`). This makes the disposition
  exemption (`integrity.rs:74`) honorable with no upstream change. Prerequisite:
  the disposition closures in the forward-only universe are labelled —
  retroactively for the closures already past the 2026-07-27 boundary, and going
  forward as the bootstrap discipline. A structured crosslink close-reason field
  (upstream request filed, see Decisions) supersedes the label-carry when it
  ships, moving the read from `labels` to that field with no query-side change.

## Acceptance Criteria

- [ ] AC-1: With a crosslink repo containing a review-round issue and a child
  finding closed by fix after the boundary with no `plan` comment, an
  integration test over `acquire_snapshot` + `snapshot_integrity` yields the
  `unrouted-findings` kind.
- [ ] AC-2: The same finding with a `plan` comment filed yields no
  `unrouted-findings` kind (routing present suppresses it).
- [ ] AC-3: A finding closed before the boundary yields no `unrouted-findings`
  kind (outside the forward-only universe), even with no routing.
- [ ] AC-4: An issue with no review-labelled parent is never present in
  `snapshot.findings` (the `is_finding` acquisition filter).
- [ ] AC-5: A live spine-only snapshot (owner/validator/evidence unacquired)
  produces neither `findings-missing-owner-or-validator` nor
  `closed-findings-missing-evidence`, while the convergence fixtures continue to
  produce them (the field-readiness guard).
- [ ] AC-6: A simulated finding-leg subprocess failure leaves
  `acquisition_outcome == Acquired` with `findings` empty and the milestone and
  session fields populated.
- [ ] AC-7: Exceeding the finding cap records a worded truncation marker
  observable on the snapshot; no finding is silently dropped.
- [ ] AC-8: `cargo test --workspace` and `mdatron verify` remain green; the
  existing convergence-corpus finding tests still pass unchanged.
- [ ] AC-9: A closed finding in the universe carrying a `dismissed` label yields
  no `unrouted-findings` kind (the disposition exemption via the label-carry),
  and the same finding with no disposition label and no routing does yield it.
- [ ] AC-10: The disposition label -> `FindingRecord.disposition` mapping is
  unit-tested for each of `dismissed`/`hallucinated`/`consolidated` and the
  unlabelled (`None`) case.

## Architecture

The join lands entirely on the shell side. `acquire_snapshot`
(`vsdd-core/src/snapshot/acquire.rs`) already sequences the milestone leg then
the session leg with a uniform outcome mapping over `Subprocess::{Completed,
NotFound, Refused, ...}`. The finding leg is a third leg with a *different*
outcome mapping (REQ-8): its failure degrades `findings` to empty, not the whole
snapshot to `Unusable`.

The finding walk (REQ-3) reuses the three green pure mappers already in
`acquire.rs` (`is_finding`, `routing_present`, `closed_before_ratification`) as
the field derivations, and `run_bounded` (`vsdd-core/src/subprocess.rs`) as the
bounded-subprocess primitive. `crosslink issue list --json` carries `parent_id`,
`status`, and `closed_at` but neither `labels` nor `comments` (verified), so the
bulk list resolves the finding set (children of review-round ids), the
forward-only filter, and `closed_before_ratification`, while the per-finding
`issue show` reaches only `labels` (for `disposition`) and `comments[].kind`
(for `routing_present`); the N+1 is confined to that per-finding step over the
forward-only universe (REQ-5) and a hard cap (REQ-4). The parent-side
`subissues` field is not usable for the walk (it is never populated on `issue
show`; vsdd-cli #828), so linkage is read from child `parent_id`.

`FindingRecord` (`vsdd-core/src/snapshot/mod.rs:29`) already carries all the
fields the query reads; its serde defaults (`routing_present` default false,
`closed_before_ratification` default true) mean an unpopulated record stays
outside the universe by default — the safe direction. The field-readiness guard
(REQ-7) requires the snapshot to distinguish "acquired as absent" from "not
acquired": an additive `Snapshot` field naming the acquired finding-field groups
(spine / lifecycle-roles / evidence), consulted by the three finding-reading
checks in `snapshot_integrity`. This is an additive change to `Snapshot` and its
mirror `templates/registry/snapshot-schema.md` (`mod.rs:3` — the struct mirrors
the schema verbatim), a non-breaking minor schema bump; the convergence fixtures
gain the marker set to all-acquired so their existing expectations hold
unchanged.

Scope boundary the join respects: the contract's Status requirement
(`.design/agent-first-vsdd-toolkit.md:173`) defines the query over findings that
"closed by fix, or that survive their round open" — but `integrity.rs:72`
implements only the closed-by-fix case, because the open-survivor case needs
round-membership data (round manifests and children), which is Slice 6. The join
populates what the closed-by-fix case needs; the open-survivor case remains a
Slice-6 extension of both the acquisition and the query.

Error handling follows the module's existing discipline: absent and unusable are
outcomes carried in the snapshot, never errors, and are never swapped
(`acquire.rs` module doc). The finding leg extends this with a third shape —
findings-absent-within-an-acquired-snapshot — for REQ-8.

## Decisions

### D1: Disposition is sourced from a label-carry (resolves the former Q1)

Ruling (operator, on #820): adopt a disposition label-carry now, and file an
upstream crosslink close-reason request to supersede it if implemented.

The unrouted-findings query reads `f.disposition.is_none()` (`integrity.rs:74`)
to exempt disposition closures — the contract makes these exempt and their
absence a falsification (`.design/agent-first-vsdd-toolkit.md:78,177`), so
`disposition` is a spine input the join must source. Crosslink stores no
structured disposition (verified: `issue close` takes no reason, `issue update`
has no disposition field, no disposition labels are in use). The join therefore
reads disposition from a label — `dismissed` / `hallucinated` / `consolidated` —
per REQ-9. This slots into the contract's existing format-carry pattern: routing
rides a `plan` comment, parity rides result-comment handles, disposition rides a
label — each an input the mechanized query consumes.

Contract note: promoting the label-carry from a build convention to a
contract-level bootstrap format-carry (symmetric with the routing carry,
`.design/agent-first-vsdd-toolkit.md:177`) re-enters the spec-amendment loop
under review — it is NOT hand-edited into the contract from this design.

Supersession: the upstream crosslink structured close-reason field (request filed
for operator relay) replaces the label-carry when it ships; the join's read moves
from `labels` to that field with no change to the pure query.

## Out of Scope

- Owner, validator, and evidence-reference acquisition for live findings — the
  lifecycle-role and evidence field groups (Slice 5); the field-readiness guard
  keeps their checks dormant meanwhile.
- Round manifests, round children, and the open-survivor ("survives its round
  open") branch of the unrouted-findings query — round-membership data (Slice 6).
- The routing-before-fix blocking guardrail itself (the phase-4 unrouted-findings
  gate command) — this join makes the DETECTION live; the block/pass guardrail
  that consumes the now-live query is the Slice-1 guardrail increment that
  follows this join.
- Comment-handle and resolvability acquisition (the `comment_handles` join).
