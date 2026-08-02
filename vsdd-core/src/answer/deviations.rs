//! The deviations gate leg over `.vsdd/registry/deviation-registry.yaml`
//! (build-plan Phase 1, the deviations gate leg; ratified remediation design
//! `.design/decomposition-topology-remediation.md` REQ-4/REQ-6, fixture
//! family AC-5).
//!
//! Shape: pure over parsed data — `deviations_verdict` takes the parsed
//! registry, a caller-supplied `today` (no clock in vsdd-core), the gate
//! mode, and an injectable issue-state oracle, and returns a verdict plus
//! the warn-grade surface (a warning list). `load_deviation_registry` is
//! the I/O boundary; `deviations_gate` composes the two, mapping every
//! load failure to `Unverifiable` — deleting the registry must never pass.
//!
//! PHASE-2A STATUS: red-gate stub. The signatures and types are the 1b
//! decisions; every behavior body is a placeholder (always `Pass`, one
//! placeholder warning) so `vsdd-core/tests/deviations_red_gate.rs` fails
//! EXECUTED — the non-vacuous red. Phase 2b fills the bodies.

use std::path::Path;

use serde::Deserialize;

use super::integrity::GateVerdict;

/// The parsed deviation registry (remediation design REQ-4: the file
/// carries `schema_version` plus the entry list).
///
/// The self-governance instance additionally carries a founding-population
/// record (`initial_population_sweep`); the gate reads entries only, so
/// the top level tolerates unknown keys while entries are strict.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DeviationRegistry {
    /// The registry schema version.
    pub schema_version: u32,
    /// The deviation entries, `standing` and `resolved` alike; the gate
    /// enumerates `standing` entries (and premature resolutions treated
    /// as still standing).
    pub entries: Vec<DeviationEntry>,
}

/// One registry entry (REQ-4's field roster).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeviationEntry {
    /// Plain-name identifier (no coined letter-clusters).
    pub id: String,
    /// What rides off-affordance or persists as a fallback.
    pub deviation: String,
    /// The act-to-affordance map `act` key departed from; null with a
    /// stated class for non-affordance deviations.
    pub deviates_from: Option<String>,
    /// The stated class when `deviates_from` is null (e.g. toolchain-pin).
    #[serde(default)]
    pub deviation_class: Option<String>,
    /// Why the deviation stands.
    pub stated_reason: String,
    /// Exactly one machine-decidable retest predicate; absent on resolved
    /// entries.
    #[serde(default)]
    pub retest_trigger: Option<RetestTrigger>,
    /// Optional prose context for the trigger (held context, not predicate).
    #[serde(default)]
    pub trigger_context: Option<String>,
    /// Entry date (ISO date); the 30-day default expiry computes from it.
    pub entry_date: String,
    /// Expiry date; re-arm rewrites it in the entry, so the gate compares
    /// only in-entry dates. Absent on resolved entries.
    #[serde(default)]
    pub expiry: Option<String>,
    /// The tracker issue owning the deviation.
    pub owning_issue: String,
    /// `standing` or `resolved` (terminal; regression requires a new entry).
    pub status: DeviationStatus,
    /// Machine shape of the Solution Owner decision: issue plus comment
    /// timestamp. The local gate treats it as an unverified claim
    /// (ref-presence plus in-entry date recency, warn grade); the CI leg
    /// verifies it over server-synced state (a #815-bound future).
    #[serde(default)]
    pub disposition_ref: Option<DispositionRef>,
}

/// Entry lifecycle status (REQ-4: `standing` → `resolved`, or `standing` →
/// re-armed `standing` with a new expiry; no parked third state).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviationStatus {
    /// The deviation stands; the gate enumerates and expiry-checks it.
    Standing,
    /// Resolved under a Solution Owner disposition; terminal. A resolved
    /// entry WITHOUT its own `disposition_ref` is a premature resolution,
    /// treated by the gate as still standing.
    Resolved,
}

/// The retest trigger: exactly one machine-decidable predicate, typed by
/// the schema-fixed grammar (REQ-4).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetestTrigger {
    /// The predicate's grammar class.
    #[serde(rename = "type")]
    pub trigger_type: TriggerType,
    /// The predicate text; a fully-machine boolean compound within one
    /// class counts as one predicate.
    pub predicate: String,
}

/// The schema-fixed trigger grammar (REQ-4): `date | issue-state |
/// version-compare | artifact-presence`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TriggerType {
    /// Fires when `today` reaches the predicate date (in-entry comparison).
    Date,
    /// Fires on an upstream issue reaching the predicated state; decidable
    /// only through an issue-state oracle.
    IssueState,
    /// Fires on a version comparison against a named artifact.
    VersionCompare,
    /// Fires on a grep-decidable artifact-presence predicate.
    ArtifactPresence,
}

