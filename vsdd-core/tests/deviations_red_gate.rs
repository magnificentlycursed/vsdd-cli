//! Phase-1 red gate — the deviations gate leg (vsdd-cli #820, phase 2a).
//!
//! Seeds the AC-5 fixture family from the ratified remediation design
//! (`.design/decomposition-topology-remediation.md` REQ-4/REQ-6) against
//! the `answer::deviations` stub, whose placeholder verdict (always Pass
//! plus a placeholder warning) guarantees every non-ignored test FAILS
//! EXECUTED — the non-vacuous red. Phase 2b turns this suite green.
//!
//! Every fixture is YAML written to a temp dir; every date the gate
//! compares is in-entry, against a caller-supplied `today` (no clock in
//! vsdd-core). The issue-state oracle is injected per test.

use std::path::PathBuf;

use vsdd_core::answer::deviations::{deviations_gate, DeviationsOutcome, GateMode, IssueState};
use vsdd_core::answer::integrity::GateVerdict;

/// Write a fixture registry into a temp dir at the deployed home's shape.
fn write_registry(dir: &tempfile::TempDir, yaml: &str) -> PathBuf {
    let path = dir.path().join(".vsdd/registry/deviation-registry.yaml");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, yaml).unwrap();
    path
}

/// The no-oracle-available direction: every issue-state query is
/// undecidable.
fn no_oracle(_issue_ref: &str) -> Option<IssueState> {
    None
}

/// Assert the outcome blocks and the block payload names the entry.
fn assert_blocks_entry(outcome: &DeviationsOutcome, entry_id: &str, context: &str) {
    match &outcome.verdict {
        GateVerdict::Block(ids) => assert!(
            ids.iter().any(|i| i.contains(entry_id)),
            "{context}: Block payload names the offending entry {entry_id}, got {ids:?}"
        ),
        other => panic!("{context}: expected Block({entry_id}), got {other:?}"),
    }
}

// AC-5 / build-plan bullet 2: "lapsed-expiry fails" — in both modes; the
// expiry comparison is in-entry against the caller-supplied today.
#[test]
fn lapsed_expiry_standing_entry_blocks_in_both_modes() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_registry(
        &dir,
        r"schema_version: 1
entries:
- id: lapsed-expiry-entry
  deviation: A fixture deviation whose expiry has lapsed unre-armed.
  deviates_from: null
  deviation_class: test-fixture
  stated_reason: Red-gate fixture (AC-5 lapsed-expiry direction).
  retest_trigger:
    type: date
    predicate: '2099-01-01'
  entry_date: 2026-06-15
  expiry: 2026-07-01
  owning_issue: vsdd-cli#820
  status: standing
  disposition_ref:
    issue: vsdd-cli#845
    comment_timestamp: 2026-06-15 12:00
",
    );
    for mode in [GateMode::Local, GateMode::Ci] {
        let outcome = deviations_gate(&path, "2026-08-01", mode, &no_oracle);
        assert_blocks_entry(
            &outcome,
            "lapsed-expiry-entry",
            &format!("lapsed expiry in {mode:?} mode"),
        );
    }
}

// AC-5: "a current entry passes" — future expiry within the 30-day
// default, unfired trigger, disposition present: nothing warn-worthy, so
// the clean pass is warning-free.
#[test]
fn current_standing_entry_passes_clean_in_both_modes() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_registry(
        &dir,
        r"schema_version: 1
entries:
- id: current-standing-entry
  deviation: A fixture deviation currently in force.
  deviates_from: null
  deviation_class: test-fixture
  stated_reason: Red-gate fixture (AC-5 current-entry direction).
  retest_trigger:
    type: date
    predicate: '2099-01-01'
  entry_date: 2026-07-20
  expiry: 2026-08-15
  owning_issue: vsdd-cli#820
  status: standing
  disposition_ref:
    issue: vsdd-cli#845
    comment_timestamp: 2026-07-20 12:00
",
    );
    for mode in [GateMode::Local, GateMode::Ci] {
        let outcome = deviations_gate(&path, "2026-08-01", mode, &no_oracle);
        assert_eq!(
            outcome.verdict,
            GateVerdict::Pass,
            "a current standing entry passes in {mode:?} mode"
        );
        assert!(
            outcome.warnings.is_empty(),
            "a clean current entry passes warning-free in {mode:?} mode, got {:?}",
            outcome.warnings
        );
    }
}

