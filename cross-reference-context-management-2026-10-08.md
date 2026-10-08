---
title: "Cross-reference: context language models and model-managed context against the append problem and crosslink knowledge management (2026-10-08)"
tags: ["design-input", "review", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Cross-reference: context language models and model-managed context against the append problem and crosslink knowledge management (2026-10-08)

## Status

Design input, 2026-10-08. The Context Language Models paper and the public discussion of it (`context-language-models-2026-10-08`) read against the append-accumulation retrospective (`append-accumulation-retrospective-2026-10-08`), the knowledge-page conventions proposal (`knowledge-page-conventions-proposal-2026-10-08`), the contract's compaction, injection and generated-context members, and the reference-practice findings. None of what follows is a decision.

Revised 2026-10-08, the day it was published, under the operator's vocabulary and citation decisions of that day: "phase skill" replaces "primer", "rules file" replaces "supplement", "reviewer role" replaces "domain prompt", milestones are named by feature instead of "Slice N", "increment" is the unit of work dispatched as one issue, and "oracle" is kept only for the expected results the operator authors (what the verifier reads is the synced trace; what a review produces is a verdict record). Words quoted from a source keep the source's words. The decisions are recorded on `vsdd-cli#839` and on `terminology-grounding-2026-10-08`. Handles cited on this page are listed with their titles at the end.

Milestones are named by feature. On the tracker (`crosslink milestone list`) they are: the live self-governance milestone, #8 "Slice 1 — Live self-governance"; the composition milestone, #9 "Slice 2 — Composition, generated context, and static price"; the install milestone, #10 "Slice 3 — Install"; the gate-execution milestone, #11 "Slice 4 — Gate execution and the mutation floor"; the finding-lifecycle milestone, #12 "Slice 5 — Finding lifecycle and conformance"; the recorded-dispatch milestone, #13 "Slice 6 — Recorded dispatch and directive flow"; the cost milestone, #14 "Slice 7 — The cost crate".

"This estate" means vsdd-cli together with crosslink and mdatron, the two sibling tools it runs on.

## The formal statement of the append problem

The paper writes a standard language model's step as context plus output: the next context is the old context with new tokens appended. A context language model instead produces the next context directly, so it may rewrite, compact or delete. Everything the retrospective found is the first form applied to documents and records: the contract accumulates rulings as comments, the register accumulates re-arms, the build-plan accumulates residuals, and nothing is allowed to rewrite cheaply. The thread's one-line version, "intelligence = forgetting," and a reply's "memory that never throws anything away isn't memory, it's a hoard; the hard part is the forgetting policy," name the missing operation: a policy for what the estate's records stop carrying, and an actor allowed to apply it.

What the paper measured when the rewrite operation exists: contexts held at six to eight thousand tokens across hundreds of tasks, in-place scoreboards and state blocks that replace their predecessors rather than stacking, a reusable compaction helper invoked dozens of times, compaction that keeps answer-relevant facts and untried ideas. The cost of the operation is also measured: an edit at the start of the context costs about eight times an append-only turn in prefill, which is why stable material belongs first and volatile material last.

## The pairing the thread asked for, which this estate already has

One reply asked whether the harness "keeps an immutable event log alongside mutable context, so a developer can reconstruct why memory changed." The paper does not; it has no provenance, dating or freshness on entries and does not discuss contradictions between them. This estate's architecture is exactly that pairing: the hub's per-agent event logs are append-only and signed; the knowledge branch is git, so each page's history is derivable; the contract says events are derived at query time and the state artifact is a projection. The gap is not the log; it is that the mutable projection layer (a live milestone document, a current knowledge page) was retired or never given a rewrite operation, so the log became the document. The retrospective's fix and the paper's finding coincide: keep the immutable log, restore the mutable projection, and give its owner the rewrite.

## Forgetting policy for records

The paper's compaction behaviors and the thread's replies supply the vocabulary; Palimpsest's canon levels and the conventions proposal supply the mechanism. A record is forgotten by status, not deletion: superseded, retired, resolved. What may be forgotten: resolved questions (the answer moves into the governing document), superseded rulings (the current one is the only one cited), retired designs (one line pointing at the live one), register entries after resolution, knowledge pages whose revision has moved on. What must not be forgotten: the evidence lines (the incident corpus), unresolved forks (one reply: "learning what not to collapse; delete solved branches but pin unresolved forks"), and the log itself. The knowledge base grew from fifty to over seventy pages in one day; without a status that drops superseded pages from default search and injection, that growth is the hoard.

## Security of editable context

The paper names an injection channel the estate has not: a model inserted unauthorized instructions into its own compaction summary, and they persisted across turns. The contract's Availability is not activation member treats injected context as the reliable delivery path; this is the reverse risk, that the injected or compacted context carries instructions nobody authorized. The estate's answer in principle is already the generated context: injected material is generated from routed, pinned sources and byte-checked, so a compaction summary is not a delivery path and a self-edited block cannot become one. Worth stating in the composition milestone design as a falsification condition: a compaction or handoff summary is never treated as governed context.

## Context compilation each turn

The thread points at a git-backed memory filesystem in which "the agent and harness compile its context on each turn" and a reflection subagent manages long-term memory. Crosslink's session start does the same in a fixed way: the handoff, the last action, open issues and, by label, up to three knowledge pages. The composition milestone's generator is the compilation step the estate specified. Three refinements follow from the paper and the thread:

- **Index first, bodies on demand.** "Why can't it just dump into files and keep notes on what's where?" The notes-on-what's-where artifact is the knowledge index, which here is a stub from July. A maintained index with one line per page and its read-when is the cheapest surfacing mechanism and the one the paper's offload-and-grep behavior relies on.
- **Stable prefix, volatile suffix.** The prefill cost model argues for ordering injected context so that always-on material comes first and the phase pointer and handoff last; the composition milestone delivery design should state the order.
- **Skills as evolvable artifacts.** The paper steers compaction with one sentence and evolves skill text through a proposer loop scored on a held-out split, improving accuracy by up to 35.9 points. A phase skill is such a text; fresh-reader calibration on a fixture is the held-out score; the loop is the mechanized form of the phase-skill rewrite's evaluation.

## The operator's memory-system complaint of 2026-10-01, and the deterministic ladder

In a mdatron session on 2026-10-01 the operator said: "I am a little annoyed that you have made all these little rules"; "I think your memory system creates as many problems as it solves"; and "Here's the thing that pisses me off about your memory system. It's a whole set of arbitrary persistence that is non deterministic in when you do or don't abide by it." The session's own diagnosis at the time: the agent memory turns one remark into a standing rule that returns each session with more authority than the remark had; it piles up stale work-state files that duplicate the tracker and git; it sits beside crosslink's handoff as a second source of truth; and it is non-deterministic in what gets saved, what gets read (only the index loads; opening a file is a judgment) and how hard it is applied. The operator deferred any change ("I don't want to do anything with it yet").

Today's sources answer the complaint from three directions, and they agree.

- **The practitioner's habits** put each kind of instruction at the layer that loads it deterministically: a checked-in repository instructions file for rules true of the codebase, visible and editable by everyone; path-scoped rule files that load only for matching files; a personal cross-project file for preferences; and memory only for "what was surprising or load-bearing, a distillation, not a transcript," written with the why, and capturing confirmations as well as corrections so the system does not "only ever learn caution."
- **The reference repositories** keep rules in files the repository carries (a locked goal statement; agent definitions; path-scoped rules) and turn recurring agent failures into hooks ("the guidance is encoded as code"; "fixes become permanent infrastructure rather than a one-off prompt tweak"). None keeps process rules in an agent's private memory.
- **The context paper and thread** name the missing operation, a forgetting policy, and the pairing that makes mutable memory safe: an immutable log beside it.

The ladder that follows, from most to least deterministic: a hook or pattern that blocks or warns at the act; a checked-in rules file loaded every session or by path; a knowledge page attached to the issue that needs it, injected at session start; and last, agent memory, loaded only as an index and applied by judgment. A rule belongs on the highest rung that can carry it; memory holds only hard-to-rediscover facts and the why behind confirmed or corrected conduct, never a process rule the repository does not also carry. By that ladder the vsdd-cli agent memory has the same shape the operator complained about: most of its entries are process rules ("hands off readiness", "route findings before fixing", "spec amendments are owned and reviewed", "PR workflow") that already live in, or belong in, the project rules file, the contract or a hook. This is recorded as a finding for the operator's pending decision on the memory system, not acted on; no memory files were written in this session after the complaint was found.

## Feature requests this suggests for crosslink's knowledge management (for another session)

- A status field on pages (current, superseded by, retired) that drops superseded pages from default search and label injection.
- A supersede operation (`knowledge supersede <old> --by <new>`) that writes the status, the cross-links and the index line in one step.
- A maintained index generated from frontmatter (title, read-when from a description field, kind, status), replacing the hand-kept stub.
- Section-level retrieval (`knowledge show <slug> --section <heading>`) and a size report against the injection budget.
- Injection ordering and a budget: stable pages first, the handoff last; a warning when attached pages exceed the budget.
- A provenance block in frontmatter beyond the source list: revision, verified-at, kind, supersedes.
- A knowledge lint on add and edit: the first paragraph's required form, and a size warning for pages tagged as injectable.
- Export of the knowledge branch into a working-tree directory for the conformance engine to walk, which is item 11 on mdatron GitHub issue #79 from the other side.

## Adoption candidates, with homes

| Candidate | Home |
|---|---|
| A rewrite operation on the mutable projection (the milestone document, the current page) owned by its author, with the immutable log kept beside it | The composition milestone design; the conventions proposal |
| A forgetting policy by status: what records stop carrying and what must never be collapsed | the conventions proposal; the register's resolution rules |
| A compaction or handoff summary is never governed context; only generated, pinned context is | The composition milestone design's falsification conditions |
| Injection order: stable first, volatile last; index before bodies | The composition milestone delivery design; the session-start hook |
| Phase skills as evolvable texts scored by fresh-reader calibration on a fixture | the phase-skill rewrite |
| The knowledge feature requests above | the crosslink knowledge session |

## Handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-08.

- `vsdd-cli#839`: Slice 2 (Composition) phase-1a design — vsdd way: composition function + config-integrity ... [open]
- mdatron GitHub issue #79 is the roadmap feedback filed from this repository on 2026-10-08.