/// Machine shape of a Solution Owner disposition reference (REQ-4).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DispositionRef {
    /// The tracker issue carrying the decision comment.
    pub issue: String,
    /// The decision comment's timestamp (in-entry date material for the
    /// recency comparison).
    pub comment_timestamp: String,
}

/// The gate's strictness mode (REQ-6's mode seam): the difference is
/// confined to undecidable-trigger handling and (future) disposition
/// verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateMode {
    /// The developer surface: undecidable triggers pass with a recorded
    /// warning; `disposition_ref` is an unverified claim (warn grade).
    Local,
    /// The enforcement surface: undecidable is inconclusive — exit-2 class
    /// `Unverifiable`, never a silent pass; dispositions verify over
    /// server-synced state (#815-bound).
    Ci,
}

/// The state an issue-state oracle reports for an upstream issue reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueState {
    /// The referenced issue is open.
    Open,
    /// The referenced issue is closed.
    Closed,
}

/// The deviations check's result: a three-valued verdict (the existing
/// 0/1/2 exit-class currency) plus the warn-grade surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviationsOutcome {
    /// Pass / Block(offending entry ids) / Unverifiable(reason).
    pub verdict: GateVerdict,
    /// Recorded warnings (undecidable triggers on the local gate, abusive
    /// overrides carrying the applied default, unverified-claim notes).
    pub warnings: Vec<String>,
}

/// Registry load/shape failure — every variant maps to `Unverifiable`
/// (fail-closed: an absent, malformed, or shape-invalid registry must
/// never pass the gate).
#[derive(Debug, thiserror::Error)]
pub enum DeviationRegistryError {
    /// The registry file is absent at the expected path; restore it (or
    /// re-run `vsdd init` to redeploy the scaffold) — deletion never passes.
    #[error("deviation registry absent at {path} — deleting the registry never passes the gate")]
    Absent {
        /// The path probed.
        path: String,
    },
    /// The file exists but could not be read.
    #[error("deviation registry unreadable at {path}: {detail}")]
    Unreadable {
        /// The path probed.
        path: String,
        /// What the read failed on.
        detail: String,
    },
    /// The file read but is not valid YAML or misses the registry shape
    /// (missing `schema_version`, an entry missing a required field).
    #[error("deviation registry shape-invalid at {path}: {detail}")]
    ShapeInvalid {
        /// The path probed.
        path: String,
        /// What the shape check failed on.
        detail: String,
    },
}

/// Read and shape-validate the deviation registry at `path` — the I/O
/// boundary in front of the pure verdict.
///
/// PHASE-2A STUB: returns an empty registry unconditionally; the real
/// read + shape validation land in 2b.
pub fn load_deviation_registry(
    path: &Path,
) -> Result<DeviationRegistry, DeviationRegistryError> {
    let _ = path;
    Ok(DeviationRegistry {
        schema_version: 0,
        entries: Vec::new(),
    })
}

/// The pure deviations verdict over a parsed registry (REQ-6): enumerate
/// `standing` entries (premature resolutions included); fail loud on a
/// lapsed expiry or a fired machine-decidable retest trigger without a
/// newer Solution Owner `disposition_ref`, comparing only in-entry dates
/// against the caller-supplied `today` (ISO date; no clock in vsdd-core).
/// `issue_oracle` resolves an upstream issue reference to its state, or
/// `None` when no oracle is available (the undecidable direction).
///
/// PHASE-2A STUB: always `Pass` with one placeholder warning, so every
/// red-gate test — including the pass-direction fixtures asserting a
/// warning-free clean pass — fails executed.
pub fn deviations_verdict(
    registry: &DeviationRegistry,
    today: &str,
    mode: GateMode,
    issue_oracle: &dyn Fn(&str) -> Option<IssueState>,
) -> DeviationsOutcome {
    let _ = (registry, today, mode, issue_oracle);
    DeviationsOutcome {
        verdict: GateVerdict::Pass,
        warnings: vec![
            "phase-2a placeholder: the deviations check is not implemented".to_string(),
        ],
    }
}

/// The composed deviations gate leg: load the registry at `registry_path`
/// and evaluate it; every load failure maps to `Unverifiable` in BOTH
/// modes (the fail-closed keystone).
pub fn deviations_gate(
    registry_path: &Path,
    today: &str,
    mode: GateMode,
    issue_oracle: &dyn Fn(&str) -> Option<IssueState>,
) -> DeviationsOutcome {
    match load_deviation_registry(registry_path) {
        Ok(registry) => deviations_verdict(&registry, today, mode, issue_oracle),
        Err(err) => DeviationsOutcome {
            verdict: GateVerdict::Unverifiable(err.to_string()),
            warnings: Vec::new(),
        },
    }
}
