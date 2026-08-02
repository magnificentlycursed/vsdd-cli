//! The structured phase answer (contract: Deterministic phase answer;
//! the Convergence test compares every field by exact match, integrity
//! findings at the kind-set grain).

pub mod derive;
pub mod deviations;
pub mod integrity;

use serde::{Deserialize, Serialize};

use crate::state::ActiveComposition;

/// The provenance of a gate verdict that drove `next_action` (contract:
/// Trust boundaries; vsdd-cli #818 Fix 1). `read_state` validates only the
/// state artifact's shape, and a `GateResult`'s `evidence` is a free string
/// never resolved to a real boundary — so a gate verdict read from the
/// agent-writable state artifact is a SELF-REPORT, not a verified result.
/// The derivation and the machine/human forms mark it as such so a
/// self-authored `last_gate_result{result: pass}` is never presented as
/// verified advancement.
///
/// The sole variant emitted today is `UnverifiedSelfReport`. Its ABSENCE
/// on a `PhaseAnswer` means the action was not gate-driven (an authoring
/// step) — never "verified". The load-bearing invariant: the advancement
/// tokens `close-phase` / `enter-next-phase` are emitted ONLY with this
/// provenance present (see `derive::next_action`). Resolving a gate's
/// evidence to a real boundary — the mechanized gate that would earn a
/// verified provenance — is not built here; it belongs to the
/// enforcement-spine work (vsdd-cli #815).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GateProvenance {
    /// The driving gate result came from the state artifact with evidence
    /// unresolved to a boundary — trusted for the derivation's flow, but
    /// never asserted as a verified verdict.
    UnverifiedSelfReport,
}

/// Why a finding-reading integrity check did not run this derivation
/// (vsdd-cli #818 Fix 2 — the dormant-vs-clean distinction). Enumerated so
/// agent surfaces branch on members, never prose; the human form matches
/// exhaustively, so a future variant forces a new worded arm there (the
/// Fix-1 revise lesson).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CheckNotRunReason {
    /// The acquisition version defers the check's field group by declared
    /// scope (the Slice-5 deferral) — dormant, not an error.
    Dormant,
    /// The acquisition tried to read the check's inputs and FAILED — the
    /// finding query failed with the tracker present, so no finding record
    /// exists to check. The could-not-check condition guardrail REQ-4 fails
    /// the gate closed on.
    CouldNotCheck,
}

/// A finding-reading integrity check that did NOT run this derivation, with
/// why (vsdd-cli #818 Fix 2). Its silence in `integrity_findings` is no
/// assurance — the false-assurance defect was exactly reading a dormant or
/// failed check's silence as checked-clean.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckNotRun {
    /// The check id, mirroring the snapshot-schema audit block's members.
    pub check: String,
    pub reason: CheckNotRunReason,
}

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
    /// Present exactly when `next_action` was driven by a state-sourced gate
    /// verdict (the phase-2a/2b/2c advancement arms); marks that verdict's
    /// provenance so a self-authored `last_gate_result` is never presented as
    /// verified (vsdd-cli #818 Fix 1). Absent when no gate drove the action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gate_provenance: Option<GateProvenance>,
    /// The degraded kind — present exactly when corroboration is absent
    /// or unusable, never a bare flag (tracker-absent | tracker-unusable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<String>,
    /// Snapshot-scoped integrity finding kinds; the comparison grain is
    /// the kind-set.
    pub integrity_findings: Vec<String>,
    /// The finding-reading checks that did NOT run this derivation, each
    /// with why — dormant (a field group this acquisition version defers) or
    /// could-not-check (the finding query failed with the tracker present)
    /// (vsdd-cli #818 Fix 2). The false-assurance guard: an empty
    /// `integrity_findings` reads checked-clean ONLY when this is also
    /// empty. Empty under a degraded outcome — the degraded kind carries
    /// that story, like `integrity_findings` itself.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checks_not_run: Vec<CheckNotRun>,
}
