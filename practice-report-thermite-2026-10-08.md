---
title: "VSDD in practice: Thermite (agent report, 2026-10-08)"
tags: ["evidence", "reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---


## Design Specification

### 0. sources read, and what could not be accessed

**Repository files (main)**
- `goal.md` (21,450 B; 3 commits, last 2026-06-17) — in full.
- `thermite-design.md` (27,462 B; 8 commits, last 2026-07-30) — section list only.
- `.design/thermite2-program.md` (17,217 B) and `.design/stage1-forge-tier.md` (25,873 B) — in full; `.design/stage1-forge-tier-kickoff-plan.md` (13,190 B) — headings + first 60 lines; `.design/m0-spikes.md`, `stage2-stratified-cage.md`, `stage3-bv-reconstruction.md`, `stage4-epr-reconstruction.md`, `tone-and-voice.md`, `tone-comment-pass-plan.md` — headers only.
- `.design/forge/check.md` (32,921 B), `.design/forge/spec-review.md` (20,640 B) — header, headings, Summary. `.design/verified/self-verification.md` — Summary.
- `.design/tooling/req-registry.md` (13,648 B), `doc-drift-tripwire.md` (31,544 B), `control-plane.md` (13,889 B) — header, headings, Summary.
- `.design/reqs/registry.toml` (719,352 B) — schema header, one entry, status counts; `.design/reqs/status.md` (241,840 B) — head.
- `.claude/agents/acto-{doc-author,critic,fixer,builder}.md` (26,048 B total) and `.claude/settings.json` — in full.
- `tooling/spec-discipline.py`, `anti-pattern-gate.py`, `doc-drift.py`, `control-plane-check.py`, `req-registry.py` — docstrings and exit paths (not run); `tooling/spec-routes.toml` — header (184 routes).
- `.github/workflows/ci.yml` (step names), `Makefile` (targets), `scripts/audit.sh` (header), `README.md` (headings + "Tests and audits"), `CHANGELOG.md` (head), `THERMITE.skill.md` (head), `.crosslink/hook-config.json` (head), `.gitignore`.
- Git history: 698 commits (first `c9d31826` 2026-06-04); the last 40 bodies; marker counts across all bodies; per-month and design-vs-code tallies; first-commit dates per design doc and governed file; several full bodies (`c7dc32be`, `c9b7420be`, PR #95's two commits).
- Divergence-test and Lean pin inventories (`<crate>/tests/divergence_*.rs`, `lean/Thermite/**/Pin*.lean`), `conformance/` counts, `tests/golden/`.

**Origin branches (not checked out)**: `rfc/process-and-migration` (+7), `rfc/full-words` (+8), `rfc/thermite-3` (+9), `agent/kernel-primitives-only` (+85, 270 files). From the first: `.design/rfcs/0001`–`0004` heads, `0005-rfc-process.md` in full, `tooling/rfc-check.py` docstring and exit paths.

**GitHub**: all 96 merged PRs (number/title/date/author) and the 1 open PR (#134); bodies, commit/review/comment counts for PRs #3, #19, #20, #42, #52, #55, #56, #59, #74, #78, #94, #95, #98, #113; PR comment authorship over all merged PRs; the issue list (34 issues shown, #1–#133); bodies/threads of issues #2, #17, #92, #93, #121, #131.

**Crosslink hub** (bare fetch of `refs/heads/crosslink/*`: `checkpoint`, `knowledge`, `hub`, `meta`, `agents/Yffe`, `agents/rApq`): checkpoint README, `meta/milestones.json`, all 282 `issues/*.json` (aggregated; #243, #193, #40, #7 read); knowledge `index.md`, `kickoff-orchestration-ops.md` (full), `trust-audit-93d3cbc0.md` (head), the knowledge commit log; event-type tallies for the 38 per-agent event logs on `crosslink/hub`. Hub format cross-checked against the hub-v3 design note in the crosslink repo (head only).

**Whitepaper**: the gist (18,001 B), sections II–V.

**Could not access / not read**: the full RFC-1 body and its three companion comments (GH #2; truncated); `RATIONALE.md`, `thermite2-semantics.md`, `docs/*` bodies; the crosslink hooks' source (`.claude/hooks/*.py`, present on the hub branch, gitignored in the tree); the trust-audit page beyond its head; **hub state after 2026-06-18** — the checkpoint's compaction watermark is 2026-06-12 (last checkpoint commit 2026-06-18) and holds issues #1–#282, while PRs cite crosslink #297–#351 and hub event logs lock up to #356 with zero `IssueCreated` events past #282, so the later tracking record is unpublished (**inferred**: it lived in the contributors' local `.crosslink/issues.db`); the kickoff agents' own `crosslink/agents/<id>` refs (only `Yffe` and `rApq` are published); anything about the "[codex]" PRs' process beyond their bodies; the downstream Thermite-Microkernel repository referenced in PR #107's comment.

### 1. document set and authority chain

**The chain.** `goal.md` fixes it: `thermite-design.md` (thesis and pillars) → `.design/<area>/<doc>.md` (per-component contract of REQs + ACs) → implementation (`thermite-*` crates, `forge`) → verification (`cargo test` + conformance corpus + Verus/Lean golden files). "The chain runs design → impl → verification, never the reverse"; when code and doc disagree the doc wins unless the doc is wrong about intent, in which case the fix is a design-doc amendment via `acto-doc-author` (R-SPEC-4), "never let code silently define the contract". Because there is no upstream to mirror, two external truths anchor the critic: the conformance corpus (`conformance/*.th` + hand-certified `*.cert.json`, 63 programs, 12 golden certificates, 20 `cases.json` oracles) and golden lowerings under `tests/golden/` (18 files). Expected values may never be copied from the toolchain's own output (R-CHAR-3).

**Document tiers observed.**
1. Thesis: `thermite-design.md` (13 sections + appendices; 8 commits in two months; amended with a "#21-decision realization note" per hub #193).
2. RFCs: on `main` they are GitHub issue bodies with companion comments (RFC-1 = #2 with metatheory sketch, program plan and Appendix A as comments 2–4; #17 registry RFC; #119, #120 drafts). On `origin/rfc/process-and-migration` they become `.design/rfcs/000N-*.md` with YAML front matter (`rfc`, `title`, `status: draft|accepted|rejected|superseded`, `supersedes`, `introduces: [REQ ids]`, `discussion:`), gated by `tooling/rfc-check.py` (RFC-5, commit `44a70395`, 2026-08-06; renumbering done once in `0639cf1f`…`bc7d8beb`).
3. Program and stage docs: `.design/thermite2-program.md` (umbrella, REQ-1..10/AC-1..15, Q1–Q10 register with decide-by milestones, stage→surface map, baseline-drift section) → `stage1`…`stage4` docs and `m0-spikes.md`, each a `/design` pass with a `.pipeline.json` sidecar (`schema_version`, `design_doc`, `doc_hash`, `stage: "designed"`, empty `plans`/`runs`) → a kickoff plan (`stage1-forge-tier-kickoff-plan.md`) that sequences REQs into committable increments with a per-increment gauntlet.
4. Component contracts: 81 Markdown files under `.design/` (~2.9 MB) across `forge/`, `syntax/`, `spec/`, `lower/`, `verified/`, `basis/`, `boundary/`, `build/`, `strat/`, `tooling/`, `scaffold/`, `skill/`. Template (from `acto-doc-author.md`): HTML header comment (`tier: 3-component`, `status: draft`, `governs:`, `thesis-refs:`, later `audited-sha:`/`audited-content-sha256:`), then `## Summary`, `## Requirements` (REQ-n), `## Acceptance criteria` (AC-n, "mechanically checkable; tied to a conformance corpus entry or golden file where possible"), `## Architecture` (symbol anchors, never line numbers, R-CITE-2b), `## Verification`, `## REQ status` table, `## Open questions`.
5. Normative semantics: `thermite2-semantics.md` (PR #56; module headers point at it instead of restating conventions).
6. Rule book: `goal.md` — the "locked /goal statement", ~45 named rules (R-CITE, R-HONEST, R-CODE, R-TONE, R-SPEC, R-DEFER, R-GIT, R-LOOP, R-INJECT, R-XLATE, R-APG, R-CHAR, plus stage R-rule candidates R-VERDICT-1/R-COV-1/R-GATE-1/R-SIDE-1/R-BV-1) and eight speed disciplines S1–S8.
7. Register rule: `.design/tone-and-voice.md` (R-TONE-1), applied by PRs #9–#14 and the comment-pass plan.
8. Requirement registry: `.design/reqs/registry.toml` (524 `[[requirement]]`, 125 `[[view]]`; typed evidence `file|symbol|test`; six registry-declared statuses) with generated views (`.design/reqs/status.md` and `//!` regions in source) — RFC #17, PR #19, turnover PRs #21–#41.
9. Route table: `tooling/spec-routes.toml` (184 routes: `crate_pattern` → `design` [+ `reference`]) — "the authoritative module map".
10. Generated agent-facing spec: `THERMITE.skill.md` under a 6,000-token CI budget ("Do not edit it by hand; refresh it with `forge skill --write`").
11. `CHANGELOG.md` organized by stage gates (G1 2026-06-18, G2 2026-06-22, G3 2026-07-29), curated at gate time (PR #79), with `--no-changelog` on every agent issue close.
12. Hub knowledge pages (9): architecture v0.1, mirrors of the program/stage docs, the trust audit, and `kickoff-orchestration-ops.md` (process lore: worktrees, merge-on-green, re-pin rules, registry union conflicts).

**How documents are amended.** Dated in-body "Amendment (YYYY-MM-DD …)" paragraphs appended to the governing doc, e.g. `stage1-forge-tier.md`'s 2026-06-15 amendment "driven by a `crosslink kickoff plan` gap analysis + a git-history sweep … REQ/AC IDs and structure unchanged", and `forge/check.md`'s "Amendment 2026-06-12 (doc-freshness re-audit, #262)" and "#92 Amendment" (PR #95 body: "R-HONEST-4: recorded as an Amendment in .design/forge/check.md"). Dated amendments cluster: 12 on 2026-06-12 (the #262 freshness re-audit), 10 on 2026-07-29 (re-pins after the prose pass). `forge/check.md` has been touched by 42 commits, the registry by 60, the route table by 58, `goal.md` by 3. Component docs never leave `status: draft`; stage banners carry status in prose ("RE-PASS COMPLETE · kickoff-ready", "PROVISIONAL — re-run the design pass before kickoff"), and `stage4-epr-reconstruction.md` is the only one with `status: shipped`.

**How decisions are recorded.** (a) `## Open Questions` entries with Q-ids and "(resolved) **Decision:** …" (Q-ORACLE, Q-BURN, Q-KBSIGNAL, Q-DECWF, Q-NLSAT in stage 1; Q-TRACK in the umbrella); (b) the Q1–Q10 register table with decide-by milestones and the rule "a merge that contradicts a default must update the register in the issue thread first" (REQ-6); (c) hub `--kind decision` comments (6 of 572; e.g. #40, #7); (d) RFC thread comments on GH #2 (12), including the maintainer's baseline-drift note and "freezing any additions until we both approve this RFC"; (e) `goal.md` R-rule candidates promoted when a stage lands (PR #56).

**What keeps the documents honest.** `doc-drift.py` (content digest over each routed doc's governed files, CI + `make doc-drift`; exit 0/1/3); `control-plane-check.py` (the hook wiring itself, after #93); `req-registry.py --check` (stale generated regions fail CI; MISSING-/BAD-/CLOSED-BLOCKER findings); `req-status.py` (legacy comment-table lint); `spec-discipline.py` (a routed edit requires the doc to exist and to have been read this session); `rfc-check.py` on the RFC branch; the `.pipeline.json` `doc_hash`. Note the control-plane doc's own finding: the two agent-facing gates were dormant from `5581b65f` (2026-06-21) to PR #94 (2026-07-29) "while README, goal.md and all four acto-*.md kept asserting they fire" — "the design layer governed everything except the file that decides whether the governance runs".

### 2. per phase

Phase names below are the whitepaper's; Thermite does not use them. Its own vocabulary is the ACToR loop (doc-author → builder → critic → fixer), increments, gates, pins and the gauntlet.

### 1a Behavioral Specification
- **Artifact**: a `.design/<area>/<doc>.md` with REQ-n / AC-n (template above); for program work, RFC → umbrella → stage doc → kickoff plan. Edge cases are not a named catalog; they appear as ACs and in the critic's Step-3 corner-case list (`acto-critic.md`). Non-functional requirements present: determinism (R-CODE-5), the 6,000-token skill budget, kernel budgets (Q4).
- **Producer**: `acto-doc-author` (no `Edit`; `Write` to `.design/*.md` only) in the June bootstrap — e.g. `7fa9ea0f3` "spec: thermite-spec design doc + combinator oracle (#2)", `80cca29d4` "lower: design docs + verus-verified L3 golden files (#4)" (both 2026-06-04), hub #193 plan comment ("authored `.design/forge/goal-repl.md` … all new verbs NOT-STARTED … No toolchain code edits (R-DOC-1)"); crosslink `/design` passes for stage docs (umbrella REQ-10: "Each stage gets its own `/design` pass before its first implementation issue opens"); the maintainer directly in July (`.design/tooling/control-plane.md` in PR #94; `.design/build/*` on `agent/kernel-primitives-only`).
- **Gate**: `spec-discipline.py` R-XLATE-2/3 — an edit to a routed file blocks (exit 2) until a route exists and its design doc exists; the block message embeds the `acto-doc-author` dispatch prompt. AC-15 of the umbrella: a stage design doc must exist before the first implementation issue is worked.
- **Record**: design-only commits (157 in June); `.design/*.pipeline.json` at `stage: "designed"`; registry entries created with the doc (`contributors` lists the doc).
- **Divergence to note**: `acto-doc-author`'s R-DOC-1 "the doc adapts to the code, never the reverse" governs backfill of already-shipped modules; the spec-first direction is preserved for new behavior by R-SPEC-4.

### 1b Verification Architecture
- **Artifact**: each contract's `## Verification` section and AC wording ("tied to a conformance corpus entry or golden file"); `goal.md` "The verification model" ((A) crates verified by `cargo test` against ACs + goldens, (B) `forge` verified by the corpus certificate oracle); route `reference` fields naming the oracle a file is checked against; `.design/verified/*` (11 docs: `contract-tv`, `exec-tv`, `exec-stmt-tv`, `loop-tv`, `proof-backends`, `rust-lean-correspondence`, `strat-rust-lean-correspondence`, `exporter-surface-correspondence`, `self-verification`, `z3-demotion`, `thermite-semantics`) which are the provable-properties catalogue and tool-selection record.
- **Purity boundary**: not drawn as a Phase-1b map; it is the product's own `fx` effect row, `pure`, R-CODE-5 determinism, and the seccomp runtime sandbox. `self-verification.md` names a "soundness-critical pure core (Tier 1)" ported into Verus so "the code that runs IS the code that was proved".
- **Producer**: the doc-author/the `/design` pass; oracle fixtures hand-derived from the design (`conformance/sum.cert.json` "survivor string byte-exact" to Appendix A, `c7dc32be`).
- **Gate/record**: R-CHAR-3 (tautological tests are themselves divergences); the critic's Step 5 "verify the test actually fails".
- **Examples**: `8f4b4cb16` "lower: amend REQ-8/AC-1/AC-2 — verify emitted output, don't byte-match goldens" (the verification strategy amended on first contact, same day as the doc); stage-1 AC-1..AC-14 each name the test shape; PR #20 "`oracle_subset` byte-identical on all 7 `conformance/*.cert.json` (AC-4)".

### 1c Spec Review Gate
- **Observed forms**: (a) the critic auditing the spec itself: hub blockers #205–#216 (2026-06-10/11), twelve "Divergence: proof-backends.md … contradicts / misstates / ill-typed / unbound …", closed by doc amendments; (b) human RFC review in the GH #2 thread (companion documents, the maintainer's baseline-drift comment, the agreed freeze, and the decision to go PR-based); #17 → PR #19 within a day; (c) kickoff-time gap analysis (stage-1 amendment 2026-06-15; stage-2 "G1 re-pass", PR #63, resolving Q-KIT/Q-TV2 against the M0 spike results, PR #13); stage 3 marked PROVISIONAL until its re-pass (PR #83); (d) the external trust audit at `93d3cbc0` (findings F1–F10 with a reconciliation table; F10 motivates stage-1 REQ-2, F4 the Lean CI job).
- **Gate**: no mechanical gate and no status flip (`status: draft` throughout). Human sign-off is in-thread.
- **Not observed**: a routine fresh-context adversarial pass over every spec before tests; a multi-domain review. **Inferred**: the critic-on-spec pins are the closest practice, and they occur after implementation starts.

### 2a Test Suite Generation (red gate)
- **Artifact**: failing "pin" tests `<crate>/tests/divergence_<n>_<slug>.rs` (forge 39, lower 19, syntax 6, spec 5, tv 2, skill 1) with a doc header stating divergence class, authority, "Expected (authority, not forge's own output, R-CHAR-3)" and the commit it fails against (`forge/tests/divergence_249_axiom_mask.rs`); `#[ignore = "divergence: …; tracking #N"]` while open; "teeth" suites that inject an infidelity and assert the oracle catches it (`scripts/audit.sh` check 3; hub #143 "teeth-test (R-CHAR-3)"); 33 `Pin*.lean` negative pins; the SplitMix64 `--generated` corpus with a rotating-seed cron (`.github/workflows/generated-tv.yml`, umbrella AC-4).
- **Producer**: `acto-critic` (`Write` only). Red gate enforced by its Step 5: "must FAIL … If it passes, the candidate is not a divergence — drop it".
- **Examples**: bootstrap on 2026-06-04: `a419e0081` "pin 3 parser divergences as failing tests (#28 #29 #30)" → `5bb910efd` fix → `0b70c326d` re-pin → `f0ceb120e` fix; 2026-06-16 pairs `test(forge): pin …` / `fix(forge): …` for #298–#302 (PR #42).
- **Divergence**: for new components the builder writes "tests + production in the SAME commit" (goal.md loop step 3; `acto-builder.md` Step 3), so red-before-green holds for divergences, not for feature increments.

### 2b Minimal Implementation
- **Role**: `acto-fixer` — "exactly ONE pinned divergence … smallest edit … single file … no renames, no restructuring, no 'while I'm here' cleanup", revert if the gauntlet fails; `acto-builder` — pre-declared manifest "≤~10 files", cannot widen scope; R-DEFER-1 (a new `pub` API needs a non-test consumer in the same commit); the anti-pattern gate (exit 2) blocks `todo!/unimplemented!/unreachable!`, `.unwrap()/.expect()/panic!` outside `#[cfg(test)]`, module-root `#![allow]`, `Arc<Mutex>`/`Rc<RefCell>`.
- **Record**: the goal.md commit template (DESIGN SOURCES / REQ STATUS / VERIFICATION with integer counts) — 188 and 146 of 698 bodies carry the first two sections; a `--kind result` comment, then close with `--no-changelog`.
- **Examples**: #298–#302 one-fix-per-blocker commits; PR #95 "one defect, three surfaces … Fix: seed the referrers with everything woven"; hub #243 result "Four divergences fixed on the one exporter seam".

### 2c Refactor
- **Not observed** as a phase. The fixer forbids adjacent cleanup; the builder has no refactor step. The nearest practices are register-only passes (PRs #9–#14; `tone-comment-pass-plan.md`: "a register change only — no code, no identifiers, no semantics move"), the registry turnover series (PRs #21–#41, mostly "[codex]"), and re-pin chores (#75, #82, #85). The "fix the cause's whole class" rule (`a2662b0e8`) is the only structural-cleanup instruction.

### 3 Adversarial Refinement
- **Role**: `acto-critic` is the adversary. Dispatched "after every substantive builder/fixer" (S4; skipped for cite/fixture/doc refreshes, S7), looped "until clean". It may not fix, approve, or give prose verdicts; the only verdicts are "GENERATOR MUST FIX" / "NO DIVERGENCE FOUND"; "There is no 'ACCEPTABLE DRIFT' verdict (R-DEFER-3)". Divergence classes: wrong certificate, wrong lowering, design-REQ miss, proof cheat (R-DEFER-9).
- **Finding form**: a runnable failing test + `crosslink quick "Divergence: <crate>::<fn> diverges from <authority>" -p high -l blocker` + a `--kind observation` comment naming the test path; the test is committed ("the tests ARE the audit artifact"). In the hub snapshot 151 of 282 issues carry `blocker`, 92 titles begin "Divergence:", 211 are priority high.
- **Examples**: PR #42 "The acto-critic loop ran to convergence, finding + fixing 5 real divergences, each pinned by a failing test … Final pass found no remaining divergence"; `c9b7420be` "critic: doc-drift re-audit — pin #261 … after #259/#260 fixes" with an "Audited-not-divergent" list; hub #243 observation "Pinned: lean/Thermite/PinExportUndefinedCallee.lean (kernel-checked)".
- **Vocabulary**: "gauntlet" in Thermite is the mechanical per-crate check set (`cargo test`/`clippy -D warnings`/`fmt --check` + conformance; `make gauntlet`), not the adversarial pass.
- **Context reset / model diversity**: each critic run is a separate subagent dispatch with no `Edit` (**inferred** fresh context); same model family throughout (`Co-Authored-By: Claude Opus 4.8` on 2026-06-04, `Claude Fable 5` by 2026-06-11). Frontmatter `model: fable` on critic and doc-author since `7bc940487` (2026-06-09) while the bodies still say "Opus — always". The "[codex]" PRs (#21, #22, #26, #28, #37, #38, #41, #46, #53, #54; branch `codex/g4-epr-reconstruction`, PR #98) are a second harness used as a builder, not as the adversary (**inferred**).
- **Product-level adversary**: the toolchain itself embeds the roast as gates on user programs (vacuity battery, mutation scoring with `MUTANT_CAP` 64 and a kill-ratio floor, covenant falsification, `Pin*.lean`).

### 4 Feedback Integration
- **Spec-level** → design-doc amendment by doc-author (R-SPEC-4) and R-HONEST-4 corrections recorded as Amendments (PR #95; `check.md` 2026-06-12); thesis touched only via "realization notes" (hub #193).
- **Test-level** → critic re-pins after fixer commits (`c9b7420be`; "Followed by an acto-critic re-audit" in `acto-fixer.md`).
- **Implementation-level** → one fixer per blocker, serially.
- **External** → hub `decision` comments (#40: "External reviewer (human + agent) caught a real gap … Verdict: partly true, worth fixing"), the trust-audit reconciliation table (F1 ADDRESSED; F2/F3/F4 OPEN → stage-1 increments 0/1), real-code usage (#92 "Found while lowering real code into Thermite"), downstream audits posted as PR comments by the maintainer (PR #107 "Downstream completion audit passed"), process incidents (#93 → PR #94 → #121).
- **Registers**: the "known-divergence ledger" (#148; cleared per umbrella AC-3), the Q-register (REQ-6/AC-11, still unchecked).

### 5 Formal Hardening
- In Thermite the product is the verifier, so hardening is continuous and partly indistinguishable from the product: the Lean spine (76 modules) with `#print axioms` gating in CI (`lean-probe`, `scripts/lean-axiom-probe.sh`; allowlist `{propext, Classical.choice, Quot.sound}`), the `make audit` deep audit (checks 1–6 plus the G2 block, `scripts/audit.sh`), G3/G4 gate scripts, the Rust↔Lean correspondence drift tripwire (audit check 4, `.design/verified/rust-lean-correspondence.md`), `self-verification.md` (Tier-1 pure functions proven with Verus and delegated to), and mutation testing inside `forge`.
- **Not observed**: AFL/libFuzzer-style fuzzing (generator + rotating-seed cron instead), Semgrep/Wycheproof, a separate purity-boundary audit step.
- **Record**: CHANGELOG "Trust boundary" sections per gate; registry evidence rows; PR bodies' "axiom-clean" claims (PR #3, #78).

### 6 Convergence
- **Unit level**: `goal.md` "Stopping condition" + a mechanical check (routed-count == `## REQ status`-count, gauntlet green, corpus passing); R-LOOP-2 "never declare the goal complete until the mechanical check says so"; the critic's "NO DIVERGENCE FOUND".
- **Program level**: stage gates G1–G4 with checklists (stage-1 AC-14; umbrella REQ-5), gate artifacts (PR #55), headline flips only at gate time (R-GATE-1; PR #59), pinned "Gate reached" comments on GH #2 (G1 2026-06-18, G2 2026-06-22), gate PRs #97 (G3) and #98 (G4), and the gate-organized CHANGELOG.
- **Registry**: 518 shipped / 2 partial / 3 not_started / 1 retired of 524.
- **Divergence**: convergence is mechanical, not "hallucination-based"; the spec dimension has no separate signal (docs stay `draft`).

### 3. roles and dispatch shape

- **Agents** (`.claude/agents/`): `acto-doc-author` (tools Read/Write/Bash/Grep/Glob; `model: fable`), `acto-critic` (same tools; `model: fable`), `acto-fixer` (Read/Edit/Write/Bash/Grep/Glob; `model: opus`), `acto-builder` (same as fixer; `model: opus`). All four bodies demand "Opus — always"; critic and doc-author lost `Edit` "by harness, not convention" per `c7dc32be`, though #93 observes the read-only roles are "by convention, not capability" since they keep `Write` and `Bash`. Each carries "Operational discipline": stay on the dispatched branch, clean scratch, `--no-changelog`, R-TONE-1, fix the cause's whole class. Report caps: doc-author 400 words, fixer 500, critic 700, builder 800.
- **Orchestrator**: the interactive Claude Code session running under `/goal $(cat goal.md)`; hub agent `Yffe` created all 282 checkpoint issues and holds 286 lock claims (2026-06-04→06-12) — **inferred** to be the maintainer's orchestrator session; `rApq` (34 lock claims, 06-13→06-22) authored the knowledge pages — **inferred** to be the second contributor's session.
- **Dispatch rules** (`goal.md` S1–S8): batch by component not per function; parallel-dispatch independent units "in ONE message" (fixers serialize per blocker); symbol anchors not line numbers; critic only after substantive builds; builder manifests ≤~10 files; "Do not ask which — the dependency DAG is the answer"; aggressive won't-fix on noise.
- **Kickoff shape** (from 2026-06-12): crosslink `kickoff` agents in `.worktrees/<agent-id>/` tmux sessions, 38 agent ids on the hub branch, branches `feature/5i5F-<id>-<slug>` (PRs #3, #20, #29, #42–#49, #55–#57, #64–#78, #81–#91); agents "cannot `git push`"; "the orchestrator pushes, watches CI, and merges each branch" (merge-on-green with a `gh pr checks` poll); stage-1 increments 2a–2f declared parallelizable once increment 1 lands (kickoff plan).
- **Pinning and closing a critic finding**: failing test committed + `-l blocker` issue + observation comment; one fixer per blocker; the blocker closes only when the fix lands and the test goes green with `#[ignore]` removed (R-DEFER-3); then a critic re-audit. "Resolved in <sha> (not closed per dispatch)" appears when the orchestrator retains closing (hub #243).
- **Ceilings**: no explicit round cap; S8 and the won't-fix rule are the only brake; the kickoff plan fixes increment order and a per-increment gauntlet.

### 4. review conduct

- **Rounds and stop rules**: builder → critic → (fixer → critic)* until "NO DIVERGENCE FOUND"; critic skipped for cite/fixture/doc refreshes and mechanical reverts; "Honest underclaim beats unverified overclaim" ("NO DIVERGENCE FOUND" with the audited-area list is a valid report).
- **Filing/labelling/routing**: hub issues (`crosslink quick … -l blocker`; labels in the snapshot: blocker 151, feature 60, docs 29, bug 11, epic 6; comment kinds plan 199, result 245, observation 106, note 11, decision 6, handoff 3). GitHub holds RFCs, gates, umbrellas and human-reported defects (Q-TRACK split; GH labels sparse: RFC, blocker, bug, documentation).
- **Closing**: `--kind result` first, then close with `--no-changelog`.
- **"Gauntlet"**: the per-crate mechanical check list (goal.md; `make gauntlet`). **"Pin"** has three senses: a critic's failing test ("critic pin", "re-pin them"), a Lean negative lemma (`Pin*.lean`), and a doc-drift content digest ("re-pin", "repin tax", PRs #82/#85).
- **Cold vs warm**: the critic is a separate no-`Edit` dispatch that must re-read the authority (Steps 1–2) but shares the same rule book and model family (warm by design, cold by context — **inferred**). Genuinely cold reviews on record: the external trust audit at `93d3cbc0`; the external reviewer behind hub #40; issue #93 (filed by `magnificentlycursed`, verified against HEAD `8c022a62`); issue #121 (`maxinelevesque`). GitHub PR review is absent: 0 reviews on 96 merged PRs; 23 PR comments in total, 22 by the maintainer (e.g. downstream audit results), 1 by the second contributor. Authorship split: 50 PRs by `maxine-at-forecast`, 46 by `dollspace-gay`; the maintainer merges (**inferred** from ownership; branch protection not visible).

### 5. records discipline

- **Commit messages**: June bootstrap subjects `<crate>: <area> — <summary> (closes #N)` / `<crate>: critic — pin …` / `<crate>: #N fix — …`; bodies per the goal.md template (DESIGN SOURCES THIS ITERATION / REQ STATUS / VERIFICATION / `Refs #N`; `Co-Authored-By: Claude …` on 577 of 698). PR-era squash subjects `<Title> (#PR)` with the PR body as message; the maintainer's July commits use Summary / Root cause / Behavioral note / Validation / `Closes #N` (PR #113) or "Validated with the full workspace test suite, strict Clippy, formatter, control-plane, requirements, and doc-drift checks" (`b8dc3947`).
- **PR bodies**: "## Delivered / ## Adversarial verification / ## Gauntlet (local)" with "Tracking: crosslink #NNN" and a "Generated with Claude Code" footer (kickoff PRs); "## Summary / ## Why / ## Checks / Refs #17" (maintainer tooling PRs); "## What changed / ## Why / ## CI runner fix" (codex PR #98). Honesty markers recur: "the Lean-spine tests were skipped locally … the CI job is the real gate" (PR #42), "README headline flip … explicitly NOT done here" (PR #55).
- **Issues**: hub titles "Divergence: <symbol> <claim> (§/REQ cite)"; GH defect reports with a Repro and the structural claim ("No file declaring more than one ADT has ever certified", #92); GH blockers state "Specified in `.design/...`. Nothing is implemented today" with symbol anchors (#131).
- **Hub vs GitHub**: hub = increments, pins, day-to-day; GitHub = RFCs (#2, #17, #119, #120), gate announcements, umbrellas, human-found defects; knowledge pages = process lore and mirrors of program docs.
- **Hub mechanics**: v3 per-agent refs with signed append-only events (`LockClaimed`/`LockReleased`/`CommentAdded`/`IssueCreated`/`LabelAdded`/`StatusChanged`), a machine-written checkpoint ("do not edit by hand"), `meta/allowed_signers`. The published checkpoint stops at 2026-06-18 (282 issues, 5 milestones v0.1–v0.5), while later work cites crosslink #297–#356 — the public hub record is partial.
- **CHANGELOG**: curated per gate (PR #79), never by agents.

### 6. gates and tooling

| Gate | Checks | When | Exit contract |
|---|---|---|---|
| `tooling/spec-discipline.py` | routed `thermite-*/src`, `forge/src` edit requires `goal.md` + the route's design doc (+ ≥1 declared reference) read this session (`.crosslink/.spec-reads.json`); no route or missing doc blocks | PreToolUse Write/Edit; PostToolUse Read recorder (`.claude/settings.json`) | exit 2 blocks with a corrective message; 0 otherwise |
| `tooling/anti-pattern-gate.py` | stubs/unwrap/expect/panic/root `#![allow]`/`Arc<Mutex>`/`Rc<RefCell>` outside `#[cfg(test)]` | PreToolUse Write/Edit | exit 2 "BLOCKED — N forbidden"; override = per-item `#[allow(..., reason)]` + observation comment |
| `tooling/doc-drift.py` | every routed doc's `audited-content-sha256` (or legacy `audited-sha`) matches its governed files | CI `checks` job, `make doc-drift` | 0 clean / 1 drift or missing pin / 3 inconclusive; not part of `make audit` (decision 5) |
| `tooling/control-plane-check.py` | required hooks wired in `settings.json` and scripts present | CI, `make control-plane` | 0 / 1 / 3 |
| `tooling/req-status.py` | legacy `//!` REQ tables consistent | CI | lint fail |
| `tooling/req-registry.py --check` / `tooling/reqs check` | registry schema, evidence rules, blocker references, generated regions current | CI, `make req-registry` | findings (MISSING-BLOCKER, BAD-BLOCKER, CLOSED-BLOCKER…); 3 when `tomllib` unavailable |
| `tooling/rfc-check.py` (RFC branch) | front matter, status set, unique numbers (drafts may collide), `introduces` REQs exist | CI step on the branch | 0 / 1 |
| `tooling/tests/*` | oracle fixtures for the gates (R-CHAR-3) | CI | unittest |
| `cargo run -p thermite-skill -- --check-budget` | generated skill ≤ 6,000 tokens | CI | fail |
| bv build-flag gate | `@bv` rejected without shadow-flag plumbing (R-BV-1) | CI | fail |
| `cargo fmt --check`, `clippy -D warnings`, `nextest` 4 shards, doctests | the gauntlet | CI, `make gauntlet` | 0 failures |
| `lean-probe`, `lean-spine-forge` (4 shards) | `lake build` + `#print axioms` allowlist; live Lean-backed forge tests | CI | fail; "0 ignored" is the local signal |
| `scripts/audit.sh` (`make audit`) | checks 1–6 + G2 block (axiom probe, full-corpus TV, teeth battery, correspondence drift, third-party re-check, residual-trust statement) | manual / gate | SKIP-with-consequence reporting |
| `scripts/g3-gate.sh`, `scripts/g4-gate.sh` | stage gate bundles (G4 under a 6 GiB ceiling, pinned CaDiCaL/drat-trim) | gate time | fail |
| `.github/workflows/generated-tv.yml` | rotating-seed generated corpus | cron | fail on divergence |
| crosslink hooks (`work-check`, `prompt-guard`, `pre-web-check`, `post-edit-check`, `heartbeat`, `session-start`) | active-issue gate, blocked git commands, lint/test commands (`.crosslink/hook-config.json`) | per tool call | crosslink-deployed; gitignored |

### 7. cadence and proportions

- 698 commits: 677 in June 2026, 20 in July, 1 in August; 424 landed directly on `main` before the first PR (#3, merged 2026-06-12, after the GH #2 thread agreed "move this to PR based"); 274 after; 67 merge commits; 96 PRs merged 2026-06-12→08-01. The trust audit notes "~385 commits / 8 days" at `93d3cbc0`.
- June mix by files touched: design-only 157, design+code 180, code-only 227, other 113 — roughly half of all June commits touch `.design/`.
- Design doc → first implementation: `forge/check.md` and `syntax/parser.md` same day as their code (2026-06-04); `goal-repl.md` 1 day (06-09→06-10); `doc-drift-tripwire.md` 0 days; stage 1 3 days (doc `c390aced3` 06-12 → PR #20 06-15); stage 2 8 days (06-12 → PR #64 06-20, after the 06-19 re-pass); stage 3 10 days (06-12 → PR #81 06-22); stage 4 and `control-plane.md` landed in the same PR/commit as their code (PR #98 07-30; `904f4bc6` 07-28).
- Amendment during build: `forge/check.md` 42 commits, registry 60, route table 58, `goal.md` 3, `thermite-design.md` 8; dated Amendment paragraphs: 06-04 (1), 06-10 (2), 06-11 (2), 06-12 (12), 06-15 (1), 06-18 (2), 06-21 (1), 07-28 (1), 07-29 (10).
- Hub: 282 issues and 572 comments in nine days (06-04→06-12), 276 closed; stage 1 ran as 11 PRs in 5 days (#20→#57), stage 2 as 11 in 3 days (#64→#78), stage 3 as 8 in 4 days (#81→#91).

### 8. divergences from the whitepaper, and practices it does not mention

**Divergences**
1. *Red before green* holds for divergences (critic pins), not for feature increments: builders ship tests and production in one commit (goal.md step 3; `acto-builder.md` Step 3) versus "Do NOT write implementation code until I confirm all tests fail".
2. *Spec Review Gate* is not a gate: no status flip, no fresh-context adversary over every spec; review is RFC-thread conversation, kickoff gap analyses, and critic pins filed against docs after implementation starts (#205–#216).
3. *Phase 2c refactor* is absent and the fixer forbids it.
4. *The Adversary* writes failing tests and may not propose fixes or prose verdicts (versus "every piece of feedback is a concrete flaw … and a proposed fix"); it is the same model family as the builder (no cross-family diversity); no "Sarcasmotron" persona.
5. *Convergence* is mechanical (registry count, gauntlet, gate checklists, R-GATE-1) rather than hallucination-based, and the spec dimension has no separate signal.
6. *Doc adapts to code* (R-DOC-1) for backfill, in tension with Spec Supremacy; reconciled by R-SPEC-4 for contract changes and "SHIPPED needs quoted evidence".
7. *Tracking*: crosslink issues + a TOML REQ registry (evidence kinds file/symbol/test) instead of per-property Chainlink sub-issues; provable properties are not tracked separately from tests.
8. *Phase 5 tooling*: Verus/Z3, Lean (+ lean-smt/cvc5), CaDiCaL/drat-trim; mutation testing built into the product; no AFL/libFuzzer, Semgrep or Wycheproof; hardening is continuous (per-increment `Pin*.lean`, CI axiom probe), not a late phase.
9. *Status vocabulary*: goal.md R-DEFER-2 demands binary SHIPPED/NOT-STARTED, but the registry (RFC #17, PR #19) declares six statuses and two `partial` rows exist (PR #20 registered REQ-S1-4 as partial).
10. *Human checkpoint* after Phase 2 is replaced by the maintainer's downstream audits and the designed-but-external `forge review` spec-intent slot (`spec-review.md`: "forge review does not call an LLM").

**Practices the whitepaper does not mention**
- Route table + read-before-edit hook (R-XLATE); anti-pattern gate; doc-drift content pins; the control-plane gate ("the gate that guards the gates") and the dormant-gate incident (#93/#121) that motivated the rule "anything load-bearing for a trust claim must be CI-enforced and harness-agnostic; authoring-time tooling may never be cited as the reason a property holds".
- R-CHAR-3 oracle provenance; R-DEFER-1 consumer rule; R-DEFER-3 no-skip closure; R-DEFER-9 no proof cheats; R-GATE-1 headline-at-gate-time; R-HONEST-4 corrections recorded as Amendments; R-TONE-1 register rule; S1–S8 speed disciplines; the Q-register with decide-by milestones.
- The commit template with DESIGN SOURCES / REQ STATUS / VERIFICATION and integer counts; `--no-changelog` with a gate-time curated CHANGELOG; knowledge pages; the hub/GitHub split; kickoff worktrees with merge-on-green and push-less agents; RFCs-as-files with a front-matter gate and "always merge, never close".
- "The skill is the spec": a generated, token-budgeted agent-facing reference (`THERMITE.skill.md`) kept current by CI.
- Stage gates (G1–G4) as the unit of public claim, with gate artifacts and pinned gate comments.

### 9. open questions the record does not settle

1. Whether critic dispatches ran in fresh contexts and how often S4/S7 skipped them — no dispatch logs are public; hub events show only locks and comments.
2. Which model actually ran the critic and doc-author after `7bc940487` (frontmatter `fable` versus the bodies' "Opus — always").
3. The hub record after 2026-06-18 (crosslink #283–#356): whether it exists anywhere public, and whether the kickoff agents' own refs were ever published.
4. Whether the "[codex]" builder output received a critic pass (PR bodies list checks only).
5. The umbrella's unchecked AC-1/AC-6/AC-9/AC-10/AC-11 while G2–G4 are declared complete — deliberate per the knowledge page's "leave process-historical ACs honestly unchecked", or drift in the umbrella itself.
6. The de-wiring of the gates: the PR #94 comment says it was disabled intentionally for local preference; issue #121 says "I did not do that deliberately". The record holds both.
7. Whether branch protection exists and who merges; R-GIT-1 "the human performs all pushes" versus the kickoff pattern where the orchestrator session pushes.
8. Whether the `forge review` spec-intent verdict slot has ever been filled by an external reviewer (the Phase-2 "spirit of the spec" check).
9. Time-per-phase: only commit timestamps and hub lock events exist; no round counts per increment beyond PR prose (e.g. "5 real divergences").

