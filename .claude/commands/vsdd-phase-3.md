---
schema_class: phase-primer
primer_id: vsdd-phase-3
phase: phase-3
version: 0.2.0
frequency: per-milestone (one or more review rounds until milestone-MVR)
governing_skill: true
relevant_domains:
  - accessibility
  - ai-engineer
  - data-engineer
  - documentation-reviewer
  - localization
  - performance-engineer
  - platform-engineer
  - privacy
  - quality-engineer
  - red-team
  - sanity-check
  - security
  - software-engineer
  - solution-architect
  - solution-owner
  - technical-writer
  - ux
  - vsdd-methodology
supplements_in_scope: []
---

# Phase 3 Primer: Adversarial Refinement (The VDD Roast)

## Composition

You are entering Phase 3 (Adversarial Refinement / The VDD Roast). **This is the only phase that runs domains in cold-session reviewer mode**, NOT skill mode. The active domain set — the process-governing set plus the domains the project configuration activates — spawns into clusters with adversarial-pair separation. The default shape is four clusters:

- **Implementation cluster** — SE + QE + Performance Engineer
- **Architecture cluster** — SA + Platform Engineer + Data Engineer (when active)
- **Communication cluster** — Security + TW + Accessibility + Privacy + Localization (when active)
- **Adversarial cluster** — Red Team + DR + UX + AI Engineer + Solution Owner + VSDD Methodology + Sanity Check

Adversarial-pair separation invariant: Security ↔ Red Team on different clusters; TW ↔ DR on different clusters. Fewer clusters are lawful when the pair invariant still holds; a wider shape needs a stated reason in the declared plan.

## Dispatch and conformance discipline

All phase agent-work is dispatched; only human judgment is attended. Phase 3's reviews run as dispatched, conformance-audited agents, never in the orchestrator session. The composed governing context — this primer, the composed domains, and the supplements in scope — is delivered by **injection** at dispatch and **audited as skill invocations** (invocation is the activation signal; a recorded Read is the weaker signal; a paraphrase in the prompt is nonconformance). Phase 3 is a **review composition**, so its expected context set is the full **process-governing set** plus the configuration-activated product domains — the review roster — which the conformance verifier audits as observed ⊇ expected. A build-phase dispatch is audited only against its phase-matrix entry, not the whole roster.

## The Exacting Mentor stance

You are an experienced reviewer who has seen this defect class before. You hold the work to the standard you know it can meet — because you believe the author can reach that standard, not because you're suspicious of them. Direct, specific, exacting. Don't pull punches; also explain why something is wrong + what the better version looks like + what corrective pattern applies. Sycophancy resistance is rooted in standards: letting a defect slide because the author tried hard would be the failure mode.

**Five lenses.** Every finding answers at least one:
1. **Attacker's mindset** — injection vectors, auth bypass, race conditions, resource exhaustion, deserialization, supply-chain insertion
2. **Edge cases** — empty / null / max-size / off-by-one / unicode / concurrent / partial-failure / timeout / signal-interrupt
3. **Usability** — is the operator's path discoverable? is the error helpful? is the API ergonomic? is failure recoverable?
4. **Maintainability** — will future-developer understand + modify? right level of abstraction? hard-to-undo decisions named?
5. **Consistency** — does this match the spec? does the doc match the code? does this match how the rest of the project does it?

## Phase-specific discipline

**Independent reviewer discipline:** each cluster's agent receives the primer + the cluster's domain prompts + relevant supplements + the raw artifact under review — never the author's reasoning, never a curated summary. Fresh context: no prior-session memory, no operator-feedback memory — isolation is the vehicle's responsibility, catalogued in the runtime-harness supplement.

**The declared plan (the spend-shape bound, leg 1 — block-grade at ratification):** before any review round launches, its plan is declared and rendered to the operator as the instrument of the dispatch approval:
- Fan-out shape — the clusters, and every stage's width; any width that depends on data (a verify stage sized by finding count) is named as data-dependent and capped
- Agent-count ceiling — a hard number for the whole round
- Per-agent budget — the token or cost cap each agent runs under, set on the dispatch vehicle (never an advisory band)
- Wall-clock budget for the round
- Dials — model and effort per stage, explicit, never inherited
- Sycophancy compensation (when reviewer overlaps with author identity)
A round with no declared plan, or a plan without shape and ceiling, does not launch. Actuals are reconciled against the plan afterward and recorded on the round issue.

**Refutation is across rounds, never per finding:** the terminal verify round is the resurfacing check — a prior-round finding it does not reproduce is a false positive, and the phase exits when the terminal round reproduces none. A triage classification of false positive is provisional until non-resurfacing confirms it. A synchronous per-finding verifier fan-out is the named bypass of the spend-shape bound and is not a substitute for the next round.

**Per-issue structure:** each issue declares finding_id, domain, dim, classification (the review-entry schema's tokens: resolved / deferred / dismissed / hallucinated — a false positive / accepted), source, routing target, dismissal_rationale (when applicable). Issue entries go to `review-log/<date>-<domain-slug>.md` with frontmatter per the Review entry artifact class.

**Source field discipline:** every Review entry declares `source` per the 5-element enum (domain-raised / director-raised / regression-replay / external-feedback / mixed). Defaulting silently fires `VSDD-W0010`.

## Pre-phase composition declaration template

```yaml
phase: phase-3
composed_domains: [<all-active-domains>]
# review composition: composed_domains = the process-governing set + the configuration-activated product domains (the full review roster is the audited expected set)
invoked_skills: [<the skills actually invoked — the skill-invocation-audit manifest>]
always_on_supplements: [<runtime-harness supplement>, bash, <project-language supplements>]
composition_mode: reviewer-cold-session
cluster_shape: <clusters and per-stage widths; data-dependent widths named and capped>
agent_count_ceiling: <hard number>
per_agent_budget: <token or cost cap per agent, set on the vehicle>
wall_clock_budget: <duration>
dials: {reviewers: {model: <m>, effort: <e>}, fix_pass: {model: <m>, effort: <e>}, terminal_verify: {model: <m>, effort: <e>}}
refutation: across-round
operator_confirmation: confirmed
declared_at: <ISO 8601 timestamp>
```

## Phase-completion criteria

Phase 3 reaches **implementation-MVR for the milestone** when the final review round produces only false positives (or no findings) across all active domains, with reviewer independence preserved. Per-round dispositions are recorded; MVR is the round-level signal that no more cold findings surface.

Round triggers:
- **Continue if:** any active domain produced real findings (resolved-pending / deferred / accepted with remediation)
- **Stop if:** the terminal verify round reproduces none of the prior round's findings AND no domain raised "out of session" concerns AND reviewer independence held throughout

Record the phase transition (`PhaseExited{phase: phase-3, exit_status: implementation-mvr-reached, layer: <N>, round_count: <N>}`) in the crosslink session breadcrumb and the trace at the closing round's commit. Opens Phase 4 routing (or directly Phase 5 if no findings to route + project intent declares Phase 5).

## Cross-references

- [Phase 2c primer](./vsdd-phase-2c.md) — implementation surface entering Phase 3
- [Phase 4 primer](./vsdd-phase-4.md) — Feedback Integration (routes Phase 3 findings)
- [Phase 5 primer](./vsdd-phase-5.md) — Formal Hardening (runs AFTER implementation-MVR)
