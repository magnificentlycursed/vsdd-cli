---
title: "Agent-first controls — auditing that VSDD's mechanisms are invoked at the correct times"
tags: ["design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-07-28
updated: 2026-07-28
---


## Design Specification

### Summary

The product's major goal, stated by the operator (2026-07-28): **audit that
the methodology's mechanisms — phase primers, domain-skill composition, the
design affordance, routing, phase gates, red/exit gates, governed-file pins
— were invoked at the correct times.** The Layer-3 audit gaps
(`.design/layer-3-compliance-audit.md`) are one class of failure seen from
several sides: *a required mechanism was not invoked at its correct time,
and nothing detected it but the operator.* The routing collapse
(route-before-fix not invoked), the phase-boundary informality (the gate not
invoked), and this cycle's own design-phase bypass (the phase-1a primer, the
composition declaration, and the design skill not invoked) are the same
failure.

This is a phase-1a design (composition declared on #815; authored under the
loaded phase-1a primer, Solution Owner / Solution Architect / AI Engineer
domain skills, and the `claude-code-cli` supplement, via crosslink's `design`
skill). Per operator ruling it is an **invariant-first re-sequencing +
enhancement** with **dogfood-now** self-governance and a **declared vehicle
mix**. Templates: Thermite's harness
(`.design/thermite-control-architecture-notes.md`) and mdatron v0.2.0's
shipped capabilities (#816). The design's keystone is the
mechanism-invocation-timing audit; every other control is an instance of it
plus the trace it needs. This draft was itself re-entered through the
mechanism after a hand-rolled bypass — that bypass is the first acceptance
case, not a footnote.

### Requirements

- REQ-1: **The mechanism-invocation-timing audit is the keystone control.** The system audits, from the traces mechanisms leave, that each governed act invoked its required mechanism at its correct time — for example: phase entry declared its composition (the `VSDD-E0050` phase-composition-not-declared condition); a design or spec document was authored through the design affordance under a loaded phase primer; a review round's findings were routed before any fix-close; a phase transition presented its gate artifact; a governed-file edit was preceded by the read-gate. A required invocation that is absent at its correct time is a detected finding, never silent. The other requirements are its instances and the traces they need.
- REQ-2: **The trace substrate makes invocation auditable.** Each mechanism leaves an auditable trace, and the audit reads them: the composition declaration and phase events (`.vsdd/events/`, which already holds a June composition event — the precedent), the action-time hook invocation log, crosslink typed comments and dependency edges, git history and boundary commits, and mdatron's pin/verify records. Where a mechanism leaves no trace, adding the trace is part of the control — an invocation that records nothing cannot be audited.
- REQ-3: **Routing-before-fix, mechanized and structural** (the unrouted-findings, route-after-fix, and narrated-routing gaps). A finding — from a discovery round or a verify round — is not fixable until it carries a pinned reproduction and a routing to the phase that owns its prevention, recorded as a crosslink dependency edge (not prose); a fix-close without a prior routing is malformed. Available as a mechanized bootstrap form now, not deferred to the Layer-6 gate. Template: the tool-restricted critic that lacks `Edit` and must pin a failing test + file a blocker before any fixer acts.
- REQ-4: **Phase-transition gates on their artifacts** (the phase-boundary gap). Each boundary requires its named gate artifact — a recorded red-gate failure before phase-2b entry, an exit-gate record before advancing — self-applied during bootstrap. Template: the read-gate entry gate + the artifact-set-green-in-one-run exit gate.
- REQ-5: **Governed design/spec-phase entry** (the design-phase-bypass gap, this cycle's own). A design or spec authoring act is governed: its phase primer was loaded, its composition declared, the design affordance used, and its pipeline state and knowledge page present — checked, so a hand-rolled design cannot proceed ungoverned. This is the direct control for the bypass recorded in the audit addendum.
- REQ-6: **Round and finding closure discipline** (the round-closure gap). A check fails if a review round is open without a closing artifact or a tracked blocker with remaining scope; no silenced-but-open state.
- REQ-7: **Per-layer phase-5 gate** (the per-layer-phase-5 gap, #814 ruling). Each verified layer attaches a mutation score against the kill floor and a non-vacuity check over its pure core; survivors and vacuous properties fail the gate. Records the Layers 1/2 retroactive decision.
- REQ-8: **Self-governance, dogfooded now** (the self-governance meta-gap). vsdd-cli runs its own controls over its own construction as early as possible: it self-hosts the state artifact (`.vsdd/state.yaml`) so `vsdd status` reads real state here, and the earliest controls ship as dependency-free scripts that are themselves registered, routed, and pinned with their own tests — a cheap tripwire first, a typed registry next. Template: the stdlib-only self-checking tooling; the two-tier status-tripwire → typed-registry bootstrap.
- REQ-9: **Each control declares its vehicle**, from the mix (action-time Claude Code hook + Rust mirror; CI/commit gate; crosslink-native edges and integrity queries; mdatron conformance), honoring **fail-closed** (an environment failure is a distinct outcome, never a pass) and the **mechanical-versus-adversarial split** (regex/structure-gate the unambiguous; route judgment to a tool-restricted adversarial reviewer).
- REQ-10: **mdatron v0.2.0 is leveraged** (#816) where it fits: `pin --update` for the governed-file/read-gate trace and the amendment discipline; `verify --compact` for hook-budget conformance output; per-route `citations: true` for evidence-gated citations; `numeric_claims` for the prose-restatement rule; `routes.yaml` for jurisdiction. Its breaking change (verify refuses a tree with no `.mdatron/config.yaml`) is an integration precondition, sequenced where a control depends on it.
- REQ-11: **Naming/register is a control, not a manual guard.** The coinage/register check (mdatron's register family, strengthened in v0.2.0) is placed with a vehicle; the letter-cluster pattern recurred inside this cycle's own audit (corrected 2026-07-28) — a manual guard that keeps failing is the argument for mechanizing it.

### Acceptance Criteria

- [ ] AC-1: The Architecture holds the unifying frame — every audit gap mapped to (the mechanism it failed to invoke, its correct time, the trace that would detect the omission, whether that trace exists today). (REQ-1)
- [ ] AC-2: The trace substrate is enumerated with, per mechanism, the trace it leaves and whether it must be added. (REQ-2)
- [ ] AC-3: This session's three bypasses (routing collapse, phase-boundary, design-phase) are named as acceptance-test cases the product should flag, each with the trace and the detecting control. (REQ-1, REQ-3, REQ-4, REQ-5)
- [ ] AC-4: The routing, phase-gate, governed-design-entry, round-closure, and phase-5 controls each specify their mechanism, bootstrap form, and eventual layer form. (REQ-3..REQ-7)
- [ ] AC-5: The self-governance requirement specifies the self-hosted state artifact and the dependency-free self-checked control scripts, with the earliest layer each can land. (REQ-8)
- [ ] AC-6: Every control names its vehicle and honors fail-closed + the mechanical/adversarial split. (REQ-9)
- [ ] AC-7: Each mdatron-dependent control names its v0.2.0 capability and the config.yaml precondition. (REQ-10)
- [ ] AC-8: The register/naming control is placed with a vehicle. (REQ-11)
- [ ] AC-9: The design states what amends the contract (the decomposition re-sequencing; any new normative control) versus new build, drafts the Revision-line entry, and is register-clean, verified by the Documentation Reviewer and Technical Writer lenses at cold review. (all)

### Architecture

### The unifying frame: every gap is a mechanism-invocation-timing failure

| Gap | Mechanism not invoked | Correct time | Detecting trace | Trace exists today? |
|---|---|---|---|---|
| Unrouted findings | route-before-fix | before a fix-close | routing `plan` + dependency edge on the finding | partial (comments; no edge) |
| Narrated routing | structural routing | at routing | dependency edge | no (prose only) |
| Route-after-fix | route-before-fix (verify) | before the verify fix | same | partial |
| Phase-boundary informal | phase-transition gate | at each boundary | red-gate / exit-gate record in state | partial |
| Round-closure | round closure | at round close | round-parity + lifecycle disposition | partial (L6/L7) |
| Per-layer phase 5 | phase-5 gate | at layer verify | mutation score + non-vacuity artifact | no |
| Design-phase bypass | primer + composition decl + design affordance | at phase entry | composition declaration + pipeline/knowledge | partial (VSDD-E0050 contracted, unbuilt) |
| Self-governance | the tool over itself | throughout | self-hosted state + hook/CI logs | no (not self-hosted) |
| Naming/register | register check | at authoring | mdatron register finding | partial (mdatron v0.2.0) |

The pattern: the right mechanisms exist or are contracted, but they either
leave no auditable trace or their trace is checked only in the back-third
layers. The keystone control reads the traces; the instance controls make
the missing traces exist and pull the cheapest checks early.

### This session's bypasses as acceptance cases

Three real failures this session are the product's own regression fixtures:
the six-round routing collapse; the phase-compressed rebuild; and the
design-phase bypass (this cycle). For each, the product should answer "which
mechanism went un-invoked, and where is the trace that proves it" — and flag
it. If the built controls would not have flagged these, they are
insufficient by construction.

### Re-sequencing (invariant-first) and the controls

The cheapest enforceable form of each control — a trace + a script that
reads it — is dependency-light and belongs at the next layer's phase-3
onward, not Layer 6/7. The heavy forms (full gate commands, mdatron
conformance families) stay at their true-dependency layers. Each control's
mechanism, vehicle (REQ-9), Thermite template, and mdatron leverage (REQ-10)
are specified per requirement; the self-governance requirement (REQ-8) is
the keystone's enabler — the tool must run over itself for the audit to be
real rather than operator-performed.

### The AI Engineer cost dimension

The audit's cheap leg is trace-reading (pure/stdlib, near-zero model cost);
the expensive leg is the adversarial reviewer dispatches. Cost-band them,
right-size the model tier (mechanical trace checks on pure code or
Haiku-class; adversarial refinement on Opus/Fable), and batch within the
cache window. The self-governance scripts are dependency-free by design —
part of why they are the correct bootstrap form.

### What amends the contract vs. what is new build

Amends the contract (decomposition): the invariant-first re-sequencing of
the control forms earlier; the two unowned gaps (structural routing;
self-governance) given owners; the governed-design-entry control; the
per-layer phase-5 gate; the mechanism-invocation-timing audit named as a
Status/Conformance surface. New build (a ratification drives): the trace
substrate additions, the bootstrap control scripts, the self-hosted state
artifact, the crosslink-edge routing check, the mdatron-vehicle wiring.

### Out of Scope

- The build itself — this is the design; ratification drives the build.
- mdatron's own v1.0 work; the reciprocal cold-read favor is separate (#816) and needs a genuinely fresh session.
- Rounds 1–5 re-routing — forward-only; the deviation stands.
- Re-opening closed milestones — new controls land as tracked increments under their boundaries (the #811 precedent), not reopens.

