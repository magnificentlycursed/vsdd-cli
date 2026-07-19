# Feature: Agent-first VSDD toolkit on the crosslink chassis

Crosslink issue: "Respec: agent-first VSDD toolkit on the crosslink chassis" (vsdd-cli tracker; provisional local ID L1 pending hub sync — reference by title) · Phase: 1a (behavioral specification), skill-interactive
Supersedes on ratification: DESIGN-METHODOLOGY.md, DESIGN-SCHEMA.md, DESIGN-OBSERVABILITY.md, DESIGN-VERIFICATION.md, docs/refactor/binary-first-plan.md (#11–#17 rescoped)
Evidence base: five-dataset domain scorecard, VSDD/VDD whitepapers, crosslink governing docs, Thermite harness (ACToR), operator interview 2026-07-18 (session record)

## Summary

The toolkit provably enables and enforces VSDD for projects whose work is executed by AI agents and directed by a human operator. Delivery is agent-first: the product is an installed environment — context, constraints, and hooks wired into crosslink's seams — in which following the methodology is the path of least resistance and deviating from it produces immediate, compiler-shaped feedback. The operator's session-start intent ("draw an owl") expands deterministically because the *process* is deterministic: same project state + same process spec → same next action, for any competent cold agent.

Division of labor:
- **crosslink** (chassis): issues, dispatch (swarm), worktrees, gates, sessions, knowledge, cost. The contract chain lives here.
- **vsdd** (methodology layer): process spec, phase state, composition, criteria packs, generated agent context, hooks. Installed into crosslink's seams (rules, skills, hook config, house style).
- **mdatron** (conformance engine): validates every methodology artifact — including the phase state file and route table — invoked at action time by hooks and at boundary time by gates. Methodology-agnostic per its boundary preamble.
- **operator**: doctrine, scope, priorities, approval, oracle authorship (acceptance criteria + manual tests). Everything else is deliberately not the operator's job.

## Project declarations

Intent tiers are retired (operator decision 2026-07-18: the tier system was cut from vsdd-suite in practice — reviewers were mixed and matched ad hoc, which the tiers made invisible rather than impossible). In their place: a review configuration file, preset-seeded and operator-customized (see the deterministic-composition contract).

This project's own declarations:
**Review config:** thorough preset (the toolkit governs other projects; its defects propagate) — all load-bearing packs + mutation floor; declared in `.vsdd/config.yaml` on adoption.
**Phase 5 strategy:** mutation testing on vsdd-core and mdatron-core (kill-ratio floor enforced in CI); fuzz testing on mdatron's frontmatter/DSL parsers (trust boundary: untrusted markdown); no proof execution — no cryptographic or safety-critical surface. Re-evaluate if Thermite becomes viable for these crates.
**Phase 6 strategy:** applicable at v1.0 cut — four-dimension convergence check before crates.io publish.

## Behavioral contracts

Referenced by name — plain names, no code labels. Each is observable from outside the implementation and names its falsification path.

### Deterministic phase answer

"What phase are we in?" has exactly one correct answer, derivable by any cold agent, at any time.

- The phase state is a written artifact (`.vsdd/state.yaml`) in the project repo: current layer, current phase, open findings pointer (crosslink query), last gate result, active composition.
- The state file is advanced only at phase boundaries, by the agent, in the same commit as the boundary evidence; mdatron validates its schema and its consistency with the evidence (a state claiming phase-2b requires a red-gate result recorded for the layer).
- The state rides the chassis's session machinery: the phase pointer travels in crosslink session breadcrumbs (surviving compaction and re-injected on resume), `session start` surfaces phase status with the prior handoff, phase boundaries require handoff notes, and each layer is a crosslink milestone — tracker-side progress and the state file corroborate each other.
- Falsification: two cold sessions given the same repo answer differently → spec defect (not agent defect). A state file inconsistent with repo evidence → mdatron finding, gate-blocking.

### Directive reconciliation

Every mid-flow operator directive is classified before execution. Classifications: (a) phase-4 routing (bind: file the routing, name target phase), (b) spec amendment (amend the design doc, then act), (c) recorded override (execute, record via `crosslink intervene`, state where the flow stands and the re-entry path).

- Silent execution of an unclassified directive is a named defect (VSDD-side dual of the operator's no-self-authorized-overrides rule).
- The classification is stated to the operator at the moment of receipt, not discovered later — including the inferred scope: whether the operator's words are being read as a specific instruction or as instances of a general rule. (Root cause on record: specificity drift in both directions — single utterances over-generalized into named concepts, and illustrative examples under-generalized into enumerated special cases. The scope echo makes the inference correctable at the moment it is made.)
- Falsification: a directive executed with no classification recorded in the tracker or session record.

### Phase exit by gate, not assertion

A phase closes only when its exit gate passes. Gates are commands (`crosslink swarm gate` runs them); their results are recorded in the state file and the tracker.

- Red Gate (mechanized): phase 2a closes only when the layer's test suite provably fails against the pre-implementation commit and the failing run is recorded. Phase 2b closes only when the same suite passes at HEAD. The gate performs the checkout-and-run; the agent asserts nothing.
- Phase 3 closes only at the stop signal (a round producing only hallucinated findings) AND all round findings filed. Phase 4 closes only when zero unrouted findings exist (crosslink query returns empty).
- Anti-Goodhart floor: mutation kill-ratio on changed code is a standing gate criterion when the review config declares a mutation floor (the thorough preset does by default).
- Falsification: any phase transition in the state file without its recorded gate evidence → mdatron finding, blocking.

### Finding lifecycle in one place

Finding lifecycle state (classification, routing, closure) lives in crosslink issues only. Review narrative may exist but carries no state.

- Findings are filed in the subject project's tracker (subject_project is a routing instruction, not a disambiguator).
- Every finding names an owner domain and a validator that is not its owner (validator lifecycle; sanity-check is validator of last resort). Adversarial pairs are never co-located in one reviewer session.
- Filing is evidence-gated: a finding carries checkable evidence at filing time — a file:line citation mdatron verifies against HEAD, or a failing test where the surface allows one. A finding citing code not in HEAD is rejected at the door, not classified later. (Root cause on record: bookmark-cli layer 3 round 3 — 7 of 8 findings cited nonexistent code while tests were green; 8 hallucinated findings in the toolkit cycles each cost filing, triage, and closure traffic.)
- Lifecycle acts use crosslink's typed comments as the single vocabulary: closures are `result` comments carrying the evidence reference, operator rulings are `decision`, routing is `plan` — the classification system and the chassis comment kinds are one system, not two.
- Closure is evidence-graded: a closing comment references a commit, gate run, or test — or the closure is invalid. Dismissed/hallucinated findings close immediately with their disposition; they do not remain open as audit records (git and the tracker history are the audit trail).
- Waivers are greppable: any waived criterion, deferred obligation, or accepted risk carries a single canonical marker enumerable by one search.
- Falsification: lifecycle state found in a review-log file that differs from the tracker; a closed finding with no evidence reference; an open finding with no owner.

### Deterministic composition

Given the phase, the project's declared axes (in DESIGN.md), and the project's review configuration, the active criteria packs and their dispatch shape are computed, not judged.

- The review configuration is a file (`.vsdd/config.yaml`), modeled on crosslink's layered config: shipped presets (suggested starting points, each with a priced bill of materials) → project config → local overrides. The operator customizes freely; the customization is a declared, git-tracked, schema-validated input — mixing and matching is visible and deterministic, not ad hoc. Fields: active packs, round budget, stop sensitivity, model tiers, mutation floor, cost bands.
- Config integrity rules are validated by mdatron: pair co-activation (activating an author-lens pack activates its cold-reader pair), pair separation at dispatch, validator-not-owner. Narrowing the active set never weakens the per-pack bar.
- A mid-cycle config change is an operator directive: it gets the directive-reconciliation treatment and a recorded event.
- The same inputs produce the same composition every time; presets carry no doctrine — the rules above do.
- Composition operates at two timescales. Session composition (above) selects packs per phase. Action-time activation binds content to its moment mechanically, never by agent recall: every supplement declares its activation trigger as a required field of the supplement artifact class — the harness surfaces whichever supplements match the task at hand, with a hook backstop (editing a file whose matching supplement was not read this session fires at action time). Generality is structural: adding a supplement means content plus trigger, no harness wiring; "write python" loading the python supplement is one row of a table any supplement joins. The chassis supplement's always-relevant core rides in the generated always-on session context, priced like everything generated. (Root cause on record: the crosslink-usage knowledge existed in the claude-code-cli supplement but was positioned as a review-time lens — the operator believed crosslink was in use, it was not, and nothing was positioned to notice.)
- Domain criteria are content (packs), not agents. Zero-yield-shaped packs simply do not activate; a non-activated or no-finding pack produces no artifacts.
- Agent-facing context (the session skill, criteria packs, supplements) is generated from these data sources and token-budget-gated in CI; hand-edited drift between source and emitted context is a build failure.
- Falsification: two sessions computing different compositions from identical inputs; an emitted context artifact differing from its source data.

### Review dispatch is a recorded mechanical operation

Phase 3 dispatch runs on crosslink swarm. Every dispatch produces a manifest: what was sent, to which reviewer role, containing which inputs (content hashes), under which composition, what came back, where findings were filed.

- Reviewer roles are tool-restricted (a critic role cannot edit; approval never belongs to an agent).
- Dispatch manifests and filed findings are signed with crosslink agent identities (the trust model both repos already carry): who found what, under which dispatch, is non-repudiable — the cryptographic half of "provably."
- Build dispatches carry scope manifests, same as review dispatches: a pre-declared writable file set per task; an off-manifest write blocks with "manifest needs expansion," not a silent widening. One concern per manifest keeps commits revertible. (Root causes on record: 18 mdatron reviews written into vsdd-cli's review-log; the bundle commit with no rollback path flagged by the operator's own review.)
- Coverage is declared, not silent: the dispatch manifest records which packs did NOT run and why (not activated: no surface / declined by config / deferred with trigger). Absence of findings and absence of looking are distinguishable facts; neither produces tracker issues. (The methodology found this principle twice — the empty supplements-in-scope ambiguity finding and the Phase 5 "not applicable — rationale" rule — and never generalized it; the 23 empty no-findings issues in the toolkit cycles are what silence costs.)
- Round stop rules are the whitepaper's: continue on any real finding; stop when a round is all-hallucinated; rounds past the stop signal require named new evidence.
- Review budget follows the evidence: cold review spends on code-shaped artifacts; prose artifacts get the operator's read and external cold readers; spec-stage structural review (small, early, few domains) is first-class.
- Falsification: a Phase 3 record with no manifest; findings whose provenance cannot be traced to a dispatch; a build commit touching files outside its declared manifest; a pack that neither ran nor has a recorded reason.

### Conformance at action time

mdatron validates methodology artifacts (state file, DESIGN.md structure, route table, criteria packs, generated context) with rustc-shaped diagnostics, invoked by hooks at action time (PostToolUse) and by gates at boundary time.

- Spec-drift is mechanical: design docs pin a content hash over the files they govern; governed-file changes without a re-pin fail the gate (Thermite's doc-drift, generalized).
- Session-entry enforcement: edits to governed files are blocked until the session has read the governing docs (generalizing Thermite's spec-discipline hook), configured per project.
- Vocabulary and register are governed: a register spec (modeled on Thermite's tone-and-voice: concrete named anti-patterns, rationale — agents anchor on the register they read, so corpus tics reproduce; vocabulary creep is context contamination) plus the vocabulary registry (`registry/vocabulary.yaml`, promoted to load-bearing; term additions carry earned-by-recurrence justification). mdatron checks the mechanical subset at action time: invented label schemes (letter+number clusters outside the chassis allowlist), unregistered abbreviations, new-coinage detection, listed register tics. The evidence this needs a mechanical check rather than a memo: four suite-era recurrences of the letter-cluster pattern past written correction, and two in this design session past a loaded memory.
- Vocabulary and schemas have a maturity lifecycle: **draft** terms and schemas may be renamed or deleted freely, carry zero compatibility obligation, and are exempt from strict consistency checks and consistency findings; promotion to **established** is an explicit operator act (first-publish is the gate), never implicit and never agent-initiated. An utterance is not a concept: agents do not reify casual operator phrasing into named terms, frontmatter fields, error codes, or schema elements without registration. Forward-only discipline applies to established artifacts only. (Root cause on record: single-utterance reification plus forward-only produced backwards-compatibility work on schemas still being designed, and vocabulary-consistency finding storms in the review cycles.)
- The artifact set is closed-world: the route table is an allowlist, not an index — an unrouted artifact blocks (generalizing Thermite's no-route-blocks rule). New artifact classes and new name slugs are registration acts under the same maturity gate as vocabulary (a slug is a coinage wearing a filename). Generated artifacts carry computed names derived from their manifest — agents never hand-name what the tooling can name. (Root cause on record: mdatron validated per-file shape in an open world; documentation-type proliferation moved to the unconstrained axis — the review-log topic slugs — and per-file validation never noticed.)
- Falsification: a governed file changed with a stale pin passing the gate; a hook-bypassed edit leaving no trace; a file admitted with no route; an artifact whose name is not derivable from its manifest or registered grammar.

### Cost is knowable, not vibes

Every methodology operation has a measurable cost, a breakdown, and a baseline — answerable as queries, not estimates. The canonical questions ("is 100k a lot? what makes it up? cached or not? would a rewrite reduce it?") each have a recorded-evidence answer.

- Cost is denominated in native units — tokens by cache class, wall-clock, rate-limit/window consumption, context occupancy — never dollars. Dollars are a projection of the ledger through a declared billing context: API pay-as-you-go prices the tokens; under subscription auth the binding constraints are usage windows and operator time, and the dollar projection is zero-marginal. (The AI Engineer pack's auth-method × cost-model dimension already draws this distinction; the ledger honors it structurally.) The insight layer — waste, repetition, right-sizing, feedback latency, yield — is unit-independent and is the point; the billing lens varies per adopter.

- Static price: every generated context artifact (primer, pack, supplement, skill) carries its token count at build time; the composition function emits a priced bill of materials (total, per-artifact, cacheable fraction) at session start — cost feedback before the spend, visible to the agent. CI budget-gates each artifact class.
- Runtime capture: consume, don't build — Claude Code/Agent SDK telemetry, SDK usage results, `crosslink swarm harvest`, and `crosslink context` (the chassis already measures its own context-injection overhead) supply actuals; every recorded figure carries capture-source provenance (the fabrication wound stays closed: an agent never reports a cost it did not observe).
- Unit economics: cost per finding, per round, per phase, per layer recorded in events and the tracker; calibration bands are updated from actuals, not fossilized at first guess. Dispatch manifests declare the expected band; the orchestrator compares actuals per result and reacts in-cycle (the pre-cycle methodology check, mechanized).
- Optimization loop: content rewrites are measured passes — price (tokens) down at constant yield (findings-per-pack from the lifecycle record) is the win condition.
- Efficiency advisories are ledger queries the AI Engineer pack owns running, surfaced at session boundaries with numbers: repeated searches across sessions → propose a crosslink knowledge page; repeated reference-corpus reads → propose an index or docs server once the ledger shows repetition pays for it (infrastructure is evidence-provisioned, neither built ahead nor never); cost-per-outcome by model tier by task class (dispatch manifests record the model; the lifecycle records the yield) → propose tier defaults in the review config. Advisories follow decision routing: absorbable ones execute, provisioning and tier defaults reach the operator with the math. Scheduled execution (advisory queries, drift sweeps, staleness checks) runs on crosslink sentinel — a standing duty needs a scheduler, or it means "when someone remembers." Ledger and events stay legible to crosslink's existing viewers (tui, mission control, web dashboard) — vsdd grows no viewer of its own.
- Falsification: a cost figure in any record without capture-source provenance; a composition whose price cannot be produced; a calibration band with no linked actuals after a completed cycle.

### The operator authors the oracle

Acceptance criteria, manual test checklists, and approval are operator-authored. No agent authors the oracle it is judged against; no agent self-certifies a gate.

- Manual director testing is a first-class verification surface with its own checklist artifact per layer (the record shows it catches what cold review misses).
- Falsification: an acceptance criterion or manual-test item whose provenance is agent-only; a gate marked passed by the agent that the gate command did not record.

## Requirements

Each requirement is referenced by its leading name.

- **Install** — `vsdd init` installs the environment into a crosslink-initialized repo: state file, hooks, generated skills/criteria packs, route table stub, gate configs. Idempotent; refuses drifted managed files (existing init.rs behavior extends).
- **Status** — `vsdd status` (or equivalent) answers the phase question from the state file + tracker in one command, machine- and human-readable (deterministic phase answer). Status includes chassis-usage absence detection: crosslink initialized but no session started, mid-cycle project with no milestones, findings discussed but not filed — silent non-use of the chassis is a detected condition, reported at session start.
- **Composition function** — composition is computed from DESIGN.md axes + phase + review config by a deterministic function shipped in vsdd-core; presets and calibration bands ship as versioned data; mdatron validates config integrity (pair co-activation, pair separation, validator-not-owner) (deterministic composition).
- **Conformance checks** — mdatron gains: state-file schema + consistency checks, route-table validation, content-hash pin checks, vocabulary/register checks against the register spec + vocabulary registry (conformance at action time). DSL scope: cross-file/registry validation.
- **Gates** — phase gates ship as commands runnable by `crosslink swarm gate` (red-gate checkout-and-run, unrouted-findings query, mutation floor, fresh-container install-and-smoke via `crosslink kickoff --container` — mechanizing the fresh-system install check that has been satisfied exactly once, by an external reviewer's manual favor) (phase exit by gate).
- **Reviewer roles** — tool-restricted role definitions + the dispatch manifest format for swarm-backed Phase 3 (recorded review dispatch).
- **Generated context** — agent context (session skill + packs) built from source data, token-budget-gated in CI (deterministic composition). Supplement content is authored as (supplement × lens) cells — the existing per-domain extension sections, kept and completed with an authoring cell per supplement for pre-review use; the generator slices cells per dispatch, so each reviewer or builder receives exactly its slice, priced per cell. Pack↔supplement mappings are derived from the cell structure at generation time, never hand-declared on both sides (the live tree's drift — a domain declaring no supplements while a supplement claims that domain — is the counter-example). Intentionally absent cells are declared, distinguishing not-applicable from not-yet-authored.
- **Directive flow** — directive-reconciliation guidance ships in the session skill; `crosslink intervene` wired into its flow (directive reconciliation). Reconciliation includes a substrate step drawn from the AI Engineer pack's content: when a chassis or harness mechanism serves the directive (milestone, session, hook, skill, config), it is named and offered at classification time. The AI Engineer pack additionally owns the recurrence-to-structure duty: the second occurrence of an operator correction or struggle obliges a proposed mechanization (pattern entry, hook, skill, config change) — turning recurring findings into structural fixes is a contract obligation, not reviewer initiative.
- **Methodology rewrite** — methodology.md rewritten lean: whitepaper-faithful phases, the contracts above, plain names, no coinages; the calibration and composition tables referenced as data.
- **Waiver enumeration** — waiver marker convention + one-command enumeration (finding lifecycle).
- **Cost crate** — the cost-knowability crate (packaging per the cost-engine-packaging question below): artifact token-pricing at build, priced bill of materials from the composition function, capture adapters (SDK usage / Claude Code telemetry / crosslink harvest) with provenance, unit ledger + query subcommands, CI budget gates.

## Acceptance criteria

Each criterion is referenced by its leading name; the contract it verifies follows in parentheses.

- **Convergence test** (deterministic phase answer) — two independent cold sessions given the same mid-cycle repo state answer "what phase, what next action" identically, on 5 prepared fixtures. This is the falsifiability instrument for the process spec; the spec ships only at 100% on phase answer.
- **Red-gate cheat blocked** (phase exit by gate) — a fixture repo attempting phase-2b entry without a recorded red-gate failure is blocked by the gate; the layer-7-style cheat from the issue-tracker record (red suite green against the pre-implementation base) is detected mechanically.
- **Lifecycle in the tracker** (finding lifecycle) — `crosslink issue list` filters reproduce the full finding lifecycle for a fixture cycle; zero lifecycle state exists outside the tracker; waiver enumeration returns all seeded waivers; a seeded finding citing code absent from HEAD is rejected at filing.
- **Composition purity** (deterministic composition) — composition function is pure and property-tested; generated context artifacts byte-match their sources' emission; token budget gate fails on an over-budget pack.
- **Swarm live fire** (recorded review dispatch) — one live-fire `crosslink swarm review` run against a real repo completes review→findings-filed→manifest-recorded; documented as the reference run. (Precondition for binding Phase 3 to swarm — currently unproven in anger.)
- **Drift pin** (conformance at action time) — doc-drift fixture: governed file edited without re-pin → gate fails; with re-pin → passes.
- **Directive walkthrough** (directive reconciliation) — a scripted directive mid-fixture-cycle produces a recorded classification before any execution.
- **Cost queries** (cost knowability) — for a completed fixture cycle, the canonical cost questions are answerable by query: session bill of materials (total, per-artifact, cacheable fraction) matches the composed artifacts' stamped counts; cost-per-finding computed from captured actuals with provenance on every figure; an over-budget generated pack fails CI.

## Architecture (sketch — Phase 1b/1c deepen this)

vsdd-core: state schema, composition function + calibration data, context generator, gate commands. vsdd (binary): init, status, gate entrypoints. mdatron: new check families (the conformance-checks requirement) behind its existing verify pipeline; no methodology knowledge (boundary preamble § 3 holds — vsdd supplies the schemas/patterns, mdatron executes them). Hooks: thin Python wrappers invoking mdatron/vsdd, installed by init into .claude/ + .crosslink/ per chassis conventions — enforcement logic lives in mdatron pattern/registry data, never in the hook scripts themselves (the suite's 13 hand-authored hooks are the counter-example: one bespoke program per escape axis, discovered post-hoc; here a new defect class is closed by adding a pattern, a diffable artifact under the same maturity gate as vocabulary). Events: `.vsdd/events.jsonl` records phase transitions, gate results, dispatch manifests — the audit half of "provably"; nothing else in v1.

## Open questions

<!-- OPEN: dsl-scope -->
### DSL scope (adopted default: narrow)
Narrowed to cross-file/registry validation; the falsifiability report's 7 body-content revisions parked as one tracked issue gated on a re-run ≥80% falsifiability test. Overridable at spec review.
<!-- /OPEN -->

<!-- OPEN: phase-state-location -->
### Phase-state location: state file vs crosslink-native
`.vsdd/state.yaml` is the draft answer (mdatron-validatable, repo-local, survives offline). Alternative: crosslink session/milestone fields as primary with the file as cache. Decide in Phase 1b after checking what crosslink exposes for structured session state.
<!-- /OPEN -->

<!-- OPEN: swarm-fallback -->
### Swarm fallback: the live fire may rescope the review-dispatch contract
swarm review/fix/pipeline are changelog-documented and never run here. If the live fire reveals gaps, the dispatch contract falls back to swarm primitives (worktree launch + gate) with vsdd supplying the review stage — or contributes fixes upstream (absorbability goal).
<!-- /OPEN -->

<!-- OPEN: cost-engine-packaging -->
### Cost-engine packaging (adopted default: bounded crate, not third app)
The cost-knowability engine ships as a separate crate in the vsdd workspace with mdatron-style boundary discipline: methodology-agnostic (dimensions are caller-supplied labels), zero vsdd-core imports, versioned public data contracts (ledger entry schema, capture-source provenance enum, bill-of-materials format), fronted by `vsdd cost …` subcommands. Extraction to a standalone app triggers on a second real consumer (a non-VSDD adopter, or upstream crosslink interest in the ledger schema — an absorption proposal to dollspace is worth making regardless; Thermite's token-economics thesis has no measurement layer today). Rationale: engine-shaped by the agnosticism test, but one consumer today — "earned by recurrence" applies to repo splits too; the two-repo record shows coordination cost is real.
<!-- /OPEN -->

<!-- OPEN: estate-cleanup -->
### Estate cleanup mechanics
Retire vsdd-cli-wip (archive/ harvested to a preserved location), delete `review-log 2/`, truth-reconcile mdatron README/DESIGN to implemented surface, bulk-close ~124 audit-record issues with `result` disposition comments and archive them (crosslink archive, not bare closure), correct the portfolio README's bookmark-cli-crosslink overclaim, and bring mdatron fully onto the chassis: tracker_remote configured, hub sync, signing enforcement matching vsdd-cli — subject-repo finding filing requires a working subject-repo tracker. Sequenced in Phase 1c as layer 0 (cleanup precedes build).
<!-- /OPEN -->

## Out of scope (v1)

OTel collectors, dashboards, FinOps surfaces, LSP, SARIF, cosign/SLSA release pipeline, body-content DSL extraction, Thermite-language adoption (revisit on ecosystem + macOS maturity), two-mode manual parity (retired by operator decision 2026-07-18). MCP servers: none built in v1 — reference indexes and docs servers are evidence-provisioned via the efficiency-advisory loop in the cost contract, not built ahead.
