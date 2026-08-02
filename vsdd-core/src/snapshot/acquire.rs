//! The effectful snapshot acquisition (the shell side of the purity
//! split): builds the [`Snapshot`](super::Snapshot) from crosslink's
//! query surface via bounded, timed subprocess runs (vsdd-cli #751).
//! One acquisition per invocation; the count-conduct instruments join
//! at Layer 3.
//!
//! Absent and unusable are OUTCOMES carried in the snapshot, never
//! errors — and they are never swapped (vsdd-cli #747): only a binary
//! missing from PATH is the offline shape; present-but-broken
//! crosslink (spawn failure, timeout, oversize output, refused command,
//! unparseable output) is the unusable outcome.
//!
//! Bootstrap scope, declared (vsdd-cli #741): milestones ride
//! crosslink's list surface and the session fields its status JSON; the
//! tracker-join fields (findings, round manifests and children, comment
//! handles) acquire EMPTY until their query consumers land with the
//! parity and lifecycle gates (Layer 6) — the convergence corpus
//! supplies those fields for the pure checks meanwhile. The finding
//! record's routing_present and closed_before_ratification datums (the
//! unrouted-findings query's inputs; vsdd-cli #811) ride that same Layer
//! 6 join: the routing presence from the finding's `plan` comments, and
//! the forward-only universe boundary (REQ-5) applied here at
//! acquisition, keyed to the routing amendment's ratification boundary,
//! so a finding closed before it acquires closed_before_ratification
//! true and stays outside the query's universe. The milestone
//! list is a human-format parse (no JSON surface exists upstream);
//! output that fails the parse IS the unusable outcome, never a guess
//! (vsdd-cli #748). DECLARED CONFLATION (vsdd-cli #753): the session
//! status surface cannot distinguish no-active-session from a refusal
//! at bootstrap — a refused session query renders the worded absence;
//! the distinction lands when crosslink exposes it. DECLARED
//! CONFLATION (vsdd-cli #763, implemented per #766): the
//! active-milestone gauge renders the milestone's open-ISSUE count —
//! at bootstrap that count stands in for the open-finding count the
//! snapshot schema names, until the tracker join (Layer 6) can tell
//! findings from other issues; the schema's own quantity takes over
//! there.

use std::path::Path;

use crate::subprocess::{run_bounded, Subprocess};

use serde::Deserialize;

use super::{AcquisitionOutcome, FindingFieldsAcquired, FindingRecord, MilestoneState, Snapshot};

/// The bootstrap absence wordings. Declared mirrors of the statusline
/// data set's `absence_text` for the fields that have one (vsdd-cli
/// #791; the #724 declared-mirror pattern): a fidelity test pins the
/// two registered ones so an edit to the data set cannot silently
/// diverge the acquisition from the render layer. `session` has no
/// registered display field (the demotion ruling), so it carries no
/// mirror to pin.
const ABSENT_SESSION: &str = "no session";
const ABSENT_WORK_ITEM: &str = "no work item";
const ABSENT_MILESTONE: &str = "no milestone";

/// crosslink's empty-state output for a repo with no milestones — a declared
/// mirror (vsdd-cli #829): recognized as a lawful empty list, distinct from the
/// whole-format drift the parser otherwise fails loud on.
const EMPTY_MILESTONE_LIST: &str = "No milestones found.";

