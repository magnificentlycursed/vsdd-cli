//! `vsdd status` — the three renderings and the wiring script
//! (Layer 3; vsdd-cli #772). One acquisition, one derivation, one
//! rendering per invocation; the composition root owns the effectful
//! shell and the conduct instruments; every renderer is pure.
//!
//! PHASE-2A STUBS: bodies are pre-implementation placeholders the red
//! gate fails against, executed; phase 2b replaces them.

pub mod broken;
pub mod human;
pub mod instruments;
pub mod machine;
pub mod multi;
pub mod segment;

use std::io::Read;
use std::path::Path;

use vsdd_core::registry::sets::{CompositionScopeAndActions, StatuslineData};
use vsdd_core::snapshot::Snapshot;

/// The statusline path's output plus its conduct instruments.
pub struct StatuslineRun {
    pub segment: String,
    pub instruments: instruments::InvocationInstruments,
}

/// The composition root for `vsdd status --statusline`: reads the
/// state artifact, performs exactly ONE snapshot acquisition through
/// `acquire`, derives once, renders once; stdin is accepted only to be
/// counted — the statusline path reads zero bytes of it.
pub fn run_statusline(
    repo_root: &Path,
    stdin: impl Read,
    data: &StatuslineData,
    actions: &CompositionScopeAndActions,
    acquire: impl FnMut(&Path) -> Snapshot,
) -> StatuslineRun {
    // Phase-2a stub: deliberately wrong on every conduct member — it
    // consumes stdin, acquires nothing, and renders nothing.
    let _ = (repo_root, data, actions, acquire);
    let mut sink = Vec::new();
    let mut stdin = stdin;
    let consumed = stdin.read_to_end(&mut sink).unwrap_or(0) as u64;
    StatuslineRun {
        segment: String::new(),
        instruments: instruments::InvocationInstruments {
            stdin_bytes_read: consumed,
            acquisition_count: 0,
            wall_clock: std::time::Duration::ZERO,
        },
    }
}
