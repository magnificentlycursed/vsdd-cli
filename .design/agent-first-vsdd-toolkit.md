# Feature: Agent-first VSDD toolkit on the crosslink chassis

Crosslink issue: "Respec: agent-first VSDD toolkit on the crosslink chassis" (vsdd-cli tracker; provisional local ID pending hub sync — reference this issue by title) · Phase: 1a (behavioral specification), skill-interactive
Supersedes on ratification: DESIGN-METHODOLOGY.md, DESIGN-SCHEMA.md, DESIGN-OBSERVABILITY.md, DESIGN-VERIFICATION.md, docs/refactor/binary-first-plan.md (its phases rescoped)
Evidence base: the domain-value-scorecard and thermite-assessment knowledge pages (crosslink knowledge store, this repo), the VSDD/VDD whitepapers, the crosslink governing docs, the Thermite development harness, and the operator interview of 2026-07-18. See References and name map for every artifact cited in this document.
Revision: spec-review round 1 applied (44 findings, 4 cold lenses; dispositions filed as child issues of the tracking issue). Amended 2026-07-19 post-ratification: citation verification basis changed from HEAD to the working tree (operator ruling, recorded on this tracker and the mdatron increment issue).

## Summary

The toolkit enables and enforces VSDD for projects whose work is executed by AI agents under a human operator's direction. The product is an installed environment: context, constraints, and hooks wired into crosslink's seams, in which following the methodology is the easiest available path and deviation produces immediate, compiler-shaped feedback. The operator's session-start intent expands deterministically because the process is deterministic: the same project state and process spec produce the same next action from any competent cold agent.

Division of labor:
- **crosslink** (chassis): issues, dispatch (swarm), worktrees, gates, sessions, knowledge, and raw usage capture (harvest, context measurement). The contract chain lives here.
- **vsdd** (methodology layer): process spec, phase state, composition, domain prompts, generated agent context, hooks, and the cost ledger with its insight queries. Installed into crosslink's seams (rules, skills, hook config, house style).
- **mdatron** (conformance engine): validates every methodology artifact, including the phase state file and route table, invoked at action time by hooks and at boundary time by gates. Methodology-agnostic per its boundary preamble (see References).
- **operator**: doctrine, scope, priorities, approval, and oracle authorship (acceptance criteria and manual tests). All other decisions belong to the process or the tooling.

## Project declarations

Intent tiers are retired (operator decision 2026-07-18: the tier system was cut from vsdd-suite in practice; reviewers were mixed and matched ad hoc, and the tiers hid that fact). In their place: a review configuration, preset-seeded and operator-customized (see Deterministic composition).

