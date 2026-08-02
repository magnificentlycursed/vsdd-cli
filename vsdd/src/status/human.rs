//! The pure human form (Layer 3): two sections per the adopted facets
//! note (the design note on vsdd-cli #738) — the answer (position) and
//! the report (health, with the degraded kind's full next-step text) —
//! a verified superset of every segment field plus the session
//! rendering (the demotion ruling, vsdd-cli #679). Color-independent:
//! the renderer emits plain words; decoration, if any ever arrives,
//! must strip losslessly.

use vsdd_core::answer::{CheckNotRunReason, GateProvenance, PhaseAnswer};
use vsdd_core::registry::sets::StatuslineData;
use vsdd_core::snapshot::Snapshot;

use super::segment::clean_for_terminal;

/// A display value or its worded absence — the human form never prints
/// an empty slot (the criterion's own words; vsdd-cli #779), and every
/// value crosses the terminal boundary cleaned (vsdd-cli #777).
fn worded(value: &str, absence: &str) -> String {
    let cleaned = clean_for_terminal(value);
    if cleaned.is_empty() {
        absence.to_string()
    } else {
        cleaned
    }
}

/// The registered absence text for a display field — the human form
/// shares the segment's source rather than hardcoding a parallel that
/// could silently drift (vsdd-cli #786).
fn registered_absence<'a>(data: &'a StatuslineData, field: &str) -> &'a str {
    data.display_fields
        .iter()
        .find(|f| f.field == field)
        .map(|f| f.absence_text.as_str())
        // The same loud posture the segment's fit() takes (vsdd-cli
        // #791): a missing registered field is a drifted registry, not
        // a silent empty slot — and the schema pair requires all four,
        // so this is a belt-and-braces guard, never a live path.
        .expect("the display field is registered")
}

/// Render the human terminal form. Pure.
pub fn render_human(answer: &PhaseAnswer, snapshot: &Snapshot, data: &StatuslineData) -> String {
    let mut out = String::new();
    out.push_str("answer\n");
    out.push_str(&format!(
        "  repo: {}\n",
        worded(&snapshot.display_repo_name, "unnamed repo")
    ));
    match (answer.phase.as_deref(), answer.layer) {
        (Some(p), Some(l)) => {
            out.push_str(&format!("  phase: {} (layer {l})\n", clean_for_terminal(p)))
        }
        (Some(p), None) => out.push_str(&format!("  phase: {}\n", clean_for_terminal(p))),
        (None, _) => out.push_str("  phase: not entered\n"),
    }
    out.push_str(&format!("  next action: {}\n", answer.next_action));
    // A next action driven by a state-sourced gate verdict names that
    // verdict's provenance: it is a self-report from the agent-writable state
    // artifact, evidence unresolved to a boundary — never verified
    // advancement (vsdd-cli #818 Fix 1).
    // Exhaustive match, not `is_some` / `if let` / `Option::map` (#818 Fix 1
    // revise): a future variant — a verified provenance from a mechanized
    // gate (#815) — must force a new arm HERE (the compile error is the
    // point) rather than silently render as "unverified self-report" and
    // diverge from the machine form that serializes the real value. Every
    // idiomatic rewrite the lints suggest drops that exhaustiveness, so the
    // allow is deliberate.
    #[allow(clippy::single_match)]
    match &answer.gate_provenance {
        Some(GateProvenance::UnverifiedSelfReport) => {
            out.push_str(
                "    (gate: unverified self-report — evidence not resolved to a boundary)\n",
            );
        }
        None => {}
    }
    // The mode renders through its registered serde name — the enum
    // carries no display of its own by design.
    let mode = serde_json::to_value(answer.active_composition.mode)
        .ok()
        .and_then(|v| v.as_str().map(str::to_string))
        .unwrap_or_default();
    // The composition fields cross the same terminal boundary every
    // other value does (vsdd-cli #784): scope and each domain are
    // state-sourced text, cleaned before interpolation.
    let scope = clean_for_terminal(&answer.active_composition.scope);
    let domains = answer
        .active_composition
        .domains
        .iter()
        .map(|d| clean_for_terminal(d))
        .collect::<Vec<_>>()
        .join(", ");
    out.push_str(&format!("  composition: {scope} [{domains}] {mode}\n"));
    out.push_str(&format!(
        "  session: {}\n",
        worded(&snapshot.display_session, "no session")
    ));
    out.push_str(&format!(
        "  work item: {}\n",
        worded(
            &snapshot.display_work_item,
            registered_absence(data, "work-item")
        )
    ));
    out.push_str(&format!(
        "  milestone: {}\n",
        worded(
            &snapshot.display_active_milestone,
            registered_absence(data, "milestone-with-count")
        )
    ));

    out.push_str("report\n");
    match &answer.degraded {
        Some(kind) => {
            // The registry strings arrive pre-cleaned from the loader
            // (vsdd-cli #794); only the dynamic strings clean at render.
            let next_step =
                super::degraded_next_step(data, kind).unwrap_or(super::UNREGISTERED_DEGRADED_KIND);
            // The next-step text leads (it already words the
            // condition); the kind token trails for the machine-form
            // cross-reference — one statement, not three (vsdd-cli
            // #782).
            out.push_str(&format!(
                "  corroboration: degraded — {next_step} (kind: {kind})\n"
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
    // The dormant-vs-clean distinction (vsdd-cli #818 Fix 2): checks whose
    // inputs were not acquired are NAMED with why — their silence in the
    // findings list above is not checked-clean. Absent exactly when every
    // finding-reading check ran, mirroring the conditional provenance line.
    // Exhaustive match on the reason (the Fix-1 revise lesson): a future
    // variant must force a new worded arm here, never fall through to a
    // wording that diverges from the machine form's enumerated member.
    if !answer.checks_not_run.is_empty() {
        out.push_str("  checks not run (silence is not checked-clean):\n");
        for not_run in &answer.checks_not_run {
            let why = match not_run.reason {
                CheckNotRunReason::Dormant => {
                    "dormant — this acquisition does not read its inputs yet"
                }
                CheckNotRunReason::CouldNotCheck => {
                    "could not check — the finding query failed with the tracker present"
                }
            };
            out.push_str(&format!("    - {}: {why}\n", not_run.check));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::registered_absence;
    use vsdd_core::registry::sets::StatuslineData;

    fn data() -> StatuslineData {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        vsdd_core::registry::load_set(&root, "statusline-data").expect("statusline data loads")
    }

    #[test]
    fn registered_absence_is_loud_on_an_unregistered_field() {
        // The expect posture pinned (vsdd-cli #795): asking for a field
        // the registry does not carry panics rather than returning an
        // empty slot — the same loud failure fit() takes.
        let d = data();
        assert_eq!(registered_absence(&d, "work-item"), "no work item");
        // Return an owned bool from the closure, never the borrowed &str
        // (which would reference the captured data).
        let panicked = std::panic::catch_unwind(|| {
            let d = data();
            registered_absence(&d, "not-a-field").is_empty()
        });
        assert!(
            panicked.is_err(),
            "an unregistered field is loud, never an empty slot"
        );
    }
}
