//! Layer 2 red gate — the shell-side integrity checks (vsdd-cli #738):
//! the refs query's pure membership core over the registered grammar,
//! and the installed-artifact-integrity check over the installed-artifact
//! manifest. Fails executed against the pre-implementation stubs.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use vsdd_core::integrity_shell::refs::{normalize_ref_lines, off_grammar_refs};
use vsdd_core::integrity_shell::installed_artifact::{installed_artifact_integrity_check, CheckResult};
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
fn installed_artifact_check_passes_on_a_complete_tree() {
    // A CONTROLLED positive (vsdd-cli #750): the test builds the tree
    // the manifest describes rather than trusting the live checkout,
    // whose drift would silently narrow what this test proves. Every
    // entry with a filesystem path gets its payload created; prose-path
    // entries are inconclusive by design, and home-anchored entries
    // (none among today's entries — they live on the reference-surface
    // list) would be excluded here because a test must not fabricate
    // artifacts in the operator's home.
    let dir = tempfile::tempdir().unwrap();
    let manifest: InstalledArtifactManifest =
        registry::load_set(&repo_root(), "installed-artifact-manifest").expect("manifest loads");
    for entry in &manifest.entries {
        let p = &entry.path;
        if entry.resolution == "worded-absence"
            || p.contains(' ')
            || p.contains('—')
            || p.starts_with("~/")
        {
            continue;
        }
        let concrete = p.replace('*', "payload");
        let target = dir.path().join(concrete.trim_end_matches('/'));
        if concrete.ends_with('/') {
            fs::create_dir_all(&target).unwrap();
        } else {
            fs::create_dir_all(target.parent().unwrap()).unwrap();
            fs::write(&target, "fixture payload").unwrap();
        }
    }

    let findings = installed_artifact_integrity_check(dir.path(), dir.path(), &manifest);
    let failures: Vec<_> = findings
        .iter()
        .filter(|f| f.result == CheckResult::Fail)
        .collect();
    assert!(
        failures.is_empty(),
        "no entry fails on the complete tree: {failures:?}"
    );

    // The inconclusive set is pinned by EQUALITY against the set the
    // manifest itself implies (vsdd-cli #757): every prose-path entry
    // (its surface-specific check lands with its consumer) plus every
    // created `exists-and-referenced` entry (the referenced-by half has
    // no check yet, vsdd-cli #746). Equality is the two-directional
    // pin — a regression that silently upgrades presence to pass makes
    // the sets differ, where a membership-only check stayed green.
    for finding in &findings {
        assert_eq!(
            finding.result,
            CheckResult::Inconclusive,
            "{}: only inconclusive findings remain on the complete tree",
            finding.entry_id
        );
    }
    let got: BTreeSet<String> = findings.iter().map(|f| f.entry_id.clone()).collect();
    let expected: BTreeSet<String> = manifest
        .entries
        .iter()
        .filter(|e| e.resolution != "worded-absence")
        .filter(|e| {
            let prose = e.path.contains(' ') || e.path.contains('—');
            let home_anchored = e.path.starts_with("~/");
            prose || (!home_anchored && e.resolution == "exists-and-referenced")
        })
        .map(|e| e.id.clone())
        .collect();
    assert_eq!(
        got, expected,
        "the inconclusive set equals the manifest-implied set, both directions"
    );
    assert!(
        got.contains("githook-wiring"),
        "the wiring entry the manifest names in prose is among the pinned set"
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
    let findings = installed_artifact_integrity_check(dir.path(), dir.path(), &manifest);
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
    let findings = installed_artifact_integrity_check(&repo_root(), elsewhere.path(), &manifest);
    let first = findings.first().expect("the binding member reports first");
    assert_eq!(first.result, CheckResult::Fail);
    assert!(
        first.detail.contains("project root"),
        "the finding names the binding: {}",
        first.detail
    );
}

#[test]
fn ref_normalization_strips_any_remote_and_skips_symbolic_heads() {
    // The pure half of the two-query listing (vsdd-cli #752, full
    // refnames on both halves per #761): the remote segment strips
    // structurally for ANY remote name, and each remote's symbolic
    // HEAD never enters the membership set.
    let normalized = normalize_ref_lines(
        "refs/heads/main\nrefs/heads/feature/statusline-wiring\n",
        "refs/remotes/origin/HEAD\nrefs/remotes/origin/main\nrefs/remotes/upstream/HEAD\nrefs/remotes/upstream/issue/42\n",
    );
    assert_eq!(
        normalized,
        vec![
            "main".to_string(),
            "feature/statusline-wiring".to_string(),
            "issue/42".to_string()
        ],
        "locals first, remotes stripped for origin and upstream alike, HEADs skipped, main deduplicated"
    );
}

#[test]
fn a_local_branch_resembling_a_remote_ref_is_never_mangled() {
    // The hardcoded-remote defect the rework removes (vsdd-cli #752): a
    // LOCAL branch literally named `origin/x` is a name, not a remote
    // ref — it must survive for the grammar to judge it. Full refnames
    // make this structural (vsdd-cli #761): `refs/heads/origin/x`
    // strips exactly its namespace, immune to the short form's
    // tag-collision ambiguity.
    let normalized = normalize_ref_lines("refs/heads/origin/x\n", "");
    assert_eq!(
        normalized,
        vec!["origin/x".to_string()],
        "the local branch keeps its full name past the namespace"
    );
}

#[test]
fn ref_normalization_preserves_nested_branch_paths() {
    // Multi-segment branch names keep every segment past the remote:
    // only the remote name strips, never inner path structure.
    let normalized = normalize_ref_lines("", "refs/remotes/origin/feature/deep/nesting\n");
    assert_eq!(normalized, vec!["feature/deep/nesting".to_string()]);
}

#[test]
fn worded_absence_passes_by_declaration() {
    // statusline-wiring declares its absence in words; the check honors
    // the declaration rather than probing a path that is prose.
    let manifest: InstalledArtifactManifest =
        registry::load_set(&repo_root(), "installed-artifact-manifest").expect("manifest loads");
    let findings = installed_artifact_integrity_check(&repo_root(), &repo_root(), &manifest);
    assert!(
        !findings
            .iter()
            .any(|f| f.entry_id == "statusline-wiring" && f.result != CheckResult::Pass),
        "the worded absence is a pass, not a probe failure"
    );
}
