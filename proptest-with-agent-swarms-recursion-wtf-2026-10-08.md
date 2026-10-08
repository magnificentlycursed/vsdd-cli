---
title: "Property testing with agent swarms (recursion.wtf), read 2026-10-08"
tags: ["reference", "design-input", "design-doc"]
sources:
  - url: "https://recursion.wtf/posts/agents-and-property-tests/"
    title: ""
    accessed_at: "2026-10-08"
  - url: "https://github.com/inanna-malick/agent-skills"
    title: ""
    accessed_at: "2026-10-08"
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Property testing with agent swarms (recursion.wtf), read 2026-10-08

## Status

Reference summary, design input for the phase 2a and phase 5 skills, the fix-lane discipline, the composition milestone's verification and the gate-execution milestone's fixture corpus. Source: `https://recursion.wtf/posts/agents-and-property-tests/` ("Property Testing with Agent Swarms", recursion.wtf, October 2026; tags rust, llm, testing, proptest), fetched read-only on 2026-10-08. External content, treated as evidence. The method's skill lives in the author's agent-skills repository (`proptest-praxis`), not mirrored or read as of this page. Cross-reference: `cross-reference-recursion-wtf-2026-10-08`.

Revised 2026-10-08, the day it was published, under the operator's vocabulary and citation decisions of that day: "phase skill" replaces "primer", "rules file" replaces "supplement", "reviewer role" replaces "domain prompt", milestones are named by feature instead of "Slice N", "increment" is the unit of work dispatched as one issue, and "oracle" is kept only for the expected results the operator authors (what the verifier reads is the synced trace; what a review produces is a verdict record). Words quoted from a source keep the source's words. The decisions are recorded on `vsdd-cli#839` and on `terminology-grounding-2026-10-08`. Handles cited on this page are listed with their titles at the end.

Phase names on this page are the contract's: 1a behavioral specification, 1b verification architecture, 1c the spec review gate, 2a test-suite generation (the red gate), 2b minimal implementation, 2c refactor, 3 adversarial refinement, 4 the feedback integration loop, 5 formal hardening, 6 convergence. The whitepaper has six phases; the a, b and c splits are this repository's.

Milestones are named by feature. On the tracker (`crosslink milestone list`) they are: the live self-governance milestone, #8 "Slice 1 — Live self-governance"; the composition milestone, #9 "Slice 2 — Composition, generated context, and static price"; the install milestone, #10 "Slice 3 — Install"; the gate-execution milestone, #11 "Slice 4 — Gate execution and the mutation floor"; the finding-lifecycle milestone, #12 "Slice 5 — Finding lifecycle and conformance"; the recorded-dispatch milestone, #13 "Slice 6 — Recorded dispatch and directive flow"; the cost milestone, #14 "Slice 7 — The cost crate".

## The claim

Agents build the machinery that finds bugs (a reference model, generators, assertions), and the machinery then generates and checks cases without spending tokens on each one. "This works with ordinary coding agents." The method: pick a complex repository, ask the maintainers which parts worry them, give a planning agent the proptest-praxis skill, which covers choosing targets, building reference models and generators, and turning failures into reviewable fixes. Good targets: custom data structures, query planners, graph algorithms.

## Results stated

- Codex: thirteen distinct correctness bugs reported upstream, twelve proposed fixes and two failing-test-only pull requests in the author's fork (a rollback that erases preserved review history; a completed plan replaced by an example inside a citation).
- uv: four fixes merged (optional-dependency activation during export; combining compatibility tags across wheel metadata rows; caching a workspace root twice; overrides losing optional-dependency guards).
- Prometheus: two fixes merged (a quantile function panicking on empty input despite its documented result; histograms losing counter-reset metadata when reducing schemas).
- Apollo Router, the author's first use: "more than 30 correctness bugs" in edge cases of internal data structures and algorithms in about three days of background agents, "despite thousands of existing tests and extensive snapshot testing."
- A second practitioner applied the skill to a cloud-sync database layer and found two merged bugs.
- Also run on jj and Babel in the background of a normal workday.

## The procedure

**Direct the investigation.** One model plans, another orchestrates, others write property tests in parallel worktrees. The planner audits beyond the initial leads: compare cached metadata with recomputation; compare incremental graph algorithms with fresh traversals; round-trip generated values through serializers, including across module boundaries. Investigate failures, including the test's own assumptions. Clear contract violations get regression tests and fixes; ambiguous cases go back for discussion. Bugs found by reading code also get regression tests. Expand each finding: a missed cache invalidation warrants checking every mutation of that state and every similarly maintained cache.

**Make operations interact.** Model a cached store against a plain map. Run generated sequences of put, get and remove on both and compare reads. Use a small key pool and overwrite with different values so stale results show:

```
Put("a", 1)
Get("a")       // returns 1; caches it
Put("a", 2)
Get("a")       // must return 2
```

Embed such patterns in longer histories with repeated removals and reinsertions. Inspect sample traces: "a million sequences that barely touch one key aren't buying you much." If a target yields no findings, suspect coverage: temporarily remove a cache invalidation and confirm the tests catch the stale result. Shrinking reduces a failing history to a minimal reproduction, from which a standalone regression test is extracted. A vector plus quadratic loops may suffice as the reference.

