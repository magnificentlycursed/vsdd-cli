//! Layer 3 red gate — the statusline-conduct slice of Status detection
//! (vsdd-cli #772). Phase 2a suite: fails executed against the
//! pre-implementation stubs; 2b turns it green.
//!
//! MODEL-ABSENCE, the harness property: this suite runs under cargo
//! test with no model credentials injected and no network use anywhere
//! in the exercised path — a model call would fail loudly here, which
//! is the criterion's enforcement shape ("enforced by the offline
//! criterion rather than asserted"). The env scrub below makes the
//! property explicit per process.
//!
//! Every expectation below is drawn from the registered statusline
//! data set (widths, marks, marker words, next-step texts) or the
//! ratified Status requirement — data-driven, never hardcoded twice.

use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use vsdd::status::broken::compose_broken_state;
use vsdd::status::human::render_human;
use vsdd::status::instruments::CountingReader;
use vsdd::status::machine::render_machine;
use vsdd::status::multi::{read_repo_set_config, render_multi};
use vsdd::status::run_statusline;
use vsdd::status::segment::render_segment;

use vsdd_core::answer::derive::derive_phase_answer;
use vsdd_core::answer::PhaseAnswer;
use vsdd_core::registry::{
    self,
    sets::{CompositionScopeAndActions, StatuslineData},
};
use vsdd_core::snapshot::Snapshot;
use vsdd_core::state::read_state;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/statusline")
}

fn corpus() -> PathBuf {
    repo_root().join("vsdd-core/tests/fixtures/convergence")
}

fn data() -> StatuslineData {
    registry::load_set(&repo_root(), "statusline-data").expect("statusline data loads")
}

fn actions() -> CompositionScopeAndActions {
    registry::load_set(&repo_root(), "composition-scope-and-actions")
        .expect("composition set loads")
}

/// The model-absence scrub: the exercised path must need none of these,
/// and their absence makes any model call fail loudly.
fn scrub_model_credentials() {
    for key in ["ANTHROPIC_API_KEY", "CLAUDE_API_KEY", "OPENAI_API_KEY"] {
        std::env::remove_var(key);
    }
}

fn load(dir: &Path) -> (PhaseAnswer, Snapshot) {
    let state = read_state(&dir.join("state.yaml"), &data()).expect("fixture state reads");
    let snapshot: Snapshot =
        serde_yaml_ng::from_str(&fs::read_to_string(dir.join("snapshot.yaml")).unwrap())
            .expect("fixture snapshot parses");
    let answer = derive_phase_answer(&state, &snapshot, &actions());
    (answer, snapshot)
}

fn field_budget(d: &StatuslineData, name: &str) -> usize {
    d.display_fields
        .iter()
        .find(|f| f.field == name)
        .unwrap_or_else(|| panic!("display field `{name}` registered"))
        .width_budget_chars as usize
}

fn degraded_kind<'a>(
    d: &'a StatuslineData,
    kind: &str,
) -> &'a vsdd_core::registry::sets::DegradedKind {
    d.degraded_kinds
        .iter()
        .find(|k| k.kind == kind)
        .unwrap_or_else(|| panic!("degraded kind `{kind}` registered"))
}

fn strip_ansi(s: &str) -> String {
    // Enough for the conduct check: drop ESC [ ... final-byte sequences.
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            if chars.peek() == Some(&'[') {
                chars.next();
                for f in chars.by_ref() {
                    if ('\u{40}'..='\u{7e}').contains(&f) {
                        break;
                    }
                }
                continue;
            }
            continue;
        }
        out.push(c);
    }
    out
}

// ── The segment's enumerated properties ────────────────────────────────

#[test]
fn segment_renders_the_four_fields_in_order() {
    scrub_model_credentials();
    let (answer, snapshot) = load(&corpus().join("3-reviewing"));
    let segment = render_segment(&answer, &snapshot, &data());
    assert!(!segment.is_empty(), "the segment renders, never blank");
    let repo = segment
        .find(&snapshot.display_repo_name)
        .expect("repo name present");
    let phase = segment
        .find(&answer.next_action)
        .expect("phase answer present");
    let work = segment
        .find(&snapshot.display_work_item)
        .expect("work item present");
    assert!(
        repo < phase && phase < work,
        "field order: repo, answer, work item, milestone"
    );
    assert!(
        !segment.contains(&snapshot.display_session),
        "the session is not a segment field (the demotion ruling)"
    );
}

