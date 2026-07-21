---
schema_class: state-schema
schema_version: 0.1.0
status: draft-proposal
state_fields:
  - name: schema_version
    type: semver-string
    required: true
    semantics: the version of the state-instance format this file conforms to; bootstrap self-validation checks it before reading further
  - name: current_phase
    type: scope-member-id-nullable
    required: true
    semantics: "the phase the project is in, from the composition-scope enumeration's phase members; never the lane member — fix-lane work does not move the phase pointer; null is pinned to mean 'not yet entered' and the derivation maps it to the enter action (operator ruling 2026-07-20, vsdd-cli #665)"
  - name: current_layer
    type: non-negative-integer-nullable
    required: true
    semantics: "the active layer per the project's own decomposition; range is project-defined, not fixed by this schema; null is pinned to mean 'decomposition not yet authored' (same ruling as current_phase)"
  - name: open_findings_pointer
    type: object
    required: true
    semantics: "{milestone: <exact milestone name>} — the active layer's milestone; open findings are its open children, resolved by one crosslink query (operator ruling 2026-07-20, vsdd-cli #665)"
  - name: last_gate_result
    type: object
    required: false
    semantics: "the most recent gate run: {gate: <gate kind>, phase: <scope-member-id>, layer: <integer>, result: pass|fail, evidence: <commit sha or tracker handle>, recorded: <ISO 8601>}; absent until the first gate runs"
  - name: active_composition
    type: object
    required: true
    semantics: "the computed domain set in force: {scope: <scope-member-id>, domains: [<domain slugs>], mode: skill-interactive|cold-dispatch, config_inputs_hash: <hash over the DESIGN.md surfaces and review config the composition was computed from>} — the hash makes a stale composition mechanically detectable: state hash vs recomputed hash (operator ruling 2026-07-20, vsdd-cli #665)"
  - name: published
    type: object
    required: false
    semantics: "the published marker — absent before first publish; written once by the promotion act: {at: <ISO 8601 date>, version: <semver>, act: <tracker handle>}; the machine-readable publish-state the fix-lane falsifiers consult; immutable once present (forward-only)"
declared_constraints:
  - entry: phase-gate consistency
    rule: a state whose current_phase is phase-2b or later within a layer requires a last_gate_result carrying that layer's red-gate fail record; executed by mdatron's state-consistency family (Layer 7), by vsdd bootstrap self-validation until it lands
  - entry: published immutability
    rule: once published is present its fields never change; a diff touching it after first write fails the check
  - entry: scope-member validity
    rule: current_phase and last_gate_result.phase and active_composition.scope resolve to composition-scope-and-actions members; current_phase resolves to a phase-kind member only
  - entry: boundary-commit discipline
    rule: the state advances only in the same commit as its boundary evidence (contract; checked against git history, not expressible in-instance)
---

# State schema — the .vsdd/state.yaml contents enumeration

The Deterministic phase answer contract's written artifact, as versioned
data (contract: Deterministic phase answer; the phase-state resolution:
the repo artifact is primary, tracker corroborates). `state_fields` is the
contents enumeration — current layer, current phase, open-findings pointer,
last gate result, active composition, and the published marker once the
first-publish promotion act writes it.

The state advances only at phase boundaries, by the agent, in the same
commit as the boundary evidence. Read failures take the enumerated
discipline (malformed, absent, permission-or-IO — the kinds and their
recovery actions are the statusline data set's members, round 3, drawing
on the action vocabulary's recovery family). Layer 1 implements read and
write against this enumeration; mdatron validates instances once its
state-consistency family lands, vsdd self-validating at read until then.

Three OPEN decision notes above await the operator's walk-through; the
seeded initial form `vsdd init` deploys (Layer 4) follows directly from
the pre-entry representation decision.

Authored under phase-1c data authoring (vsdd-cli #598, set issue #665).
Draft vocabulary under the maturity lifecycle until first publish.
