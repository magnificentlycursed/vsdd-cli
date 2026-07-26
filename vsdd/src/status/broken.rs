//! The shared broken-state composition (Layer 3): when the state
//! artifact cannot be read, all three surfaces still speak — the
//! segment renders the registered broken-state mark, the human form
//! the rustc-shaped diagnostic (its grammar aligned to mdatron's
//! rendering: error[token], the arrow line, `=` detail lines) with the
//! kind's recovery, the machine form the structured state-unreadable
//! signal. Carries the last boundary truth from git when one exists
//! (vsdd-cli #740) and a worded absence when none does.

use serde_json::json;

use vsdd_core::diagnostics::Diagnostic;
use vsdd_core::registry::sets::StatuslineData;

/// The worded absence for a history with no boundary commit yet.
const NO_BOUNDARY: &str = "no boundary commit recorded";

/// The three surfaces' broken-state outputs.
pub struct BrokenSurfaces {
    pub segment: String,
    pub human: String,
    pub machine: serde_json::Value,
}

/// Compose the broken-state outputs. Pure over the diagnostic, the
/// data set, and the (already-acquired) last-boundary subject.
pub fn compose_broken_state(
    diagnostic: &Diagnostic,
    data: &StatuslineData,
    last_boundary: Option<&str>,
) -> BrokenSurfaces {
    let registered = data
        .read_failure_kinds
        .iter()
        .find(|k| k.machine_token == diagnostic.machine_token);
    let boundary = last_boundary.unwrap_or(NO_BOUNDARY);

    let mut human = format!(
        "error[{}]: {}\n",
        diagnostic.machine_token, diagnostic.message
    );
    match diagnostic.location {
        Some((line, column)) => human.push_str(&format!(
            " --> {} (line {line}, column {column})\n",
            diagnostic.file.display()
        )),
        None => human.push_str(&format!(" --> {}\n", diagnostic.file.display())),
    }
    human.push_str(&format!("  = kind: {}\n", diagnostic.kind));
    match registered {
        Some(k) => human.push_str(&format!("  = recovery: {}\n", k.human_recovery)),
        None => human.push_str(
            "  = recovery: the read-failure kind is unregistered — repair the statusline data set\n",
        ),
    }
    human.push_str(&format!("  = last boundary: {boundary}\n"));

    let machine = json!({
        "state_unreadable": {
            "kind": diagnostic.machine_token,
            "diagnostic": {
                "file": diagnostic.file.display().to_string(),
                "message": diagnostic.message,
                "location": diagnostic.location.map(|(l, c)| json!({"line": l, "column": c})),
            },
            "recovery_action": registered.map(|k| k.recovery_action.as_str()).unwrap_or(""),
            "recovery_text": registered.map(|k| k.human_recovery.as_str()).unwrap_or(""),
        },
        "last_boundary": boundary,
    });

    BrokenSurfaces {
        segment: data.broken_state_mark.clone(),
        human,
        machine,
    }
}
