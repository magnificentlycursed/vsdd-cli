---
title: "Agent-first controls — auditing that VSDD's mechanisms are invoked at the correct times"
tags: ["design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-07-28
updated: 2026-07-29
---

# Feature: Agent-first controls — auditing that VSDD's mechanisms are invoked at the correct times

## Summary

The product's major goal (operator, 2026-07-28): audit that the
methodology's mechanisms — phase primers, composition declaration, the
design affordance, routing, phase gates, governed-file pins — were invoked
at the correct times. The Layer-3 audit gaps are one failure class: a
required mechanism was not invoked at its correct time, and nothing
detected it but the operator.

This is a phase-1a design (composition declared on #815; authored under the
loaded phase-1a primer, Solution Owner / Solution Architect / AI Engineer
domain skills, and the claude-code-cli supplement, via crosslink's design
skill). It is an invariant-first re-sequencing + enhancement amendment,
with the tool running its own controls over its own construction as early as
possible, and a declared per-control vehicle mix.

This is the v2 draft, revised after a unanimous 7-lens cold review
(7/7 revise-before-ratify) and operator triage. The review reshaped it: the
trace source moves off the retired event store onto the contract's
derived-view model and gains a tamper-evidence requirement; the primary
control gets one home (a Status aggregate that consumes conformance records
as inputs, never a second home); a fabricated diagnostic code — asserted
"contracted" when it is nowhere in the contract — is removed (that
fabrication, caught by four lenses inside the very design meant to catch it,
is now recorded as an acceptance case); routing is reconciled with the
ratified plan-comment mechanism (the structural edge augments it, never
replaces it); the now-built controls are detection-only (the blocking form
needs a later layer); the mdatron upgrade is sequenced separately (#816);
and a runtime-cost budget becomes a requirement, not prose.

*Register note: "primary control", "the traces the audit reads",
"self-hosting" and similar are orientation phrases for this draft, not
contract terms; any text proposed for the contract uses plain leading names
and plain description, per the contract's register.*

## Requirements

- REQ-1: **The invocation-timing audit is one control with one home.** It is
  a Status process-integrity aggregate: it reads the traces mechanisms
  leave and reports, per governed act, whether the act's required mechanism
  was invoked at its correct time — a missing invocation is an integrity
  finding that never degrades the answer, exactly like Status's existing
  process-integrity queries. It is an **aggregate over the instance
  controls' outputs and consumes mdatron conformance records as inputs** (it
  never co-owns the conformance home — the Layer-2-query-consumed-by-a-gate
  pattern #810 established, avoiding the double-home #810 corrected). It is
  not the organizing frame (that is the Architecture table, AC-1) and not a
  superset that subsumes routing, gates, or naming — those are separate
  controls whose outputs it reads.

- REQ-2: **The traces are derived-view and tamper-evident.** The audit reads
  the contract's derived-view record model — a query over boundary commit
  history, milestone closes, typed result comments, and the state
  artifact's active-composition field — **never a store beside the records**
  (the `.vsdd/events/` store is retired by the contract's derived-view
  ruling; this design does not resurrect it). Every trace the audit trusts
  must be **tamper-evident by construction** — git history and the tracker's
  own integrity, not an agent-editable log — so the audit cannot be defeated
  by a self-authored or edited trace. Where a mechanism leaves no
  derived-view trace, adding a non-forgeable trace is that control's own
  sub-requirement; a trace an actor can forge is not an admissible trace.

- REQ-3: **Routing-before-fix augments the ratified mechanism.** The routing
  *is* the ratified `plan` comment (contract Finding lifecycle; the #810
  Gates format-carry) — this requirement does not redefine it. It adds a
  **structural dependency edge** linking a routed finding to its resolving
  issue (so routing enters the dependency graph and is machine-checkable),
  and **detection** of a fix-close lacking a filed routing. Detection
  checks routing *presence* and that the edge *resolves* — not routing
  correctness, which stays the reviewer's adversarial job (the
  mechanical-versus-adversarial split, REQ-8). The blocking form lives at
  its true-dependency layer (the install+hook seam, Layers 4/7, or the Layer
  6 gate); the now-built form is detection only (per the dogfood decision).

- REQ-4: **Phase transitions are gated on their artifacts.** Each boundary
  requires its named gate artifact — a recorded red-gate failure before
  phase-2b entry, an exit-gate record before advancing — read from the
  derived-view traces (REQ-2), self-applied during bootstrap.

- REQ-5: **Design/spec-phase entry is governed.** A design or spec authoring
  act is checked: its phase primer was loaded, its composition **declared**
  (the phase-composition-not-declared condition — a diagnostic the owning
  phase allocates; this design does not mint or assert a code for it), the
  design affordance used, and its pipeline state and knowledge page present.
  Presence is not correctness — a self-authored wrong-composition declaration
  passes a presence check — so the mechanical presence check is paired with
  the adversarial composition-correctness pass (REQ-8).

- REQ-6: **Per-layer phase-5 gate.** Each verified layer attaches a mutation
  score against the declared kill floor and a **non-vacuity** check over its
  pure core, where non-vacuity is defined and mechanized: a property is
  vacuous if it simplifies to a constant true, never references the result,
  or is implied by its own precondition (the vacuity battery); vacuous
  properties and surviving mutants fail the gate like a failing test.
  Records the Layers 1/2 retroactive disposition (Resolved decisions).

- REQ-7: **Self-governance, the tool over its own construction.** The
  now-built increment is three pieces, each mapped to a contract layer with a
  falsifiable done condition: (a) **self-host the state artifact** — a
  Data-authoring + Layer-1 act (author `.vsdd/state.yaml` for this repo; done
  when `read_state` accepts it), consumed by Layer 2 so `vsdd status`
  answers on this repo; (b) a **trace-audit detection** member — a named
  Layer-2 process-integrity query (done when it flags a seeded
  un-invoked-mechanism trace and passes a clean one); (c) **routing
  detection** — non-blocking, the #810 carry's mechanized read (done when it
  flags a seeded unrouted fix-close). Blocking forms are placed at their true
  layers, not built now. The requirements registry is crosslink primary
  (milestones/issues) with a thin status-consistency lint over the crosslink
  data. The scripts are dependency-light (they shell out to crosslink, git,
  and mdatron — **zero model cost, not dependency-free**) and are themselves
  registered, routed, and pinned with their own tests, and installed as
  **managed artifacts** (via `vsdd init`'s managed-section and the
  installed-artifact manifest, not hand-placed, so they inherit drift-refusal
  rather than bypassing it). Self-governance is owned by the earliest layer
  whose trace exists, not placed "throughout".

