---
primer_id: vsdd-phase-4
phase: phase-4
version: 0.1.0
frequency: per Phase 3 round
governing_skill: true
relevant_domains: []
supplements_in_scope: []
---

# Phase 4 Primer: Feedback Integration Loop

## Composition

Phase 4 is **operator-orchestrated routing** — no specific domain composition. The operator (or a single agent with the routing primer loaded) walks Phase 3's classified findings and routes each to the earliest phase that can fix it correctly.

Always-on baseline still applies for routing-coordination but no domain is the "owner" of Phase 4 work itself.

## Phase-specific discipline

Phase 4 closes the loop from Phase 3 back to earlier phases. Per the routing table:

| Finding signal | Trace | Route to |
|---|---|---|
| Behavior undefined in DESIGN.md; implementation guessed | Spec gap | **Phase 1a + 1b** |
| Edge case in DESIGN.md but no test covers it | Test discipline gap | **Phase 2a** |
| Test exists but passes against empty body | Test quality gap | **Phase 2a** (rewrite to fail-against-stub) |
| Layer's acceptance criteria don't cover the failed behavior | Decomposition gap | **Phase 1c** |
| Implementation diverges from correctly-specified + correctly-tested behavior | Implementation defect | **Phase 2b** |
| Refactor regressed clarity | Refactor regression | **Phase 2c** (re-refactor or back out) |
| Property-based counterexample / surviving mutant / fuzzer crash / failing proof | Hardening gap | **Phase 5** |
| Spec/test/impl/formal inconsistency | Convergence gap | **Phase 6** (route the inconsistent dimension's destination) |
| Architectural concern crosses layers DESIGN.md didn't anticipate | Spec architecture gap | **Phase 1a + 1b** (may force re-decomposition) |
| Suite gap (adversary couldn't have caught this) | Suite gap | **Suite-development** (file at suite's FINDINGS-INDEX; not a project phase) |
| Process gap (Red Gate skipped, layer merged without IAR) | VDD-IAR Alignment finding | **Phase 4 itself** (document the deviation) |

## The Exacting Mentor stance applied to routing

**Primary failure mode:** Routing every finding to Phase 2b ("the implementation is what's wrong"). This collapses the VSDD pipeline into a single phase. Signal: spec hasn't changed in N rounds; test plan hasn't changed in N rounds; codebase keeps churning. Fix: re-examine the finding set + ask "of these findings, which are *actually* implementation defects, and which are spec or test defects misclassified as implementation?"

**Routing question for each finding:** "What artifact, had it been correct, would have prevented this finding?" That artifact's owning phase is the route. If the answer is "the implementation" — verify, because that's the easy answer + the failure mode. Ask one level up: "what artifact would have caused the implementation to be correct?" If the answer is "a more complete spec" or "a test that asserted this" — route up.

**Multi-phase routes:** A finding routing to `1a → 2a → 2b` (spec gap + test gap + implementation defect) is the correct shape; recording it as `2b only` is the failure mode.

## Pre-phase composition declaration template

```yaml
phase: phase-4
composed_domains: []
composition_mode: operator-orchestrated
operator_confirmation: confirmed
declared_at: <ISO 8601 timestamp>
```

## Routing output

For each routed finding, record:
- **Finding ID** (from Phase 3 review log)
- **Route** — `1a`, `1c`, `2a`, `2b`, `2c`, `5`, `6`, `Suite`, or multi-phase chain
- **Owning artifact** — `DESIGN.md` / `tests/<file>` / `src/<file>` / `suite-development/FINDINGS-INDEX.md`
- **Gate** — what must be true before the routed work is done at that phase
- **Sequencing** — does this route block the next layer? Block merge? Defer to named future layer?

Emit `FindingRouted{finding_id, target_phase, target_artifact}` per finding.

## Phase-completion criteria

Phase 4 closes when:
- Every real finding from Phase 3 has a recorded route
- Every route names the gate at each phase
- Every blocking relationship is recorded
- Suite findings filed at `suite-development/FINDINGS-INDEX.md`, not collapsed into project-phase routes
- Proportion routed to Phase 2b matches reality (if every finding routes to 2b, re-run the routing pass with spec-defect bias check)

Emit `PhaseExited{phase: phase-4, exit_status: complete, layer: <N>, routed_count: <N>}`. The next pass begins (re-enter the routed phase's primer).

## Cross-references

- [Phase 3 primer](./vsdd-phase-3.md) — Adversarial Refinement (produces the findings Phase 4 routes)
- All earlier-phase primers — routing destinations
- [methodology.md § MVR and Exit Signal convergence](../../methodology.md#mvr-and-exit-signal-convergence) — the IAR refinement loop continues until MVR
