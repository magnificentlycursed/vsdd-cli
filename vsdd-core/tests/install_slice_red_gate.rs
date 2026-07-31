//! Phase 2a Red Gate for Slice 3 (Install) — the STATIC-MEMBER half.
//!
//! Oracle for `.design/install-slice.md` (16 REQ / 16 AC), crosslink #838.
//! Each test encodes one ratified acceptance criterion's behavior and FAILS
//! against the current `vsdd_core::init` on its named assertion — the
//! oracle-before-implementation discipline. Phase 2b turns them green.
//!
//! AC → test map (see the per-test doc lines):
//!   AC-1  ac1_three_way_classification_yields_four_states
//!   AC-2  ac2_dry_run_writes_nothing
//!   AC-3  ac3_force_overwrites_conflict
//!   AC-4  ac4_update_applies_toolkit_upgrade_only
//!   AC-5  ac5_no_prompt_and_ci_mode_skip_conflict
//!   AC-6  ac6_conflict_resolution_is_per_file
//!   AC-7  ac7_manifest_records_template_version_at_deploy
//!         ac7_pre_field_manifest_sha_first_migration
//!   AC-8  ac8_idempotent_including_generated_manifest
//!   AC-9  ac9_unmanaged_file_at_template_dest_is_conflict
//!   AC-10 ac10_delete_subset_reconverges
//!   AC-11 ac11_deploys_fifteen_templates_count_62
//!   AC-12 ac12_managed_template_drift_and_upgrade
//!   AC-13 ac13_template_destinations_and_design_scaffold
//!   AC-14 ac14_registry_loader_reads_vsdd_registry
//!   AC-15 ac15_generates_installed_artifact_manifest
//!   AC-16 ac16_scaffold_design_deploy_if_absent
//!         ac16_section_managed_vocabulary_preserves_operator_region
//!
//! DEFERRED (out of the static-member half — needs Slice 2's generator):
//!   deploying the GENERATED members (skills, generated-form domain prompts).
//!   None of install-slice.md's 16 ACs targets generated-member deployment, so
//!   no AC is dropped here; that surface is a separate work item.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use vsdd_core::init::{classify, init, Classification, ConflictChoice, InitOptions};

// ── Scaffold ─────────────────────────────────────────────────────────────────

/// The vsdd-cli source tree root (parent of the `vsdd-core` manifest dir).
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// A throwaway project root; a `.git` marker is created so `init` passes the
/// non-git refusal (unchanged REQ-9 gate).
struct TempProject(tempfile::TempDir);

impl TempProject {
    fn bare() -> Self {
        Self(tempfile::tempdir().unwrap())
    }
    fn git() -> Self {
        let p = Self::bare();
        fs::create_dir_all(p.path().join(".git")).unwrap();
        p
    }
    fn path(&self) -> &Path {
        self.0.path()
    }
}

fn read_manifest(project: &Path) -> serde_json::Value {
    let raw = fs::read_to_string(project.join(".vsdd/init-manifest.json"))
        .expect("init-manifest.json is readable");
    serde_json::from_str(&raw).expect("init-manifest.json is valid JSON")
}

// ── AC-1 (REQ-1): three-way classification yields four named states ──────────

/// Asserts the triple (manifest, disk, template) classifies into exactly
/// Unchanged / ToolkitUpgrade / Conflict / Missing.
#[test]
fn ac1_three_way_classification_yields_four_states() {
    assert_eq!(
        classify(Some("t"), Some("t"), "t"),
        Classification::Unchanged,
        "AC-1: all three hashes equal → Unchanged (skip)"
    );
    assert_eq!(
        classify(Some("x"), Some("x"), "t"),
        Classification::ToolkitUpgrade,
        "AC-1: disk == manifest != template → ToolkitUpgrade (updated, not refused)"
    );
    assert_eq!(
        classify(Some("x"), Some("y"), "t"),
        Classification::Conflict,
        "AC-1: disk != manifest → Conflict (not silently overwritten)"
    );
    assert_eq!(
        classify(Some("x"), None, "t"),
        Classification::Missing,
        "AC-1: destination absent → Missing (deploy)"
    );
}

// ── AC-2 (REQ-2): --dry-run prints the plan and writes nothing ───────────────