#[test]
fn segment_is_byte_identical_across_invocations() {
    scrub_model_credentials();
    let (answer, snapshot) = load(&corpus().join("3-reviewing"));
    let d = data();
    let first = render_segment(&answer, &snapshot, &d);
    let second = render_segment(&answer, &snapshot, &d);
    assert!(!first.is_empty(), "a rendered segment is never blank");
    assert_eq!(first, second, "byte-identical across repeated invocations");
}

#[test]
fn over_width_field_truncates_with_the_worded_mark_and_the_count_survives() {
    scrub_model_credentials();
    let (answer, snapshot) = load(&fixtures().join("over-width"));
    let d = data();
    let segment = render_segment(&answer, &snapshot, &d);
    assert!(
        segment.contains(&format!(" {}", d.truncation_mark)),
        "the mark is set off by a space, never glued: {segment:?}"
    );
    assert!(
        segment.contains("(5 open)"),
        "the open count survives name truncation (the #680 ruling): {segment:?}"
    );
    let budget = field_budget(&d, "milestone-with-count");
    let milestone_rendering = segment
        .split("  ")
        .find(|part| part.contains("(5 open)"))
        .expect("the milestone field is locatable in the segment");
    assert!(
        milestone_rendering.chars().count() <= budget,
        "the field honors its width budget of {budget}: {milestone_rendering:?}"
    );
}

#[test]
fn absent_fields_render_worded_absences_never_empty_slots() {
    scrub_model_credentials();
    let (answer, snapshot) = load(&fixtures().join("absences"));
    let d = data();
    let segment = render_segment(&answer, &snapshot, &d);
    let work_absence = &d
        .display_fields
        .iter()
        .find(|f| f.field == "work-item")
        .unwrap()
        .absence_text;
    let milestone_absence = &d
        .display_fields
        .iter()
        .find(|f| f.field == "milestone-with-count")
        .unwrap()
        .absence_text;
    assert!(
        segment.contains(work_absence),
        "worded work-item absence: {segment:?}"
    );
    assert!(
        segment.contains(milestone_absence),
        "worded milestone absence: {segment:?}"
    );
}

#[test]
fn degraded_segments_render_the_kinds_marker_word() {
    scrub_model_credentials();
    let d = data();
    for (fixture, kind) in [
        ("degraded-tracker-absent", "tracker-absent"),
        ("degraded-tracker-unusable", "tracker-unusable"),
    ] {
        let (answer, snapshot) = load(&corpus().join(fixture));
        let segment = render_segment(&answer, &snapshot, &d);
        let marker = &degraded_kind(&d, kind).marker_word;
        assert!(
            segment.contains(marker.as_str()),
            "{fixture}: the degraded marker renders as the plain word {marker:?}: {segment:?}"
        );
    }
}

// ── The human form ─────────────────────────────────────────────────────

#[test]
fn human_form_names_the_degraded_kind_and_its_full_next_step() {
    scrub_model_credentials();
    let d = data();
    for (fixture, kind) in [
        ("degraded-tracker-absent", "tracker-absent"),
        ("degraded-tracker-unusable", "tracker-unusable"),
    ] {
        let (answer, snapshot) = load(&corpus().join(fixture));
        let human = render_human(&answer, &snapshot, &d);
        let k = degraded_kind(&d, kind);
        assert!(human.contains(kind), "{fixture}: the kind is named");
        assert!(
            human.contains(k.next_step_text.as_str()),
            "{fixture}: the kind's full next-step text renders, exact"
        );
    }
}

#[test]
fn human_form_is_a_superset_of_the_segment_and_renders_the_session() {
    scrub_model_credentials();
    let (answer, snapshot) = load(&corpus().join("3-reviewing"));
    let d = data();
    let human = render_human(&answer, &snapshot, &d);
    for value in [
        snapshot.display_repo_name.as_str(),
        answer.next_action.as_str(),
        snapshot.display_work_item.as_str(),
        snapshot.display_active_milestone.as_str(),
    ] {
        assert!(
            human.contains(value),
            "the human form carries the segment field {value:?}"
        );
    }
    assert!(
        human.contains(snapshot.display_session.as_str()),
        "the session renders in the human form (the demotion ruling)"
    );
}

