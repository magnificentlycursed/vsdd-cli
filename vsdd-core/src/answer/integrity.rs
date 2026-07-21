//! The pure snapshot-scoped integrity checks — the snapshot-schema
//! audit's members whose inputs are materialized fields. The
//! degraded-kind check is the derivation's own enforced property; the
//! three shell-side checks (refs, substrate, unsigned events) join the
//! report in `integrity_shell`.
//!
//! Kinds emitted (the report's comparison grain):
//! `round-parity`, `unresolvable-handles-in-result-comments`,
//! `findings-missing-owner-or-validator`,
//! `closed-findings-missing-evidence`,
//! `phase-pointer-against-milestone-state`.

use crate::snapshot::Snapshot;
use crate::state::State;

/// Run the snapshot-scoped checks; returns finding kinds (deduplicated,
/// order stable). Pure; only meaningful when the snapshot was acquired —
/// the derivation skips it under a degraded outcome.
pub fn snapshot_integrity(state: &State, snapshot: &Snapshot) -> Vec<String> {
    let _ = (state, snapshot);
    todo!("2b: the five materialized checks")
}
