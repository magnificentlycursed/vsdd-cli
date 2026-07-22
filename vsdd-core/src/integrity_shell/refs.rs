//! The off-grammar branch query (contract: Conformance at action time,
//! the branch-grammar seam): a pure membership core over injected ref
//! names, consuming the registered grammar — both forms perpetually
//! valid, the exemption set as data (vsdd-cli #688 addendum), decidable
//! from the ref alone. The git listing is the shell's, through the
//! bounded runner (vsdd-cli #751).
//!
//! The listing runs as TWO queries (vsdd-cli #752), BOTH over full
//! refnames (vsdd-cli #761: `%(refname:short)` disambiguates against
//! tags, so a branch-tag name collision renders `heads/<name>` and
//! forges an off-grammar finding) — prefix stripping is structural on
//! both halves, so remote-name handling covers any remote and a local
//! branch whose name merely resembles `<remote>/x` is never mangled.

use std::path::{Path, PathBuf};

use crate::diagnostics::Diagnostic;
use crate::registry::sets::BranchGrammar;
use crate::registry::REGISTRY_REPAIR_ACTION;
use crate::subprocess::{run_bounded, Subprocess};

/// The refs the query runs over: this clone's own branches — local refs
/// and their remote-tracking counterparts, the remote segment stripped
/// so membership stays decidable from the ref alone (the shell half).
pub fn local_refs(repo_root: &Path) -> Result<Vec<String>, Box<Diagnostic>> {
    let local = git_ref_lines(repo_root, "%(refname)", "refs/heads")?;
    let remote = git_ref_lines(repo_root, "%(refname)", "refs/remotes")?;
    Ok(normalize_ref_lines(&local, &remote))
}

/// The pure normalizer over the two listings' raw output (vsdd-cli
/// #752, #761): both halves arrive FULL. Local refs strip exactly
/// `refs/heads/` — immune to the short form's tag-collision ambiguity,
/// and a local branch literally named `origin/x`
/// (`refs/heads/origin/x`) stays `origin/x`. Remote-tracking refs
/// strip `refs/remotes/<remote>/` structurally for any remote name,
/// each remote's symbolic `HEAD` is skipped, and the merged list
/// dedups preserving order, locals first.
pub fn normalize_ref_lines(local: &str, remote: &str) -> Vec<String> {
    let mut refs: Vec<String> = Vec::new();
    let mut push = |name: &str| {
        if !name.is_empty() && !refs.iter().any(|have| have == name) {
            refs.push(name.to_string());
        }
    };
    for line in local.lines() {
        let Some(name) = line.trim().strip_prefix("refs/heads/") else {
            continue;
        };
        push(name);
    }
    for line in remote.lines() {
        let Some(rest) = line.trim().strip_prefix("refs/remotes/") else {
            continue;
        };
        // Drop the remote segment; skip the remote's symbolic HEAD.
        let Some((_remote_name, branch)) = rest.split_once('/') else {
            continue;
        };
        if branch == "HEAD" {
            continue;
        }
        push(branch);
    }
    refs
}

/// The pure membership core: refs matching neither registered form and
/// not exempt are off-grammar, reported in input order. An invalid
/// pattern in the registered data is a diagnostic naming the form —
/// the grammar is adopter-owned registered data.
pub fn off_grammar_refs(
    refs: &[String],
    grammar: &BranchGrammar,
) -> Result<Vec<String>, Box<Diagnostic>> {
    let mut compiled = Vec::with_capacity(grammar.forms.len());
    for form in &grammar.forms {
        let re = regex::Regex::new(&form.pattern).map_err(|e| {
            Box::new(Diagnostic {
                file: std::path::PathBuf::from("templates/registry/dispatch-data.md"),
                kind: "malformed".to_string(),
                machine_token: "malformed".to_string(),
                location: None,
                message: format!(
                    "branch grammar form `{}` carries an invalid pattern: {e}",
                    form.id
                ),
                recovery_action: REGISTRY_REPAIR_ACTION.to_string(),
                recovery_text: "repair the registry artifact to match its schema pair, then re-run"
                    .to_string(),
            })
        })?;
        compiled.push(re);
    }
    Ok(refs
        .iter()
        .filter(|r| {
            !grammar.exempt_refs.iter().any(|e| e == *r)
                && !compiled.iter().any(|re| re.is_match(r))
        })
        .cloned()
        .collect())
}

/// One bounded git query; every non-completed outcome is a diagnostic
/// naming what actually happened (vsdd-cli #754) — never a silent empty
/// list, and never machine-identifying text (the file renders as the
/// repo-relative `.git`, stderr sanitized of the absolute root).
fn git_ref_lines(
    repo_root: &Path,
    format: &str,
    namespace: &str,
) -> Result<String, Box<Diagnostic>> {
    let format_arg = format!("--format={format}");
    match run_bounded("git", &["for-each-ref", &format_arg, namespace], repo_root) {
        Subprocess::Completed { stdout } => Ok(stdout),
        Subprocess::NotFound => Err(git_diagnostic(
            "git is not on PATH — the branch query cannot run".to_string(),
        )),
        Subprocess::SpawnBroken(detail) => Err(git_diagnostic(format!(
            "git is present but failed to start: {}",
            sanitize(&detail, repo_root)
        ))),
        Subprocess::TimedOut => Err(git_diagnostic(
            "git for-each-ref ran past the deadline and was stopped".to_string(),
        )),
        Subprocess::Refused { stderr } => Err(git_diagnostic(format!(
            "git for-each-ref over {namespace} exited nonzero: {}",
            sanitize_then_truncate(stderr.trim(), repo_root)
        ))),
        Subprocess::Oversize => Err(git_diagnostic(
            "git for-each-ref output exceeded the artifact cap".to_string(),
        )),
    }
}

/// Record-destined text renders repo-relative (contract clause, #730):
/// the root becomes `.` in BOTH its given and canonicalized spellings
/// (macOS aliases `/tmp`-family roots under `/private`), and the home
/// directory renders `~` so an out-of-root path drops its account
/// segment (vsdd-cli #760). Root replacements run before the home one,
/// since the root usually lives under the home.
fn sanitize(text: &str, repo_root: &Path) -> String {
    let mut out = text.replace(&repo_root.display().to_string(), ".");
    if let Ok(canonical) = repo_root.canonicalize() {
        out = out.replace(&canonical.display().to_string(), ".");
    }
    if let Some(home) = std::env::var_os("HOME").filter(|h| !h.is_empty()) {
        out = out.replace(&PathBuf::from(home).display().to_string(), "~");
    }
    out
}

/// Truncation happens AFTER sanitization (vsdd-cli #760): a cut that
/// bisects an absolute path would defeat the exact-match replace.
fn sanitize_then_truncate(text: &str, repo_root: &Path) -> String {
    sanitize(text, repo_root).chars().take(500).collect()
}

fn git_diagnostic(message: String) -> Box<Diagnostic> {
    Box::new(Diagnostic {
        file: PathBuf::from(".git"),
        kind: "permission-or-io".to_string(),
        machine_token: "permission-or-io".to_string(),
        location: None,
        message,
        // The empty action is the signal: no registered recovery exists
        // for broken git; the text is the whole guidance.
        recovery_action: String::new(),
        recovery_text: String::new(),
    })
}
