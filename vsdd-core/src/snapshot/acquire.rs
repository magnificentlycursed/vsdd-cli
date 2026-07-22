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
//! supplies those fields for the pure checks meanwhile. The milestone
//! list is a human-format parse (no JSON surface exists upstream);
//! output that fails the parse IS the unusable outcome, never a guess
//! (vsdd-cli #748). DECLARED CONFLATION (vsdd-cli #753): the session
//! status surface cannot distinguish no-active-session from a refusal
//! at bootstrap — a refused session query renders the worded absence;
//! the distinction lands when crosslink exposes it.

use std::path::Path;

use crate::subprocess::{run_bounded, Subprocess};

use super::{AcquisitionOutcome, MilestoneState, Snapshot};

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
            Subprocess::Refused { .. } => ("no session".to_string(), "no work item".to_string()),
            _ => return empty(AcquisitionOutcome::Unusable, repo_name),
        };

    let active_display = parsed
        .iter()
        .rev()
        .find(|p| p.state.is_active)
        .map(|p| match p.counts {
            // The schema's precomputed gauge: the open-finding count
            // scoped to the active milestone (vsdd-cli #748).
            Some((closed, total)) => {
                format!("{} ({} open)", p.state.name, total.saturating_sub(closed))
            }
            None => p.state.name.clone(),
        })
        .unwrap_or_else(|| "no milestone".to_string());

    Snapshot {
        acquisition_outcome: AcquisitionOutcome::Acquired,
        milestones: parsed.into_iter().map(|p| p.state).collect(),
        findings: Vec::new(),
        round_manifests: Vec::new(),
        round_children: Vec::new(),
        comment_handles: Vec::new(),
        display_repo_name: repo_name,
        display_session: session,
        display_work_item: work_item,
        display_active_milestone: active_display,
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
        .unwrap_or_else(|| "no session".to_string());
    let work_item = value
        .get("working_on")
        .and_then(|w| {
            let id = w.get("display_id")?.as_str()?;
            let title = w.get("title")?.as_str()?;
            Some(clean_for_display(&format!("{id} {title}")))
        })
        .unwrap_or_else(|| "no work item".to_string());
    Some((session, work_item))
}

/// Terminal-destined strings drop control characters at the boundary
/// (vsdd-cli #754): crosslink output is data, never terminal input.
fn clean_for_display(s: &str) -> String {
    s.chars().filter(|c| !c.is_control()).collect()
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
        display_session: "no session".to_string(),
        display_work_item: "no work item".to_string(),
        display_active_milestone: "no milestone".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_milestones, split_count_suffix};

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
}
