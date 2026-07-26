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

use super::segment::clean_for_terminal;

/// The rendered state path folds the home prefix to `~` (vsdd-cli
/// #782): the diagnostic stays actionable while the account segment
/// stays out of captured output; the degenerate-HOME rule matches the
/// refs sanitizer's (vsdd-cli #767).
fn display_path(diagnostic: &Diagnostic) -> String {
    let raw = diagnostic.file.display().to_string();
    if let Some(home) = std::env::var_os("HOME") {
        let home = std::path::PathBuf::from(home).display().to_string();
        if home.trim().len() > 1 {
            if let Some(rest) = raw.strip_prefix(&home) {
                return format!("~{rest}");
            }
        }
    }
    raw
}

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

    // The diagnostic's message and kind carry state-sourced text (a
    // malformed state.yaml echoes its own bytes); they cross the
    // terminal boundary cleaned, the same rule the segment and human
    // forms hold (vsdd-cli #784). The machine token is a registered
    // enum member — no adopter content — but cleaning it too costs
    // nothing and keeps the rule uniform.
    let message = clean_for_terminal(&diagnostic.message);
    let mut human = format!("error[{}]: {message}\n", diagnostic.machine_token);
    let shown_path = clean_for_terminal(&display_path(diagnostic));
    match diagnostic.location {
        Some((line, column)) => human.push_str(&format!(
            " --> {shown_path} (line {line}, column {column})\n"
        )),
        None => human.push_str(&format!(" --> {shown_path}\n")),
    }
    human.push_str(&format!(
        "  = kind: {}\n",
        clean_for_terminal(&diagnostic.kind)
    ));
    match registered {
        // human_recovery is a registry string, pre-cleaned at load
        // (vsdd-cli #794); the dynamic message/kind/path clean here.
        Some(k) => human.push_str(&format!("  = recovery: {}\n", k.human_recovery)),
        None => human.push_str(
            "  = recovery: the read-failure kind is unregistered — repair the statusline data set\n",
        ),
    }
    human.push_str(&format!("  = last boundary: {boundary}\n"));

    // The machine form is a terminal surface too (it prints to stdout
    // and an agent consumes it): the same cleaning the human form gets
    // (vsdd-cli #799). serde escapes controls but not the bidi/tag
    // class, so the dynamic strings reuse the cleaned values, and the
    // registered strings are already clean from the loader's PostLoad.
    let machine = json!({
        "state_unreadable": {
            "kind": clean_for_terminal(&diagnostic.machine_token),
            "diagnostic": {
                "file": shown_path,
                "message": message,
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