- REQ-8: **Each control declares its vehicle, fail-closed, with the
  mechanical/adversarial split.** Vehicles: action-time Claude Code hook
  (bound to a concrete PreToolUse/PostToolUse seam) + the Rust mirror; CI or
  commit-time gate; crosslink-native edges and integrity queries; mdatron
  conformance. **Fail-closed**: an unreadable trace source yields a distinct
  "could not check" outcome, never a clean pass (the contract's three-valued
  discipline). **Mechanical vs adversarial**: regex/structure-gate only the
  unambiguous; route judgment (composition correctness, routing correctness,
  register, evidence quality) to a tool-restricted adversarial reviewer.
  Action-time hooks are the **most bypassable** vehicle — the bypass vectors
  (permission modes, a no-verify path, the hook-bypass marker) are
  enumerated, and each early hook control names the non-bypassable Layer-6
  backstop that eventually supersedes it. The Python hook and its Rust mirror
  are **parity-tested** (the declared-mirror fidelity-pin pattern), so the
  two enforcement surfaces cannot silently diverge. Every control's own
  agent-consumed output — a finding that quotes external tracker or git
  content — honors the Terminal output safety requirement (#807), routed
  through the shared cleaner like any other agent-consumed surface.

- REQ-9: **The control machinery's runtime cost is bounded and declared.** A
  per-session hook wall-clock budget and each control's firing cadence ship
  as versioned data (beside the existing budgets); cost-banding and
  model-tier right-sizing of the adversarial-reviewer dispatches (the only
  model-cost leg — mechanical trace reads are zero model cost) are a
  requirement with an acceptance criterion, not prose. An action-time
  control that would exceed its declared per-invocation band routes to
  Phase 4.

- REQ-10: **mdatron is a sequenced dependency, consumed via its machine
  envelope.** The mdatron-vehicle controls depend on mdatron v0.2.0
  (`pin --update`, `verify --compact`, config.yaml jurisdiction, the
  published envelope); the upgrade is a **separate integration (#816** —
  install v0.2.0, author `.mdatron/config.yaml`, update the #764 harness)
  done before the controls that need it, not owned by this design and not
  folded across the repo boundary. Controls consume mdatron's **machine
  envelope** (`--json` / the published schema), never human-form engine
  lines (the #49 cold-read found adopter content bleeding inline into human
  form — a trace-integrity hazard). `pin --update` is a **shape-tier**
  supporting check, not the truth-tier amendment authorization (that is the
  Layer-7 gate) — the contract's tier rule holds.

- REQ-11: **Naming/register is a control with an early detection form.** The
  register/coinage check (mdatron's register family, Layer 7) has its
  **cheapest detection form pulled early** as a bootstrap register tripwire
  — the design's own cheapest-form-earliest doctrine applied to the
  invariant that keeps failing. Motivating evidence: a phase-composition
  diagnostic code was fabricated and asserted "contracted" inside this very
  cycle and caught by four cold-review lenses (see acceptance cases) — the
  manual guard failing again is the argument for the early mechanized form.

## Acceptance Criteria

*(Spec-review loop: no red-gate stage; falsification paths are the review's
targets. Contract text uses plain leading names, not the draft's REQ-n
labels.)*

- [ ] AC-1: The audit control has a both-directions falsification path:
  seed a governed act missing its required mechanism trace and assert the
  control flags it; seed a compliant act and assert it does not. The
  Architecture table is the organizing frame, distinct from the control.
  (REQ-1)
- [ ] AC-2: The trace substrate is the derived-view model (no `.vsdd/events/`
  store), and every trusted trace is named tamper-evident; a forgeable trace
  is declared inadmissible. (REQ-2)
- [ ] AC-3: The three acceptance-case bypasses are each specified concretely
  enough to become fixtures — named once, with the un-invoked mechanism, the
  derived-view trace, and the flagging control. (REQ-1, REQ-3, REQ-5)
- [ ] AC-4: Routing detection augments the plan-comment mechanism (edge +
  detection, not redefinition); the blocking form is placed at its true
  layer; the now-built form is detection only. (REQ-3)
- [ ] AC-5: Phase-transition, governed-entry, round-closure, and phase-5
  controls each specify mechanism, bootstrap form, and eventual layer form;
  phase-5 defines non-vacuity mechanically. (REQ-4, REQ-5, REQ-6)
- [ ] AC-6: The three now-built pieces are mapped to contract layers with a
  falsifiable done condition each; the scripts are named dependency-light
  (zero model cost), self-registered/routed/pinned/tested. (REQ-7)
- [ ] AC-7: Every control names a concrete vehicle seam and honors
  fail-closed (could-not-check ≠ clean) and the mechanical/adversarial
  split; hook bypass vectors are enumerated with the Layer-6 backstop.
  (REQ-8)
- [ ] AC-8: A per-session wall-clock budget + per-control cadence + adversarial
  dispatch cost-banding/model-tier are specified as data + criterion. (REQ-9)
- [ ] AC-9: The mdatron dependency is sequenced (#816) with its migration
  path; controls consume the machine envelope; `pin --update` is shape-tier.
  (REQ-10)
- [ ] AC-10: The register control names a vehicle and an early detection
  form. (REQ-11)
- [ ] AC-11: The design partitions what amends the contract vs new build vs
  the separate #816 integration, and drafts the Revision-line entry;
  register clean (no coined labels, no minted codes, no unexpanded
  acronyms), verified by the Documentation Reviewer and Technical Writer
  lenses. (all)

## Architecture

### The unifying frame: every gap is a mechanism-invocation-timing failure

| Gap | Mechanism not invoked | Correct time | Derived-view trace | Trace exists? |
|---|---|---|---|---|
| Unrouted findings | route-before-fix | before a fix-close | plan comment + resolving-issue edge | partial (comment; no edge) |
| Routing narrated, not linked | structural routing edge | at routing | dependency edge | no |
| Route-after-fix | route-before-fix (verify) | before the verify fix | same | partial |
| Phase-boundary informal | phase-transition gate | at each boundary | red-gate/exit-gate record in state + boundary commit | partial |
| Round-closure | round closure | at round close | round-parity + lifecycle disposition | partial |
| Per-layer phase 5 | phase-5 gate | at layer verify | mutation score + non-vacuity artifact | no |
| Design-phase entry | primer + composition declaration + design affordance | at phase entry | composition in state.yaml + pipeline/knowledge presence | partial |
| Self-governance | the tool over its own construction | throughout the build | self-hosted state + hook/CI records | no (not self-hosted) |
| Naming/register | register check | at authoring | mdatron register finding | partial (mdatron) |

The right mechanisms exist or are contracted, but their traces are missing,
forgeable, or checked only in the back-third layers. The audit control reads
the traces; the instance controls make the missing traces exist
(non-forgeable) and pull the cheapest checks early.

### The delta over #810 (stated, not overstated)

#810 already ratified invariant-first as a principle, placed the routing and
parity bootstrap format-carries, and homed the unrouted-findings query at
Layer 2. This design adds, on top of that: the **invocation-timing audit
aggregate** (auditing invocation across *all* mechanisms, not routing
alone); the two gaps #810 did not own (the structural routing *edge*; the
tool's self-governance over its own construction); the **governed
design/spec-phase-entry** control; the **per-layer phase-5** gate; the
**runtime-cost budget**; and the **dogfood-now detection increment**. It
does not restate #810's ordering.

### The three acceptance-case bypasses (fixtures)

Named once each, with the trace and the flagging control:
1. **The six-round routing collapse** (#806/#808): route-before-fix never
   invoked across rounds 1–5; trace = fix-close result comments with no
   prior plan comment; flagged by routing detection (REQ-3).
2. **The phase-compressed Layer-3 rebuild** (#813): phase-transition gate
   artifacts not separately recorded; trace = a phase advance in state.yaml
   with no red-gate/exit-gate record + boundary commit between; flagged by
   the phase-transition gate (REQ-4).
3. **The design-phase bypass** (this cycle, #815): primer/composition/design-
   affordance not invoked at entry; trace = a `.design` authoring act with no
   composition in state.yaml and no pipeline/knowledge; flagged by governed
   design-entry (REQ-5). The fabricated-and-"contracted" diagnostic code is
   a register instance of the same case, flagged by REQ-11's early form.

If the built controls would not flag these three, they are insufficient by
construction (AC-1, AC-3).

### The dogfood-now increment, mapped to layers

Per the dogfood decision (detection + self-host now; blocking at true
layers): self-host state (Data-authoring + Layer 1) → `vsdd status` reads it
(Layer 2); trace-audit detection (a Layer-2 process-integrity member);
routing detection (the #810 carry's read). Each has a falsifiable done
condition (AC-6). The blocking routing form and the full gate commands stay
at the install/hook seam and Layer 6.

### Cost, vehicle, and what amends the contract

Cost (REQ-9): the adversarial-dispatch leg is the only model-cost; bound it
with a declared budget + cadence + model-tier. Vehicles (REQ-8) bind to
concrete substrate seams (hooks + Rust mirror; CI; crosslink; mdatron
envelope), fail-closed. **Amends the contract**: the audit aggregate as a
Status surface; the two newly-owned gaps; the governed-entry control; the
per-layer phase-5 gate; the cost budget as versioned data; the re-sequencing
of the cheapest forms earlier. **New build**: the derived-view trace
additions, the detection scripts, the self-hosted state. **Separate
integration (#816)**: the mdatron v0.2.0 upgrade + config.yaml + #764.

### Proposed Revision-line entry (draft, AC-11)

> Amended 2026-07-DD under the phase-1a spec-amendment loop (vsdd-cli #815,
> ratified by operator decision on that issue after a seven-lens cold review
> — solution architect, solution owner, security, ai-engineer, platform
> engineer, quality engineer, documentation reviewer and technical writer —
> and a terminal verify round; motivated by the Layer-3 compliance audit and
> the process bypasses it recorded, #806/#808, #813, and the design-phase
> bypass of this cycle): agent-first controls auditing that the methodology's
> mechanisms are invoked at the correct times — a Status process-integrity
> aggregate reading tamper-evident derived-view traces; the structural
> routing edge and the tool's self-governance given owners; governed
> design/spec-phase entry; the per-layer phase-5 gate with a defined
> non-vacuity check; a declared runtime-cost budget for the control
> machinery; the cheapest detection forms re-sequenced earlier, blocking
> forms at their true layers; the mdatron dependency consumed via its
> machine envelope and sequenced separately.

## Resolved decisions (design phase 3 — operator, 2026-07-28)

- **Audit home — one control, one home:** a Status process-integrity
  aggregate that consumes mdatron conformance records as inputs (not a
  second, co-owned home). (Refined from "both split by trace type" after the
  cold review flagged the double-home; mdatron remains the record-estate
  conformance engine, consumed, not co-owning the audit.)
- **Dogfood scope — detection + self-host now; blocking at its true layer.**
  The now-built pieces are self-host state, trace-audit detection, and
  routing detection (non-blocking). The blocking routing form needs the
  Layer-4/7 hook seam and is placed there, not built now (cold-review
  correction of the "dependency-free blocking control now" premise).
- **Requirements registry — crosslink primary + a thin status tripwire.**
- **Reviewer-role tool-restriction — pulled earlier**, committed to
  capability enforcement (a critic that lacks the edit capability, so it
  cannot edit the traces it checks).
- **Retroactive phase 5 for Layers 1 and 2 — forward-only deferral with a
  retest trigger** (run when either layer's pure core is next materially
  touched; tracked debt, non-blocking).
- **mdatron v0.2.0 upgrade — sequenced separately as #816**, not folded into
  this cycle (cold-review correction: no migration path, repo-boundary
  crossing, breaks live CI).

## Out of Scope

- The full control set's build — ratification drives it. The dogfood-now
  increment builds only the three detection/self-host pieces above, after
  ratification, against the reviewed spec.
- The mdatron v0.2.0 upgrade — the separate #816 integration.
- Blocking control forms — placed at their true-dependency layers, not built
  now.
- Rounds 1–5 re-routing — forward-only; the deviation stands.
- Re-opening closed milestones — new controls land as tracked increments
  under their boundaries (the #811 precedent), not reopens.
