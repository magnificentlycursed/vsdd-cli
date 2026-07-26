//! The composed multi-repo display (Layer 3): one segment line per
//! configured repo, current repo first, per-line worded repo identity;
//! the repo set is explicit adopter configuration, never discovered.

use std::path::{Path, PathBuf};

use vsdd_core::diagnostics::Diagnostic;

/// The adopter-owned repo-set configuration the wiring script reads.
#[derive(Debug)]
pub struct RepoSetConfig {
    pub repos: Vec<PathBuf>,
    pub per_repo_budget_ms: u64,
}

/// Read the repo-set config; a malformed file is a diagnostic, never
/// a panic and never a silent empty set.
pub fn read_repo_set_config(path: &Path) -> Result<RepoSetConfig, Box<Diagnostic>> {
    // Phase-2a stub.
    let _ = path;
    Ok(RepoSetConfig {
        repos: Vec::new(),
        per_repo_budget_ms: 0,
    })
}

/// Compose the display: the current repo's line first, then the rest.
pub fn render_multi(current: &str, others: &[String]) -> String {
    // Phase-2a stub.
    let _ = (current, others);
    String::new()
}
