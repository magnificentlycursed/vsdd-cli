---
title: "Reference practice: the build and review phases (2a to 6)"
tags: ["design-input", "process", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---


## Design Specification

### status

Design input for the phase-primer rewrite. How the whitepaper's build, review, hardening and convergence phases are practised in Thermite, Peritus, OpenClaudia and crosslink, reconstructed read-only on 2026-10-08 (index: `vsdd-in-practice-reference-repositories-2026-10-08`; evidence: the four `practice-report-*` pages). The repositories do not use the phase names.

### 2a test suite generation (the red gate)

**Not practised as a phase in any repository.** Tests, proofs and implementation land in one commit per increment: Thermite's builder step is "tests plus production in the same commit"; Peritus B2 shipped 67 files at once; OpenClaudia's first slice landed source, end-to-end tests and data in one commit; crosslink's large commits touch source and tests together. Four hits for "failing test" in crosslink's 1,503 comments; two in Peritus's 3,300.

**Three substitutes carry the intent:**

1. **Failing-first for bug fixes.** Peritus's rule: "Every bug fix begins with a failing regression test at the narrowest meaningful boundary"; its bug-discovery design requires demonstrating that the regression fails before the fix and passes after. Honoured in fix commits ("baseline reducer failure, real permission-injection/restart/retry acceptance").
2. **Critic pins** (Thermite). A divergence is a committed failing test with a header stating the class, the authority, the expected value ("authority, not forge's own output") and the commit it fails against, marked ignore with the tracking issue while open. The critic's step five: "verify the test actually fails; if it passes, the candidate is not a divergence, drop it." The pin closes only when the fix lands and the marker is removed, never by a skip. Bootstrap sequence on one day: pin three parser divergences as failing tests, fix, re-pin, fix.
3. **Mutation testing** as the mechanical check for tests that would pass anyway: Peritus's weekly campaigns ("32 receipt and 23 cancellation mutants, 18 caught, 5 unviable, 0 missed"); Thermite's mutation scoring with a kill-ratio floor inside the product. Peritus also requires at least one negative executable test per proof invariant "that would fail if the guard disappeared", and tautological tests are themselves divergences in Thermite.

### 2b implementation

**Complete, not minimal.** "No stage is an MVP" (Peritus umbrella); "placeholder success paths and todo!() in reachable production code are prohibited"; "Every function body must contain a working implementation" (Peritus rules); a pre-edit gate blocks stubs, unwraps, panics and root-level lint suppressions outside tests (Thermite, ferrotorch); a new public API needs a non-test consumer in the same commit (Thermite).

**The unit is one tracker issue per increment, and its body is the dispatch prompt.** Thermite's increment issues name the authoritative spec by requirement and criterion, the kickoff plan for sequencing, the merged predecessor pull requests to build on, the loop discipline ("Follow the read-write-verify-commit loop; commit per coherent sub-unit; self-verify before each commit; revert anything that fails"), the self-verify commands including re-pinning the drift digests and running the registry check, and a stop rule ("If low on time or budget, stop at a sub-unit boundary and report what landed").

**Manifests and scope.** Builders carry a pre-declared manifest of about ten files and must stop and report "manifest needs expansion: file, because reason" rather than widen scope (Thermite). "Keep adjacent cleanup out of this change unless it is required to preserve compilation or the stated contract" appears in every OpenClaudia slice; discovered work becomes a new slice. Workers in Peritus edit only their frozen path set and never shared files or workspace-wide verification; after an incident, OpenClaudia's workers became code-only, with the driver inspecting every diff, running the gates serially, fixing and committing.

**Records.** A signed commit with a verification paragraph carrying integer counts; a result comment with the commit hash, file count, signature verification and gate outcomes; in Thermite's June, a commit template with design sources, requirement status and verification sections.

### 2c refactor

**Absent as a phase.** Thermite's fixer forbids "renames, restructuring, 'while I'm here' cleanup". What replaces it:

- **Architecture as policy, continuously** (Peritus): soft 400 and hard 700 source lines, root module 80, forbidden module names (common, helpers, manager, misc, utils), layer dependency rules, per-package owner and verification class, exceptions registered with owner and rationale (25 entries); enforced by a workspace task before every signed commit and in the hosted gate. "Architecture gate found 14 layout issues; split 12 responsibilities without changing wire/state semantics."
- **Register-only passes as gated base slices** (Thermite's tone pass): one agent per crate, an adversarial verifier confirming the diff is comments-only, everything else rebases onto it.
- **Mechanical lint fixes recorded inside the verification record** (OpenClaudia: "strict Clippy initial FAIL exactly 3, mechanical corrections applied, strict rerun PASS").
- A planned refactor sequence as its own design document (crosslink's architecture overhead map, executed as three pull requests with a progress table).

### 3 adversarial refinement

**Shape, common to all four:** a separate dispatch, read-only, over the exact tree, returning typed findings; scoped repair by the owning worker; a bounded re-check by the same reviewer; CI as the second reviewer.

**Strength, by repository:**

- **Thermite:** the critic is dispatched after every substantive builder or fixer run and looped until clean. It may not fix, approve, or give prose verdicts; the only verdicts are "generator must fix" and "no divergence found"; "there is no acceptable-drift verdict". Its finding is a runnable failing test plus a blocker issue titled "Divergence: <symbol> diverges from <authority>" plus an observation comment naming the test path; "the tests are the audit artifact". In the published hub, 151 of 282 issues are blockers and 92 titles begin "Divergence:". "The critic loop ran to convergence, finding and fixing 5 real divergences, each pinned by a failing test; final pass found no remaining divergence." Same model family as the builder.
- **Peritus:** an independent final review of the exact tree by a read-only subagent of a different model family, a distinct fresh principal per formally governed record ("Every new authorization enrolls a fresh independent reviewer; no historical reviewer identity is reused"). Findings are typed records with severity, blocking flag and disposition (fixed, invalid, superseded), each with a retained detail file, inside a content-addressed verdict with a mandatory review report ("an empty findings array is never the only retained evidence for a no-findings verdict"). Stop: "no blocking findings remain and every gate passed." Example: four high blocking findings, all fixed, on one record.
- **OpenClaudia:** one fresh-context pass by the orchestrator with a written changes-required or pass verdict, corrections, one re-review. Real teeth: the first slice's review failed on four substantive points (self-asserted effect IDs, trace-hash scope, unbounded artifacts, non-independent review identity). The reviewer is forbidden to edit.
- **crosslink:** architect redirect rounds and an independent audit comment inside the build session ("independent gate exposed a real fallback publication race"), ending in "independent audit PASS"; 23 of 25 follow-up commits on three large pull requests were CI fixes, each announced with the run id.

**Not observed anywhere:** a persona, negative prompting, a judgment-based exit, a multi-lens roster, refutation fan-out per finding, or an adversary that proposes fixes (the whitepaper requires a proposed fix; Thermite forbids it).

### 4 feedback integration

- **Findings become tracker issues**, fixed in their own commits, closed with a result comment and a machine-generated changelog line committed unchanged (OpenClaudia: 260 closes in 13 days).
- **Specification-level feedback** returns as a dated amendment paragraph appended to the governing document (Thermite: twelve on one re-audit day; "recorded as an Amendment", with requirement and criterion IDs unchanged) or an amendment commit tied to the slice issue (Peritus: five in nine days, then the umbrella froze). OpenClaudia's audit was never reopened; feedback became new slices and issues.
- **Scope held constant** during repair ("no B2 scope expansion"); exceptions carried explicitly across slices ("the six #1055 failures were a worktree fixture defect, so this slice did not edit worktree or sandbox code").
- **External loops:** benchmark failure journal to a remediation design to pull requests (Peritus); an external trust audit to a reconciliation table to stage increments (Thermite); a live human audit to a 196-comment issue to a 60-commit pull request (Peritus).

### 5 formal hardening

**Continuous and early, not fifth.** Peritus verifies Verus proofs per slice from the first slice under a no-cheating flag, with an empty trusted-computing-base baseline, and gates every pull request on them; proof coverage closure is tracked as gap issues with 917 evidence files. Thermite gates CI on a Lean axiom probe with an allowlist from the start, with correspondence drift tripwires and negative pin lemmas per increment. Fuzzing, mutation and chaos arrive later as campaigns with their own design document (Peritus's proactive bug discovery; Thermite's rotating-seed generated corpus). crosslink hardens continuously through audit, strict lints, property tests and nightly fuzzing; no proofs.

### 6 convergence

- **Slice level:** gate green locally, hosted on the pull request, and again on the merged commit on fresh main; independent review with no blocking findings; signed merge; changelog; issue closed with a result comment ("Closure complete").
- **Program level:** stage gates with checklists and pinned gate comments, the public headline flipping only at gate time (Thermite's rule R-GATE-1); a verified release policy reducing 25 criteria and 44 evidence requirements to Ready or NotReadyForProduction, never yet reached, while five releases shipped labelled pre-qualification (Peritus); an administrative merge of 191 commits while the backlog's own vocabulary says nothing is Verified (OpenClaudia); a closing result comment plus merge (crosslink).
- **Mechanical, never judgment-based.** Thermite: "never declare the goal complete until the mechanical check says so" (routed count equals status-table count, gauntlet green, corpus passing). Peritus: "no implicit success" everywhere; timeouts never accept; exhausting a budget never converts an incomplete run into success.

### divergences from the whitepaper in these phases

- No red gate for features; failing-first for fixes and pins; mutation as the substitute.
- Implementation complete, not minimal.
- No refactor phase.
- One typed read-only reviewer plus CI; no persona; no judgment-based exit; the adversary does not propose fixes in Thermite.
- Hardening early and continuous.
- Convergence by checklist and registry, with headlines at gate time; releases ship before the policy is met, labelled.

### what to take for the primers (candidates)

- 2a splits: red-first mandatory for fixes and pins; for new increments, negative tests per invariant plus a mutation floor, with tests and code landing together.
- 2b: complete under anti-stub gates, a manifest, a consumer rule for new public API, and an issue body that is the dispatch prompt with predecessors, loop discipline, self-verify commands and a stop rule.
- 2c: continuous architecture policy plus explicit register slices, or no phase.
- 3: one read-only reviewer per round with typed findings, blocking flag and dispositions; findings as issues fixed in their own commits; a bounded re-check; CI as the second reviewer; a written stop rule; a different model family where available; review dimensions enumerated in the receipt.
- 4: findings to issues; amendments as dated paragraphs in the slice document; the umbrella not reopened.
- 5: continuous from the first slice.
- 6: a mechanical checklist, headline at gate time, three evidence classes kept distinct, no implicit success.

