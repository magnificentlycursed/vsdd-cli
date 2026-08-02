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
    /// Whether a routing `plan` comment was filed for this finding — the
    /// datum the unrouted-findings process-integrity query reads (contract:
    /// Status; vsdd-cli #810/#811). Bootstrap-empty like the rest of the
    /// tracker join: the acquisition populates it when the Layer 6
    /// finding-query lands; the convergence corpus supplies it for the pure
    /// check meanwhile.
    #[serde(default)]
    pub routing_present: bool,
    /// The forward-only universe boundary (contract: Status, REQ-5 of the
    /// re-sequence-enforcement-spine amendment; vsdd-cli #810): a finding
    /// closed BEFORE the routing amendment's ratification boundary sits
    /// outside the query's universe and is never flagged. It defaults
    /// `true` so records predating closure-time capture — the fixed-baseline
    /// fixtures among them — stay outside the universe; the acquisition sets
    /// it `false` for findings closed at or after the boundary (and for open
    /// findings, which are in the universe) when the Layer 6 join lands.
    #[serde(default = "closed_before_ratification_default")]
    pub closed_before_ratification: bool,
}

fn closed_before_ratification_default() -> bool {
    true
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

/// Which finding-field groups the acquisition populated (vsdd-cli #820, REQ-7 of
/// the `finding-query-join` knowledge page; design retired under #845). The finding-reading integrity checks
/// consult it so a spine-only live join (Slice 1) does not mis-fire the sibling
/// checks that read the field groups Slice 5 defers. It defaults to all-acquired
/// so pre-marker snapshots and the convergence fixtures declare full
/// acquisition; the live spine-only join sets [`FindingFieldsAcquired::SPINE_ONLY`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FindingFieldsAcquired {
    /// handle, status, disposition, routing_present, closed_before_ratification.
    pub spine: bool,
    /// owner, validator (Slice 5).
    pub lifecycle_roles: bool,
    /// evidence_reference_present (Slice 5).
    pub evidence: bool,
}

impl Default for FindingFieldsAcquired {
    fn default() -> Self {
        FindingFieldsAcquired {
            spine: true,
            lifecycle_roles: true,
            evidence: true,
        }
    }
}

impl FindingFieldsAcquired {
    /// The Slice-1 live join populates the spine only; the lifecycle-role and
    /// evidence groups acquire in Slice 5.
    pub const SPINE_ONLY: Self = FindingFieldsAcquired {
        spine: true,
        lifecycle_roles: false,
        evidence: false,
    };

    /// A FAILED finding-query leg: NO group acquired (vsdd-cli #818 Fix 2).
    /// Distinct from [`SPINE_ONLY`](Self::SPINE_ONLY) — a group the
    /// acquisition version defers by scope is dormant, not failed — and
    /// never bit-identical to a genuinely clean tracker, whose record claims
    /// the spine acquired with findings empty. The routing gate fails closed
    /// on the unacquired spine (guardrail REQ-4, knowledge page
    /// `routing-before-fix-guardrail`): an unverifiable gate never passes
    /// vacuously.
    pub const NONE: Self = FindingFieldsAcquired {
        spine: false,
        lifecycle_roles: false,
        evidence: false,
    };
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
    /// Which finding-field groups the acquisition populated (vsdd-cli #820); the
    /// finding-reading integrity checks gate on it. Defaults to all-acquired.
    #[serde(default)]
    pub finding_fields_acquired: FindingFieldsAcquired,
    /// A worded degradation note for the finding query: present when the query
    /// was capped this acquisition (findings past the cap were not examined;
    /// REQ-4 of the finding-query join, vsdd-cli #820) or when it FAILED with
    /// the tracker present (findings could not be acquired, the failed step
    /// named; vsdd-cli #818 Fix 2 — the machine marker is the unacquired
    /// spine in `finding_fields_acquired`, this note is its worded WHY). None
    /// when the query ran whole. Never a silent drop.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finding_acquisition_note: Option<String>,
}
