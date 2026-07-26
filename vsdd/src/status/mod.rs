//! `vsdd status` — the three renderings and the wiring script
//! (Layer 3; vsdd-cli #772, #773). One acquisition, one derivation,
//! one rendering per invocation; the composition root owns the
//! effectful shell and the conduct instruments; every renderer is
//! pure.

pub mod broken;
pub mod human;
pub use segment::clean_for_terminal;
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

/// The one fallback wording every surface uses for an unregistered
/// degraded kind — the machine form included, so no surface goes
/// silent where its siblings speak (vsdd-cli #779).
pub(crate) const UNREGISTERED_DEGRADED_KIND: &str =
    "unregistered degraded kind — repair the statusline data set";

/// The worded repo identity for a root — the same leaf derivation the
/// acquisition uses; composed display lines carry it even when broken
/// (vsdd-cli #776).
pub(crate) fn worded_repo_name(root: &Path) -> String {
    root.file_name()
        .map(|n| segment::clean_for_terminal(&n.to_string_lossy()))
        .unwrap_or_else(|| "unnamed repo".to_string())
}

/// The registered next-step text for a degraded kind; the fallback
/// wording is shared by every surface that names an unregistered kind.
pub(crate) fn degraded_next_step<'a>(data: &'a StatuslineData, kind: &str) -> Option<&'a str> {
    data.degraded_kinds
        .iter()
        .find(|k| k.kind == kind)
        .map(|k| k.next_step_text.as_str())
}

/// A member line bounded by the configured per-repo budget (vsdd-cli
/// #778: the registered field finally has a consumer — one wedged
/// member repo can no longer stall the whole composed display). On
/// breach the line still identifies its repo and points at the pull
/// surface; the render thread is abandoned to finish or die on its
/// own, the same posture the bounded subprocess runner takes. DRAFT
/// COPY, raised for registration with the statusline data set: the
/// breach wording below.
pub fn bounded_line(
    root: &Path,
    budget: std::time::Duration,
    render: impl FnOnce() -> String + Send + 'static,
) -> String {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(render());
    });
    rx.recv_timeout(budget).unwrap_or_else(|_| {
        format!(
            "{}  no answer within budget — vsdd status",
            worded_repo_name(root)
        )
    })
}

/// One repo's segment line for the composed display: its own state,
/// its own single acquisition; a broken member renders its mark. The
/// effectful sibling of `multi::render_multi`, homed with the
/// composition root rather than the binary's argument parsing.
pub fn segment_for_repo(
    root: &Path,
    data: &StatuslineData,
    actions: &CompositionScopeAndActions,
) -> String {
    run_statusline(
        root,
        std::io::empty(),
        data,
        actions,
        vsdd_core::snapshot::acquire::acquire_snapshot,
    )
    .segment
}

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
            let surfaces = broken::compose_broken_state(&diagnostic, data, last.as_deref());
            // The broken line still says WHICH repo (vsdd-cli #776):
            // two broken members of the composed display must never be
            // indistinguishable, and the single-repo glance gains the
            // same identity.
            format!("{}  {}", worded_repo_name(repo_root), surfaces.segment)
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
