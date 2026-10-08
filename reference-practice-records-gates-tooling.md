---
title: "Reference practice: records discipline, gates and tooling"
tags: ["design-input", "process", "reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Reference practice: records discipline, gates and tooling

## Status

Design input for the phase-skill rewrite and for the gate-execution and finding-lifecycle milestones. What the reference repositories record, where, and which mechanical gates they run, reconstructed read-only on 2026-10-08 (index: `vsdd-in-practice-reference-repositories-2026-10-08`; evidence: the four `practice-report-*` pages).

Revised 2026-10-08, the day it was published, under the operator's vocabulary and citation decisions of that day: "phase skill" replaces "primer", "rules file" replaces "supplement", "reviewer role" replaces "domain prompt", milestones are named by feature instead of "Slice N", "increment" is the unit of work dispatched as one issue, and "oracle" is kept only for the expected results the operator authors (what the verifier reads is the synced trace; what a review produces is a verdict record). Words quoted from a source keep the source's words. The decisions are recorded on `vsdd-cli#839` and on `terminology-grounding-2026-10-08`. Handles cited on this page are listed with their titles at the end.

Milestones are named by feature. On the tracker (`crosslink milestone list`) they are: the live self-governance milestone, #8 "Slice 1 — Live self-governance"; the composition milestone, #9 "Slice 2 — Composition, generated context, and static price"; the install milestone, #10 "Slice 3 — Install"; the gate-execution milestone, #11 "Slice 4 — Gate execution and the mutation floor"; the finding-lifecycle milestone, #12 "Slice 5 — Finding lifecycle and conformance"; the recorded-dispatch milestone, #13 "Slice 6 — Recorded dispatch and directive flow"; the cost milestone, #14 "Slice 7 — The cost crate".

"This estate" means vsdd-cli together with crosslink and mdatron, the two sibling tools it runs on.

## Where things live

- **The hub is the work ledger; GitHub is intake and CI.** crosslink's maintainer authored zero GitHub issues in the current repository; all 90 are outside reports. Peritus's 16 GitHub issues are user reports; 825 hub issues carry the work. Thermite splits by audience: the hub owns increments, pins and day-to-day discipline; GitHub owns RFCs, gate announcements, umbrellas and human-found defects.
- **Comment kinds, by volume:** result dominates everywhere (738 of 1,503 in crosslink; 956 of 3,300 in Peritus; 392 of 1,361 in OpenClaudia's driver log), then note or plan, then decision (767 in Peritus; 80; 28; 6 in Thermite), handoff, observation, intervention. OpenClaudia's first-wave workers used a dense key-value receipt grammar inside comments.
- **Interventions record blocked commands faithfully**, including hook false positives repeated by four agents over three months until the command was allow-listed.
- **Knowledge pages** hold process lore and mirrors of program documents: Thermite's kickoff-orchestration operations page records the orchestrator and agent split, merge-on-green, the two drift-pin kinds, the registry union conflict on parallel kickoffs, a stale signing-key failure and its fix, and the gate discipline. crosslink's "every validated design becomes a knowledge page" step ran for two of five designs.

## Commit and pull-request forms

- **Subjects:** conventional type, scope and imperative, with the tracker id in parentheses (446 of 813 Peritus subjects; 170 crosslink lines). Thermite's June form: "<crate>: <area> — <summary> (closes #N)", "<crate>: critic — pin ...", "<crate>: #N fix — ...".
- **Bodies:** a verification paragraph with integer counts ("Full workspace: 3687 passed, 0 failed, 19 ignored"); Thermite's June template with design sources, requirement status and verification sections (188 and 146 of 698 bodies); a root cause, changes and verified structure in crosslink's spring. Later eras moved verification detail into pull-request bodies and result comments and dropped attribution trailers.
- **Signed commits whose signature verification is itself logged** in the hub result ("signature=verified SSH; key ...").
- **Pull-request bodies:** Summary and Verification with exact commands, counts and run ids; Thermite's kickoff pull requests add Delivered, Adversarial verification and Gauntlet (local) with "Tracking: crosslink #N"; crucible's template adds Motivation and evidence, Scope and compatibility, Verus and trusted-boundary impact, AI assistance, Security and privacy, and a checklist. Honesty markers recur ("the Lean-spine tests were skipped locally; the CI job is the real gate"; "README headline flip explicitly not done here").
- **Two-commit cadence per slice** in OpenClaudia: the implementation, then the machine-generated changelog line committed unchanged ("Generated CHANGELOG content was not hand-edited"); duplicate lines from double closes kept deliberately. Thermite curates its changelog only at gate time and agents close issues without changelog entries.

## Status wording and receipts

- **Three evidence classes, never upgraded in prose.** OpenClaudia's 108 status lines use distinct families: "Implemented and adversarially reviewed; verifier receipt pending", "Implemented and deterministically verified; receipt pending", "Complete", "Implemented, awaiting verification"; none claims a receipt that does not exist ("no such receipt is fabricated"). Peritus's obligations file: "Nothing in this file claims discharge until a registered independent reviewer approves."
- **Receipts are digests.** SHA-256 over sorted file manifests or diff ranges (48 of 108 OpenClaudia slices); generation identifiers invalidated by any mutation ("any change to the listed source or test artifacts invalidates it"); bound to the CI run on the exact head; failed attempts retained ("so the receipt does not hide failed attempts or substitute targeted results for the full gate").
- **Generated capability documentation that refuses to promote.** OpenClaudia's capability registry generates a matrix whose header says "Do not edit this table by hand: prose is not readiness evidence", and every route still reads "not operational".

## The gate catalogue

| Gate | What it checks | Where | When |
|---|---|---|---|
| Issue-before-edit | pre-tool hook blocks edits and commands without an active tracker issue; mutating version-control commands denied to agents; stub patterns scanned after each edit; rules injected per prompt | all four (crosslink hooks) | every tool call |
| Read-before-edit over a route table | each governed source file maps to its design doc and reference; an edit blocks until the doc exists and was read this session; the block message embeds the doc-author dispatch prompt | Thermite, ferrotorch | pre-edit |
| Anti-pattern gate | stubs, unwraps, panics, root-level lint suppressions, shared-mutable wrappers outside tests; override per item with a reason and a tracker comment | Thermite, ferrotorch | pre-edit |
| Document drift | every routed design doc pins a content digest over its governed files; CI fails when the code moves under the doc; clearing is a conscious re-pin or amendment; not part of the proof audit by decision | Thermite | CI and a task target |
| Control plane | the hook wiring itself (settings file, agent files) is pinned and checked, after both agent-facing gates were dormant five weeks following a tool re-initialization "while README, goal.md and all four agent files kept asserting they fire" | Thermite | CI |
| Requirements registry | stale generated views fail; shipped requires file, symbol or test evidence; blocked requires an open blocker; partial requires remaining scope | Thermite | CI |
| Architecture as policy | layer dependency rules, verification classes, owner slices, line limits, forbidden module names, owned exceptions with rationale | Peritus | before every signed commit and in the hosted gate |
| Reproducibility policy | canonical workflow copies, the ruleset template, task runner, dependency policy, lint pins and timeouts compared byte for byte | Peritus | CI |
| Review ledger | actor registry with provenance; change records with raw-byte fingerprints; detached signed verdicts; authorize-then-apply in two pull requests so a source edit cannot approve its own review record | Peritus | formally governed changes |
| Formal verification | proofs per slice under a no-cheating flag with an empty trusted baseline; an axiom allowlist probe; correspondence drift tripwires | Peritus, Thermite | CI, every pull request |
| Generated agent-facing spec with a budget | the language definition generated from the implementation's registries, held to 6,000 tokens | Thermite | CI |
| Backlog integrity | exactly-once finding ownership, acyclic dependencies, size bounds; rerun on any change to ownership or count | OpenClaudia | by convention on each backlog edit |
| Capability evidence registry | a capability cannot be marked operational without executable receipts for its entrypoints and failure modes | OpenClaudia | tests |
| Hosted gate aggregator | one required status composed of every job, fail-closed, bound to the ruleset; repository ruleset requires a pull request, no bypass, zero approvals | Peritus | every pull request |

**The lesson the control-plane gate encodes:** anything load-bearing for a trust claim must be CI-enforced and harness-agnostic; authoring-time tooling may never be cited as the reason a property holds. Local hooks are friction an agent can route around; CI and server-side rulesets are the blocks.

## Grades, as the references themselves state them

- Hook-level gates are friction (an agent can run outside the hook; a re-initialization can strip them).
- CI legs and rulesets are mechanical blocks; a required status composed fail-closed is the strongest single control observed.
- Status wording and receipts are detective and honest by convention, enforced only where a generator refuses to promote.
- An accepted residual gap is documented where it exists (Peritus: a candidate that edits both the workflow and the checker in one pull request can weaken the gate on the hosting plan).

## What to take (candidates)

- Issue-before-edit and read-before-edit as the two pre-edit gates; drift pinned to code, not to another document.
- A requirements record with typed evidence and a generated, CI-checked status view.
- Three evidence classes in every status line; receipts as digests bound to the run on the exact head; failed attempts kept.
- A control-plane check over the hook wiring, since the install-manifest check is this estate's analog.
- Verification detail in pull-request bodies and result comments with counts and run ids; changelog lines generated and committed unchanged.

## Handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-08.

- `vsdd-cli#839`: Slice 2 (Composition) phase-1a design — vsdd way: composition function + config-integrity ... [open]
