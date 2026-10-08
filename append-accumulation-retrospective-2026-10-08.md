---
title: "Append-accumulation retrospective (2026-10-08)"
tags: ["retrospective", "design-input", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Append-accumulation retrospective (2026-10-08)

## Status

Retrospective, recorded as design input for the regime decision and the composition milestone design. Why rulings accumulated as appends instead of integrating into a current design, found during the reconciliation of 2026-10-08 (see `reconciliation-ledger-2026-10-08`). Numbers are from the repository and tracker at main 609f9132. Rulings on what to change are the operator's; none is taken here.

Revised 2026-10-08, the day it was published, under the operator's vocabulary and citation decisions of that day: "phase skill" replaces "primer", "rules file" replaces "supplement", "reviewer role" replaces "domain prompt", milestones are named by feature instead of "Slice N", "increment" is the unit of work dispatched as one issue, and "oracle" is kept only for the expected results the operator authors (what the verifier reads is the synced trace; what a review produces is a verdict record). Words quoted from a source keep the source's words. The decisions are recorded on `vsdd-cli#839` and on `terminology-grounding-2026-10-08`. Handles cited on this page are listed with their titles at the end.

Phase names on this page are the contract's: 1a behavioral specification, 1b verification architecture, 1c the spec review gate, 2a test-suite generation (the red gate), 2b minimal implementation, 2c refactor, 3 adversarial refinement, 4 the feedback integration loop, 5 formal hardening, 6 convergence. The whitepaper has six phases; the a, b and c splits are this repository's.

Milestones are named by feature. On the tracker (`crosslink milestone list`) they are: the live self-governance milestone, #8 "Slice 1 — Live self-governance"; the composition milestone, #9 "Slice 2 — Composition, generated context, and static price"; the install milestone, #10 "Slice 3 — Install"; the gate-execution milestone, #11 "Slice 4 — Gate execution and the mutation floor"; the finding-lifecycle milestone, #12 "Slice 5 — Finding lifecycle and conformance"; the recorded-dispatch milestone, #13 "Slice 6 — Recorded dispatch and directive flow"; the cost milestone, #14 "Slice 7 — The cost crate".

"This estate" means vsdd-cli together with crosslink and mdatron, the two sibling tools it runs on.

## The symptom

Thirty-nine divergences between recorded decisions and the contract, build-plan, data sets or register, across all seven milestones: 26 recorded nowhere as an open item for their milestone, 13 recorded on another milestone's or a closed issue. Two were found by the operator (the session-start hook's home; the crate collapse) before the sweep; the rest by the sweep. Examples: a Solution Owner ruling of 2026-10-01 assigning the state artifact to the composition milestone with a register exemption, neither reaching `vsdd-cli#839` nor the register; two rulings on the conformance evidence with two different homes, neither recording the other; four register entries expiring within seven weeks, two owned by closed issues.

## The measured context

| Measure | Value |
|---|---|
| Commits to main since the live self-governance milestone merged (2026-08-02) | 30, none of type feature |
| Production code changed in that window | 452 lines added, 51 removed |
| Governed text changed | 1,326 added, 948 removed |
| Contract amendments in the revision history | 5 |
| Rulings as tracker comments in the same window | dozens (four on `vsdd-cli#839` in two weeks) |
| the live self-governance milestone, ratified design to shipped build | 4 days |
| the composition and gate-execution milestones, ratified design to build start | not started after 68 and 70 days |
| Commits by week | 5 in the week of 1 August; none for six weeks; 25 from 18 September to 8 October |

The six empty weeks were a break spent on a sibling tool. The three active weeks were re-entry: adopting two sibling releases, the readiness model, the container pilot, the kickoff vehicle and swarm findings, then the compaction and rename the return prompted. Twenty-five commits of catch-up, each a governance cycle with rulings, and the build was never reached.

## The mechanism

Four of the estate's own rules interact to produce appends:

1. **Directive reconciliation** requires every ruling to be recorded at receipt. Appending is cheap and mandatory.
2. **Solution Owner change authority** requires an owned composition, a cold review and a ratification to change the contract. Integrating is expensive: five amendments in the window, each costing a review round (two reviewers read the whole contract twice at the compaction, 564k tokens).
3. **Attended decisions freeze.** Every recorded ruling is immediately binding, integrated or not.
4. **The 2026-08-02 unification** ruled that `.design/` holds the contract and the build-plan only, retiring the milestone designs to knowledge pages. That removed the one middle-sized artifact where a milestone's rulings could have been integrated without a full contract amendment. The build-plan was meant to be that projection but is a paragraph per bullet and defers to "the retired page".

