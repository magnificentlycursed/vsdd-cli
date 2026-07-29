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
required mechanism was not invoked at its correct time, and nothing detected
it but the operator.

This is a phase-1a design (composition declared on #815; authored under the
loaded primer, the Solution Owner / Solution Architect / AI Engineer domain
skills, and the claude-code-cli supplement, via crosslink's design skill;
Red Team added to the composition). It is an invariant-first re-sequencing +
enhancement amendment; the tool runs its own controls over its own
construction; the vehicle is a declared per-control mix.

This is the v3 draft, after a 7-lens cold review, a 4-lens terminal verify,
and two Red Team passes (escape analysis + a wider evaluation of the built
system). Their **load-bearing correction:** detection alone is *advisory* —
both Red Team adversaries judged the golden path "not the easy path" because
nothing *forces* the now-built controls to run in the bootstrap window, so
v2 reproduced the corpus topology (a record a human must read). v3 answers
it: the now-built controls **bind to forcing seams that exist today** (the
repo's git pre-commit; a session-stop hook) so deviation produces
compiler-shaped feedback — enforcement, not a latent record. Further
corrections folded: traces are tamper-evident by **corroboration** (git +
the signed remote tracker), not local git alone; the audit gets an
**external heartbeat** so it is not exempt from its own discipline; the
disposition-closure and fix-with-no-owning-finding routing bypasses are
closed; and three shipped Layer-1/3 vulns the wider eval found are fixed in
the defect lane (#818 — prose-injection done; forgeable-state and
false-assurance specified here as REQ-4 and REQ-8, built next).

Honest note carried on the record: the currently built Layer-2 queries
(including the unrouted-findings query, #811) are **live-dormant** — they
fire on fixtures but not on the real repo, because acquisition holds the
tracker-join inputs empty until the Layer-6 join. The could-not-check
control (REQ-8) makes that visible instead of reporting a false clean.

*Register note: "audit aggregate", "forcing seam", "tamper-evident by
corroboration" and similar are draft orientation phrases; any text proposed
for the contract uses plain leading names and plain description.*

## Requirements

- REQ-1: **The invocation-timing audit is one control, one home, with an
  external heartbeat.** A Status process-integrity aggregate that reads the
  traces and reports, per governed act, whether the act's required mechanism
  was invoked at its correct time — an integrity finding that never degrades
  the answer. It aggregates the instance controls' outputs and consumes
  mdatron conformance records as inputs (never co-owning that home). Because
  an auditor that only runs when polled is itself subject to the gap it
  audits, it has an **external heartbeat it does not author**: a scheduled
  run (a crosslink sentinel) whose *absence* is a loud tracker condition,
  plus a per-governed-act expectation so a governed act with no matching
  audit invocation is itself detectable. The audit is not the frame (AC-1's
  table) and not a superset of the instance controls.

- REQ-2: **Traces are derived-view and tamper-evident by corroboration.**
  The audit reads the contract's derived-view model — boundary commit
  history, milestone closes, typed result comments, the state artifact's
  active-composition field — never a store beside the records. A local git
  leg is **rewritable by the audited actor**, so non-forgeability comes from
  **corroboration** between the local record and the signed/remote tracker;
  and the state artifact's active-composition is a self-declaration whose
  git-commit provenance proves *authorship*, not *truth*. A trace an actor
  can author or rewrite without corroboration is inadmissible; the control
  must name, per trace, the corroborating leg.

- REQ-3: **Routing-before-fix augments the ratified mechanism and closes its
  bypasses.** The routing *is* the ratified `plan` comment (contract Finding
  lifecycle; the #810 carry). This adds a structural dependency edge linking
  a routed finding to its resolving issue — the crosslink primitive that
  represents the edge is named, or flagged as an upstream capability
  requirement (as the contract flags comparable seams) — and detection, both
  firing directions. It closes two bypasses the corpus used: (a) the
  disposition-closure exemption — a burst of same-target consolidated or
  evidence-free dispositions is a suspicious pattern, and the
  consolidation-substance check (survivor names pins, label union) is pulled
  to this early layer; (b) fix-with-no-owning-finding — a governed-set commit
  must name a resolvable owning finding **filed before the commit** (the
  pre-existing tracker object plus its filing timestamp is the non-forgeable
  half). Routing detection checks presence and edge-resolution; correctness
  stays the adversarial reviewer's job (REQ-8). The blocking form is bound to
  the forcing seam (REQ-7).

- REQ-4: **Phase transitions are gated on tamper-evident records (the
  forgeable-state fix).** A phase advance requires a red-gate/exit-gate
  record **whose evidence resolves to a real boundary commit in this clone's
  git history**, not a self-authored `GateResult.evidence` string trusted
  from the state artifact, plus a boundary commit between advances. Until the
  evidence is resolved, the machine envelope marks phase/gate provenance as
  **unverified self-report** so an agent does not treat it as corroborated
  (the built-system fix for the Red-Gate-cheat analog the wider eval found
  live; #818 Fix 1).

- REQ-5: **Design/spec-phase entry is governed.** Presence checks (primer
  loaded, composition declared, design affordance used, pipeline + knowledge
  present) paired with the adversarial composition-correctness pass —
  presence is not correctness (a self-authored wrong composition passes a
  presence check).

- REQ-6: **Per-layer phase-5 gate.** Mutation score against the kill floor +
  a defined non-vacuity check (a property is vacuous if it simplifies to a
  constant true, never references the result, or is implied by its
  precondition); survivors and vacuous properties fail the gate.

- REQ-7: **Self-governance, dogfooded via a forcing seam.** Detection alone
  is advisory; the now-built increment therefore **binds detection to seams
  that exist today** so deviation produces compiler-shaped feedback: (a) the
  repo's **git pre-commit** refuses a governed-set commit while an
  unrouted, un-owned, or ungated finding stands; (b) a **session-stop hook**
  non-zero-exits on any new integrity finding; (c) **self-host
  `.vsdd/state.yaml`** so `vsdd status` reads real state on this repo. The
  now-built detection is compiled Layer-1/2 binary members (no `vsdd init`
  and no mdatron dependency — the managed-install and mdatron-shell language
  applies only to the *deferred* blocking/conformance forms); the *forcing*
  is the hook wiring (the pre-commit already exists; the session-stop hook is
  thin). The detection scripts and their pins are **themselves governed
  artifacts** (an edit to a control script is a governed-file change under
  conformance), so self-modification is caught, and the corroborating truth
  legs (git, the signed tracker) are not agent-editable. The registry is
  crosslink primary (milestones/issues) with a thin status-consistency lint.

- REQ-8: **Vehicle mix, fail-closed, mechanical/adversarial split, bypass
  enumeration.** Vehicles bind to concrete seams (Claude Code
  PreToolUse/PostToolUse hooks + the Rust mirror; CI/commit gate;
  crosslink-native edges and queries; mdatron conformance). **Fail-closed,
  reconciled with benign-offline:** tracker-*absent* is the contract's
  benign offline mode, but a tracker-*dependent* check whose input is
  unavailable emits a distinct **could-not-check**, never a clean pass — the
  report distinguishes *checks-run-clean* from *checks-dormant /
  could-not-check* via a checks-run-vs-dormant manifest (the false-assurance
  fix; #818 Fix 2 — today the Layer-2 queries are dormant, so this is the
  difference between an honest and a lying report). A slow or timing-out hook
  **fails closed (blocks)**, so "make the hook slow" is not a bypass. The
  bypass vectors are enumerated — permission modes, `--no-verify`, the
  hook-bypass marker, **the mis-rooted-session / no-live-hooks vector** (the
  estate's demonstrated real bypass, whose session-substrate backstop is
  self-referential when the session is mis-rooted) — and each early
  commit-time control names its qualifier: a commit-time gate is skippable
  via `--no-verify`, so true non-bypassability needs **server-side CI**, not
  the local hook. Regex/structure-gate only the unambiguous; route judgment
  (composition/routing correctness, register, evidence quality) to a
  tool-restricted adversarial reviewer that lacks the edit capability.

- REQ-9: **The control machinery's runtime cost is bounded and declared.** A
  per-session hook wall-clock budget and per-control firing cadence ship as
  versioned data; the adversarial-reviewer dispatches (the only model-cost
  leg) are cost-banded and model-tier right-sized; an action-time control
  exceeding its per-invocation band routes to Phase 4; a control that would
  time out fails closed (REQ-8), never allow-open.

- REQ-10: **mdatron is a sequenced dependency, consumed via its machine
  envelope.** The mdatron-vehicle controls depend on mdatron v0.2.0; the
  upgrade is the separate integration #816 (install v0.2.0, author
  `.mdatron/config.yaml`, update the #764 harness), done before the controls
  that need it, not owned here. Controls consume the machine envelope
  (`--json` / the published schema), not human-form engine lines.
  `pin --update` is a shape-tier supporting check, not truth-tier
  authorization — and a **re-pin requires re-review evidence**, not a bare
  hash update, so a re-pin cannot silently pass the drift gate.

- REQ-11: **Naming/register is a control with an early detection form.** The
  register/coinage check (mdatron's register family, Layer 7) has its
  cheapest detection form pulled early as a bootstrap register tripwire.
  Motivating evidence: a phase-composition diagnostic code was fabricated and
  asserted "contracted" inside this cycle and caught by four cold-review
  lenses (acceptance cases) — the manual guard failing again.

- REQ-12: **Escape-vector coverage is stated.** For each escape class in the
  real corpus (this session and prior cycles), the design names the control,
  the **non-forgeable trace**, and whether the control **shifts the
  incentive gradient** (deviation produces compiler-shaped feedback via a
  forcing seam) or only detects. A control that only detects, during the
  bootstrap window, is marked advisory, not enforcement (the Red Team honesty
  bar). The table is in Architecture.

## Acceptance Criteria

*(Spec-review loop: no red-gate stage; falsification paths are the review's
targets. Contract text uses plain leading names, not the draft's REQ-n
labels.)*

- [ ] AC-1: The audit control has a both-directions falsification path and
  an external-heartbeat criterion (a governed act with no matching audit
  invocation, and an absent scheduled run, are both detectable). (REQ-1)
- [ ] AC-2: Each trusted trace names its corroborating leg; a
  local-only/agent-authored trace is declared inadmissible. (REQ-2)
- [ ] AC-3: Routing detection augments the plan-comment (edge + detection,
  both directions), and the disposition-closure and fix-with-no-owning-
  finding bypasses are each closed with a named non-forgeable half. (REQ-3)
- [ ] AC-4: Phase-transition gating resolves gate evidence to a boundary
  commit; the envelope marks unresolved gate provenance unverified-self-
  report. (REQ-4)
- [ ] AC-5: Governed-entry pairs presence with adversarial correctness;
  phase-5 defines non-vacuity. (REQ-5, REQ-6)
- [ ] AC-6: The now-built increment binds detection to the git pre-commit +
  session-stop seam (deviation → compiler-shaped feedback), maps its three
  pieces to contract layers with falsifiable done conditions, and the
  detection scripts + pins are governed artifacts. (REQ-7)
- [ ] AC-7: Fail-closed distinguishes checks-run-clean from
  dormant/could-not-check and is reconciled with benign-offline; a
  slow/timing-out hook blocks; the bypass vectors incl. mis-rooted-session
  and `--no-verify` are enumerated with the server-side-CI qualifier. (REQ-8)
- [ ] AC-8: A per-session wall-clock budget + cadence + adversarial-dispatch
  cost-banding/model-tier are specified as data + criterion. (REQ-9)
- [ ] AC-9: mdatron is sequenced (#816), machine-envelope-consumed;
  `pin --update` is shape-tier and a re-pin requires re-review. (REQ-10)
- [ ] AC-10: The register control names a vehicle and an early detection
  form. (REQ-11)
- [ ] AC-11: The escape-vector coverage table names, per corpus escape, the
  control, the non-forgeable trace, and enforcement-vs-advisory. (REQ-12)
- [ ] AC-12: The design partitions amends-contract vs new-build vs the #816
  integration, and drafts a correctly-shaped Revision-line entry (the lens
  count matches its enumeration); register clean (no coined labels, no
  minted codes, no unexpanded acronyms), verified by the Documentation
  Reviewer and Technical Writer lenses. (all)

## Architecture

### The unifying frame

| Gap | Mechanism not invoked | Correct time | Non-forgeable trace (corroborated) | Forcing seam? |
|---|---|---|---|---|
| Unrouted findings | route-before-fix | before a fix-close | plan comment + resolving-issue edge, tracker-corroborated | pre-commit / session-stop (REQ-7) |
| Fix with no owning finding | pre-file the finding | before the commit | pre-existing tracker object + filing timestamp vs commit | pre-commit |
| Disposition bypass | real disposition | at closure | consolidation-substance + disposition-rate corroboration | detection + adversarial |
| Phase-boundary informal | phase-transition gate | at each boundary | gate evidence resolved to a boundary commit | pre-commit / gate |
| Forgeable phase/gate (state) | corroborate the gate | at the answer | boundary-commit resolution; else marked unverified | REQ-4 |
| Round-closure | round closure | at round close | round-parity + lifecycle disposition | Layer 6/7 |
| Per-layer phase 5 | phase-5 gate | at layer verify | mutation score + non-vacuity artifact | CI |
| Design-phase entry | primer + composition + affordance | at phase entry | composition corroboration + pipeline/knowledge | session-stop + adversarial |
| Self-governance / audit-not-run | the audit + its heartbeat | throughout | scheduled-run presence + per-act expectation | sentinel heartbeat |
| Naming/register | register check | at authoring | mdatron register finding | early tripwire + Layer 7 |

### The forcing-seam correction (the load-bearing change)

Both Red Team adversaries showed detection-only is advisory: nothing forces
it in the bootstrap window, so the incentive gradient still favors
deviation. v3 binds the now-built detection to seams that exist today — the
git pre-commit and a session-stop hook — so a deviation (an unrouted
fix-close, an ungated phase advance) produces compiler-shaped feedback at
the moment of the act, not a latent record a human must poll. This is the
invariant-first forcing form pulled as early as a real seam allows; the
heavier non-bypassable form (server-side CI, the Layer-6 gate) lands at its
true layer, and the design is honest that the local hook is `--no-verify`-
skippable until then.

### Escape-vector coverage (REQ-12)

Every corpus escape maps to a control, a non-forgeable (corroborated) trace,
and an enforcement-vs-advisory mark (see the frame table's forcing-seam
column). Enforced-now (via the seam): unrouted findings, fix-with-no-finding,
ungated phase advance. Detected-now / adversarial: disposition bypass,
governed-entry correctness. Advisory-until-its-layer (marked honestly):
round-closure, register (Layer 7), the non-bypassable gate (server CI /
Layer 6). Structurally exempt without the heartbeat: the audit itself
(REQ-1). This table is the Red Team acceptance bar: an escape with no
non-forgeable trace or no gradient-shift is not covered.

### Honest state of the built system (from the wider eval)

Built and effective on live data: the robustness/shape layer (malformed/
oversize state refused, degraded outcomes never swapped, garbage-phase
parking) and terminal-output-safety over every agent surface. Built but
**live-dormant**: the Layer-2 process-integrity queries (incl. #811) — they
fire on fixtures, not on the real repo, until the Layer-6 join; REQ-8's
could-not-check makes this visible. Shipped vulns being fixed (#818): prose-
injection (done), forgeable-state envelope (REQ-4), false-assurance (REQ-8).

### Delta over #810, contract partition, and the shipped fixes

#810 ratified invariant-first, the routing/parity carries, and the
unrouted-findings query at Layer 2. This adds the audit aggregate + its
heartbeat, the forcing-seam binding, the two newly-owned gaps, governed
entry, per-layer phase-5, the runtime-cost budget, and escape-vector
coverage. **Amends the contract:** these as Status/Conformance surfaces +
the re-sequencing. **New build:** the forcing-seam hooks, the detection
scripts, the self-hosted state, the REQ-4/REQ-8 envelope shapes. **Separate
integration (#816):** the mdatron upgrade. The #818 shipped-vuln fixes
implement REQ-4 (forgeable-state) and REQ-8 (false-assurance) once this
ratifies; Fix 3 (prose-injection) is already landed in the defect lane.

### Proposed Revision-line entry (draft, AC-12)

> Amended 2026-07-DD under the phase-1a spec-amendment loop (vsdd-cli #815,
> ratified by operator decision on that issue after a seven-lens cold review
> — solution architect, solution owner, security, ai-engineer, platform
> engineer, quality engineer, and a combined documentation-reviewer /
> technical-writer lens — a terminal verify round, and two Red Team passes
> (escape analysis and a wider evaluation of the built system); motivated by
> the Layer-3 compliance audit and the escape corpus it records, #806/#808,
> #813, and the design-phase bypass of this cycle): agent-first controls
> auditing that the methodology's mechanisms are invoked at the correct
> times — a Status process-integrity aggregate over tamper-evident,
> corroborated derived-view traces with an external heartbeat; detection
> bound to forcing seams (the git pre-commit and a session-stop hook) so
> deviation produces compiler-shaped feedback in the bootstrap window rather
> than a latent record; the structural routing edge, governed design/spec
> entry, and self-governance given owners; the disposition-closure and
> fix-with-no-owning-finding routing bypasses closed; phase transitions
> gated on gate evidence resolved to a boundary commit; a per-layer phase-5
> gate; a declared runtime-cost budget; escape-vector coverage stated; the
> mdatron dependency sequenced separately and consumed via its machine
> envelope.

## Resolved decisions (design phase 3 + post-Red-Team, operator 2026-07-28)

- **Audit home:** one Status aggregate consuming mdatron records as inputs.
- **Dogfood scope:** detection bound to a forcing seam now (the git
  pre-commit + a session-stop hook), not detection-only — the Red Team
  correction; blocking/non-bypassable forms at their true layers.
- **Requirements registry:** crosslink primary + a thin status tripwire.
- **Reviewer-role tool-restriction:** pulled earlier, committed to
  capability enforcement (the critic lacks the edit capability, so it cannot
  edit the traces it checks).
- **Retroactive phase 5 for Layers 1/2:** forward-only deferral with a
  retest trigger.
- **mdatron v0.2.0 upgrade:** sequenced separately as #816.
- **Shipped Layer-1/3 vulns:** fixed in the defect lane now (#818) — prose-
  injection done; forgeable-state (REQ-4) and false-assurance (REQ-8) built
  against this spec.

## Out of Scope

- The full control set's build — ratification drives it; the now-built
  increment is the three seam-bound detection/self-host pieces + Fix 1/2.
- The mdatron v0.2.0 upgrade — the separate #816 integration.
- The non-bypassable server-side-CI / Layer-6 gate forms — their true layers.
- Rounds 1–5 re-routing — forward-only; the deviation stands.
- Re-opening closed milestones — new controls land as tracked increments.