/// Asserts a dry run creates no manifest, no files, and no event, and that
/// `--force --dry-run` likewise writes nothing.
#[test]
fn ac2_dry_run_writes_nothing() {
    // Fresh repo (everything Missing): a dry run must write nothing and exit 0.
    let proj = TempProject::git();
    init(
        proj.path(),
        &InitOptions {
            dry_run: true,
            ..Default::default()
        },
    )
    .expect("AC-2: --dry-run exits 0");
    assert!(
        !proj.path().join(".vsdd/init-manifest.json").exists(),
        "AC-2: --dry-run writes no init-manifest.json"
    );
    assert!(
        !proj.path().join(".mdatron/schemas/phase-primer.json").exists(),
        "AC-2: --dry-run deploys no files"
    );
    let events = proj.path().join(".vsdd/events.jsonl");
    assert!(
        !events.exists() || fs::read_to_string(&events).unwrap().trim().is_empty(),
        "AC-2: --dry-run appends no event"
    );

    // --force --dry-run over a real repo with a Conflict still writes nothing.
    let proj2 = TempProject::git();
    init(proj2.path(), &InitOptions::default()).expect("first real init succeeds");
    let dest = proj2.path().join(".mdatron/schemas/phase-primer.json");
    let edit = b"{ \"operator\": \"edit\" }".to_vec();
    fs::write(&dest, &edit).unwrap();
    let manifest_before = fs::read(proj2.path().join(".vsdd/init-manifest.json")).unwrap();

    init(
        proj2.path(),
        &InitOptions {
            force: true,
            dry_run: true,
            ..Default::default()
        },
    )
    .expect("AC-2: --force --dry-run exits 0");
    assert_eq!(
        fs::read(&dest).unwrap(),
        edit,
        "AC-2: --force --dry-run does not overwrite the Conflict file"
    );
    assert_eq!(
        fs::read(proj2.path().join(".vsdd/init-manifest.json")).unwrap(),
        manifest_before,
        "AC-2: --force --dry-run does not rewrite the manifest"
    );
}

// ── AC-3 (REQ-3): --force overwrites a Conflict file ─────────────────────────

/// Asserts --force overwrites an operator-edited managed file and records the
/// template sha + template_version_at_deploy.
#[test]
fn ac3_force_overwrites_conflict() {
    let proj = TempProject::git();
    init(proj.path(), &InitOptions::default()).expect("first init succeeds");

    // Operator edits a managed file → Conflict (disk != manifest).
    let dest = proj.path().join(".mdatron/schemas/phase-primer.json");
    fs::write(&dest, b"{ \"operator\": \"edit\" }").unwrap();

    init(
        proj.path(),
        &InitOptions {
            force: true,
            no_prompt: true,
            ..Default::default()
        },
    )
    .expect("AC-3: --force run exits 0");

    assert_eq!(
        fs::read_to_string(&dest).unwrap(),
        vsdd_core::schemas::PHASE_PRIMER,
        "AC-3: --force overwrites the operator-edited file with the current template"
    );

    let manifest = read_manifest(proj.path());
    let entry = &manifest["files"][".mdatron/schemas/phase-primer.json"];
    assert_eq!(
        entry["sha256"].as_str(),
        Some(sha256_hex(vsdd_core::schemas::PHASE_PRIMER.as_bytes()).as_str()),
        "AC-3: the forced entry's sha256 matches the template"
    );
    assert_eq!(
        entry["template_version_at_deploy"].as_str(),
        Some(env!("CARGO_PKG_VERSION")),
        "AC-3: the forced entry records template_version_at_deploy = current toolkit version"
    );
}

// ── AC-4 (REQ-4): --update applies ToolkitUpgrade only ───────────────────────

