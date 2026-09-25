# Slice 1 manual-test checklist — live self-governance (the tracker join and the routing guardrail)

PROPOSED (agent-drafted 2026-09-24 under vsdd-cli#874's session; adoption is the
operator's act per The operator authors the oracle — record the decision on the
tracker and replace this line with the adoption record). These are the director
tests the automated suite cannot grade: whether the guardrail's block reads as an
instruction, whether the live tracker read tells the truth about a real repo, and
whether the CI backstop is honest about what it could not verify. Surfaces:
`vsdd gate` (bare, the 0/1/2 contract), `vsdd gate --ci`, `vsdd status`, and the
`routing-gate.yml` workflow. Outcomes are recorded on the slice's tracker trail.

1. **A fix with no routing blocks, and the block reads as an instruction.** Open a
   scratch finding issue, close it with a `result` comment that cites a commit,
   and file no `plan` comment. Run `vsdd gate`. Expect: exit 1; the diagnostic
   names the finding by handle, says what routing is missing (a `plan` comment
   naming a target phase or the fix lane), and names the one act that unblocks.
   Judge: could a tired human do the unblock from the message alone?
2. **A routed fix passes on its merits.** Add the `plan` comment to the same
   finding; run `vsdd gate`. Expect: exit 0 and a line saying which findings were
   checked — never silence.
3. **A disposition closure is exempt, and says so.** Close a scratch finding with
   a disposition (`dismissed` / the schema token `hallucinated` / `consolidated`)
   and no routing. Expect: exit 0; the exemption is stated, not silent.
4. **Tracker unreachable is unverifiable, never clean.** Point `crosslink` at an
   unreachable hub (or stop the daemon) and run `vsdd gate --ci`. Expect: exit 2
   with "UNVERIFIABLE (fail-closed)" and the reason; on the human form, the
   next step. Then run bare `vsdd gate` locally: the per-commit posture fails
   open with a recorded advisory (a tracker outage never blocks all commits) —
   confirm the advisory is actually written somewhere a human will see.
5. **The universe boundary holds.** Take a finding that closed by fix before the
   ratification boundary (any June-cycle finding) with no routing. Expect: not
   counted — the gate reports the universe it scanned (at-or-after the boundary)
   so the exclusion is visible, not silent.
6. **Live data, not fixture data.** With the real tracker reachable, run
   `vsdd status` and compare its finding/round counts against `crosslink issue
   list -l review -s all` by hand for one round. Expect: the numbers agree, or
   the disagreement is reported as an integrity finding, never absorbed.
7. **The CI backstop is a landing block, not a post-hoc red.** Push a branch
   carrying the unrouted-fix condition from item 1 and open a pull request.
   Expect: the `routing-gate` required check fails and the merge button is
   blocked by the ruleset — verify the block on the PR page, not in the log.
8. **Color-strip and assistive read.** Run items 1 and 4 in a no-color terminal
   and read them with the named screen reader on the operator's checklist slot:
   no information lost; the diagnostics read as complete sentences.
9. **Deviations gate leg.** Edit `.vsdd/registry/deviation-registry.yaml` to give
   a standing entry an expiry in the past; run `vsdd gate --ci`. Expect: a loud
   failure naming the entry id and the lapsed date. Restore the file; expect clean
   (or the known local-oracle "undecidable" line, which must say why).
