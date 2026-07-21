//! The session-substrate check over the installed-artifact manifest
//! (contract: Conformance at action time, the closed-world install):
//! three-valued and fail-closed per entry, surfaced in Status as
//! integrity findings that never degrade the answer.
//!
//! Resolution semantics per the manifest's own vocabulary gloss:
//! `exists` and `exists-and-referenced` resolve filesystem paths (the
//! referenced half checks the entry's reference surfaces name it);
//! `worded-absence` passes by its declaration. An entry whose path is
//! not a filesystem path (the plugin-set prose path) is `inconclusive` —
//! reported, never silently passed — until its surface-specific check
//! lands with its consumer.

use std::path::Path;

use crate::registry::sets::InstalledArtifactManifest;

/// Three-valued, fail-closed: inconclusive never reads as pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckResult {
    Pass,
    Fail,
    Inconclusive,
}

#[derive(Debug, Clone)]
pub struct SubstrateFinding {
    pub entry_id: String,
    pub result: CheckResult,
    pub detail: String,
}

/// Run the check. The first member is the binding itself: the session's
/// project root equals the repo root (the session-shape rule); entry
/// checks follow, one finding per non-pass.
pub fn session_substrate_check(
    repo_root: &Path,
    project_root: &Path,
    manifest: &InstalledArtifactManifest,
) -> Vec<SubstrateFinding> {
    let _ = (repo_root, project_root, manifest);
    todo!("2b: root member first, then per-entry resolution")
}
