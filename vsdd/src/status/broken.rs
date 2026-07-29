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

/// Cap echoed external content (adopter file bytes quoted by a parse
/// error, git commit subjects) before it reaches an agent-consumed
/// surface (vsdd-cli #818, Red Team wider eval): an oversized verbatim
/// quote is a prompt-injection carrier, and terminal-output-safety strips
/// only the invisible-Unicode class, not visible prose. This bounds the
/// visible-prose volume; the terminal cleaner has already run, so the
/// count is over cleaned characters.
const QUOTED_MAX: usize = 512;

fn bound_quote(s: String) -> String {
    if s.chars().count() > QUOTED_MAX {
        let kept: String = s.chars().take(QUOTED_MAX).collect();
        format!("{kept}… (quote truncated)")
    } else {
        s
    }
}

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
    // Cleaned at composition too (contract: Terminal output safety, the
    // third sink; vsdd-cli #807): the boundary subject is source-cleaned
    // at acquisition today, but this pure function must not let a future
    // caller passing an un-source-cleaned subject leak the class to the
    // human form, which has no whole-of-output backstop (the machine form
    // does). Idempotent on the already-clean value.
    let boundary = bound_quote(clean_for_terminal(last_boundary.unwrap_or(NO_BOUNDARY)));

    // The diagnostic's message and kind carry state-sourced text (a
    // malformed state.yaml echoes its own bytes); they cross the
    // terminal boundary cleaned, the same rule the segment and human
    // forms hold (vsdd-cli #784). The machine token is a registered
    // enum member — no adopter content — but cleaning it too costs
    // nothing and keeps the rule uniform.
    let message = bound_quote(clean_for_terminal(&diagnostic.message));
    // The machine token is a registered read-failure kind (tool-authored,
    // no adopter content), but the rule is uniform — the human form cleans
    // it exactly as the machine form does (vsdd-cli #805: the human
    // error[token] rendered it raw while the machine form cleaned it).
    let machine_token = clean_for_terminal(&diagnostic.machine_token);
    let mut human = format!("error[{machine_token}]: {message}\n");
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

    // The machine form is a terminal surface too (it prints to stdout and
    // an agent consumes it). The broken-state composition is the third
    // cleaning sink (contract: Terminal output safety, vsdd-cli #807): the
    // read failed, so no source boundary fired over this payload. A
    // whole-of-output pass over the built value is the systematic
    // guarantee — every string and key cleaned regardless of shape — so a
    // field added later cannot escape (the field-by-field misses this
    // retires, #799/#805).
    let mut machine = json!({
        "state_unreadable": {
            "kind": machine_token,
            // The diagnostic quotes external content (adopter file bytes a
            // parse error echoes, git commit subjects): it is DATA for
            // display, never an instruction (vsdd-cli #818). The actionable
            // signal is the enumerated `kind` above; an agent must branch on
            // that and treat the quoted text as untrusted.
            "quoted_content_untrusted": true,
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
    vsdd_core::text::clean_json_strings(&mut machine);

    BrokenSurfaces {
        segment: data.broken_state_mark.clone(),
        human,
        machine,
    }
}
