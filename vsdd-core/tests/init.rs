//! Phase 2a Red Gate for `vsdd_core::init`.
//!
//! Each test asserts one behavioral contract from the Phase 1c v0.1 init scope:
//!
//!   1. refuses non-git substrate (PE tightening)
//!   2. deploys all expected artifacts (schemas + pattern + 10 primers + 18 domains + 14 supplements)
//!   3. creates the .vsdd/ skeleton
//!   4. writes init-manifest.json with SHA-256 per deployed file
//!   5. emits ProjectInitialized event
//!   6. idempotent on unchanged state
//!   7. refuses drifted managed file with clear error (DR caveat)
//!
//! Tests fail to compile until Phase 2b lands `vsdd_core::init` + `vsdd_core::artifacts`.
//! Once compiling, fires-cases initially produce empty/wrong reports; Phase 2b implementation
//! turns each green.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use vsdd_core::init::{init, InitError, InitOptions};

// ── Tempdir scaffold ───────────────────────────────────────────────────────────

struct TempProject(PathBuf);

impl TempProject {
    fn new(label: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("vsdd-init-{label}-{nanos}"));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn with_git(label: &str) -> Self {
        let proj = Self::new(label);
        fs::create_dir_all(proj.path().join(".git")).unwrap();
        proj
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempProject {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

// ── 1. Refuses non-git substrate ──────────────────────────────────────────────

#[test]
fn init_refuses_non_git_substrate() {
    let proj = TempProject::new("no-git");
    let result = init(proj.path(), &InitOptions::default());
    assert!(
        matches!(result, Err(InitError::SubstrateNotGit { .. })),
        "expected InitError::SubstrateNotGit; got {result:?}"
    );
}

// ── 2. Deploys all expected artifacts ─────────────────────────────────────────

#[test]
fn init_deploys_all_expected_artifacts() {
    let proj = TempProject::with_git("deploys");
    let report = init(proj.path(), &InitOptions::default()).expect("init succeeds");

    // Schemas (4)
    for name in &[
        "phase-primer.json",
        "domain-prompt.json",
        "supplement.json",
        "review-entry.json",
    ] {
        let p = proj.path().join(".mdatron/schemas").join(name);
        assert!(p.exists(), "schema {} should be deployed", p.display());
    }

    // Pattern (1)
    assert!(proj
        .path()
        .join(".mdatron/patterns/cross-references.yaml")
        .exists());

    // Phase primers (10), domain prompts (18), supplements (14)
    assert_eq!(
        vsdd_core::artifacts::PHASE_PRIMERS.len(),
        10,
        "phase primer registry should have 10 entries"
    );
    assert_eq!(
        vsdd_core::artifacts::DOMAIN_PROMPTS.len(),
        18,
        "domain prompt registry should have 18 entries"
    );
    assert_eq!(
        vsdd_core::artifacts::SUPPLEMENTS.len(),
        14,
        "supplement registry should have 14 entries"
    );

    for (name, _content) in vsdd_core::artifacts::PHASE_PRIMERS {
        let p = proj.path().join(".claude/commands").join(name);
        assert!(p.exists(), "phase primer {} should be deployed", p.display());
    }
    for (name, _content) in vsdd_core::artifacts::DOMAIN_PROMPTS {
        let p = proj.path().join(".claude/commands").join(name);
        assert!(p.exists(), "domain prompt {} should be deployed", p.display());
    }
    for (name, _content) in vsdd_core::artifacts::SUPPLEMENTS {
        let p = proj.path().join("supplements").join(name);
        assert!(p.exists(), "supplement {} should be deployed", p.display());
    }

    // Report exposes deployment list
    assert!(
        !report.deployed.is_empty(),
        "InitReport.deployed should list emitted files"
    );
}

// ── 3. Creates the .vsdd/ skeleton ────────────────────────────────────────────

#[test]
fn init_creates_vsdd_skeleton() {
    let proj = TempProject::with_git("vsdd-skel");
    init(proj.path(), &InitOptions::default()).expect("init succeeds");

    assert!(proj.path().join(".vsdd/events.jsonl").exists());
    assert!(proj.path().join(".vsdd/config.yaml").exists());
    assert!(proj.path().join(".vsdd/init-manifest.json").exists());
}

// ── 4. Writes init-manifest.json with SHA-256 per deployed file ───────────────

#[test]
fn init_writes_manifest_with_sha256_hashes() {
    let proj = TempProject::with_git("manifest");
    let report = init(proj.path(), &InitOptions::default()).expect("init succeeds");

    let manifest_path = proj.path().join(".vsdd/init-manifest.json");
    let raw = fs::read_to_string(&manifest_path).expect("manifest readable");
    let manifest: serde_json::Value = serde_json::from_str(&raw).expect("manifest is valid JSON");

    let entries = manifest
        .get("files")
        .and_then(|v| v.as_object())
        .expect("manifest.files is an object");

    assert!(
        !entries.is_empty(),
        "manifest should record at least one deployed file"
    );

    // Each deployed file (other than the manifest itself + the events.jsonl audit log,
    // which both legitimately mutate after init) should have an entry whose sha256
    // matches the actual file content.
    for path in &report.deployed {
        let rel = path
            .strip_prefix(proj.path())
            .expect("deployed path is under project root")
            .to_string_lossy()
            .replace('\\', "/");
        if rel == ".vsdd/init-manifest.json" || rel == ".vsdd/events.jsonl" {
            continue;
        }
        let entry = entries
            .get(&rel)
            .unwrap_or_else(|| panic!("manifest missing entry for {rel}"));
        let claimed = entry
            .get("sha256")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| panic!("entry for {rel} missing sha256"));
        let actual_bytes = fs::read(path).expect("file readable");
        let actual = sha256_hex(&actual_bytes);
        assert_eq!(
            claimed, actual,
            "manifest sha256 for {rel} should match file content"
        );
    }
}

// ── 5. Emits ProjectInitialized event ─────────────────────────────────────────

#[test]
fn init_emits_project_initialized_event() {
    let proj = TempProject::with_git("event");
    init(proj.path(), &InitOptions::default()).expect("init succeeds");

    let log = fs::read_to_string(proj.path().join(".vsdd/events.jsonl"))
        .expect("events.jsonl readable");
    let lines: Vec<&str> = log.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines.len(),
        1,
        "exactly one event line after a single init; got {lines:?}"
    );
    let event: serde_json::Value =
        serde_json::from_str(lines[0]).expect("event line is valid JSON");
    assert_eq!(
        event.get("event").and_then(|v| v.as_str()),
        Some("ProjectInitialized")
    );
    assert!(
        event.get("vsdd_version").is_some(),
        "event must carry vsdd_version field"
    );
    assert!(
        event.get("deployed_artifact_count").is_some(),
        "event must carry deployed_artifact_count field"
    );
}

// ── 6. Idempotent on unchanged state ──────────────────────────────────────────

#[test]
fn init_is_idempotent_on_unchanged_state() {
    let proj = TempProject::with_git("idempotent");
    let first = init(proj.path(), &InitOptions::default()).expect("first init succeeds");
    assert!(!first.deployed.is_empty(), "first init should deploy files");

    let second = init(proj.path(), &InitOptions::default()).expect("second init succeeds");
    assert!(
        second.deployed.is_empty(),
        "second init on unchanged state should deploy nothing; deployed={:?}",
        second.deployed
    );
    assert!(
        !second.skipped.is_empty(),
        "second init should record the skipped (unchanged) files"
    );

    // Event log should not grow on the idempotent re-run.
    let log = fs::read_to_string(proj.path().join(".vsdd/events.jsonl"))
        .expect("events.jsonl readable");
    let lines: Vec<&str> = log.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines.len(),
        1,
        "events.jsonl should still have exactly one line after idempotent re-run; got {lines:?}"
    );
}

