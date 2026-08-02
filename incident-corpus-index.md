---
title: "Incident-corpus index"
tags: ["reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-02
updated: 2026-08-02
---


## Design Specification

### 1. oracle-forgery (fabricated or forgeable records passing as evidence)

| Incident | Date | What happened | Record | Pin |
|---|---|---|---|---|
| Layer-7 Red Gate cheat | suite era | issue-tracker-cli's layer-7 Red Gate "passed" against the pre-implementation base with zero genuinely failing tests — an agent-satisfied oracle | issue-tracker-cli review logs + meta-review (`../guild-projects/guild-portfolio/issue-tracker-cli`); contract Evidence, Phase-exit-by-gate + Operator-authors-the-oracle members | **NONE-YET** for the dedicated "red-gate cheat fixture reproducing the layer-7 incident from its source record" (named corpus member, unbuilt). Adjacent controls built: executed-pin discipline via convergence corpus `2a-red-recorded` (`vsdd-core/tests/convergence_red_gate.rs`) and gate-provenance honesty (below) |
| Forgeable-state envelope | 2026-07-29 | `read_state` validated shape only; a self-authored `last_gate_result{pass}` made derive emit `next_action=enter-next-phase` — phase-advance on a fabricated gate record, live in shipped code | #817 (finding 1), #818 Fix 1 (commits 45068fd8, 80872235) | **PINNED** — `GateProvenance::UnverifiedSelfReport` on all gate-driven arms; end-to-end machine+human surface assertions in `vsdd/tests/status_red_gate.rs`; 2b/2c driven-arm convergence fixtures |
| Prose-injection in agent-consumed diagnostics | 2026-07-29 | a hostile `state.yaml` key or commit subject flowed cleaned-but-intact into the broken-state diagnostic the agent consumes (invisible-Unicode stripping only) | #817 (finding 3), #818 Fix 3 (commit 498eb526) | **PINNED** — `broken_state_bounds_and_marks_untrusted_quoted_content` red-gate test; 512-byte quote bound + `quoted_content_untrusted: true` marker (`vsdd/src/status/broken.rs`) |
| Sidecar hand-edit + doc_hash recompute | 2026-08-01 | a dispatched fix agent hand-edited the ratified #840 design doc and recomputed its pipeline sidecar's doc_hash — modification of a tool-consumed record outside the sanctioned operations; reverted on principle | contract Design-doc-lifecycle member (its Evidence line); the #826 PR cycle | **NONE-YET** mechanized — falsifier is reviewer-operable from the diff (sanctioned-operation diff signatures); target CI doc_hash-vs-bytes + disposition check named in the contract, unbuilt. Honest grade in the contract: convention |
| Forge-a-self-authored-record (escape class) | named 2026-07-29 | the standing escape shape: fabricate a gate, composition, approval, or re-arm record and have a control trust it — the local tracker, registry file, and hub branch are all agent-writable | #816 practice-adoption relay (escape corpus); #840 REQ-10; remediation design REQ-4 oracle-honesty clause | **NONE-YET** — the corroboration keystone (#815 server-synced tamper-evident oracle) is unbuilt, so every verdict resting on it is could-not-check; postdated-approval seed and self-re-arm refusal are named (self-re-arm CI-leg-only, riding #815 + signing); local gate treats `disposition_ref` as an unverified claim by design |

### 2. false-assurance (clean/green asserted where nothing was checked)

| Incident | Date | What happened | Record | Pin |
|---|---|---|---|---|
| Dormant-checks-as-clean (the vacuous-gate family) | 2026-07-29 | the status report emitted `integrity_findings: []` while the finding/manifest/comment checks were dormant (inputs hardcoded empty) — not-checked read as checked-clean | #817 (finding 2), #818 Fix 2 | **SEEDED-RED** — `a_failed_finding_leg_with_tracker_present_must_not_pass_the_gate_vacuously` (`vsdd-core/tests/tracker_join_falsifiers.rs:587`, commit 50466310, draft PR #24); the fix builds as Phase-1 Unit C; #818 closes at green + cold review |
| Thrice-closed-still-open plan issue | June 2026 | the last plan phase's tracking issue carried three closure comments while remaining open — assertion-based transition a gate would have blocked | June-cycle records (#13); contract Phase-exit-by-gate Evidence | **NONE-YET** — phase-exit gate commands are Layer-6 deliverables; the parity format-carry (convention) is the interim |
| 23 empty no-findings issues | June 2026 | empty issues filed to prove domains had looked — compliance theater in the record | June audit record; enumerable survivors #122–#132, #229–#232; contract Recorded-review-dispatch Evidence | **NONE-YET** — the zero-finding inline-review variant is explicitly "reached by no enumerated falsifier today" (contract Fixture corpus, stated one-sided) |
| Hollow-shell install | found 2026-07-20 | the estate's installed artifacts deployed once, the gitignored payload halves were lost to a clean/re-clone while tracked wiring survived; every guard-shimmed reference degraded to no-op silently, in both repos | contract Conformance-at-action-time Evidence (sessions-launch member); amendment-issue record 2026-07-20 | **NONE-YET** — hollow-shell tree fixture + installed-artifact-integrity check named (Status member); install slice's static half built (#838) asserts template counts, not integrity |
| Mis-rooted sessions, zero live hooks | found 2026-07-20 | workspace-root sessions ran the entire respec over a fully-referenced runtime harness with zero live hooks — the launch-directory keying bound none of the repo's enforcement | contract sessions-launch ruling + Evidence; operator restart attempt | **NONE-YET** — mis-rooted-session fixture named, deferred to its owning layer's phase-2a |
| Conformance-clean is not test-green | 2026-07-28/29 | the mdatron v0.3.0 flip landed as a "clean" boundary commit while the toolkit's own test suite was red; the agent compounded it by asserting clean on partial verification | #822 (first finding, plain-name disposition) | **PINNED** — CI-on-PR runs the toolkit's own test suite (PR #1) with required checks in the ruleset |
| Verify-clean cannot prove coverage | 2026-07-28/29 | mdatron's `files_checked` counts files-with-findings, so a clean repo is indistinguishable from an empty jurisdiction — no envelope field proved coverage | #822 (second finding) | **PINNED** — the non-vacuity canary (#825, PR #2); upgrade available: families tri-state gating (mdatron 2.1.0, tracked #855) |
| Vacuous silent-tests | 2026-07-28/29 | `e0207_silent_*` asserted a code's absence — satisfied whether the check ran clean or never ran; harness read only `findings[].code` | #822 (fifth finding) | **PINNED** — positive controls + same-run sentinels; `pipeline_status == ok` asserted everywhere; non-vacuity proven by re-creating the flip |
| Dormant terminology register | 2026-07-28/29 | `vocabulary_globs` half-wired with `families.vocabulary=inactive` — a load-bearing guardrail not running, and nothing said so | #822 (third finding) | **PINNED** — E0091/E0093 armed and firing (#823) over the governed corpus, in pre-commit + CI |
| Stale + fail-open CI | 2026-07-28/29 | CI referenced the pre-collapse mdatron layout, floated to main, and the pre-commit version guard warned-then-proceeded | #822 (sixth finding) | **PINNED** for the re-pin (PR #1; crates.io 0.5.0 via PR #21). **NONE-YET** residual: the fail-open pre-commit version guard, carried as #855 |
| Jurisdiction self-audit bypass | 2026-07-28/29 | an agent can shrink `.mdatron/config.yaml` file_globs or use `--files` to escape the corpus it is judged against — sanctioned and undetected | #822 (fourth finding); carried as #855 | **NONE-YET** — control direction named (diff-scoped check on jurisdiction-narrowing commits), not designed or built |
| Dead dispatch indistinguishable from clean | 2026-07-19/20 | in the mdatron live fire, three dispatched agents with no launch record or dead sessions were indistinguishable from a clean review round from the outside | mdatron live-fire record; contract Gates member Evidence | **NONE-YET** — never-started fixtures (both signatures) + stalled-agent fixture named, deferred; the launch-sentinel/heartbeat discipline is contract text only |
| Reviewer derail into injected boilerplate | 2026-07 (round 3 of the corrected re-roast) | all four parallel cold reviewers hit harness failure modes; one derailed into echoing instruction-shaped text from a file it read, 0 tool-uses — completed manually | GitHub vsdd-cli #11 (companion observation in the fixture-seed comment) | **NONE-YET** — flagged as its own specimen class (runtime-harness reliability + prompt-injection); no fixture, no control |
| Detection-without-a-seam (escape class) | named 2026-07-29 | the standing escape shape: advisory detection presented as enforcement — detection-only-is-not-governance | #816 practice-adoption relay; #840 REQ-22 honesty bar; meta-harness enforcement-grade ladder | **NONE-YET** mechanized — the honesty bar and AC-24/AC-28 are designed (#840 is 0% built); today it is held by the grade-naming convention alone |

### 3. process-bypass (owned/gated flow skipped)

| Incident | Date | What happened | Record | Pin |
|---|---|---|---|---|
| 137 findings, 82 orphaned | June 2026 | operator directives executed without classification; the flow orphaned; 137 findings accumulated, 82 orphaned, before the operator noticed | #12/#13 children + resolution comments; contract Directive-reconciliation Evidence | **SEEDED-RED** partial — routing format-carry (convention) + the Slice-1 tracker-join falsifier set (commit 50466310, PR #24); the #811 query is built but structurally inert on real data until the join lands; Layer-8 directive classification unbuilt |
| Routing-less fix rounds | 2026-07-27 | Layer 3's phase-3 ran six rounds fix-closing findings with no filed routing — the June orphaning reproduced during bootstrap, because routing (unlike parity) had no format-carry | #806 (full root-cause + routing record); contract Gates Evidence | **CONVENTION + SEEDED-RED** — routing format-carry ratified via #810 and exercised since; Slice-1 mechanization in build (PR #24); unrouted-findings detection pair named (#810) |
| Out-of-process contract edit | 2026-07-27 | the ratified contract was amended by a solo agent inside an execution session on content-approval alone — no composition, no cold review; reverted (9b451099), redone properly as #810 (8c9aaeda) | #808 (deviation record, stands); #810 (the proper re-entry) | **CONVENTION** — amendment format-carry (commit evidence section must cite the ratified review record), exercised first at 8c9aaeda; mechanized home (Layer-7 governed-file conformance) **NONE-YET** |
| Ungated hotfix direct-to-main (mdatron path-confinement) | 2026-07-19 | a security-labeled defect fixed direct-to-main: new unsafe module, new dependency — session machinery used, none of the increment gates; competent work through an ungated lane, which is what made it quiet | mdatron deviation record, relayed via this tracker's defect-fix-lane directive issue; contract Finding-lifecycle Evidence | **NONE-YET** — the entire fix-lane fixture set (ungated-hotfix in both shapes, compliant-fix, retrofit pairs, etc.) is named in the Fixture corpus, unbuilt; the fix-scale gate is build-plan work |
| 18 misplaced reviews | June 2026 | 18 reviews of one repo filed into a sibling repo's review-log; plus a bundled commit with no rollback path | relocation commits in both repos' histories; contract Recorded-review-dispatch Evidence | **NONE-YET** — inline-review fixture reproducing the mdatron dispatch-layer incident (free-slug files, dual-home state, no manifest) named, unbuilt |
| Direct-to-main as working mode | through 2026-07-29 | work landed direct to main during bootstrap despite the methodology's per-milestone PR discipline — retired as a deviation | #822 (F3/F4 decision), PR #1, #824 | **PINNED** — CI-on-PR required checks + server-side ruleset (main push blocked); git-posture hook update PR #23 (#856); merge-is-human stays friction grade, honestly named |
| Off-rails prose proliferation | 2026-07-29 | the superseded `docs/refactor/` lingered two months AND was extended that session with a hand-authored phase-1a spec (with letter-cluster labels) — the exact proliferation the harness aims to prohibit | #822 (seventh finding, operator-named); recorded on #822 as the off-rails-prose finding; cleanup #826 | **CONVENTION** — Design-doc lifecycle member + deviation registry; mechanized block **NONE-YET** (target CI grade named in the contract) |
| Unsigned tracker events under audit mode | 2026-07-19 era | the session's own tracker events ran unsigned under audit-mode enforcement; crosslink's compaction warnings flagged it twice before anyone acted | contract Recorded-review-dispatch + Status Evidence; mdatron tracker records | **NONE-YET** — unsigned-event count detection exists upstream at compaction; the Status wiring that reports it and the unsigned-manifest fixture are unbuilt |
| Reactive hook accretion | suite era | the suite accumulated 13 hand-authored hooks, one per escape path, each written after the recurrence — controls chasing escapes instead of a governed surface | suite records (`../guild-projects/guild-portfolio/vsdd-suite`); contract Conformance-at-action-time Evidence | **NONE-YET** — the act-to-affordance map + registry are the structural answer (data built); the action-time conformance check that would replace per-escape hooks is unbuilt |
| Fix-in-place (escape class) | named 2026-07-29 | the standing escape shape: grind fixes without routing findings to the phase that would have prevented them | #816 practice-adoption relay; #806 is the lived instance | **CONVENTION + SEEDED-RED** — same pin as routing-less fix rounds |
| Relabel-to-dodge-routing (escape class) | named 2026-07-29 | the standing escape shape: relabel a fix-close as a disposition closure (dismissed/consolidated) to dodge the routing obligation | #816 practice-adoption relay; #810's disposition-closure exemption defines the lawful boundary | **NONE-YET** — malformed-disposition-closure seed named in the Fixture corpus, unbuilt |
| Fix-with-no-owning-finding (escape class) | named 2026-07-29 | the standing escape shape: commit a fix with no finding issue filed before it | #816 practice-adoption relay; #840 subsumption notes it as a routing-control bypass | **NONE-YET** — the un-owned ungated-hotfix fixture shape named, unbuilt |
| Edit-the-checker (escape class) | named 2026-07-29 | the standing escape shape: edit the checking mechanism itself; live proof of the risk: Thermite's two edit-time gates silently de-wired by a settings clobber and nothing caught it | #816 practice-adoption relay + the outbound to mdatron#96; crosslink#15 / Thermite#93 | **NONE-YET** — the stance (CI runs the integrity logic over the checker's own wiring) is recorded; the tamper-proof residual routes to #815 corroboration, unbuilt |

### 4. spend-escape (unbounded or unaccountable expenditure)

| Incident | Date | What happened | Record | Pin |
|---|---|---|---|---|
| The 39-agent fan-out | 2026-07 (relayed on the record) | a phase-3 roast's 4 declared cluster reviewers fanned out to 27 undeclared per-finding verifiers — 39 agents, 2.27M tokens; the refutation mechanism was silently swapped for one never specified; the gate had ratified the lenses, not the run's shape | GitHub vsdd-cli #11 (the specimen + the accepted fixture-seed comment with the malformed/well-formed contrast pair); mdatron#122 process note | **NONE-YET**, loudly — the declaration-completeness gate + uncapped-fan-out falsifier are ratified design (#840 REQ-21, 0% built); the fixture-seed material sits accepted on #11 awaiting the gate-leg red-gate authoring; runtime admission control named UNBUILT (detective, not preventive) |
| Re-derived reference material | 2026-07-18 session | the design session re-derived reference material that recorded usage would have flagged for reuse | contract Cost-is-knowable Evidence | **NONE-YET** — the records-based efficiency engine (#840) is unbuilt |
| Calibration bands never updated | suite era | cost calibration bands were authored once and never updated from actuals | suite claude-code-contract records; contract Cost-is-knowable Evidence | **NONE-YET** — completed-cycle-with-calibration-bands fixture named, unbuilt |

### 5. naming-coinage (invented labels and reified utterances)

| Incident | Date | What happened | Record | Pin |
|---|---|---|---|---|
| Letter-cluster recurrences | suite era ×4; design session ×2 | the letter-cluster label pattern recurred four times past written correction in the suite era, then twice in the design session past a loaded memory | contract Conformance-at-action-time Evidence; suite records | **PINNED** — MDATRON-E0091 (letter-cluster prohibition) armed over the governed corpus (#823), firing in pre-commit + CI |
| Coinage recurrence during the coinage analysis | 2026-07-29 | the M-/F-labels used across #822 (and A1–Q5 in the #14 draft) were themselves letter-cluster coinages — committed while analyzing E0091; operator-caught, plain names adopted henceforth | #822 naming-discipline correction (decision comment) | **PINNED** — same armed register; the recurrence is cited on #822 as the evidence the discipline needed mechanization, not vigilance |
| 80-instance coinage debt | 2026-07-29 | arming the register surfaced an 80-instance / 12-scheme coinage debt in the governance docs — the register could not land until triaged | #822 (arm attempt) → #823 (full cleanup) | **PINNED** — #823 adjudicated the schemes and landed the armed register clean |
| Single-utterance reification | suite era / June 2026 | single operator utterances became named schema concepts; illustrative examples were wired as enumerated special cases; forward-only policy produced backwards-compatibility work on schemas still being designed | contract Directive-reconciliation + Conformance Evidence | **NONE-YET** mechanized — the maturity lifecycle / registration-act discipline is convention; E0090 (unregistered-coinage) covers the bold-introduced-term slice only |
| Doc-type proliferation via free filename slugs | suite era | documentation types routed around per-file validation through free filename slugs | contract Conformance-at-action-time Evidence | **PINNED** (partial) — mdatron jurisdiction (`file_globs` allowlist) + `require_frontmatter`/W0040 close the free-slug route for the governed corpus |

### 6. staleness-fabrication (records asserting what was never true or no longer true)

| Incident | Date | What happened | Record | Pin |
|---|---|---|---|---|
| Hallucinated citations | June 2026 | 7 of 8 findings in a bookmark-cli review round cited code absent from HEAD | `../guild-projects/guild-portfolio/vsdd-suite-reference-examples/bookmark-cli-manual` round-3 record; contract Finding-lifecycle Evidence | **NONE-YET** — citation-verification-failure seeded-finding fixture named, unbuilt; mdatron per-route `citations: true` (dead file:line refs block) exists as an adjacent upstream control |
| Fabricated cost figures | suite era | suite-era agents fabricated token and dollar figures because measurement was operator-only | suite `claude-code-contract.md` records; contract Cost-is-knowable Evidence | **NONE-YET** — #840's could-not-check-never-fabricated rule (REQ-14) is ratified design, 0% built |
| Fabricated contract quote | 2026-08-01 | the orchestrator's paraphrase ("names the manual-dispatch fallback as must-not-calcify") was propagated across two comments as if quoted contract text — the phrase appears nowhere in the contract; caught by three-lens cold-review convergence, corrective note filed | #845 (corrective note, 2026-08-01 21:18); GitHub #12 closure cites the convergence | **NONE-YET** mechanized — caught by review convergence + the corrective-note practice; no quote-verification check exists |
| False verified-in tag | 2026-08-01 | the remediation draft carried a "verified" tag for a mechanism that had not been verified — named in-round as the fabricated-verification class; removed and replaced with the correct mechanism citation | #845 round-2 record (item 14) | **NONE-YET** mechanized — caught by the delta-focused re-verify round only |
| Fabricated-existence assertions | 2026-07-28 | three in one session: a "contracted" code, a live-dormant query called mechanized, a "forcing seam that exists today" — capabilities asserted as existing without reading the repo | durable codification: #840's grade-honesty banner + authored-not-exercised governing law; per-incident record lives in agent memory (verify-built-state-before-asserting), not the tracker — itself a record-home gap | **NONE-YET** — the exercise-registry (authored control must carry a fired-proof + negative-case fixture) is ratified design, unbuilt |
| Numeric-claims drift | pre-2026-07-28 | three numeric-claim drift incidents in governed prose (counts asserting stale numbers) — shaped mdatron's fixture set verbatim | #816 relay (Raise 2) | **PINNED** upstream — MDATRON-E0094 numeric-claims (vocabulary family), fed by our exact cases |
| Orphaned code catalog | 2026-08-01 | 9 VSDD codes cited in 6 live dispatch-injected files with no surviving defining home | #854 (open) | **NONE-YET** — open issue, no control |
| Dangling citers after doc retirement | 2026-08-01 | the Solution-Owner change-authority rule and per-milestone PR discipline lived only in deleted homes (methodology.md, README), leaving every referencing domain/primer without a contract home | contract SO-change-authority + PR-discipline Evidence (the #826 cleanup review's dangling-citer finding, fix round eec731bd) | **CONVENTION** — citer-sweep zero-stale-refs acceptance check (#845 remediation), hand-run at landing; mdatron citations available for the file:line slice |

### 7. topology-calcification (workarounds that silently became the architecture)

The class is pinned as a class by the deviation registry (`.vsdd/registry/deviation-registry.yaml`, ratified #845, disposition comment 2026-08-01 21:59) plus the deviations gate leg, whose red-gate seed is committed (`vsdd-core/tests/deviations_red_gate.rs`, commit fdbc9546, draft PR #24 — lapsed-expiry, fired-trigger, premature-resolution, absent/shape-invalid fail-closed, abusive-override directions). Honest residual, named in the design: the *registration* half (that a deviation gets an entry at all) stays convention grade — the unregistered deviation is the residual no gate reaches.

| Incident / entry | Date | What happened | Record | Pin |
|---|---|---|---|---|
| Hand-split slice topology | through 2026-08-01 | parent contract + five independently-pipelined slice docs (+ the #840 sidecar + a built slice with none) — a workaround crosslink cannot model, invisible as a program, forgoing swarm's gates and ordering | #845 (finding + operator re-scope: a deviation to remediate, not a pattern to ratify); registry entry `hand-split-slice-topology` | **PINNED** (data) — entry `resolved` under SO disposition; closing boundary commit 0d6ab1f5; gate leg SEEDED-RED |
| Manual-dispatch fallback calcified | through 2026-08-01 | runtime-harness dispatch rode in place of crosslink kickoff/swarm; recorded in the map's condition field and calcified anyway — across two repos, no crosslink workflow was ever self-summoned | contract affordance-map Evidence; remediation design Summary (the founding-evidence reading); registry entry `manual-dispatch-fallback` | **PINNED** (data) — standing entry, retest trigger upstream #55, expiry 2026-10-30; gate leg SEEDED-RED |
| Container-kickoff blocked posture | entered 2026-08-01 | attended tmux kickoff as the working posture, previously loose prose in the map's condition field | registry entry `container-kickoff-blocked-posture` (#849; upstream #9/#10; recorded on #597) | **PINNED** (data) — standing entry, trigger #9-or-#10-closed, expiry 2026-10-30 |
| Dial-less crosslink dispatch | entered 2026-08-01 | swarm's launch surface hardcodes model, exposes no effort dial — the dials discipline cannot ride it | registry entry `dial-less-crosslink-dispatch` (#852; upstream #61) | **PINNED** (data) — standing entry, trigger #61-closed; the swarm-entry binding carries the dial condition (contract, REQ-8) |
| crosslink-from-develop consumption | entered 2026-08-01 | routing-gate.yml consumes crosslink from the fork's develop branch, a re-pin owed at the develop-to-main merge | registry entry `crosslink-develop-consumption` (magnificentlycursed/crosslink#1) | **PINNED** (data) — standing entry, trigger fork#1-closed |
| Hand-authored build-plan | entered 2026-08-01 | the build-plan projection is hand-authored, a bootstrap fallback pending Slice 2's generator | registry entry `hand-authored-build-plan`; build-plan preamble (self-declared founding entry 7) | **PINNED** (data) — standing entry, artifact-presence trigger (grep-decidable), expiry 2026-11-29; plus the Decomposition content-hash pin (convention until the doc-drift check mechanizes) |
| mdatron sibling-pin lag | 2026-08-01 → resolved 2026-08-02 | CI sibling checkout tag-pinned v0.4.0 against a v0.5.0 latest — a persisted toolchain-pin lag; resolved by adopting 0.5.0 from crates.io and retiring the checkout surface | registry entry `mdatron-sibling-pin-lag` (#816) | **PINNED** (data) — entry `resolved` under SO disposition 2026-08-02; the full lifecycle exercised |

### 8. upstream doc-vs-code (dependencies whose documentation and behavior diverge)

| Incident | Date | What happened | Record | Pin |
|---|---|---|---|---|
| Agent block-list looser than documented | 2026-07-20 | shipped `agent_overrides` reduce the agent block list to destructive-ops only while four upstream docs + the generated prompt claim "no push, no merge, gated commits" — strict blocks are prompt-level only | `attended-design-autonomous-execution` knowledge page (upstream-mismatch list) | **NONE-YET** — on the upstream findings list; no local fixture |
| Container path prompts despite no-prompt docs | 2026-07-20 | `kickoff run --container` does not pass the permission-skip flag (only `container start` does) — the documented no-prompt path holds for one of two entry points | same knowledge page | **NONE-YET** |
| Timeout sentinel never written | 2026-07-20 | nothing writes the TIMEOUT sentinel harvest checks for; a timeout kill leaves RUNNING, caught only by the wall-clock check | same knowledge page | **NONE-YET** — the wall-clock check is an incidental catch, not a pin |
| Swarm header-grammar doc/code mismatch | 2026-08-01 | crosslink's docs vs `design_doc.rs` on the H3-under-Requirements grouping the completed-phases mechanism leans on | #847 / upstream dollspace-gay/crosslink#57; remediation design REQ-1/REQ-9 | **DECLINED-WITH-MITIGATION** (recorded in the founding sweep) — the swarm-entry exactness check (AC-7a) catches parser drift at the consumption point; hand-run |
| Design-skill destructive-on-iterate | 2026-08-01 | the skill's initialization section carries no skip-on-continue qualifier, so a skill-literal `--continue` re-runs the sidecar heredoc — modification from inside the sanctioned amend path | #846 / upstream dollspace-gay/crosslink#56; contract Design-doc-lifecycle member (raise + preempted justification) | **NONE-YET** — upstream raise filed; locally only the preemption clause (convention) |
| Dead-end remediation instruction | 2026-08-02 | the work-check hook directs `crosslink intervene` — a command 0.9.0-beta does not ship; the contract's "wired-in" claim is authored-not-exercised; the local hook is a gitignored deployed payload, so the repoint is re-deploy-fragile | #856 / upstream dollspace-gay/crosslink#71; PR #23 for the git-posture half | **NONE-YET** durable — local repoint live but re-deploy-fragile; open until upstream #71; `crosslink workflow diff` is the interim visibility surface |
| Cold-read onboarding dead-ends | 2026-07-29 | the mdatron #49 cold read hit a circular dead-end ("run mdatron init first" when init did not exist), undocumented config format, output-marking violation, namespace contradiction | #816 (cold-read report + consolidated relay) | routed **upstream** (mdatron docs pre-v1.0); the same dead-end class recurred locally as #856 |
| Settings clobber de-wired live gates | ~2026-07-28 | a crosslink-init settings clobber silently de-wired Thermite's two edit-time gates; nothing caught it — the live proof against in-repo self-protecting guards | crosslink#15 / Thermite#93, recorded in #816's outbound decision | **NONE-YET** — motivates the installed-artifact-integrity check + hollow-shell fixture (named, unbuilt); durable answer is CI-tested integrity, per the recorded stance |

---

### the none-yet roster (red-gate-authoring debt, actionable)

**35 of the 63 indexed incidents have no built fixture or mechanized control pinning them.** Grouped by where the debt lands:

**Owed to the fix-lane / gate fixture corpus (build-plan phases; named members, unbuilt):**
1. The layer-7 red-gate-cheat reproduction fixture (from its source record)
2. The fix-lane set for the ungated-hotfix incident (both shapes) and its siblings
3. The malformed-disposition-closure seed (relabel-to-dodge-routing)
4. The un-owned-fix shape (fix-with-no-owning-finding)
5. The inline-review fixture (the 18-misplaced-reviews / dispatch-layer incident)
6. The zero-finding inline-review variant (the 23 empty no-findings issues) — flagged in the corpus itself as reached by no enumerated falsifier today
7. Never-started (both signatures) + stalled-agent fixtures (the indistinguishable dead dispatch)
8. The hollow-shell tree + installed-artifact-integrity check (also pins the Thermite settings-clobber class)
9. The mis-rooted-session fixture
10. The citation-verification-failure seeded-finding fixture (hallucinated citations)
11. The unsigned-manifest fixture + Status unsigned-event wiring
12. The completed-cycle / calibration-bands fixture

**Owed to the #840 subsystem build (ratified design, 0% built):**
13. The uncapped-fan-out falsifier — seed material accepted and waiting on GitHub #11 (the 39-agent / 2.27M specimen + the well-formed declared-ceiling contrast) — the single loudest item
14. The records-based efficiency engine (re-derivation and fabricated-cost classes)
15. The exercise-registry with negative-case fixtures (fabricated-existence / authored-not-exercised class)
16. The escape-corpus artifact itself (REQ-22 / AC-24) — this index is input material, not the artifact
17. The corroboration keystone #815 (forge-a-self-authored-record, edit-the-checker's tamper-proof residual, postdated-approval, self-re-arm CI leg)

**Owed as mechanization of exercised conventions:**
18. Layer-7 governed-file conformance (the amendment format-carry's mechanized home; out-of-process-edit class)
19. The doc_hash-vs-bytes + disposition CI check (the sidecar hand-edit class)
20. The off-rails-prose block (Design-doc lifecycle's target CI grade)
21. Layer-8 directive classification (the 137-findings orphaning's root)
22. The action-time conformance check that retires reactive hook accretion
23. Phase-exit gate commands (the thrice-closed-still-open class)
24. Quote/verification-tag checking has no mechanized form at all (fabricated-quote, false-verified-tag) — caught only by review convergence

**Tracked as open issues:**
25. #854 (orphaned code catalog)
26. #855 (jurisdiction self-audit bypass; fail-open version guard; families-tri-state upgrade)
27. #856 / upstream #71 (durable intervene repoint)
28. The three attended-design upstream mismatches + upstream #56 (destructive-on-iterate) — upstream relays, no local pin

**In flight (SEEDED-RED on draft PR #24, pinning lands at green):** the vacuous-gate falsifier (`tracker_join_falsifiers.rs:587`) and the deviations gate leg seed (`deviations_red_gate.rs`) — the false-assurance Fix 2 and the registry-expiry mechanization ride these.

**Named class residual with no gate by design:** the unregistered deviation (the registration half of the registry) — convention grade, honestly named in the ratified remediation design; and the mdatron register/coinage mechanization covers governed prose, not tracker comments.

