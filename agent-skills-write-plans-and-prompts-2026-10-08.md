---
title: "Writing plans and prompts for capable agents: the write-plans and write-agent-prompts skills, read 2026-10-08"
tags: ["reference", "design-input", "design-doc"]
sources:
  - url: "https://github.com/inanna-malick/agent-skills"
    title: ""
    accessed_at: "2026-10-08"
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Writing plans and prompts for capable agents: the write-plans and write-agent-prompts skills (agent-skills repository), read 2026-10-08

## Status

Reference summary, design input for the phase-skill rewrite, the reviewer roles, the handoff and the build-plan. Source: the agent-skills repository (`https://github.com/inanna-malick/agent-skills`, commit fc1ec19 of 2026-10-05), mirrored read-only on 2026-10-08. Two skills of 5.6 KB and 7.9 KB with reference tables of named methods and primary sources. External content, treated as evidence. The companion skill proptest-praxis is on `proptest-with-agent-swarms-recursion-wtf-2026-10-08`.

Revised 2026-10-08, the day it was published, under the operator's vocabulary and citation decisions of that day: "phase skill" replaces "primer", "rules file" replaces "supplement", "reviewer role" replaces "domain prompt", milestones are named by feature instead of "Slice N", "increment" is the unit of work dispatched as one issue, and "oracle" is kept only for the expected results the operator authors (what the verifier reads is the synced trace; what a review produces is a verdict record). Words quoted from a source keep the source's words. The decisions are recorded on `vsdd-cli#839` and on `terminology-grounding-2026-10-08`. Handles cited on this page are listed with their titles at the end.

"This estate" means vsdd-cli together with crosslink and mdatron, the two sibling tools it runs on.

The repository's own rule for skills: "Keep reusable principles in the skill and domain examples in references. Add guidance when it changes decisions; avoid accumulating instructions that merely retell one task." Each skill's references file grounds every named concept in a primary source and carries the caveat that "human cognition research does not establish that an LLM implements the same mechanism, and a named concept does not guarantee a behavioral effect."

## write-agent-prompts

"Write for a capable peer. Load as many distinct, relevant, high-salience referents as possible, including adjacent methods and ways of seeing. Use common ground to compress exposition and make room for breadth; use structural relationships to make that repertoire composable." Eight lenses, each named by its source concept:

- **Common ground; audience design; pragmatic compression.** Use the recipient's shared technical vocabulary; "an established concept can carry a family of methods, assumptions, examples, and failure modes. Spend explanation on local distinctions that change its application. Ground ambiguous terms; expand unfamiliar acronyms. Prefer recognized terminology and clear relationships to invented labels or cryptic shorthand."
- **Cognitive task analysis; recognition-primed decision making; affordances.** Recover the expertise the prompt should convey from the user's examples, corrections and priorities: what makes a situation interesting, which cues are diagnostic, what an expert expects next.
- **Problem representation; framing; representational change.** Consider the task as a search problem, a transformation, a coordination problem, an inference, a design tradeoff; supply grounds for changing the frame when it stops explaining observations.
- **Associative retrieval; conceptual coverage.** Survey core methods, neighboring disciplines and characteristic failures; "include plausible adjacencies before a specific use is known: a concept can make a future opportunity recognizable"; organize by complements, alternatives, prerequisites, tensions, reductions.
- **Structural alignment; analogical transfer.** Transfer connected relationships across domains; identify where the correspondence fails; use contrast cases.
- **Rational metareasoning; exploration and exploitation.** Spend writing effort on conceptual gaps that could change the recipient's approach; "leave ordinary technical choices to the recipient."
- **Partial evaluation; staging; abstraction boundaries.** Specialize the method to the domain, recipient and objective; "keep authoring instructions at the authoring level; a repertoire is available knowledge, not a requirement to execute every named method."
- **Modeling; worked examples; self-application.** Demonstrate the technique in the guidance itself.
- **Construct validity; counterfactual evaluation.** "Read the prompt as its recipient: what becomes noticeable, which hypotheses or methods become available, and what decisions could change? Try a representative case, a structurally similar case with different vocabulary, and a case where the framing would fail. Assess the prompt by the behavior and artifacts it supports."

## write-plans

