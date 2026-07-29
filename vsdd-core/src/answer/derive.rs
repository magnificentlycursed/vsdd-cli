//! The pure phase-answer derivation: (state, snapshot, the loaded
//! vocabulary) to the structured answer. Deterministic; never acquires;
//! the property-test and mutation target.
//!
//! The next-action rules are the ADOPTED ORACLE (operator adoption
//! 2026-07-22 on vsdd-cli #738; the convergence corpus's rationale
//! comments carry each rule's grounding):
//!
//! - `current_phase` null: the first phase's authoring action
//!   (`1a-author-behavioral-spec`) — the work, not the ceremony (operator
//!   amendment 2026-07-22 on vsdd-cli #665).
//! - phase-1a/1b/1c: the phase's authoring action.
//! - A gate record satisfies an arm when it matches on kind, outcome,
//!   layer, AND phase — uniformly across the 2a, 2b, and 2c arms
//!   (operator amendment 2026-07-22 on vsdd-cli #738): a stale record
//!   from another phase of the same layer never advances the answer.
//! - phase-2a: `2a-author-red-gate-tests` until this phase's red-gate
//!   fail record appears in `last_gate_result`; then `close-phase`.
//! - phase-2b: `2b-implement-to-green` while the red record stands;
//!   `close-phase` once this phase's green-gate pass is recorded.
//! - phase-2c: `2c-refactor`; `enter-next-phase` once this phase's
//!   phase-exit-gate pass is recorded.
//! - phase-3: `3-dispatch-review-round`. phase-4: `4-route-findings`.
//!   phase-5: `5-run-hardening`. phase-6: `6-run-convergence-check`.
//! - Degradation never changes the action: the answer is computed from
//!   the state artifact alone and reported degraded with its kind.
//! - The fix-lane members are never output (vsdd-cli #689); await-family
//!   selection needs a blocker surface no Layer 2 input materializes —
//!   a recorded scope note, not a silent gap.

use crate::registry::sets::CompositionScopeAndActions;
use crate::snapshot::{AcquisitionOutcome, Snapshot};
use crate::state::{GateKind, GateOutcome, State};

use super::integrity::snapshot_integrity;
use super::{GateProvenance, PhaseAnswer};

/// Derive the structured phase answer. Pure over its inputs.
pub fn derive_phase_answer(
    state: &State,
    snapshot: &Snapshot,
    actions: &CompositionScopeAndActions,
) -> PhaseAnswer {
    let degraded = match snapshot.acquisition_outcome {
        AcquisitionOutcome::Acquired => None,
        AcquisitionOutcome::Absent => Some("tracker-absent".to_string()),
        AcquisitionOutcome::Unusable => Some("tracker-unusable".to_string()),
    };
    // Integrity is snapshot-scoped: under a degraded outcome there is no
    // usable snapshot to check, and the degraded kind carries the story.
    let integrity_findings = if degraded.is_none() {
        snapshot_integrity(state, snapshot)
    } else {
        Vec::new()
    };

    let (token, gate_provenance) = next_action(state);
    // Every emitted token is a registered vocabulary member; the release
    // pin is the membership test over the full emission set — this
    // assert catches a drifted registry in development runs.
    debug_assert!(
        actions.action_vocabulary.iter().any(|a| a.id == token),
        "next-action token `{token}` is not in the loaded action vocabulary"
    );

    PhaseAnswer {
        phase: state.current_phase.clone(),
        layer: state.current_layer,
        next_action: token.to_string(),
        active_composition: state.active_composition.clone(),
        gate_provenance,
        degraded,
        integrity_findings,
    }
}

/// The adopted rule table. Total: the wildcard arm is unreachable against
/// a state honoring the scope-member validity constraint, and answers
/// `await-operator` rather than panicking at a trust boundary — an
/// unknown phase member is precisely a blocked-on-operator condition.
///
/// Returns the token and, when a state-sourced gate verdict DROVE it (the
/// phase-2a/2b/2c advancement arms), that verdict's provenance — marked
/// unverified-self-report, because `read_state` validates only shape and the
/// gate's evidence is never resolved to a boundary, so the verdict is a
/// self-report from the agent-writable state artifact, not a verified result
/// (vsdd-cli #818 Fix 1). A non-gate-driven token carries no provenance.
fn next_action(state: &State) -> (&'static str, Option<GateProvenance>) {
    let Some(phase) = state.current_phase.as_deref() else {
        return ("1a-author-behavioral-spec", None);
    };
    let gate = state.last_gate_result.as_ref();
    // The uniform conjunct (operator amendment, vsdd-cli #738): a gate
    // record satisfies an arm only when kind, outcome, layer, and phase
    // ALL match the state's own position.
    let gate_matches = |kind: GateKind, outcome: GateOutcome| {
        gate.is_some_and(|g| {
            g.gate == kind
                && g.result == outcome
                && g.phase == phase
                && state.current_layer == Some(g.layer)
        })
    };
    // A token reached via a gate match rests on that state-sourced,
    // evidence-unresolved verdict — self-report provenance (#818 Fix 1).
    let driven = |token: &'static str| (token, Some(GateProvenance::UnverifiedSelfReport));
    match phase {
        "phase-1a" => ("1a-author-behavioral-spec", None),
        "phase-1b" => ("1b-author-verification-architecture", None),
        "phase-1c" => ("1c-author-decomposition", None),
        "phase-2a" => {
            if gate_matches(GateKind::RedGate, GateOutcome::Fail) {
                driven("close-phase")
            } else {
                ("2a-author-red-gate-tests", None)
            }
        }
        "phase-2b" => {
            if gate_matches(GateKind::GreenGate, GateOutcome::Pass) {
                driven("close-phase")
            } else {
                ("2b-implement-to-green", None)
            }
        }
        "phase-2c" => {
            if gate_matches(GateKind::PhaseExitGate, GateOutcome::Pass) {
                driven("enter-next-phase")
            } else {
                ("2c-refactor", None)
            }
        }
        "phase-3" => ("3-dispatch-review-round", None),
        "phase-4" => ("4-route-findings", None),
        "phase-5" => ("5-run-hardening", None),
        "phase-6" => ("6-run-convergence-check", None),
        _ => ("await-operator", None),
    }
}
