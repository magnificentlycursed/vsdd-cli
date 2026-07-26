//! The pure human form (Layer 3): two sections per the adopted facets
//! note (the design note on vsdd-cli #738) — the answer (position) and
//! the report (health, with the degraded kind's full next-step text) —
//! a verified superset of every segment field plus the session
//! rendering (the demotion ruling, vsdd-cli #679). Color-independent:
//! the renderer emits plain words; decoration, if any ever arrives,
//! must strip losslessly.

use vsdd_core::answer::PhaseAnswer;
use vsdd_core::registry::sets::StatuslineData;
use vsdd_core::snapshot::Snapshot;

/// Render the human terminal form. Pure.
pub fn render_human(answer: &PhaseAnswer, snapshot: &Snapshot, data: &StatuslineData) -> String {
    let mut out = String::new();
    out.push_str("answer\n");
    out.push_str(&format!("  repo: {}\n", snapshot.display_repo_name));
    match (answer.phase.as_deref(), answer.layer) {
        (Some(p), Some(l)) => out.push_str(&format!("  phase: {p} (layer {l})\n")),
        (Some(p), None) => out.push_str(&format!("  phase: {p}\n")),
        (None, _) => out.push_str("  phase: not entered\n"),
    }
    out.push_str(&format!("  next action: {}\n", answer.next_action));
    // The mode renders through its registered serde name — the enum
    // carries no display of its own by design.
    let mode = serde_json::to_value(answer.active_composition.mode)
        .ok()
        .and_then(|v| v.as_str().map(str::to_string))
        .unwrap_or_default();
    out.push_str(&format!(
        "  composition: {} [{}] {mode}\n",
        answer.active_composition.scope,
        answer.active_composition.domains.join(", "),
    ));
    out.push_str(&format!("  session: {}\n", snapshot.display_session));
    out.push_str(&format!("  work item: {}\n", snapshot.display_work_item));
    out.push_str(&format!(
        "  milestone: {}\n",
        snapshot.display_active_milestone
    ));

    out.push_str("report\n");
    match &answer.degraded {
        Some(kind) => {
            let next_step = super::degraded_next_step(data, kind)
                .unwrap_or("unregistered degraded kind — repair the statusline data set");
            out.push_str(&format!(
                "  corroboration: degraded — {kind}: {next_step}\n"
            ));
        }
        None => out.push_str("  corroboration: acquired\n"),
    }
    if answer.integrity_findings.is_empty() {
        out.push_str("  integrity findings: none\n");
    } else {
        out.push_str("  integrity findings:\n");
        for kind in &answer.integrity_findings {
            out.push_str(&format!("    - {kind}\n"));
        }
    }
    out
}