/// Asserts --update writes ToolkitUpgrade files and leaves Conflict files
/// untouched, while --update --force also overwrites the Conflict.
#[test]
fn ac4_update_applies_toolkit_upgrade_only() {
    let proj = TempProject::git();
    init(proj.path(), &InitOptions::default()).expect("first init succeeds");

    // File A → ToolkitUpgrade: seed disk to an old body and align its recorded
    // sha, so disk == manifest != template.
    let a = proj.path().join(".mdatron/schemas/phase-primer.json");
    let old_a = b"{ \"old\": \"toolkit body\" }".to_vec();
    fs::write(&a, &old_a).unwrap();

    // File B → Conflict: operator edit; the recorded sha stays the template's.
    let b = proj.path().join(".mdatron/schemas/domain-prompt.json");
    let edit_b = b"{ \"operator\": \"edit B\" }".to_vec();
    fs::write(&b, &edit_b).unwrap();

    let mut manifest = read_manifest(proj.path());
    manifest["files"][".mdatron/schemas/phase-primer.json"]["sha256"] =
        serde_json::Value::String(sha256_hex(&old_a));
    fs::write(
        proj.path().join(".vsdd/init-manifest.json"),
        serde_json::to_string_pretty(&manifest).unwrap(),
    )
    .unwrap();

    init(
        proj.path(),
        &InitOptions {
            update: true,
            no_prompt: true,
            ..Default::default()
        },
    )
    .expect("AC-4: --update exits 0 without --force");

    assert_eq!(
        fs::read_to_string(&a).unwrap(),
        vsdd_core::schemas::PHASE_PRIMER,
        "AC-4: --update upgrades the ToolkitUpgrade file to the template"
    );
    assert_eq!(
        fs::read(&b).unwrap(),
        edit_b,
        "AC-4: --update leaves the Conflict file byte-unchanged"
    );

    init(
        proj.path(),
        &InitOptions {
            update: true,
            force: true,
            no_prompt: true,
            ..Default::default()
        },
    )
    .expect("AC-4: --update --force exits 0");
    assert_eq!(
        fs::read_to_string(&b).unwrap(),
        vsdd_core::schemas::DOMAIN_PROMPT,
        "AC-4: --update --force overwrites the Conflict file with the template"
    );
}

// ── AC-5 (REQ-5): --no-prompt and ci_mode skip Conflict, no prompt ───────────

/// Asserts a non-interactive run (via --no-prompt, and via ci_mode) skips a
/// Conflict with exit 0, and that adding --force overwrites it.
#[test]
fn ac5_no_prompt_and_ci_mode_skip_conflict() {
    for label in ["no_prompt", "ci_mode"] {
        let proj = TempProject::git();
        init(proj.path(), &InitOptions::default()).expect("first init succeeds");
        let dest = proj.path().join(".mdatron/schemas/phase-primer.json");
        let edit = b"{ \"operator\": \"edit\" }".to_vec();
        fs::write(&dest, &edit).unwrap();

        let mut skip = InitOptions::default();
        let mut forced = InitOptions {
            force: true,
            ..Default::default()
        };
        if label == "no_prompt" {
            skip.no_prompt = true;
            forced.no_prompt = true;
        } else {
            skip.ci_mode = true;
            forced.ci_mode = true;
        }

        init(proj.path(), &skip)
            .unwrap_or_else(|e| panic!("AC-5 [{label}]: a non-interactive Conflict is skipped (exit 0); got {e:?}"));
        assert_eq!(
            fs::read(&dest).unwrap(),
            edit,
            "AC-5 [{label}]: the Conflict file is skipped, not overwritten"
        );

        init(proj.path(), &forced).expect("AC-5: the forced run exits 0");
        assert_eq!(
            fs::read_to_string(&dest).unwrap(),
            vsdd_core::schemas::PHASE_PRIMER,
            "AC-5 [{label}]: --force overwrites the previously skipped Conflict"
        );
    }
}

// ── AC-6 (REQ-6): the Conflict resolution applies per file ───────────────────