#[test]
fn human_form_carries_the_two_sections_of_the_facets_note() {
    scrub_model_credentials();
    let (answer, snapshot) = load(&corpus().join("disagreement-files-finding"));
    let human = render_human(&answer, &snapshot, &data());
    let answer_at = human.find("answer").expect("the position section is named");
    let report_at = human.find("report").expect("the health section is named");
    assert!(answer_at < report_at, "position before health");
    assert!(
        human.contains("phase-pointer-against-milestone-state"),
        "the integrity finding renders in the report section"
    );
}

// ── The machine form ───────────────────────────────────────────────────

#[test]
fn machine_form_carries_the_named_blocks_and_is_a_superset() {
    scrub_model_credentials();
    let (answer, snapshot) = load(&corpus().join("3-reviewing"));
    let machine = render_machine(&answer, &snapshot, &data());
    let answer_block = machine.get("answer").expect("the `answer` block is named");
    let report_block = machine.get("report").expect("the `report` block is named");
    assert_eq!(
        answer_block.get("next_action").and_then(|v| v.as_str()),
        Some(answer.next_action.as_str()),
        "position lives in the answer block"
    );
    assert!(report_block.is_object(), "health lives in the report block");
    let rendered = machine.to_string();
    for value in [
        snapshot.display_repo_name.as_str(),
        snapshot.display_work_item.as_str(),
        snapshot.display_active_milestone.as_str(),
        snapshot.display_session.as_str(),
    ] {
        assert!(
            rendered.contains(value),
            "the machine form carries {value:?}"
        );
    }
}

// ── The broken-state branch, all three surfaces, every kind ────────────

fn broken_fixture(kind: &str) -> (tempfile::TempDir, vsdd_core::diagnostics::Diagnostic) {
    let dir = tempfile::tempdir().unwrap();
    let state_path = dir.path().join(".vsdd/state.yaml");
    fs::create_dir_all(state_path.parent().unwrap()).unwrap();
    match kind {
        "malformed" => fs::write(&state_path, "{{ this is not yaml: [").unwrap(),
        "absent" => {}
        "permission-or-io" => {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::write(&state_path, "schema_version: \"0.1.0\"\n").unwrap();
                fs::set_permissions(&state_path, fs::Permissions::from_mode(0o000)).unwrap();
            }
        }
        other => panic!("unregistered kind {other}"),
    }
    let diagnostic = *read_state(&state_path, &data())
        .expect_err("the seeded state read fails with a diagnostic");
    (dir, diagnostic)
}

#[test]
fn broken_state_speaks_on_all_three_surfaces_for_every_kind() {
    scrub_model_credentials();
    let d = data();
    let acts = actions();
    for kind in ["malformed", "absent", "permission-or-io"] {
        let (_dir, diagnostic) = broken_fixture(kind);
        let surfaces = compose_broken_state(&diagnostic, &d, None);
        // The glance surface is never blank: the registered mark.
        assert!(
            surfaces.segment.contains(d.broken_state_mark.as_str()),
            "{kind}: the segment renders the broken-state mark"
        );
        // The human form: rustc-shaped, file, kind, per-kind recovery.
        let registered = d
            .read_failure_kinds
            .iter()
            .find(|k| k.machine_token == diagnostic.machine_token)
            .expect("the diagnostic's kind is a registered read-failure kind");
        assert!(
            surfaces.human.contains("state.yaml"),
            "{kind}: the human form names the state file"
        );
        assert!(
            surfaces.human.contains(registered.human_recovery.as_str()),
            "{kind}: the human form carries the kind's recovery text"
        );
        if kind == "malformed" {
            assert!(
                diagnostic.location.is_some() && surfaces.human.contains("line"),
                "malformed carries its parse location into the human form"
            );
        }
        // The machine form: enumerated kind exact, payload, vocabulary action.
        let signal = surfaces
            .machine
            .get("state_unreadable")
            .expect("{kind}: the structured state-unreadable signal is present");
        assert_eq!(
            signal.get("kind").and_then(|v| v.as_str()),
            Some(diagnostic.machine_token.as_str()),
            "{kind}: the kind matches by exact match"
        );
        assert!(
            signal.get("diagnostic").is_some(),
            "{kind}: the diagnostic payload is present"
        );
        let action = signal
            .get("recovery_action")
            .and_then(|v| v.as_str())
            .expect("{kind}: the recovery action is present");
        assert_eq!(
            action, registered.recovery_action,
            "{kind}: the kind's registered action"
        );
        assert!(
            acts.action_vocabulary.iter().any(|a| a.id == action),
            "{kind}: the action is a vocabulary member"
        );
    }
}

