---
title: "Cross-reference: the recursion.wtf articles against the 2026-10-08 findings and this estate's design"
tags: ["design-input", "review", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Cross-reference: the recursion.wtf articles (property testing with agents, ExoMonad, Tidepool) against the 2026-10-08 findings and this estate's design

## Status

Design input, 2026-10-08. Three articles by one practitioner (`proptest-with-agent-swarms-recursion-wtf-2026-10-08`, `exomonad-recursion-wtf-2026-10-08`, `tidepool-recursion-wtf-2026-10-08`) read against the contract, the reference-practice findings (`vsdd-in-practice-reference-repositories-2026-10-08`) and the earlier cross-reference (`cross-reference-2026-10-08-findings-vs-prior-knowledge`). The agent-skills and ExoMonad repositories were mirrored read-only on 2026-10-08 after the operator's say-so, and five of the author's bug pull requests were sampled; Tidepool was not read. The repository findings are in the section before the adoption table. Adoption candidates are collected at the end; none is a decision.

## Property testing with agent swarms

**Confirms.**

- **The fix-lane discipline, from outside.** Each confirmed bug ships as a branch from main containing only the regression test and the fix; the test must fail before the fix and pass after; the discovery machinery stays elsewhere. That is the contract's fix-scale red gate and the references' failing-first-for-fixes, stated by a third practitioner with hundreds of bugs behind it.
- **The seeded-defect check.** "If a target yields no findings, suspect coverage: temporarily remove a cache invalidation and confirm the tests catch the stale result." That is the contract's negative-case fixture and clean twin, the no-self-oracle rule, and the mutation floor's purpose, as a procedure.
- **Expand each finding to its class.** A missed invalidation means checking every mutation of that state and every similar cache: Thermite's "fix the cause's whole class", the gist's escalate-to-a-rule, the contract's "a new defect class is closed by adding a pattern."
- **The domain scorecard's verdict.** Mutation and property machinery were the single most reliable value source in the estate's five datasets; this article's yield (thirteen, four, two, more than thirty bugs in repositories with thousands of existing tests) is the same finding at larger scale. Agent budget spent on building test machinery pays; agent budget spent on prose review mostly does not.

**Adds.**

- **An attested agent form for the 2a primer.** The whitepaper's "generate the test suite first" has no reference implementation for feature increments, but it has one for machinery: build a reference model, generators of interacting operations over a small key pool, and assertions, then inspect sample traces, then prove the machinery's coverage by removing an invariant. The red proof for machinery is the seeded removal, not a failing feature test.
- **Discovery suite versus deliverable.** The article separates the machinery branch from the regression deliverables. The estate's Slice 4 corpus does not make that distinction; the runnable-mini-repo fixtures are deliverables, and nothing says where discovery machinery lives or how long it stays.
- **A procedure for Slice 2's verification.** The composition function's properties (determinism, domain-narrowing, pair co-activation, pair separation) map directly onto "model against a naive reference and run generated edit sequences"; the stale-composition hash the state schema carries is the article's "compare cached metadata with recomputation" check.
- **A harness caveat for Red Team work.** Generator and adversarial work trips the runtime harness's content notice; the author's workaround is conversational and compaction. The estate's Red Team dispatches should expect the same and record it as friction, not as a finding.
- **Roles and models.** Planning, orchestration and parallel test-writing on different models in parallel worktrees: the dials per lens and per stage the contract already specifies, with a task class (test machinery) that tolerates a cheap tier.

## ExoMonad

**Confirms.**

- **Hooks as code, compounding.** "The guidance is encoded as code"; a hook costs nothing per agent while a prompt costs tokens for every agent including those that never touch the file; fixes "become permanent infrastructure rather than a one-off prompt tweak" and compound. That is the contract's Conformance at action time ("enforcement logic lives in pattern and registry data; hook scripts stay thin wrappers; a new defect class is closed by adding a pattern") with the token argument made explicit, and it is the strongest external support for action-time activation over always-on prose.
- **Token arbitrage by role** is the contract's model and effort dials per lens and per model-running stage; the subsidy-of-the-week dynamic argues for cost bands as data, as the contract already has them.
- **One reviewer plus an automated reviewer** (a lead reviews; Copilot reviews): the same shape as the four reference repositories.
- **Throughput as evidence.** Thirty-two thousand lines across sixty-one pull requests in four active days, under half of two subscriptions, is the cadence point again: design and build in the same window, waves of small units.

**Adds.**

- **The scaffolding commit is a wave-level red gate.** A lead commits types, tests, stub files and planning documents as the base every subtask forks from; the wave's workers make that base pass. The reference repositories have no red gate for feature increments; this is the attested shape in which one exists: not a failing test per feature, but a failing scaffold per wave. It is also the increment-zero pattern in Thermite (foundation lands first, the rest parallelizes) and Peritus (conformance foundation before consumers), now with the tests in the scaffold.
- **Nested merge targets.** Subtask pull requests target the parent branch, not main; parents fold upward; every agent in a wave forks from the same commit so there is no rebase burden within a wave. The estate's kickoff model is one worktree per issue against one feature branch with one pull request per milestone; the recursion (a lead that spawns leads) and the per-wave base commit are what the Slice 6 review stage and build-entry proposal lack.
- **Config as a restricted typed language, not YAML with expressions.** The author abandoned YAML plus custom expression languages after repeatedly re-implementing effect composition. The estate's open question on DSL scope sits at "narrow" by adopted default; this is a third practitioner's evidence that expression languages grown inside data become a language anyway. It supports keeping the pattern lane narrow and moving anything compositional into code the host executes.
- **The message bus as on-disk mailboxes and JSONL** is the same shape as the hub's per-agent append-only event logs; the estate's injection seam and the bus are the same kind of surface.

## Tidepool

**Adds.**

- **One eval replaces many tool calls.** The conformance oracle's observed set is the trace's tool-use events; a tool that collapses a sequence of reads, globs and searches into one evaluated effect changes what a trace shows and what the efficiency advisories can count. A caveat for Verifiable conformance and a design point for the cost member: the unit of observation is the effect, not the call.
- **Suspension for an unresolved question.** The `ask` effect suspends a run as a resumable continuation and resumes with the human's decision. The contract's rule that an unresolved design question reaching a dispatched run "produces a blocker record on the owning issue" has here a mechanized form: suspend with a stub, resume with the answer, no stall and no guess. A candidate for Slice 6's dispatcher.
- **Budgeted, tree-aware truncation with continuation.** A character budget, stubs for the tails of arrays and large fields of objects, resumable by stub identifier, free on any shape. That is a delivery mechanism for large generated context and large reports: slices on demand against a budget instead of whole-file injection, which is the open question the 2026-10-02 assessment left on the kickoff block and compaction.
- **Functional core, effect shell, at the tooling level.** The sequence is pure and typed; Rust executes each effect. The estate's pure core and effectful shell is the same split for its own code.

## Repository findings (added after the mirrors were read)

**The proptest-praxis skill** (`proptest-with-agent-swarms-recursion-wtf-2026-10-08`, last section) turns the article into a method with four standing lenses, a catalogue of oracle forms per target class, generator construction rules, the four-link fault-activation chain (reachability, infection, propagation, revealability), mutation as harness calibration, and a failure-investigation procedure ending in "a readable deterministic unit test with explicit expected behavior" that fails for the claimed reason. Two of its sentences belong in the 2a and fix-lane primers verbatim in substance: "initially treat no findings as a coverage problem", and "a seed, timeout, or disagreement with an opaque generated model alone is not the final deliverable." Its roles (planner, orchestrator, bounded implementers each in a worktree, allocation by expected information gain) are the references' shape again.

**The sampled pull requests** are the delivered form in evidence: two files each, a validation paragraph naming the upstream commit, toolchain and exact command, the statement that the regression fails without the fix and passes with it, and an honesty line separating the demonstrated contract violation from unproven application impact, with an AI-assistance trailer. That is the fix-lane record the contract specifies, as practised by someone else, and it is the shape a conformance check could parse.

**The write-plans and write-agent-prompts skills** (`agent-skills-write-plans-and-prompts-2026-10-08`) state the authoring discipline the primers and the handoff lack: compress through recognized vocabulary and spend words on the local distinctions that change application; evaluate a prompt by what it makes noticeable and which decisions could change, on a representative case, a structurally similar case and a case where the framing fails; keep observations, inferences, assumptions, proposals and decisions separate; ground claims in inspectable artifacts with dates where freshness matters; "confident prose must not convert an unresolved premise into an established fact"; make a plan resumable by linking artifacts, not copying transcripts, and by marking superseded decisions. The repository's rule for skills, principles in the skill and examples in references and "add guidance when it changes decisions", is a size discipline the primers could carry.

**ExoMonad's repository** (`exomonad-recursion-wtf-2026-10-08`, last section) adds three things the article did not state. The spec commit is a decision record with three layers by depth, types always, intent at shallow depth, acceptance as failing tests and property stubs when feasible, and it "need not be finished, globally green, or even compilable"; a fresh child "independently re-derives the implementation from those artifacts, which is part of the review architecture." Depth over breadth is quantified: no context window reasons about more than about four children, because sub-leads are compression boundaries. And the tool keeps seventeen decision records with status lines and review provenance, including a post-wave audit of reviewer feedback that a poller defect had silently dropped: the same authored-but-not-exercised failure Thermite had, repaired by an audit document with per-item verdicts.

## Adoption candidates, with homes

| Candidate | Home |
|---|---|
| The 2a primer names property-test machinery (reference model, interacting-operation generators, seeded-removal coverage proof, shrink to a standalone regression test) as the attested agent form of test-suite generation; red-first holds for fixes and pins; the scaffold commit is the wave-level red gate | the 2a primer; the Slice 4 design |
| Discovery machinery kept apart from regression deliverables, with a stated home and lifetime | the Slice 4 fixture corpus |
| Composition and pricing functions modeled against naive references under generated edit sequences; the stale-composition hash as cached-versus-recomputed | the Slice 2 design |
| Hooks as patterns with the token argument; a count of hooks born from incidents as a learning-loop indicator | the Conformance at action time primer text; the Slice 7 criteria |
| A per-wave scaffolding commit as the fork base; nested merge targets; waves from a merged base | the Slice 6 design (review stage and build entry) |
| Suspension with a resumable record for an unresolved question in a dispatched run | the Slice 6 design |
| Budgeted tree-aware truncation with continuation for serving generated context and reports | the Slice 2 design (delivery); the Slice 7 design |
| Keep the pattern lane narrow; anything compositional belongs in host-executed code | the DSL-scope open question (supports the adopted default) |
| Expect and record the harness's content-notice friction on generator and adversarial work | the Red Team domain prompt; the runtime-harness supplement |
| Test-machinery work as a task class with a cheap model tier | the review config's tier and effort defaults |
| The spec commit's three layers (types always; intent at shallow depth; failing tests and property stubs when feasible) as the shape of a slice's wave base, explicitly allowed to be red or non-compiling | the 2a primer; the Slice 2 design |
| Fresh child contexts re-derive from artifacts by default; transcript inheritance opt-in | the Slice 6 dispatcher; the kickoff block question |
| Depth over breadth: more than about four independent units means an intermediate lead, so no context reasons about more than its fan-out | the Slice 6 review stage and build entry |
| Prompt-authoring discipline (recognized vocabulary, local distinctions, construct-validity evaluation on three cases) and plan-authoring discipline (epistemic state separated, artifacts linked, superseded decisions marked) | the primer rewrite; the session skill's handoff; the build-plan |
| The sampled pull-request form (trigger, behavior, expectation, fix; validation with commit, toolchain and command; the claim boundary stated) as the fix-lane record a check can parse | the fix-lane primer; Slice 5's commit evidence section |
