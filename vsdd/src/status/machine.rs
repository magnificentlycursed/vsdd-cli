//! The pure machine form (Layer 3): named blocks per the adopted
//! facets note — `answer` (position) and `report` (health) — a
//! verified superset of every segment field; agents branch on
//! enumerated members, never prose.

use serde_json::json;

use vsdd_core::answer::PhaseAnswer;
use vsdd_core::registry::sets::StatuslineData;
use vsdd_core::snapshot::Snapshot;

/// Render the machine form. Pure.
pub fn render_machine(
    answer: &PhaseAnswer,
    snapshot: &Snapshot,
    data: &StatuslineData,
) -> serde_json::Value {
    let degraded = answer.degraded.as_ref().map(|kind| {
        let next_step = super::degraded_next_step(data, kind).unwrap_or_default();
        json!({ "kind": kind, "next_step": next_step })
    });
    json!({
        "vsdd_status_version": "0.1.0",
        "answer": {
            "phase": answer.phase,
            "layer": answer.layer,
            "next_action": answer.next_action,
            "active_composition": serde_json::to_value(&answer.active_composition)
                .unwrap_or(serde_json::Value::Null),
            "display": {
                "repo": snapshot.display_repo_name,
                "session": snapshot.display_session,
                "work_item": snapshot.display_work_item,
                "milestone": snapshot.display_active_milestone,
            },
        },
        "report": {
            "degraded": degraded,
            "integrity_findings": answer.integrity_findings,
        },
    })
}
