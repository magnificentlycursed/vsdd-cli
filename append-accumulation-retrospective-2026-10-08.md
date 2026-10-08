---
title: "Append-accumulation retrospective (2026-10-08)"
tags: ["retrospective", "design-input", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---


## Design Specification

### status

Retrospective, recorded as design input for the regime decision and the Slice 2 design. Why rulings accumulated as appends instead of integrating into a current design, found during the reconciliation of 2026-10-08 (see `reconciliation-ledger-2026-10-08`). Numbers are from the repository and tracker at main 609f9132. Rulings on what to change are the operator's; none is taken here.

### the symptom

Thirty-nine divergences between recorded decisions and the contract, build-plan, data sets or register, across Slices 1 to 7: 26 recorded nowhere as an open item for their slice, 13 recorded on another slice's or a closed issue. Two were found by the operator (the session-start hook's home; the crate collapse) before the sweep; the rest by the sweep. Examples: a Solution Owner ruling of 2026-10-01 assigning the state artifact to Slice 2 with a register exemption, neither reaching #839 nor the register; two rulings on the conformance oracle with two different homes, neither recording the other; four register entries expiring within seven weeks, two owned by closed issues.

### the measured context

| Measure | Value |
|---|---|
| Commits to main since Slice 1 merged (2026-08-02) | 30, none of type feature |
| Production code changed in that window | 452 lines added, 51 removed |
| Governed text changed | 1,326 added, 948 removed |
| Contract amendments in the revision history | 5 |
| Rulings as tracker comments in the same window | dozens (four on #839 in two weeks) |
| Slice 1, ratified design to shipped build | 4 days |
| Slices 2 and 4, ratified design to build start | not started after 68 and 70 days |
| Commits by week | 5 in the week of 1 August; none for six weeks; 25 from 18 September to 8 October |

The six empty weeks were a break spent on a sibling tool. The three active weeks were re-entry: adopting two sibling releases, the readiness model, the container pilot, the kickoff vehicle and swarm findings, then the compaction and rename the return prompted. Twenty-five commits of catch-up, each a governance cycle with rulings, and the build was never reached.

### the mechanism

Four of the estate's own rules interact to produce appends:

1. **Directive reconciliation** requires every ruling to be recorded at receipt. Appending is cheap and mandatory.
2. **Solution Owner change authority** requires an owned composition, a cold review and a ratification to change the contract. Integrating is expensive: five amendments in the window, each costing a review round (two reviewers read the whole contract twice at the compaction, 564k tokens).
3. **Attended decisions freeze.** Every recorded ruling is immediately binding, integrated or not.
4. **The 2026-08-02 unification** ruled that `.design/` holds the contract and the build-plan only, retiring the slice designs to knowledge pages. That removed the one middle-sized artifact where a slice's rulings could have been integrated without a full contract amendment. The build-plan was meant to be that projection but is a paragraph per bullet and defers to "the retired page".

So a ruling has exactly one cheap home, the tracker, and it lands on whichever issue was open at the time.

Two further causes:

5. **The contract binds to sibling-tool specifics in normative text** (crosslink swarm as the dispatch vehicle, the crosslink session-start hook, kickoff's container mode, the hub sync channel as the oracle, conformance families by name). When a sibling moves, the contract must move through the full amendment route. The contract states the right principle in one place ("how the dials are set on each vehicle is catalogued in the runtime-harness supplement and the paved-path map, never here") and violates it elsewhere; the Solution Owner reviewer said so on 2026-10-02. Of the 39 ledger items, the hook home, the kickoff block, the oracle, the swarm residue and the register expiries all come from sibling specifics in the contract.
6. **The handoff overwrite.** A re-scoping mid-flight wrote no residual, and the next session's "next" line replaced the folded-in item. The crate merge vanished on 2026-07-29 in two comments hours apart: "folds in at 2b entry", then "separate design, not crammed into Slice 1", then nothing.

### the prior fixes, and why they did not hold

Six design fixes in ten weeks, each a full cycle (owned composition, cold review, ratification, merge), each repairing the artifact and leaving the recording rules unchanged:

- #819 (28 July): horizontal layers re-cut into vertical slices so something would ship early. It worked once: Slice 1.
- #826 (29 July): the superseded binary-first plan removed; issues re-grounded.
- #860 (2 August): the contract made the single spec; slice designs retired to knowledge pages. The one that removed the cheap integration home.
- #873 (18 September): the contract compacted from 256 KB to 103 KB after a deletion-test sweep.
- #874 (24 September): the corpus renamed to the ratified vocabulary.
- #895 (6 October): structure rules and pin, link and marker checks so the build-plan cannot drift silently.

Each consumed the build window, each produced its own rulings, and the mess was rebuilt by the same hands that cleaned it. The agent session is part of the mechanism: it arrives with a handoff, finds what is wrong, records it, asks for rulings, records those, and hands off; every step is rewarded by the process, and nothing penalizes a session that ends with more records and no code.

### what the reference repositories do differently

Measured on 2026-10-08 (see `reference-practice-design-documents-and-estate-divergences`): an umbrella frozen within days plus per-slice documents written at build time and amended during it, with decisions at the question; idea cycles as their own documents; requirements in a registry with typed evidence; drift pinned to code; design and build in the same window. Thermite's own RFC-5 diagnoses the issue-as-document failure in the same words this estate's provenance rule prescribes.

### levers (candidates, not decisions)

- Cut the slices to Slice 1's size; design and build each within days.
- Give each open slice a live design document as the integration home for its rulings; the contract becomes the umbrella.
- When a sibling moves, the change lands as data (a pin, a map entry, a supplement line), not an amendment cycle, once the sibling bindings are out of the contract. Slice 2's phase-1a already has to touch three such passages for the hook wording.
- A reconciliation ledger as the phase-1a entry act, read-only, one session, producing one batched ask.
- Rulings recorded on the owning slice's issue; a re-scoping that drops a folded-in item writes its residual at that moment.
- A fourteen-day expiry warning on register entries in the status command.

The scope question that sits above these: whether the Verifiable conformance and efficiency member, which generated about a third of the ledger and the most cycles, stays in this program's build or becomes the next program's.

