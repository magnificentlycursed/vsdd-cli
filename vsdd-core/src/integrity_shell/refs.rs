//! The off-grammar branch query (contract: Conformance at action time,
//! the branch-grammar seam): a pure membership core over injected ref
//! names, consuming the registered grammar — both forms perpetually
//! valid, the exemption set as data (vsdd-cli #688 addendum), decidable
//! from the ref alone. The git listing is the shell's.

use std::path::Path;

use crate::diagnostics::Diagnostic;
use crate::registry::sets::BranchGrammar;
use crate::registry::REGISTRY_REPAIR_ACTION;

/// The refs the query runs over: this clone's own branches — local refs
/// and their remote-tracking counterparts, the remote name stripped so
/// membership stays decidable from the ref alone (the shell half).
pub fn local_refs(repo_root: &Path) -> Result<Vec<String>, Box<Diagnostic>> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args([
            "for-each-ref",
            "--format=%(refname:short)",
            "refs/heads",
            "refs/remotes",
        ])
        .output()
        .map_err(|e| git_diagnostic(repo_root, format!("cannot run git: {e}")))?;
    if !output.status.success() {
        return Err(git_diagnostic(
            repo_root,
            format!(
                "git for-each-ref failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        ));
    }
    let mut refs = Vec::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let name = line.trim();
        if name.is_empty() || name.ends_with("/HEAD") {
            continue;
        }
        // A remote-tracking counterpart reads `origin/feature/x`; strip
        // the remote segment so both halves share one membership check.
        let stripped = match name.split_once('/') {
            Some(("origin", rest)) => rest,
            _ => name,
        };
        if !refs.iter().any(|have: &String| have == stripped) {
            refs.push(stripped.to_string());
        }
    }
    Ok(refs)
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

fn git_diagnostic(repo_root: &Path, message: String) -> Box<Diagnostic> {
    Box::new(Diagnostic {
        file: repo_root.to_path_buf(),
        kind: "permission-or-io".to_string(),
        machine_token: "permission-or-io".to_string(),
        location: None,
        message,
        recovery_action: String::new(),
        recovery_text: String::new(),
    })
}
