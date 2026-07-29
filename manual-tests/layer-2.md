# Layer 2 manual-test checklist — snapshot, phase answer, corroboration

Operator-adopted 2026-07-22 (decision on vsdd-cli #738). Director tests
the automated suite cannot grade; outcomes recorded on the layer trail.

1. **Convergence independence check** — the one check no agent performs
   on its own oracle. Pick two or three fixtures under
   `vsdd-core/tests/fixtures/convergence/`; from each `state.yaml` and
   `snapshot.yaml` alone, answer: what phase, what layer, what next,
   degraded?, anything inconsistent? Compare against `expected.yaml`.
   Your independent answer wins every disagreement.
2. **Degraded texts read well.** Trigger both degraded fixtures through
   the human form when Layer 3 lands (or read the statusline set's
   next-step texts against the fixtures now): does "tracker offline"
   read as the contracted normal condition, not an alarm?
3. **The hollow-shell substrate report reads as instructions.** Run the
   substrate check against a stripped temp tree; judge whether each
   finding names the artifact and the repair a tired human can follow.
4. **The live refs query.** Run the off-grammar query over this repo's
   own branches; confirm the output flags nothing and reads plainly.
5. **The live finding-query join** (vsdd-cli #820). Run `cargo test -p
   vsdd-core --lib snapshot::acquire::tests::live_finding_walk_in_isolation
   -- --ignored --nocapture` against a repo with a live crosslink tracker.
   Confirm the walk lists exactly the review-round children in the
   forward-only universe (open, or closed at/after 2026-07-27), and judge
   each finding's routing (a `plan` comment), disposition (a
   dismissed/hallucinated/consolidated label), and universe classification
   against your own read of the tracker. A closed fix-close with no routing
   is the unrouted-findings condition the guardrail exists to catch.
   End-to-end via `vsdd status` awaits the milestone-empty-parse fix (#829).
