---
title: "VSDD in practice: crosslink (agent report, 2026-10-08)"
tags: ["evidence", "reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---


## Design Specification

### 0. sources read, exclusions, inaccessible material

### Maintainer identity used for filtering
- GitHub login `dollspace-gay`; git author strings `Doll` (560 commits) and `dollspace.gay` (242, same e-mail as `Doll`) and `dollspace-field` (49). 587 non-merge maintainer commits, 2025-12-27 to 2026-09-12. The task named two author strings; the third (`dollspace.gay`) shares the first's e-mail and is treated as the maintainer.
- Hub agent identities attributed to the maintainer: `dashboard-bootstrap` (a `--role driver` identity; its own commit d3e7772c describes re-initialising it as the driver; active 2026-04-21 to 2026-06-25), `anon-ab6221f8` (**inferred** maintainer's primary interactive identity: 478 of 810 hub issues created, 350 of 1503 comments, active 2026-03 to 2026-08-25, and its 2026-08-19 comments narrate the same PR #89/#90 work the maintainer's commits carry), and the Aug–Sep 2026 kickoff/subagent identities `w4c5`, `sQl8`, `anon-62b057cf`, `anon-396e79b5`, `anon-6aac1553`, `anon-aeb72d19`, `xXMe` (their events reference `[CL-793..798]` commits authored by `dollspace-field`).
- Hub agent identity `ldsw` (2026-10-08, hub issues #799–#811) belongs to the excluded login (its issue titles duplicate GitHub issues #120–#128 filed by that login) and is excluded.

### Read (sizes/dates)
- Pristine mirror root: `CLAUDE.md` (2.7 KB), `AGENTS.md` (1.3 KB), `quality.md` (4.6 KB), `CHANGELOG.md` (62 KB, head 90 lines + version index), `justfile` (7.6 KB), `README.md` not read, `REVIEW-DESIGN-DOC-KICKOFF.md` (24 KB; head, §3.3, §5 Phase 3, §6), `DESIGN-CROSSLINK-DASHBOARD.md` (37.7 KB; header, §14–15), `DESIGN-CROSSLINK-OPS.md` (27.5 KB; header, §14).
- `.design/hub-v3-per-agent-refs.md` (15.5 KB, read in full), `.design/patrol-autonomous-maintenance.md` (40.9 KB; headings, Design Decisions, Out of Scope, Milestones), `.design/issue-scheduling-fields.md` (9.6 KB; headings, Open Questions), `.design/first-class-codex-provider-support.md` (43.7 KB; head, Verification strategy, Implementation amendments, Open Questions, Out of Scope), `.design/crosslink-architecture-overhead-map.md` (79.6 KB; headings, first 56 lines).
- `docs_src/guides/design-workflow.qmd`, `kickoff.qmd`, `swarm.qmd`, `session-workflow.qmd` (in full), `docs_src/reference/kickoff-report.qmd`, `state-files.qmd`, `rules.qmd` (heads).
- Skills under `crosslink/resources/agent/skills/`: `design`, `kickoff`, `commit`, `review-pre-commit`, `qa`, `preflight`, `architect`, `dev-release` (all `SKILL.md`, in full; 0.8–2.3 KB each). `crosslink/resources/agent/instructions/crosslink-agents.md`.
- Rules: `.crosslink/rules/*.md` (29 files) and `crosslink/resources/crosslink/rules/*.md` (31 files): **all zero bytes** (the path named in the task, `crosslink/resources/agent/rules/`, does not exist).
- Hooks: `.crosslink/hook-config.json`, `.claude/settings.json`, heads and gate tables of `crosslink/resources/agent/hooks/work-check.py`, `prompt-guard.py`, `post-edit-check.py`.
- Prompt template `crosslink/src/commands/kickoff/prompt.rs` (lines 1–160, 246–275, 318–345).
- CI: `.github/workflows/{ci,ci-feature,container-image,docs,fuzz-nightly,publish,release-builds}.yml` (step outlines).
- Git history: author census; per-month counts; maintainer commit bodies (latest 40, a 2026-03/04 sample of ~25); trailer census; per-file history of every design/review document; merge-commit census.
- Hub (fresh bare clone, refs `crosslink/*`): `crosslink/checkpoint` (119 commits, `state.json` 1.9 MB, 810 issues), `crosslink/meta` (`hub.json`, `allowed_signers`), `crosslink/knowledge` (48 commits, 34 pages; `index.md`, `testing-strategy.md`, `git-flow-branch-strategy.md`, `adversarial-review-adr.md` head, `adversarial-review-v1.md` headings), ten `crosslink/agents/*` refs (event logs and heartbeats, kind census and comment text for the maintainer's identities), the reconciliation genesis checkpoint 6d6107e1 (793 issues, 1503 comments) and the archived v2 SQLite snapshot (tree only).
- GitHub, current repo: all 44 merged PRs listed; 12 maintainer PRs viewed (#78, #84, #85, #86, #88, #89, #90, #91, #92, #93, #95, #97); all 90 issues listed. GitHub, former home `forecast-bio/crosslink` (public): maintainer PR census (193 merged), PRs #147, #594, #609, #632, #638 viewed; maintainer issue census (148), issues #113, #364, #429, #593, #629 viewed.
- Whitepaper gist (fetched once, summarised).

### Excluded
Everything authored by login `magnificentlycursed`: upstream PRs #32, #35, #38, #46, #50, #51, #54, #63, #64, #65, #72, #76, #77, #80, #103, #107, #108, #112, #114, #115, open #129; GitHub issues #9–#128 by that login; `.design/dispatch-dials-and-injection-seam.md`; hub identity `ldsw` and hub issues #799–#811. They are mentioned below only where the maintainer's own record reacts to them (e.g. PR #88's promotion body lists merges of them).

### Could not access or settle
- The GitHub home before `forecast-bio` (commits from 2025-12 to 2026-02 reference "GH #" numbers below ~100 whose issues now live in `forecast-bio`; the earliest PRs there start 2026-02). The 2025-12/2026-01 period has no PR record.
- The hub's v2-era per-issue event logs were deleted at finalize (2026-08-19); March–June comments survive only as reduced objects inside the genesis checkpoint, without per-issue grouping in the file I parsed (the parse saw them as one flat set).
- `gh pr list --author` on the former org hits a GraphQL node limit; the census was taken from an unfiltered list instead.
- Old-org PR bodies were sampled (5), not read exhaustively (193).

### 1. document set and authority chain

### INTENDED (docs, skills)
- `CLAUDE.md`/`AGENTS.md` are "project documentation, not a substitute for the current user request" (CLAUDE.md line 3). They prescribe: start from `develop`; focused branch; "Record substantial work in Crosslink when a session and issue are available"; conventional commit subjects; "Commit messages must describe the delivered behavior without provider attribution trailers"; GitHub work as `owner/repository#number`, local Crosslink work as `#number`.
- `quality.md` is a generic code-quality standard ("Inject this skill on ANY code generation"); it is not referenced by any skill or hook I read.
- The design document is the unit of specification: `/design` writes `.design/<slug>.md` with a fixed skeleton (design `SKILL.md`: Summary, User-visible behavior, Requirements, Acceptance criteria, Current architecture, Proposed design, Data and compatibility, Failure handling, Security considerations, Verification, Rollout and rollback, Open questions, Out of scope). `design-workflow.qmd` shows a shorter REQ-n/AC-n skeleton and a validator that prints `[PASS]/[OPEN]` lines; the guide says validated designs are "automatically stored as crosslink knowledge pages" and that "a plan comment is also recorded on the issue".
- Amendment path: `/design --continue <slug>` "detects which open questions you resolved ... updates requirements and acceptance criteria". The kickoff prompt (`prompt.rs` `build_canonical_doc_stanza`) makes the doc "canonical, read-only input": it is chmod 0444 in the worktree, mounted read-only in containers, SHA-256 checked post-run, and the agent must "surface the proposed delta in your final report or in a Crosslink issue comment. Do not rewrite the source."
- Records of decisions: `crosslink issue comment ... --kind decision|plan|observation|result|blocker|resolution` plus `crosslink issue intervene` (prompt.rs lines 257–268).
- Rules: `docs_src/reference/rules.qmd` says every bundled rule file "is zero bytes in this release"; the paths remain so `init --update` can blank legacy content while preserving the loader. `CLAUDE.md`, `AGENTS.md`, `preflight/SKILL.md` and `crosslink-agents.md` each restate that rules must remain zero bytes.

### OBSERVED
- Five `.design/` documents exist (`git log` per file). Two were written and committed by the maintainer inside the implementing PR series: `hub-v3-per-agent-refs.md` (added in 8abe7c01, 2026-06-11, the hardening PR that precedes PR1 of the series; amended once in 980130f8, 2026-06-12, by inlining a "**DELIVERED (754b):** also deleted —" paragraph into REQ-10) and `first-class-codex-provider-support.md` (4 commits on 2026-08-16, all inside PRs #84–#86; it carries an "Implementation amendments" section and closes with "No unresolved architecture questions remain"). The third, `crosslink-architecture-overhead-map.md` (2026-08-19), is an audit-plus-design document ("Status: repository-grounded audit baseline", "Revision: 35255a17 (origin/develop)"), committed with the first reconciliation commit 8aaacb2a and amended twice (fca715c0, 7be09240 on 2026-09-01) and again per hub #797 ("added an authoritative nine-phase progress table, marked Phase 0 ... delivered", 2026-09-03). Two older docs (`patrol-autonomous-maintenance.md`, `issue-scheduling-fields.md`) and `REVIEW-DESIGN-DOC-KICKOFF.md` entered this repository only via a snapshot merge (1b0ad187, 2026-06-12) and the 2026-08-04 snapshot 3bd4cd0e; their authorship dates predate that (REVIEW is dated 2026-03-03; the scheduling design exists as a knowledge page from 2026-03-23).
- Two root-level `DESIGN-*.md` documents follow an older, longer template (Status/Issue/Authors/Reviewers header, numbered sections, Open questions Q1–Q6, Phased rollout, Success criteria). `DESIGN-CROSSLINK-DASHBOARD.md` was amended during the build to record progress ("docs(dashboard): mark Phase 2 complete, defer agent-request to follow-up" 0691d26b; "note that Phases 1-5 are shipping on a single PR" 30564bea, both 2026-04-20) and its §14 says "the project owner elected to ship Phases 1–3 ... on a *single* PR". `DESIGN-CROSSLINK-OPS.md` is an operator runbook with a revision log.
- Decisions are recorded in three places: inline in the design doc ("### Q1 ... — RESOLVED/DECIDED", patrol §Design Decisions, scheduling §Open Questions), in commit bodies (the 2026-04 bodies carry `## Root cause / ## Changes / ## Verified` sections, e.g. d3e7772c), and as hub `decision` comments (80 in the genesis checkpoint). The dashboard doc's OQ-1 (hidden refs) was overturned by a hub `plan` comment on 2026-06-12 ("user requirement: hub MUST be visible on browsable branches -- corrects the OQ-1 hidden-refs decision"), i.e. the reversal is recorded in the tracker, not in the design file.
- Pins and gates that keep documents honest: (a) `tests/ac10_deleted_machinery.rs` turns design AC-10 into a permanent grep test ("It caught five stale doc references during this very commit's preparation", 980130f8); (b) `python3 crosslink/scripts/sync-codex-plugin.py --check` in CI ("Verify generated Codex plugin assets") keeps generated provider assets hash-synchronised with the canonical skills; (c) the kickoff doc SHA-256 check; (d) `just _docs-lint` fails the docs build on broken asset references. There is no mechanical pin tying a `.design/` document to the code it describes (the AC-10 test is the only AC that became executable).
- The knowledge branch holds a "Design Documents" section (`index.md`), but of 48 commits only two are the maintainer's (`issue-scheduling-fields` 2026-03-23; `first-class-codex-provider-support` 2026-08-15); the rest are the collaborator's (login `maxine-at-forecast`). The "automatically stored as knowledge page" step was therefore executed for 2 of the 5 `.design/` docs.

### 2. per phase: intended versus observed

The repository never uses VSDD's phase names; the mapping below is mine. "Not observed" means no record in the maintainer's commits, PRs, design docs or hub comments.

### 1a Behavioral Specification
- INTENDED: `/design` Explore→Draft with "Requirements must be testable. Acceptance criteria must state observable completion conditions" (design `SKILL.md`); REQ-n/AC-n numbering (design-workflow.qmd).
- OBSERVED: present for the maintainer's own designs. `hub-v3-per-agent-refs.md` has REQ-1..13 and AC-1..11 with every AC cross-referencing its REQ ("(REQ-1, REQ-3, REQ-4)"); the codex doc has Requirements and Acceptance Criteria sections; the overhead map has a "### Acceptance criteria" subsection for reconciliation; the scheduling doc has both. Commit messages cite them: 20 maintainer commit lines mention `REQ-`, 26 mention `AC-` (e.g. ea272b13 "REQ-1/REQ-2", "seeds the design's AC-1"). Hub comments contain 65 `REQ-n` and 47 `AC-n` mentions. The older dashboard doc uses user stories (US-1..5) and success criteria (SC-1..) instead.
- Gap versus the whitepaper: no "edge case catalog" or non-functional section as a named artifact; the design skill's "Failure handling"/"Security considerations" headings are the nearest, and the three maintainer-written `.design/` docs do not use the skill's 13-heading skeleton (they use the older 5–7 heading form).

### 1b Verification Architecture
- INTENDED: design `SKILL.md` requires a "## Verification" section; the kickoff pipeline extracts ACs into `.kickoff-criteria.json`.
- OBSERVED: the codex doc has a "### Verification strategy" (unit/CLI fixture/hook subprocess/container smoke/plugin/regression layers); the hub-v3 doc puts verification in its ACs themselves (a crash-injection harness, 100-iteration concurrency runs, a byte-identical checkpoint check, a grep test). No "provable properties catalog" or "purity boundary map"; property tests (25 `proptest!` sites in `crosslink/src`) and fuzz targets (`crosslink/fuzz`, nightly workflow) exist as tooling but are not planned per feature in any design doc. **Not observed as a separate phase.**

### 1c Spec Review Gate
- INTENDED: the `/design` validator (`[PASS] ... [OPEN] 1 unresolved open question`), resolution of open questions, then `crosslink kickoff plan <doc>` gap analysis. `architect/SKILL.md`: "Resolve unclear product choices with the user before committing to an irreversible interface".
- OBSERVED: a review gate exists in the recent (Aug–Sep 2026) practice, but on a *pre-flight comment*, not on the design file: hub #793 `plan` comment "PRE-FLIGHT — VERIFIED REMOTE RECONCILIATION PUBLICATION / North star: ..." followed by `observation` "paused at the architect pre-flight approval gate. No source files were changed" (2026-08-31 21:54–21:55); hub #794 "Architect preflight ... awaiting explicit user approval" and the `human` comment "directed implementation to proceed without further ceremonial approval. The recorded preflight is authorized as the complete implementation contract" (2026-09-01 03:48–03:50). Earlier designs show no review round: hub-v3 doc committed 2026-06-11 and PR1 merged the same day; dashboard doc and Phases 1–3 the same day (2026-04-20). The REVIEW-DESIGN-DOC-KICKOFF document is itself a Phase-1c-shaped artefact (a model-written review of the codebase against a vision, "Reviewer: Claude", 2026-03-03) and was acted on the same day (11 commits prefixed with a cuneiform glyph on 2026-03-03/04 implement its Phase 2–4: `feat: add spec validation loop for kickoff agents`, `feat: add structured machine-readable build reports`, `feat: add /design skill`).

### 2a Test Suite Generation (red gate)
- INTENDED: not described anywhere in the docs or skills. The kickoff prompt orders "Implement the feature fully ... Run tests ... Document results"; `review-pre-commit` runs "the smallest complete test and build set"; nothing asks for failing tests first.
- OBSERVED: **not observed.** Tests and implementation land in the same commits (every large maintainer commit touches both `src/` and tests; test-only commits are rare and are fixes of tests: 2026-09 has three, all `fix(ci)/fix(test)`; 2026-04 has none). The hub has 4 hits for "failing test" phrasing across 1503 comments and 0 for "TDD"/"red gate". The nearest practice is AC-10 (a grep test written *with* the deletion it guards) and "Two-agent concurrency test ... seeds the design's AC-1" (ea272b13), both written alongside code.

### 2b Minimal Implementation
- INTENDED: kickoff step 7 "Implement the feature fully (no stubs or placeholders)"; `post-edit-check.py` flags TODO/FIXME/`todo!()`/`unimplemented!()`; `architect/SKILL.md` "Break the work into complete, verifiable increments".
- OBSERVED: the maintainer ships in large vertical increments, not minimal ones: PR #84 +19,691/−3,344 (261 files), #90 +26,642/−5,527 (133 files, 13 commits), #92 +5,535/−2,059, #638 (old org) +2,464/−12,569. The design's own sequencing is honoured when it exists (hub-v3 "PR 1..4" → old-org PRs #631, #632, #634, #638 mapped to hub #751–#754, all within 2026-06-11/12; the closing hub comment of 2026-06-12 lists the full series "#630 hardening, #631 HubSource abstraction, #632 write path + dual-write soak, #633 full event sourcing, #634 migrate command, #635 v3 operation mode, #636/#637 migration fixes, #638 v2 machinery deletion").

### 2c Refactor
- INTENDED: `qa/SKILL.md` and `architect/SKILL.md` check "dependency direction, module ownership, cohesion, duplication"; `quality.md` size limits.
- OBSERVED: refactor is a planned slice, not a post-green step: the overhead map's "Recommended refactor sequence" (Phase 0 reconciliation, Phase 1 application boundary, Phase 2 causal frontiers) was executed as PRs #90, #92, #95 with the doc updated to a "nine-phase progress table" (hub #797). 10 maintainer commits carry the `refactor` prefix; `refactor(shared_writer): replace update_issue's 8-arg signature with IssueUpdate struct` (6776985c) is a representative in-series refactor.

### 3 Adversarial Refinement
- INTENDED: `--verify thorough` adds an "Adversarial Self-Review" (prompt.rs: debug code, commented-out code, unintended changes, error handling); `qa/SKILL.md` is an "evidence-based architecture, correctness, security, and maintainability review" ending in `pass | pass with risks | changes required`; `architect/SKILL.md` verdict `ready | changes required | blocked ...`.
- OBSERVED: adversarial review is real but is performed by a *second agent session in the same build*, not by a fresh-context adversary after convergence. Hub #793/#794 (2026-09-01): "Architect review REDIRECT: initial implementation is not commit-safe. Release blockers are ..."; "Second architect REDIRECT: independent review found ReadyCurrent/Adopt could overwrite ..."; `audit` comment "Independent gate exposed a real fallback publication race: production_importer_fallback_two_clone_race_has_one_verified_adopter failed ..."; "Hostile production-path audit added four no-loss blockers". The result comment then reports "independent audit PASS". In the June series the term is "Director-verified gates" (7 hits). The word "adversarial" appears 59 times in hub comments; "Director" 7; "independent audit" 3. Earlier (2026-02-28/03-01) a "full-system adversarial review" was run as five parallel streams by the collaborator's agents (knowledge page `adversarial-review-adr.md`, GH #364), and the maintainer closed its punch list (old-org issues #454, #462, #472 "Hub: 20+ best-effort error-swallowing calls").

### 4 Feedback Integration
- INTENDED: `kickoff-report.qmd` verdict routing (`fail` → fix before proceeding; `needs_clarification` → unresolved_questions); design-workflow "Resolved questions become concrete requirements".
- OBSERVED: findings from review and from CI are integrated in-branch as further commits on the same PR (PR #90: 13 commits of which 9 are `fix(ci)/fix(windows)`; PR #92: 7 commits, 6 fixes; PR #95: 5 commits, 4 fixes), each announced by a hub `result` comment ("Committed 113c2f71: fix(ci): shard Windows stress workloads ..."). Spec-level feedback goes back into the design file by amendment (overhead map 3 amendments; hub-v3 REQ-10 "DELIVERED" note) or into the tracker (OQ-1 reversal). The kickoff report mechanism itself is barely used: "kickoff-report" appears 6 times in hub comments and `.kickoff-criteria.json` 0 times.

### 5 Formal Hardening
- INTENDED: none of the docs mention proofs; CI runs `cargo audit`, strict clippy with `unwrap_used`/`expect_used` warnings, proptests on Ubuntu, fuzz nightly (300 s per target).
- OBSERVED: hardening is continuous, not a phase: `fix(deps): remediate security advisories [CL-796]` (PR #94), the security-labelled hub issues (25), signing enforcement (`signing_enforcement: audit` in hook-config, `allowed_signers` on `crosslink/meta`, 274 `unsigned_event_warnings` in the live checkpoint). No formal proofs, mutation testing (0 hits) or purity audits. **Not observed as a phase.**

### 6 Convergence
- INTENDED: the swarm model has phase gates (`crosslink swarm gate N` runs the full suite) and checkpoints; the kickoff report ends with a criteria summary.
- OBSERVED: convergence is declared by the maintainer's own closing `result` comment and PR merge: "DELIVERED. Full hub v3 program complete across 8 PRs: #630 ... #638 (net -10k lines)" (dashboard-bootstrap, 2026-06-12); #797 "Delivered automatic repository reconciliation ... through merged PRs #90, #91, and #93". There is no fresh-context adversary pass after delivery; the signal is "all gates green + merged".

### 3. roles and dispatch shape

### INTENDED
- Roles in the shipped material: the human "user" whose "present request determines whether to commit, push, open a pull request, or merge" (AGENTS.md); the "driver" identity (`agent.json` `role: driver`, signs hub commits with the human's key, d3e7772c) versus "agent" identities for kickoff/swarm worktrees; the "architect" (skill: plans/reviews changes with cross-subsystem effect); "qa" reviewer; "sentinel" (patrol design: an autonomous maintenance daemon that triages signals and dispatches reproduce/fix agents, "nothing merges without a human").
- Dispatch: `/kickoff` → feature branch + worktree + agent identity + `KICKOFF.md` prompt + tmux/container; `crosslink swarm init --doc` decomposes a design into phases with gates and budget; `crosslink kickoff plan <doc>` for read-only gap analysis. `.kickoff-metadata.json` records provider/model/effort/budget.

### OBSERVED
- The maintainer dispatched through kickoff-style identities from March onward: 200 of 810 hub issues were created by identities of the form `<parent>--<slug>` (kickoff agents) and 31 by `m1` (a 2026-03 driver prefix, e.g. `m1--full-system-adve...`). In the Aug–Sep series each PR has one or two short-named agent identities (`w4c5`, `sQl8`, `anon-62b057cf`) whose logs show a three-party shape: a "root architect" session that writes the pre-flight and "is preparing independent verification and will not monitor CI", an "implementation agent" that "owns source edits", and the human who "owns the final push" ("branch is intentionally not pushed"; `intervention` "Attempted: git push -u origin ..." blocked by hook, xXMe 2026-08-19). Fan-out is one implementer per issue; I found no evidence of parallel implementers on one design in the maintainer's own work.
- Swarm: 13 hub issue titles contain "swarm" (all about building the feature); "swarm launch" appears twice and "swarm gate/init/checkpoint" zero times in 1503 comments. **The maintainer did not observably use swarm orchestration on crosslink itself.** Phased builds were done as commit sequences on one branch (dashboard, 2026-04-20) or as sequential PRs (hub v3, reconciliation).
- Sentinel: the design's V0 skeleton was implemented (`feat: add sentinel module skeleton with CLI dispatch (#650)`, `feat: add sentinel database schema v16 migration (#651)`, 2026-04-10) and a cpitd source added 2026-06-12; no hub record shows sentinel dispatching work on this repository.
- Manifests: `.kickoff-metadata.json`/`.kickoff-report.json` are worktree files, git-excluded; none are in the hub. The only dispatch "manifest" that survives is the hub event log (IssueCreated → LabelAdded → LockClaimed → plan → ... → result → LockReleased → handoff), which is complete for the Aug–Sep issues.

### 4. review conduct

### INTENDED
- `review-pre-commit`: checklist (diff review, fmt, lint, tests, build, generated assets, security scan, tracking state) each `pass|fail|not applicable` with evidence; "A failure blocks the commit until fixed or explicitly accepted by the user".
- `qa`: findings "from highest to lowest impact", each with location, observed behavior, consequence, evidence, corrective direction; separate "confirmed defects, risks, and optional improvements"; close with checks run/unavailable and a verdict.
- `architect`: "Run the relevant checks yourself. Separate verified facts from assumptions"; verdict with evidence; "A failed check is investigated at its cause before proposing a patch".
- `--verify thorough` self-review list (prompt.rs).

### OBSERVED
- GitHub review is essentially absent for the maintainer: 0 reviews and 0 reviewer comments on all 12 sampled current-repo PRs (one self-comment on #84 reporting a Codex smoke run); in the former org 1 of 193 maintainer PRs has a review and 14 have any comment. PRs are opened and merged the same day in 10 of 12 sampled cases (#88, the develop→main promotion, merged the next day; #90 was opened 2026-08-19 as a draft and merged 2026-09-01 after 13 commits).
- Review happens inside the session and is recorded in the hub as `note`/`audit`/`observation` comments: two "Architect review REDIRECT" rounds and one `audit` "Independent gate exposed a real ... race" on #793 within three hours (2026-09-01 00:18–03:17), ending in `result` "independent audit PASS". On #798 the driver intervened once ("Driver clarified that checking the failed runners included implementing the correction") and the agent recorded `decision` "shard workload rather than raise the timeout" — a review-shaped exchange with findings, a decision and closure in four comments.
- Findings-to-closure: CI is the second reviewer. 23 of the 25 commits on PRs #90/#92/#95 after the first are CI/platform fixes, each with a `result` comment naming the run id ("Committed d84021b after CI run 33566626103 ...", "CI is fully green on 0de7e01c. Main CI run 33569258083, feature CI run 33569253648 ...").
- The June 2026 pattern (dashboard-bootstrap) names the reviewer "Director" and the loop is: implement → "Director-verified gates: 1767 lib + 2776 bin + 194 integration" → "Awaiting push" → "GH#NNN auto-closed by PR merge ... Complete."

### 5. records discipline

### Commit messages (OBSERVED)
- Conventional prefixes dominate: `fix` 191, `feat` 167, `style` 35, `chore` 30, `docs` 16, `test` 11, `refactor` 10, `ci` 6; 11 subjects carry a cuneiform-glyph prefix (2026-03-02/04 only); early (2025-12/2026-01) subjects are free-form ("Add", "Fixed", "Update").
- Issue references by era: `(#NNN)` = hub issue in the subject (170 lines, e.g. `(#754)`); `GH #NNN`/`GH#NNN` = GitHub issue (126 lines); `gh#NN` (27) in the mid-2026 period; `[CL-NNN]` suffix in `dollspace-field` subjects (80 lines, Aug–Sep 2026). `Closes #` 45 lines, `Fixes #` 4.
- Bodies: the 2026-03/06 era writes long bodies with `## Root cause / ## Changes / ## Verified` or "Tests: 1761 lib + 2765 bin + 194 integration green ... Cumulative 754b diffstat: +2381/-12542" (980130f8); 417 commit lines carry `Co-Authored-By: Claude ...` (through 2026-06). The 2026-08/09 era has one-line subjects, bodies in only 3 of the latest 40 commits, and no trailers; `CLAUDE.md` now forbids "provider attribution trailers". Verification detail migrated from commit bodies to PR bodies and hub `result` comments ("Committed: feat(checkpoint): add per-agent causal frontiers [CL-798] | Files: 20 files changed ... | Verification: fmt, strict Clippy, all-feature tests, doctests, release, Windows GNU, plugin sync").
- `chore(changelog)` commits are "Auto-generated by the crosslink issue close hook" (bb3a52c9) — the CHANGELOG is appended on issue close (hub #777 records a bug in that appender).

### PR bodies (OBSERVED)
- Current-repo form: `## Summary` (bullets) + `## Verification` (command list with counts: "cargo test --all-features: 5,473 passed, 4 ignored") and sometimes `## Notes`/`## Safety`/`## Runtime proof`/`## Problem`/`## Behavior after this change`/`## Safety boundaries`. Closing lines `Closes #798` or `Crosslink: #795` reference hub ids, not GitHub ids. Former-org form (#632, #638): the body opens by naming the design and requirement ids ("PR2 of the hub v3 migration (`.design/hub-v3-per-agent-refs.md`, REQ-1/REQ-2; crosslink issue #752, second of the #751-#754 sequence)") and ends with ACs seeded. #609 contains a disposition table of the issue's five failure modes and a note that an alternative was tried on a separate branch first.

### Issues and comments (OBSERVED)
- The maintainer's issues live in the hub (810; 762 closed; labels enhancement 292, bug 210, feature 74, ops 26, security 25) and, before July, also on GitHub in the former org (148 of 357 issues there; e.g. #178 "Phase 2: Design document ingestion for kickoff", #186 "Define design document format specification", #194 "Instrument agent prompt to produce structured .kickoff-report.json"). In the current GitHub repo the maintainer authored zero issues; all 90 are by outside reporters, 84 of them by the excluded login. Hub issues frequently wrap GitHub ones ("Fix GH#604: atomic JSON writes in hub cache", "Review GH#629: assess staleness after hub v3 migration"), so the hub is the work ledger and GitHub is the intake.
- Comment kinds (genesis checkpoint, 1503 comments to 2026-09-01): result 738, plan 271, handoff 146, note 145, decision 80, observation 61, intervention 37, resolution 19, blocker 3, progress 1, human 1, audit 1. The `result` comment is the dominant record; `plan` before commit was enforced for a period ("Implementing two hard enforcement hooks in work-check.py: (1) pre-commit gate requiring --kind plan comment ... (2) issue close gate requiring --kind result comment", anon-ab6221f8 2026-03-23); this repo's `hook-config.json` now sets `comment_discipline: encouraged`. `handoff` includes many auto-generated "Session auto-ended (stale after N minutes). No handoff notes provided." lines (w4c5 has 13 handoffs, 9 of them auto).
- Interventions record hook false positives faithfully and repeatedly: `git merge-base` was blocked as a "git merge" mutation on 2026-06-13, 08-19, 09-01 and 09-02 (four separate agents) before it was allow-listed (`work-check.py` line 64 lists `git merge-base`).
- Hub versus GitHub: hub holds issues, locks, plans, results, handoffs, interventions, signatures; GitHub holds PRs (bodies with verification), outside-reported issues, CI runs. Knowledge pages hold conventions and the collaborator's ADRs; the maintainer contributed two design-doc copies.

### 6. gates and tooling

| Gate | What it checks | When |
|---|---|---|
| `work-check.py` (PreToolUse on Write/Edit/Bash) | blocks `git push/merge/rebase/reset/clean/stash/tag/...` for drivers; narrower list for agents (`agent_overrides.blocked_git_commands`); gates `git commit` on an active issue (`gated_git_commands: ["git commit"]`; agents in this repo: `[]`); optional plan-comment-before-commit; readiness state ("repository work blocked while readiness is ..."); dashboard pause | every tool call |
| `post-edit-check.py` (PostToolUse) | stub/placeholder patterns (TODO, FIXME, `todo!()`, `unimplemented!()`, bare `pass`) | after each edit |
| `prompt-guard.py` (UserPromptSubmit) | loads `.crosslink/rules/*.md` (all zero bytes) and the external-content notice | each prompt |
| `pre-web-check.py` | provenance notice for WebFetch/WebSearch | before web tools |
| `session-start.py`, `heartbeat.py` | handoff replay; agent heartbeat to own ref | session start; after tool use |
| `kickoff` doc stanza | design doc chmod 0444 / read-only mount; post-run SHA-256 compare | per kickoff |
| `.kickoff-criteria.json` → `.kickoff-report.json` | per-AC verdicts with evidence (`pass|fail|partial|not_applicable|needs_clarification`) | end of a `--doc` kickoff |
| `just ci` = `lint build test`; CI `ci.yml` | `cargo fmt --check`; `cargo clippy -- -D warnings -W clippy::unwrap_used -W clippy::expect_used`; doctests; release build; Codex plugin drift check; `cargo audit`; test matrix Ubuntu (with proptests) / macOS / Windows (bounded partitions); reconciliation fixture tests; provider hook fixture tests; readiness integration tests; VS Code extension compile/lint/tests | push/PR to develop/main |
| `ci-feature.yml` | build, plugin check, reconciliation fixtures, unit, integration, provider hooks | feature branches |
| `container-image.yml` | multi-arch build, provider smoke (both CLIs, non-root, login refusal, isolated volumes), post-publish smoke | PR / develop push / dispatch |
| `fuzz-nightly.yml`, `publish.yml` (tag must match Cargo version), `release-builds.yml`, `docs.yml` (`just render-docs` with collision and link lint) | nightly / tags / pushes |
| `tests/ac10_deleted_machinery.rs` | greps production source for deleted v2 symbols | every test run |
| Hub: signed events (`signed_by`/`signature` per event), `allowed_signers` on `crosslink/meta`, `signing_enforcement: audit` | attribution; unsigned events are warned (274) not rejected | every hub write/reduce |
| Daemon readiness barrier (PRs #90–#93) | fails closed on unreconciled/stale repository state before any mutation | daemon start / each mutation |

Grade note: the git-command and commit gates are hook-level friction (an agent can run outside the hook); CI and the readiness barrier are mechanical blocks; `comment_discipline` is configuration-dependent.

### 7. cadence and proportions

- Maintainer non-merge commits per month: 2025-12 19, 2026-01 43, 02 58, 03 219, 04 135, 05 33, 06 30, 07 0 (July has only merge commits), 08 23, 09 27. Commits touching `.design/`/`DESIGN-*`/`REVIEW-*`: 03 5, 04 8, 06 2, 08 5, 09 2 — about 3–20 % of src-touching commits in a design month, zero in others. Docs-only commits: heavy in 2025-12/2026-01 (7 and 20 of 19/43) when the repository was documentation-first; 4–21 per month after.
- PRs merged by the maintainer (merge commits): 01 5, 02 24, 03 129, 04 19, 05 21, 06 26, 07 19, 08 14, 09 7. Hub issues created: 02 31, 03 614, 04 82, 05 24, 06 34, 08 7, 09 5. Hub comments: 02 60, 03 816, 04 391, 05 53, 06 79, 08 52, 09 52. March 2026 is the dominant month on every axis.
- Design doc → first implementation PR: REVIEW-DESIGN-DOC-KICKOFF 2026-03-03 → same day (spec validation loop, build reports); scheduling knowledge page 2026-03-23 → implementation 2026-04-20 (four commits, 28 days); DASHBOARD doc 2026-04-20 → Phases 1–3 same day, Phase 5.3 next day; hub-v3 doc 2026-06-11 → PR1 same day, PR4 2026-06-12; codex doc committed inside its implementing PR #84 (2026-08-16); overhead map 2026-08-19 → draft PR #90 the same day (merged 2026-09-01, 13 days). Median: same day.
- Amendment during build: 3 of 5 `.design/` docs were amended while or after being built (hub-v3 once, codex three times within a day, overhead map three times over 15 days); the dashboard doc twice on its build day. Amendments are status annotations ("DELIVERED", "Implementation amendments", "nine-phase progress table") rather than requirement rewrites.

### 8. divergences: intended workflow, observed practice, whitepaper

1. **Red gate.** The whitepaper's 2a ("All tests must fail before any implementation begins") is absent from the intended docs/skills and not observed; tests ship with code.
2. **Fresh-context adversary.** The whitepaper's Phase 3 adversary works in "a fresh context" after the build; observed practice runs architect/independent-audit rounds *during* the build in the same session tree, and the maintainer's GitHub PRs receive no external review. Intended tooling (`--verify thorough` self-review, `qa` skill) also keeps the reviewer inside the build.
3. **Spec review gate on the document.** Intended: design validator + open-question resolution before kickoff. Observed: the document is committed with or after the code (codex, hub-v3 PR1 same day); the gate that is actually honoured (Aug–Sep) sits on a hub pre-flight comment, and the human waived it on #794 ("without further ceremonial approval").
4. **Swarm/phase gates.** Intended centrepiece (`swarm init/launch/gate/checkpoint`) is not used on the repository itself; phasing is done by sequential PRs and a progress table in the design doc.
5. **Kickoff criteria/report loop.** Built in 2026-03 from the REVIEW doc, documented in `kickoff-report.qmd`, but `.kickoff-criteria.json` never appears in hub comments and "kickoff-report" six times; AC traceability is instead carried by commit/PR prose (`REQ-`/`AC-` mentions) and by one executable AC (AC-10).
6. **Records medium.** Intended: commit via `/commit` which adds a `result` comment. Observed: yes, and in the latest era the `result` comment and PR body carry the verification evidence while commit bodies went bare (CLAUDE.md bans attribution trailers; earlier 417 trailer lines).
7. **Knowledge integration.** Intended: every validated design becomes a knowledge page. Observed: 2 of 5.
8. **Rules.** Intended guidance surfaces (`.crosslink/rules/*.md`) were deliberately zeroed on 2026-08-16 (PR #85 "making all 60 bundled rule Markdown files zero bytes"); conduct now rests on skills, hooks and the kickoff prompt.
9. **Roles.** Whitepaper roles (Architect = human, Builder, Tracker = Chainlink, Adversary = Sarcasmotron) map loosely: human = "user/driver/operator"; Builder = kickoff implementer; Tracker = the crosslink hub (not Chainlink); Adversary = a second agent session labelled architect/independent audit/Director. No separate Solution-Owner or domain-reviewer roles exist in the record.
10. **Minimal implementation.** Not practised; PRs are tens of thousands of lines, justified by design sequencing rather than minimality.
11. **Formal hardening/proofs.** Neither intended nor observed; continuous CI hardening (audit, strict clippy, fuzz, proptest) substitutes.

### 9. open questions i could not settle

- Whether `anon-ab6221f8` is the maintainer's interactive identity or a shared machine identity (inferred from timing and content only; it is unsigned, hence "anon").
- Who the "Director" of June 2026 and the "root architect" of September 2026 are in process terms (a distinct model session, a skill, or the human): the comments read as a second agent session, but no `.kickoff-metadata.json` survives to confirm provider/model.
- Whether the maintainer ever ran `crosslink kickoff plan`/`swarm` against this repository outside the hub record (worktree sidecars are git-ignored; the hub has no trace).
- The authoring dates of `patrol-autonomous-maintenance.md` and `REVIEW-DESIGN-DOC-KICKOFF.md` in git (both arrive via snapshot merges); the REVIEW doc's own date (2026-03-03) and the sentinel skeleton commits (2026-04-10) bound them.
- What review, if any, the 193 former-org PRs received outside GitHub (only 1 has a GitHub review; the hub shows "Director-verified gates" only from 2026-06).
- Whether the former-org GitHub issues the maintainer filed (148) were mirrored into the hub systematically or ad hoc; titles like "Fix GH#604" suggest ad hoc wrapping.

