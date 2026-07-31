---
schema_class: phase-primer
primer_id: vsdd-phase-2b
phase: phase-2b
version: 0.1.0
frequency: per-milestone
governing_skill: true
relevant_domains: [software-engineer, quality-engineer, technical-writer, documentation-reviewer, platform-engineer]
supplements_in_scope: []
---

# Phase 2b Primer: Minimal Implementation

## Composition

You are entering Phase 2b (Minimal Implementation). Per the phase-domain composition matrix, load:

- **Software Engineer** — primary; owns implementation
- **Quality Engineer** — test-pyramid maintenance during implementation
- **Technical Writer** — prose-surface updates (README, DESIGN, manual-tests stay coherent during implementation)
- **Documentation Reviewer** — cold-reader pass on prose updates
- **Platform Engineer** — when Phase 2b adds a dependency, PE artifact (lockfile, audit gate, env pin) lands in the same commit

Plus the core always-on quartet + per-feature-axes-activated domains (DE / AI Engineer / etc. per project axes). Skill mode.

## Dispatch & conformance discipline

Phase 2b's agent-work — authoring **and** implementation — runs as a **dispatched, conformance-audited agent**, never in the orchestrator session (the phases-dispatched keystone; supersedes the 2026-07-20 attended/autonomous split: human-judgment work is attended, all phase agent-work is dispatched). The composed governing context — this primer, the composed domains, and the supplements in scope — is delivered by **injection** at dispatch and **audited as skill invocations** (the skill-invocation audit: invocation is the activation signal; a recorded Read is the weaker signal; a paraphrase in the prompt is nonconformance). For a build-phase dispatch the composition SHOULD is this phase's matrix entry — the phase primer, its composed domains, the **core always-on quartet** (SO + SA + SE + QE; PE + PerfE when the project ships code), and the axis-activated product domains — which the conformance verifier audits as `WAS ⊇ SHOULD`. The full **process-governing set** (the eleven process-governing domains) is the audited SHOULD for **review compositions** (Phase 3), not for every build dispatch: wiring the whole set into a per-build gate would force every build dispatch to load all of it, against the efficiency thesis and cold-review independence.

## Phase-specific discipline

Phase 2b implements **minimal code to turn the Red Gate green**. Minimal means: the smallest implementation that passes the Phase 2a tests without violating any methodology disciplines.

The Exacting Mentor stance applies: "implementation that adds features not asserted by tests" is the failure mode (Phase 2b is not the place for speculative features); "implementation that introduces complexity not named in the spec" is the failure mode (refactor that work to Phase 2c).

Per-commit discipline:
- **Red → Green transitions are commit-level visible** — each commit closes a named subset of failing tests; `cargo test` regression-checks at each commit
- **No "implementation drift"** — implementation matches DESIGN.md § Behavioral contracts; if the implementation needs to do something the spec doesn't assert, route to Phase 4 → Phase 1a+1b
- **Dependency additions trigger the dependency approval discipline** — new entries in `Cargo.toml` / `package.json` / `pyproject.toml` / `requirements.txt` require SO + PE + Security investigation + corresponding `docs/dependencies/<crate>.md` entry (per the methodology amendment dated 2026-05-27)
- **Prose-surface updates compose with TW + DR** — every commit touching `README.md` / `DESIGN.md` / `manual-tests/` / `PROCESS.md` carries `Co-authored-by: Technical Writer <tw@vsdd-domains>` + `Co-authored-by: Documentation Reviewer <dr@vsdd-domains>` trailers
- **CHANGELOG entries land per commit** (when crosslink is in use, `crosslink close` auto-manages; otherwise per-commit Keep-a-Changelog entries)

## Pre-phase composition declaration template

```yaml
phase: phase-2b
composed_domains: [software-engineer, quality-engineer, technical-writer, documentation-reviewer, platform-engineer, ...axes-activated]
# audited SHOULD for this build dispatch = the phase-matrix entry (these composed_domains + the core always-on quartet + the supplements in scope); the full process-governing set is the audited SHOULD only for review compositions (Phase 3)
invoked_skills: [<the skills actually invoked — the skill-invocation-audit manifest>]
always_on_supplements: [claude-code-cli, bash, rust]
composition_mode: skill-interactive
operator_confirmation: confirmed
declared_at: <ISO 8601 timestamp>
```

## Phase-completion criteria

Phase 2b closes when:

- All Phase 2a Red Gate tests pass (`cargo test` returns 0)
- No new behavior in the implementation lacks a spec assertion
- Dependency additions have corresponding `docs/dependencies/<crate>.md` investigations + SO + PE + Security approval trailers
- Prose surfaces (README, DESIGN, manual-tests, CHANGELOG) stay current per the TW + DR composition

Record the phase transition (`PhaseExited{phase: phase-2b, exit_status: complete, layer: <N>}`) in the crosslink session breadcrumb and the harness run record at the closing commit. Opens Phase 2c.

## Cross-references

- [Phase 2a primer](./vsdd-phase-2a.md) — Red Gate (consumed)
- [Phase 2c primer](./vsdd-phase-2c.md) — Refactor (next)
- [Software Engineer domain](./vsdd-domain-software-engineer.md)
- [Platform Engineer domain](./vsdd-domain-platform-engineer.md) — dependency approval discipline
- [README § Per-milestone PR discipline](../../README.md#per-milestone-pr-discipline) — PR accumulates Phase 2b commits