/// Asserts the per-file resolution (the non-interactive stand-in for the
/// prompt) applies to each file independently: accept overwrites A, keep
/// preserves B.
#[test]
fn ac6_conflict_resolution_is_per_file() {
    let proj = TempProject::git();
    init(proj.path(), &InitOptions::default()).expect("first init succeeds");

    let a = proj.path().join(".mdatron/schemas/phase-primer.json");
    let b = proj.path().join(".mdatron/schemas/domain-prompt.json");
    let edit_b = b"{ \"operator\": \"edit B\" }".to_vec();
    fs::write(&a, b"{ \"operator\": \"edit A\" }").unwrap();
    fs::write(&b, &edit_b).unwrap();

    let mut resolved = BTreeMap::new();
    resolved.insert(
        ".mdatron/schemas/phase-primer.json".to_string(),
        ConflictChoice::AcceptNewTemplate,
    );
    resolved.insert(
        ".mdatron/schemas/domain-prompt.json".to_string(),
        ConflictChoice::KeepOperatorEdit,
    );

    init(
        proj.path(),
        &InitOptions {
            resolved_conflicts: resolved,
            ..Default::default()
        },
    )
    .expect("AC-6: the per-file resolution run exits 0");

    assert_eq!(
        fs::read_to_string(&a).unwrap(),
        vsdd_core::schemas::PHASE_PRIMER,
        "AC-6: the file resolved 'accept' is overwritten with the template"
    );
    assert_eq!(
        fs::read(&b).unwrap(),
        edit_b,
        "AC-6: the file resolved 'keep' is left byte-unchanged — the choice is per file"
    );
}

// ── AC-7 (REQ-7): template_version_at_deploy + sha-first migration ───────────

/// Asserts every manifest entry written by this slice carries
/// template_version_at_deploy.
#[test]
fn ac7_manifest_records_template_version_at_deploy() {
    let proj = TempProject::git();
    init(proj.path(), &InitOptions::default()).expect("init succeeds");

    let manifest = read_manifest(proj.path());
    let files = manifest["files"].as_object().expect("files is an object");
    assert!(!files.is_empty(), "the manifest records deployed files");
    for (rel, entry) in files {
        assert!(
            entry
                .get("template_version_at_deploy")
                .and_then(|v| v.as_str())
                .is_some(),
            "AC-7: manifest entry for {rel} carries template_version_at_deploy"
        );
    }
}

/// Asserts the sha-first migration of a pre-field manifest: a matching sha is
/// backfilled (no overwrite); a differing sha classifies Conflict (no silent
/// overwrite).
#[test]
fn ac7_pre_field_manifest_sha_first_migration() {
    // Case 1: recorded sha == disk → adopt + backfill the field, no overwrite.
    let proj = TempProject::git();
    init(proj.path(), &InitOptions::default()).expect("first init succeeds");
    let dest = proj.path().join(".mdatron/schemas/phase-primer.json");
    let deployed = fs::read(&dest).unwrap();

    strip_version_field(proj.path());
    init(
        proj.path(),
        &InitOptions {
            no_prompt: true,
            ..Default::default()
        },
    )
    .expect("re-init over a pre-field manifest exits 0");
    assert_eq!(
        fs::read(&dest).unwrap(),
        deployed,
        "AC-7: a pre-field entry whose sha == disk is adopted, not overwritten"
    );
    let migrated = read_manifest(proj.path());
    assert!(
        migrated["files"][".mdatron/schemas/phase-primer.json"]
            .get("template_version_at_deploy")
            .and_then(|v| v.as_str())
            .is_some(),
        "AC-7: the migrated pre-field entry is backfilled with template_version_at_deploy"
    );

    // Case 2: recorded sha != disk → Conflict, never a silent overwrite.
    let proj2 = TempProject::git();
    init(proj2.path(), &InitOptions::default()).expect("first init succeeds");
    let dest2 = proj2.path().join(".mdatron/schemas/phase-primer.json");
    let edit = b"{ \"operator\": \"edit\" }".to_vec();
    fs::write(&dest2, &edit).unwrap();
    strip_version_field(proj2.path());

    init(
        proj2.path(),
        &InitOptions {
            no_prompt: true,
            ..Default::default()
        },
    )
    .expect("AC-7: re-init over a drifted pre-field manifest is a Conflict skip, not an error");
    assert_eq!(
        fs::read(&dest2).unwrap(),
        edit,
        "AC-7: a pre-field entry whose sha != disk is a Conflict (preserved), never overwritten"
    );
}

/// Rewrite the manifest to the pre-field shape (no template_version_at_deploy).
fn strip_version_field(project: &Path) {
    let mut manifest = read_manifest(project);
    for (_k, v) in manifest["files"].as_object_mut().unwrap() {
        v.as_object_mut().unwrap().remove("template_version_at_deploy");
    }
    fs::write(
        project.join(".vsdd/init-manifest.json"),
        serde_json::to_string_pretty(&manifest).unwrap(),
    )
    .unwrap();
}

