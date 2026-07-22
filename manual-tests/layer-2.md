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
