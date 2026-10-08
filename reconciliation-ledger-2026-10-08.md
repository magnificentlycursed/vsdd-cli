---
title: "Reconciliation ledger (2026-10-08)"
tags: ["review", "evidence", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---


## Design Specification

### status

**Record of the 2026-10-08 reconciliation sweep; items unrouted pending the operator's decision on their home.** Every recorded decision, finding or obligation touching a build-plan phase that the contract, the build-plan, the data sets or the owning issue does not reflect, or that conflicts with another record, across Slices 1 to 7. Counted: 26 items recorded nowhere as an open item for their slice, 13 recorded somewhere the slice would not find them; the operator had flagged two before the sweep (the session-start hook's home; the vsdd-core crate collapse). The mechanism behind the accumulation is on `append-accumulation-retrospective-2026-10-08`; the reference practice the items are measured against is on `reference-practice-design-documents-and-estate-divergences`. Each item's intended disposition under the reference practice is an inline resolution in the owning slice's design document, with a routed comment on the owning issue citing this page; none has been routed yet.


Scope: every recorded decision, finding or obligation touching a build-plan phase that the contract, the build-plan, the data sets or the owning issue does not reflect, or that conflicts with another record. Sources read: the contract at main 609f9132 (every behavioral member, Requirements, Acceptance criteria, Verification architecture, Architecture, Decomposition, References), the build-plan (open phases and Completed phases), the exception register, templates/registry (economics-data, installed-artifact-manifest, dispatch-data), the mdatron schemas, vsdd-core/src/state/schema.rs, Cargo.toml, .gitignore, .githooks, manual-tests/, the open issues (#881 #876 #875 #855 #844 #843 #842 #839 #836 #818 #713 #664 #657 #656), the closed issues #820 #15 #14 #835 #870 #873 #885 #891 #882 #845 #859 #620 #821, and the knowledge pages composition-slice, content-delivery-assessment-2026-10-02, mdatron-0.7.0-adoption-review-2026-10-06, crosslink-integration-surfaces, verifiable-conformance-and-efficiency, regression-corpus, gate-execution-slice.

Status key: UNFLAGGED = recorded nowhere as an open item for the slice; PARTLY = recorded somewhere, but not where the slice's design would find it, or recorded with a contradiction; KNOWN = already on the owning issue as an open question (listed for completeness, not counted).

### slice 1 (complete; retroactive)

| # | Item | Status | Evidence |
|---|---|---|---|
| S1-1 | The vsdd-core crate merge (#15) was folded into #820 as a phase-2a act and never done; not among the three recorded residuals; no open issue. The reason to keep two crates (a cost crate) was retired by #860/#870. | UNFLAGGED (operator found it 2026-10-08) | Cargo.toml members; #820 comment 1202; build-plan Completed phases (residuals a, b, c only); contract Architecture still says "decided when the collapse schedules" |
| S1-2 | The #14 fold-in carried the vsdd subprocess client, init drift-handling and template deployment. Init and templates landed as Slice 3's static half (#838). The subprocess client was closed as defunct (#835, 2026-07-30). Nothing on #820 or the build-plan says where the three pieces went. | PARTLY (resolved in substance, unrecorded on the slice) | #14, #835, #838 |
| S1-3 | Manual-test checklist: residual (c) on the build-plan still says "owed"; #820's close says the 2026-08-02 decline is superseded by the 2026-09-18 "make it real" ruling with drafts under #874; manual-tests/slice-1.md and slice-3-static.md exist (2026-09-28) with no recorded operator adoption on #820. The build-plan residual and the files disagree. | PARTLY | build-plan Completed phases; manual-tests/; #820 comment 1479 |
| S1-4 | The per-commit wall-clock budget was deferred "until a per-commit hook is bound". A per-commit hook has since been bound (the fail-closed pre-commit version guard, #855 residual 2 / PR #59) with no budget authored. The slice obligation fired silently. | UNFLAGGED | contract Decomposition, Slice obligations; .githooks/pre-commit; #855 |
| S1-5 | The register entry hand-authored-build-plan (owned by closed #845) expires 2026-11-29; a lapsed expiry fails the deviations gate leg on every PR. Its trigger fires only when Slice 2 lands. Nothing watches the date. Same hazard on crosslink-develop-consumption, synced-trace-oracle-unbuilt and native-spawn-interceptor-unbuilt (all 2026-11-30, two owned by closed #873). | UNFLAGGED | .vsdd/registry/deviation-registry.yaml |
| S1-6 | The upstream raise crosslink#52 (bulk issue query with comments) was filed as the exit for the bounded per-finding fan-out in acquisition. No register entry, retest trigger or issue tracks it. | UNFLAGGED | #820 comment 1175; register has no entry |
| S1-7 | Install count: Slice 1's Completed-phases entry says 16 templates / 63 artifacts; Slice 3's and the contract say 15 / 62; templates/ holds 17 files; the manifest holds 20 entries. The SO ruled 2026-10-02 (#881 item 2) to replace the literals with citations "in the Slice 2 phase-1a amendment (#839)". Not recorded on #839. | PARTLY | #881; contract Install behaviors; build-plan |
| S1-8 | The disposition label-carry (dismissed / hallucinated / consolidated labels) was adopted as a build convention with its contract-level promotion deferred to the amendment loop. The contract names the schema token but not the label-carry; the build-plan assigns the promotion to Slice 5. Consistent, but the adopting decision lives only on #820 comment 1242. | KNOWN (Slice 5 bullet) | #820; build-plan Phase 4 |

### slice 2 (current; phase-1a opening)

Operator-flagged this session: the session-start hook home (S2-1) and the crate collapse (S1-1).

| # | Item | Status | Evidence |
|---|---|---|---|
| S2-1 | Guardrail wording: contract Slice 2 member, Verifiable conformance ("its control is the session-start hook"), Architecture ("installed by init into .claude/ and .crosslink/ per crosslink conventions") and the retired design's REQ-10 all place the hook in crosslink's payload. Findings 2026-10-02: a project cannot ship crosslink hook scripts; its settings entries are replaced by any plain init and always in kickoff worktrees; operator decision 2026-10-01 "no custom crosslink setup". Decision 2026-10-02: supplements via Claude Code rules files; the remaining hook is vsdd's own Claude Code entry. The contract is unamended and the three places are not listed on #839. | PARTLY (operator flagged) | content-delivery-assessment page; PR #50; #839 comments 1577/1578 |
| S2-2 | .vsdd/state.yaml does not exist in this repo. SO ruling 2026-10-01 (#881 item 4): it lands with Slice 2 when the composition function can compute config_inputs_hash; until then register a bootstrap exemption with retest trigger "Slice 2 composition function ships". No such register entry exists; the ruling is not on #839 or the build-plan Phase 2. The contract's Deterministic phase answer calls the artifact authoritative. | UNFLAGGED | ls .vsdd/ (events, registry only); deviation-registry.yaml; #881 |
| S2-3 | No project configuration exists: the contract says the composition reads "the project's declared characteristics in DESIGN.md"; this repo has no DESIGN.md (only templates/DESIGN.md.vsdd-template for adopters) and no .vsdd/config.yaml (init writes only a version stub). The "thorough" preset is declared only in contract prose. Self-governance cannot compose until both files exist, and nothing says who writes this repo's. The build-plan entry decision names a reader and a closed vocabulary but not the missing instances. | UNFLAGGED | find DESIGN.md; ls .vsdd; init.rs stub; contract Project configuration |
| S2-4 | Generator output shape: contract Generated context says per-domain sections, each reviewer its slice, "priced per section"; the 2026-10-02 decision says three outputs (kickoff block, rules files, skill files). A rules file is a whole supplement for a session with no domain; the token-budget classes (session-skill, domain-prompt, phase-primer, supplement-section, always-on-core) map onto neither rules files nor skill files. The claude-code-cli supplement at 9.7 KB exceeds the supplement-section budget if emitted whole. | UNFLAGGED | contract Requirements: Generated context; economics-data token_budgets; #839 comment 1578; AI Engineer minor on the assessment page |
| S2-5 | The kickoff block has no owner at its seam: kickoff's template is a file path with eight fixed placeholders and a silent default on a missing file; the dispatcher that writes it is Slice 6 (depends on Slices 2 and 5). The Red Team's fail-closed rule (the rendered prompt must contain every composed member) and the Solution Architect's pure-delimited-separately-hashed block are on the knowledge page only. Neither #839 nor the build-plan Phase 2 or 5 names the block's format or who fails it closed. | UNFLAGGED | kickoff-swarm-dispatch-pipeline page; assessment page reviewer findings |
| S2-6 | Compaction: the AI Engineer's major finding that inlined kickoff content does not survive compaction (crosslink passes neither an agent type nor a system-prompt file) has no recorded disposition; question 10 to the operator (raise upstream or accept detection only) has no recorded answer. Bears on whether the kickoff block is a valid delivery form at all. | UNFLAGGED | assessment page, AI Engineer findings and questions 6, 10 |
| S2-7 | Supplement activation trigger: the contract requires every supplement to declare one as a required field; supplement.json has no such field; the rust supplement's own prose calls the field, its mdatron schema change and the composition wiring "the named Slice-2 cross-repo follow-on". The 2026-10-02 decision maps the field onto rules-file path scoping, which covers task-determined supplements only; the runtime- and project-determined always-on tiers need a different value. Not on #875's unauthored list, not on #839 as a data item. | UNFLAGGED | .mdatron/schemas/supplement.json; supplements/rust.md Activation section; contract Verifiable conformance three tiers |
| S2-8 | The edit-gate hook for the brand-new-file case: Deterministic composition (Slice 2's member) names "a hook backstop when a file is edited before its matching supplement was read"; the 2026-10-02 decision says the new-file case "is covered by the edit gate"; neither build-plan Phase 2 nor Phase 3 lists who builds it. | UNFLAGGED | contract; #839 comment 1578; build-plan |
| S2-9 | The "session skill" has no source: the contract names it five times (session-start injection, Generated context, token budget class session-skill 5000, Slice 6 "the session skill with change classification"); no file, template or command defines it. The generator's first named output has nothing to emit. | UNFLAGGED | grep across templates, .claude/commands, supplements, vsdd-core, vsdd |
| S2-10 | The build-plan generator: the build-plan preamble, the contract Decomposition line 320 and the register entry all say Slice 2's generator will produce the build-plan from the contract; the Slice 2 member and Phase 2 bullets list no such deliverable. | UNFLAGGED | build-plan line 5; contract line 320; register entry hand-authored-build-plan |
| S2-11 | Install-offer re-open: SO ruling 2026-09-28 re-opens the install-offer members the contract marks closed and builds the six-direction fixtures in Slice 2; the contract's Install behaviors still says "static half CLOSED ... with the install-offer conduct fixtures in every enumerated direction"; build-plan Phase 2 does not list the fixtures; #881 item 1 (closure claims without oracles) is open on exactly this. | PARTLY | #839 comments 1529/1540; contract Acceptance criteria; #881 |
| S2-12 | Install count citation fix ruled into "the Slice 2 phase-1a amendment (#839)" on #881 and not recorded on #839. | PARTLY | #881 item 2 ruling 2026-10-02 |
| S2-13 | Install manifest classes: the schema enum has command-listing, plugin-listing, rules and others but no skill class and no content-hash resolution kind; init hardcodes .claude/commands/. The 2026-10-06 note records the missing skill class; the missing content-hash kind is on the assessment page only ("the install manifest cannot detect blanked rule files"). | PARTLY | installed-artifact-manifest.json enum; #839 comment 1603; assessment page corrections |
| S2-14 | Cacheable portion: the pricing function emits "cacheable portion bounded by the total" with no grounded definition for the kickoff path (crosslink's built prompt precedes vsdd's block and varies by version; per-worktree directories make cross-agent cache reuse nil). | UNFLAGGED | assessment page, Solution Architect and AI Engineer findings |
| S2-15 | Composition-mode enumeration: state schema has skill-interactive and cold-dispatch; the primers declare skill-interactive (7), operator-orchestrated (2), reviewer-cold-session (1). | KNOWN (ontological review 2026-09-24) | schema.rs; .claude/commands/vsdd-phase-*.md |
| S2-16 | Presets omit the process-governing set; "core" has three meanings; active_domains is prose; the dispatch shape has no ceiling field. | KNOWN (ontological review; SO ruling 2026-09-28) | economics-data presets |
| S2-17 | Skills packaging: frontmatter coexistence, description-not-mechanism, 19 files naming the commands folder, the skills folder ignored wholesale, the mdatron config must move with the files, the standards-pack profile reuse. | KNOWN (#839 comments 1568, 1603) | |
| S2-18 | Tokenizer, stamp placement, loader shape, DESIGN.md reader vocabulary. | KNOWN (build-plan Phase 2 entry decisions) | |
| S2-19 | mdatron's config-integrity family (pair co-activation, pair separation, validator-differs-from-owner) is "a flagged cross-repo dependency gating on the release that ships it"; no mdatron GitHub issue mentions co-activation or config integrity (search 2026-10-08). Slice 2 would land with the integrity rules unenforced and no filed ask. | UNFLAGGED | gh issue list on magnificentlycursed/mdatron |

### slice 3, generated half (rides slice 2)

| # | Item | Status | Evidence |
|---|---|---|---|
| S3-1 | #842 (operator 2026-07-31: REMOVE the vsdd observe PR-body workflow template, the init.rs deploy entry and the install-slice test assertions, "sequenced after #840 lands") is undone two months after #840 closed; the template is still deployed and the adopter vsdd-verify.yml template still calls the nonexistent `vsdd verify check`. | PARTLY (open issue, unsequenced) | templates/.github/workflows/; #842 |
| S3-2 | #713 nit 2: the installed-artifact manifest says statusline wiring is user-level; no statusLine key exists on the host. Re-homed to the install slice 2026-07-29; open since. | KNOWN (open) | #713 |
| S3-3 | #664: the vsdd crate name is unclaimed on crates.io; the adopter templates fail closed until it exists; reserving it is the operator's act. | KNOWN (open) | #664 |
| S3-4 | The generated members' install path depends on the skill class (S2-13) and the keep-line in the managed ignore block that a plain crosslink init rewrites (crosslink#20). | KNOWN (#839 comment 1568 item 4) | |

### slice 4

| # | Item | Status | Evidence |
|---|---|---|---|
| S4-1 | Oracle home conflict: #876 and #836 (SO rulings 2026-09-28/29) say option (b), signed trace to a hub/data branch by a non-agent actor, "lands as a Slice 4 phase-1a contract amendment (#836)". #873 (2026-10-08, a closed issue) says re-home the oracle to a dispatcher-written, operator-signed transcript digest, "a Slice 6 design-first amendment". Neither #876 nor #836 records the supersession. The register entry synced-trace-oracle-unbuilt carries the 10-08 direction. | UNFLAGGED (two rulings, two homes, no cross-record) | #876, #836, #873, deviation-registry.yaml |
| S4-2 | The Slice-4 conformance-verifier leg, the control-effectiveness registry and the persona and supplement checks all read the oracle. Until the oracle re-home is ratified (Slice 6), every Slice 4 verifier verdict is could-not-check by the contract's own rule, and the registry's "fire-check" pairs have no trace to read. The build-plan Phase 3 states the could-not-check for the phase derivation only. | UNFLAGGED | contract Verifiable conformance; build-plan Phase 3 |
| S4-3 | The ratified Slice 4 design (gate-execution-slice, 2026-07-30) predates: the 80% floor ruling, the swarm removal (#891), the Verifiable conformance legs assigned by #894 (five legs to Slices 4 and 6), the oracle rulings, and the contract compaction. The build-plan says Phase 3 is its projection. Same append-accumulation shape as Slice 2's. | UNFLAGGED | knowledge page gate-execution-slice; #836 |
| S4-4 | Slice 4 "depends on Phase 2's composition behavior (fix-scale labels; the config's mutation-floor activation field)"; the review config loader is Slice 2's and the config file does not exist (S2-3). The mutation floor "config declares only whether the criterion is in force" has no field anywhere yet. | UNFLAGGED | build-plan Phase 3 Depends on |
| S4-5 | #818 (security hardening, high, open): three convergence redlines adopted 2026-09-28; its remaining fixes (forgeable state envelope provenance, visible-prose injection) are not assigned to any slice. | PARTLY | #818 |
| S4-6 | Fix-lane fixture corpus (~40 members) and the runnable-mini-repo form are Slice 4's; the regression corpus page says the named evasions are "built when Slice 4 needs them". Consistent. | KNOWN | |
| S4-7 | The commit-msg friction hook (Slice 1 residual a) is "Slice 4's deliverable"; build-plan Phase 3 names only "a thin pre-commit wrapper running the fix-scale gate". A commit-msg routing hook and a pre-commit fix-scale gate are different hooks. | UNFLAGGED | build-plan Completed phases residual (a); Phase 3 Guardrail bullet |

### slice 5

| # | Item | Status | Evidence |
|---|---|---|---|
| S5-1 | No authored design; build-plan says design-first. The branch grammar with its refs-query seam (an engine data set Slice 5 depends on) is unauthored, "left until a slice needs them" (#875). Consistent but the dependency is not on #875 as Slice 5's trigger. | KNOWN | #875 |
| S5-2 | #843 (events-store decommission, high, open since 2026-07-31; ontological review 2026-09-24 confirms init.rs still creates events.jsonl and a retired-schema events file is tracked). The contract says the store must not exist. No slice owns it. | PARTLY (open, unhomed) | #843 |
| S5-3 | #656 / #657 (driver-key blob, phantom co-author trailers on public main): one bundled history-rewrite decision, operator's, pending since 2026-07-20; re-homed to "the Security lane" 2026-07-31; no slice. | KNOWN (open, operator's) | #656, #657 |
| S5-4 | The doc-drift check (sidecar hash vs body, flag on .design/ changes with no decision) is named in the contract's Solution Owner change authority as the target and in the regression corpus as "Slice 5"; the pin family landed early (#895) covers the build-plan's contract pin only. The sidecar check is in no slice's bullets. | UNFLAGGED | contract Solution Owner change authority; regression-corpus B; build-plan Phase 4 |

### slice 6

| # | Item | Status | Evidence |
|---|---|---|---|
| S6-1 | The oracle re-home ruling (2026-10-08) and the hub-ref protection finding (container login is a write token to unprotected crosslink/* refs) are recorded on #873, a closed issue, and in a register entry's stated_reason. Slice 6's design-first step has no open issue carrying them. | UNFLAGGED | #873 closed; register |
| S6-2 | The operator-session audit ("session-start-hook-fired and session-skill-invoked are audited ... over the synced copy only") has no oracle after the re-home: the dispatcher-written digest covers dispatched agents, not the operator session. With S2-1, the hook itself moves to vsdd's own entry; what records that it fired is undefined. | UNFLAGGED | contract Verifiable conformance |
| S6-3 | Two open questions on #875 (unresolvable phase at dispatch; state-artifact versus breadcrumb precedence) are Slice 6's by the record; but the phase is also Slice 2's composition input, so the composition function's handling of a null phase is Slice 2's to decide first. | PARTLY | #875 |
| S6-4 | Kickoff plan permission mode as the critic's read-only posture is "untested, to test at the live fire" (#891); whether such an agent can still write its status file and post comments is on the assessment page as unverified. | KNOWN | #891; assessment page |
| S6-5 | The review stage (per-reviewer fan-out, child issues, de-duplication and filing) was homed in Slice 6 by #891; the Solution Owner's finding that it is "unpriced scope growth" needing "a stated size" has no recorded size. | PARTLY | assessment page SO major; #891 decision 1583 |
| S6-6 | Hardening (managed settings; a signing key the agent cannot read) is deferred past Slice 2 by decision 2026-10-02 (7); the Red Team's blocker that the driver key is the same key the interactive agent signs with means approve-then-dispatch cannot distinguish an operator act from an agent act. The contract's Trust boundaries already says so for decisions; Slice 6's manifest gate is specified as if it could. | PARTLY | contract Recorded review dispatch; Trust boundaries; assessment page Red Team blocker |
| S6-7 | The register entry native-spawn-interceptor-unbuilt says "re-arm on a filed upstream or harness issue"; none is filed; expiry 2026-11-30. | UNFLAGGED | register |

### slice 7

| # | Item | Status | Evidence |
|---|---|---|---|
| S7-1 | The report reads "the harness-produced traces (the transcripts and Phase 5's dispatch manifests)". After the oracle re-home the transcript stays local and agent-writable; the report's provenance tags would be could-not-check on every transcript-sourced figure unless the dispatcher-written digest carries usage by cache class. The 10-08 ruling mentions "transcript digest and usage" without defining the usage fields. | UNFLAGGED | #873 ruling; contract Cost is knowable |
| S7-2 | Crosslink's usage harvest drops the cache-creation token class; the transcript has it. Recorded on the assessment page only. | PARTLY | assessment page Test 2 |
| S7-3 | The CI wiring of the token-budget gate is Slice 7's while the gate is Slice 2's; with S2-4 the budget classes themselves are undefined for the new output forms, so the wiring target is undefined too. | UNFLAGGED (consequence of S2-4) | |
| S7-4 | The rightsizing signal catalog and tier-and-effort defaults are unauthored (#875, deferred until a slice needs them). | KNOWN | #875 |
| S7-5 | "The completed-cycle fixture" baseline requires a cycle completed under the built process; none exists and no slice produces one before Slice 7. | UNFLAGGED | contract Cost queries criterion |

### cross-cutting

| # | Item | Status |
|---|---|---|
| X-1 | Four standing register entries expire 2026-11-29 and 2026-11-30; two are owned by closed issues (#845, #873). A lapsed expiry turns routing-gate red on every PR. | UNFLAGGED |
| X-2 | Both retired ratified slice designs (Slice 2 of 2026-08-01, Slice 4 of 2026-07-30) predate every October decision; the build-plan calls each phase "its projection". | UNFLAGGED |
| X-3 | Decisions for a slice are recorded on other slices' or closed issues: #881 (Slice 2 items 2 and 4), #873 (Slice 6 oracle), #836 (Slice 4 oracle, superseded), #820 (label-carry, Slice 5). | UNFLAGGED as a pattern |
| X-4 | The contract names the "session skill", "DESIGN.md" and ".vsdd/state.yaml" as load-bearing artifacts; none of the three exists in this repo. | UNFLAGGED |