/// Acquire the corroboration snapshot for the repo.
pub fn acquire_snapshot(repo_root: &Path) -> Snapshot {
    let repo_name = repo_root
        .file_name()
        .map(|n| clean_for_display(&n.to_string_lossy()))
        .unwrap_or_else(|| "unnamed repo".to_string());

    if !repo_root.join(".crosslink").is_dir() {
        return empty(AcquisitionOutcome::Absent, repo_name);
    }

    let milestone_text = match run_bounded("crosslink", &["milestone", "list"], repo_root) {
        Subprocess::Completed { stdout } => stdout,
        Subprocess::NotFound => return empty(AcquisitionOutcome::Absent, repo_name),
        // Broken, wedged, oversize, or refused: unusable, never offline.
        _ => return empty(AcquisitionOutcome::Unusable, repo_name),
    };
    let Some(parsed) = parse_milestones(&milestone_text) else {
        return empty(AcquisitionOutcome::Unusable, repo_name);
    };

    let (session, work_item) =
        match run_bounded("crosslink", &["session", "status", "--json"], repo_root) {
            Subprocess::Completed { stdout } => match parse_session(&stdout) {
                Some(pair) => pair,
                None => return empty(AcquisitionOutcome::Unusable, repo_name),
            },
            Subprocess::NotFound => return empty(AcquisitionOutcome::Absent, repo_name),
            // The declared bootstrap conflation: a refused session query
            // renders the worded absence (module doc; vsdd-cli #753).
            Subprocess::Refused { .. } => {
                (ABSENT_SESSION.to_string(), ABSENT_WORK_ITEM.to_string())
            }
            _ => return empty(AcquisitionOutcome::Unusable, repo_name),
        };

    let active_display = parsed
        .iter()
        .rev()
        .find(|p| p.state.is_active)
        .map(|p| match p.counts {
            // The gauge slot the schema precomputes; at bootstrap the
            // value is the open-ISSUE count standing in for the
            // open-finding count — the declared conflation in the
            // module doc (vsdd-cli #763, #766).
            Some((closed, total)) => {
                format!("{} ({} open)", p.state.name, total.saturating_sub(closed))
            }
            None => p.state.name.clone(),
        })
        .unwrap_or_else(|| ABSENT_MILESTONE.to_string());

    // The finding-query join: findings acquire as the spine-only Slice-1 leg.
    // A leg failure leaves findings absent, the snapshot still Acquired
    // (REQ-8) — but RECORDED, never erased (vsdd-cli #818 Fix 2): the marker
    // drops to no-groups-acquired and the note words the failed step, so a
    // failed leg is never bit-identical to a genuinely clean tracker and the
    // routing gate fails closed on it (guardrail REQ-4) instead of passing
    // vacuously.
    let (findings, finding_fields_acquired, finding_acquisition_note) =
        match acquire_findings(repo_root) {
            Ok((findings, truncated)) => {
                let note = truncated.then(|| {
                    format!(
                        "finding query capped at {FINDING_QUERY_CAP}; findings past the cap were not examined this acquisition"
                    )
                });
                (findings, FindingFieldsAcquired::SPINE_ONLY, note)
            }
            Err(failed_step) => (
                Vec::new(),
                FindingFieldsAcquired::NONE,
                Some(format!(
                    "finding query failed ({failed_step}); findings could not be acquired this acquisition"
                )),
            ),
        };

    Snapshot {
        acquisition_outcome: AcquisitionOutcome::Acquired,
        milestones: parsed.into_iter().map(|p| p.state).collect(),
        findings,
        round_manifests: Vec::new(),
        round_children: Vec::new(),
        comment_handles: Vec::new(),
        display_repo_name: repo_name,
        display_session: session,
        display_work_item: work_item,
        display_active_milestone: active_display,
        finding_fields_acquired,
        finding_acquisition_note,
    }
}

/// The list surface's line shape: `#5   [ ] name (1/2)` — `[✓]` closed.
///
/// Fail-loud discipline (vsdd-cli #748): output containing non-empty
/// lines but ZERO parseable milestone lines is `None` — whole-format
/// drift becomes the unusable outcome, never an empty success. The
/// count suffix strips only when it matches the exact `(N/M)` digit
/// shape, so a name legitimately ending in a parenthetical survives.
struct ParsedMilestone {
    state: MilestoneState,
    /// `(closed, total)` from the `(N/M)` suffix when present.
    counts: Option<(u64, u64)>,
}

