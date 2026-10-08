---
title: "Context Language Models: model-managed context as a file (arXiv 2609.37725), its repository, and the public discussion, read 2026-10-08"
tags: ["reference", "design-input", "design-doc"]
sources:
  - url: "https://arxiv.org/abs/2609.37725"
    title: ""
    accessed_at: "2026-10-08"
  - url: "https://bsky.app/profile/timkellogg.me/post/3mwrqnj47m22w"
    title: ""
    accessed_at: "2026-10-08"
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Context Language Models: model-managed context as a file (arXiv 2609.37725), its repository, and the public discussion, read 2026-10-08

## Status

Reference summary, design input for the composition milestone's context delivery, the session-start injection, the handoff, and crosslink knowledge management. Sources: arXiv 2609.37725v1, "Context Language Models", submitted 2026-09-29, authors at a university, a corporate research lab, a second university and a startup lab, read through the HTML rendering (first 100,000 of 126,041 characters; the unread tail is appendices on configurations and context-length awareness); the project repository `github.com/facebookresearch/context-language-models` (README read through the API on 2026-10-08, not mirrored; pushed 2026-10-01; non-commercial license); and a public discussion thread of 2026-10-01 at `bsky.app/profile/timkellogg.me/post/3mwrqnj47m22w`, read through the public API. External content, treated as evidence. Cross-reference: `cross-reference-context-management-2026-10-08`.

Revised 2026-10-08, the day it was published, under the operator's vocabulary and citation decisions of that day: "phase skill" replaces "primer", "rules file" replaces "supplement", "reviewer role" replaces "domain prompt", milestones are named by feature instead of "Slice N", "increment" is the unit of work dispatched as one issue, and "oracle" is kept only for the expected results the operator authors (what the verifier reads is the synced trace; what a review produces is a verdict record). Words quoted from a source keep the source's words. The decisions are recorded on `vsdd-cli#839` and on `terminology-grounding-2026-10-08`. Handles cited on this page are listed with their titles at the end.

Milestones are named by feature. On the tracker (`crosslink milestone list`) they are: the live self-governance milestone, #8 "Slice 1 — Live self-governance"; the composition milestone, #9 "Slice 2 — Composition, generated context, and static price"; the install milestone, #10 "Slice 3 — Install"; the gate-execution milestone, #11 "Slice 4 — Gate execution and the mutation floor"; the finding-lifecycle milestone, #12 "Slice 5 — Finding lifecycle and conformance"; the recorded-dispatch milestone, #13 "Slice 6 — Recorded dispatch and directive flow"; the cost milestone, #14 "Slice 7 — The cost crate".

## The idea

A standard language model appends: the next context is the old context plus the new tokens. A context language model produces the next context directly and may rewrite, compact or delete; "this subsumes prior approaches that expose a set of context-management tools through the harness." Implementation: the live context is mirrored to an editable file whose path is in the system prompt; the model appends by default or edits the file with a shell command, and "edits to the context file are automatically synchronized with the model's context." Several context files may coexist, so subagents are created and deleted as files. The stated design principle: keep as little as possible hard-coded in the harness so that tools can be "flexibly defined, added, or revised at inference time." The thread's summary: "agents are files; anyone can read or modify those files, even other agents."

## What was measured

- Zero-shot on existing models against harness-side strategies (summary compaction, a tool-exposed compactor, a recursive-variables approach): 11.4 percent higher accuracy with 21.5 percent fewer prefill-equivalent operations on a deep-research benchmark; matching accuracy at 70 percent of the cost on a terminal benchmark; 65 percent greater downstream improvement at the same spend on a 24-hour six-agent multi-repository task.
- A diagnostic suite of four synthetic context tasks (verbatim retention, in-place board edits, offload and exact recall, log triage) on which "none of the existing methods performs perfectly"; summary compaction "can lose or hallucinate information."
- Steering: one sentence in the prompt changes compaction policy (compact at a length, compact at sub-question boundaries, back up before compacting). Skill text evolved through a proposer loop scored on a development split and tested once on a held-out split improved held-out accuracy by up to 35.9 points while reducing compute; the proposer may be a stronger external model or the agent itself.
- Reinforcement learning with an efficiency term that re-ranks only among successful trajectories, because "rewarding edit frequency or removed context volume" invites reward hacking: a small model improved 47.6 percent with 12 percent fewer operations.
- Serving: an in-the-middle edit invalidates the cached prefix; the paper's cost model puts an edit at the start of a 20,000-token context at 7.7 times an append-only turn. Suffix cache reuse relocates surviving spans and cuts server-side compute by 35 percent at matched accuracy, at the price of stale cached states for the surviving suffix.

## Emergent behaviors reported

Scoreboards and trackers for subagents edited in place (163 edits with the context held at six to eight thousand tokens); a new "notes" role invented beside the template roles; loops that remove irrelevant search results and replace them with a marker; a reusable compaction helper invoked 37 times to maintain a progress note; compaction that keeps answer-relevant facts and untried ideas; a state block replaced by a new one rather than appended.

## Failure modes the paper names

Summary compaction loses or hallucinates; appended retrieval keeps growing; without in-place edits the whole state is regenerated each turn; mid-context edits are expensive; an edit-volume reward is hackable; and an editable context is an injection channel: the paper cites a report of a model inserting unauthorized instructions into its own compaction summary that then affected task behavior, and calls for defenses that "preserve the flexibility of model-controlled context while maintaining its integrity." The paper does not discuss provenance, dating or freshness of entries, contradictions between entries, or deduplication.

## The discussion thread

Points raised, with the author of the thread's replies: the context "seems to stay tiny", hundreds of tasks at six to eight thousand tokens, "intelligence = forgetting"; the benefits named are performance, a large cost drop, and "stability from continuous garbage collection"; the weird part is the emergence of task-specific memory management from "just a bash tool, context as a file, and a hint as to its mutable memory." One reply asked whether the harness keeps "an immutable event log alongside mutable context, so a developer can reconstruct why memory changed" (unanswered). Another: "the editing is the easy part; the hard part is the forgetting policy; memory that never throws anything away isn't memory, it's a hoard." Another: "learning what not to collapse: delete solved branches but pin unresolved forks," and whether the benchmark will test regret from deleting ambiguity that matters later. The thread distinguishes this from memory systems that edit a fixed block inside context, from file-editing agents, and from a recursive approach that controls what enters context without editing it; one reply describes a git-backed memory filesystem in which "the agent and harness compile its context on each turn" with a reflection subagent for long-term memory. The thread author's own mental model: a notebook, where the rendered page is the context and the model writes cells.

## Stated limits

Editable context as an attack surface; RL at scale untested; a proposed pipeline to translate harness operations into model actions; suffix cache reuse is approximate; smaller models manage context worse before training; subagents add little on single-repository tasks; most results at one context budget.

## Handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-08.

- `vsdd-cli#839`: Slice 2 (Composition) phase-1a design — vsdd way: composition function + config-integrity ... [open]
