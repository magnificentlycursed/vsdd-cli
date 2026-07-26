//! `vsdd status` — the three renderings and the wiring script
//! (Layer 3; vsdd-cli #772, #773). One acquisition, one derivation,
//! one rendering per invocation; the composition root owns the
//! effectful shell and the conduct instruments; every renderer is
//! pure.

pub mod broken;
pub mod human;
pub mod instruments;
pub mod machine;
pub mod multi;
pub mod segment;

use std::io::Read;
use std::path::Path;
use std::time::Instant;

use vsdd_core::answer::derive::derive_phase_answer;
use vsdd_core::registry::sets::{CompositionScopeAndActions, StatuslineData};
use vsdd_core::snapshot::Snapshot;

/// The statusline path's output plus its conduct instruments.
pub struct StatuslineRun {
    pub segment: String,
    pub instruments: instruments::InvocationInstruments,
}

/// The composition root for `vsdd status --statusline`: reads the
/// state artifact, performs exactly ONE snapshot acquisition through
/// `acquire`, derives once, renders once. Stdin is accepted only to be
/// counted — the statusline path reads zero bytes of it: the
/// substrate's session JSON stays cataloged but unconsumed, and the
/// counting seam makes consumption fail a count rather than pass on
/// output invariance. On a broken state the boundary query replaces
/// the acquisition and the three-surface composition speaks instead.
pub fn run_statusline(
    repo_root: &Path,
    stdin: impl Read,
    data: &StatuslineData,
    actions: &CompositionScopeAndActions,
    mut acquire: impl FnMut(&Path) -> Snapshot,
) -> StatuslineRun {
    let started = Instant::now();
    // Held, counted, never read: the zero on this counter is the
    // criterion's stdin seam.
    let counting = instruments::CountingReader::new(stdin);
    let mut acquisition_count = 0u64;

    let segment = match vsdd_core::state::read_state(&repo_root.join(".vsdd/state.yaml"), data) {
        Ok(state) => {
            acquisition_count += 1;
            let snapshot = acquire(repo_root);
            let answer = derive_phase_answer(&state, &snapshot, actions);
            segment::render_segment(&answer, &snapshot, data)
        }
        Err(diagnostic) => {
            let last = vsdd_core::snapshot::acquire::last_boundary_subject(repo_root);
            broken::compose_broken_state(&diagnostic, data, last.as_deref()).segment
        }
    };

    StatuslineRun {
        segment,
        instruments: instruments::InvocationInstruments {
            stdin_bytes_read: counting.bytes_read(),
            acquisition_count,
            wall_clock: started.elapsed(),
        },
    }
}
