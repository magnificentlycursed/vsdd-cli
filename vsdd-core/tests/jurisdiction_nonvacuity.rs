//! Standing non-vacuity proof for the REAL `.mdatron/` jurisdiction
//! (vsdd-cli #822 M2 / #825 F5; updated for mdatron 0.4.0, #830).
//!
//! mdatron 0.4.0 reports `files_checked` as the count of files VALIDATED (79 on
//! the real tree; it was 0 under 0.3.0, which counted only files-WITH-findings),
//! so a misscoped jurisdiction that matched nothing now shows `files_checked: 0`
//! rather than passing silently. This canary is the stronger, behavioral proof
//! beyond that count: it copies the real `.mdatron/` config + schemas into a
//! temp project, drops a frontmatter-less governed file at a path the real
//! `file_globs` + `require_frontmatter` match, and asserts mdatron CATCHES it
//! (MDATRON-W0040) — proving the RULES fire, not merely that files were walked.
//! If the real jurisdiction ever goes vacuous (globs match nothing, config fails
//! to load, the rule stops firing), this goes red.
//!
//! Runs the mdatron BINARY (the #739 tool-to-tool boundary), like
//! cross_references; mdatron on PATH is the estate's tooling requirement.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn copy_files(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            fs::copy(&path, dst.join(entry.file_name())).unwrap();
        }
    }
}

#[test]
fn the_real_jurisdiction_catches_an_injected_violation() {
    let root = repo_root();
    let dir = tempfile::tempdir().unwrap();
    let proj = dir.path();

    // The REAL config under test (its file_globs + require_frontmatter) and the
    // real schemas. Patterns are intentionally NOT copied: W0040 is a
    // config-level (require_frontmatter) check, and copying registry-integrity's
    // key-index rules without their source files would fail the pipeline
    // (MDATRON-E0080) for an unrelated reason.
    fs::create_dir_all(proj.join(".mdatron")).unwrap();
    fs::copy(
        root.join(".mdatron/config.yaml"),
        proj.join(".mdatron/config.yaml"),
    )
    .expect("the real .mdatron/config.yaml exists");
    copy_files(&root.join(".mdatron/schemas"), &proj.join(".mdatron/schemas"));

    // Seed the managed manifest the adopter way (idempotent; preserves the
    // copied config + schemas).
    let init = std::process::Command::new("mdatron")
        .args(["init", "-q", "--project-root"])
        .arg(proj)
        .status()
        .expect("mdatron runs from PATH — the estate's tooling requirement");
    assert!(init.success(), "mdatron init seeds the managed manifest");

    // Inject a naked (frontmatter-less) governed file at a path the REAL
    // file_globs match (.claude/commands/vsdd-*.md, also in require_frontmatter).
    let canary = proj.join(".claude/commands/vsdd-domain-canary.md");
    fs::create_dir_all(canary.parent().unwrap()).unwrap();
    fs::write(&canary, "# a naked governed file — no frontmatter\n").unwrap();

    let output = std::process::Command::new("mdatron")
        .args(["verify", "--json", "-q", "--project-root"])
        .arg(proj)
        .output()
        .expect("mdatron runs from PATH — the estate's tooling requirement");
    let parsed: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&output.stdout).trim())
            .expect("mdatron --json output parses");

    // The real config LOADED (a genuine run, not an E0080 pipeline failure) ...
    assert_eq!(
        parsed["pipeline_status"].as_str(),
        Some("ok"),
        "the real config loads and the pipeline runs: {parsed}"
    );
    // ... and the REAL jurisdiction CAUGHT the naked file — proof the file_globs
    // resolve and require_frontmatter is live, i.e. a clean verify is not vacuous.
    let codes: Vec<&str> = parsed["findings"]
        .as_array()
        .expect("the envelope carries a findings array")
        .iter()
        .filter_map(|f| f["code"].as_str())
        .collect();
    assert!(
        codes.contains(&"MDATRON-W0040"),
        "the real file_globs + require_frontmatter catch a naked governed file \
         (the jurisdiction is non-vacuous); got {codes:?}"
    );
}
