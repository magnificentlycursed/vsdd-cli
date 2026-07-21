//! State write: atomic, boundary-evidenced, forward-only on `published`.
//!
//! The state advances only at phase boundaries, in the same commit as
//! the boundary evidence — the write takes that evidence as a required
//! input. The write is temp-plus-rename atomic: no partial file survives
//! a failure (the Install requirement's no-partial-write discipline,
//! applied to the state artifact).

use std::path::Path;

use crate::diagnostics::Diagnostic;
use crate::registry::sets::StatuslineData;

use super::schema::State;

/// The boundary evidence a state advance records with.
#[derive(Debug, Clone)]
pub struct BoundaryEvidence {
    /// The boundary commit sha or tracker handle; never empty.
    pub commit: String,
}

/// Write the state artifact atomically.
///
/// Refused with a diagnostic when: the evidence is empty; a prior state
/// exists and the new state's `published` block differs from a written
/// one (forward-only immutability); or a declared constraint fails
/// (phase-gate consistency at the 2b entry, scope-member validity).
/// A write failure leaves the prior file untouched — no partial write
/// survives.
pub fn write_state(
    path: &Path,
    state: &State,
    evidence: &BoundaryEvidence,
    vocabulary: &StatuslineData,
) -> Result<(), Box<Diagnostic>> {
    let _ = (path, state, evidence, vocabulary);
    todo!("2b: atomic write with constraint enforcement")
}

/// The pure immutability check: true exactly when a written `published`
/// block survives unchanged into the next state. Property-test target.
pub fn published_unchanged(prior: &State, next: &State) -> bool {
    let _ = (prior, next);
    todo!("2b: forward-only comparison")
}