fn parse_milestones(text: &str) -> Option<Vec<ParsedMilestone>> {
    // crosslink's empty-state message is a lawful empty list, not the
    // whole-format drift the fail-loud check below guards against (vsdd-cli #829).
    if text.trim() == EMPTY_MILESTONE_LIST {
        return Some(Vec::new());
    }
    let mut out = Vec::new();
    let mut content_lines = 0usize;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        content_lines += 1;
        if !line.starts_with('#') {
            continue;
        }
        let open_bracket = line.find('[')?;
        let close_bracket = line.find(']')?;
        let closed = line.get(open_bracket + 1..close_bracket)? != " ";
        let rest = line.get(close_bracket + 1..)?.trim();
        let (name, counts) = split_count_suffix(rest);
        if name.is_empty() {
            return None;
        }
        out.push(ParsedMilestone {
            state: MilestoneState {
                name: clean_for_display(name),
                state: if closed { "closed" } else { "open" }.to_string(),
                // Bootstrap: open reads as active; crosslink's own
                // active concept deepens this when it surfaces one.
                is_active: !closed,
            },
            counts,
        });
    }
    if content_lines > 0 && out.is_empty() {
        return None;
    }
    Some(out)
}

/// Strip a trailing ` (N/M)` count suffix exactly; anything else —
/// including a name ending in a parenthetical phrase — stays intact.
fn split_count_suffix(rest: &str) -> (&str, Option<(u64, u64)>) {
    if let Some(cut) = rest.rfind(" (") {
        if let Some(inner) = rest[cut + 2..].strip_suffix(')') {
            if let Some((closed, total)) = inner.split_once('/') {
                if !closed.is_empty()
                    && !total.is_empty()
                    && closed.bytes().all(|b| b.is_ascii_digit())
                    && total.bytes().all(|b| b.is_ascii_digit())
                {
                    if let (Ok(c), Ok(t)) = (closed.parse(), total.parse()) {
                        return (rest[..cut].trim_end(), Some((c, t)));
                    }
                }
            }
        }
    }
    (rest, None)
}

fn parse_session(text: &str) -> Option<(String, String)> {
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    let session = value
        .get("session_id")
        .and_then(|v| v.as_u64())
        .map(|id| format!("session {id}"))
        .unwrap_or_else(|| ABSENT_SESSION.to_string());
    let work_item = value
        .get("working_on")
        .and_then(|w| {
            let id = w.get("display_id")?.as_str()?;
            let title = w.get("title")?.as_str()?;
            Some(clean_for_display(&format!("{id} {title}")))
        })
        .unwrap_or_else(|| ABSENT_WORK_ITEM.to_string());
    Some((session, work_item))
}

/// Terminal-destined strings drop control characters at the boundary
/// (vsdd-cli #754): crosslink output is data, never terminal input.
fn clean_for_display(s: &str) -> String {
    // The one shared terminal-cleaning policy (vsdd-cli #788): control
    // characters AND display-spoofing bidi/zero-width/format chars.
    crate::text::clean_for_terminal(s)
}

fn empty(outcome: AcquisitionOutcome, repo_name: String) -> Snapshot {
    Snapshot {
        acquisition_outcome: outcome,
        milestones: Vec::new(),
        findings: Vec::new(),
        round_manifests: Vec::new(),
        round_children: Vec::new(),
        comment_handles: Vec::new(),
        display_repo_name: repo_name,
        display_session: ABSENT_SESSION.to_string(),
        display_work_item: ABSENT_WORK_ITEM.to_string(),
        display_active_milestone: ABSENT_MILESTONE.to_string(),
        finding_fields_acquired: FindingFieldsAcquired::default(),
        finding_acquisition_note: None,
    }
}

/// The most recent boundary commit's subject from this clone's own
/// history (vsdd-cli #740): the broken-state surfaces render it as
/// recovery context — where the build stood when the state artifact
/// was last provably right. Bootstrap heuristic, declared: the newest
/// first-parent subject containing the word "boundary" within the last
/// hundred; the boundary-commit grammar deepens when the gate
/// machinery (Layer 6) registers one. Every failure shape is None —
/// the caller words the absence.
pub fn last_boundary_subject(repo_root: &Path) -> Option<String> {
    match run_bounded(
        "git",
        &["log", "--format=%s", "-n", "100", "--first-parent"],
        repo_root,
    ) {
        Subprocess::Completed { stdout } => stdout
            .lines()
            .map(str::trim)
            .find(|s| s.to_lowercase().contains("boundary"))
            .map(clean_for_display),
        _ => None,
    }
}

