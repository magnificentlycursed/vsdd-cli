//! The session-substrate check over the installed-artifact manifest
//! (contract: Conformance at action time, the closed-world install):
//! three-valued and fail-closed per entry, surfaced in Status as
//! integrity findings that never degrade the answer.
//!
//! Resolution semantics per the manifest's own vocabulary gloss:
//! `worded-absence` passes by its declaration; filesystem paths (glob
//! members included) resolve against the tree; an entry whose path is
//! prose (the plugin-set surface) is `inconclusive` — reported, never
//! silently passed — until its surface-specific check lands with its
//! consumer. `exists-and-referenced` claims MORE than presence, and
//! this check verifies only presence (vsdd-cli #746): absence still
//! fails loudly, but presence is `inconclusive` scoped to the
//! unverified half until the reference-surface check lands — presence
//! never silently upgrades to the full claim. Only non-pass results
//! are reported: one finding per dangling reference.

use std::path::{Path, PathBuf};

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
/// project root equals the repo root (the session-shape rule) — and a
/// broken binding returns alone, because entry checks against the wrong
/// root would mislead.
pub fn session_substrate_check(
    repo_root: &Path,
    project_root: &Path,
    manifest: &InstalledArtifactManifest,
) -> Vec<SubstrateFinding> {
    if repo_root != project_root {
        // Record-destined text names no absolute path (contract clause,
        // vsdd-cli #730): the leaf names locate the mismatch; the
        // operator's own shell holds the full paths.
        let leaf = |p: &Path| {
            p.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "(root)".to_string())
        };
        return vec![SubstrateFinding {
            entry_id: "project-root-equals-repo-root".to_string(),
            result: CheckResult::Fail,
            detail: format!(
                "the session's project root (…/{}) does not equal the repo root (…/{}) — the binding member of the session-shape rule; entry checks are withheld because they would run against the wrong root",
                leaf(project_root),
                leaf(repo_root)
            ),
        }];
    }

    let mut findings = Vec::new();
    for entry in &manifest.entries {
        let (result, detail) = check_entry(repo_root, &entry.path, &entry.resolution);
        if result != CheckResult::Pass {
            findings.push(SubstrateFinding {
                entry_id: entry.id.clone(),
                result,
                detail,
            });
        }
    }
    findings
}

fn check_entry(root: &Path, path: &str, resolution: &str) -> (CheckResult, String) {
    if resolution == "worded-absence" {
        return (
            CheckResult::Pass,
            "a deliberate absence recorded in words".to_string(),
        );
    }
    // Prose paths (spaces or an em dash) have no filesystem referent;
    // their surface-specific checks land with their consumers.
    if path.contains(' ') || path.contains('—') {
        return (
            CheckResult::Inconclusive,
            format!("the entry's path is prose, not a filesystem path: {path}"),
        );
    }
    // A home-anchored path with no HOME cannot resolve: inconclusive,
    // never a fabricated in-repo miss (vsdd-cli #746).
    if path.starts_with("~/") && std::env::var_os("HOME").is_none() {
        return (
            CheckResult::Inconclusive,
            format!("HOME is unset; the home-anchored path cannot resolve: {path}"),
        );
    }
    let expanded = expand(root, path);
    let present = if let Some((dir, prefix, suffix)) = glob_parts(&expanded) {
        match std::fs::read_dir(&dir) {
            Ok(entries) => entries.flatten().any(|e| {
                let name = e.file_name().to_string_lossy().into_owned();
                // The length guard keeps prefix and suffix from
                // overlapping on a short name (vsdd-cli #746).
                name.len() >= prefix.len() + suffix.len()
                    && name.starts_with(&prefix)
                    && name.ends_with(&suffix)
            }),
            Err(e) => {
                return (
                    CheckResult::Fail,
                    format!("the glob's directory cannot be read ({path}): {e}"),
                )
            }
        }
    } else {
        expanded.exists()
    };

    match (present, resolution) {
        (false, _) => (
            CheckResult::Fail,
            format!("the referenced artifact is absent: {path}"),
        ),
        // Presence alone never satisfies the fuller claim (#746).
        (true, "exists-and-referenced") => (
            CheckResult::Inconclusive,
            format!(
                "present, but the referenced-by half of `exists-and-referenced` has no check yet — it lands with the reference-surface consumer: {path}"
            ),
        ),
        (true, _) => (CheckResult::Pass, String::new()),
    }
}

fn expand(root: &Path, path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join(rest);
        }
    }
    root.join(path)
}

/// One `*` in the final segment is the manifest's only glob shape.
fn glob_parts(expanded: &Path) -> Option<(PathBuf, String, String)> {
    let name = expanded.file_name()?.to_string_lossy().into_owned();
    let star = name.find('*')?;
    Some((
        expanded.parent()?.to_path_buf(),
        name[..star].to_string(),
        name[star + 1..].to_string(),
    ))
}
