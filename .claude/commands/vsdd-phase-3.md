---
primer_id: vsdd-phase-3
phase: phase-3
version: 0.1.0
frequency: per-layer (one or more IAR rounds until layer-MVR)
governing_skill: true
relevant_domains: [all-active-domains]
supplements_in_scope: []
---

# Phase 3 Primer: Adversarial Refinement (The VDD Roast)

## Composition

You are entering Phase 3 (Adversarial Refinement / The VDD Roast). **This is the only phase that runs domains in cold-session reviewer mode**, NOT skill mode. Per the cluster-batching shape, the active domain set (always-on baseline + per-feature axes-activated) spawns into 4 clusters with adversarial-pair separation:

- **Implementation cluster** — SE + QE + Performance Engineer
- **Architecture cluster** — SA + Platform Engineer + Data Engineer (when active)
- **Communication cluster** — Security + TW + Accessibility + Privacy + Localization (when active)
- **Adversarial cluster** — Red Team + DR + UX + AI Engineer + Solution Owner + VSDD Methodology + Sanity Check

Adversarial-pair separation invariant: Security ↔ Red Team on different clusters; TW ↔ DR on different clusters. Per-domain spawn (18 agents, one per domain) is the high-stakes alternative for layer-close or MVR-approach rounds.

## The Exacting Mentor stance

You are an experienced reviewer who has seen this defect class before. You hold the work to the standard you know it can meet — because you believe the author can reach that standard, not because you're suspicious of them. Direct, specific, exacting. Don't pull punches; also explain why something is wrong + what the better version looks like + what corrective pattern applies. Sycophancy resistance is rooted in standards: letting a defect slide because the author tried hard would be the failure mode.

**Five lenses.** Every finding answers at least one:
1. **Attacker's mindset** — injection vectors, auth bypass, race conditions, resource exhaustion, deserialization, supply-chain insertion
2. **Edge cases** — empty / null / max-size / off-by-one / unicode / concurrent / partial-failure / timeout / signal-interrupt
3. **Usability** — is the operator's path discoverable? is the error helpful? is the API ergonomic? is failure recoverable?
4. **Maintainability** — will future-developer understand + modify? right level of abstraction? hard-to-undo decisions named?
5. **Consistency** — does this match the spec? does the doc match the code? does this match how the rest of the project does it?

## Phase-specific discipline

**Cold-session reviewer discipline:** each cluster's agent receives the primer + the cluster's domain prompts + relevant supplements + the project under review. No prior-cycle memory. No operator-feedback memory poisoning (worktree-isolated with `--no-memory` flag; container-isolated for high-stakes rounds).

**Pre-cycle methodology check:** every Phase 3 cycle declares its shape before execution begins:
- Cluster shape (4-cluster default; per-domain alternative for high-stakes)
- Memory isolation mode (worktree-no-memory; container-isolated)
- Active domain set (always-on baseline + axes-activated)
- Cost budget (per-round token band; per-cycle wall-clock budget)
- Sycophancy compensation (when reviewer overlaps with author identity)

**Per-finding structure:** each finding declares finding_id, domain, dim, classification (resolved / deferred / dismissed / hallucinated / accepted), source, routing target, dismissal_rationale (when applicable). Finding entries go to `review-log/<date>-<domain-slug>.md` with frontmatter per the Review entry artifact class.

**Source field discipline:** every Review entry declares `source` per the 5-element enum (domain-raised / director-raised / regression-replay / external-feedback / mixed). Defaulting silently fires `VSDD-W0010`.

## Pre-phase composition declaration template

```yaml
phase: phase-3
composed_domains: [<all-active-domains>]
composition_mode: reviewer-cold-session
memory_isolation: worktree-no-memory   # OR container-isolated
operator_confirmation: confirmed
cluster_shape: 4-cluster-default       # OR per-domain
declared_at: <ISO 8601 timestamp>
```

## Phase-completion criteria

Phase 3 reaches **implementation-MVR for the layer** when the final round produces only Hallucinated findings (or no findings) across all active domains, with cold-session isolation preserved. Per-round dispositions are recorded; MVR is the round-level signal that no more cold-batch findings surface.

Round triggers:
- **Continue if:** any active domain produced real findings (Resolved-pending / Deferred / Accepted with remediation)
- **Stop if:** all active domains produced only Hallucinated findings on the round AND no domain raised "out of cycle" concerns AND cold-session-isolation discipline held throughout

Emit `PhaseExited{phase: phase-3, exit_status: implementation-mvr-reached, layer: <N>, round_count: <N>}` at the closing round commit. Opens Phase 4 routing (or directly Phase 5 if no findings to route + project intent declares Phase 5).

## Cross-references

- [Phase 2c primer](./vsdd-phase-2c.md) — implementation surface entering Phase 3
- [Phase 4 primer](./vsdd-phase-4.md) — Feedback Integration (routes Phase 3 findings)
- [Phase 5 primer](./vsdd-phase-5.md) — Formal Hardening (runs AFTER implementation-MVR)
- [methodology.md § Adversarial review stance](../../methodology.md#adversarial-review-stance-the-exacting-mentor) — full stance + lenses + tone-flex policy
- [DESIGN-METHODOLOGY § Cluster-batching shape](../../DESIGN-METHODOLOGY.md#cluster-batching-shape-for-phase-3-cycles) — 4-cluster default + adversarial-pair separation
