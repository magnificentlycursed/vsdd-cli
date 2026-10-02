---
schema_class: act-to-affordance-map
schema_version: 0.1.1
status: draft-proposal
entries:
  - {act: design-authoring, affordance: crosslink design, kind: crosslink-workflow, condition: ""}
  - {act: spec-to-build-gap-analysis, affordance: "crosslink kickoff plan <doc> / kickoff launch --plan / kickoff show-plan (the read-only gap analysis)", kind: crosslink-workflow, condition: "re-pointed 2026-08-02 by the recorded pair: the prior binding named crosslink design --gap-analysis, which exists on no released surface; the built surface is the kickoff plan family (help-surface-verified at the installed version). Adopting decision handle: the #857 triage disposition (2026-08-02, design-impact audit, phantom-gap-analysis-map-entry)"}
  - {act: autonomous-execution, affordance: crosslink kickoff --container, kind: crosslink-workflow, condition: "ADOPTED as the execution vehicle (SO ruling 2026-08-02, vsdd-cli#859): non-interactive by construction, no gate-disabling; the image blocker resolved 2026-09-25 (container-kickoff-blocked-posture; the upstream image stays private, crosslink#101) and the vehicle runs on the fork's image and binary; the attended local path stalls headless and is not the vehicle"}
  - {act: phase-3-review-round, affordance: "vsdd dispatch over crosslink kickoff run --container, one dispatch per reviewer, each reviewer on its own child issue of the round issue (kickoff's session work takes an exclusive issue lock); vsdd supplies the review stage (each reviewer's composition, the manifest, the round's de-duplication and filing of findings)", kind: crosslink-workflow, condition: "re-bound 2026-10-02 (operator decision, vsdd-cli#891): crosslink swarm review writes a plan and launches no agents, swarm launch carries neither the design doc nor a container, and swarm gate runs only the detected test command (verified in source at the fork tree ddc0cbe57; knowledge page kickoff-swarm-dispatch-pipeline). Kickoff's tool list always carries Write and Edit; its plan permission mode is the untested read-only posture, and the critic no-diff check (detective, against the post-init pre-launch tree) is the compensating control until the live fire tests it. Activates with Slice 6's golden-path dispatcher; until then rounds are hand-run through the attended-review-round-fan-out entry and recorded as such"}
  - {act: phase-exit-gate, affordance: "vsdd gate: today the routing and deviations legs, run by CI as vsdd gate --ci (routing-gate.yml); the phase-exit subcommands land with Slice 4, run by the agent at the boundary through the installed git-hook wrapper and re-run by CI", kind: vsdd-command, condition: "re-bound 2026-10-02 (vsdd-cli#891): crosslink swarm gate runs only a project's detected test command and cannot run vsdd's gates; the phase-exit legs do not exist yet — this entry names the surface that will carry them, not a built one"}
  - {act: run-monitoring, affordance: "crosslink kickoff list / check surface / mission control", kind: crosslink-workflow, condition: "kickoff status covers pipeline-sidecar runs only (dollspace-gay/crosslink#18); list is the all-modes surface"}
  - {act: commit-with-documentation, affordance: the commit skill, kind: skill, condition: ""}
  - {act: issue-lifecycle, affordance: crosslink issue commands with typed comments, kind: crosslink-workflow, condition: ""}
  - {act: session-binding, affordance: crosslink session work / end, kind: crosslink-workflow, condition: ""}
  - {act: knowledge-capture, affordance: crosslink knowledge, kind: crosslink-workflow, condition: ""}
  - {act: mid-flow-intervention-record, affordance: crosslink issue intervene, kind: crosslink-workflow, condition: "the contract's prose names crosslink intervene; the installed 0.8.0 surface nests it under issue — naming drift recorded on vsdd-cli #597, 2026-07-20"}
  - {act: versioned-data-set-authoring, affordance: the data-engineer domain lens, kind: domain-lens, condition: "mandatory — mechanizes the 2026-07-20 composition miss recorded on the #598 trail; the lens runs before any set lands"}
  - {act: schema-bearing-artifact-authoring, affordance: "the pair rule — data artifact plus .mdatron/schemas/<class>.json, validated at pre-commit", kind: skill, condition: "operator-adopted 2026-07-20 (vsdd-cli #660)"}
  - {act: attended-review-round-fan-out, affordance: "the design session's own dispatch surface — the plain fan-out, or the Workflow orchestration surface when the round needs per-lens model and effort dials or structured capture", kind: session-surface, condition: "operator-adopted 2026-07-21 (decision on vsdd-cli #597); distinct from phase-3-review-round, which governs the installed process's autonomous rounds and activates with Slice 6's golden-path dispatcher; until then this entry is the interim vehicle for hand-run rounds. Dial conduct is part of the adoption: every Workflow dispatch sets model and effort explicitly — never inherited (the two surfaces default differently: the plain fan-out to the agent-type model, Workflow to the session model at twice the per-token weight) — and the round manifest records chosen values plus post-hoc telemetry confirmation, closing the assumed-tier class caught at round 1 (#673 correction)"}
rules:
  - "every methodology act with a mapped affordance rides it; hand-rolling an equivalent while the paved path exists carries a stated reason recorded as a directive classification or a decision comment, or is nonconformant"
  - "where a ridden workflow's conduct conflicts with the contract's discipline, the contract governs — the ride adapts the vehicle, never the methodology"
  - "divergence is decidable at audit against this map and the session records"
  - "additions enter by the recorded pair: a new act-to-vehicle binding lands here with its adopting decision handle"
---

# Paved-path map (the file keeps its `act-to-affordance-map.md` name until the schema-pair rename)

The default-vehicle map (contract: Conformance at action time, the
paved-path closure; owned by the AI Engineer domain — the
directive-reconciliation mechanization step's duty at act scale). Proposals
until operator adoption is recorded (vsdd-cli #670).

The `kind` field generalizes the map beyond crosslink workflows: a
`domain-lens` entry summons a composition member for an act class (the
data-engineer entry mechanizes this session's operator-caught miss), and
a `skill` entry names a conduct convention with a mechanical backstop, and a `vsdd-command` entry names a vsdd binary surface.
Conditions are data, not prose — the phase-3 binding activates with the
golden-path dispatcher, the container posture carried its upstream
blockage with a retest trigger until that resolved, and each condition
names its evidence handle.

Evidence: across two repos and the whole respec's sessions, no crosslink
workflow was ever self-summoned — every paved-path use traced to an
operator instruction. This map plus the availability-is-not-activation
delivery paths are the closure.

Authored under phase-1c data authoring (vsdd-cli #598, set issue #670).
Draft vocabulary under the maturity lifecycle until first publish.

Member adoptions recorded on the set issue do not advance this
artifact's status: the status field advances by the phase-exit
adoption act, then first publish (vsdd-cli #715, executing the #697
item-5 standing disposition at the cold pass's finding).
