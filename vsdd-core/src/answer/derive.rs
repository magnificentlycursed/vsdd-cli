//! The pure phase-answer derivation: (state, snapshot, the loaded
//! vocabulary) to the structured answer. Deterministic; never acquires;
//! the property-test and mutation target.
//!
//! The next-action rules are DRAFT ORACLE TERRITORY: the table below is
//! agent-drafted from the contract's phase definitions and stands as a
//! proposal until the convergence corpus's reference answers are
//! operator-adopted (The operator authors the oracle; vsdd-cli #738).
//!
//! Draft rules:
//! - `current_phase` null: the enter action (`enter-next-phase`).
//! - phase-1a/1b/1c: the phase's authoring action (`1a-author-behavioral-spec`,
//!   `1b-author-verification-architecture`, `1c-author-decomposition`).
//! - phase-2a: `2a-author-red-gate-tests` until the layer's red-gate fail
//!   record appears in `last_gate_result`; then `close-phase`.
//! - phase-2b: `2b-implement-to-green` while the red record stands;
//!   `close-phase` once this layer's green-gate pass is recorded.
//! - phase-2c: `2c-refactor`; `enter-next-phase` once this phase's
//!   phase-exit-gate pass is recorded.
//! - phase-3: `3-dispatch-review-round`.
//! - phase-4: `4-route-findings`. phase-5: `5-run-hardening`.
//!   phase-6: `6-run-convergence-check`.
//! - Degradation never changes the action: the answer is computed from
//!   the state artifact alone and reported degraded with its kind.
//! - The fix-lane members are never output (vsdd-cli #689); await-family
//!   selection needs a blocker surface no Layer 2 input materializes —
//!   a recorded scope note, not a silent gap.

use crate::registry::sets::CompositionScopeAndActions;
use crate::snapshot::Snapshot;
use crate::state::State;

use super::integrity::snapshot_integrity;
use super::PhaseAnswer;

/// Derive the structured phase answer. Pure over its inputs.
pub fn derive_phase_answer(
    state: &State,
    snapshot: &Snapshot,
    actions: &CompositionScopeAndActions,
) -> PhaseAnswer {
    let _ = (state, snapshot, actions, snapshot_integrity);
    todo!("2b: the draft rule table over the vocabulary, degraded kinds never swapped")
}
