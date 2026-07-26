//! The composed multi-repo display (Layer 3): one segment line per
//! configured repo, current repo first, per-line worded repo identity
//! (the repo-name field every segment carries); the repo set is
//! explicit adopter configuration, never discovered, so the display
//! cannot grow unbounded. Aggregate cost is denominated repo count
//! times the per-repo budget — the count is the config's own list
//! length.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use vsdd_core::diagnostics::Diagnostic;

/// The adopter-owned repo-set configuration the wiring script reads.
#[derive(Debug)]
pub struct RepoSetConfig {
    pub repos: Vec<PathBuf>,
    pub per_repo_budget_ms: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRepoSetConfig {
    repos: Vec<PathBuf>,
    per_repo_budget_ms: u64,
}

/// Read the repo-set config; a malformed file is a diagnostic, never
/// a panic and never a silent empty set.
pub fn read_repo_set_config(path: &Path) -> Result<RepoSetConfig, Box<Diagnostic>> {
    let text = std::fs::read_to_string(path).map_err(|e| {
        config_diagnostic(
            path,
            "permission-or-io",
            format!("cannot read the repo-set config: {e}"),
            None,
        )
    })?;
    let raw: RawRepoSetConfig = serde_yaml_ng::from_str(&text).map_err(|e| {
        let location = e.location().map(|l| (l.line(), l.column()));
        config_diagnostic(
            path,
            "malformed",
            format!("the repo-set config does not match its registered shape: {e}"),
            location,
        )
    })?;
    Ok(RepoSetConfig {
        repos: raw.repos,
        per_repo_budget_ms: raw.per_repo_budget_ms,
    })
}

/// Compose the display: the current repo's line first, then the rest,
/// one line each.
pub fn render_multi(current: &str, others: &[String]) -> String {
    let mut lines = Vec::with_capacity(1 + others.len());
    lines.push(current.to_string());
    lines.extend(others.iter().cloned());
    lines.join("\n")
}

fn config_diagnostic(
    path: &Path,
    token: &str,
    message: String,
    location: Option<(usize, usize)>,
) -> Box<Diagnostic> {
    Box::new(Diagnostic {
        file: path.to_path_buf(),
        kind: token.to_string(),
        machine_token: token.to_string(),
        location,
        message,
        // Adopter-owned config: the message is the guidance; no
        // registered recovery member exists for this surface yet.
        recovery_action: String::new(),
        recovery_text: String::new(),
    })
}
