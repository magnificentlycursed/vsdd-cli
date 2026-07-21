//! The State struct mirrors `templates/registry/state-schema.md`'s
//! `state_fields` enumeration verbatim (contract: Deterministic phase
//! answer; the state artifact's contents enumeration).

use serde::{Deserialize, Serialize};

/// The state-instance format version this crate reads and writes;
/// bootstrap self-validation checks it before reading further.
pub const SUPPORTED_STATE_SCHEMA_VERSION: &str = "0.1.0";

/// The `.vsdd/state.yaml` contents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub schema_version: String,
    /// Null is pinned to mean "not yet entered"; the derivation maps it
    /// to the enter action (operator ruling 2026-07-20, vsdd-cli #665).
    pub current_phase: Option<String>,
    /// Null is pinned to mean "decomposition not yet authored" (same ruling).
    pub current_layer: Option<u32>,
    pub open_findings_pointer: OpenFindingsPointer,
    /// Absent until the first gate runs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_gate_result: Option<GateResult>,
    pub active_composition: ActiveComposition,
    /// Absent before first publish; written once; immutable thereafter
    /// (forward-only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published: Option<Published>,
}

/// The active layer's milestone; open findings are its open children.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenFindingsPointer {
    pub milestone: String,
}

/// The closed gate-kind enumeration (vsdd-cli #694).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateKind {
    #[serde(rename = "red-gate")]
    RedGate,
    #[serde(rename = "green-gate")]
    GreenGate,
    #[serde(rename = "fix-scale-gate")]
    FixScaleGate,
    #[serde(rename = "phase-exit-gate")]
    PhaseExitGate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GateOutcome {
    Pass,
    Fail,
}

/// The most recent gate run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GateResult {
    pub gate: GateKind,
    pub phase: String,
    pub layer: u32,
    pub result: GateOutcome,
    /// A commit sha or tracker handle.
    pub evidence: String,
    /// ISO 8601.
    pub recorded: String,
}

/// The attended and autonomous sides of the dispatch split, respectively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompositionMode {
    #[serde(rename = "skill-interactive")]
    SkillInteractive,
    #[serde(rename = "cold-dispatch")]
    ColdDispatch,
}

/// The computed domain set in force; the hash makes a stale composition
/// mechanically detectable (operator ruling 2026-07-20, vsdd-cli #665).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActiveComposition {
    pub scope: String,
    pub domains: Vec<String>,
    pub mode: CompositionMode,
    pub config_inputs_hash: String,
}

/// The published marker — written once by the promotion act; immutable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Published {
    pub at: String,
    pub version: String,
    pub act: String,
}
