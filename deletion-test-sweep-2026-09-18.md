---
title: "Deletion-Test sweep of the contract + build-plan: durable / register / delete / consolidate (2026-09-18)"
tags: ["design-input", "observability"]
sources:
  - url: "http://oreilly.com/catalog/errata.csp?isbn=9781098179922"
    title: ""
    accessed_at: "2026-09-19"
contributors: ["xqjG"]
created: 2026-09-19
updated: 2026-09-19
---






# Deletion-Test sweep of the contract and build-plan (2026-09-18, vsdd-cli #869)

**Method.** Three tests from *Observability Engineering* 2e applied by hand to every contract member, requirement, acceptance criterion, verification-architecture member, decomposition paragraph, open question, and build-plan phase: (1) **the Deletion Test** (Ch32/Fowler) — *if we deleted this, what named failure would we stop catching, and has it ever occurred?*; (2) **activities vs learning** (Ch27) — *can it name the feedback mechanism that turns it into learning, and is that mechanism used?*; (3) **sediment** (Ch32) — *is this prose carrying history that git and the tracker already carry?* Four bins: **DURABLE** (named failure with recorded evidence — keep as contract), **REGISTER** (a named risk whose control cannot be built yet, or whose occurrence has never happened — a deviation/escape-registry entry with retest trigger + expiry, not contract prose), **DELETE** (superseded, stale, duplicated, or evidence-free), **CONSOLIDATE** (one thing stated in several homes). These are proposals; the Solution Owner rules. Anything the contract backs with an *Evidence:* record I binned DURABLE by construction — the sweep does not second-guess recorded incidents.

**Headline.** The contract's *spine* passes the Deletion Test cleanly: twelve behavioral contracts, each with a recorded incident behind it. The cruft is not in the commitments; it is in (a) **speculative defense enumerations** authored from cold-review hypotheticals rather than occurrences (the fix-scale gate lanes, the ~90-member fixture corpus, the 23-REQ subsystem's unbuildable half), (b) **sediment** (superseded clauses kept inline, ratification markers repeated dozens of times, migration provenance, stale operational to-dos, source-line citations into another repo at a commit that has moved), and (c) **boilerplate obligations that have never once produced learning** (per-slice "owes at 2a" lists, manual-test checklists). Rough estimate, not a measurement: a contract that keeps every DURABLE item verbatim and moves the rest to the registry/tracker/revision history lands at **40–50% of today's size** — and every later slice's phase-1a stops inheriting the sediment.

## Bin 1 — DURABLE (keep as contract; evidence recorded)
- All twelve behavioral contracts' normative cores: Deterministic phase answer (June position loss), Directive reconciliation (137 orphaned findings), Phase exit by gate — the layer-scale red gate and the fix-lane principle (layer-7 cheat; mdatron hotfix), Finding lifecycle in one place (dual-home drift; 7/8 absent citations), Deterministic composition (composition-as-prose), Recorded review dispatch (18 reviews filed into a sibling repo; 23 empty issues; the live fire), Conformance at action time (13 hand hooks; hollow shell; letter-cluster labels ×4), the Verifiable-conformance **law** and its oracle boundary (#821 personas unread; the paraphrase bypass), Cost is knowable (fabricated figures), The operator authors the oracle (the ID-reuse invariant caught by a manual checklist after 11 cold reviews missed it), Solution Owner change authority (the rule that outlived its only home), the PR-required ruleset leg of Per-milestone PR discipline.
- The enforcement-ladder / three-grades doctrine and the engine/slice structure in Decomposition.
- Of the subsystem's REQs: 1, 2/3/4 (as targets, honestly graded), 5, 7, 9, 10 (principle), 12, 14, 16, 17, 19, 21, 22, 23-leg-1.
- Fixtures that reproduce **recorded** incidents: layer-7 red-gate cheat, the inline-review (mdatron) fixture, hollow-shell tree, the un-owned and owned-but-ungated hotfix pair, postdated-approval, never-started (both signatures), the unrouted-findings pair, the drifted pin, seeded coinage.
- **Durable but authored-not-exercised (make real once, or register with expiry):** the operator manual-test checklist obligation (its one incident is exactly why it exists — yet Layers 1–2 never got one and `manual-tests/layer-*.md` are stubs on retired vocabulary); Dependency approval (the lapse — icu_properties landed with no record — is still standing).

