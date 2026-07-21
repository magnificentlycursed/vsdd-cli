//! The corroboration snapshot: an explicitly-acquired materialized view
//! (contract: Verification architecture — the phase-answer derivation is
//! pure over it and never acquires). The struct mirrors
//! `templates/registry/snapshot-schema.md`'s `snapshot_fields` verbatim.

pub mod acquire;

use serde::{Deserialize, Serialize};

/// The corroboration condition the degraded-kind derivation branches on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AcquisitionOutcome {
    Acquired,
    Absent,
    Unusable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MilestoneState {
    pub name: String,
    pub state: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FindingRecord {
    pub handle: String,
    pub status: String,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub validator: Option<String>,
    pub evidence_reference_present: bool,
    #[serde(default)]
    pub disposition: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoundManifest {
    pub handle: String,
    pub declared_finding_count: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoundChildren {
    pub handle: String,
    pub child_count: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommentHandle {
    pub handle: String,
    pub resolves: bool,
}

/// One acquisition, one derivation, one rendering — never a second
/// computation (the Status requirement).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub acquisition_outcome: AcquisitionOutcome,
    pub milestones: Vec<MilestoneState>,
    pub findings: Vec<FindingRecord>,
    pub round_manifests: Vec<RoundManifest>,
    pub round_children: Vec<RoundChildren>,
    pub comment_handles: Vec<CommentHandle>,
    pub display_repo_name: String,
    pub display_session: String,
    pub display_work_item: String,
    pub display_active_milestone: String,
}
