//! The effectful snapshot acquisition (the shell side of the purity
//! split): builds the [`Snapshot`](super::Snapshot) from the chassis's
//! query surface. One acquisition per invocation; the count-conduct
//! instruments join at Layer 3.
//!
//! Absent and unusable are OUTCOMES carried in the snapshot, never
//! errors: an unreachable chassis is the contracted normal offline
//! mode, and the derivation degrades with the kind rather than this
//! function failing.
//!
//! Bootstrap scope, declared (vsdd-cli #741): milestones ride the
//! chassis's list surface and the session fields its status JSON; the
//! tracker-join fields (findings, round manifests and children, comment
//! handles) acquire EMPTY until their query consumers land with the
//! parity and lifecycle gates (Layer 6) — the convergence corpus
//! supplies those fields for the pure checks meanwhile. The milestone
//! list is a human-format parse at bootstrap because the chassis
//! exposes no JSON for it; the parse failing is the unusable outcome,
//! never a guess.

use std::path::Path;
use std::process::Command;

use super::{AcquisitionOutcome, MilestoneState, Snapshot};

/// Acquire the corroboration snapshot for the repo.
pub fn acquire_snapshot(repo_root: &Path) -> Snapshot {
    let repo_name = repo_root
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "unnamed repo".to_string());

    if !repo_root.join(".crosslink").is_dir() {
        return empty(AcquisitionOutcome::Absent, repo_name);
    }

    let milestone_lines = match chassis(repo_root, &["milestone", "list"]) {
        ChassisResult::Ok(text) => text,
        ChassisResult::Unreachable => return empty(AcquisitionOutcome::Absent, repo_name),
        ChassisResult::Failed => return empty(AcquisitionOutcome::Unusable, repo_name),
    };
    let Some(milestones) = parse_milestones(&milestone_lines) else {
        return empty(AcquisitionOutcome::Unusable, repo_name);
    };

    let (session, work_item) = match chassis(repo_root, &["session", "status", "--json"]) {
        ChassisResult::Ok(text) => match parse_session(&text) {
            Some(pair) => pair,
            None => return empty(AcquisitionOutcome::Unusable, repo_name),
        },
        ChassisResult::Unreachable => return empty(AcquisitionOutcome::Absent, repo_name),
        // A missing session is a worded absence, not a failure.
        ChassisResult::Failed => ("no session".to_string(), "no work item".to_string()),
    };

    let active_display = milestones
        .iter()
        .filter(|m| m.is_active)
        .last()
        .map(|m| m.name.clone())
        .unwrap_or_else(|| "no milestone".to_string());

    Snapshot {
        acquisition_outcome: AcquisitionOutcome::Acquired,
        milestones,
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

enum ChassisResult {
    Ok(String),
    /// The binary is not runnable: the offline shape.
    Unreachable,
    /// The binary ran and refused: a store problem, not an absence.
    Failed,
}

fn chassis(repo_root: &Path, args: &[&str]) -> ChassisResult {
    match Command::new("crosslink")
        .current_dir(repo_root)
        .args(args)
        .output()
    {
        Err(_) => ChassisResult::Unreachable,
        Ok(out) if !out.status.success() => ChassisResult::Failed,
        Ok(out) => ChassisResult::Ok(String::from_utf8_lossy(&out.stdout).into_owned()),
    }
}

/// The list surface's line shape: `#5   [ ] name (1/2)` — `[✓]` closed.
/// Any non-empty line that does not parse makes the whole read unusable.
fn parse_milestones(text: &str) -> Option<Vec<MilestoneState>> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || !line.starts_with('#') {
            continue;
        }
        let open_bracket = line.find('[')?;
        let close_bracket = line.find(']')?;
        let closed = line.get(open_bracket + 1..close_bracket)? != " ";
        let rest = line.get(close_bracket + 1..)?.trim();
        let name = match rest.rfind(" (") {
            Some(cut) if rest.ends_with(')') => rest[..cut].trim(),
            _ => rest,
        };
        if name.is_empty() {
            return None;
        }
        out.push(MilestoneState {
            name: name.to_string(),
            state: if closed { "closed" } else { "open" }.to_string(),
            // Bootstrap: open reads as active; the chassis's own active
            // concept deepens this when it surfaces one.
            is_active: !closed,
        });
    }
    Some(out)
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
            Some(format!("{id} {title}"))
        })
        .unwrap_or_else(|| "no work item".to_string());
    Some((session, work_item))
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
