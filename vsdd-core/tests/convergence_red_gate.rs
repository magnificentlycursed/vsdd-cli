//! Layer 2 red gate — the phase-answer derivation against the
//! convergence corpus (vsdd-cli #738).
//!
//! Phase 2a suite: fails executed against the pre-implementation stubs;
//! 2b turns it green. Eleven fixtures carry OPERATOR-ADOPTED oracles
//! (adoption 2026-07-22 on vsdd-cli #738); nine are DRAFT ORACLES
//! awaiting adoption, marked so in their rationale comments — the four
//! round-1 additions (vsdd-cli #749: three integrity-kind positives
//! and the round-parity negative control) and the five round-2
//! additions (vsdd-cli #758: the two stale-gate falsifiers for the
//! #738 uniform conjunct; vsdd-cli #759: the three clean negatives
//! pinning each integrity kind's discrimination). The runner compares every
//! field by exact match and integrity findings at the kind-set grain,
//! exactly the Convergence test's comparison.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use vsdd_core::answer::derive::derive_phase_answer;
use vsdd_core::registry::{self, sets::CompositionScopeAndActions, sets::StatuslineData};
use vsdd_core::snapshot::Snapshot;
use vsdd_core::state::read_state;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/convergence")
}

fn vocabulary() -> StatuslineData {
    registry::load_set(&repo_root(), "statusline-data").expect("statusline data set loads")
}

fn actions() -> CompositionScopeAndActions {
    registry::load_set(&repo_root(), "composition-scope-and-actions")
        .expect("composition set loads")
}

/// The reference-answer shape the corpus carries (draft oracles).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Expected {
    phase: Option<String>,
    layer: Option<u32>,
    next_action: String,
    degraded: Option<String>,
    integrity_findings: Vec<String>,
}

fn load_fixture(name: &str) -> (vsdd_core::state::State, Snapshot, Expected) {
    let dir = corpus_dir().join(name);
    let state = read_state(&dir.join("state.yaml"), &vocabulary())
        .unwrap_or_else(|e| panic!("{name}: state fixture reads: {e}"));
    let snapshot: Snapshot =
        serde_yaml_ng::from_str(&fs::read_to_string(dir.join("snapshot.yaml")).unwrap())
            .unwrap_or_else(|e| panic!("{name}: snapshot fixture parses: {e}"));
    let expected: Expected =
        serde_yaml_ng::from_str(&fs::read_to_string(dir.join("expected.yaml")).unwrap())
            .unwrap_or_else(|e| panic!("{name}: expected fixture parses: {e}"));
    (state, snapshot, expected)
}

#[test]
fn every_corpus_fixture_matches_its_reference_answer() {
    let acts = actions();
    let mut ran = 0;
    for entry in fs::read_dir(corpus_dir()).unwrap() {
        let dir = entry.unwrap().path();
        if !dir.is_dir() {
            continue;
        }
        let name = dir.file_name().unwrap().to_string_lossy().into_owned();
        let (state, snapshot, expected) = load_fixture(&name);
        let answer = derive_phase_answer(&state, &snapshot, &acts);

        assert_eq!(answer.phase, expected.phase, "{name}: phase, exact match");
        assert_eq!(answer.layer, expected.layer, "{name}: layer, exact match");
        assert_eq!(
            answer.next_action, expected.next_action,
            "{name}: next action, exact match against the vocabulary"
        );
        assert_eq!(answer.degraded, expected.degraded, "{name}: degraded kind");
        // The composition passes through untouched — the answer echoes
        // the state's own record, never a recomputation (vsdd-cli #749).
        assert_eq!(
            answer.active_composition, state.active_composition,
            "{name}: the active composition is the state's, verbatim"
        );
        let got: BTreeSet<_> = answer.integrity_findings.iter().collect();
        let want: BTreeSet<_> = expected.integrity_findings.iter().collect();
        assert_eq!(
            got, want,
            "{name}: integrity findings at the kind-set grain"
        );
        ran += 1;
    }
    // Exact, not at-least (vsdd-cli #749): a silently skipped fixture
    // directory would otherwise read as coverage.
    assert_eq!(ran, 20, "the corpus holds exactly its twenty fixtures");
}