// AC-5: "a fixture whose date-based retest trigger has fired fails until
// its disposition_ref cites a newer Solution Owner disposition" — the
// re-arm respected; the gate compares only in-entry dates.
#[test]
fn fired_date_trigger_blocks_without_newer_disposition_and_rearm_passes() {
    let dir = tempfile::tempdir().unwrap();
    // Direction 1: trigger fired 2026-07-15, disposition older (07-12).
    let stale = write_registry(
        &dir,
        r"schema_version: 1
entries:
- id: fired-date-trigger-entry
  deviation: A fixture deviation whose date trigger has fired unre-armed.
  deviates_from: null
  deviation_class: test-fixture
  stated_reason: Red-gate fixture (AC-5 fired-date-trigger direction).
  retest_trigger:
    type: date
    predicate: '2026-07-15'
  entry_date: 2026-07-10
  expiry: 2026-08-05
  owning_issue: vsdd-cli#820
  status: standing
  disposition_ref:
    issue: vsdd-cli#845
    comment_timestamp: 2026-07-12 09:00
",
    );
    for mode in [GateMode::Local, GateMode::Ci] {
        let outcome = deviations_gate(&stale, "2026-08-01", mode, &no_oracle);
        assert_blocks_entry(
            &outcome,
            "fired-date-trigger-entry",
            &format!("fired date trigger without newer disposition in {mode:?} mode"),
        );
    }

    // Direction 2: the same entry re-armed — disposition (07-20) newer
    // than the trigger date (07-15) → the re-arm is respected.
    let rearmed_dir = tempfile::tempdir().unwrap();
    let rearmed = write_registry(
        &rearmed_dir,
        r"schema_version: 1
entries:
- id: fired-date-trigger-entry
  deviation: A fixture deviation whose fired date trigger was re-armed.
  deviates_from: null
  deviation_class: test-fixture
  stated_reason: Red-gate fixture (AC-5 re-arm direction).
  retest_trigger:
    type: date
    predicate: '2026-07-15'
  entry_date: 2026-07-10
  expiry: 2026-08-05
  owning_issue: vsdd-cli#820
  status: standing
  disposition_ref:
    issue: vsdd-cli#845
    comment_timestamp: 2026-07-20 09:00
",
    );
    let outcome = deviations_gate(&rearmed, "2026-08-01", GateMode::Local, &no_oracle);
    assert_eq!(
        outcome.verdict,
        GateVerdict::Pass,
        "a fired date trigger with a NEWER in-entry disposition timestamp passes (re-arm respected)"
    );
}

// AC-5 / resolved Q3: "an undecidable-trigger fixture passes-with-warning
// on the local gate" — issue-state class with no oracle available; the
// developer surface stays usable offline but the pass is recorded.
#[test]
fn undecidable_issue_trigger_warn_passes_on_the_local_gate() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_registry(
        &dir,
        r"schema_version: 1
entries:
- id: undecidable-issue-trigger-entry
  deviation: A fixture deviation with an issue-state trigger and no oracle.
  deviates_from: null
  deviation_class: test-fixture
  stated_reason: Red-gate fixture (AC-5 undecidable-trigger direction).
  retest_trigger:
    type: issue-state
    predicate: example-org/upstream#9 state == closed
  entry_date: 2026-07-20
  expiry: 2026-08-15
  owning_issue: vsdd-cli#820
  status: standing
  disposition_ref:
    issue: vsdd-cli#845
    comment_timestamp: 2026-07-20 12:00
",
    );
    let outcome = deviations_gate(&path, "2026-08-01", GateMode::Local, &no_oracle);
    assert_eq!(
        outcome.verdict,
        GateVerdict::Pass,
        "an undecidable issue-state trigger passes on the local gate"
    );
    assert!(
        outcome
            .warnings
            .iter()
            .any(|w| w.contains("undecidable-issue-trigger-entry")),
        "the local pass records a warning naming the undecidable entry, got {:?}",
        outcome.warnings
    );
}

// AC-5 / resolved Q3: the same undecidable fixture "blocks as inconclusive
// on the CI leg" — inconclusive maps to exit 2, the Unverifiable
// convention; the enforcement surface never passes silently.
#[test]
fn undecidable_issue_trigger_is_inconclusive_on_the_ci_leg() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_registry(
        &dir,
        r"schema_version: 1
entries:
- id: undecidable-issue-trigger-entry
  deviation: A fixture deviation with an issue-state trigger and no oracle.
  deviates_from: null
  deviation_class: test-fixture
  stated_reason: Red-gate fixture (AC-5 undecidable-trigger CI direction).
  retest_trigger:
    type: issue-state
    predicate: example-org/upstream#9 state == closed
  entry_date: 2026-07-20
  expiry: 2026-08-15
  owning_issue: vsdd-cli#820
  status: standing
  disposition_ref:
    issue: vsdd-cli#845
    comment_timestamp: 2026-07-20 12:00
",
    );
    let outcome = deviations_gate(&path, "2026-08-01", GateMode::Ci, &no_oracle);
    assert!(
        matches!(outcome.verdict, GateVerdict::Unverifiable(_)),
        "an undecidable trigger is inconclusive (Unverifiable, exit-2 class) on the CI leg, got {:?}",
        outcome.verdict
    );
}

