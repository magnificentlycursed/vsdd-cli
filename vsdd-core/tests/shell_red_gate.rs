//! Layer 2 red gate — the shell-side integrity checks (vsdd-cli #738):
//! the refs query's pure membership core over the registered grammar,
//! and the session-substrate check over the installed-artifact
//! manifest. Fails executed against the pre-implementation stubs.

use std::fs;
use std::path::{Path, PathBuf};

use vsdd_core::integrity_shell::refs::off_grammar_refs;
use vsdd_core::integrity_shell::substrate::{session_substrate_check, CheckResult};
use vsdd_core::registry::{
    self,
    sets::{BranchForm, BranchGrammar, DispatchData, InstalledArtifactManifest},
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn grammar() -> BranchGrammar {
    let d: DispatchData =
        registry::load_set(&repo_root(), "dispatch-data").expect("dispatch data loads");
    d.branch_grammar
}

fn refs(names: &[&str]) -> Vec<String> {
    names.iter().map(|s| s.to_string()).collect()
}

#[test]
fn registered_forms_and_exempt_refs_pass_the_query() {
    let flagged = off_grammar_refs(
        &refs(&[
            "feature/statusline-wiring",
            "issue/42",
            "issue/42-fix-lane",
            "main",
            "crosslink/hub",
            "crosslink/knowledge",
            "wip-archive",
        ]),
        &grammar(),
    )
    .expect("the registered grammar evaluates");
    assert!(
        flagged.is_empty(),
        "both registered forms and every exempt ref pass; flagged: {flagged:?}"
    );
}

#[test]
fn off_grammar_refs_are_reported_by_name() {
    let flagged = off_grammar_refs(
        &refs(&["Feature/Upper", "hotfix-7", "issue/", "feature/ok"]),
        &grammar(),
    )
    .expect("the registered grammar evaluates");
    assert_eq!(
        flagged,
        vec![
            "Feature/Upper".to_string(),
            "hotfix-7".to_string(),
            "issue/".to_string()
        ],
        "each off-grammar ref reported, membership decidable from the ref alone"
    );
}

#[test]
fn an_invalid_pattern_in_the_registered_data_is_a_diagnostic() {
    let broken = BranchGrammar {
        forms: vec![BranchForm {
            id: "session-form".to_string(),
            pattern: "^feature/[unclosed".to_string(),
            meaning: "a deliberately broken pattern".to_string(),
        }],
        exempt_refs: vec!["main".to_string()],
        rules: vec!["fixture".to_string()],
    };
    let diag = off_grammar_refs(&refs(&["feature/x"]), &broken)
        .expect_err("a broken registered pattern is a diagnostic, never a panic");
    assert!(
        diag.message.contains("session-form"),
        "the diagnostic names the broken form"
    );
}

#[test]
fn substrate_check_passes_on_this_live_tree() {
    // The estate's own install is the positive fixture: no entry FAILS.
    // Inconclusive is lawful for the prose-path entries (their
    // surface-specific checks land with their consumers) — reported,
    // never silently passed.
    let manifest: InstalledArtifactManifest =
        registry::load_set(&repo_root(), "installed-artifact-manifest").expect("manifest loads");
    let findings = session_substrate_check(&repo_root(), &repo_root(), &manifest);
    let failures: Vec<_> = findings
        .iter()
        .filter(|f| f.result == CheckResult::Fail)
        .collect();
    assert!(
        failures.is_empty(),
        "no entry fails on the live tree: {failures:?}"
    );
}

#[test]
fn a_missing_tracked_payload_fails_the_check() {
    // The hollow-shell shape: wiring present, payload absent — a loud
    // finding, never a quiet no-op.
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join(".claude/hooks")).unwrap();
    let manifest: InstalledArtifactManifest =
        registry::load_set(&repo_root(), "installed-artifact-manifest").expect("manifest loads");
    let findings = session_substrate_check(dir.path(), dir.path(), &manifest);
    assert!(
        findings.iter().any(|f| f.result == CheckResult::Fail),
        "a tree missing tracked payloads fails loudly"
    );
}

#[test]
fn a_mis_rooted_session_fails_on_the_first_member() {
    // The session-shape rule: the binding member fires before any entry
    // check when the project root is not the repo root.
    let elsewhere = tempfile::tempdir().unwrap();
    let manifest: InstalledArtifactManifest =
        registry::load_set(&repo_root(), "installed-artifact-manifest").expect("manifest loads");
    let findings = session_substrate_check(&repo_root(), elsewhere.path(), &manifest);
    let first = findings.first().expect("the binding member reports first");
    assert_eq!(first.result, CheckResult::Fail);
    assert!(
        first.detail.contains("project root"),
        "the finding names the binding: {}",
        first.detail
    );
}

#[test]
fn worded_absence_passes_by_declaration() {
    // statusline-wiring declares its absence in words; the check honors
    // the declaration rather than probing a path that is prose.
    let manifest: InstalledArtifactManifest =
        registry::load_set(&repo_root(), "installed-artifact-manifest").expect("manifest loads");
    let findings = session_substrate_check(&repo_root(), &repo_root(), &manifest);
    assert!(
        !findings
            .iter()
            .any(|f| f.entry_id == "statusline-wiring" && f.result != CheckResult::Pass),
        "the worded absence is a pass, not a probe failure"
    );
}