// ── Tracker-join mappers: the unrouted-findings query's pure inputs (#820) ──
// Slice 1's enforcement-spine increment, mechanizing the routing-before-fix
// format-carry (#810/#811). Pure over canned tracker data; the finding-query
// join below consumes routing_present and closed_before_ratification.

/// True when the finding's parent is a review-round issue — the bootstrap
/// finding-discrimination rule: a `review`-labelled parent marks its children
/// findings (vsdd-cli #820). The live walk applies this predicate SERVER-SIDE
/// via `crosslink issue list --label review` (which returns exactly the parents
/// this would match), so the walk keys on review-id membership; this pure form
/// is retained as the tested specification of the predicate.
#[allow(dead_code)]
fn is_finding(parent_labels: &[String]) -> bool {
    parent_labels.iter().any(|label| label == "review")
}

/// True when a routing edge is present — a `plan`-kind comment on the finding
/// (the bootstrap routing format-carry; vsdd-cli #810).
fn routing_present(comment_kinds: &[String]) -> bool {
    comment_kinds.iter().any(|kind| kind == "plan")
}

/// True when the finding was closed before the routing amendment's ratification
/// boundary — outside the forward-only unrouted-findings universe (REQ-5;
/// vsdd-cli #811). No `closed_at`, or a close at/after the boundary, is IN.
fn closed_before_ratification(closed_at: Option<&str>, boundary: &str) -> bool {
    // ISO-8601 timestamps sort lexicographically; strictly-before excludes the
    // boundary itself, keeping the forward-only universe inclusive of it.
    closed_at.is_some_and(|closed| closed < boundary)
}

// ── The finding-query join: acquire findings from crosslink (#820) ──────────
// The effectful walk populates snapshot.findings for the forward-only universe;
// the pure pieces (parsing, the universe filter, the record builder) are
// unit-tested with canned crosslink JSON, and the thin effectful driver
// (acquire_findings) is verified by manual-tests/layer-2.md — the acquisition
// module's existing pure-unit + manual pattern.

/// The routing amendment's ratification boundary (vsdd-cli #810), date-granular:
/// a finding closed before 2026-07-27 is outside the forward-only universe
/// (REQ-5). Date granularity is deliberate — the routing discipline is
/// day-grained, and a date boundary compares correctly against crosslink's
/// fractional-second `closed_at` under plain lexicographic order (any instant on
/// or after the date has the date as a prefix, so it sorts at or after the
/// boundary), where a zero-fraction full-timestamp boundary would mis-order a
/// fractional instant of the same second.
const RATIFICATION_BOUNDARY: &str = "2026-07-27";

/// The disposition labels the join reads (the D1 label-carry, vsdd-cli #820): a
/// disposition closure carries one of these, mapped to the record's disposition
/// so the unrouted-findings query exempts it. Superseded by an upstream
/// close-reason field (vsdd-cli #827) when it ships.
const DISPOSITION_LABELS: [&str; 3] = ["dismissed", "hallucinated", "consolidated"];

/// Hard cap on findings examined per acquisition (REQ-4): bounds the per-finding
/// `issue show` fan-out. The forward-only universe is naturally small, so this
/// is a defensive wall-clock bound; reaching it surfaces a worded marker rather
/// than silently dropping findings.
const FINDING_QUERY_CAP: usize = 500;

/// A `crosslink issue list --json` item — the fields the walk reads. Labels and
/// comments are NOT on the list surface (verified), only on `issue show`.
#[derive(Deserialize)]
struct IssueListItem {
    id: u64,
    #[serde(default)]
    parent_id: Option<u64>,
    status: String,
    #[serde(default)]
    closed_at: Option<String>,
}

/// The `issue show --json` fields the walk reads: labels (disposition) and
/// comment kinds (routing).
#[derive(Deserialize)]
struct IssueDetail {
    #[serde(default)]
    labels: Vec<String>,
    #[serde(default)]
    comments: Vec<IssueComment>,
}