// AC-5: "a premature-resolution fixture — a resolved entry without its own
// Solution Owner disposition_ref — is treated as still standing":
// enumerated and expiry-checked, so its lapsed expiry blocks.
#[test]
fn premature_resolution_is_treated_as_still_standing() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_registry(
        &dir,
        r"schema_version: 1
entries:
- id: premature-resolution-entry
  deviation: A fixture deviation marked resolved without a disposition.
  deviates_from: null
  deviation_class: test-fixture
  stated_reason: Red-gate fixture (AC-5 premature-resolution direction).
  entry_date: 2026-06-01
  expiry: 2026-07-01
  owning_issue: vsdd-cli#820
  status: resolved
",
    );
    for mode in [GateMode::Local, GateMode::Ci] {
        let outcome = deviations_gate(&path, "2026-08-01", mode, &no_oracle);
        assert_blocks_entry(
            &outcome,
            "premature-resolution-entry",
            &format!("premature resolution treated as still standing in {mode:?} mode"),
        );
    }
}

// AC-5: "absent-registry ... fixtures exit 2/Unverifiable in both modes"
// — the fail-closed keystone's first negative: deleting the registry must
// never pass the gate.
#[test]
fn absent_registry_is_unverifiable_in_both_modes() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(".vsdd/registry/deviation-registry.yaml");
    for mode in [GateMode::Local, GateMode::Ci] {
        let outcome = deviations_gate(&path, "2026-08-01", mode, &no_oracle);
        assert!(
            matches!(outcome.verdict, GateVerdict::Unverifiable(_)),
            "an absent registry is Unverifiable in {mode:?} mode — deleting the registry never passes, got {:?}",
            outcome.verdict
        );
    }
}

// AC-5: "shape-invalid-registry fixtures exit 2/Unverifiable in both
// modes" — a registry missing schema_version, and an entry missing a
// required field, are both fail-closed.
#[test]
fn shape_invalid_registry_is_unverifiable_in_both_modes() {
    // Fixture 1: the registry file misses schema_version.
    let dir = tempfile::tempdir().unwrap();
    let missing_version = write_registry(
        &dir,
        r"entries:
- id: shape-invalid-registry-entry
  deviation: A fixture entry in a registry missing schema_version.
  deviates_from: null
  deviation_class: test-fixture
  stated_reason: Red-gate fixture (AC-5 shape-invalid direction).
  entry_date: 2026-07-20
  expiry: 2026-08-15
  owning_issue: vsdd-cli#820
  status: standing
",
    );
    for mode in [GateMode::Local, GateMode::Ci] {
        let outcome = deviations_gate(&missing_version, "2026-08-01", mode, &no_oracle);
        assert!(
            matches!(outcome.verdict, GateVerdict::Unverifiable(_)),
            "a registry missing schema_version is Unverifiable in {mode:?} mode, got {:?}",
            outcome.verdict
        );
    }

    // Fixture 2: an entry misses required fields (no status, no entry_date).
    let dir2 = tempfile::tempdir().unwrap();
    let missing_fields = write_registry(
        &dir2,
        r"schema_version: 1
entries:
- id: shape-invalid-entry
  deviation: A fixture entry missing its status and entry_date.
  deviates_from: null
  deviation_class: test-fixture
  stated_reason: Red-gate fixture (AC-5 shape-invalid entry direction).
  owning_issue: vsdd-cli#820
",
    );
    for mode in [GateMode::Local, GateMode::Ci] {
        let outcome = deviations_gate(&missing_fields, "2026-08-01", mode, &no_oracle);
        assert!(
            matches!(outcome.verdict, GateVerdict::Unverifiable(_)),
            "an entry missing required fields is Unverifiable in {mode:?} mode, got {:?}",
            outcome.verdict
        );
    }
}

