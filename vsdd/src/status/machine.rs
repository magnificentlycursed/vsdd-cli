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
        // The same fallback its sibling surfaces word — the machine
        // form never goes silent where they speak (vsdd-cli #779).
        let next_step =
            super::degraded_next_step(data, kind).unwrap_or(super::UNREGISTERED_DEGRADED_KIND);
        json!({ "kind": kind, "next_step": next_step })
    });
    let mut out = json!({
        // Bumped 0.1.0 -> 0.1.1 for the additive `gate_provenance` field
        // (#818 Fix 1): a SECURITY SIGNAL a consumer must honor before
        // treating close-phase / enter-next-phase as authoritative — an
        // "unverified-self-report" value means the driving gate was a
        // self-report from the agent-writable state, evidence unresolved.
        "vsdd_status_version": "0.1.1",
        "answer": {
            "phase": answer.phase,
            "layer": answer.layer,
            "next_action": answer.next_action,
            // Present when a state-sourced gate verdict drove next_action;
            // marks it unverified-self-report so an agent never reads a
            // self-authored gate record as verified advancement (#818 Fix 1).
            "gate_provenance": answer.gate_provenance,
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
    });
    // The whole-of-output machine-form pass (contract: Terminal output
    // safety, vsdd-cli #807): sanitize every string value and object key
    // in the built form, so a hostile code point in any field — the
    // state-sourced composition fields (config_inputs_hash among them,
    // #803), the tracker-sourced display fields, the integrity kinds —
    // cannot reach the agent surface regardless of the struct's shape.
    // The systematic sink retiring the field-by-field misses (#803/#805);
    // still pure (deterministic, no IO).
    vsdd_core::text::clean_json_strings(&mut out);
    out
}
