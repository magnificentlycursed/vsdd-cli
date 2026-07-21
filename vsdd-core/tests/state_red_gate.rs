//! Layer 1 red gate — state artifact behaviors.
//!
//! Phase 2a suite (vsdd-cli #716): every test here asserts a named
//! behavior from the state-schema data set and the Deterministic phase
//! answer contract, and fails executed against the pre-implementation
//! stubs. Phase 2b turns this suite green. Criterion slice closed:
//! the state read-failure slice of Status detection — every read-failure
//! fixture yields a diagnostic, never a panic.

use std::fs;
use std::path::{Path, PathBuf};

use vsdd_core::registry::{self, sets::CompositionScopeAndActions, sets::StatuslineData};
use vsdd_core::state::{
    read_state, write_state, BoundaryEvidence, CompositionMode, GateKind, GateOutcome, State,
    SUPPORTED_STATE_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/state")
        .join(name)
}

/// The loaded vocabulary the reader draws its tokens from — the
/// kind-to-action mapping is versioned data, never a hardcoded copy.
fn vocabulary() -> StatuslineData {
    registry::load_set(&repo_root(), "statusline-data").expect("statusline data set loads")
}

/// The action vocabulary the write path draws its refusal token from
/// (registered per vsdd-cli #724).
fn actions() -> CompositionScopeAndActions {
    registry::load_set(&repo_root(), "composition-scope-and-actions")
        .expect("composition set loads")
}

#[test]
fn valid_state_reads_with_every_field() {
    let state = read_state(&fixture("valid.yaml"), &vocabulary()).expect("valid state reads");
    assert_eq!(state.schema_version, SUPPORTED_STATE_SCHEMA_VERSION);
    assert_eq!(state.current_phase.as_deref(), Some("phase-2a"));
    assert_eq!(state.current_layer, Some(1));
    assert_eq!(
        state.open_findings_pointer.milestone,
        "layer 1: state artifact and versioned data"
    );
    let gate = state.last_gate_result.expect("gate result present");
    assert_eq!(gate.gate, GateKind::RedGate);
    assert_eq!(gate.result, GateOutcome::Fail);
    assert_eq!(
        state.active_composition.mode,
        CompositionMode::SkillInteractive
    );
    assert!(
        state.published.is_none(),
        "no published marker before first publish"
    );
}

#[test]
fn pre_entry_nulls_carry_their_pinned_meanings() {
    // Null current_phase means "not yet entered"; null current_layer means
    // "decomposition not yet authored" (operator ruling, vsdd-cli #665).
    let state = read_state(&fixture("pre-entry.yaml"), &vocabulary()).expect("pre-entry reads");
    assert_eq!(state.current_phase, None);
    assert_eq!(state.current_layer, None);
}

#[test]
fn absent_file_yields_the_absent_diagnostic() {
    let missing = fixture("does-not-exist.yaml");
    let diag = read_state(&missing, &vocabulary()).expect_err("absent file is a diagnostic");
    assert_eq!(diag.kind, "absent");
    assert_eq!(diag.machine_token, "state-absent");
    assert_eq!(diag.recovery_action, "restore-state-file");
    assert!(
        diag.file.ends_with("does-not-exist.yaml"),
        "diagnostic names the file"
    );
}

#[test]
fn malformed_file_yields_location_and_content_recovery() {
    let diag = read_state(&fixture("malformed.yaml"), &vocabulary())
        .expect_err("malformed is a diagnostic");
    assert_eq!(diag.kind, "malformed");
    assert_eq!(diag.machine_token, "state-malformed");
    assert_eq!(diag.recovery_action, "fix-state-content");
    assert!(
        diag.location.is_some(),
        "a parse failure carries its location"
    );
}

#[test]
fn empty_file_is_malformed_not_a_panic() {
    let diag =
        read_state(&fixture("empty.yaml"), &vocabulary()).expect_err("empty file is a diagnostic");
    assert_eq!(diag.kind, "malformed");
}