// AC-5 / resolved Q2: "an abusive-override fixture — expiry 2099 with a
// stated reason but no Solution Owner disposition — is treated as carrying
// the 30-day default and warned". Both directions of the applied default:
// within entry_date+30d the entry warn-passes; past entry_date+30d the
// carried default has lapsed and the entry blocks.
#[test]
fn abusive_override_carries_the_thirty_day_default_and_warns() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_registry(
        &dir,
        r"schema_version: 1
entries:
- id: abusive-override-entry
  deviation: A fixture deviation with a 2099 expiry and no covering disposition.
  deviates_from: null
  deviation_class: test-fixture
  stated_reason: Red-gate fixture (AC-5 abusive-override direction).
  retest_trigger:
    type: date
    predicate: '2099-01-01'
  entry_date: 2026-07-25
  expiry: 2099-12-31
  owning_issue: vsdd-cli#820
  status: standing
",
    );
    // Within the carried default (entry_date + 30d = 2026-08-24): the
    // entry stands, and the inoperative override is warned.
    let outcome = deviations_gate(&path, "2026-08-01", GateMode::Local, &no_oracle);
    assert_eq!(
        outcome.verdict,
        GateVerdict::Pass,
        "an abusive override within entry_date+30d still passes (the default applies)"
    );
    assert!(
        outcome
            .warnings
            .iter()
            .any(|w| w.contains("abusive-override-entry")),
        "the inoperative override is warned by entry name, got {:?}",
        outcome.warnings
    );

    // Past the carried default: the entry is lapsed — the 2099 expiry
    // never took effect.
    let lapsed = deviations_gate(&path, "2026-09-10", GateMode::Local, &no_oracle);
    assert_blocks_entry(
        &lapsed,
        "abusive-override-entry",
        "an abusive override past entry_date+30d is lapsed under the carried default",
    );
}

// AC-5: "a self-re-arm fixture — a disposition_ref citing an agent-authored
// comment — is refused (CI-leg-only)". The fixture's would-be re-arm
// (disposition newer than the fired trigger) must NOT count on the CI leg
// once authorship verifies over server-synced state.
#[test]
#[ignore = "CI-leg: rides #815 + signing/identity; decidable substrate = crosslink comment author + driver_key_fingerprint columns"]
fn self_re_arm_disposition_is_refused_on_the_ci_leg() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_registry(
        &dir,
        r"schema_version: 1
entries:
- id: self-re-arm-entry
  deviation: A fixture deviation re-armed by an agent-authored comment.
  deviates_from: null
  deviation_class: test-fixture
  stated_reason: Red-gate fixture (AC-5 self-re-arm refusal direction).
  retest_trigger:
    type: date
    predicate: '2026-07-15'
  trigger_context: 'Fixture intent: the cited comment is agent-authored; the
    CI leg resolves authorship over server-synced crosslink state (author +
    driver_key_fingerprint columns) once #815 lands.'
  entry_date: 2026-07-10
  expiry: 2026-08-05
  owning_issue: vsdd-cli#820
  status: standing
  disposition_ref:
    issue: vsdd-cli#845
    comment_timestamp: 2026-07-20 09:00
",
    );
    // The re-arm timestamp postdates the fired trigger, but its author is
    // not the Solution Owner: the CI leg refuses it and the fired trigger
    // still blocks.
    let outcome = deviations_gate(&path, "2026-08-01", GateMode::Ci, &no_oracle);
    assert_blocks_entry(
        &outcome,
        "self-re-arm-entry",
        "a self-re-arm (agent-authored disposition) is refused on the CI leg",
    );
}

// Build-plan bullet 2's named 2a obligation: "The issue-trigger
// fired-direction fixture needs an injectable issue-state oracle" — a fake
// oracle returning closed makes the issue-state trigger decidable-fired,
// and with no re-arm since entry the entry blocks.
#[test]
fn fired_issue_trigger_blocks_through_the_injected_oracle() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_registry(
        &dir,
        r"schema_version: 1
entries:
- id: fired-issue-trigger-entry
  deviation: A fixture deviation whose upstream issue has closed.
  deviates_from: null
  deviation_class: test-fixture
  stated_reason: Red-gate fixture (the injectable-oracle 2a obligation).
  retest_trigger:
    type: issue-state
    predicate: example-org/upstream#42 state == closed
  entry_date: 2026-07-10
  expiry: 2026-08-05
  owning_issue: vsdd-cli#820
  status: standing
  disposition_ref:
    issue: vsdd-cli#845
    comment_timestamp: 2026-07-10 09:00
",
    );
    let closed_oracle = |issue_ref: &str| -> Option<IssueState> {
        issue_ref.contains("#42").then_some(IssueState::Closed)
    };
    let outcome = deviations_gate(&path, "2026-08-01", GateMode::Local, &closed_oracle);
    assert_blocks_entry(
        &outcome,
        "fired-issue-trigger-entry",
        "a fired issue-state trigger (oracle: closed) with no re-arm since entry",
    );
}
