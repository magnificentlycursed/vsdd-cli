//! Pre-flight environment checks for `vsdd init`.
//!
//! Probes the local environment for the four prerequisites declared in
//! BOUNDARY-PREAMBLE § 6 install-order discipline:
//! `git` repository present, `crosslink` on PATH, `mdatron` on PATH, `cargo` on PATH.
//!
//! Returns a [`PreflightReport`] the caller renders into operator-facing output.
//! Does not write files or modify any state; pure environment inspection.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Result of a single pre-flight probe.
#[derive(Debug, PartialEq, Eq)]
pub enum CheckResult {
    /// Probe found the requirement; payload is the version string or path detected.
    Found(String),
    /// Probe could not find the requirement; payload names the corrective action.
    NotFound(String),
}

impl CheckResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Found(_))
    }
}

/// Full pre-flight report for `vsdd init --check`.
#[derive(Debug)]
pub struct PreflightReport {
    pub cwd: PathBuf,
    pub git_repo: CheckResult,
    pub crosslink: CheckResult,
    pub mdatron: CheckResult,
    pub cargo: CheckResult,
}

impl PreflightReport {
    /// True when every probe found its requirement.
    pub fn all_pass(&self) -> bool {
        self.git_repo.is_ok()
            && self.crosslink.is_ok()
            && self.mdatron.is_ok()
            && self.cargo.is_ok()
    }

    /// Render as a multi-line operator-facing report (rustc-style: one line per probe).
    pub fn render(&self) -> String {
        let mut output = format!("vsdd init pre-flight (cwd: {})\n", self.cwd.display());
        for (name, result) in [
            ("git repo", &self.git_repo),
            ("crosslink", &self.crosslink),
            ("mdatron", &self.mdatron),
            ("cargo", &self.cargo),
        ] {
            match result {
                CheckResult::Found(detail) => {
                    output.push_str(&format!("  [ok]    {name:<12} {detail}\n"));
                }
                CheckResult::NotFound(reason) => {
                    output.push_str(&format!("  [error] {name:<12} {reason}\n"));
                }
            }
        }
        if self.all_pass() {
            output.push_str("all checks passed; vsdd init can proceed\n");
        } else {
            output.push_str("one or more checks failed; resolve before running vsdd init\n");
        }
        output
    }
}

/// Probe the environment from `cwd`. Returns a populated [`PreflightReport`].
pub fn check_environment(cwd: &Path) -> PreflightReport {
    PreflightReport {
        cwd: cwd.to_path_buf(),
        git_repo: check_git_repo(cwd),
        crosslink: check_tool("crosslink"),
        mdatron: check_tool("mdatron"),
        cargo: check_tool("cargo"),
    }
}

/// Probe for a `.git` directory or file (worktree case) at `cwd`.
fn check_git_repo(cwd: &Path) -> CheckResult {
    let git_path = cwd.join(".git");
    if git_path.exists() {
        CheckResult::Found(format!("{}", git_path.display()))
    } else {
        CheckResult::NotFound(format!(
            "no .git at {}; run `git init` first",
            cwd.display()
        ))
    }
}

/// Probe for a tool by invoking `<name> --version`.
fn check_tool(name: &str) -> CheckResult {
    match Command::new(name).arg("--version").output() {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let first_line = stdout.lines().next().unwrap_or("").trim();
            CheckResult::Found(first_line.to_string())
        }
        Ok(output) => CheckResult::NotFound(format!(
            "{name} --version returned exit {}; check installation",
            output.status.code().unwrap_or(-1)
        )),
        Err(e) => CheckResult::NotFound(format!(
            "{name} not on PATH ({e}); install via `cargo install {name} --locked`"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Unique tempdir for filesystem tests; cleaned up on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(label: &str) -> Self {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir().join(format!("vsdd-preflight-{label}-{nanos}"));
            std::fs::create_dir_all(&path).unwrap();
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn check_result_is_ok_only_when_found() {
        assert!(CheckResult::Found("X".into()).is_ok());
        assert!(!CheckResult::NotFound("missing".into()).is_ok());
    }

    #[test]
    fn git_repo_check_finds_existing_dot_git() {
        let temp = TempDir::new("git-found");
        std::fs::create_dir_all(temp.path().join(".git")).unwrap();
        let result = check_git_repo(temp.path());
        assert!(matches!(result, CheckResult::Found(_)));
    }

    #[test]
    fn git_repo_check_reports_missing_dot_git() {
        let temp = TempDir::new("git-missing");
        let result = check_git_repo(temp.path());
        assert!(matches!(result, CheckResult::NotFound(_)));
        if let CheckResult::NotFound(reason) = result {
            assert!(
                reason.contains("git init"),
                "reason should name corrective action; got: {reason}"
            );
        }
    }

    #[test]
    fn tool_check_finds_cargo() {
        // The test suite runs under cargo so cargo must be on PATH for the test to even start.
        let result = check_tool("cargo");
        assert!(
            matches!(result, CheckResult::Found(_)),
            "cargo should be found; got: {result:?}"
        );
    }

    #[test]
    fn tool_check_reports_missing_tool() {
        let result = check_tool("definitely-not-a-real-tool-xyz-987654321");
        assert!(matches!(result, CheckResult::NotFound(_)));
        if let CheckResult::NotFound(reason) = result {
            assert!(
                reason.contains("cargo install"),
                "reason should name install instruction; got: {reason}"
            );
        }
    }

    #[test]
    fn all_pass_requires_every_probe_to_pass() {
        let report = PreflightReport {
            cwd: PathBuf::from("/x"),
            git_repo: CheckResult::Found("a".into()),
            crosslink: CheckResult::Found("b".into()),
            mdatron: CheckResult::NotFound("c".into()),
            cargo: CheckResult::Found("d".into()),
        };
        assert!(!report.all_pass(), "missing mdatron should fail all_pass");
    }

    #[test]
    fn render_includes_each_probe_label() {
        let report = PreflightReport {
            cwd: PathBuf::from("/some/project"),
            git_repo: CheckResult::Found("/some/project/.git".into()),
            crosslink: CheckResult::Found("crosslink 0.8.0".into()),
            mdatron: CheckResult::NotFound("mdatron not on PATH".into()),
            cargo: CheckResult::Found("cargo 1.88.0".into()),
        };
        let rendered = report.render();
        assert!(rendered.contains("git repo"));
        assert!(rendered.contains("crosslink"));
        assert!(rendered.contains("mdatron"));
        assert!(rendered.contains("cargo"));
        assert!(rendered.contains("[ok]"));
        assert!(rendered.contains("[error]"));
        assert!(rendered.contains("one or more checks failed"));
    }
}
