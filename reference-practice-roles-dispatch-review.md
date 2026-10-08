---
title: "Reference practice: roles, dispatch shape and review conduct"
tags: ["design-input", "process", "dispatch", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Reference practice: roles, dispatch shape and review conduct

## Status

Design input for the phase-skill rewrite and the recorded-dispatch milestone design. Who does what, how work is dispatched, and what review means in Thermite, Peritus, OpenClaudia and crosslink, reconstructed read-only on 2026-10-08 (index: `vsdd-in-practice-reference-repositories-2026-10-08`; evidence: the four `practice-report-*` pages).

Revised 2026-10-08, the day it was published, under the operator's vocabulary and citation decisions of that day: "phase skill" replaces "primer", "rules file" replaces "supplement", "reviewer role" replaces "domain prompt", milestones are named by feature instead of "Slice N", "increment" is the unit of work dispatched as one issue, and "oracle" is kept only for the expected results the operator authors (what the verifier reads is the synced trace; what a review produces is a verdict record). Words quoted from a source keep the source's words. The decisions are recorded on `vsdd-cli#839` and on `terminology-grounding-2026-10-08`. Handles cited on this page are listed with their titles at the end.

Milestones are named by feature. On the tracker (`crosslink milestone list`) they are: the live self-governance milestone, #8 "Slice 1 — Live self-governance"; the composition milestone, #9 "Slice 2 — Composition, generated context, and static price"; the install milestone, #10 "Slice 3 — Install"; the gate-execution milestone, #11 "Slice 4 — Gate execution and the mutation floor"; the finding-lifecycle milestone, #12 "Slice 5 — Finding lifecycle and conformance"; the recorded-dispatch milestone, #13 "Slice 6 — Recorded dispatch and directive flow"; the cost milestone, #14 "Slice 7 — The cost crate".

## The common shape

A root session writes or freezes the design, creates one tracker issue per increment with the dispatch prompt in its body, dispatches a worker into an isolated worktree, runs the gates itself, commits with a signature, and hands the push to the human. Review is a separate read-only dispatch over the exact tree. CI is the second reviewer. The human approves, pushes, merges, provisions secrets and redirects.

## The roles, by repository

**Thermite.** Four Claude Code agent types in the repository, each with a tool allowlist and a report word cap:

- *doc-author* (no edit tool; writes only under the design folder; 400-word report): authors or backfills a component contract; "the doc adapts to the code" for backfill, never for new behavior.
- *builder* (edit, write, shell; 800 words): ships a whole design-governed component that does not exist, under a pre-declared manifest of about ten files; stops and reports when a file outside the manifest is needed.
- *critic* (no edit tool; 700 words): the eight-step audit cycle: read the deliverable, read the contract sources, catalogue divergence candidates, build the smallest failing test per candidate, verify it fails, file a tracking issue per divergence, mark the test with the issue, report. No tautological tests. Verdicts: "generator must fix" or "no divergence found".
- *fixer* (edit; 500 words): exactly one pinned divergence, smallest edit, single file, no renames or restructuring; revert if the gauntlet fails; followed by a critic re-audit.
- *orchestrator*: the interactive session under the locked goal statement; dispatches in dependency order ("do not ask which; the dependency graph is the answer"); from mid-June, kickoff worktrees with the orchestrator pushing, watching checks and merging on green.

**Peritus.** A root integrator agent (84 percent of 3,300 hub comments; writes designs, freezes boundaries, dispatches, owns all shared files and every build and verify command, signs commits, opens and merges pull requests, re-runs the gate on fresh main, writes changelog pull requests); path-scoped workers of a named model and effort ("Sol xhigh"), two or three per slice from a written ownership table, concurrency bounded to three, stopped before the first build because builds are never concurrent; read-only reviewers of a different model family registered in an actor file with provenance, one fresh identity per formally governed record; the human (approvals, the push early on, secrets, redirections: "User explicitly redirected delivery to Sol xhigh subagents for P1-P7"); bounded assignment agents for releases, audits and campaigns. No separate fixer: the owning worker repairs.

**OpenClaudia.** One driver agent that implemented most of 103 slices alone in 14 days; nine waves of three workers chosen by a pre-wave overlap audit; after a plugin hook launched a repository-wide lint inside a worker and interrupted all of them, workers became code-only ("no Cargo, builds, tests, Clippy, fmt, runners, Git commits, or cleanup; the parent inspects every diff, reconciles overlaps, runs resource-bounded verification serially, fixes concrete failures, commits, and monitors exact-head CI"); the orchestrator in a fresh context as the reviewer; the human owns the push.

**crosslink.** One implementer per issue; a root architect session that writes the pre-flight comment and later runs the independent audit "and will not monitor CI"; a driver identity that signs hub commits with the human's key; the human owns the final push. The tool's own swarm, sentinel and kickoff-report loop were not used on the tool itself.

## Dispatch mechanics that recur

- **The issue body is the prompt.** Authoritative spec by requirement and criterion; sequencing document; predecessors by pull request; loop discipline; self-verify commands; stop rule on budget.
- **Ceilings are explicit and recorded:** three workers; serialized builds with one build job and one test thread; swap reset before dispatch; manifests of about ten files; locks per issue with stale-lock stealing off.
- **Workers cannot push.** Blocked by the hook; recorded as intervention or blocker comments ("Repository policy permanently forbids the agent from performing the sole remaining push, so external human action is required").
- **Fan-in:** one integration commit per wave (OpenClaudia) or one pull request per increment (Thermite); merge on green with a monitor polling the pull-request checks.
- **Parallelization rule:** independent units are dispatched in one message; fixers serialize per blocker; critics parallelize; a critic runs only after substantive builds, not after cite refreshes or fixture bumps.
- **The dispatch record** is the hub event log (issue created, label, lock claimed, plan, result, lock released, handoff); no manifest files survive in Thermite or crosslink; Peritus's formal records are the exception.

## What "adversarial" meant

| | Reviewer | Freshness | Output | Stop rule |
|---|---|---|---|---|
| Thermite | critic agent type, no edit tool, same model family | separate dispatch; must re-read the authority | failing test plus blocker issue plus observation comment; no prose verdict | "no divergence found" after a loop "until clean" |
| Peritus | read-only subagent, different model family | distinct principal per formal record; "fresh-context digest" required at release | typed findings (severity, blocking, disposition) in a content-addressed verdict with a mandatory report | zero blocking findings and every gate passed; bounded re-check by the same reviewer |
| OpenClaudia | the orchestrator in a fresh context | asserted, not retained for ordinary slices | written changes-required or pass verdict with enumerated dimensions | review, corrections, one re-review, pass |
| crosslink | architect or audit session in the build | same session tree | redirect rounds and an audit comment | "independent audit PASS" |

Review dimensions are enumerated in the receipt (OpenClaudia's first slice: final-environment graders, success and failure coverage, multi-trial design, trace assertions, effect-observation assertions, artifact bounds and isolation). The reviewer is forbidden to edit in every repository.

## Findings, filing and closure

- A finding becomes a tracker issue immediately: title an imperative sentence or "Divergence: <symbol> <claim>"; label blocker or remediation; linked to the parent.
- Fixed in a dedicated commit by the owning worker; closed with a result comment; the changelog line generated on close and committed unchanged.
- Dispositions are a closed set: fixed, invalid, superseded (Peritus records); fixed, disputed, superseded, waiver-requested (the Peritus product's own engine).
- Waiver policies exist (Peritus release policy: only non-release-blocking findings, approved by an authority other than the reporter); no granted waiver was observed in any repository.
- Genuinely cold reviews are rare and external: a trust audit of Thermite at a named revision with a reconciliation table; an ontological review of Peritus after months of code; two outside-filed issues.

## Divergences from the whitepaper

The whitepaper's roles (human architect, builder, tracker, adversary) map loosely: human as user, driver or operator; builder as the kickoff implementer or path-scoped worker; the hub as tracker; the adversary as a critic agent type or a second session. No Solution Owner, no domain-reviewer roster, no pair separation, no validator-differs-from-owner rule, no refutation fan-out, no persona exist in any repository.

## What to take (candidates)

- Four roles with tool allowlists and report caps, carried as agent definitions in the repository, are the attested packaging of the method's disciplines.
- The dispatch prompt lives in the issue body and names the spec, the predecessors, the loop, the self-verify commands and the stop rule.
- One read-only reviewer per round, a different model family where available, typed findings, a bounded re-check, CI as the second reviewer, a written stop rule.
- Ceilings in the record: worker count, build concurrency, manifest size.

## Handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-08.

- `vsdd-cli#839`: Slice 2 (Composition) phase-1a design — vsdd way: composition function + config-integrity ... [open]
