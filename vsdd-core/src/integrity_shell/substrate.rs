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
//! consumer. Only non-pass results are reported: one finding per
//! dangling reference.

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
        return vec![SubstrateFinding {
            entry_id: "project-root-equals-repo-root".to_string(),
            result: CheckResult::Fail,
            detail: format!(
                "the session's project root ({}) does not equal the repo root ({}) — the binding member of the session-shape rule; entry checks are withheld because they would run against the wrong root",
                project_root.display(),
                repo_root.display()
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
    let expanded = expand(root, path);
    if let Some((dir, prefix, suffix)) = glob_parts(&expanded) {
        return match std::fs::read_dir(&dir) {
            Ok(entries) => {
                let hit = entries.flatten().any(|e| {
                    let name = e.file_name().to_string_lossy().into_owned();
                    name.starts_with(&prefix) && name.ends_with(&suffix)
                });
                if hit {
                    (CheckResult::Pass, String::new())
                } else {
                    (
                        CheckResult::Fail,
                        format!("no artifact matches the glob: {path}"),
                    )
                }
            }
            Err(e) => (
                CheckResult::Fail,
                format!("the glob's directory cannot be read ({path}): {e}"),
            ),
        };
    }
    if expanded.exists() {
        (CheckResult::Pass, String::new())
    } else {
        (
            CheckResult::Fail,
            format!("the referenced artifact is absent: {path}"),
        )
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