## Bin 2 — REGISTER (out of contract prose; into the deviation/escape registry or the tracker with trigger + expiry)
- **The fix-scale gate's speculative lanes** (Phase exit by gate, one ~1,500-word bullet): set-valued mappings, decoy mappings, bulk-mapping set-equality, cfg-vanished degradation, relocated-dormancy composition, split-invocation stamps, in-place quarantine, the declared-compile-defect fork. None has an occurrence; all came from adversarial review rounds 2026-07-19/21. The contract itself names the failure mode ("illustrative examples were wired as enumerated special cases", Directive reconciliation evidence). Proposal: the fix-lane principle stays (red at baseline, green at HEAD, executed pin, validator-approved exceptions, no bypass before publish); each speculative cheat becomes an **escape-corpus entry** (REQ-22's own rule: "a probe-found escape is added as it is found") with the control sketched and retest = first occurrence. Slice 4's ~40-member fix-lane fixture corpus shrinks accordingly.
- **The Fixture corpus requirement's one-sided / "deferred, never covered" members** (roughly two-thirds of ~90): register the class, author the fixture when the falsifier's occasion first arises. The contract's own hedging ("stated here so the one-sided members read as deferred, never as covered") is the tell.
- **Subsystem REQs resting on unbuilt capabilities**: REQ-10's synced-transcript channel and REQ-23 leg 2 (upstream crosslink asks — unfiled, see impact E1), REQ-21 leg 2's agent-count interceptor and AC-27's outer-harness deny hook (outer-harness dependencies). The *principle* stays in the contract; the *dependency* is a registry entry with a retest trigger, not a REQ paragraph.
- **REQ-6 (cache warm-handoff proxy) and REQ-8 (offset/limit reads)**: no incident; fold as advisory dimensions of REQ-14 rather than standalone REQs.
- **REQ-20's underutilization list** (Cron/PushNotification/RemoteTrigger, Sentinel, Intervene, Viewers, the milestone detector, find-or-create): a to-do list wearing requirement clothes; the hand-roll rule already lives in Conformance at action time. Each unused affordance → a registry-style note with retest = "a need recurs", or nothing.
- **Per-milestone PR discipline's unexercised legs**: VSDD-E0090 (gates on checklists that don't exist), the bypass-marker gate (authored, never wired), the co-authorship trailers (self-reported, unverifiable — #657 open). Keep the enforced ruleset leg; register the rest.
- **Swarm live fire** — already deferred under two registry entries; the contract text can shrink to the criterion + the entry ids.

## Bin 3 — DELETE (sediment, superseded, stale, evidence-free)
- **Superseded clauses kept inline**: the attended/autonomous split bullet in Recorded review dispatch and its 300-word "Superseded by…" gloss; the Methodology rewrite requirement and bookend (a rewrite target for a file deleted in #826, "retained verbatim pending ruling"); the workspace-shape paragraph's bracketed superseded cost-crate justification; the Architecture sketch's events-store retirement narrative ("the store held one hand-written file from its first day…"); the "Retired alongside DESIGN-OBSERVABILITY.md: the OTel collector…" catalogue in Cost is knowable. Git has all of it.
- **Ratification markers**: "(vsdd-cli #840, ratified 2026-08-02 under vsdd-cli #860)" appears ~20 times; "(the #845 remediation, REQ-n clause (x))" a dozen times; "Migrated here from … on the pattern the Dependency approval migration used" ×3. One revision-history section replaces all of them.
- **Resolved open questions still under "Open questions"**: Phase-state location (resolved 1b), Cost-engine packaging (resolved #860). Keep DSL scope and Swarm fallback.
- **Stale operational to-dos in the Decomposition bookends**: the Estate-cleanup enumeration (bulk-close 124 issues, retire vsdd-cli-wip, delete `review-log 2/`, sweep old URLs — 2026-07 tasks; tracker items, not contract); the mdatron upgrade hazard "v0.2.0 ready, v0.3.0 imminent" (mdatron is cutting 0.6.0; the `mdatron-sibling-pin-lag` entry is already resolved); the "What is built" paragraph asserting `acquire` hardcodes empty "verified 2026-07-28" (Slice 1 replaced it — the contract now describes a state that no longer exists).
- **Source-line citations into the crosslink mirror** (Design-doc lifecycle member: `pipeline.rs:490-491`, `design_doc.rs:16-29`, at 6b4f736f) — the mirror is at fe653b40; these rot by construction. Keep the three-sentence rule (sidecars agent-writable at birth; two sanctioned tool ops; SO disposition is the amendment record) and move the archaeology to the existing `crosslink-design-flow-facts` knowledge page.
- **Closed acceptance criteria carried in full prose** (Convergence test; the closed slices of Status detection and Install behaviors — together ~12KB describing built, tested statusline/offer conduct): collapse each to its leading name + closing record + fixture path. The fixtures are the durable record ("code becomes cache"); the prose restating them is the cache that went stale.
- **Data-authoring package list**: most of it is already authored (13 files in `templates/registry/`); replace the enumeration with "the versioned data in `templates/registry/`; unauthored: …" or move to the build-plan.
- The "Contract-integration path" paragraph and the criteria-audit prose (executed; history).

## Bin 4 — CONSOLIDATE (one thing, several homes)
- The **domain-roster partition** (REQ-18) is stated in Deterministic composition, in the subsystem member, and on the `verifiable-conformance-and-efficiency` page. One home: the contract member.
- The **23 REQs** live in the contract fold (§142–158), the 68KB knowledge page, and the REQ→home table. Proposal: the contract carries the law + the DURABLE REQs in one compact list; the page becomes the archived design rationale; the table lives in the build-plan.
- **Per-slice obligation boilerplate** ("owes at 2a its red-gate seed, its fixture-corpus slice, the operator-authored manual-test checklist, the fresh-container re-run; wires the terminal-output-safety cleaner…") is repeated 7× in Decomposition and 5× in the build-plan. State once as "slice obligations"; each slice references it.
- **Provenance annotations** → one revision-history section (see Bin 3).

## Defects the sweep found (route as findings, not opinions)
1. **Build-plan Phase 2 contradicts the ratified contract**: it carries "pricing-function home (a minimal cost crate now versus provisional in vsdd-core — recommendation: the crate…)" while the contract's resolved question rules *engine in vsdd-core, no separate crate* (#860). Contract-over-derived: the projection is wrong and would mislead #839's design.
2. **Stale current-state claim**: Decomposition's "Today `acquire` hardcodes findings… empty (verified 2026-07-28)" — superseded by Slice 1 (PR #24).
3. **Stale cross-repo cadence**: "v0.2.0 ready, v0.3.0 imminent" — five releases behind.
4. **Rotting citations**: crosslink source lines cited at 6b4f736f; mirror at fe653b40.
5. **Dependency-approval lapse still standing** (icu_properties, #813) — the contract records the lapse and no remediation issue exists (search before filing).
6. REQ-21 leg 2 inaccurate on the target vehicle (impact analysis C3).

## The shape of a compacted contract (proposal)
Twelve behavioral contracts, each ≤ one screen: normative bullets, falsification, one-line evidence pointer (incident record handle). Eight requirements (Install, Status, Terminal output safety, Composition function, Conformance checks, Gates, Dependency approval — simplified to additions-need-the-record, Cost); Reviewer roles / Generated context / Directive flow / Waiver enumeration fold into their contracts as they are already stated there. Acceptance criteria: open ones in full, closed ones as name + record. Fixture corpus: "recorded incidents + ratified cheats; everything else enters through the escape corpus as found." Verification architecture unchanged in substance, minus archaeology. Decomposition: the doctrine + the slice structure + one obligations paragraph; bookends move to the tracker. Open questions: two. A revision-history section absorbs every marker. The deviation registry and a new escape-corpus file absorb Bin 2.

## Process and sequencing
This is the largest spec amendment the contract has had; it is Solution-Owner-owned and goes through the owned process (owning-domain composition + cold review + SO ratification; PR; the Decomposition re-pin in the build-plan). Recommendation: **run the compaction before Slice 2's phase-1a opens** — #839 would otherwise inherit the sediment and the Phase-2 contradiction above — and fold the pending governed-corpus edits (impact C2/C3: the Phase-3 primer and the REQ-21 correction) into the same cycle so the primer *shrinks* with the contract instead of growing beside it. Anti-accretion guard for the cycle itself: the compaction adds no new normative statement; its diff is deletions, moves, and the revision-history section.

## Decision slots (SO)
1. Ratify the bins as the compaction's scope, with any DURABLE/REGISTER re-assignments you want (the fix-scale lanes and the fixture corpus are the two where reasonable people differ).
2. Authorize the compaction cycle and its sequencing (before #839's 1a), folding C2/C3 in.
3. File defects 1–5 as tracker issues (defect 1 is a build-plan correction under the Design-doc lifecycle rules).



## Addendum (operator, 2026-09-18): two more cruft classes, and what the book says about them

**E. Tool-specific leakage.** The reason crosslink is the tracking + orchestration target is *agentic-coding-tool agnosticism*: those parts must work with any agent runtime. Contract prose that carries Claude Code specifics — transcript file names (`agent-<id>.jsonl`), `attributionSkill`, `cache_creation_input_tokens`, the `Task`/`Agent` primitive, the `PreToolUse` deny hook, `hasTrustDialogAccepted`, effort frontmatter, `~/.claude` paths — is harness detail leaking into the tool-agnostic layer. The contract already assigns that catalog to the `claude-code-cli` supplement; the contract should say only the abstract noun (the runtime harness's run record; its spawn primitive; its consent surfaces). Bin: MOVE to the supplement. Book: Ch17 — the ontology names nouns abstractly and the emitter-specific fields are the convention layer beneath; Ch10 — one canonical name, per-harness aliases in the map.

**F. Doctrine nicknames, precedent citations, and cross-source handles.** 207 raw `#NNN` handles from seven sources; rules referred to by nickname — *the phases-dispatched keystone, the derived-view ruling, the tier rule, the #845 remediation, format-carry, the corroboration keystone, invariant-first, enforcement-spine-first* — none defined as terms in the name map, which registers repo names and ~15 draft terms and explicitly calls two of these 'descriptive metaphors, not registered terms' while the body uses them normatively. Multi-word coinages evade the vocabulary check. The book's guidance, seven threads:
1. **Ch17 semantic failure**: 'the same concept… labeled in multiple ways… nobody is quite sure which version of the truth is the real truth.' The fix is codification — 'an ontology remains an academic exercise until it is codified': a rule is stated ONCE at a stable anchor, its heading IS its name; a nickname is an alias in the map or is retired.
2. **Ch10 naming chaos**: 'names accumulate rather than converge… an agent without a canonical map of that nomenclature will confidently draw conclusions from incomplete or misattributed data' — so will a human reader. Remedy: a canonical map of names, aliases, owners.
3. **Ch7 conflicting keys**: 'dozens of conflicting attribute keys for the same logical value… models can't reliably infer which field is canonical.' Remedy: ONE handle grammar (namespace) — `vsdd-cli#845`, `mdatron#51`, `crosslink#62` (upstream), `crosslink-fork#3` — registered as data (the branch-grammar precedent); and prose never carries a bare handle: it cites the rule by heading name, the handle lives in the section's Evidence line and the revision history.
4. **Ch17 lineage as metadata**: 'glue the schema through metadata… embed lineage directly.' Ratification markers ('(vsdd-cli #840, ratified 2026-08-02 under #860)' ×20) are lineage — a per-section revision field or one revision-history section, never inline prose.
5. **Ch21/Ch8 imputed meaning** (`is_slow`): a named thing without a description at its anchor gets its meaning imputed, confidently and wrong. Every doctrine name gets a one-line definition or is retired; multi-word coinage detection is a generic mdatron primitive worth raising (through the three-question boundary procedure).
6. **Ch32 precious knowledge**: 'code becomes precious when it is the only place knowledge lives.' A precedent citation ('Thermite's doc-drift decision 5', 'the upstream waiver precedent', 'the mdatron kickoff live fire') is precious because the RULE lives only in the incident narrative. Unbundle: the normative bullet states the rule; the Evidence line cites the record by handle; the narrative lives in the record (tracker / knowledge page), never in the contract.
7. **Ch25 noisy labels**: 'the landscape feels noisy because the labels are noisy… start with the feedback loops you need to strengthen.' Organize the contract by invariant and loop, not by the history of how each rule arrived.

**Proposed conventions for the compaction cycle** (its organizing principle, not additional doctrine): (i) one registered handle grammar for cross-repo references; (ii) prose cites rules by heading name, never bare handles — handles in Evidence lines + revision history; (iii) every doctrine nickname becomes a heading (and is thereby defined) or is retired; the name map lists surviving aliases; (iv) precedents move to Evidence lines; (v) harness-specific fields move to the runtime-harness supplement. Decision slot 4 added: adopt these five conventions as the compaction's rules.



## Addendum 2 (operator, 2026-09-18): align to a common body of knowledge — the coinage → standard-term map

Operator rule: no coinages, vocabulary, or labels that obfuscate; this compaction is the chance to align to a common body of knowledge. Reference lexicons, in the estate's grounding order (AI-Eng > Platform-Eng > SWE/QE): **OpenTelemetry semantic conventions** (incl. the GenAI conventions), **SRE** (SLI/SLO/baseline/error budget), **FinOps** (unit economics, rightsizing, allocation), **internal-controls audit** (design vs operating effectiveness, detective/preventive/compensating controls, risk register, exception), **QE/testing** (expected/actual, oracle, characterization test, red-green, false positive, regression corpus, reconciliation), **ITIL change management** (change classification, documented exception), **ADRs** (decision records), **QMS document control** (controlled documents, deviation/nonconformance), **SWE** (walking skeleton, vertical slice, drift, bill of materials). Each row is a proposal; where no standard term exists the project term stays and is defined once in the registry. Rows marked ★ change the reader's model the most.

| vsdd coinage / nickname | Common term (lexicon) |
|---|---|
| run record / run transcript / harness transcript ★ | **trace** — a run is a trace, each agent a span, usage as span attributes (OTel; GenAI semconv `gen_ai.usage.*`) |
| WAS ⊇ SHOULD ★ | **actual ⊇ expected** — observed context set vs expected context set (QE) |
| capture-source provenance: recorded / measured / judgment / could-not-check ★ | **provenance: observed / derived / estimated / no-data** (data lineage; SRE 'no data ≠ pass') |
| phases-dispatched keystone ★ | the **delegation policy**: all phase work runs in dispatched agents; the operator session only orchestrates (no nickname) |
| authored is not exercised; exercise registry ★ | **design effectiveness vs operating effectiveness**; **control effectiveness testing** (internal-controls audit) |
| format-carry (bootstrap discipline carried as a comment format) ★ | **compensating control** (audit) |
| deviation registry; retest trigger + expiry ★ | **risk / exception register** with **review date** and **exit criteria** (GRC) |
| escape corpus | **regression corpus** built from real escapes (QE; Ch17/21 'production failures become the next tests') |
| disposition: hallucinated / dismissed / accepted / deferred / resolved ★ | **false positive / won't fix / accepted risk / deferred / fixed** (bug triage) |
| guardrail grades: detection / friction / CI-backed block / harness-level capability restriction ★ | **detective control / bypassable preventive control / enforced preventive control (required status check) / least privilege** (security controls) |
| self-governance guardrail vs adopter guardrail | **dogfooded control** vs **shipped control** |
| invariant-first; enforcement-spine-first | **walking skeleton** (build the enforcement path end-to-end first — Cockburn); ordering principles, not terms |
| the engine | **core library** (vsdd-core) |
| red gate / red-gate seed / pin test / executed pin | **red-green** (TDD); **characterization test** (Feathers; Ch32); 'the test ran, not skipped' |
| round-parity | **reconciliation** (counts agree between two systems) |
| dispatch preflight: pass / fail / inconclusive | **preflight check: pass / fail / unknown**, fail-closed |
| dispatch-failed: never-started / started-then-stalled | **launch failure / heartbeat timeout** (liveness) |
| dispatch manifest; expected band | **dispatch plan** (plain) with a **budget** and **baseline** (SRE baseline window) |
| static price / priced bill of materials | **token budget / cost estimate**; bill of materials is already standard |
| efficiency insight engine; efficiency advisories; effort-scaling signals | **cost and efficiency report**; **rightsizing signals** (FinOps); **unit economics** (already standard) |
| calibration band | **baseline** (SRE) |
| the derived-view ruling | 'events are **derived at query time**; no separate event store' (Ch25 unified storage) — a policy sentence, no nickname |
| the tier rule (schema pass = shape fact) | **syntactic validation vs semantic verification** (QE) |
| corroboration keystone / tamper-evidence-by-corroboration | **tamper-evident audit log** + **independent verification** (security) |
| directive reconciliation; recorded override; increment entry | **change classification** (ITIL); **documented exception**; **new work item** |
| SO disposition / operator ruling | **decision record** (ADR) |
| governed corpus / governed files; route table | **controlled documents** (QMS document control); **document classification / allowlist** |
| maturity lifecycle: draft / established; first publish | **stability guarantee: unstable / stable** (SemVer; Rust stable/unstable) |
| swarm live fire | **pilot / production acceptance trial** |
| cold review; sycophancy compensation | **independent review** (fresh context; reviewer independence) |
| phase answer | **pipeline stage / current phase** (plain) |
| terminal output safety | **output sanitization** (security) |
| installed-artifact manifest; hollow shell | **install manifest** (SBOM-shaped); **broken install** |
| act-to-affordance map | **tool map / workflow bindings** ('affordance' is HCI jargon) |
| retrieval-shaped artifacts | **retrieval-friendly** (plain) |
| composition / composed domains | keep — plain English; 'composed context' → **injected context** (Ch10) |
| surfaces (project declarations) | candidate: **project attributes / declared capabilities**; genuinely project-specific — define once if kept |
| finding, oracle, waiver, drift, vertical slice, unit economics, bill of materials, SLI/SLO | already standard — keep |

**Naming rules for the compaction** (add to the five conventions): every term in the contract either (a) is a standard term from a named lexicon, used in its standard sense, or (b) is a project term registered once in `vocabulary.yaml` with a one-line definition and its nearest standard neighbor named; nicknames for rules (keystone, ruling, remediation, format-carry, tier rule) are retired in favor of the rule's heading. The vocabulary check should be raised to catch multi-word coinages (mdatron generic-primitive raise via the syntactic-vs-semantic boundary procedure). Decision slot 5: adopt the map (with operator edits) as the compaction's naming rule.



## Rulings log
- **Decision 1 — RULED 2026-09-18 (SO):** bins ratified. Fix-lane: MIDDLE PATH (principles stay as ~6 bullets; mechanics → regression corpus). Fixture corpus: REGISTER (incident-backed + ratified cheats stay; the rest enter the regression corpus as found). Manual-test checklist: MAKE IT REAL (author now for what is built; obligation then scoped to operator-facing slices). Dependency approval: SIMPLIFY (added dependency → record + three lenses; graded machinery dropped; icu_properties lapse filed as a defect). Recorded as a decision on vsdd-cli #869.


- **Decision 2 — RULED 2026-09-18 (SO): compaction cycle AUTHORIZED**, scoped as recorded on #869; anti-accretion guard = deletions/moves/renames/revision-history + exactly two new sentences; author in-session hand-audited, 2 pair-separated cold reviewers under the fallback exception (container vehicle down), 200k hard ceiling; operator ratifies a change map; sequenced before #839's phase-1a.


- **Decisions 3/4/5 — RULED 2026-09-18 (SO):** file exactly three findings (build-plan crate contradiction; REQ-21 leg 2; icu_properties dependency-approval lapse); all five reference conventions ADOPTED with the handle grammar vsdd-cli#NNN / mdatron#NNN / crosslink#NNN (upstream) / crosslink-fork#N (mirror) + named non-tracker records; naming map ADOPTED, with 'surfaces' → 'project declarations' + 'activation criteria' and 'affordance' → 'paved path' (map → paved-path map). Recorded on vsdd-cli #869.