// ── AC-8 (REQ-8): idempotence, including the generated manifest ──────────────

/// Asserts a converged re-run deploys nothing, appends no second event, and
/// leaves the generated installed-artifact-manifest byte-identical.
#[test]
fn ac8_idempotent_including_generated_manifest() {
    let proj = TempProject::git();
    init(proj.path(), &InitOptions::default()).expect("first init succeeds");

    let gen = proj
        .path()
        .join(".vsdd/registry/installed-artifact-manifest.md");
    let after_first = fs::read(&gen)
        .expect("AC-8: the generated installed-artifact-manifest is present after the first init");

    let second = init(proj.path(), &InitOptions::default()).expect("second init succeeds");
    assert!(
        second.deployed.is_empty(),
        "AC-8: a converged re-run deploys nothing; got {:?}",
        second.deployed
    );

    let after_second = fs::read(&gen).expect("the generated manifest is still present");
    assert_eq!(
        after_first, after_second,
        "AC-8: the generated manifest is deterministic — byte-identical across converged runs"
    );

    let log = fs::read_to_string(proj.path().join(".vsdd/events.jsonl")).unwrap();
    let lines: Vec<&str> = log.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines.len(),
        1,
        "AC-8: no second ProjectInitialized event on the idempotent re-run; got {lines:?}"
    );
}

// ── AC-9 (REQ-9): an unmanaged file at a template dest is a Conflict ─────────

/// Asserts a pre-existing unmanaged file at a template destination is not
/// silently overwritten, while a non-colliding sibling template is deployed.
#[test]
fn ac9_unmanaged_file_at_template_dest_is_conflict() {
    let proj = TempProject::git();
    let collide = proj.path().join(".github/workflows/vsdd-verify.yml");
    fs::create_dir_all(collide.parent().unwrap()).unwrap();
    let operator = b"# operator's own workflow\n".to_vec();
    fs::write(&collide, &operator).unwrap();

    init(
        proj.path(),
        &InitOptions {
            no_prompt: true,
            ..Default::default()
        },
    )
    .expect("AC-9: init over a colliding unmanaged file exits 0 (Conflict skip)");

    assert_eq!(
        fs::read(&collide).unwrap(),
        operator,
        "AC-9: a pre-existing unmanaged file at a template dest is not silently overwritten"
    );
    assert!(
        proj.path()
            .join(".github/workflows/vsdd-observe-pr-body.yml")
            .exists(),
        "AC-9: the non-colliding sibling template is deployed (templates are in the plan)"
    );
}

// ── AC-10 (REQ-10): re-run-to-converge; manifest written last ────────────────

/// Asserts deleting a subset of deployed files and re-running re-deploys the
/// deleted files (Missing) and converges to a clean-init manifest.
#[test]
fn ac10_delete_subset_reconverges() {
    let proj = TempProject::git();
    init(proj.path(), &InitOptions::default()).expect("first init succeeds");

    let template_dest = proj.path().join(".vsdd/registry/gate-data.md");
    assert!(
        template_dest.exists(),
        "AC-10: the first init deploys the registry template (a member of the converge set)"
    );

    let schema_dest = proj.path().join(".mdatron/schemas/phase-primer.json");
    fs::remove_file(&schema_dest).unwrap();
    fs::remove_file(&template_dest).unwrap();

    init(
        proj.path(),
        &InitOptions {
            no_prompt: true,
            ..Default::default()
        },
    )
    .expect("re-init succeeds");
    assert!(
        schema_dest.exists(),
        "AC-10: the deleted schema is re-deployed (Missing → deploy)"
    );
    assert!(
        template_dest.exists(),
        "AC-10: the deleted template is re-deployed (Missing → deploy)"
    );

    let clean = TempProject::git();
    init(clean.path(), &InitOptions::default()).expect("clean init succeeds");
    assert_eq!(
        read_manifest(proj.path()),
        read_manifest(clean.path()),
        "AC-10: the re-converged manifest equals a clean first init's manifest"
    );
}

// ── AC-11 (REQ-11): deploys 15 templates; generates the 16th; count = 62 ─────

