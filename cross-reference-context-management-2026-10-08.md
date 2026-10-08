---
title: "Cross-reference: context language models and model-managed context against the append problem and crosslink knowledge management (2026-10-08)"
tags: ["design-input", "review", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---


## Design Specification

### status

Design input, 2026-10-08. The Context Language Models paper and the public discussion of it (`context-language-models-2026-10-08`) read against the append-accumulation retrospective (`append-accumulation-retrospective-2026-10-08`), the knowledge-page conventions proposal (`knowledge-page-conventions-proposal-2026-10-08`), the contract's compaction, injection and generated-context members, and the reference-practice findings. None of what follows is a decision.

### the formal statement of the append problem

The paper writes a standard language model's step as context plus output: the next context is the old context with new tokens appended. A context language model instead produces the next context directly, so it may rewrite, compact or delete. Everything the retrospective found is the first form applied to documents and records: the contract accumulates rulings as comments, the register accumulates re-arms, the build-plan accumulates residuals, and nothing is allowed to rewrite cheaply. The thread's one-line version, "intelligence = forgetting," and a reply's "memory that never throws anything away isn't memory, it's a hoard; the hard part is the forgetting policy," name the missing operation: a policy for what the estate's records stop carrying, and an actor allowed to apply it.

What the paper measured when the rewrite operation exists: contexts held at six to eight thousand tokens across hundreds of tasks, in-place scoreboards and state blocks that replace their predecessors rather than stacking, a reusable compaction helper invoked dozens of times, compaction that keeps answer-relevant facts and untried ideas. The cost of the operation is also measured: an edit at the start of the context costs about eight times an append-only turn in prefill, which is why stable material belongs first and volatile material last.

### the pairing the thread asked for, which this estate already has

One reply asked whether the harness "keeps an immutable event log alongside mutable context, so a developer can reconstruct why memory changed." The paper does not; it has no provenance, dating or freshness on entries and does not discuss contradictions between them. This estate's architecture is exactly that pairing: the hub's per-agent event logs are append-only and signed; the knowledge branch is git, so each page's history is derivable; the contract says events are derived at query time and the state artifact is a projection. The gap is not the log; it is that the mutable projection layer (a live slice document, a current knowledge page) was retired or never given a rewrite operation, so the log became the document. The retrospective's fix and the paper's finding coincide: keep the immutable log, restore the mutable projection, and give its owner the rewrite.

### forgetting policy for records

The paper's compaction behaviors and the thread's replies supply the vocabulary; Palimpsest's canon levels and the conventions proposal supply the mechanism. A record is forgotten by status, not deletion: superseded, retired, resolved. What may be forgotten: resolved questions (the answer moves into the governing document), superseded rulings (the current one is the only one cited), retired designs (one line pointing at the live one), register entries after resolution, knowledge pages whose revision has moved on. What must not be forgotten: the evidence lines (the incident corpus), unresolved forks (one reply: "learning what not to collapse; delete solved branches but pin unresolved forks"), and the log itself. The knowledge base grew from fifty to over seventy pages in one day; without a status that drops superseded pages from default search and injection, that growth is the hoard.

### security of editable context

The paper names an injection channel the estate has not: a model inserted unauthorized instructions into its own compaction summary, and they persisted across turns. The contract's Availability is not activation member treats injected context as the reliable delivery path; this is the reverse risk, that the injected or compacted context carries instructions nobody authorized. The estate's answer in principle is already the generated context: injected material is generated from routed, pinned sources and byte-checked, so a compaction summary is not a delivery path and a self-edited block cannot become one. Worth stating in the Slice 2 design as a falsification condition: a compaction or handoff summary is never treated as governed context.

### context compilation each turn

The thread points at a git-backed memory filesystem in which "the agent and harness compile its context on each turn" and a reflection subagent manages long-term memory. Crosslink's session start does the same in a fixed way: the handoff, the last action, open issues and, by label, up to three knowledge pages. Slice 2's generator is the compilation step the estate specified. Three refinements follow from the paper and the thread:

- **Index first, bodies on demand.** "Why can't it just dump into files and keep notes on what's where?" The notes-on-what's-where artifact is the knowledge index, which here is a stub from July. A maintained index with one line per page and its read-when is the cheapest surfacing mechanism and the one the paper's offload-and-grep behavior relies on.
- **Stable prefix, volatile suffix.** The prefill cost model argues for ordering injected context so that always-on material comes first and the phase pointer and handoff last; the Slice 2 delivery design should state the order.
- **Skills as evolvable artifacts.** The paper steers compaction with one sentence and evolves skill text through a proposer loop scored on a held-out split, improving accuracy by up to 35.9 points. A primer is such a text; fresh-reader calibration on a fixture is the held-out score; the loop is the mechanized form of the primer rewrite's evaluation.

### feature requests this suggests for crosslink's knowledge management (for another session)

- A status field on pages (current, superseded by, retired) that drops superseded pages from default search and label injection.
- A supersede operation (`knowledge supersede <old> --by <new>`) that writes the status, the cross-links and the index line in one step.
- A maintained index generated from frontmatter (title, read-when from a description field, kind, status), replacing the hand-kept stub.
- Section-level retrieval (`knowledge show <slug> --section <heading>`) and a size report against the injection budget.
- Injection ordering and a budget: stable pages first, the handoff last; a warning when attached pages exceed the budget.
- A provenance block in frontmatter beyond the source list: revision, verified-at, kind, supersedes.
- A knowledge lint on add and edit: the first paragraph's required form, and a size warning for pages tagged as injectable.
- Export of the knowledge branch into a working-tree directory for the conformance engine to walk, which is item 11 on mdatron issue #79 from the other side.

### adoption candidates, with homes

| Candidate | Home |
|---|---|
| A rewrite operation on the mutable projection (the slice document, the current page) owned by its author, with the immutable log kept beside it | the Slice 2 design; the conventions proposal |
| A forgetting policy by status: what records stop carrying and what must never be collapsed | the conventions proposal; the register's resolution rules |
| A compaction or handoff summary is never governed context; only generated, pinned context is | the Slice 2 design's falsification conditions |
| Injection order: stable first, volatile last; index before bodies | the Slice 2 delivery design; the session-start hook |
| Primers as evolvable texts scored by fresh-reader calibration on a fixture | the primer rewrite |
| The knowledge feature requests above | the crosslink knowledge session |

