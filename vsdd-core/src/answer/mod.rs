//! The structured phase answer (contract: Deterministic phase answer;
//! the Convergence test compares every field by exact match, integrity
//! findings at the kind-set grain).

pub mod derive;
pub mod integrity;

use serde::{Deserialize, Serialize};

use crate::state::ActiveComposition;

/// The one computed answer behind all three Status renderings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhaseAnswer {
    /// The scope member, or None pre-entry (the pinned null meaning).
    pub phase: Option<String>,
    pub layer: Option<u32>,
    /// A member of the action vocabulary; the fix-lane workflow tokens
    /// are outside this output domain (operator ruling, vsdd-cli #689).
    pub next_action: String,
    pub active_composition: ActiveComposition,
    /// The degraded kind — present exactly when corroboration is absent
    /// or unusable, never a bare flag (tracker-absent | tracker-unusable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<String>,
    /// Snapshot-scoped integrity finding kinds; the comparison grain is
    /// the kind-set.
    pub integrity_findings: Vec<String>,
}
