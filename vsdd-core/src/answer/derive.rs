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
//! - phase-2a: `2a-author-red-gate-tests` until the layer's red-gate fail
//!   record appears in `last_gate_result`; then `close-phase`.
//! - phase-2b: `2b-implement-to-green` while the red record stands;
//!   `close-phase` once this layer's green-gate pass is recorded.
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
use super::PhaseAnswer;

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

    let token = next_action_token(state);
    // The vocabulary stays authoritative at runtime: the token resolves
    // through the loaded set when present, and the membership test pins
    // drift red either way.
    let next_action = actions
        .action_vocabulary
        .iter()
        .find(|a| a.id == token)
        .map(|a| a.id.clone())
        .unwrap_or_else(|| token.to_string());

    PhaseAnswer {
        phase: state.current_phase.clone(),
        layer: state.current_layer,
        next_action,
        active_composition: state.active_composition.clone(),
        degraded,
        integrity_findings,
    }
}

/// The adopted rule table. Total: the wildcard arm is unreachable against
/// a state honoring the scope-member validity constraint, and answers
/// `await-operator` rather than panicking at a trust boundary — an
/// unknown phase member is precisely a blocked-on-operator condition.
fn next_action_token(state: &State) -> &'static str {
    let Some(phase) = state.current_phase.as_deref() else {
        return "1a-author-behavioral-spec";
    };
    let gate = state.last_gate_result.as_ref();
    let gate_at_layer = |kind: GateKind, outcome: GateOutcome| {
        gate.is_some_and(|g| {
            g.gate == kind && g.result == outcome && state.current_layer == Some(g.layer)
        })
    };
    match phase {
        "phase-1a" => "1a-author-behavioral-spec",
        "phase-1b" => "1b-author-verification-architecture",
        "phase-1c" => "1c-author-decomposition",
        "phase-2a" => {
            if gate_at_layer(GateKind::RedGate, GateOutcome::Fail) {
                "close-phase"
            } else {
                "2a-author-red-gate-tests"
            }
        }
        "phase-2b" => {
            if gate_at_layer(GateKind::GreenGate, GateOutcome::Pass) {
                "close-phase"
            } else {
                "2b-implement-to-green"
            }
        }
        "phase-2c" => {
            let exited = gate.is_some_and(|g| {
                g.gate == GateKind::PhaseExitGate
                    && g.result == GateOutcome::Pass
                    && g.phase == "phase-2c"
                    && state.current_layer == Some(g.layer)
            });
            if exited {
                "enter-next-phase"
            } else {
                "2c-refactor"
            }
        }
        "phase-3" => "3-dispatch-review-round",
        "phase-4" => "4-route-findings",
        "phase-5" => "5-run-hardening",
        "phase-6" => "6-run-convergence-check",
        _ => "await-operator",
    }
}
