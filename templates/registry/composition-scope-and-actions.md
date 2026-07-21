---
schema_class: composition-scope-and-actions
schema_version: 0.1.0
status: draft-proposal
scope_members:
  - {id: phase-1a, kind: phase, whitepaper_name: Behavioral Specification}
  - {id: phase-1b, kind: phase, whitepaper_name: Verification Architecture}
  - {id: phase-1c, kind: phase, whitepaper_name: Spec Review Gate / Decomposition}
  - {id: phase-2a, kind: phase, whitepaper_name: Test Suite Generation / Red Gate}
  - {id: phase-2b, kind: phase, whitepaper_name: Minimal Implementation}
  - {id: phase-2c, kind: phase, whitepaper_name: Refactor}
  - {id: phase-3, kind: phase, whitepaper_name: Adversarial Refinement}
  - {id: phase-4, kind: phase, whitepaper_name: Feedback Integration Loop}
  - {id: phase-5, kind: phase, whitepaper_name: Formal Hardening}
  - {id: phase-6, kind: phase, whitepaper_name: Convergence}
  - {id: fix-lane, kind: lane, whitepaper_name: ""}
action_vocabulary:
  - {id: 1a-author-behavioral-spec, family: advance, phase: phase-1a, human: author the behavioral specification}
  - {id: 1a-run-spec-review-round, family: advance, phase: phase-1a, human: run the next spec-review round}
  - {id: 1b-author-verification-architecture, family: advance, phase: phase-1b, human: author the verification architecture}
  - {id: 1c-author-decomposition, family: advance, phase: phase-1c, human: author the decomposition}
  - {id: 2a-author-red-gate-tests, family: advance, phase: phase-2a, human: author the failing test suite}
  - {id: 2a-run-red-gate, family: advance, phase: phase-2a, human: run the red gate against the pre-implementation commit}
  - {id: 2b-implement-to-green, family: advance, phase: phase-2b, human: implement until the suite passes at HEAD}
  - {id: 2b-run-green-gate, family: advance, phase: phase-2b, human: run the green half of the gate}
  - {id: 2c-refactor, family: advance, phase: phase-2c, human: refactor under the green suite}
  - {id: 3-dispatch-review-round, family: advance, phase: phase-3, human: dispatch the next review round}
  - {id: 3-file-round-findings, family: advance, phase: phase-3, human: file the round's findings as tracked issues}
  - {id: 4-route-findings, family: advance, phase: phase-4, human: route unrouted findings}
  - {id: 5-run-hardening, family: advance, phase: phase-5, human: run the declared hardening surfaces}
  - {id: 6-run-convergence-check, family: advance, phase: phase-6, human: run the convergence check}
  - {id: close-phase, family: advance, phase: "", human: run the exit gate and make the boundary commit}
  - {id: enter-next-phase, family: advance, phase: "", human: enter the next phase}
  - {id: enter-next-layer, family: advance, phase: "", human: open the next layer's milestone}
  - {id: file-fix-finding, family: advance, phase: fix-lane, human: file the finding before implementing the fix}
  - {id: run-fix-gate, family: advance, phase: fix-lane, human: run the fix-scale red-and-green gate}
  - {id: resolve-blocker, family: await, phase: "", human: resolve the recorded blocker}
  - {id: await-operator, family: await, phase: "", human: "awaiting an operator act (ratification, adoption, sign-off)"}
  - {id: restore-state-file, family: recovery, phase: "", human: restore .vsdd/state.yaml from the last boundary commit}
  - {id: fix-state-content, family: recovery, phase: "", human: repair the malformed state file to match the last boundary commit}
  - {id: fix-state-permissions, family: recovery, phase: "", human: fix file permissions on the state artifact}
  - {id: reconcile-toward-artifact, family: recovery, phase: "", human: resolve the tracker disagreement toward the state artifact}
---

# Composition scope + action vocabulary

Two enumerations the pure core consumes and never computes (contract:
Deterministic composition; Verification architecture).

**scope_members** is the composition function's second input domain: the ten
whitepaper phases plus the fix-lane scope value. The fix lane is not a phase
and is never called one — declaring the domain keeps every input defined and
determinism testable at both scales. The reserved-word rule holds: *phase*
means a whitepaper phase and nothing else; the lane member carries
`kind: lane`.

**action_vocabulary** is the closed token set for "what happens next": the
phase-answer derivation outputs a member, the convergence test compares
members by exact match, and the Status machine form's per-kind recovery
actions are the `recovery` family (the statusline data set's kind-to-action
mapping selects from it — authored at round 3 against these ids). Families:
`advance` (the work the phase position calls for), `await` (blocked on an
operator act or recorded blocker), `recovery` (the state artifact itself
needs repair). Convergence reference answers are written in these tokens
and are operator-adopted oracles; membership here is a proposal until that
adoption.

Authored under phase-1c data authoring (vsdd-cli #598, set issue #666).
Draft vocabulary under the maturity lifecycle until first publish.