#[cfg(unix)]
#[test]
fn permission_failure_yields_the_io_kind() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.yaml");
    fs::copy(fixture("valid.yaml"), &path).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o000)).unwrap();
    if fs::read(&path).is_ok() {
        eprintln!(
            "skipping: this environment ignores mode bits (euid 0), so it cannot express the case"
        );
        return;
    }

    let diag = read_state(&path, &vocabulary()).expect_err("unreadable file is a diagnostic");
    assert_eq!(diag.kind, "permission-or-io");
    assert_eq!(diag.machine_token, "state-unreadable-io");
    assert_eq!(diag.recovery_action, "fix-state-permissions");

    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
}

#[test]
fn future_schema_version_is_refused_naming_versions() {
    // Bootstrap self-validation: the version gate fires before any further read.
    let diag = read_state(&fixture("future-version.yaml"), &vocabulary())
        .expect_err("unsupported version is a diagnostic");
    assert!(diag.message.contains("9.9.9"), "names the version seen");
    assert!(
        diag.message.contains(SUPPORTED_STATE_SCHEMA_VERSION),
        "names the supported version"
    );
}

#[test]
fn unicode_milestone_survives_a_write_read_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.yaml");
    let vocab = vocabulary();
    let mut state = read_state(&fixture("valid.yaml"), &vocab).expect("valid state reads");
    state.open_findings_pointer.milestone = "layer 1: état — 状態 artifact ✓".to_string();

    write_state(&path, &state, &evidence(), &vocab, &actions()).expect("write succeeds");
    let reread = read_state(&path, &vocab).expect("reread succeeds");
    assert_eq!(
        reread.open_findings_pointer.milestone,
        "layer 1: état — 状態 artifact ✓"
    );
}

#[cfg(unix)]
#[test]
fn failed_write_leaves_no_partial_file() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let locked = dir.path().join("locked");
    fs::create_dir(&locked).unwrap();
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o555)).unwrap();
    if fs::write(locked.join(".probe"), b"x").is_ok() {
        eprintln!(
            "skipping: this environment ignores mode bits (euid 0), so it cannot express the case"
        );
        return;
    }
    let target = locked.join("state.yaml");

    let vocab = vocabulary();
    let state = read_state(&fixture("valid.yaml"), &vocab).expect("valid state reads");
    let result = write_state(&target, &state, &evidence(), &vocab, &actions());

    assert!(
        result.is_err(),
        "write into a read-only directory is refused"
    );
    assert!(!target.exists(), "no partial file survives the failure");
    assert!(
        fs::read_dir(&locked).unwrap().next().is_none(),
        "no temp file survives the failure"
    );
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o755)).unwrap();
}

#[test]
fn published_marker_is_forward_only() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.yaml");
    let vocab = vocabulary();
    let mut state = read_state(&fixture("valid.yaml"), &vocab).expect("valid state reads");
    state.published = Some(vsdd_core::state::Published {
        at: "2026-07-21".into(),
        version: "1.0.0".into(),
        act: "vsdd-cli #716".into(),
    });
    write_state(&path, &state, &evidence(), &vocab, &actions()).expect("first publish writes");

    let mut tampered = state.clone();
    tampered.published.as_mut().unwrap().version = "1.0.1".into();
    let diag = write_state(&path, &tampered, &evidence(), &vocab, &actions())
        .expect_err("a write touching a written published block is refused");
    assert!(
        diag.message.contains("published"),
        "the refusal names the field"
    );

    let survivor = read_state(&path, &vocab).expect("file still reads");
    assert_eq!(
        survivor.published.unwrap().version,
        "1.0.0",
        "the written marker survives unchanged"
    );
}

#[test]
fn write_requires_boundary_evidence() {
    // The state advances only in the same commit as its boundary evidence;
    // an empty evidence reference is refused.
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.yaml");
    let vocab = vocabulary();
    let state = read_state(&fixture("valid.yaml"), &vocab).expect("valid state reads");
    let diag = write_state(
        &path,
        &state,
        &BoundaryEvidence {
            commit: String::new(),
        },
        &vocab,
        &actions(),
    )
    .expect_err("empty boundary evidence is refused");
    assert!(
        diag.message.contains("evidence"),
        "the refusal names the missing evidence"
    );
}

