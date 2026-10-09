---
title: "Agent-generated tests during issue resolution: what six coding agents actually write (arXiv 2602.07900), read 2026-10-09"
tags: ["design-input", "paper", "design-doc"]
sources:
  - url: "https://arxiv.org/abs/2602.07900"
    title: ""
    accessed_at: "2026-10-09"
contributors: ["xqjG"]
created: 2026-10-09
updated: 2026-10-09
---


## Design Specification

### status

Reference summary, design input for the phase 2a skill in the phase-skill rewrite (`vsdd-cli#898`), the scaffold commit of the cadence decision, and the fix lane. Source: arXiv 2602.07900, "Rethinking the Value of Agent-Generated Tests for LLM-Based Software Engineering Agents", seven authors at universities in Singapore and China (David Lo among them), submitted 2026-02-08; the HTML of version 1 was read on 2026-10-09 (the abstract page lists a version 2 of 2026-04-09, not read). Tier: an academic paper, the first tier of the source weighting on `terminology-grounding-2026-10-08`. Everything below is the paper's; this estate's reading is on `cross-reference-agent-generated-tests-2026-10-09`.

Phase names on this page are the contract's: 2a test-suite generation (the red gate), 2b minimal implementation. "This estate" means vsdd-cli together with crosslink and mdatron.

### the question and the setup

Whether the tests coding agents write while resolving issues help them resolve, or mainly consume budget. Three research questions: what testing behavior emerges under a light scaffold (frequency, timing, execution); what feedback signals the tests provide and what assertions they use; whether the tests affect resolution and cost.

Setup: SWE-bench Verified (500 real GitHub issues with a fixed repository snapshot and an official harness), run through mini-SWE-agent, a bash-only loop with no testing tools, so every testing decision is the model's. Six models, one per family, each its family's best on the bash-only leaderboard as of 2025-12-11: claude-opus-4.5 (74.4 percent resolved), gemini-3-pro-preview (74.2), gpt-5.2 (71.8), kimi-k2-thinking (63.4), minimax-m2 (61.0), deepseek-v3.2-reasoner (60.0). About 1,600 dollars of API spend. A test is counted only when the agent writes a new file through the bash tool whose name matches a Python test pattern; inline scripts and ad hoc probes are not counted, which the paper lists as a threat.

### what the agents do (research question 1)

- **Most agents write tests on most tasks, and the rate does not separate solved from unsolved.** Share of tasks with at least one test file: claude 83 percent, gemini 62, kimi 97, minimax 99, deepseek 89. Resolved versus unresolved tasks differ by a few points (claude 84 versus 79; deepseek 92 versus 85) and unresolved tasks write slightly more tests per task. gpt-5.2 wrote a test file on 0.6 percent of tasks and still resolved 71.8 percent.
- **Tests come mid-run and are revisited.** Across models the first test lands at about 40 percent of the run and the last at about 78 percent. Each test file is executed about 1.5 times; about a fifth of executions exit non-zero, which the paper reads as friction, not as a verdict on the patch.

### what the tests contain (research question 2)

- **Prints outnumber assertions by four to six to one per task.** Per-task means: claude 5 assertions against 25 value-revealing prints; kimi 3 against 21; minimax 7 against 34; deepseek 4 against 16; gemini 1.5 against 4. The paper calls the tests "primarily an observational feedback channel" and, in its conclusion, "a runtime probing interface rather than a systematic verification mechanism".
- **Of the assertions, roughly half constrain little.** The paper's four categories, assigned by a rule-based classifier over the syntax tree: sanity (existence or type only) 15 to 20 percent; property (an attribute, no exact output) 34 to 41 percent; relational (a bound, a relationship, or an expected exception) 3 to 8 percent; exact (an exact value or deep equality) 35 to 43 percent. A compound assertion takes its strongest member's category.

### whether it matters (research question 3)

Prompt-only interventions on four models: two told to write at least one runnable new test file, two told to write none (with the "test edge cases" sentence removed). Behavior moved a lot (the intended test status changed on 37 to 75 percent of tasks); outcomes barely moved: gpt-5.2 encouraged 359 to 359 resolved, gemini encouraged 371 to 366, kimi discouraged 317 to 304, deepseek discouraged 300 to 291. On average 83.2 percent of tasks kept the same outcome. The paper: "more agent-written tests do not mean more solves."

Cost moved with behavior. Discouraging tests cut API calls by 25 to 35 percent and input tokens by 33 to 49 percent (kimi: 668 thousand to 341 thousand input tokens per task) at the price of 2 to 3 points of resolution. Encouraging raised gpt-5.2's output tokens 20 percent for no change in outcome.

### what the authors conclude and what they do not claim

Their reading: test writing "frequently reflects how an agent chooses to work, not whether it can reliably validate the patch". Implications they draw: agents need instruction "not only on whether tests should be written, but also on how they should be designed" and how to read their feedback; practitioners "may consider adopting a more conservative approach to agent-generated tests" and a cost-benefit monitoring of test overhead; conventional test-quality metrics assume a fixed code snapshot that agentic work does not give.

Not claimed, and worth holding onto: no statistical test is reported anywhere, the results are descriptive; no causal account of why prints dominate; no analysis of whether an asserting test, as opposed to a probe, changes outcomes, since the interventions moved volume, not kind; nothing on whether the tests survive into the final patch; one scaffold, one language, no enforced CI, and the authors name "enforced CI" as a toolchain under which magnitudes may differ. Two small inconsistencies in the text (a "four LLMs" sentence beside six models; 61.1 versus 61.6 percent for one baseline) do not touch the findings.

### handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-09.

- `vsdd-cli#898`: Phase-skill rewrite (design-first): the ten phase skills, the prose reviewer roles and the register standard, rewritten from the 2026-10-08 reference practice [open]