So a ruling has exactly one cheap home, the tracker, and it lands on whichever issue was open at the time.

Two further causes:

5. **The contract binds to sibling-tool specifics in normative text** (crosslink swarm as the dispatch vehicle, the crosslink session-start hook, kickoff's container mode, the hub sync channel as the conformance evidence, conformance families by name). When a sibling moves, the contract must move through the full amendment route. The contract states the right principle in one place ("how the dials are set on each vehicle is catalogued in the runtime-harness rules file and the paved-path map, never here") and violates it elsewhere; the Solution Owner reviewer said so on 2026-10-02. Of the 39 ledger items, the hook home, the kickoff block, the evidence channel, the swarm residue and the register expiries all come from sibling specifics in the contract.
6. **The handoff overwrite.** A re-scoping mid-flight wrote no residual, and the next session's "next" line replaced the folded-in item. The crate merge vanished on 2026-07-29 in two comments hours apart: "folds in at 2b entry", then "separate design, not crammed into the live self-governance milestone", then nothing.

## The prior fixes, and why they did not hold

Six design fixes in ten weeks, each a full cycle (owned composition, cold review, ratification, merge), each repairing the artifact and leaving the recording rules unchanged:

- `vsdd-cli#819` (28 July): horizontal layers re-cut into vertical slices so something would ship early. It worked once: the live self-governance milestone.
- `vsdd-cli#826` (29 July): the superseded binary-first plan removed; issues re-grounded.
- `vsdd-cli#860` (2 August): the contract made the single spec; milestone designs retired to knowledge pages. The one that removed the cheap integration home.
- `vsdd-cli#873` (18 September): the contract compacted from 256 KB to 103 KB after a deletion-test sweep.
- `vsdd-cli#874` (24 September): the corpus renamed to the ratified vocabulary.
- `vsdd-cli#895` (6 October): structure rules and pin, link and marker checks so the build-plan cannot drift silently.

Each consumed the build window, each produced its own rulings, and the mess was rebuilt by the same hands that cleaned it. The agent session is part of the mechanism: it arrives with a handoff, finds what is wrong, records it, asks for rulings, records those, and hands off; every step is rewarded by the process, and nothing penalizes a session that ends with more records and no code.

## What the reference repositories do differently

Measured on 2026-10-08 (see `reference-practice-design-documents-and-estate-divergences`): an umbrella frozen within days plus per-slice documents written at build time and amended during it, with decisions at the question; idea cycles as their own documents; requirements in a registry with typed evidence; drift pinned to code; design and build in the same window. Thermite's own RFC-5 diagnoses the issue-as-document failure in the same words this estate's provenance rule prescribes.

## Levers (candidates, not decisions)

- Cut the milestones to the live self-governance milestone's size; design and build each within days.
- Give each open milestone a live design document as the integration home for its rulings; the contract becomes the umbrella.
- When a sibling moves, the change lands as data (a pin, a map entry, a rules file line), not an amendment cycle, once the sibling bindings are out of the contract. The composition milestone's phase-1a already has to touch three such passages for the hook wording.
- A reconciliation ledger as the phase-1a entry act, read-only, one session, producing one batched ask.
- Rulings recorded on the owning milestone's issue; a re-scoping that drops a folded-in item writes its residual at that moment.
- A fourteen-day expiry warning on register entries in the status command.

The scope question that sits above these: whether the Verifiable conformance and efficiency member, which generated about a third of the ledger and the most cycles, stays in this program's build or becomes the next program's.

## Handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-08.

- `vsdd-cli#819`: Phase-1c re-decomposition: horizontal component layers -> vertical capability slices ... [closed]
- `vsdd-cli#826`: clean up superseded binary-first-plan (docs/refactor legacy); re-ground #14/#15 to the slice ... [closed]
- `vsdd-cli#839`: Slice 2 (Composition) phase-1a design — vsdd way: composition function + config-integrity ... [open]
- `vsdd-cli#860`: Design unification: consolidate the contract — fold #840 subsystem + retire #845 doc, apply ... [closed]
- `vsdd-cli#873`: contract compaction: deletion-test sweep bins, reference conventions, naming map, primer ... [closed]
- `vsdd-cli#874`: Governed-corpus rename (Part 2 follow-on to the compaction): apply the ratified naming map + ... [closed]
- `vsdd-cli#895`: Land the link, pin and marker families and two build-plan structure rules early, with the ... [closed]