#[derive(Deserialize)]
struct IssueComment {
    kind: String,
}

fn parse_issue_list(json: &str) -> Option<Vec<IssueListItem>> {
    serde_json::from_str(json).ok()
}

fn parse_issue_detail(json: &str) -> Option<IssueDetail> {
    serde_json::from_str(json).ok()
}

/// Map a finding's labels to its recorded disposition (D1; #820). None when no
/// disposition label is present.
fn disposition_from_labels(labels: &[String]) -> Option<String> {
    labels
        .iter()
        .find(|l| DISPOSITION_LABELS.contains(&l.as_str()))
        .cloned()
}

/// The forward-only universe of findings to examine, keyed on child `parent_id`
/// (issue-show `subissues` is never populated; vsdd-cli #828): issues whose
/// parent is a review-round issue (the finding-discrimination predicate, applied
/// via the server-side review-id set) and which are NOT closed before the
/// ratification boundary (REQ-5). Ordered by id for a deterministic snapshot and
/// cap; capped at [`FINDING_QUERY_CAP`], the bool reporting truncation (REQ-4).
fn findings_in_universe<'a>(
    review_ids: &[u64],
    all_issues: &'a [IssueListItem],
) -> (Vec<&'a IssueListItem>, bool) {
    let mut findings: Vec<&IssueListItem> = all_issues
        .iter()
        .filter(|it| it.parent_id.is_some_and(|p| review_ids.contains(&p)))
        .filter(|it| !closed_before_ratification(it.closed_at.as_deref(), RATIFICATION_BOUNDARY))
        .collect();
    findings.sort_by_key(|it| it.id);
    let truncated = findings.len() > FINDING_QUERY_CAP;
    findings.truncate(FINDING_QUERY_CAP);
    (findings, truncated)
}

/// Build a [`FindingRecord`] from a finding's list item (status, closed_at) and
/// its detail (labels → disposition, comment kinds → routing). Slice 1 acquires
/// the enforcement spine only; owner/validator/evidence acquire in Slice 5, so
/// the snapshot marks [`FindingFieldsAcquired::SPINE_ONLY`] and the sibling
/// checks stay dormant (vsdd-cli #820).
fn finding_record(item: &IssueListItem, detail: &IssueDetail) -> FindingRecord {
    let comment_kinds: Vec<String> = detail.comments.iter().map(|c| c.kind.clone()).collect();
    FindingRecord {
        handle: format!("#{}", item.id),
        status: item.status.clone(),
        owner: None,
        validator: None,
        evidence_reference_present: false,
        disposition: disposition_from_labels(&detail.labels),
        routing_present: routing_present(&comment_kinds),
        closed_before_ratification: closed_before_ratification(
            item.closed_at.as_deref(),
            RATIFICATION_BOUNDARY,
        ),
    }
}