#[test]
fn the_derivation_is_deterministic() {
    // Over the WHOLE corpus (vsdd-cli #749), not one convenient member:
    // determinism is a property of the derivation, not of a fixture.
    let acts = actions();
    for entry in fs::read_dir(corpus_dir()).unwrap() {
        let dir = entry.unwrap().path();
        if !dir.is_dir() {
            continue;
        }
        let name = dir.file_name().unwrap().to_string_lossy().into_owned();
        let (state, snapshot, _) = load_fixture(&name);
        let first = derive_phase_answer(&state, &snapshot, &acts);
        let second = derive_phase_answer(&state, &snapshot, &acts);
        assert_eq!(first, second, "{name}: identical inputs, identical answer");
    }
}

#[test]
fn degraded_kinds_are_never_swapped() {
    // The absent condition yields the absent kind and the unusable
    // condition the unusable kind — the derivation property the
    // verification architecture names.
    let acts = actions();
    let (state_a, snap_a, _) = load_fixture("degraded-tracker-absent");
    assert_eq!(
        derive_phase_answer(&state_a, &snap_a, &acts)
            .degraded
            .as_deref(),
        Some("tracker-absent")
    );
    let (state_u, snap_u, _) = load_fixture("degraded-tracker-unusable");
    assert_eq!(
        derive_phase_answer(&state_u, &snap_u, &acts)
            .degraded
            .as_deref(),
        Some("tracker-unusable")
    );
}

#[test]
fn disagreement_files_a_finding_without_degrading() {
    // A present-but-disagreeing snapshot is an integrity finding, never
    // degradation (the phase-state resolution).
    let acts = actions();
    let (state, snapshot, _) = load_fixture("disagreement-files-finding");
    let answer = derive_phase_answer(&state, &snapshot, &acts);
    assert_eq!(answer.degraded, None, "disagreement does not degrade");
    assert!(
        answer
            .integrity_findings
            .iter()
            .any(|k| k == "phase-pointer-against-milestone-state"),
        "the disagreement is a filed finding kind"
    );
}

#[test]
fn fix_lane_tokens_never_leave_the_derivation() {
    // The output-domain exclusion (operator ruling, vsdd-cli #689),
    // checked across the whole corpus.
    let acts = actions();
    for entry in fs::read_dir(corpus_dir()).unwrap() {
        let dir = entry.unwrap().path();
        if !dir.is_dir() {
            continue;
        }
        let name = dir.file_name().unwrap().to_string_lossy().into_owned();
        let (state, snapshot, _) = load_fixture(&name);
        let answer = derive_phase_answer(&state, &snapshot, &acts);
        assert!(
            answer.next_action != "file-fix-finding" && answer.next_action != "run-fix-gate",
            "{name}: fix-lane tokens are workflow tokens, not derivation output"
        );
    }
}

#[test]
fn every_next_action_is_a_registered_vocabulary_member() {
    // The derivation outputs members of the closed token set, nothing
    // coined at runtime.
    let acts = actions();
    for entry in fs::read_dir(corpus_dir()).unwrap() {
        let dir = entry.unwrap().path();
        if !dir.is_dir() {
            continue;
        }
        let name = dir.file_name().unwrap().to_string_lossy().into_owned();
        let (state, snapshot, _) = load_fixture(&name);
        let answer = derive_phase_answer(&state, &snapshot, &acts);
        assert!(
            acts.action_vocabulary
                .iter()
                .any(|a| a.id == answer.next_action),
            "{name}: `{}` resolves in the action vocabulary",
            answer.next_action
        );
    }
}