#[test]
fn broken_state_carries_the_last_boundary_truth_or_its_worded_absence() {
    scrub_model_credentials();
    let d = data();
    let (_dir, diagnostic) = broken_fixture("malformed");
    let subject = "phase 1c: decomposition ratified (boundary)";
    let with = compose_broken_state(&diagnostic, &d, Some(subject));
    assert!(
        with.human.contains(subject),
        "the last boundary subject renders on the human form (vsdd-cli #740)"
    );
    assert!(
        with.machine.to_string().contains(subject),
        "and on the machine form"
    );
    let without = compose_broken_state(&diagnostic, &d, None);
    assert!(
        without.human.contains("no boundary commit recorded"),
        "absence of a boundary is a worded absence, never an omission"
    );
}

// ── The conduct instruments ────────────────────────────────────────────

#[test]
fn the_statusline_path_consumes_zero_stdin_and_leaks_no_sentinel() {
    scrub_model_credentials();
    let sentinel = fs::read_to_string(fixtures().join("sentinel-stdin/session.json")).unwrap();
    let reader = CountingReader::new(sentinel.as_bytes());
    let snapshot_dir = corpus().join("3-reviewing");
    let snapshot: Snapshot =
        serde_yaml_ng::from_str(&fs::read_to_string(snapshot_dir.join("snapshot.yaml")).unwrap())
            .unwrap();
    let run = run_statusline(&repo_root(), reader, &data(), &actions(), move |_root| {
        snapshot.clone()
    });
    assert_eq!(
        run.instruments.stdin_bytes_read, 0,
        "the statusline path reads zero stdin bytes — the counted seam"
    );
    for sentinel_value in ["XYZZY-SENTINEL-MODEL", "XYZZY-SENTINEL-DIR", "424242"] {
        assert!(
            !run.segment.contains(sentinel_value),
            "no sentinel value leaks into the segment: {sentinel_value}"
        );
    }
    assert!(!run.segment.is_empty(), "the run renders a segment");
}

#[test]
fn exactly_one_acquisition_per_invocation() {
    scrub_model_credentials();
    let snapshot_dir = corpus().join("3-reviewing");
    let snapshot: Snapshot =
        serde_yaml_ng::from_str(&fs::read_to_string(snapshot_dir.join("snapshot.yaml")).unwrap())
            .unwrap();
    let mut calls = 0u64;
    let run = run_statusline(
        &repo_root(),
        std::io::empty(),
        &data(),
        &actions(),
        |_root| {
            calls += 1;
            snapshot.clone()
        },
    );
    assert_eq!(
        calls, 1,
        "exactly one snapshot acquisition — the counted seam"
    );
    assert_eq!(
        run.instruments.acquisition_count, 1,
        "and the instrument reports it"
    );
}

