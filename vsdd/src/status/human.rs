//! The pure human form (Layer 3): two sections per the adopted facets
//! note (vsdd-cli #738's design note) — the answer (position) and the
//! report (health, with the degraded kind's full next-step text) — a
//! verified superset of every segment field plus the session rendering
//! (the demotion ruling, vsdd-cli #679). Color-independent.

use vsdd_core::answer::PhaseAnswer;
use vsdd_core::registry::sets::StatuslineData;
use vsdd_core::snapshot::Snapshot;

/// Render the human terminal form. Pure.
pub fn render_human(answer: &PhaseAnswer, snapshot: &Snapshot, data: &StatuslineData) -> String {
    // Phase-2a stub.
    let _ = (answer, snapshot, data);
    String::new()
}
