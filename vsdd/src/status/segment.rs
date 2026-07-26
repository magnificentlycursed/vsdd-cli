//! The pure segment renderer (Layer 3): one line, four fields in
//! order — repo name, phase answer, work item, milestone-with-count —
//! per-field width budgets with the worded truncation mark, worded
//! absences, the degraded marker word, no session field. Deterministic
//! byte-for-byte; color is optional decoration the renderer does not
//! emit — the meaning lives in the words (the color-channel conduct).
//!
//! Round-1 hardening (vsdd-cli #777, #780): every value is cleaned of
//! control characters at this terminal boundary — state-sourced
//! strings get the same discipline the acquisition gives
//! crosslink-sourced ones; protected tails (the milestone's open
//! count, the phase field's layer suffix) survive truncation while
//! names yield (the #680 order, generalized); the budget invariant
//! holds even for degenerate registered budgets — a budget below the
//! mark's own width renders a bare hard cut rather than overflowing.

use vsdd_core::answer::PhaseAnswer;
use vsdd_core::registry::sets::StatuslineData;
use vsdd_core::snapshot::Snapshot;

/// Fields join with two spaces; a field's rendering never exceeds its
/// registered budget.
const FIELD_SEPARATOR: &str = "  ";

/// Render the one-line segment. Pure.
pub fn render_segment(answer: &PhaseAnswer, snapshot: &Snapshot, data: &StatuslineData) -> String {
    let (phase_name, layer_tail) = match (answer.phase.as_deref(), answer.layer) {
        (Some(p), Some(l)) => (p.to_string(), Some(format!("L{l}"))),
        (Some(p), None) => (p.to_string(), None),
        // The pinned null meaning: pre-entry renders the field's own
        // registered absence text below.
        (None, _) => (String::new(), None),
    };
    let mut parts = vec![
        fit(data, "repo-name", &snapshot.display_repo_name, None),
        fit(data, "phase-answer", &phase_name, layer_tail.as_deref()),
        fit(data, "work-item", &snapshot.display_work_item, None),
        fit_milestone(data, &snapshot.display_active_milestone),
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

/// The milestone field: its `(N open)` gauge is the protected tail
/// (the #680 ruling) — but only the REAL digit-exact gauge; a name
/// legitimately ending in a parenthetical stays a name (vsdd-cli
/// #780, mirroring the acquisition's digit-exact strip).
fn fit_milestone(data: &StatuslineData, value: &str) -> String {
    match split_open_count(value) {
        Some((name, count)) => fit(data, "milestone-with-count", name, Some(count)),
        None => fit(data, "milestone-with-count", value, None),
    }
}

/// Fit a value with an optional protected tail (the layer suffix, the
/// open count): worded absence when empty; on truncation the name
/// yields and the tail survives, with the mark set off by spaces,
/// never glued.
fn fit(data: &StatuslineData, field: &str, name: &str, tail: Option<&str>) -> String {
    let registered = data
        .display_fields
        .iter()
        .find(|f| f.field == field)
        .expect("segment fields are registered display fields");
    let budget = registered.width_budget_chars as usize;

    // Terminal boundary: control characters never render (vsdd-cli
    // #777) — state-sourced values get the same cleaning the
    // acquisition gives crosslink-sourced ones.
    let name = clean_for_terminal(name);
    let name = if name.is_empty() {
        registered.absence_text.clone()
    } else {
        name
    };
    let tail = tail.map(clean_for_terminal).filter(|t| !t.is_empty());

    let full = match &tail {
        Some(t) => format!("{name} {t}"),
        None => name.clone(),
    };
    if full.chars().count() <= budget {
        return full;
    }

    let mark = &data.truncation_mark;
    let mark_len = mark.chars().count();
    if let Some(t) = &tail {
        let tail_len = t.chars().count();
        let keep = budget.saturating_sub(mark_len + tail_len + 2);
        if keep > 0 {
            // The name yields; the tail survives (the #680 order).
            return format!("{} {} {}", take_chars(&name, keep), mark, t);
        }
        // No room for any name: the tail and mark alone, hard-cut to
        // the budget so the invariant holds (vsdd-cli #780).
        return take_chars(&format!("{mark} {t}"), budget);
    }
    let keep = budget.saturating_sub(mark_len + 1);
    if keep > 0 {
        return format!("{} {}", take_chars(&name, keep), mark);
    }
    // A degenerate budget below the mark's own width: a bare hard cut,
    // never an over-budget rendering (vsdd-cli #780).
    take_chars(&name, budget)
}

/// Split a trailing `(N open)` gauge off the milestone display — the
/// exact digit shape only (vsdd-cli #780).
fn split_open_count(value: &str) -> Option<(&str, &str)> {
    let cut = value.rfind(" (")?;
    let count = &value[cut + 1..];
    let digits = count.strip_prefix('(')?.strip_suffix(" open)")?;
    if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
        Some((value[..cut].trim_end(), count))
    } else {
        None
    }
}

/// Terminal-destined strings drop control characters at the boundary —
/// the render-side mirror of the acquisition's cleaning (vsdd-cli
/// #777): state-sourced and git-sourced text is data, never terminal
/// input.
pub fn clean_for_terminal(s: &str) -> String {
    s.chars().filter(|c| !c.is_control()).collect()
}

fn take_chars(value: &str, keep: usize) -> String {
    value
        .chars()
        .take(keep)
        .collect::<String>()
        .trim_end()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::split_open_count;

    #[test]
    fn the_gauge_split_is_digit_exact() {
        assert_eq!(
            split_open_count("layer 3 (5 open)"),
            Some(("layer 3", "(5 open)"))
        );
        // A countless milestone whose NAME ends in ` open)` stays a
        // name — the #680 protection covers only the real gauge.
        assert_eq!(split_open_count("hardening the gateway (kept open)"), None);
        assert_eq!(split_open_count("review (many open)"), None);
        assert_eq!(split_open_count("plain name"), None);
    }
}
