//! The rustc-shaped diagnostic every Layer 1 failure path emits.
//!
//! Failure kinds and recovery members are loaded from the statusline data
//! set (`templates/registry/statusline-data.md`) — the kind-to-action
//! mapping is versioned data, never a hardcoded copy (contract: the Status
//! requirement; the trust-boundary rule: a read failure yields a
//! diagnostic, never a panic).

use std::path::PathBuf;

use crate::registry::sets::StatuslineData;

/// Map a YAML parse error to a 1-indexed (line, column), shifting by
/// `line_offset` when the parsed slice does not begin at the file's
/// first line (frontmatter begins on line 2).
pub(crate) fn yaml_location(
    error: &serde_yaml_ng::Error,
    line_offset: usize,
) -> Option<(usize, usize)> {
    error
        .location()
        .map(|l| (l.line() + line_offset, l.column()))
}

/// The three state read-failure kinds, mirroring the statusline data
/// set's `read_failure_kinds` enumeration exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateReadKind {
    Absent,
    Malformed,
    PermissionOrIo,
}

impl StateReadKind {
    /// The data set's kind id for this variant.
    pub fn kind_id(self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::Malformed => "malformed",
            Self::PermissionOrIo => "permission-or-io",
        }
    }
}

/// A rustc-shaped diagnostic: the file, the failure kind, the parse
/// location when the failure is a parse, the message, and the kind's
/// recovery members drawn from the loaded statusline set.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub file: PathBuf,
    pub kind: String,
    pub machine_token: String,
    /// (line, column), present exactly when the failure is a parse.
    pub location: Option<(usize, usize)>,
    pub message: String,
    pub recovery_action: String,
    pub recovery_text: String,
}

impl Diagnostic {
    /// Build the diagnostic for a state read failure; every token comes
    /// from the loaded vocabulary, never from a constant in this crate.
    pub fn state_read_failure(
        kind: StateReadKind,
        file: PathBuf,
        detail: String,
        location: Option<(usize, usize)>,
        vocabulary: &StatuslineData,
    ) -> Self {
        let entry = vocabulary
            .read_failure_kinds
            .iter()
            .find(|k| k.kind == kind.kind_id());
        match entry {
            Some(member) => Diagnostic {
                file,
                kind: member.kind.clone(),
                machine_token: member.machine_token.clone(),
                location,
                message: detail,
                recovery_action: member.recovery_action.clone(),
                recovery_text: member.human_recovery.clone(),
            },
            // The schema pair pins the set to exactly the three kinds, so
            // this arm is unreachable against a validated vocabulary; it
            // stays total rather than panicking at a trust boundary.
            None => Diagnostic {
                file,
                kind: kind.kind_id().to_string(),
                machine_token: kind.kind_id().to_string(),
                location,
                message: format!(
                    "{detail} (the loaded statusline set does not enumerate the `{}` kind)",
                    kind.kind_id()
                ),
                recovery_action: String::new(),
                recovery_text: String::new(),
            },
        }
    }

    /// The human form: rustc-shaped, complete sentences, information
    /// carried by text alone (the color-channel conduct).
    pub fn render_human(&self) -> String {
        let mut out = format!("error: {}\n", self.message);
        out.push_str(&format!("  --> {}", self.file.display()));
        if let Some((line, column)) = self.location {
            out.push_str(&format!(":{line}:{column}"));
        }
        out.push('\n');
        if !self.recovery_text.is_empty() {
            out.push_str(&format!("  = recovery: {}\n", self.recovery_text));
        }
        out
    }

    /// The machine form: structured JSON carrying kind, machine token,
    /// diagnostic payload, and recovery action for mechanical branching.
    pub fn render_machine(&self) -> serde_json::Value {
        serde_json::json!({
            "kind": self.kind,
            "machine_token": self.machine_token,
            "file": self.file.display().to_string(),
            "location": self.location.map(|(l, c)| serde_json::json!([l, c])),
            "message": self.message,
            "recovery_action": self.recovery_action,
            "recovery_text": self.recovery_text,
        })
    }
}

impl std::fmt::Display for Diagnostic {
    /// The human rendering, trimmed — so `?` interop, `{}` printing, and
    /// error-chain composition all speak the same rustc-shaped text
    /// (the init.rs thiserror idiom, extended; vsdd-cli #727).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.render_human().trim_end())
    }
}

impl std::error::Error for Diagnostic {}
