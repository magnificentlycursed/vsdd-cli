//! The shared broken-state composition (Layer 3): when the state
//! artifact cannot be read, all three surfaces still speak — the
//! segment renders the registered broken-state mark, the human form
//! the rustc-shaped diagnostic with the kind's recovery and the
//! boundary re-run, the machine form the structured state-unreadable
//! signal. Carries the last boundary truth from git when one exists
//! (vsdd-cli #740) and a worded absence when none does.

use vsdd_core::diagnostics::Diagnostic;
use vsdd_core::registry::sets::StatuslineData;

/// The three surfaces' broken-state outputs.
pub struct BrokenSurfaces {
    pub segment: String,
    pub human: String,
    pub machine: serde_json::Value,
}

/// Compose the broken-state outputs. Pure over the diagnostic, the
/// data set, and the (already-acquired) last-boundary subject.
pub fn compose_broken_state(
    diagnostic: &Diagnostic,
    data: &StatuslineData,
    last_boundary: Option<&str>,
) -> BrokenSurfaces {
    // Phase-2a stub.
    let _ = (diagnostic, data, last_boundary);
    BrokenSurfaces {
        segment: String::new(),
        human: String::new(),
        machine: serde_json::Value::Null,
    }
}