/// The effectful finding-query leg (vsdd-cli #820): walk crosslink for the
/// forward-only universe of findings and build their records. Returns `Err`
/// naming the failed step on ANY subprocess or parse failure in the leg —
/// findings are a join that can be absent without invalidating the milestone
/// and session legs (REQ-8), so the caller keeps the snapshot `Acquired` —
/// but the failure is RECORDED (no finding-field group acquired, plus the
/// worded note), never erased into the shape of a clean tracker (vsdd-cli
/// #818 Fix 2; guardrail REQ-4). The bool in `Ok` is the truncation flag
/// (REQ-4 of the finding-query join).
fn acquire_findings(repo_root: &Path) -> Result<(Vec<FindingRecord>, bool), &'static str> {
    // (1) review-round issue ids — crosslink applies the finding-discrimination
    // predicate server-side via the `review` label.
    let review_json = match run_bounded(
        "crosslink",
        &["issue", "list", "--label", "review", "-s", "all", "--json"],
        repo_root,
    ) {
        Subprocess::Completed { stdout } => stdout,
        _ => return Err("the review-round list query failed"),
    };
    let review_ids: Vec<u64> = parse_issue_list(&review_json)
        .ok_or("the review-round list output did not parse")?
        .iter()
        .map(|it| it.id)
        .collect();

    // (2) every issue's parent_id / status / closed_at — one bulk list.
    let all_json = match run_bounded("crosslink", &["issue", "list", "-s", "all", "--json"], repo_root)
    {
        Subprocess::Completed { stdout } => stdout,
        _ => return Err("the all-issues list query failed"),
    };
    let all_issues = parse_issue_list(&all_json).ok_or("the all-issues list output did not parse")?;

    // (3) the forward-only universe (children of review rounds, not closed before
    // the boundary), capped.
    let (universe, truncated) = findings_in_universe(&review_ids, &all_issues);

    // (4) per-finding show for labels (disposition) and comments (routing).
    let mut findings = Vec::with_capacity(universe.len());
    for item in universe {
        let id_str = item.id.to_string();
        let detail_json =
            match run_bounded("crosslink", &["issue", "show", &id_str, "--json"], repo_root) {
                Subprocess::Completed { stdout } => stdout,
                _ => return Err("a per-finding show query failed"),
            };
        let detail =
            parse_issue_detail(&detail_json).ok_or("a per-finding show output did not parse")?;
        findings.push(finding_record(item, &detail));
    }
    Ok((findings, truncated))
}

#[cfg(test)]
mod tests {
    use super::{
        closed_before_ratification, disposition_from_labels, finding_record, findings_in_universe,
        is_finding, parse_milestones, routing_present, split_count_suffix, IssueComment, IssueDetail,
        IssueListItem, ABSENT_MILESTONE, ABSENT_WORK_ITEM, FINDING_QUERY_CAP,
    };

    #[test]
    fn the_absence_wordings_mirror_the_registered_set() {
        // The declared-mirror fidelity pin (vsdd-cli #791): the
        // acquisition's bootstrap absence words match the statusline
        // data set's absence_text, so an edit to one cannot silently
        // diverge the render layer from the acquisition.
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let data: crate::registry::sets::StatuslineData =
            crate::registry::load_set(&root, "statusline-data").expect("statusline data loads");
        let registered = |field: &str| {
            data.display_fields
                .iter()
                .find(|f| f.field == field)
                .map(|f| f.absence_text.clone())
                .unwrap_or_default()
        };
        assert_eq!(registered("work-item"), ABSENT_WORK_ITEM);
        assert_eq!(registered("milestone-with-count"), ABSENT_MILESTONE);
    }

    #[test]
    fn content_with_zero_parses_is_none_never_an_empty_success() {
        // The #748 fail-loud discipline: whole-format drift reads as
        // unusable, not as a repo with no milestones.
        assert!(parse_milestones("some drifted header\nno list lines here\n").is_none());
    }

    #[test]
    fn empty_output_is_an_empty_list() {
        let parsed = parse_milestones("\n  \n").expect("no content lines is a lawful empty");
        assert!(parsed.is_empty());
    }

    #[test]
    fn crosslink_empty_state_message_is_an_empty_list() {
        // vsdd-cli #829: crosslink prints "No milestones found." for a repo with
        // no milestones; that is a lawful empty list, not the whole-format drift
        // the fail-loud check guards against. A genuine drift line still parses
        // to None (the sibling test above).
        let parsed =
            parse_milestones("No milestones found.\n").expect("the empty-state message is empty");
        assert!(parsed.is_empty());
    }

    #[test]
    fn open_and_closed_lines_parse_with_counts() {
        let text = "#4   [\u{2713}] layer 1 (3/3)\n#5   [ ] layer 2 (1/4)\n";
        let parsed = parse_milestones(text).expect("the list shape parses");
        assert_eq!(parsed.len(), 2);
        assert!(!parsed[0].state.is_active, "closed reads inactive");
        assert!(parsed[1].state.is_active, "open reads active");
        assert_eq!(parsed[1].counts, Some((1, 4)));
        assert_eq!(parsed[1].state.name, "layer 2");
    }

