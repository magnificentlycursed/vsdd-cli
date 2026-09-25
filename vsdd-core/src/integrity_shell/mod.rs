//! The shell-side integrity checks that cannot materialize into the
//! snapshot (contract: Verification architecture): the refs query over
//! git references and the installed-artifact-integrity check over the filesystem.
//! Crosslink's unsigned-event count is consumed, not computed; crosslink's
//! read surface exposes no such count today, so that check reports
//! could-not-check with the reason stated — never clean (the contract's
//! Verifiable conformance rule).
//!
//! [`run_shell_checks`] is the effectful join (vsdd-cli #880): `vsdd status`
//! runs it once per invocation and the machine and human forms carry the
//! result; the one-line glance does not (the registered
//! `installed_artifact_findings_visibility` rule).

pub mod installed_artifact;
pub mod refs;

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::registry::{
    self,
    sets::{DispatchData, InstalledArtifactManifest},
};
use crate::subprocess::{run_bounded, Subprocess};
use installed_artifact::{installed_artifact_integrity_check, CheckResult};

/// The three shell-side check ids, mirroring the snapshot-schema audit
/// block's `shell_side_checks` members.
pub const CHECK_OFF_GRAMMAR_BRANCH_NAMES: &str = "off-grammar-branch-names";
pub const CHECK_INSTALLED_ARTIFACT_INTEGRITY: &str = "installed-artifact-integrity-check";
pub const CHECK_UNSIGNED_EVENT_COUNT: &str = "unsigned-event-count";

/// Three-valued, fail-closed, serialized as the enumerated member the
/// machine form's consumers branch on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShellResult {
    Pass,
    Fail,
    CouldNotCheck,
}

/// One item a check reported: a flagged ref, or a manifest entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ShellCheckItem {
    pub id: String,
    pub detail: String,
}

/// One shell-side check's outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ShellCheck {
    pub check: String,
    pub result: ShellResult,
    /// The worded outcome — for could-not-check, WHY the check could not run.
    pub detail: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ShellCheckItem>,
}

/// The effectful shell's report: every registered shell-side check, each
/// with a three-valued result — the report never omits a member.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ShellReport {
    pub checks: Vec<ShellCheck>,
}

impl ShellReport {
    /// The check ids that FAILED — joined into the answer's integrity
    /// finding kinds by the shell, so the kind-set carries them.
    pub fn failing_kinds(&self) -> Vec<String> {
        self.checks
            .iter()
            .filter(|c| c.result == ShellResult::Fail)
            .map(|c| c.check.clone())
            .collect()
    }

    /// True when every check ran and passed; a could-not-check member
    /// means the report is NOT checked-clean.
    pub fn is_checked_clean(&self) -> bool {
        self.checks.iter().all(|c| c.result == ShellResult::Pass)
    }
}

/// The repo root as git sees it from the project root; None when git is
/// absent, refuses, or the directory is not a work tree.
fn git_toplevel(project_root: &Path) -> Option<PathBuf> {
    match run_bounded("git", &["rev-parse", "--show-toplevel"], project_root) {
        Subprocess::Completed { stdout } => {
            let line = stdout.trim();
            (!line.is_empty()).then(|| PathBuf::from(line))
        }
        _ => None,
    }
}