This project's declarations:
**Review config:** thorough preset — the toolkit governs other projects, so its defects propagate. The full default domain set plus the mutation floor; declared in `.vsdd/config.yaml` on adoption.
**Phase 5 strategy:** mutation testing on vsdd-core and mdatron-core with a kill-ratio floor enforced in CI; fuzz testing on mdatron's frontmatter and DSL parsers (trust boundary: untrusted markdown); no proof execution, since no cryptographic or safety-critical surface exists. Re-evaluate if the Thermite language becomes viable for these crates.
**Phase 6 strategy:** applicable at the v1.0 cut — before crates.io publish, confirm that specification, tests, implementation, and verification each stand complete and consistent with one another (the whitepaper's Phase 6 convergence).

## Behavioral contracts

Contracts are referenced by their exact heading names. Each contract states its normative content, then its falsification path, then the evidence that motivated it.

### Deterministic phase answer

"What phase are we in?" has exactly one correct answer, derivable by any cold agent at any time.

- The phase state is a written artifact in the project repo. The draft location is `.vsdd/state.yaml`; the phase-state-location open question decides the final store in Phase 1b. The contract's mechanics below hold for whichever store is chosen. Contents: current layer, current phase, open-findings pointer, last gate result, active composition.
- The state advances only at phase boundaries, by the agent, in the same commit as the boundary evidence. mdatron validates the schema and the consistency of state against evidence: a state claiming phase 2b requires a recorded red-gate result for the layer.
- The state rides the chassis session machinery: the phase pointer travels in crosslink session breadcrumbs and is re-injected on resume after compaction; `session start` surfaces phase status with the prior handoff; phase boundaries require handoff notes; each phase execution and each layer is a crosslink milestone, so tracker-side progress and the state corroborate each other and the phase-versus-milestone integrity query is well-defined from phase 1a onward.
- When the tracker is unreachable, the phase answer is computed from the state artifact alone and reported as degraded. Concurrency, mid-phase crash recovery, state-versus-tracker precedence, and bootstrap validation are decision criteria of the phase-state-location open question.

Falsification: two cold sessions given the same repo and tracker state answer differently (a spec defect, chargeable to the spec rather than the agents); a state artifact inconsistent with repo evidence surviving an mdatron check.

Evidence: session entry has depended on prose reconstruction since the suite era; two sessions could disagree about the project's position, and in the June cycles the flow's position was lost entirely (June-cycle records and the operator interview, in References).

### Directive reconciliation

Every mid-flow operator directive is classified before execution. The classifications: phase-4 routing (bind it — file the routing, name the target phase), spec amendment (amend the design doc, then act), or recorded override (execute, record via `crosslink intervene`, state where the flow stands and the re-entry path).

- The classification is stated to the operator at the moment of receipt. The statement includes the inferred scope: whether the operator's words are read as a specific instruction or as instances of a general rule.
- Silent execution of an unclassified directive is a named defect. The operator holds the authority to redirect; the agent holds the duty to say what the redirect does to the flow.
- Reconciliation includes a substrate step drawn from the AI Engineer domain: when a chassis or harness mechanism serves the directive (milestone, session, hook, skill, config), it is named and offered at classification time. The AI Engineer domain also owns the recurrence-to-structure duty: the second occurrence of an operator correction or struggle obliges a proposed mechanization.

Falsification: a directive executed with no classification recorded in the tracker or session record; a recorded classification lacking the scope reading; a directive served by an available chassis mechanism with no mechanism named at classification; a second-occurrence correction with no proposed mechanization.

Evidence: the operator issued directives in the June cycles believing them to be phase-4 routing; the agent executed them without classification, the flow was orphaned, and 137 findings accumulated with 82 of them orphaned before the operator noticed. Scope drift has failed in both directions: single utterances became named schema concepts, and illustrative examples were wired as enumerated special cases (June-cycle records and the operator interview, in References).

### Phase exit by gate

A phase closes only when its exit gate passes. Gates are commands; `crosslink swarm gate` runs them, and their results are recorded in the state artifact and the tracker.

- Red Gate, mechanized: phase 2a closes only when the layer's test suite fails against the pre-implementation commit and the failing run is recorded. Phase 2b closes only when the same suite passes at HEAD. The gate performs the checkout and run; agent assertion plays no part.
- Phase 3 closes only at the stop signal (a round producing only hallucinated findings) with all round findings filed. Phase 4 closes only when the unrouted-findings query returns empty.
- Mutation floor: when the review config declares one (the thorough preset does), the mutation kill-ratio on changed code is a standing gate criterion.

Falsification: any phase transition in the state artifact without its recorded gate evidence surviving an mdatron check.

Evidence: the issue-tracker-cli project's layer-7 Red Gate passed against the pre-implementation base with zero genuinely failing tests, and the tracking issue for the last plan phase carries three closure comments while remaining open — both were assertion-based transitions that a gate would have blocked (see References for both records).

### Finding lifecycle in one place

Finding lifecycle state — classification, routing, closure — lives in crosslink issues only. Review narrative may exist but carries no state.

- Findings are filed in the subject project's tracker; the subject project field is a routing instruction the tooling enforces.
- Filing is evidence-gated: a finding carries checkable evidence at filing time, either a file-and-line citation that mdatron verifies against the working tree as it stands (uncommitted edits count) or a failing test where the surface allows one. A finding citing absent content is rejected at filing. (Amended 2026-07-19 by operator ruling during the mdatron increment: verification basis changed from HEAD to the working tree — the intent is that cited code exists in the agent's reality at action time, and the change removes any git dependency from the check.)
- Every finding names an owner domain and a validator domain that differs from the owner; sanity-check is the validator of last resort. The halves of an adversarial pair — an author-side domain and its cold-reader domain — never share a reviewer session.
- Lifecycle acts use crosslink's typed comments as the single vocabulary: closures are `result` comments carrying the evidence reference, operator rulings are `decision`, routing is `plan`.
- Closure is evidence-graded: a closing comment references a commit, gate run, or test; otherwise the closure is invalid. Dismissed and hallucinated findings close immediately with their disposition; git and tracker history are the audit trail, so closed records need no open placeholder.
- Waivers are enumerable: any waived criterion, deferred obligation, or accepted risk carries a single canonical marker that one search lists completely.
- Tracker objects are cited by lookup handle wherever they are referenced: a repo-qualified issue ID once hub sync has assigned it (`vsdd-cli #15`), the exact issue title before that, milestone names as recorded, and knowledge-page slugs. Records predating the chassis are cited by file path. A reference that cannot be resolved by a crosslink query or a path is a defect the conformance checks flag.

Falsification: lifecycle state found in a review-log file that differs from the tracker; a closed finding without an evidence reference; an open finding without an owner; a filed finding whose citation fails working-tree verification.

Evidence: classification state lived in both review-log files and tracker issues during the June cycles with no synchronization, and drifted; 7 of 8 findings in one bookmark-cli review round cited code absent from HEAD; 137 findings sat unrouted until the operator intervened (records in References).

### Deterministic composition

Given the phase, the project's declared surfaces in DESIGN.md, and the project's review config, the active domains and their dispatch shape are computed.

- The review config is a file (`.vsdd/config.yaml`) with layered loading modeled on crosslink's config: shipped presets, then project config, then local overrides. Presets are suggested starting points, each with a priced bill of materials. The operator customizes freely; the customization is a declared, git-tracked, schema-validated input. Fields: active domains, round budget, stop sensitivity, model tiers, mutation floor, cost bands.
- Config integrity rules are validated by mdatron: pair co-activation (activating an author-side domain activates its cold-reader domain), pair separation at dispatch, and validator-differs-from-owner. The per-domain bar holds at any active-set size.
- A mid-cycle config change is an operator directive and receives the directive-reconciliation treatment with a recorded event.
- Identical inputs produce identical compositions. The rules above carry the doctrine; presets are starting points.
- Composition operates at two timescales. Session composition selects domains per phase. Action-time activation binds content to its moment mechanically: every supplement declares its activation trigger as a required field of the supplement artifact class, and the harness surfaces whichever supplements match the task at hand, with a hook backstop when a file is edited before its matching supplement was read. Generality is structural — adding a supplement means content plus trigger, with no harness wiring. The chassis supplement's always-relevant core rides in the generated always-on session context.

Falsification: two sessions computing different compositions from identical inputs; an emitted context artifact differing from its source data; a config violating an integrity rule surviving validation.

Evidence: the composition matrix lived as prose that agents applied from memory; the tier system was cut in practice while reviews ran at unrecorded ad-hoc compositions; the crosslink-usage knowledge sat in a review-time supplement while the operator believed crosslink was in use and no mechanism was positioned to notice (operator interview; June-cycle records).

### Recorded review dispatch

Phase 3 dispatch runs on crosslink swarm. Every dispatch produces a manifest: what was sent, to which reviewer role, containing which inputs by content hash, under which composition, what came back, and where findings were filed.

- Reviewer roles are tool-restricted; a critic role cannot edit, and approval never belongs to an agent. A domain prompt is criteria content; a reviewer role is the tool-restricted agent identity that receives domain prompts at dispatch — the two are distinct, and pairing constraints bind domains while session constraints bind roles.
- Dispatch manifests and filed findings are signed with crosslink agent identities, which both repos already carry. Who found what, under which dispatch, is then a verifiable record.
- Build dispatches carry scope manifests on the same pattern: a pre-declared writable file set per task. An off-manifest write blocks with a manifest-expansion request. One concern per manifest keeps commits revertible.
- Coverage is declared: the dispatch manifest records which domains did not run and why (no matching surface, declined by config, or deferred with a trigger). Absence of findings and absence of looking are distinguishable facts, and neither produces tracker issues.
- Round stop rules follow the VSDD whitepaper: continue on any real finding; stop when a round is entirely hallucinated; rounds past the stop signal require named new evidence.
- The review budget follows the evidence: cold review concentrates on code-shaped artifacts; prose artifacts get the operator's read and external cold readers; spec-stage structural review — small, early, few lenses — is a first-class use of cold review.

Falsification: a Phase 3 record without a manifest; findings whose provenance traces to no dispatch; a build commit touching files outside its declared manifest; a domain that neither ran nor recorded a reason; an unsigned manifest or finding where signing is configured.

Evidence: 18 reviews of one repo were filed into a sibling repo's review-log; a bundled commit had no rollback path; 23 empty no-findings issues were filed to prove domains had looked; this session's own tracker events ran unsigned under audit-mode enforcement (records in References).

### Conformance at action time

mdatron validates methodology artifacts — the state artifact, DESIGN.md structure, the route table, domain prompts, generated context — with rustc-shaped diagnostics, invoked by hooks at action time and by gates at boundary time.

- Spec drift is checked mechanically: design docs pin a content hash over the files they govern, and a governed-file change without a re-pin fails the gate (Thermite's doc-drift check, generalized).
- Session entry is enforced: edits to governed files are blocked until the session has read the governing docs (Thermite's spec-discipline hook, generalized), configured per project.
- Vocabulary and register are governed. A register spec (modeled on Thermite's tone-and-voice document: concrete named anti-patterns, with the rationale that agents anchor on the register they read, so corpus tics reproduce) pairs with the vocabulary registry (see References for its location, current and planned). Term additions carry earned-by-recurrence justification. mdatron checks the mechanical subset at action time: invented label schemes, unregistered abbreviations, new-coinage detection, and the listed register anti-patterns.
- Vocabulary and schemas have a maturity lifecycle. Draft terms and schemas may be renamed or deleted freely, carry zero compatibility obligation, and are exempt from strict consistency checks and findings. Promotion to established is an explicit operator act, gated at first publish, and never agent-initiated. An utterance is a concept only after registration: agents do not reify casual operator phrasing into named terms, frontmatter fields, error codes, or schema elements. Forward-only discipline applies to established artifacts only.
- The artifact set is closed-world: the route table is an allowlist, and an unrouted artifact blocks (Thermite's no-route rule, generalized). New artifact classes and new name slugs are registration acts under the same maturity gate as vocabulary. Generated artifacts carry computed names derived from their manifest; agents never hand-name what the tooling can name.
- Enforcement logic lives in mdatron pattern and registry data. Hook scripts stay thin wrappers; a new defect class is closed by adding a pattern, which is a diffable artifact under the maturity gate.

Falsification: a governed file changed with a stale pin passing the gate; a governed edit absent from the hook trace log (every governed edit leaves a trace, and a gap between the trace log and git history is itself a finding); a file admitted with no route; an artifact whose name is underivable from its manifest or registered grammar; a coinage or register violation in a governed artifact surviving the check.

Evidence: the letter-cluster label pattern recurred four times in the suite era past written correction, and twice in this design session past a loaded memory; single-utterance reification plus forward-only policy produced backwards-compatibility work on schemas still being designed; documentation-type proliferation routed around per-file validation through free filename slugs; the suite accumulated 13 hand-authored hooks, one per escape axis, each written after the recurrence (records in References).

### Cost is knowable

Every methodology operation has a measurable cost, a breakdown, and a baseline, answerable as queries. The canonical questions — is 100k tokens a lot, what makes it up, how much was cached, would a rewrite reduce it — each have a recorded-evidence answer.

- Cost is denominated in native units: tokens by cache class, wall-clock, rate-limit and window consumption, context occupancy. Dollars are a projection of the ledger through a declared billing context. API pay-as-you-go prices the tokens; under subscription auth the binding constraints are usage windows and operator time. The insight layer — waste, repetition, right-sizing, feedback latency, yield — is unit-independent, and the billing lens varies per adopter.
- Static price: every generated context artifact carries its token count at build time, and the composition function emits a priced bill of materials (total, per artifact, cacheable fraction) at session start, before the spend. CI budget-gates each artifact class.
- Runtime capture consumes existing sources: Claude Code and Agent SDK telemetry, SDK usage results, `crosslink swarm harvest`, and `crosslink context`. Every recorded figure carries capture-source provenance; an agent never reports a cost it did not observe.
- Unit economics: cost per finding, per round, per phase, per layer, recorded in events and the tracker. Calibration bands update from actuals. Dispatch manifests declare the expected band, and the orchestrator compares actuals per result in-cycle.
- The optimization loop treats content rewrites as measured passes: token price down at constant finding yield is the win condition.
- Efficiency advisories are ledger queries the AI Engineer domain owns, surfaced at session boundaries with numbers: repeated searches propose a crosslink knowledge page; repeated reference-corpus reads propose an index or docs server once the ledger shows the repetition pays for it; cost-per-outcome by model tier by task class proposes tier defaults in the review config. Advisories follow decision routing — absorbable ones execute, and provisioning or tier defaults reach the operator with the math. Scheduled execution runs on crosslink sentinel. The ledger and events stay legible to crosslink's existing viewers (tui, mission control, web dashboard); vsdd builds no viewer.

Falsification: a cost figure in any record without capture-source provenance; a composition whose price cannot be produced; a calibration band with no linked actuals after a completed cycle.

Evidence: suite-era agents fabricated token and dollar figures because measurement was operator-only; the calibration bands were authored once and never updated from actuals; this session re-derived reference material that recorded usage would have flagged for reuse (suite claude-code-contract records; operator interview).

### The operator authors the oracle

Acceptance criteria, manual test checklists, and approval are operator-authored. No agent authors the oracle it is judged against, and no agent self-certifies a gate.

- Manual director testing is a first-class verification surface with its own checklist artifact per layer.
- Criteria and manual-test items carry provenance; agent-drafted candidates exist only as proposals until operator adoption is recorded.

Falsification: an acceptance criterion or manual-test item whose recorded provenance is agent-only; a gate marked passed by an agent that the gate command did not record.

Evidence: the ID-reuse invariant violation in issue-tracker-cli was caught by the operator running a manual checklist after eleven cold reviews missed it; the layer-7 Red Gate cheat was an agent-satisfied oracle (records in References).

## Requirements

Each requirement is referenced by its leading name.

- **Install** — `vsdd init` installs the environment into a crosslink-initialized repo: state artifact, hooks, generated skills and domain prompts, route table stub, gate configs, review config. Idempotent; refuses drifted managed files (extending existing init.rs behavior).
- **Status** — `vsdd status` answers the phase question from the state artifact and tracker in one command, machine- and human-readable (Deterministic phase answer). Status includes chassis-usage absence detection: crosslink initialized without a session, a mid-cycle project without milestones, findings discussed without filings. Silent non-use of the chassis is a detected condition reported at session start. Status also runs the process-integrity queries and reports drift unprompted, the way the chassis's own integrity and sync commands report stale locks and hydration mismatches: round-parity (manifest counts against tracked children), unresolvable handles in result comments, findings missing an owner or validator, closed findings missing evidence references, phase pointer against milestone state, and the unsigned-event count the chassis already detects at compaction. Detection lives in routine operation; nobody has to remember to check. (Evidence: the chassis's compaction warnings flagged this project's unsigned events twice before anyone acted, and the first manual round-parity run recovered a silently dropped finding — both records on the mdatron tracker.)
- **Composition function** — composition computed from DESIGN.md surfaces, phase, and review config by a deterministic function in vsdd-core; presets, calibration bands, the phase enumeration, and the action vocabulary ship as versioned data; mdatron validates config integrity (Deterministic composition).
- **Conformance checks** — mdatron gains: state schema and consistency checks, route-table validation, content-hash pin checks, and vocabulary and register checks against the register spec and registry (Conformance at action time). DSL scope: cross-file and registry validation.
- **Gates** — phase gates ship as commands runnable by `crosslink swarm gate`: red-gate checkout-and-run, unrouted-findings query, round-parity query (a dispatch manifest's reported finding count reconciles with its round issue's tracked children — a count claimed in narration without matching tracker objects is a detected condition; evidence: the 2026-07-19 spec-review rounds, where finding state regressed to comment narration for two rounds, quietly, because narration mimics compliance), mutation floor, and fresh-container install-and-smoke via `crosslink kickoff --container`, mechanizing the fresh-system install check that has been satisfied once in project history, manually (Phase exit by gate). During bootstrap, the parity discipline is format-carried: a round result comment is malformed unless it cites its child-issue handles.
- **Reviewer roles** — tool-restricted role definitions and the dispatch manifest format for swarm-backed Phase 3, including build-side scope manifests (Recorded review dispatch).
- **Generated context** — agent context (session skill and domain prompts) built from source data, token-budget-gated in CI (Deterministic composition). Supplement content is authored as per-domain sections — the existing extension sections, completed with an authoring section per supplement for pre-review use. The generator emits the sections matching the dispatch, so each reviewer or builder receives exactly its slice, priced per section. Domain-to-supplement mappings derive from the section structure at generation time; hand-declared duplicate mappings are removed (the live tree's contradiction between a domain prompt's declared supplements and a supplement's declared domains is the motivating case). Intentionally absent sections are declared, distinguishing not-applicable from not-yet-authored.
- **Directive flow** — directive-reconciliation guidance ships in the session skill with `crosslink intervene` wired in, including the substrate step and the recurrence-to-structure duty (Directive reconciliation).
- **Methodology rewrite** — methodology.md rewritten lean: whitepaper-faithful phases, the contracts above, plain names, with calibration and composition tables referenced as data. It defines the spec-review loop as phase 1a's internal cycle (operator amendment 2026-07-19, from the respec and mdatron increments' own practice): draft under declared composition; cold multi-lens rounds with manifests, declared coverage, evidence-gated citations, and clean-round validity stated; operator triage; fix passes routing findings back into the draft or upward into governing documents under the amend-then-act rule; repeat to the whitepaper's spec-dimension stop signal; operator ratification as the phase exit. The loop is chassis-managed: each round is a tracked child issue whose plan comment is the dispatch manifest; every triaged finding is an evidence-gated child issue with owner and validator, closed by the fix pass with revision evidence (dismissed and hallucinated findings file and close immediately with their disposition, making per-lens yield and hallucination rates queryable); the next round is gated on the prior round's full disposition; the stop signal is a tracker query. The loop has no red-gate stage — falsification paths and seeded criteria play that role for prose. Verified by operator read and an external cold reader, per the review-budget rule in Recorded review dispatch; it has no fixture-based criterion by design.
- **Waiver enumeration** — the waiver marker convention plus one-command enumeration (Finding lifecycle in one place).
- **Cost crate** — the cost ledger and insight engine (packaging per the cost-engine-packaging open question): artifact token-pricing at build, the priced bill of materials from the composition function, capture adapters with provenance, the unit ledger with query subcommands, CI budget gates (Cost is knowable).
- **Fixture corpus** — the acceptance suite's fixtures are a declared deliverable, and the corpus provides a fixture for every acceptance criterion and every falsification condition the criteria exercise. Named members: mid-cycle repo states for the convergence test; a red-gate cheat fixture reproducing the layer-7 incident from its source record; phase-3 terminal rounds in both failing shapes (unfiled findings; a real finding present); a mutation-floor fixture with a computable kill ratio; seeded finding sets including a HEAD-citation failure, an evidence-free closure, an ownerless open finding, and seeded waivers; an integrity-violating review config per integrity rule; drifted, unrouted, and misnamed artifacts; a session-entry violation and a trace-log gap; an unsigned manifest; two install trees (unmodified, hand-modified); status-detection repos (sessionless, milestone-less); oracle-provenance seeds (agent-only criterion, unrecorded gate pass); a scripted mid-cycle directive with a chassis-serviceable request and a second-occurrence correction; and a completed cycle for cost queries with calibration bands. Each fixture cites the incident record it reproduces where one exists (References).

## Acceptance criteria

Each criterion is referenced by its leading name; the parenthetical names what it verifies — a contract heading or a requirement name — matching exactly.

- **Convergence test** (Deterministic phase answer) — five independent cold sessions per fixture, across at least eight mid-cycle fixtures, answer the phase question in structured form: phase from the phase enumeration, next action from the action vocabulary (both ship as versioned data with the composition function; see Composition function). Independence means separate sessions with no shared conversation state or memory; sessions share only the fixture inputs. Equality is exact match on both fields. The spec ships only at full agreement on both fields across all sessions and fixtures. This is a smoke gate for determinism, and the sample sizes bound its strength; divergences found later route to the spec as defects.
- **Red-gate cheat blocked** (Phase exit by gate) — the fixture reproducing the layer-7 incident (red suite green against the pre-implementation base) is blocked by the gate; a fixture attempting phase-2b entry without a recorded red-gate failure is blocked.
- **Gate coverage** (Phase exit by gate) — a fixture with unfiled round findings fails phase-3 exit; a fixture whose terminal round contains a real finding fails phase-3 exit; a fixture with unrouted findings fails phase-4 exit; a fixture below the declared mutation floor fails the gate.
- **Lifecycle in the tracker** (Finding lifecycle in one place) — crosslink queries reproduce the full lifecycle for the fixture cycle; no lifecycle state exists outside the tracker; waiver enumeration returns all seeded waivers; the seeded finding citing absent content is rejected at filing; a seeded closure without an evidence reference is flagged as invalid; a seeded open finding without an owner is flagged.
- **Composition purity** (Deterministic composition) — the composition function is pure and property-tested; generated context artifacts byte-match their sources' emission; the token budget gate fails an over-budget generated artifact; each seeded integrity-violating config (pair co-activation, pair separation, validator-same-as-owner) fails validation.
- **Swarm live fire** (Recorded review dispatch) — one live `crosslink swarm review` run against a real repo completes with findings filed and manifest recorded, and the run demonstrates the contract's falsification checks: an off-manifest write blocks, coverage declarations appear in the manifest, every finding traces to the dispatch, and the manifest and findings verify as signed. The run's record becomes the reference fixture. This criterion is the precondition for binding Phase 3 to swarm; the swarm-fallback open question holds the alternative.
- **Drift pin** (Conformance at action time) — the drifted-pin fixture fails the gate and passes after re-pin; the unrouted-file fixture blocks; the underivably-named artifact fixture blocks; a seeded coinage in a governed artifact is flagged; a governed-file edit attempted before the session has read the governing docs is blocked; a seeded gap between the hook trace log and git history is reported.
- **Directive walkthrough** (Directive reconciliation) — the scripted mid-cycle directive produces a recorded classification before any execution, and the recorded classification contains the scope reading; the scripted directive is served by a chassis mechanism and the classification names it; a scripted second-occurrence correction produces a proposed mechanization.
- **Install behaviors** (Install requirement) — a second `vsdd init` run is a no-op on an unmodified tree; a hand-modified managed file causes refusal with a rustc-shaped diagnostic.
- **Status detection** (Status requirement) — a fixture repo with crosslink initialized and no session started is reported by status; a mid-cycle fixture without milestones is reported.
- **Oracle provenance** (The operator authors the oracle) — a provenance audit over the fixture cycle's criteria and manual-test items finds operator adoption recorded for each; a seeded agent-only criterion is flagged; a seeded agent-asserted gate pass with no gate record is flagged.
- **Cost queries** (Cost is knowable) — for the completed fixture cycle, the canonical questions are answerable by query: the session bill of materials matches the composed artifacts' stamped counts; cost-per-finding computes from captured actuals with provenance on every figure; an over-budget generated artifact fails CI; each calibration band links to the cycle's actuals, and a band with no linked actuals is reported.

## Architecture (sketch — Phases 1b and 1c deepen this)

vsdd-core: state schema, composition function with calibration data, context generator, gate commands. vsdd (binary): init, status, gate entrypoints, cost queries. mdatron: the new check families behind its existing verify pipeline, with no methodology knowledge — vsdd supplies schemas and patterns, mdatron executes them, per the boundary preamble. Hooks: thin Python wrappers invoking mdatron and vsdd, installed by init into `.claude/` and `.crosslink/` per chassis conventions; enforcement logic lives in mdatron pattern and registry data rather than in the scripts. Events: `.vsdd/events.jsonl` records phase transitions, gate results, and dispatch manifests — the audit record behind the gates.

Engineering conventions: crosslink and Thermite are the default reference for routine choices — tooling, CLI shape (uniform global flags, subcommand conventions), configuration (layered loading and config surfaces), CI shape, toolchain pinning. Divergence from an upstream convention carries a stated reason (operator ruling 2026-07-19; applies to this toolkit and mdatron alike).

Workspace shape: both repos keep their current two-crate workspaces. The binary-first plan's single-crate collapse (tracker issue #15) is superseded for vsdd — the cost crate requires a workspace — and deferred to Phase 1c for mdatron as an optional simplification (operator decision 2026-07-18).

## References and name map

External authorities:
- VSDD whitepaper (canonical methodology; "the whitepaper" in this document): https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00
- VDD whitepaper (predecessor; introduced the adversarial-review discipline VSDD extends): https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25
- dollspace (dollspace.gay): the author of both whitepapers, crosslink, and Thermite; the upstream collaborator. An "absorption proposal" is an offer of a pattern to these upstream projects.
- crosslink: https://github.com/dollspace-gay/crosslink (moved from forecast-bio/crosslink; older estate documents carry the stale URL) — local mirror at `../crosslink`.
- Thermite: https://github.com/dollspace-gay/Thermite — local mirror at `../Thermite`. Two distinct referents: the Thermite *language* (the verified programming language the repo builds, assessed in the thermite-assessment knowledge page) and the Thermite *development harness* (the process that builds it: `goal.md`, the four tool-restricted subagent roles its docs call the ACToR loop — doc-author, builder, critic, fixer — and the `tooling/` hooks). This document generalizes the harness; the language is out of scope (see Out of scope).
  - doc-drift: `../Thermite/tooling/doc-drift.py` · spec-discipline: `../Thermite/tooling/spec-discipline.py` · route table: `../Thermite/tooling/spec-routes.toml` · register standard: `../Thermite/.design/tone-and-voice.md`
- mdatron boundary preamble: `../mdatron/BOUNDARY-PREAMBLE.md` — the methodology-agnosticism anchor is the `consumes_from: []` declaration recorded in § 3's frontmatter.
- vocabulary registry: `templates/registry/vocabulary.yaml` today; the content rewrite relocates it into the generated-context source tree, and this entry updates on the move.
- operator interview of 2026-07-18: distilled as the operator-interview-2026-07-18 knowledge page in this repo's crosslink knowledge store (`crosslink knowledge show operator-interview-2026-07-18`); rulings also recorded as `decision` comments on the tracking issue.
- DSL falsifiability report: `../mdatron/dsl-falsifiability-report.md` — the falsifiability test hands a cold agent only the DSL spec and measures whether it authors correct rules; the report records 43% against an 80% bar and lists the 7 parked revisions.

Evidence records (the incidents this document cites as root causes):
- domain-value-scorecard and thermite-assessment: knowledge pages in this repo's crosslink knowledge store (`crosslink knowledge show <slug>`).
- issue-tracker-cli: `../guild-projects/guild-portfolio/issue-tracker-cli` — the layer-7 Red Gate cheat and the ID-reuse manual-test catch live in its review logs and meta-review.
- bookmark-cli-manual: `../guild-projects/guild-portfolio/vsdd-suite-reference-examples/bookmark-cli-manual` — the layer-3 round-3 hallucination incident (7 of 8 findings citing code absent from HEAD).
- The June 2026 toolkit cycles: issues #12/#13 and their children in this repo's tracker. Specific incidents: the empty no-findings issues (#122–#132, #229–#232), the thrice-closed-still-open plan issue (#13), the 18 misplaced reviews (relocation commits in both repos' histories), the 137-finding routing intervention (82 orphaned; #12/#13 children, resolution comments).
- The suite era: `../guild-projects/guild-portfolio/vsdd-suite` — the 13 hooks, the letter-cluster recurrences, the fabricated cost figures (`claude-code-contract.md`), the two-mode parity rule this design retires.

Estate name map:
- **vsdd-cli** (this repo): the toolkit under respecification; contains the `vsdd` binary crate and `vsdd-core` library crate.
- **mdatron** (sibling repo): the conformance engine; stays separate per its boundary preamble.
- **vsdd-suite** (in guild-portfolio): the predecessor prompt-and-process library; superseded, retained as evidence.
- **vsdd-cli-wip** (sibling tree): a frozen pre-implementation authoring snapshot; retired by the estate-cleanup work.
- **bookmark-cli-manual**, **issue-tracker-cli** (in guild-portfolio): reference projects built with the predecessor methodology; primary evidence sources.

Terms used from the chassis: the crosslink repo is the definition of record for its commands (intervene, sentinel, kickoff, harvest, context, and the rest); this subsection defines only the terms needing disambiguation here. *hub sync* is crosslink's git-branch synchronization, which assigns permanent display IDs to issues (until it runs, issues carry provisional local IDs). *Typed comments* are crosslink issue comments carrying a kind (`plan`, `decision`, `observation`, `blocker`, `resolution`, `result`). *skill-interactive* names the session mode where methodology content loads as skills in the working conversation, in contrast to the cold reviewer mode used for Phase 3.

Reserved word: *phase* means a VSDD methodology phase (1a through 6, per the whitepaper) and nothing else in this project's artifacts. Other staged sequences use other words: stage, step, round, or layer. Two known colliders this rule guards against: the retired binary-first plan numbered its stages "Phase 0–6" (its "Phase 4" was confused with methodology phase 4 in the June record), and crosslink's swarm command calls its plan segments "phases" — this project's documents say *swarm segment* when the distinction matters, since the chassis's own vocabulary is upstream's to define.

Terms defined by this design (draft vocabulary under the maturity lifecycle, registered here pending the registry's first publish):
- *domain* / *domain prompt*: a review perspective, and its criteria content as a deployable artifact — the established names carried forward from the predecessor. Distinct from a reviewer role (the tool-restricted agent identity that receives domain prompts).
- *surfaces*: the project declarations in DESIGN.md (for example: ui, user data, locales, ai runtime, attack surface) that drive domain activation. Supersedes the predecessor's *axes*. Surfaces describe the project; the review config describes the review.
- *section*: a supplement's per-domain slice — the existing extension blocks — plus one authoring section per supplement; the unit the context generator emits and prices.
- Retired by operator ruling 2026-07-18, recorded for the registry trail: *pack*, *axes*, *cell*, and the phrase *four-dimension convergence check* (enumerated inline instead).

## Open questions

<!-- OPEN: dsl-scope -->
### DSL scope (adopted default: narrow)
Narrowed to cross-file and registry validation; the falsifiability report's 7 body-content revisions parked as one tracked issue gated on a re-run falsifiability test meeting the 80% bar. Overridable at spec review.
<!-- /OPEN -->

<!-- OPEN: phase-state-location -->
### Phase-state location: repo artifact or crosslink-native
`.vsdd/state.yaml` is the draft answer (mdatron-validatable, repo-local, survives offline). Alternative: crosslink session and milestone fields as primary with the repo artifact as cache. Decide in Phase 1b against these criteria, raised by the round-1 quality review: offline and degraded-tracker semantics; concurrent-session conflict handling; mid-phase crash and resume; precedence when state and tracker disagree; and bootstrap validation (what checks the state and config that configure the checker, before the first gate).
<!-- /OPEN -->

<!-- OPEN: swarm-fallback -->
### Swarm fallback: the live fire may rescope the review-dispatch contract
swarm review, fix, and pipeline are changelog-documented upstream and have never run here. If the live fire reveals gaps, the dispatch contract falls back to swarm primitives (worktree launch plus gate) with vsdd supplying the review stage, or contributes fixes upstream per the absorbability goal.
<!-- /OPEN -->

<!-- OPEN: cost-engine-packaging -->
### Cost-engine packaging (adopted default: bounded crate)
The cost engine ships as a separate crate in the vsdd workspace with mdatron-style boundary discipline: methodology-agnostic (dimensions are caller-supplied labels), zero vsdd-core imports, versioned public data contracts (ledger entry schema, capture-source provenance enumeration, bill-of-materials format), fronted by `vsdd cost` subcommands. Extraction to a standalone app triggers on a second real consumer: a project outside this methodology adopting it, or upstream crosslink interest in the ledger schema. An absorption proposal to the upstream author is worth making regardless; Thermite's token-economics thesis currently has no measurement layer. Rationale: the engine passes the agnosticism test, but it has one consumer today, and the two-repo record shows coordination cost is real.
<!-- /OPEN -->

<!-- OPEN: estate-cleanup -->
### Estate cleanup mechanics
Retire vsdd-cli-wip (harvest `archive/` to a preserved location), delete the stray `review-log 2/` directory, truth-reconcile mdatron's README and DESIGN to the implemented surface, bulk-close the ~124 audit-record issues with `result` disposition comments and archive them via crosslink archive, correct the portfolio README's claim of a crosslink-mode reference example that is absent from disk, sweep the estate's stale crosslink URLs (the repo moved from forecast-bio/crosslink to dollspace-gay/crosslink; methodology.md, the READMEs, and the suite docs carry the old address), and bring mdatron fully onto the chassis: tracker remote configured, hub sync, signing enforcement matching vsdd-cli. Subject-repo finding filing requires a working subject-repo tracker. Sequenced in Phase 1c as layer 0, since cleanup precedes build.
<!-- /OPEN -->

## Out of scope (v1)

OTel collectors, dashboards, FinOps report surfaces, LSP, SARIF, cosign and SLSA release pipeline, body-content DSL extraction, adoption of the Thermite language (revisit on ecosystem and macOS maturity), and the retired two-mode manual parity (operator decision 2026-07-18). MCP servers: none built in v1; reference indexes and docs servers are evidence-provisioned via the efficiency-advisory loop in the Cost is knowable contract.
