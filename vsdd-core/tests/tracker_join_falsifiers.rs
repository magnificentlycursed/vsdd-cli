//! The tracker-join falsifier set (build-plan Phase 1, bullet 1; vsdd-cli
//! #820) — the integration coverage the built Slice-1 detection owes.
//!
//! These tests target SHIPPED behavior (the mappers and acquisition landed
//! via PR #9/#10, boundary d146019f), so they are expected green; a red test
//! here is a found defect for Phase-4 routing, never a fix-in-place. One
//! test is a deliberate expected-defect falsifier — see
//! `a_failed_finding_leg_with_tracker_present_must_not_pass_the_gate_vacuously`,
//! which asserts guardrail REQ-4 (knowledge page
//! `routing-before-fix-guardrail`) against the shipped gate.
//!
//! Controllable tracker state: a fake `crosslink` executable — a shell
//! script serving canned responses from the invoking repo's `fake-tracker/`
//! directory — shadows PATH for this test binary only. `acquire_snapshot`
//! runs its subprocess legs against the fake, so every test drives the FULL
//! effectful walk (review-list, all-issues list, per-finding show) over a
//! tracker state the test authored. Failure markers (`fail-review-list`,
//! `fail-all-list`, `fail-show`) make individual legs exit nonzero, driving
//! the degradation paths.
//!
//! Seam honesty: the round-manifest/children parity and comment-handle
//! falsifiers cannot ride the live acquisition — the shipped join acquires
//! those fields EMPTY until their Slice-6 consumers land (the declared
//! bootstrap scope in `snapshot/acquire.rs`'s module doc). They are covered
//! at the pure `snapshot_integrity` seam over fixture-shaped snapshots, the
//! same shape the convergence corpus feeds.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use vsdd_core::answer::integrity::{
    gate_verdict, snapshot_integrity, unrouted_findings, GateVerdict,
};
use vsdd_core::snapshot::acquire::acquire_snapshot;
use vsdd_core::snapshot::{AcquisitionOutcome, FindingFieldsAcquired, Snapshot};
use vsdd_core::state::schema::{ActiveComposition, CompositionMode, OpenFindingsPointer, State};

// ── The controllable tracker: a fake crosslink on PATH ──────────────────────

/// The fake crosslink: serves canned responses from `$PWD/fake-tracker/`
/// (the cwd `run_bounded` passes is the repo root under acquisition), and
/// exits nonzero when the test dropped a failure marker for the leg.
const FAKE_CROSSLINK: &str = r#"#!/bin/sh
d="$PWD/fake-tracker"
case "$1/$2" in
  milestone/list) cat "$d/milestone-list.txt" ;;
  session/status) cat "$d/session.json" ;;
  issue/list)
    case "$*" in
      *"--label review"*)
        [ -e "$d/fail-review-list" ] && { echo "fake leg failure" >&2; exit 1; }
        cat "$d/review-list.json" ;;
      *)
        [ -e "$d/fail-all-list" ] && { echo "fake leg failure" >&2; exit 1; }
        cat "$d/all-list.json" ;;
    esac ;;
  issue/show)
    [ -e "$d/fail-show" ] && { echo "fake leg failure" >&2; exit 1; }
    if [ -f "$d/issue-$3.json" ]; then cat "$d/issue-$3.json"; else cat "$d/issue-default.json"; fi ;;
  *) echo "fake crosslink: unexpected args: $*" >&2; exit 1 ;;
esac
"#;

static FAKE_BIN_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Install the fake crosslink and prepend its directory to PATH, once per
/// test process. Every test constructs its tracker through [`TempRepo`],
/// which calls this first, so no subprocess spawns before PATH settles.
fn ensure_fake_crosslink_on_path() {
    FAKE_BIN_DIR.get_or_init(|| {
        let dir = std::env::temp_dir().join(format!("vsdd-fake-crosslink-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("fake bin dir creates");
        let script = dir.join("crosslink");
        std::fs::write(&script, FAKE_CROSSLINK).expect("fake crosslink writes");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755))
                .expect("fake crosslink marks executable");
        }
        let path = std::env::var("PATH").unwrap_or_default();
        std::env::set_var("PATH", format!("{}:{path}", dir.display()));
        dir
    });
}