fn canonical(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

/// Run the three shell-side checks over the project root. Effectful (git,
/// the filesystem, the registry); the result is data the pure renderers
/// carry. Never panics: every failure to run is a could-not-check member
/// with its reason.
pub fn run_shell_checks(project_root: &Path) -> ShellReport {
    ShellReport {
        checks: vec![
            off_grammar_check(project_root),
            installed_artifact_check(project_root),
            unsigned_event_count_check(),
        ],
    }
}

fn off_grammar_check(project_root: &Path) -> ShellCheck {
    let could_not = |detail: String| ShellCheck {
        check: CHECK_OFF_GRAMMAR_BRANCH_NAMES.to_string(),
        result: ShellResult::CouldNotCheck,
        detail,
        items: Vec::new(),
    };
    let data: DispatchData = match registry::load_set(project_root, "dispatch-data") {
        Ok(d) => d,
        Err(diag) => {
            return could_not(format!(
                "the branch grammar could not be loaded: {}",
                diag.message
            ))
        }
    };
    let refs = match refs::local_refs(project_root) {
        Ok(r) => r,
        Err(diag) => return could_not(diag.message),
    };
    match refs::off_grammar_refs(&refs, &data.branch_grammar) {
        Ok(flagged) if flagged.is_empty() => ShellCheck {
            check: CHECK_OFF_GRAMMAR_BRANCH_NAMES.to_string(),
            result: ShellResult::Pass,
            detail: format!(
                "{} refs examined; every ref matches a registered form or the exemption set",
                refs.len()
            ),
            items: Vec::new(),
        },
        Ok(flagged) => ShellCheck {
            check: CHECK_OFF_GRAMMAR_BRANCH_NAMES.to_string(),
            result: ShellResult::Fail,
            detail: format!(
                "{} of {} refs match neither registered form and are not exempt",
                flagged.len(),
                refs.len()
            ),
            items: flagged
                .into_iter()
                .map(|r| ShellCheckItem {
                    id: r,
                    detail: "off-grammar branch name".to_string(),
                })
                .collect(),
        },
        Err(diag) => could_not(diag.message),
    }
}

fn installed_artifact_check(project_root: &Path) -> ShellCheck {
    let could_not = |detail: String| ShellCheck {
        check: CHECK_INSTALLED_ARTIFACT_INTEGRITY.to_string(),
        result: ShellResult::CouldNotCheck,
        detail,
        items: Vec::new(),
    };
    let manifest: InstalledArtifactManifest =
        match registry::load_set(project_root, "installed-artifact-manifest") {
            Ok(m) => m,
            Err(diag) => {
                return could_not(format!(
                    "the install manifest could not be loaded: {}",
                    diag.message
                ))
            }
        };
    let Some(repo_root) = git_toplevel(project_root) else {
        return could_not(
            "the repo root could not be resolved (git rev-parse --show-toplevel did not answer)"
                .to_string(),
        );
    };
    let findings = installed_artifact_integrity_check(
        &canonical(&repo_root),
        &canonical(project_root),
        &manifest,
    );
    let examined = manifest.entries.len();
    if findings.is_empty() {
        return ShellCheck {
            check: CHECK_INSTALLED_ARTIFACT_INTEGRITY.to_string(),
            result: ShellResult::Pass,
            detail: format!(
                "{examined} manifest entries examined; every entry resolves as declared"
            ),
            items: Vec::new(),
        };
    }
    let failed = findings
        .iter()
        .filter(|f| f.result == CheckResult::Fail)
        .count();
    let inconclusive = findings.len() - failed;
    let result = if failed > 0 {
        ShellResult::Fail
    } else {
        ShellResult::CouldNotCheck
    };
    ShellCheck {
        check: CHECK_INSTALLED_ARTIFACT_INTEGRITY.to_string(),
        result,
        detail: format!(
            "{examined} manifest entries examined; {failed} failed, {inconclusive} inconclusive"
        ),
        items: findings
            .into_iter()
            .map(|f| ShellCheckItem {
                id: f.entry_id,
                detail: format!(
                    "{}: {}",
                    match f.result {
                        CheckResult::Pass => "pass",
                        CheckResult::Fail => "fail",
                        CheckResult::Inconclusive => "inconclusive",
                    },
                    f.detail
                ),
            })
            .collect(),
    }
}

fn unsigned_event_count_check() -> ShellCheck {
    ShellCheck {
        check: CHECK_UNSIGNED_EVENT_COUNT.to_string(),
        result: ShellResult::CouldNotCheck,
        detail: "crosslink's read surface exposes no unsigned-event count (integrity checks: schema, counters, hydration, locks, layout; sign-backfill needs a signing key and reports nothing without one) — consumed when crosslink publishes one".to_string(),
        items: Vec::new(),
    }
}