#[test]
fn human_render_is_rustc_shaped_and_complete() {
    let diag = read_state(&fixture("does-not-exist.yaml"), &vocabulary())
        .expect_err("absent file is a diagnostic");
    let human = diag.render_human();
    assert!(
        human.starts_with("error"),
        "rustc-shaped: opens with the severity word"
    );
    assert!(human.contains("does-not-exist.yaml"), "names the file");
    assert!(
        human.contains(&diag.recovery_text),
        "carries the recovery text"
    );
    assert!(
        !human.contains('\u{1b}'),
        "information is text, not color codes"
    );
}

#[test]
fn machine_render_carries_the_loaded_tokens() {
    let diag = read_state(&fixture("does-not-exist.yaml"), &vocabulary())
        .expect_err("absent file is a diagnostic");
    let machine = diag.render_machine();
    assert_eq!(machine["kind"], "absent");
    assert_eq!(machine["machine_token"], "state-absent");
    assert_eq!(machine["recovery_action"], "restore-state-file");
    assert!(
        machine["message"].as_str().is_some(),
        "diagnostic payload present"
    );
}

#[test]
fn recovery_text_matches_the_data_set_verbatim() {
    // Token fidelity: the emitted recovery text IS the loaded set's
    // human_recovery for the kind — the code holds no copy of its own.
    let vocab = vocabulary();
    let diag = read_state(&fixture("does-not-exist.yaml"), &vocab)
        .expect_err("absent file is a diagnostic");
    let from_set = vocab
        .read_failure_kinds
        .iter()
        .find(|k| k.kind == "absent")
        .expect("the set enumerates the absent kind");
    assert_eq!(diag.recovery_text, from_set.human_recovery);
    assert_eq!(diag.machine_token, from_set.machine_token);
}

fn evidence() -> BoundaryEvidence {
    BoundaryEvidence {
        commit: "vsdd-cli #716 red-gate fixture evidence".to_string(),
    }
}

// Keep State's Clone in the public surface honest for the tests above.
#[test]
fn state_type_supports_comparison_and_clone() {
    let vocab = vocabulary();
    let state = read_state(&fixture("valid.yaml"), &vocab).expect("valid state reads");
    let cloned: State = state.clone();
    assert_eq!(state, cloned);
}

// ── Round-1 fix-pass additions (vsdd-cli #722, #724, #728) ────────────────────

#[test]
fn entering_2b_without_the_red_record_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.yaml");
    let vocab = vocabulary();
    let acts = actions();
    let mut state = read_state(&fixture("valid.yaml"), &vocab).expect("valid state reads");
    write_state(&path, &state, &evidence(), &vocab, &acts).expect("seed state writes");

    state.current_phase = Some("phase-2b".to_string());
    state.last_gate_result = None;
    let diag = write_state(&path, &state, &evidence(), &vocab, &acts)
        .expect_err("a 2b entry without the red record is refused");
    assert!(diag.message.contains("phase-gate consistency"));
    let survivor = read_state(&path, &vocab).expect("file still reads");
    assert_eq!(
        survivor.current_phase.as_deref(),
        Some("phase-2a"),
        "the prior file survives the refusal unchanged"
    );
}

#[test]
fn each_gate_conjunct_mismatch_is_refused() {
    // One conjunct wrong at a time — wrong layer, result pass, wrong gate
    // kind — so deleting or inverting any single conjunct turns a test red
    // (the mutant-killing shape vsdd-cli #722 names).
    let vocab = vocabulary();
    let acts = actions();
    let base = read_state(&fixture("valid.yaml"), &vocab).expect("valid state reads");

    let mut wrong_layer = base.clone();
    wrong_layer.current_phase = Some("phase-2b".to_string());
    wrong_layer.last_gate_result.as_mut().unwrap().layer = 7;

    let mut result_pass = base.clone();
    result_pass.current_phase = Some("phase-2b".to_string());
    result_pass.last_gate_result.as_mut().unwrap().result = GateOutcome::Pass;

    let mut wrong_gate = base.clone();
    wrong_gate.current_phase = Some("phase-2b".to_string());
    wrong_gate.last_gate_result.as_mut().unwrap().gate = GateKind::GreenGate;

    for (name, state) in [
        ("wrong layer", wrong_layer),
        ("result pass", result_pass),
        ("wrong gate kind", wrong_gate),
    ] {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("state.yaml");
        let result = write_state(&path, &state, &evidence(), &vocab, &acts);
        let diag = result.expect_err("a mismatched conjunct must refuse");
        assert!(
            diag.message.contains("phase-gate consistency"),
            "{name}: the refusal names the constraint"
        );
        assert!(!path.exists(), "{name}: nothing written on refusal");
    }
}

