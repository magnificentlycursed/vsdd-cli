//! The rustc-shaped diagnostic every Layer 1 failure path emits.
//!
//! Failure kinds and recovery members are loaded from the statusline data
//! set (`templates/registry/statusline-data.md`) — the kind-to-action
//! mapping is versioned data, never a hardcoded copy (contract: the Status
//! requirement; the trust-boundary rule: a read failure yields a
//! diagnostic, never a panic).

use std::path::PathBuf;

use crate::registry::sets::StatuslineData;

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
        let _ = (kind, file, detail, location, vocabulary);
        todo!("2b: construct from the loaded kind-to-action mapping")
    }

    /// The human form: rustc-shaped, complete sentences, information
    /// carried by text alone (the color-channel conduct).
    pub fn render_human(&self) -> String {
        todo!("2b: rustc-shaped rendering")
    }

    /// The machine form: structured JSON carrying kind, machine token,
    /// diagnostic payload, and recovery action for mechanical branching.
    pub fn render_machine(&self) -> serde_json::Value {
        todo!("2b: structured rendering")
    }
}