/// Asserts all 15 template destinations are present and manifest-tracked, the
/// generated manifest is present but not a deployed-template entry, and the
/// event count is 62.
#[test]
fn ac11_deploys_fifteen_templates_count_62() {
    let proj = TempProject::git();
    init(proj.path(), &InitOptions::default()).expect("init succeeds");

    let templates = [
        ".github/workflows/vsdd-verify.yml",
        ".github/workflows/vsdd-observe-pr-body.yml",
        "DESIGN.md",
        ".vsdd/statusline/vsdd-statusline.sh",
        ".vsdd/registry/act-to-affordance-map.md",
        ".vsdd/registry/composition-scope-and-actions.md",
        ".vsdd/registry/dispatch-data.md",
        ".vsdd/registry/economics-data.md",
        ".vsdd/registry/gate-data.md",
        ".vsdd/registry/snapshot-schema.md",
        ".vsdd/registry/state-schema.md",
        ".vsdd/registry/statusline-data.md",
        ".vsdd/registry/anonymization-patterns.yaml",
        ".vsdd/registry/canonical-patterns.yaml",
        ".vsdd/registry/vocabulary.yaml",
    ];
    assert_eq!(templates.len(), 15, "the deployed template set is 15 files");

    let manifest = read_manifest(proj.path());
    let files = manifest["files"].as_object().unwrap();
    for t in templates {
        assert!(
            proj.path().join(t).exists(),
            "AC-11: template {t} is deployed to its destination"
        );
        assert!(
            files.contains_key(t),
            "AC-11: template {t} is recorded in init-manifest.json"
        );
    }

    assert!(
        proj.path()
            .join(".vsdd/registry/installed-artifact-manifest.md")
            .exists(),
        "AC-11: the generated installed-artifact-manifest is present"
    );
    assert!(
        !files.contains_key(".vsdd/registry/installed-artifact-manifest.md"),
        "AC-11: the generated manifest is not a deployed-template manifest entry"
    );

    let log = fs::read_to_string(proj.path().join(".vsdd/events.jsonl")).unwrap();
    let event: serde_json::Value =
        serde_json::from_str(log.lines().next().expect("one event line")).unwrap();
    assert_eq!(
        event["deployed_artifact_count"].as_u64(),
        Some(62),
        "AC-11: deployed_artifact_count = 47 prior + 15 templates"
    );
}

// ── AC-12 (REQ-12): a managed template is drift-tracked whole-file ───────────

/// Asserts a deployed managed-class template is drift-tracked and overwritten
/// under --force (whole-file classification).
#[test]
fn ac12_managed_template_drift_and_upgrade() {
    let proj = TempProject::git();
    init(proj.path(), &InitOptions::default()).expect("first init succeeds");

    let dest = proj.path().join(".vsdd/registry/gate-data.md");
    let template = fs::read_to_string(repo_root().join("templates/registry/gate-data.md")).unwrap();
    assert!(
        dest.exists(),
        "AC-12: the managed registry template is deployed by init"
    );
    assert_eq!(
        fs::read_to_string(&dest).unwrap(),
        template,
        "AC-12: the deployed content equals the template"
    );

    fs::write(&dest, b"# operator edit\n").unwrap();
    init(
        proj.path(),
        &InitOptions {
            force: true,
            no_prompt: true,
            ..Default::default()
        },
    )
    .expect("forced re-init succeeds");
    assert_eq!(
        fs::read_to_string(&dest).unwrap(),
        template,
        "AC-12: a managed template Conflict is overwritten under --force (whole-file)"
    );
}

// ── AC-13 (REQ-13): template destinations + DESIGN scaffold ──────────────────