#[test]
fn entering_2b_with_the_layer_red_fail_record_writes() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.yaml");
    let vocab = vocabulary();
    let acts = actions();
    let mut state = read_state(&fixture("valid.yaml"), &vocab).expect("valid state reads");
    // valid.yaml carries the layer's red-gate fail record at layer 1.
    state.current_phase = Some("phase-2b".to_string());
    write_state(&path, &state, &evidence(), &vocab, &acts).expect("the compliant record writes");
    let reread = read_state(&path, &vocab).expect("reread succeeds");
    assert_eq!(reread.current_phase.as_deref(), Some("phase-2b"));
}

#[test]
fn unknown_field_yields_malformed_with_location() {
    // The likeliest real-world corruption of an adopter-edited file: a
    // typo'd key under deny_unknown_fields (vsdd-cli #728).
    let diag = read_state(&fixture("unknown-field.yaml"), &vocabulary())
        .expect_err("an unknown field is a diagnostic");
    assert_eq!(diag.kind, "malformed");
    assert_eq!(diag.machine_token, "state-malformed");
    assert!(
        diag.location.is_some(),
        "the decode failure carries its location"
    );
}

#[test]
fn wrong_typed_field_yields_malformed() {
    let diag = read_state(&fixture("wrong-type.yaml"), &vocabulary())
        .expect_err("a wrong-typed field is a diagnostic");
    assert_eq!(diag.kind, "malformed");
    assert_eq!(diag.recovery_action, "fix-state-content");
}

#[test]
fn directory_as_state_path_yields_the_io_kind_on_every_platform() {
    // The cross-platform companion to the unix-only mode-bit tests
    // (vsdd-cli #728): reading a directory fails with a non-NotFound IO
    // error everywhere.
    let dir = tempfile::tempdir().unwrap();
    let diag = read_state(dir.path(), &vocabulary()).expect_err("a directory is not a state file");
    assert_eq!(diag.kind, "permission-or-io");
    assert_eq!(diag.machine_token, "state-unreadable-io");
}

#[test]
fn write_refusal_token_resolves_in_the_action_vocabulary() {
    // The write-side fidelity discipline (vsdd-cli #724): the refusal's
    // recovery members are the registered correct-the-write member's,
    // loaded, never a constant's copy.
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.yaml");
    let vocab = vocabulary();
    let acts = actions();
    let state = read_state(&fixture("valid.yaml"), &vocab).expect("valid state reads");
    let diag = write_state(
        &path,
        &state,
        &BoundaryEvidence {
            commit: String::new(),
        },
        &vocab,
        &acts,
    )
    .expect_err("empty evidence is refused");
    let member = acts
        .action_vocabulary
        .iter()
        .find(|a| a.id == diag.recovery_action)
        .expect("the refusal's recovery action resolves to a registered member");
    assert_eq!(member.family, "recovery");
    assert_eq!(diag.recovery_text, member.human);
}

#[test]
fn oversize_state_file_is_refused_before_parsing() {
    // The capped reader per the #726 ruling, landed at #732: the flat
    // pre-parse materialization is bounded and oversize fails loud.
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state.yaml");
    let mut body = String::from("schema_version: \"0.1.0\"\n");
    while (body.len() as u64) <= vsdd_core::MAX_ARTIFACT_BYTES {
        body.push_str("# padding far past the documented limit\n");
    }
    fs::write(&path, body).unwrap();

    let diag = read_state(&path, &vocabulary()).expect_err("an oversize state file is refused");
    assert_eq!(diag.kind, "malformed");
    assert!(
        diag.message
            .contains(&vsdd_core::MAX_ARTIFACT_BYTES.to_string()),
        "the diagnostic names the documented limit"
    );
}