// ── 7. Refuses drifted managed file with clear error (DR caveat) ──────────────

#[test]
fn init_refuses_drifted_managed_file_with_clear_error() {
    let proj = TempProject::with_git("drift");
    init(proj.path(), &InitOptions::default()).expect("first init succeeds");

    // Operator-edits a managed file (e.g., a phase primer) outside the managed-section
    // discipline. Re-init must refuse rather than silently overwrite the edit.
    let drifted = proj.path().join(".claude/commands/vsdd-phase-2a.md");
    let mut content = fs::read_to_string(&drifted).expect("primer readable");
    content.push_str("\n<!-- operator edit -->\n");
    fs::write(&drifted, &content).expect("primer writable");

    let result = init(proj.path(), &InitOptions::default());
    match result {
        Err(InitError::ManagedFileDrifted { path, .. }) => {
            assert!(
                path.ends_with("vsdd-phase-2a.md"),
                "error should name the drifted file; got {path:?}"
            );
            let display = format!("{}", InitError::ManagedFileDrifted {
                path: path.clone(),
                expected_sha256: "abc".into(),
                actual_sha256: "def".into(),
            });
            // DR caveat: the error must surface resolution flags so the operator
            // knows how to proceed without losing work.
            assert!(
                display.contains("--keep-operator-edits")
                    || display.contains("--accept-managed-defaults"),
                "drift error Display should name a resolution flag; got: {display}"
            );
        }
        other => panic!("expected InitError::ManagedFileDrifted; got {other:?}"),
    }
}

// ── Helpers ────────────────────────────────────────────────────────────────────

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    digest.iter().map(|b| format!("{b:02x}")).collect()
}