/// Asserts every template lands at its REQ-13 destination, the DESIGN scaffold
/// deploys verbatim when absent, and an existing DESIGN.md is left untouched.
#[test]
fn ac13_template_destinations_and_design_scaffold() {
    let proj = TempProject::git();
    init(proj.path(), &InitOptions::default()).expect("init succeeds");

    assert!(
        proj.path().join(".github/workflows/vsdd-verify.yml").exists(),
        "AC-13: the verify workflow lands at .github/workflows/"
    );
    assert!(
        proj.path()
            .join(".github/workflows/vsdd-observe-pr-body.yml")
            .exists(),
        "AC-13: the observe workflow lands at .github/workflows/"
    );
    assert!(
        proj.path()
            .join(".vsdd/statusline/vsdd-statusline.sh")
            .exists(),
        "AC-13: the statusline script lands at .vsdd/statusline/"
    );
    assert!(
        proj.path().join(".vsdd/registry/vocabulary.yaml").exists(),
        "AC-13: each registry set lands at .vsdd/registry/"
    );

    let design = proj.path().join("DESIGN.md");
    assert!(design.exists(), "AC-13: the DESIGN scaffold deploys when absent");
    let design_src =
        fs::read_to_string(repo_root().join("templates/DESIGN.md.vsdd-template")).unwrap();
    assert_eq!(
        fs::read_to_string(&design).unwrap(),
        design_src,
        "AC-13: the DESIGN template deploys verbatim (operator-fill placeholders, no substitution)"
    );

    // An existing DESIGN.md is left untouched and not entered as a drift Conflict.
    let proj2 = TempProject::git();
    let existing = proj2.path().join("DESIGN.md");
    fs::write(&existing, b"# my design\n").unwrap();
    init(
        proj2.path(),
        &InitOptions {
            no_prompt: true,
            ..Default::default()
        },
    )
    .expect("AC-13: init over an existing DESIGN.md exits 0 (no Conflict error)");
    assert_eq!(
        fs::read(&existing).unwrap(),
        b"# my design\n",
        "AC-13: an existing DESIGN.md (scaffold) is left untouched"
    );
}

// ── AC-14 (REQ-14): the registry loader reads .vsdd/registry/ ────────────────

/// Asserts load_set reads deployed sets from .vsdd/registry/, and falls back to
/// templates/registry/ when the former is absent.
#[test]
fn ac14_registry_loader_reads_vsdd_registry() {
    use vsdd_core::registry;
    use vsdd_core::registry::sets::StatuslineData;

    // Estate with the set at .vsdd/registry/ (REQ-13 dest) and NO templates/registry/.
    let estate = tempfile::tempdir().unwrap();
    let reg = estate.path().join(".vsdd/registry");
    fs::create_dir_all(&reg).unwrap();
    fs::copy(
        repo_root().join("templates/registry/statusline-data.md"),
        reg.join("statusline-data.md"),
    )
    .unwrap();
    let schemas = estate.path().join(".mdatron/schemas");
    fs::create_dir_all(&schemas).unwrap();
    fs::copy(
        repo_root().join(".mdatron/schemas/statusline-data.json"),
        schemas.join("statusline-data.json"),
    )
    .unwrap();

    let loaded: Result<StatuslineData, _> = registry::load_set(estate.path(), "statusline-data");
    assert!(
        loaded.is_ok(),
        "AC-14: the loader reads deployed sets from .vsdd/registry/; got {:?}",
        loaded.err().map(|d| d.message.clone())
    );

    // Fallback: the source repo (no .vsdd/registry/) still loads from templates/registry/.
    let src: Result<StatuslineData, _> = registry::load_set(&repo_root(), "statusline-data");
    assert!(
        src.is_ok(),
        "AC-14: absent .vsdd/registry/, the loader falls back to templates/registry/; got {:?}",
        src.err().map(|d| d.message.clone())
    );
}

// ── AC-15 (REQ-15): init generates the adopter's installed-artifact manifest ─