#[test]
fn the_whole_invocation_fits_the_wall_clock_budget() {
    scrub_model_credentials();
    let d = data();
    let budget = Duration::from_millis(d.wall_clock_budget_ms);
    let snapshot_dir = corpus().join("3-reviewing");
    let snapshot: Snapshot =
        serde_yaml_ng::from_str(&fs::read_to_string(snapshot_dir.join("snapshot.yaml")).unwrap())
            .unwrap();
    // The registered flake shape: 7 runs, median at or under budget,
    // max at or under twice budget.
    let mut samples: VecDeque<Duration> = VecDeque::new();
    let mut last_segment = String::new();
    for _ in 0..7 {
        let started = Instant::now();
        let run = run_statusline(
            &repo_root(),
            std::io::empty(),
            &data(),
            &actions(),
            |_root| snapshot.clone(),
        );
        samples.push_back(started.elapsed());
        last_segment = run.segment;
    }
    let mut sorted: Vec<Duration> = samples.into_iter().collect();
    sorted.sort();
    assert!(
        sorted[3] <= budget,
        "median at or under the {budget:?} budget"
    );
    assert!(
        *sorted.last().unwrap() <= budget * 2,
        "ceiling at or under twice the budget"
    );
    assert!(!last_segment.is_empty(), "the timed run renders a segment");
}

// ── Color conduct ──────────────────────────────────────────────────────

#[test]
fn stripping_color_loses_no_information() {
    scrub_model_credentials();
    let (answer, snapshot) = load(&corpus().join("degraded-tracker-absent"));
    let d = data();
    let segment = render_segment(&answer, &snapshot, &d);
    let human = render_human(&answer, &snapshot, &d);
    for (name, text) in [("segment", &segment), ("human form", &human)] {
        let stripped = strip_ansi(text);
        for value in [
            snapshot.display_repo_name.as_str(),
            snapshot.display_work_item.as_str(),
            degraded_kind(&d, "tracker-absent").marker_word.as_str(),
        ] {
            assert!(
                stripped.contains(value),
                "{name}: {value:?} survives color stripping"
            );
        }
    }
}

// ── The composed multi-repo display ────────────────────────────────────

#[test]
fn the_composed_display_orders_current_first_with_worded_identity() {
    scrub_model_credentials();
    let d = data();
    let (answer_a, snap_a) = load(&corpus().join("3-reviewing"));
    let (answer_b, snap_b) = load(&corpus().join("degraded-tracker-absent"));
    let current = render_segment(&answer_a, &snap_a, &d);
    let other = render_segment(&answer_b, &snap_b, &d);
    let composed = render_multi(&current, &[other]);
    let lines: Vec<&str> = composed.lines().collect();
    assert_eq!(lines.len(), 2, "one line per configured repo");
    assert!(
        lines[0].contains(&snap_a.display_repo_name),
        "the current repo renders first, named"
    );
    assert!(
        lines[1].contains(&snap_b.display_repo_name),
        "every line carries its worded repo identity"
    );
    assert!(
        lines[1].contains(degraded_kind(&d, "tracker-absent").marker_word.as_str()),
        "a degraded member renders degraded in the composition"
    );
    let stripped = strip_ansi(&composed);
    assert!(
        stripped.contains(&snap_b.display_repo_name),
        "the composition passes the color-strip check"
    );
}

#[test]
fn the_repo_set_config_parses_and_a_malformed_one_is_a_diagnostic() {
    scrub_model_credentials();
    let dir = tempfile::tempdir().unwrap();
    let good = dir.path().join("statusline.yaml");
    fs::write(
        &good,
        "repos:\n  - /one\n  - /two\nper_repo_budget_ms: 250\n",
    )
    .unwrap();
    let config = read_repo_set_config(&good).expect("the registered shape parses");
    assert_eq!(config.repos.len(), 2, "the repo set is the explicit list");
    assert_eq!(config.per_repo_budget_ms, 250);
    let bad = dir.path().join("broken.yaml");
    fs::write(&bad, "repos: [unclosed").unwrap();
    let diagnostic = read_repo_set_config(&bad)
        .expect_err("a malformed config is a diagnostic, never a silent empty set");
    assert!(
        !diagnostic.message.is_empty(),
        "the diagnostic says what failed"
    );
}

// ── The wiring script deliverable ──────────────────────────────────────

#[test]
fn the_wiring_script_template_exists_and_names_its_config() {
    scrub_model_credentials();
    let script = repo_root().join("templates/statusline/vsdd-statusline.sh");
    let content = fs::read_to_string(&script)
        .expect("the wiring script template exists at its registered home");
    assert!(
        content.contains("statusline.yaml"),
        "the script names the repo-set config"
    );
    assert!(
        content.contains("vsdd status --statusline"),
        "the script carries the exact invocation line"
    );
}
