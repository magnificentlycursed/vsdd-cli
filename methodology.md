---
schema_class: methodology-spec
methodology_version: 0.1.0
methodology_anchors: [README.md, DESIGN-METHODOLOGY.md, DESIGN-SCHEMA.md, DESIGN-OBSERVABILITY.md, DESIGN-VERIFICATION.md]
whitepaper_url: https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00
substrate_anchors: [https://github.com/forecast-bio/crosslink, https://code.claude.com/docs/en/agent-sdk/overview]
last_revision_trigger: "Phase 1a spec authoring per DESIGN-METHODOLOGY § Methodology spec section list"
---

# VSDD Methodology

This document is the **canonical governing spec** for projects adopting Verified Spec-Driven Development (VSDD) via the `vsdd` toolkit. It captures the load-bearing disciplines in concise prose. Per-subsystem design lives in the DESIGN docs at the toolkit repo (`DESIGN-METHODOLOGY.md`, `DESIGN-SCHEMA.md`, `DESIGN-OBSERVABILITY.md`, `DESIGN-VERIFICATION.md`); the methodology itself is authored by [@dollspace.gay](https://bsky.app/profile/dollspace.gay) in the [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00). The toolkit is **one collaborator's interpretation + operationalization** of dollspace's methodology as a Rust CLI; not the methodology itself.

---

```yaml
---
section_name: opening-scope
required: true
target_lines: 25
event_variants_referenced: []
domains_referenced: []
phases_referenced: []
---
```

## Opening + scope

VSDD is a software methodology that treats spec, tests, implementation, and formal verification as four independent dimensions that must independently reach maximum viable refinement (MVR) and then converge. The canonical articulation lives in the [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) authored by dollspace; this spec governs the mechanical operationalization of that methodology in adopting projects via the `vsdd` toolkit.

This spec is **load-bearing prose** for both human operators and AI agents. Every architectural decision in the DESIGN docs surfaces here in at least one section; every event variant and artifact class declared by the toolkit is named here; every phase has a corresponding primer; every active domain has a corresponding prompt. The `check-methodology-semantics.py` hook mechanically validates these invariants.

The spec is **not** the implementation runbook. Per-operator runbooks live in DESIGN docs + per-phase primers. The spec captures the methodology disciplines that adopting projects honor regardless of toolkit version.

---

```yaml
---
section_name: four-governing-design-goals
required: true
target_lines: 35
event_variants_referenced: []
domains_referenced: []
phases_referenced: []
---
```

## Four governing design goals

Every subsystem the toolkit ships serves all four goals; every artifact demonstrably advances at least one.

**Goal 1 — Absorbability-ready patterns.** Every enhancement is designed in shapes that absorb cleanly into [crosslink](https://github.com/forecast-bio/crosslink) if upstream chooses. Absorbability is a quality property of the design — not a strategic dependency. Patterns ship standalone; absorption proposals follow when each is mature.

**Goal 2 — Auditable + machine-enforceable + dual-audience.** Every artifact serves humans and agents with three structural properties: structured audit-trail evidence emits on every action (auditable); every methodology rule has a hook OR schema validator OR crosslink workflow check (machine-enforceable); narrative prose + structured frontmatter coexist in every artifact (dual-audience-readable).

**Goal 3 — Observability + Monitoring + Alerting + Traces + Dashboarding as first-class.** Observability Engineering + FinOps are design dimensions. Every artifact is born observable. Default-on, not opt-in. The toolkit composes against the [Claude Agent SDK's OpenTelemetry export](https://code.claude.com/docs/en/agent-sdk/observability) as primitive; methodology-specific event variants augment SDK signals.

**Goal 4 — Shift VSDD left into CI/CD pipelines.** The methodology runs in CI/CD, not only operator-local terminals. Phase-transition provability, phase-composition declaration, schema validation, drift sweeps, anonymization, identity-correlation — all shift to PR-time mechanical enforcement. Pre-merge mechanical enforcement frees reviewer time for findings requiring judgment.

---

```yaml
---
section_name: phase-taxonomy
required: true
target_lines: 30
event_variants_referenced: [PhaseEntered, PhaseExited, PhaseTransitionAttested]
domains_referenced: []
phases_referenced: [phase-1a, phase-1b, phase-1c, phase-2a, phase-2b, phase-2c, phase-3, phase-4, phase-5, phase-6]
---
```

## Phase taxonomy

Strict whitepaper-canonical 10 sub-phases. Each phase has a primer at `.claude/commands/vsdd-phase-<id>.md`.

| Phase | Name | One-line summary |
|---|---|---|
| 1a | Behavioral Specification | Author DESIGN.md's behavioral contracts for the layer; spec-authority lens |
| 1b | Verification Architecture | Author DESIGN.md's verification architecture; test strategy + purity boundary |
| 1c | Spec Review Gate (Decomposition) | Decompose spec into layers; spec-gate close validates layer entry |
| 2a | Test Suite Generation (Red Gate) | Author failing tests against the spec; QE-primary; falsifiability check |
| 2b | Minimal Implementation | Implement minimal code to turn the Red Gate green; SE-primary |
| 2c | Refactor | Re-shape implementation with tests staying green; SA + SE |
| 3 | Adversarial Refinement (The VDD Roast) | Cold-session multi-domain reviewer pass; classification + routing |
| 4 | Feedback Integration Loop | Route Phase 3 findings to the earliest phase that fixes them correctly |
| 5 | Formal Hardening | Property tests + Mutation Testing + Fuzz Testing + Proof Execution per project intent |
| 6 | Convergence (The Exit Signal) | Project-terminal four-dimensional convergence attestation |

Operators may author Phase 1a + 1b in a single session. Phase 5 + Phase 6 are first-class but optional per project intent (declared in DESIGN.md `Phase 5 strategy:` + `Phase 6 strategy:` lines).

---

```yaml
---
section_name: phase-domain-composition
required: true
target_lines: 40
event_variants_referenced: [PhaseCompositionDeclared]
domains_referenced: [solution-owner, solution-architect, software-engineer, quality-engineer, technical-writer, documentation-reviewer, platform-engineer, performance-engineer, security, red-team, ux, accessibility, privacy, localization, data-engineer, ai-engineer]
phases_referenced: [phase-1a, phase-1b, phase-1c, phase-2a, phase-2b, phase-2c, phase-3, phase-4, phase-5, phase-6]
---
```

## Phase-domain composition

Each phase composes with specific domains. The matrix is load-bearing; enforced at four layers (matrix declaration; per-primer instruction; pre-phase composition declaration; `check-phase-composition.py` hook).

| Phase | Composed domains (skill mode unless noted) |
|---|---|
| 1a Behavioral Spec | SO (primary); UX, Accessibility, Privacy, Localization per per-feature axes |
| 1b Verification Architecture | SO + SA + QE |
| 1c Spec Review Gate | SA (primary); SO co-stewards |
| 2a Red Gate | QE (primary) |
| 2b Minimal Implementation | SE (primary); TW + DR (prose); QE (test-pyramid); PE + axes-activated domains |
| 2c Refactor | SE + SA |
| 3 Adversarial Refinement | All active domains (cold-session reviewer mode; NOT skill mode) |
| 4 Feedback Integration | None specific (operator-orchestrated routing) |
| 5 Formal Hardening | QE + Security + SA |
| 6 Convergence | None specific (operator-orchestrated attestation) |

**Always-on domain baseline (additive over axes):** SE + QE + SA + SO activate regardless of axes; PE + Performance Engineer activate when the project ships code. The per-feature axes matrix extends from this baseline. A zero-axes project that ships code activates 6 composed domains, not zero.

**Four enforcement mechanisms:**
1. **Matrix declaration** — load-bearing prose section; operators see composition as structural
2. **Per-primer composition instruction** — each non-Phase-3 primer's opening explicitly instructs domain-as-skill loading; primer frontmatter carries `relevant_domains` for machine-readable validation
3. **Pre-phase composition declaration** — phase-boundary commits emit `PhaseCompositionDeclared` event; absent declaration is itself a finding
4. **`check-phase-composition.py` hook** — fires at phase-boundary commits; missing declaration requires bypass-marker with rationale

---

```yaml
---
section_name: adversarial-review-stance
required: true
target_lines: 30
event_variants_referenced: [SycophancySelfAudit]
domains_referenced: []
phases_referenced: [phase-3, phase-5]
---
```

## Adversarial review stance: The Exacting Mentor

The adversarial reviewer adopts an **Exacting Mentor** stance — sustainable, multi-domain, unnamed, not personified. Primer 3 retains the whitepaper-canonical "Adversarial Refinement (The VDD Roast)" title for source-material continuity; the stance the reviewer adopts is the Exacting Mentor.

**Core stance:** You are an experienced reviewer who has seen this defect class before. You hold the work to the standard you know it can meet — because you believe the author can reach that standard, not because you're suspicious of them. Direct, specific, exacting. Don't pull punches; also explain why something is wrong + what the better version looks like + what corrective pattern applies. Sycophancy resistance is rooted in standards, not paranoia.

**Five lenses.** Every finding answers at least one:
1. **Attacker's mindset** — "If I were trying to break this, where would I attack?"
2. **Edge cases** — "What about the conditions the happy-path code skips?"
3. **Usability** — "Will the operator know what to do when they see this?"
4. **Maintainability** — "Will future-developer understand and modify this?"
5. **Consistency** — "Does this match the spec? Does the doc match the code?"

**Tone-flex policy:** Mentor voice is default; Formal voice is the exception for legally-record-style attestations + schema declarations. Mentor for per-finding bodies, round-close summaries, hook output messages on failure, sycophancy-check failure-mode descriptions. Formal for Phase 6 Exit Signal records, methodology amendments, schema definitions.

**Sycophancy compensation discipline:** when the reviewer authored the artifact under review, the review entry's frontmatter declares `sycophancy_compensation: <text>` naming what the author resisted. Mechanical via `check-sycophancy-compensation.py` hook firing on identity overlap between commit-author and review-authoring identity.

---

```yaml
---
section_name: domain-set
required: true
target_lines: 25
event_variants_referenced: []
domains_referenced: [software-engineer, quality-engineer, ux, security, solution-architect, solution-owner, platform-engineer, data-engineer, red-team, performance-engineer, technical-writer, documentation-reviewer, accessibility, privacy, localization, ai-engineer, vsdd-methodology, sanity-check]
phases_referenced: []
---
```

## Domain set

**16 role + 2 meta = 18 domains.** Each has a prompt file at `.claude/commands/vsdd-domain-<slug>.md` + a knowledge page registered via `crosslink knowledge import`.

**Core role domains (6):** Software Engineer, Quality Engineer, UX, Security, Solution Architect, Solution Owner.

**Extended role domains (10):** Platform Engineer, Data Engineer, Red Team, Performance Engineer, Technical Writer, Documentation Reviewer, Accessibility, Privacy, Localization, AI Engineer.

**Meta domains (2):**
- **VSDD Methodology** — semantic-coherence reviewer of methodology application. Surviving dimensions: spec-vs-implementation semantic alignment; methodology-spirit adherence; cross-session semantic continuity; methodology-evolution coherence. Activation: on-demand (not gate-criterion).
- **Sanity Check** — validator-of-last-resort + rubber-ducking surface. Activates automatically via hook when `validator: sanity-check` declared in a finding's frontmatter.

Domain activation per project follows the always-on baseline (SE + QE + SA + SO + PE + PerfE when shipping code) + axis-driven additions (next section). Per-domain prompt frontmatter declares `classification_universe`, `validator_pair`, `sycophancy_failure_modes`. The `check-classification-universe.py` hook validates findings classify within the domain's universe.

---

```yaml
---
section_name: per-feature-axes
required: true
target_lines: 25
event_variants_referenced: [ProjectInitialized]
domains_referenced: [documentation-reviewer, technical-writer, red-team, security, data-engineer, privacy, ux, accessibility, localization, ai-engineer]
phases_referenced: [phase-5]
---
```

## Per-feature axes

Projects declare axes in `.vsdd/config.yaml` at `vsdd init` time. Each axis activates one downstream calibration **additive over the always-on baseline** (SE + QE + SA + SO + PE + PerfE).

| Axis | Activates |
|---|---|
| `ships-to-users-other-than-developer: yes` | Documentation Reviewer + Technical Writer (default) |
| `network-exposed: yes` | Red Team + Security |
| `persists-managed-schema-data: yes` | Data Engineer |
| `handles-user-data: yes` | Privacy |
| `safety-critical: yes` | Phase 5 Mutation Testing + Purity Boundary Audit recommended |
| `formal-verification-candidates: yes` | Phase 5 Proof Execution recommended |
| `ui-surface: yes` | UX + Accessibility |
| `localized: yes` | Localization |
| `ai-runtime-cost-relevant: yes` | AI Engineer |

Each axis is independent; no tier vocabulary. Cold-session budget bands per axis-combination land in the DESIGN docs. Axis declarations emit `ProjectInitialized` event with full axis manifest.

---

```yaml
---
section_name: forward-only-disciplines
required: true
target_lines: 20
event_variants_referenced: [OperatorDirectiveApplied]
domains_referenced: []
phases_referenced: []
---
```

## Forward-only disciplines

**Event-log append-only (data discipline).** Structural property of the event log. Records are append-only; no in-place updates. `.vsdd/events.jsonl` is committed to git per cycle; `git checkout` recovers from prior commit.

**Documentation narrative-preservation (prose discipline).** Authoring discipline for the methodology spec + primers + domain prompts + supplements. The forward-only restriction applies AFTER a stability commitment fires — one of: v1.0 release; first push to a public remote; first downstream adoption; operator-declared methodology-stabilization milestone via `OperatorDirectiveApplied{directive: methodology-stabilization}`. Before any trigger fires, history is malleable (rebase, amend, drop commits; dated entries can be re-authored as discipline evolves). After the trigger: append-only forward-only applies; redactions only via append-only mechanisms; retroactive edits to dated entries become findings for the VSDD Methodology meta-domain.

---

```yaml
---
section_name: bypass-marker-mechanism
required: true
target_lines: 15
event_variants_referenced: []
domains_referenced: []
phases_referenced: []
---
```

## Bypass-marker mechanism

Hybrid form. **HTML comment** (`<!-- hook-bypass[<hook-id>]: <rationale> -->`) works mechanically across markdown rendering pipelines. **Frontmatter** (`bypass: [{hook_id, rationale, pr_approval_label}]`) works cleanly for typed artifacts. Both supported.

Rules: rationale MUST be non-empty (fires `VSDD-E0016: bypass-rationale-missing`); hook-id MUST be namespaced (`<!-- hook-bypass: ... -->` without hook-id fires `VSDD-W0070: bypass-marker-scope-mismatch`); CI merge-gate requires explicit operator approval via PR label (default `bypass-approved`) AND the label-applier MUST differ from the PR-author (self-applied-label-circumvention defense).

---

```yaml
---
section_name: two-audience-principle
required: true
target_lines: 18
event_variants_referenced: []
domains_referenced: [technical-writer, documentation-reviewer]
phases_referenced: []
---
```

## Two-audience principle

Every artifact serves humans and agents simultaneously.

| Audience | Consumes via | Enforcement |
|---|---|---|
| **Humans** (operators, contributors, future-you) | Linear prose, markdown anchors, navigable cross-references | Authoring discipline; cold-reader pair (TW ↔ DR) validation; SO scope review |
| **Agents** (cold-context reviewers + main-session orchestrators) | Greppable frontmatter, schema-validated fields, event log, crosslink typed labels | Hooks (mechanical); JSON Schema validation; observability event schema |

Role-flavor distinction (developer-extending-methodology vs user-following-methodology) is captured by the SO ↔ DR adversarial-pair pattern, not by a separate audience.

---

```yaml
---
section_name: two-cooperating-audit-trail-layers
required: true
target_lines: 18
event_variants_referenced: [FindingRaised, FindingClassified, FindingRouted, ValidationPassed, ValidationFailed, HookFired]
domains_referenced: []
phases_referenced: []
---
```

## Two cooperating audit-trail layers

**Suite-side event log:** `.vsdd/events.jsonl` — NDJSON append-only file, git-tracked per cycle, schema-validated by event-variant payload schemas. 18 methodology event variants flow here (phase lifecycle, finding lifecycle, cycle convergence, discipline-enforcement, auth + identity, project lifecycle, PR lifecycle) alongside hook + validation events.

**Crosslink-side audit trail (when crosslink is in use):** crosslink hub HTTP OTLP endpoint; event-schema-compatible records flow into crosslink's `events.rs` consumer. Both layers carry the same `EventEnvelope` shape (`agent_id`, `agent_seq`, `timestamp`, `signed_by`, `signature`, `capture_source`). Total ordering preserved. Crosslink-compatible by construction.

All event-variant schemas exclude credential-shaped fields structurally; OTel collector redacts credential-shaped values before forwarding to any external backend.

---

```yaml
---
section_name: schema-versioning
required: true
target_lines: 12
event_variants_referenced: []
domains_referenced: []
phases_referenced: []
---
```

## Schema versioning

Per-artifact-class semantic versioning. Each of the 13 artifact classes carries its own `schema_version`. Independent evolution: Review entry schema can hit 2.0.0 while Domain prompt stays at 1.0.0.

Forward-only: additions are non-breaking; deletions or renames require explicit methodology Review entry + major-version bump that consuming hooks honor. Deprecated codes carry migration pointers to replacements; never reused once retired (matches Rust's E0000-series stability discipline). Methodology spec itself version-pinned per `methodology_version` frontmatter; `check-methodology-version-drift.py` warns when project + toolkit versions diverge.

---

```yaml
---
section_name: mvr-exit-signal-convergence
required: true
target_lines: 18
event_variants_referenced: [ExitSignaled, PhaseTransitionAttested]
domains_referenced: [vsdd-methodology]
phases_referenced: [phase-3, phase-5, phase-6]
---
```

## MVR and Exit Signal convergence

**Maximum Viable Refinement (MVR)** is the per-round closure signal: an IAR cycle (Phase 3) reaches implementation-MVR when all active domains produced only Hallucinated findings (or no findings) on the final round. Per-domain MVR feeds the layer-MVR signal which gates layer-close.

**Exit Signal** is the project-terminal four-dimensional convergence (Phase 6): Spec MVR (cold SO review across final layers produced no Phase 1a/1b-routed findings); Test MVR (Phase 5 Mutation Testing per layer with per-mutant disposition); Implementation MVR (Phase 3 final round per active domain produced only Hallucinated findings); Formal-verification MVR (Phase 5 Proof Execution harnesses each have recorded outcomes OR explicitly declared not-applicable with rationale).

The cross-dimension consistency check asks whether spec + tests + implementation + formal verification agree about what the system does. Convergence record is signed (attested_by + signature over canonical attestation bytes; anonymized-project posture uses commit-sha-signed-by-anonymous-pre-commit-discipline). Closes the project at the Exit Signal.

---

```yaml
---
section_name: auth-method
required: true
target_lines: 20
event_variants_referenced: [AuthMethodDeclared, OperatorDirectiveApplied]
domains_referenced: [security, platform-engineer]
phases_referenced: []
---
```

## Auth method

Auth method is declared explicitly in `.vsdd/config.yaml` per project; **separate fields for operator-local and CI contexts**. No implicit defaults.

```yaml
auth_method:
  operator_local: plan | api_key
  operator_local_credential_source: "plan-auth-no-key" | "env:<VAR_NAME>"
  ci: api_key | "none"
  ci_credential_source: "env:<VAR_NAME>" | null
```

**Operator-local context:** Plan (Max/Pro) + Agent SDK is the default for individual-operator Phase 1a-2c skill mode + small Phase 3 cycles (1-hour prompt-cache TTL auto-enabled; Agent SDK credits separate from interactive limits). API key works for operators preferring per-token billing predictability.

**CI context:** API key (Anthropic's recommended automation auth) is the only valid option. Plan auth is **structurally rejected** by the schema validator (`VSDD-E0021: auth-method-plan-incompatible-with-ci`) — Plan requires operator-interactive session that CI runners cannot provide.

**Security disciplines:** `.vsdd/config.yaml` carries credential-source-reference only (env-var name); NEVER credential value. Schema validator rejects credential-shaped fields. Anonymization hook detects API-key formats (`sk-ant-api03-...`, generic Bearer headers). All event-variant schemas structurally exclude credential-shaped fields. OTel collector redacts credential-shaped values at the forwarding boundary. Backend bearer tokens (Honeycomb, Datadog, Grafana, Langfuse, etc.) follow the same env-var-only discipline.

---

```yaml
---
section_name: domain-change-authority
required: true
target_lines: 12
event_variants_referenced: [OperatorDirectiveApplied]
domains_referenced: [solution-owner]
phases_referenced: [phase-1a, phase-1b, phase-1c]
---
```

## Domain change authority

A single Solution Owner holds change authority over DESIGN.md across all subsystems. Per-doc primary authors may differ for cognitive ownership (e.g., SA co-authors verification-architecture sections; QE co-authors test-strategy sections) but final authority + spec-contract-change discipline rests with one SO.

**Raise to SO routing discipline:** any finding that proposes a spec-contract change routes to SO via Phase 4. The SO reviews, accepts/rejects/amends with rationale, and emits `OperatorDirectiveApplied{directive: spec-contract-amended OR spec-contract-amendment-rejected, rationale: <text>}` event. Single change authority preserves audit-trail integrity for cross-doc "Raise to SO" findings.

---

```yaml
---
section_name: closing-cross-references
required: true
target_lines: 15
event_variants_referenced: []
domains_referenced: []
phases_referenced: []
---
```

## Closing + cross-references

**Methodology authorship is dollspace's.** The canonical methodology lives in the [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) + its predecessor [VDD whitepaper](https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25) + the [crosslink repo](https://github.com/forecast-bio/crosslink). This spec is one collaborator's interpretation + mechanical operationalization.

**Cross-references:**
- [`README.md`](./README.md) — toolkit positioning + adoption flow + four governing design goals + observability subsystem overview + verification subsystem overview + schema enforcement overview + error catalog
- [`DESIGN-METHODOLOGY.md`](./DESIGN-METHODOLOGY.md) — methodology subsystem design; phase primer authoring guidelines; domain prompt authoring guidelines; cluster-batching shape; memory isolation discipline; cold-session budget bands; MCP server tool surface
- [`DESIGN-SCHEMA.md`](./DESIGN-SCHEMA.md) — type system + 13 artifact-class schemas + anchor-ID generation + bypass-marker schema + error catalog file format + schema versioning + cross-field validation
- [`DESIGN-OBSERVABILITY.md`](./DESIGN-OBSERVABILITY.md) — OTel collector design + 18 methodology event variant payloads + capture-source provenance + three pillars + dashboard ladder + FinOps + MCP server architecture
- [`DESIGN-VERIFICATION.md`](./DESIGN-VERIFICATION.md) — validator architecture + ~19 methodology hooks + CI workflow templates + bypass-marker enforcement + error catalog implementation + per-error-code falsifiability fixtures + dependency approval discipline + methodology version pin discipline
- [Claude Agent SDK](https://code.claude.com/docs/en/agent-sdk/overview) — the observability + execution substrate the toolkit composes against
- [`crosslink`](https://github.com/forecast-bio/crosslink) — the operational substrate authored by dollspace + the absorption target for toolkit patterns