/// Asserts init generates .vsdd/registry/installed-artifact-manifest.md over
/// the deployed set (not the vsdd-cli-specific template), it validates against
/// the schema, and a re-run rewrites nothing.
#[test]
fn ac15_generates_installed_artifact_manifest() {
    use vsdd_core::registry;
    use vsdd_core::registry::sets::InstalledArtifactManifest;

    let proj = TempProject::git();
    init(proj.path(), &InitOptions::default()).expect("init succeeds");

    let gen = proj
        .path()
        .join(".vsdd/registry/installed-artifact-manifest.md");
    let bytes1 = fs::read(&gen)
        .expect("AC-15: init generates .vsdd/registry/installed-artifact-manifest.md");

    let src =
        fs::read(repo_root().join("templates/registry/installed-artifact-manifest.md")).unwrap();
    assert_ne!(
        bytes1, src,
        "AC-15: the generated manifest is not the vsdd-cli-specific template copied verbatim"
    );

    // Validates against its schema pair (copy the pair into the adopter tree).
    let schemas = proj.path().join(".mdatron/schemas");
    fs::create_dir_all(&schemas).unwrap();
    fs::copy(
        repo_root().join(".mdatron/schemas/installed-artifact-manifest.json"),
        schemas.join("installed-artifact-manifest.json"),
    )
    .unwrap();
    let loaded: Result<InstalledArtifactManifest, _> =
        registry::load_set(proj.path(), "installed-artifact-manifest");
    let manifest = loaded.unwrap_or_else(|d| {
        panic!("AC-15: the generated manifest validates against its schema pair; got {}", d.message)
    });
    assert!(
        !manifest.entries.is_empty(),
        "AC-15: the generated manifest enumerates the deployed artifacts"
    );

    init(proj.path(), &InitOptions::default()).expect("second init succeeds");
    let bytes2 = fs::read(&gen).unwrap();
    assert_eq!(
        bytes1, bytes2,
        "AC-15: a converged re-run leaves the generated manifest byte-identical"
    );
}

// ── AC-16 (REQ-16): per-artifact management class ────────────────────────────

/// Asserts the scaffold class (DESIGN.md): deploy-if-absent, and exempt from
/// ToolkitUpgrade once present (untouched under --update).
#[test]
fn ac16_scaffold_design_deploy_if_absent() {
    let proj = TempProject::git();
    init(proj.path(), &InitOptions::default()).expect("init succeeds");
    let design = proj.path().join("DESIGN.md");
    let src = fs::read_to_string(repo_root().join("templates/DESIGN.md.vsdd-template")).unwrap();
    assert!(design.exists(), "AC-16: the scaffold DESIGN.md deploys when absent");
    assert_eq!(
        fs::read_to_string(&design).unwrap(),
        src,
        "AC-16: the scaffold deploys from the template"
    );

    fs::write(&design, b"# operator design\n").unwrap();
    init(
        proj.path(),
        &InitOptions {
            update: true,
            no_prompt: true,
            ..Default::default()
        },
    )
    .expect("update run succeeds");
    assert_eq!(
        fs::read(&design).unwrap(),
        b"# operator design\n",
        "AC-16: a scaffold present on disk is left untouched under --update (no ToolkitUpgrade)"
    );
}

/// Asserts the section-managed class (vocabulary.yaml): --force overwrites the
/// tool-owned managed region while preserving the operator-extension region
/// below the End anchor byte-for-byte.
#[test]
fn ac16_section_managed_vocabulary_preserves_operator_region() {
    let proj = TempProject::git();
    init(proj.path(), &InitOptions::default()).expect("init succeeds");

    let dest = proj.path().join(".vsdd/registry/vocabulary.yaml");
    assert!(
        dest.exists(),
        "AC-16: the section-managed vocabulary set is deployed"
    );
    let template = fs::read_to_string(&dest).unwrap();
    let anchor = "# === End vsdd managed ===";
    assert!(
        template.contains(anchor),
        "AC-16: the deployed set carries the managed-section end anchor"
    );

    // The operator appends an extension below the End anchor.
    let operator_line = "\n  - term: OperatorTerm\n    definition: operator's own\n";
    fs::write(&dest, format!("{template}{operator_line}")).unwrap();

    // Forced re-init: for a section-managed file, --force overwrites the managed
    // region but preserves the operator extension (a whole-file managed
    // treatment would wipe it).
    init(
        proj.path(),
        &InitOptions {
            force: true,
            no_prompt: true,
            ..Default::default()
        },
    )
    .expect("forced re-init succeeds");
    let after = fs::read_to_string(&dest).unwrap();
    assert!(
        after.contains("OperatorTerm"),
        "AC-16: the operator-extension region below the End anchor is preserved under --force"
    );
    let managed_end = after.find(anchor).expect("End anchor present after re-init");
    let tmpl_end = template.find(anchor).unwrap();
    assert_eq!(
        &after[..managed_end],
        &template[..tmpl_end],
        "AC-16: the tool-owned managed region matches the current template"
    );
}