    #[test]
    fn count_suffix_strips_only_the_exact_digit_shape() {
        assert_eq!(
            split_count_suffix("layer 2 (1/4)"),
            ("layer 2", Some((1, 4)))
        );
        // A name legitimately ending in a parenthetical survives whole.
        assert_eq!(
            split_count_suffix("release (v1/2)"),
            ("release (v1/2)", None)
        );
        assert_eq!(split_count_suffix("survey (2024)"), ("survey (2024)", None));
        assert_eq!(split_count_suffix("plain name"), ("plain name", None));
    }

    // ── Tracker-join mappers (Slice 1 red gate, vsdd-cli #820) ──────────────
    // Failing-executed against the stub bodies above; the operator adopts these
    // as the red-gate oracle before 2b implements the mappers to green.

    #[test]
    fn a_review_labelled_parent_marks_its_children_findings() {
        // The bootstrap discrimination rule (#820): a finding is an issue whose
        // parent is a review-round issue (the parent carries the `review` label).
        assert!(
            is_finding(&["review".to_string()]),
            "a review-labelled parent marks a finding"
        );
        assert!(
            is_finding(&["review".to_string(), "high".to_string()]),
            "review among the parent's labels still marks a finding"
        );
        assert!(
            !is_finding(&["feature".to_string()]),
            "a non-review parent is not a finding"
        );
        assert!(!is_finding(&[]), "no parent labels is not a finding");
    }

    #[test]
    fn a_plan_kind_comment_is_a_routing_edge() {
        // The bootstrap routing format-carry (#810): routing presence is a
        // `plan`-kind comment on the finding.
        assert!(
            routing_present(&["plan".to_string()]),
            "a plan-kind comment is a routing edge"
        );
        assert!(
            routing_present(&["result".to_string(), "plan".to_string()]),
            "plan among the comment kinds reads as routed"
        );
        assert!(
            !routing_present(&["result".to_string()]),
            "a finding with only non-plan comments is unrouted"
        );
        assert!(!routing_present(&[]), "a finding with no comments is unrouted");
    }

    #[test]
    fn a_finding_closed_before_the_boundary_is_outside_the_universe() {
        // The forward-only universe boundary (REQ-5; #811): a finding closed
        // strictly before the ratification boundary is excluded; open, or closed
        // at/after the boundary, stays in. ISO-8601 Zulu sorts lexicographically.
        let boundary = "2026-07-27T00:00:00Z";
        assert!(
            closed_before_ratification(Some("2026-07-20T00:00:00Z"), boundary),
            "closed before the boundary is outside the universe"
        );
        assert!(
            !closed_before_ratification(Some("2026-07-28T00:00:00Z"), boundary),
            "closed after the boundary stays in the universe"
        );
        assert!(
            !closed_before_ratification(Some(boundary), boundary),
            "closed exactly at the boundary stays in (forward-only is inclusive)"
        );
        assert!(
            !closed_before_ratification(None, boundary),
            "an open finding is in the universe"
        );
    }

    // ── Finding-query join pure seam (Slice 1, vsdd-cli #820) ────────────────

    fn list_item(
        id: u64,
        parent_id: Option<u64>,
        status: &str,
        closed_at: Option<&str>,
    ) -> IssueListItem {
        IssueListItem {
            id,
            parent_id,
            status: status.to_string(),
            closed_at: closed_at.map(|s| s.to_string()),
        }
    }

    fn detail(labels: &[&str], comment_kinds: &[&str]) -> IssueDetail {
        IssueDetail {
            labels: labels.iter().map(|s| s.to_string()).collect(),
            comments: comment_kinds
                .iter()
                .map(|k| IssueComment {
                    kind: k.to_string(),
                })
                .collect(),
        }
    }

    #[test]
    fn disposition_reads_the_carry_labels() {
        // AC-10: each disposition label maps; a non-disposition label is None.
        for label in ["dismissed", "hallucinated", "consolidated"] {
            assert_eq!(
                disposition_from_labels(&[label.to_string()]).as_deref(),
                Some(label)
            );
        }
        assert_eq!(
            disposition_from_labels(&["security".to_string(), "consolidated".to_string()])
                .as_deref(),
            Some("consolidated"),
            "a disposition label among others is found"
        );
        assert_eq!(disposition_from_labels(&["security".to_string()]), None);
        assert_eq!(disposition_from_labels(&[]), None);
    }

