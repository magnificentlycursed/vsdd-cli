---
title: "Property testing with agent swarms (recursion.wtf), read 2026-10-08"
tags: ["reference", "design-input", "design-doc"]
sources:
  - url: "https://recursion.wtf/posts/agents-and-property-tests/"
    title: ""
    accessed_at: "2026-10-08"
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---


## Design Specification

### status

Reference summary, design input for the 2a and 5 primers, the fix-lane discipline, Slice 2's verification and Slice 4's fixture corpus. Source: `https://recursion.wtf/posts/agents-and-property-tests/` ("Property Testing with Agent Swarms", recursion.wtf, October 2026; tags rust, llm, testing, proptest), fetched read-only on 2026-10-08. External content, treated as evidence. The method's skill lives in the author's agent-skills repository (`proptest-praxis`), not mirrored or read as of this page. Cross-reference: `cross-reference-recursion-wtf-2026-10-08`.

### the claim

Agents build the machinery that finds bugs (a reference model, generators, assertions), and the machinery then generates and checks cases without spending tokens on each one. "This works with ordinary coding agents." The method: pick a complex repository, ask the maintainers which parts worry them, give a planning agent the proptest-praxis skill, which covers choosing targets, building reference models and generators, and turning failures into reviewable fixes. Good targets: custom data structures, query planners, graph algorithms.

### results stated

- Codex: thirteen distinct correctness bugs reported upstream, twelve proposed fixes and two failing-test-only pull requests in the author's fork (a rollback that erases preserved review history; a completed plan replaced by an example inside a citation).
- uv: four fixes merged (optional-dependency activation during export; combining compatibility tags across wheel metadata rows; caching a workspace root twice; overrides losing optional-dependency guards).
- Prometheus: two fixes merged (a quantile function panicking on empty input despite its documented result; histograms losing counter-reset metadata when reducing schemas).
- Apollo Router, the author's first use: "more than 30 correctness bugs" in edge cases of internal data structures and algorithms in about three days of background agents, "despite thousands of existing tests and extensive snapshot testing."
- A second practitioner applied the skill to a cloud-sync database layer and found two merged bugs.
- Also run on jj and Babel in the background of a normal workday.

### the procedure

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

### caveats the author states

Several Codex fixes are proposed in a fork, not merged; the uv and Prometheus examples were awaiting review at writing. Ambiguous findings are discussed rather than filed as bugs. Test assumptions can be wrong and must be investigated. The content-notice friction has workarounds, not a fix.