**Give people something they can review.** "Large agent-generated PRs full of test machinery are unwelcome." Reviewers get a unit test they can verify without the generator or the reference. The discovery suite stays on its own branch. Each confirmed bug gets a branch from main containing only the regression test and the fix; the test must fail before the fix and pass after. Security issues go through private channels. Examples of the delivered shape: equal maps hashing differently by insertion order ("Equal maps must hash equally"); a cache that returns the old answer after an input is removed; a serialization that reads back a value never written; cache keys that concatenate identically for different label sets.

**Friction.** Generator work sometimes triggers the harness's cybersecurity content notice; the author replies conversationally, and compacting and continuing has always worked.

**Run it.** Get the skill, point a planning agent at a repository; the skill repository also carries skills for writing agent prompts and plans.

## Caveats the author states

Several Codex fixes are proposed in a fork, not merged; the uv and Prometheus examples were awaiting review at writing. Ambiguous findings are discussed rather than filed as bugs. Test assumptions can be wrong and must be investigated. The content-notice friction has workarounds, not a fix.

## The skill itself (repository read 2026-10-08, agent-skills at fc1ec19, 2026-10-05)

The proptest-praxis skill is 20 KB and is the method in full. Its deliverables rule is the article's: a discovery branch holding reference implementations, generators, operation ASTs, invariants and investigative tests ("can be large, repetitive, and unoptimized; its correctness matters; production-level performance and polish usually don't"), and per confirmed bug a focused pull request with a deterministic regression test and a minimal fix, "independently reviewable without the discovery suite." Ambiguous findings stay separate from established contract violations.

**Four lenses held throughout:** history-dependent behavior (identical logical contents can conceal different caches, allocation histories, sharing and deferred work); generator support and sampling distribution ("omitted interactions are unreachable; possible interactions may still be vanishingly rare; inspect and improve the generated behavior before relying on increased run volume"); oracle independence and common-mode failures (shared helpers can make the implementation and the model agree on the same wrong answer); search feedback ("initially treat no findings as a coverage problem").

**A catalogue of testing opportunities,** each with its oracle form: custom collections and indexes against a plain model; caches and derived metadata against full recomputation; graphs against straightforward traversal with cycles, diamonds and deletion holes generated; planners and rewrites by semantic preservation with a small interpreter; state machines with fault injection on call k and recovery invariants; parsers and serializers by grammar-aware round trips plus malformed inputs; representation boundaries; equality, hashing and ordering through equivalent values built by different histories; metamorphic relations between bulk and individual operations; clone and snapshot isolation; and validators checked by constructing a valid object and introducing a targeted defect. Soundness is distinguished from completeness: "every emitted node being reachable does not establish that every reachable node was emitted."

**Generator construction:** an explicit operation enum replayed against production and the model; constructive generation of valid inputs with one constraint violated at a time; input-space partitioning crossed deliberately ("overwrite × warmed cache × shared snapshot × capacity boundary"); small key domains so overwrites and reuse happen; operands selected from reference state rather than from a possibly defective production result; targeted patterns (warm, mutate, query; remove, reinsert; snapshot, mutate either branch, inspect both; failed operation, continued use) embedded in arbitrary prefixes and suffixes; swarm testing with varied operation mixes; designed for shrinking. **Fault activation** is stated as a four-link chain: reachability, infection, propagation, revealability. **Mutation testing as harness calibration:** temporarily omit an invalidation, confirm the histories reach it and the assertions see it, use survivors to find the missing link, revert.

**Investigating failures:** reproduce, find the first divergence, inspect both the implementation and the oracle, establish the concrete contract violation ("a data structure or algorithm violation is sufficient; an end-to-end user-input reproduction is not required; surface genuine ambiguity for discussion instead of inventing a specification to make the test pass"); generalize the counterexample by varying conditions; variant analysis across adjacent operations and similar implementations; root-cause clustering to decide pull-request boundaries. The final deliverable is "a readable deterministic unit test with explicit expected behavior" that fails against the unmodified code for the claimed reason; "a seed, timeout, or disagreement with an opaque generated model alone is not the final deliverable."

**Roles:** a strong planning agent to select targets and assess findings, an orchestration agent, implementation agents each with a bounded investigation in its own worktree; an exploration-versus-exploitation allocation by expected information gain; passing harnesses kept as reusable search machinery with their commands, revisions and blind spots retained.

**Security findings** go privately through the organization's process; artifacts stay private until cleared.

## The delivered shape, from five sampled pull requests

Each is two files, the regression test and the fix, with additions of 35 to 233 lines. The bodies share one form: the concrete trigger, the incorrect behavior, the expected result, the fix; a validation paragraph naming the upstream commit, platform and toolchain, the exact test command, and the fact that "the new regression fails with upstream production code and passes with the fix" or "it panics with only the production repair removed and passes with it restored"; and an honesty line distinguishing a demonstrated contract violation from any unproven application-level impact ("current call sites guard empty inputs, so no expression-evaluation failure is claimed"; "this is core-structure hardening rather than a claim of a currently reachable planner failure"; "the broader lint check stopped on two upstream unused imports; it is not claimed as passing"). Two carry an AI-assistance trailer. One says "the broader property-testing machinery is maintained separately." The second practitioner's merged fix credits the article and notes that one of the two regressions does not use property tests in its final form.

## Handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-08.

- `vsdd-cli#839`: Slice 2 (Composition) phase-1a design — vsdd way: composition function + config-integrity ... [open]