    #[test]
    fn the_universe_is_review_children_at_or_after_the_boundary() {
        let review_ids = [100u64];
        let all = vec![
            list_item(1, Some(100), "open", None), // review child, open -> in
            list_item(2, Some(100), "closed", Some("2026-07-29T00:00:00Z")), // closed after -> in
            list_item(3, Some(100), "closed", Some("2026-07-20T00:00:00Z")), // closed before -> out
            list_item(4, Some(999), "open", None), // parent not a review round -> out
            list_item(5, None, "open", None),      // no parent -> out
        ];
        let (universe, truncated) = findings_in_universe(&review_ids, &all);
        let ids: Vec<u64> = universe.iter().map(|it| it.id).collect();
        assert_eq!(ids, vec![1, 2], "only in-universe review children, id-ordered");
        assert!(!truncated);
    }

    #[test]
    fn the_universe_caps_and_reports_truncation() {
        // REQ-4: exceeding the cap truncates AND reports it — never a silent drop.
        let review_ids = [100u64];
        let all: Vec<IssueListItem> = (1..=(FINDING_QUERY_CAP as u64 + 5))
            .map(|id| list_item(id, Some(100), "open", None))
            .collect();
        let (universe, truncated) = findings_in_universe(&review_ids, &all);
        assert_eq!(universe.len(), FINDING_QUERY_CAP, "capped at the bound");
        assert!(truncated, "truncation reported");
    }

    #[test]
    fn a_finding_record_maps_routing_disposition_and_universe() {
        // AC-9: a routed dismissed close reads its disposition + routing and sits
        // in the universe; the same item unrouted has neither. closed_at carries
        // fractional seconds (the real crosslink shape) against the date boundary.
        let closed_after = list_item(7, Some(100), "closed", Some("2026-07-28T09:00:00.123456Z"));
        let routed_dismissed = finding_record(&closed_after, &detail(&["dismissed"], &["plan", "result"]));
        assert_eq!(routed_dismissed.handle, "#7");
        assert_eq!(routed_dismissed.status, "closed");
        assert_eq!(routed_dismissed.disposition.as_deref(), Some("dismissed"));
        assert!(routed_dismissed.routing_present, "a plan comment is a routing edge");
        assert!(
            !routed_dismissed.closed_before_ratification,
            "closed after the boundary is in-universe (fractional-second timestamp)"
        );
        assert!(
            routed_dismissed.owner.is_none() && routed_dismissed.validator.is_none(),
            "spine-only: owner/validator deferred to Slice 5"
        );

        let unrouted = finding_record(&closed_after, &detail(&[], &["result"]));
        assert!(unrouted.disposition.is_none(), "no disposition label -> None");
        assert!(!unrouted.routing_present, "only a result comment is unrouted");
    }

    #[test]
    #[ignore = "needs a live crosslink tracker; run with --ignored --nocapture"]
    fn live_finding_walk_in_isolation() {
        // The runnable form of the manual-tests/layer-2.md finding-join check
        // (vsdd-cli #820). Exercises the effectful walk directly, bypassing the
        // milestone/session legs — the milestone leg is currently Unusable on a
        // repo with no crosslink milestones (a pre-existing parse gap tracked
        // separately), so acquire_snapshot short-circuits before the join.
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        match super::acquire_findings(repo) {
            Ok((findings, truncated)) => {
                eprintln!("findings: {} (truncated={truncated})", findings.len());
                for f in &findings {
                    eprintln!(
                        "  {} status={} routing={} closed_before_ratification={} disposition={:?}",
                        f.handle,
                        f.status,
                        f.routing_present,
                        f.closed_before_ratification,
                        f.disposition
                    );
                }
            }
            Err(step) => eprintln!("finding leg failed ({step}); findings absent"),
        }
    }
}