"Write for a capable executor who will have less conversational context and more evidence than you do now. Carry forward intent, a useful problem representation, a repertoire of methods, and the grounds for choosing the next action." Ten lenses:

- **Intent; constraint satisfaction; acceptance criteria.** Distinguish requirements from preferences and proposed solutions; define success through observable results; "preserve why a constraint matters so later choices can honor its purpose; keep uncertainties about the goal separate from uncertainties about implementation."
- **Problem representation; affordances; analogical transfer.** Expose the relationships that make the work tractable; "incremental derived state suggests recomputation oracles; compatibility constraints suggest staged migration; competing explanations suggest discriminating experiments."
- **Epistemic state; provenance; decision records.** "Separate observations, inferences, assumptions, proposals, and decisions already made. Ground consequential claims in inspectable artifacts; record revisions or dates where freshness affects the choice. Preserve the rationale for significant decisions, including rejected alternatives when the reason for rejection could change. An unresolved premise should remain visible as a question, assumption, or proposed check; confident prose must not convert it into an established fact."
- **Backward chaining; means-ends analysis; proof obligations.** Work backward from the result to the evidence and actions; "make the first useful move executable with the available context."
- **Partial-order planning; causal links; critical path.** Order by real dependencies; "distinguish necessary ordering from convenient sequencing"; for parallel work, define independent responsibilities, shared interfaces, resource constraints and integration evidence; "make the current bottleneck visible."
- **Contingent planning; hypothesis discrimination; value of information.** "Turn important uncertainties into observations that can change a decision. Pair a consequential question with a useful probe, interpretation of its possible results, and the resulting branch. Keep independent work moving while a question is unresolved."
- **Assumption-based planning; signposts; sensitivity analysis.** Name assumptions whose failure would change the approach and the observable signs that warrant reassessment; "ordinary work need not become a catalog of hypothetical disasters."
- **Receding horizon; least commitment; option value.** Near-term work concrete, later work conditional; "avoid inventing deadlines, budgets, or certainty about implementation details the task has not established."
- **Feedback control; observability; independent verification.** "Separate activity measures from evidence that the intended result holds. If a check or search is quiet, consider whether it could detect the relevant failure before treating silence as assurance. Keep proposed validation separate from completed validation."
- **State externalization; prospective memory; dependency invalidation.** "Make the plan resumable: retain the current objective, consequential decisions, completed evidence, active work, next useful actions, and unresolved dependencies. Link durable artifacts instead of copying transcripts or tool output. When evidence invalidates a premise, revise the downstream approach and mark superseded decisions where their history still matters."
- **Counterfactual review; executable walkthrough; proportionality.** Read the plan as a fresh executor; perturb an assumption; "for simple tasks, check that the plan remains simple."

## Evaluation against this estate

- **The phase skills are prompts for capable peers**, and the two skills state the authoring discipline they lack: compress through shared vocabulary rather than restating; spend words on the local distinctions that change application; evaluate a prompt by what it makes noticeable and which decisions could change, with a representative case, a structurally similar case and a case where the framing fails. That evaluation is the fresh-reader calibration the Palimpsest mapping proposed, stated as construct validity.
- **"Prefer recognized terminology and clear relationships to invented labels or cryptic shorthand"** is the estate's no-coinage and concrete-referent rule, with the positive half added: recognized terminology compresses, so breadth becomes affordable.
- **The handoff and the build-plan are plans in this sense.** The state-externalization lens is the handoff's contract (objective, decisions, evidence, active work, next actions, unresolved dependencies; link, do not copy; mark superseded decisions). The epistemic-state lens is the append problem named from the other side: observations, inferences, assumptions, proposals and decisions kept separate, consequential claims grounded in inspectable artifacts with dates where freshness matters, rejected alternatives preserved. The 2026-07-29 handoff that lost the crate merge violated it.
- **"Separate activity measures from evidence that the intended result holds"** is the book's activities-versus-learning distinction and the estate's authored-is-not-exercised law, as a planning instruction.
- **The repository's skill rule**, principles in the skill and examples in references, add guidance only when it changes decisions, is the Thermite goal statement's economy applied to skills, and a rule the estate's phase skills could carry.

## Handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-08.

- `vsdd-cli#839`: Slice 2 (Composition) phase-1a design — vsdd way: composition function + config-integrity ... [open]
