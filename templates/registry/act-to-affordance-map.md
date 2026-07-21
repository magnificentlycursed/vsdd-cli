---
schema_class: act-to-affordance-map
schema_version: 0.1.0
status: draft-proposal
entries:
  - {act: design-authoring, affordance: crosslink design, kind: chassis-workflow, condition: ""}
  - {act: spec-to-build-gap-analysis, affordance: crosslink design --gap-analysis (the read-only gap analysis), kind: chassis-workflow, condition: ""}
  - {act: autonomous-execution, affordance: crosslink kickoff --container, kind: chassis-workflow, condition: "blocked at released versions for Rust fix-lane verification (dollspace-gay/crosslink#9, #10; recorded on vsdd-cli #597); attended tmux kickoff and host-side driver-lane verification are the working postures until the retest trigger"}
  - {act: phase-3-review-round, affordance: crosslink swarm review, kind: chassis-workflow, condition: "CONDITIONAL — activates at the Swarm live fire criterion's pass (Layer 8 exit act); until then the swarm-fallback open question holds the alternative (kickoff-carried dispatch with vsdd injecting manifest and composition into the vehicle's prompt); intel 2026-07-20: swarm review emits a plan and does not itself launch agents (review.rs:99-155), strengthening the fallback shape"}
  - {act: phase-exit-gate, affordance: crosslink swarm gate, kind: chassis-workflow, condition: ""}
  - {act: run-monitoring, affordance: "crosslink kickoff list / check surface / mc", kind: chassis-workflow, condition: "kickoff status covers pipeline-sidecar runs only (dollspace-gay/crosslink#18); list is the all-modes surface"}
  - {act: commit-with-documentation, affordance: the commit skill, kind: skill, condition: ""}
  - {act: issue-lifecycle, affordance: crosslink issue commands with typed comments, kind: chassis-workflow, condition: ""}
  - {act: session-binding, affordance: crosslink session work / end, kind: chassis-workflow, condition: ""}
  - {act: knowledge-capture, affordance: crosslink knowledge, kind: chassis-workflow, condition: ""}
  - {act: mid-flow-intervention-record, affordance: crosslink issue intervene, kind: chassis-workflow, condition: "the contract's prose names crosslink intervene; the installed 0.8.0 surface nests it under issue — naming drift recorded on vsdd-cli #597, 2026-07-20"}
  - {act: versioned-data-set-authoring, affordance: the data-engineer domain lens, kind: domain-lens, condition: "mandatory — mechanizes the 2026-07-20 composition miss recorded on the #598 trail; the lens runs before any set lands"}
  - {act: schema-bearing-artifact-authoring, affordance: "the pair rule — data artifact plus .mdatron/schemas/<class>.json, validated at pre-commit", kind: skill, condition: "operator-adopted 2026-07-20 (vsdd-cli #660)"}
rules:
  - "every methodology act with a mapped affordance rides it; hand-rolling an equivalent while the affordance exists carries a stated reason recorded as a directive classification or a decision comment, or is nonconformant"
  - "where a ridden workflow's conduct conflicts with the contract's discipline, the contract governs — the ride adapts the vehicle, never the methodology"
  - "divergence is decidable at audit against this map and the session records"
  - "additions enter by the recorded pair: a new act-affordance binding lands here with its adopting decision handle"
---

# Act-to-affordance map

The default-vehicle map (contract: Conformance at action time, the
chassis-affordance closure; owned by the AI Engineer domain — the
directive-reconciliation substrate step's duty at act scale). Proposals
until operator adoption is recorded (vsdd-cli #670).

The `kind` field generalizes the map beyond chassis workflows: a
`domain-lens` entry summons a composition member for an act class (the
data-engineer entry mechanizes this session's operator-caught miss), and
a `skill` entry names a conduct convention with a mechanical backstop.
Conditions are data, not prose — the swarm binding activates at the
live fire's pass, the container posture carries its upstream blockage
with the retest trigger, and each condition names its evidence handle.

Evidence: across two repos and the whole respec's sessions, no chassis
workflow was ever self-summoned — every affordance use traced to an
operator instruction. This map plus the availability-is-not-activation
delivery paths are the closure.

Authored under phase-1c data authoring (vsdd-cli #598, set issue #670).
Draft vocabulary under the maturity lifecycle until first publish.