/// A temp repo the acquisition walks: a `.crosslink/` marker plus the
/// canned `fake-tracker/` responses the fake crosslink serves. Defaults to
/// a lawful empty tracker (no milestones, a live session, no issues).
struct TempRepo {
    dir: tempfile::TempDir,
}

impl TempRepo {
    fn new() -> Self {
        ensure_fake_crosslink_on_path();
        let dir = tempfile::Builder::new()
            .prefix("vsdd-tracker-join-")
            .tempdir()
            .expect("temp repo creates");
        std::fs::create_dir(dir.path().join(".crosslink")).unwrap();
        std::fs::create_dir(dir.path().join("fake-tracker")).unwrap();
        let repo = TempRepo { dir };
        repo.serve("milestone-list.txt", "No milestones found.\n");
        repo.serve(
            "session.json",
            r##"{"session_id": 4, "working_on": {"display_id": "#820", "title": "slice one"}}"##,
        );
        repo.serve("review-list.json", "[]");
        repo.serve("all-list.json", "[]");
        repo.serve("issue-default.json", r#"{"labels": [], "comments": []}"#);
        repo
    }

    fn serve(&self, name: &str, content: &str) {
        std::fs::write(self.dir.path().join("fake-tracker").join(name), content).unwrap();
    }

    fn fail_leg(&self, marker: &str) {
        self.serve(marker, "");
    }

    fn root(&self) -> &Path {
        self.dir.path()
    }
}

/// A `crosslink issue list --json` item in the shape the walk reads.
fn issue(id: u64, parent_id: Option<u64>, status: &str, closed_at: Option<&str>) -> serde_json::Value {
    let mut v = serde_json::json!({ "id": id, "status": status });
    if let Some(p) = parent_id {
        v["parent_id"] = p.into();
    }
    if let Some(c) = closed_at {
        v["closed_at"] = c.into();
    }
    v
}

fn issue_list(items: &[serde_json::Value]) -> String {
    serde_json::Value::Array(items.to_vec()).to_string()
}

/// A `crosslink issue show --json` detail: labels (disposition) and comment
/// kinds (routing).
fn issue_detail(labels: &[&str], comment_kinds: &[&str]) -> String {
    serde_json::json!({
        "labels": labels,
        "comments": comment_kinds
            .iter()
            .map(|k| serde_json::json!({ "kind": k }))
            .collect::<Vec<_>>(),
    })
    .to_string()
}

/// A review round (#100) with the given children in the all-issues list.
fn serve_review_round_with_children(repo: &TempRepo, children: &[serde_json::Value]) {
    let round = issue(100, None, "open", None);
    repo.serve("review-list.json", &issue_list(&[round.clone()]));
    let mut all = vec![round];
    all.extend_from_slice(children);
    repo.serve("all-list.json", &issue_list(&all));
}

fn minimal_state() -> State {
    State {
        schema_version: "0.1.0".to_string(),
        current_phase: Some("phase-4".to_string()),
        current_layer: Some(2),
        open_findings_pointer: OpenFindingsPointer {
            milestone: "layer 2".to_string(),
        },
        last_gate_result: None,
        active_composition: ActiveComposition {
            scope: "test".to_string(),
            domains: Vec::new(),
            mode: CompositionMode::SkillInteractive,
            config_inputs_hash: "sha256:test".to_string(),
        },
        published: None,
    }
}

fn handles(snapshot: &Snapshot) -> Vec<&str> {
    snapshot.findings.iter().map(|f| f.handle.as_str()).collect()
}

/// A timestamp safely after the 2026-07-27 ratification boundary, in
/// crosslink's fractional-second shape.
const CLOSED_AFTER_BOUNDARY: &str = "2026-07-29T10:00:00.123456Z";

// ── Finding-versus-issue discrimination ─────────────────────────────────────

#[test]
fn only_children_of_review_labelled_parents_are_findings() {
    // The bootstrap discrimination rule: only a child of a `review`-labelled
    // parent is a finding. The parentless issue and the child of a
    // non-review parent are the mis-map negatives — never findings; the
    // review round itself is not a finding either.
    let repo = TempRepo::new();
    serve_review_round_with_children(
        &repo,
        &[
            issue(7, Some(100), "open", None),  // review child -> the one finding
            issue(8, None, "open", None),       // parentless -> never a finding
            issue(9, Some(999), "open", None),  // child of a non-review parent -> never
        ],
    );
    repo.serve("issue-7.json", &issue_detail(&[], &[]));

    let snapshot = acquire_snapshot(repo.root());
    assert_eq!(snapshot.acquisition_outcome, AcquisitionOutcome::Acquired);
    assert_eq!(
        handles(&snapshot),
        vec!["#7"],
        "exactly the review-round child is a finding; the parentless issue and \
         the non-review child are not"
    );
    assert!(
        snapshot.finding_acquisition_note.is_none(),
        "an under-cap walk carries no truncation marker (the negative of the \
         no-silent-drop falsifier)"
    );
}

// ── Plan-comment routing presence ───────────────────────────────────────────

#[test]
fn a_plan_routed_fix_close_reads_routed_and_passes_the_gate() {
    // The positive: a fix-closed finding with a prior `plan` routing comment
    // is routed — the unrouted-findings query stays empty and the gate passes.
    let repo = TempRepo::new();
    serve_review_round_with_children(
        &repo,
        &[issue(7, Some(100), "closed", Some(CLOSED_AFTER_BOUNDARY))],
    );
    repo.serve("issue-7.json", &issue_detail(&[], &["plan", "result"]));

    let snapshot = acquire_snapshot(repo.root());
    let finding = &snapshot.findings[0];
    assert!(finding.routing_present, "a plan comment is the routing edge");
    assert!(
        unrouted_findings(&snapshot).is_empty(),
        "a routed fix-close is not unrouted"
    );
    assert_eq!(gate_verdict(&snapshot), GateVerdict::Pass);
    assert!(
        !snapshot_integrity(&minimal_state(), &snapshot)
            .iter()
            .any(|k| k == "unrouted-findings"),
        "no unrouted-findings kind for a routed close"
    );
}

#[test]
fn a_result_only_fix_close_is_unrouted_and_blocks() {
    // The mis-map negative: a `result` comment must NOT count as routing. A
    // fix-close with only a result comment is unrouted — the query names it,
    // the report emits the kind, and the gate blocks on the same predicate.
    let repo = TempRepo::new();
    serve_review_round_with_children(
        &repo,
        &[issue(8, Some(100), "closed", Some(CLOSED_AFTER_BOUNDARY))],
    );
    repo.serve("issue-8.json", &issue_detail(&[], &["result"]));

    let snapshot = acquire_snapshot(repo.root());
    let finding = &snapshot.findings[0];
    assert!(
        !finding.routing_present,
        "a result comment is not a routing edge"
    );
    assert_eq!(unrouted_findings(&snapshot), vec!["#8".to_string()]);
    assert_eq!(
        gate_verdict(&snapshot),
        GateVerdict::Block(vec!["#8".to_string()]),
        "the gate blocks naming the unrouted handle"
    );
    assert!(
        snapshot_integrity(&minimal_state(), &snapshot)
            .iter()
            .any(|k| k == "unrouted-findings"),
        "the report emits the kind from the same predicate the gate blocks on"
    );
}

// ── Round-manifest and children counts (pure seam — see module doc) ─────────

/// A fixture-shaped snapshot for the pure `snapshot_integrity` seam: the
/// live acquisition holds round manifests, children, and comment handles
/// EMPTY until Slice 6 (the declared bootstrap scope), so these falsifiers
/// ride the same snapshot shape the convergence corpus feeds.
fn snapshot_from_yaml(yaml: &str) -> Snapshot {
    serde_yaml_ng::from_str(yaml).expect("fixture-shaped snapshot parses")
}

const PURE_SNAPSHOT_SPINE: &str = r#"
acquisition_outcome: acquired
milestones: []
findings: []
display_repo_name: test
display_session: test
display_work_item: test
display_active_milestone: test
"#;

#[test]
fn round_manifest_parity_agrees_and_a_mismatch_surfaces() {
    // Parity positive: manifest count == tracked children -> no finding.
    let agreeing = snapshot_from_yaml(&format!(
        "{PURE_SNAPSHOT_SPINE}
round_manifests:
  - {{handle: \"round 1\", declared_finding_count: 3}}
round_children:
  - {{handle: \"round 1\", child_count: 3}}
comment_handles: []
"
    ));
    assert!(
        !snapshot_integrity(&minimal_state(), &agreeing)
            .iter()
            .any(|k| k == "round-parity"),
        "agreeing counts fire no round-parity finding (the mis-map negative)"
    );

    // The parity violation surfaces: declared != tracked.
    let violating = snapshot_from_yaml(&format!(
        "{PURE_SNAPSHOT_SPINE}
round_manifests:
  - {{handle: \"round 1\", declared_finding_count: 5}}
round_children:
  - {{handle: \"round 1\", child_count: 3}}
comment_handles: []
"
    ));
    assert!(
        snapshot_integrity(&minimal_state(), &violating)
            .iter()
            .any(|k| k == "round-parity"),
        "a declared/tracked mismatch surfaces the parity violation"
    );

    // A manifest with no tracked-children record reconciles against zero —
    // a nonzero declaration surfaces.
    let untracked = snapshot_from_yaml(&format!(
        "{PURE_SNAPSHOT_SPINE}
round_manifests:
  - {{handle: \"round 2\", declared_finding_count: 2}}
round_children: []
comment_handles: []
"
    ));
    assert!(
        snapshot_integrity(&minimal_state(), &untracked)
            .iter()
            .any(|k| k == "round-parity"),
        "a declared count with no tracked children surfaces the violation"
    );
}

// ── Comment-handle resolution (pure seam — see module doc) ──────────────────

#[test]
fn resolving_handles_are_clean_and_a_dangling_handle_surfaces() {
    // Positive: a round result comment citing child handles that all resolve.
    let resolving = snapshot_from_yaml(&format!(
        "{PURE_SNAPSHOT_SPINE}
round_manifests: []
round_children: []
comment_handles:
  - {{handle: \"#7\", resolves: true}}
  - {{handle: \"#8\", resolves: true}}
"
    ));
    assert!(
        !snapshot_integrity(&minimal_state(), &resolving)
            .iter()
            .any(|k| k == "unresolvable-handles-in-result-comments"),
        "resolving handles fire nothing (the mis-map negative)"
    );

    // A dangling handle surfaces.
    let dangling = snapshot_from_yaml(&format!(
        "{PURE_SNAPSHOT_SPINE}
round_manifests: []
round_children: []
comment_handles:
  - {{handle: \"#7\", resolves: true}}
  - {{handle: \"#404\", resolves: false}}
"
    ));
    assert!(
        snapshot_integrity(&minimal_state(), &dangling)
            .iter()
            .any(|k| k == "unresolvable-handles-in-result-comments"),
        "a dangling cited handle surfaces"
    );
}

// ── The forward-only-universe boundary ──────────────────────────────────────

#[test]
fn a_close_before_the_ratification_boundary_is_outside_the_universe() {
    // REQ-5: a finding closed BEFORE 2026-07-27 is outside the forward-only
    // universe — not judged even when unrouted. Closed at or after the
    // boundary is inside. The pre-boundary child (#7) is deliberately
    // unrouted: were the boundary mis-mapped, it would trip the gate.
    let repo = TempRepo::new();
    serve_review_round_with_children(
        &repo,
        &[
            issue(7, Some(100), "closed", Some("2026-07-20T12:00:00Z")), // before -> out
            issue(8, Some(100), "closed", Some(CLOSED_AFTER_BOUNDARY)),  // after -> in
            issue(9, Some(100), "closed", Some("2026-07-27T00:00:00.000Z")), // boundary day -> in
        ],
    );
    // The in-universe closes are routed so the gate verdict isolates the
    // boundary question.
    repo.serve("issue-8.json", &issue_detail(&[], &["plan"]));
    repo.serve("issue-9.json", &issue_detail(&[], &["plan"]));

    let snapshot = acquire_snapshot(repo.root());
    assert_eq!(
        handles(&snapshot),
        vec!["#8", "#9"],
        "the pre-boundary close is never acquired into the universe; \
         boundary-day and later closes are"
    );
    assert!(
        snapshot.findings.iter().all(|f| !f.closed_before_ratification),
        "in-universe records carry closed_before_ratification false"
    );
    assert_eq!(
        gate_verdict(&snapshot),
        GateVerdict::Pass,
        "the unrouted pre-boundary close is not judged (forward-only universe)"
    );
}

// ── The field-readiness guard (both directions) ─────────────────────────────

#[test]
fn a_live_spine_only_snapshot_keeps_the_sibling_checks_dormant() {
    // The live join acquires the enforcement spine only: owner/validator and
    // evidence stay unread (Slice 5), so the checks that read them must stay
    // DORMANT (the could-not-check class) — an open finding without an owner
    // and a fix-closed finding without evidence fire NOTHING on a live
    // spine-only snapshot.
    let repo = TempRepo::new();
    serve_review_round_with_children(
        &repo,
        &[
            issue(7, Some(100), "open", None), // open, owner/validator unacquired
            issue(8, Some(100), "closed", Some(CLOSED_AFTER_BOUNDARY)), // closed, evidence unacquired
        ],
    );
    repo.serve("issue-7.json", &issue_detail(&[], &[]));
    repo.serve("issue-8.json", &issue_detail(&[], &["plan"]));

    let snapshot = acquire_snapshot(repo.root());
    assert_eq!(
        snapshot.finding_fields_acquired,
        FindingFieldsAcquired::SPINE_ONLY,
        "the live join declares spine-only acquisition"
    );
    let kinds = snapshot_integrity(&minimal_state(), &snapshot);
    assert!(
        !kinds.iter().any(|k| k == "findings-missing-owner-or-validator"),
        "the owner/validator check stays dormant on a spine-only snapshot"
    );
    assert!(
        !kinds.iter().any(|k| k == "closed-findings-missing-evidence"),
        "the evidence check stays dormant on a spine-only snapshot"
    );

    // The other direction of the pair: the SAME findings in the fixture
    // shape (no acquisition marker -> defaults to full acquisition, the
    // convergence corpus's path) DO fire both checks — the guard is a gate,
    // not an always-off.
    let mut as_fixture = serde_json::to_value(&snapshot).unwrap();
    as_fixture
        .as_object_mut()
        .unwrap()
        .remove("finding_fields_acquired");
    let full: Snapshot = serde_json::from_value(as_fixture).unwrap();
    let kinds = snapshot_integrity(&minimal_state(), &full);
    assert!(
        kinds.iter().any(|k| k == "findings-missing-owner-or-validator"),
        "under full acquisition the unowned open finding fires"
    );
    assert!(
        kinds.iter().any(|k| k == "closed-findings-missing-evidence"),
        "under full acquisition the evidence-less fix-close fires"
    );
}

#[test]
fn the_convergence_fixtures_with_those_fields_still_fire_the_checks() {
    // The corpus direction of the field-readiness pair, against the real
    // fixtures: full-acquisition snapshots keep firing the owner/validator
    // and evidence checks unchanged.
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let vocabulary: vsdd_core::registry::sets::StatuslineData =
        vsdd_core::registry::load_set(repo_root, "statusline-data")
            .expect("statusline data set loads");
    let corpus = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/convergence");

    let load = |name: &str| -> (State, Snapshot) {
        let dir = corpus.join(name);
        let state = vsdd_core::state::read_state(&dir.join("state.yaml"), &vocabulary)
            .unwrap_or_else(|e| panic!("{name}: state fixture reads: {e}"));
        let snapshot: Snapshot = serde_yaml_ng::from_str(
            &std::fs::read_to_string(dir.join("snapshot.yaml")).unwrap(),
        )
        .unwrap_or_else(|e| panic!("{name}: snapshot fixture parses: {e}"));
        (state, snapshot)
    };

    let (state, snapshot) = load("integrity-missing-owner");
    assert!(
        snapshot_integrity(&state, &snapshot)
            .iter()
            .any(|k| k == "findings-missing-owner-or-validator"),
        "the missing-owner fixture still fires the lifecycle-role check"
    );

    let (state, snapshot) = load("integrity-closed-no-evidence");
    assert!(
        snapshot_integrity(&state, &snapshot)
            .iter()
            .any(|k| k == "closed-findings-missing-evidence"),
        "the closed-no-evidence fixture still fires the evidence check"
    );
}

// ── Finding-leg failure degradation ─────────────────────────────────────────

#[test]
fn a_finding_leg_failure_degrades_to_empty_findings_never_unusable() {
    // REQ-8: a subprocess failure on ANY step of the finding leg leaves the
    // snapshot Acquired with findings empty — the milestone and session legs
    // stay populated, and the outcome is never Unusable. Unparseable leg
    // output degrades the same way.
    let leg_breaks: [&dyn Fn(&TempRepo); 4] = [
        &|repo| repo.fail_leg("fail-review-list"),
        &|repo| repo.fail_leg("fail-all-list"),
        &|repo| repo.fail_leg("fail-show"),
        &|repo| repo.serve("review-list.json", "not json at all"),
    ];
    for (i, break_leg) in leg_breaks.iter().enumerate() {
        let repo = TempRepo::new();
        repo.serve("milestone-list.txt", "#1   [ ] layer 2 (0/2)\n");
        serve_review_round_with_children(
            &repo,
            &[issue(7, Some(100), "closed", Some(CLOSED_AFTER_BOUNDARY))],
        );
        break_leg(&repo);

        let snapshot = acquire_snapshot(repo.root());
        assert_eq!(
            snapshot.acquisition_outcome,
            AcquisitionOutcome::Acquired,
            "leg break {i}: a finding-leg failure never degrades the snapshot \
             to Unusable"
        );
        assert!(
            snapshot.findings.is_empty(),
            "leg break {i}: the failed leg leaves findings empty"
        );
        assert_eq!(
            snapshot.display_active_milestone, "layer 2 (2 open)",
            "leg break {i}: the milestone leg stays populated"
        );
        assert_eq!(
            snapshot.display_session, "session 4",
            "leg break {i}: the session leg stays populated"
        );
        assert_eq!(
            snapshot.display_work_item, "#820 slice one",
            "leg break {i}: the work item stays populated"
        );
    }
}

#[test]
fn a_failed_finding_leg_with_tracker_present_must_not_pass_the_gate_vacuously() {
    // EXPECTED-DEFECT FALSIFIER (red = the found defect, routed not fixed).
    //
    // Guardrail REQ-4 (knowledge page `routing-before-fix-guardrail`):
    // "when ... the finding leg returned findings-absent while the tracker
    // was present, the gate BLOCKS (non-zero) with a distinct message — an
    // unverifiable gate never passes vacuously." The shipped acquisition
    // records NO marker for a failed finding leg (findings empty, the spine
    // group still claimed acquired, no acquisition note), so a leg-failure
    // snapshot is bit-identical to a genuinely clean tracker's and the
    // shipped gate Passes — the vacuous pass the requirement forbids, and
    // the same dormant-vs-clean family as the deferred #818 Fix 2.
    let repo = TempRepo::new();
    serve_review_round_with_children(
        &repo,
        &[issue(8, Some(100), "closed", Some(CLOSED_AFTER_BOUNDARY))],
    );
    // The finding closed by fix, unrouted — and the leg that would reveal it
    // fails with the tracker present and the milestone/session legs healthy.
    repo.serve("issue-8.json", &issue_detail(&[], &["result"]));
    repo.fail_leg("fail-show");

    let snapshot = acquire_snapshot(repo.root());
    assert_eq!(snapshot.acquisition_outcome, AcquisitionOutcome::Acquired);
    assert_ne!(
        gate_verdict(&snapshot),
        GateVerdict::Pass,
        "guardrail REQ-4: a finding leg that returned findings-absent while \
         the tracker was present must not pass the gate vacuously"
    );
}

// ── The finding-cap truncation marker ───────────────────────────────────────

#[test]
fn an_over_cap_universe_truncates_with_a_worded_marker_never_silently() {
    // REQ-4 (finding-query-join): reaching the hard cap surfaces a worded
    // truncation marker on the snapshot — findings past the cap are not
    // examined, never silently dropped. Child ids sit away from the review
    // round's id; the walk is id-ordered, so exactly the first 500 acquire.
    let repo = TempRepo::new();
    let children: Vec<serde_json::Value> = (1000..1505)
        .map(|id| issue(id, Some(100), "open", None))
        .collect();
    serve_review_round_with_children(&repo, &children);

    let snapshot = acquire_snapshot(repo.root());
    assert_eq!(
        snapshot.findings.len(),
        500,
        "the walk caps at the declared bound"
    );
    assert_eq!(snapshot.findings.first().unwrap().handle, "#1000");
    assert_eq!(
        snapshot.findings.last().unwrap().handle,
        "#1499",
        "id-ordered: exactly the first five hundred acquire"
    );
    let note = snapshot
        .finding_acquisition_note
        .as_deref()
        .expect("an over-cap walk carries the worded truncation marker");
    assert!(
        note.contains("capped at 500"),
        "the marker words the cap: {note}"
    );
}

// ── The disposition label-carry mapping ─────────────────────────────────────

#[test]
fn disposition_labels_carry_and_an_unlabelled_close_maps_to_none() {
    // D1 (finding-query-join): `dismissed`/`hallucinated`/`consolidated`
    // labels map to the record's disposition, exempting the closure from the
    // unrouted-findings query; a close without a disposition label — even
    // one carrying an unrelated label — maps to None and is judged.
    let repo = TempRepo::new();
    serve_review_round_with_children(
        &repo,
        &[
            issue(7, Some(100), "closed", Some(CLOSED_AFTER_BOUNDARY)),
            issue(8, Some(100), "closed", Some(CLOSED_AFTER_BOUNDARY)),
            issue(9, Some(100), "closed", Some(CLOSED_AFTER_BOUNDARY)),
            issue(10, Some(100), "closed", Some(CLOSED_AFTER_BOUNDARY)),
        ],
    );
    repo.serve("issue-7.json", &issue_detail(&["dismissed"], &[]));
    repo.serve("issue-8.json", &issue_detail(&["hallucinated"], &[]));
    repo.serve("issue-9.json", &issue_detail(&["consolidated", "security"], &[]));
    // The mis-map negative: an unrelated label is NOT a disposition.
    repo.serve("issue-10.json", &issue_detail(&["security"], &["result"]));

    let snapshot = acquire_snapshot(repo.root());
    let disposition_of = |handle: &str| {
        snapshot
            .findings
            .iter()
            .find(|f| f.handle == handle)
            .unwrap_or_else(|| panic!("{handle} acquired"))
            .disposition
            .clone()
    };
    assert_eq!(disposition_of("#7").as_deref(), Some("dismissed"));
    assert_eq!(disposition_of("#8").as_deref(), Some("hallucinated"));
    assert_eq!(
        disposition_of("#9").as_deref(),
        Some("consolidated"),
        "a disposition label among unrelated labels still carries"
    );
    assert_eq!(
        disposition_of("#10"),
        None,
        "an unrelated label is not a disposition"
    );
    assert_eq!(
        unrouted_findings(&snapshot),
        vec!["#10".to_string()],
        "disposition closures are exempt; only the undispositioned, unrouted \
         close is judged"
    );
    assert_eq!(
        gate_verdict(&snapshot),
        GateVerdict::Block(vec!["#10".to_string()])
    );
}
