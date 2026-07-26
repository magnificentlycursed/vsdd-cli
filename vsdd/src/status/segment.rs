//! The pure segment renderer (Layer 3): one line, four fields in
//! order — repo name, phase answer, work item, milestone-with-count —
//! per-field width budgets with the worded truncation mark, worded
//! absences, the degraded marker word, no session field. Deterministic
//! byte-for-byte; color is optional decoration the renderer does not
//! emit — the meaning lives in the words (the color-channel conduct).

use vsdd_core::answer::PhaseAnswer;
use vsdd_core::registry::sets::StatuslineData;
use vsdd_core::snapshot::Snapshot;

/// Fields join with two spaces; a field's rendering never exceeds its
/// registered budget.
const FIELD_SEPARATOR: &str = "  ";

/// Render the one-line segment. Pure.
pub fn render_segment(answer: &PhaseAnswer, snapshot: &Snapshot, data: &StatuslineData) -> String {
    let phase_value = match (answer.phase.as_deref(), answer.layer) {
        (Some(p), Some(l)) => format!("{p} L{l}"),
        (Some(p), None) => p.to_string(),
        // The pinned null meaning: pre-entry renders the field's own
        // registered absence text below.
        (None, _) => String::new(),
    };
    let mut parts = vec![
        fit(data, "repo-name", &snapshot.display_repo_name),
        fit(data, "phase-answer", &phase_value),
        fit(data, "work-item", &snapshot.display_work_item),
        fit(
            data,
            "milestone-with-count",
            &snapshot.display_active_milestone,
        ),
    ];
    // The degraded marker renders exactly when the answer is degraded —
    // a segment that omits degradation is a lying gauge.
    if let Some(kind) = &answer.degraded {
        if let Some(registered) = data.degraded_kinds.iter().find(|k| &k.kind == kind) {
            parts.push(registered.marker_word.clone());
        } else {
            // An unregistered kind still marks — the plain fallback word
            // rather than silence; the vocabulary tests pin registration.
            parts.push("degraded".to_string());
        }
    }
    parts.retain(|p| !p.is_empty());
    parts.join(FIELD_SEPARATOR)
}

/// Fit a value to its field: worded absence when empty, worded
/// truncation when over budget — the mark set off by a space, never
/// glued, and the milestone's open count surviving name truncation
/// (the #680 ruling).
fn fit(data: &StatuslineData, field: &str, value: &str) -> String {
    let registered = data
        .display_fields
        .iter()
        .find(|f| f.field == field)
        .expect("segment fields are registered display fields");
    let budget = registered.width_budget_chars as usize;
    let value = if value.is_empty() {
        registered.absence_text.as_str()
    } else {
        value
    };
    if value.chars().count() <= budget {
        return value.to_string();
    }
    let mark = &data.truncation_mark;
    let mark_len = mark.chars().count();
    if field == "milestone-with-count" {
        if let Some((name, count)) = split_open_count(value) {
            let count_len = count.chars().count();
            let keep = budget.saturating_sub(mark_len + count_len + 2);
            return format!("{} {} {}", take_chars(name, keep), mark, count);
        }
    }
    let keep = budget.saturating_sub(mark_len + 1);
    format!("{} {}", take_chars(value, keep), mark)
}

/// Split a trailing `(N open)` gauge off the milestone display.
fn split_open_count(value: &str) -> Option<(&str, &str)> {
    let cut = value.rfind(" (")?;
    let count = &value[cut + 1..];
    if count.ends_with(" open)") && count.starts_with('(') {
        Some((value[..cut].trim_end(), count))
    } else {
        None
    }
}

fn take_chars(value: &str, keep: usize) -> String {
    value
        .chars()
        .take(keep)
        .collect::<String>()
        .trim_end()
        .to_string()
}
