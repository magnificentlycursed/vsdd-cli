//! One typed struct per registry schema class; field names mirror the
//! artifacts verbatim (`templates/registry/*.md`). Fields the Layer 1
//! tests assert are fully typed; deeper payloads ride as YAML values and
//! deepen with their consuming layers — the artifact and its schema pair
//! remain the authority either way.

use serde::Deserialize;
use serde_yaml_ng::Value;

// --- gate-data -----------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct FailureKind {
    pub kind: String,
    pub red_validity: String,
    pub scope: String,
    pub meaning: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FlakePolicy {
    // u64, not u32: numeric widths align with the schema pairs' unbounded
    // integers so a pair-valid value cannot fail the typed decode
    // (vsdd-cli #723's drift example).
    pub runs_per_gate_execution: u64,
    pub per_test_aggregation: String,
    pub scope: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GateData {
    pub schema_class: String,
    pub schema_version: String,
    pub status: String,
    pub failure_kinds: Vec<FailureKind>,
    /// Deepens at Layer 6 (the gate commands' consumer).
    pub pin_kind_declaration: Value,
    pub flake_policy: FlakePolicy,
    /// Deepens at Layer 6 (the gate commands' consumer).
    pub cannot_run_predicate: Value,
    /// Deepens at Layer 6 (the gate commands' consumer).
    pub mapping_schema: Value,
}

// --- statusline-data -----------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct ReadFailureKind {
    pub kind: String,
    pub machine_token: String,
    pub recovery_action: String,
    pub human_diagnostic: String,
    pub human_recovery: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DisplayField {
    pub field: String,
    pub width_budget_chars: u64,
    pub absence_text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DegradedKind {
    pub kind: String,
    pub marker_word: String,
    pub benign: bool,
    pub next_step_text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WiringOutcome {
    pub id: String,
    pub meaning: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StatuslineData {
    pub schema_class: String,
    pub schema_version: String,
    pub status: String,
    pub read_failure_kinds: Vec<ReadFailureKind>,
    // Typed because their pair shapes are final (vsdd-cli #728): the
    // statusline renderer (Layer 3) consumes these like read_failure_kinds.
    pub degraded_kinds: Vec<DegradedKind>,
    pub wiring_outcomes: Vec<WiringOutcome>,
    pub display_fields: Vec<DisplayField>,
    pub truncation_mark: String,
    pub truncation_rule: String,
    pub substrate_findings_visibility: String,
    pub broken_state_mark: String,
    pub wall_clock_budget_ms: u64,
    /// Deepens at Layer 3 (the timing check's consumer).
    pub timing_check: Value,
    /// Deepens at Layer 3 (the wiring script's consumer).
    pub repo_set_config: Value,
    pub composition_instruction_conduct: String,
}

// --- installed-artifact-manifest ------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct ManifestEntry {
    pub id: String,
    pub path: String,
    pub class: String,
    pub source: String,
    pub lifetime: String,
    pub referenced_by: Vec<String>,
    pub pairs_with: Vec<String>,
    pub resolution: String,
    pub fail_mode: String,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReferenceSurface {
    pub id: String,
    pub path: String,
    pub scope: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InstalledArtifactManifest {
    pub schema_class: String,
    pub schema_version: String,
    pub status: String,
    pub reference_surfaces: Vec<ReferenceSurface>,
    pub entries: Vec<ManifestEntry>,
}

// --- composition-scope-and-actions ----------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct ScopeMember {
    pub id: String,
    pub kind: String,
    pub whitepaper_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ActionToken {
    pub id: String,
    pub family: String,
    pub phase: String,
    pub human: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompositionScopeAndActions {
    pub schema_class: String,
    pub schema_version: String,
    pub status: String,
    pub scope_members: Vec<ScopeMember>,
    pub action_vocabulary: Vec<ActionToken>,
}

// --- act-to-affordance-map -------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct MapEntry {
    pub act: String,
    pub affordance: String,
    pub kind: String,
    pub condition: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ActToAffordanceMap {
    pub schema_class: String,
    pub schema_version: String,
    pub status: String,
    pub entries: Vec<MapEntry>,
    pub rules: Vec<String>,
}

// --- economics-data ---------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct Preset {
    pub id: String,
    pub active_domains: String,
    pub round_budget: u64,
    pub stop_sensitivity: String,
    pub mutation_floor_declared: bool,
    pub note: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EconomicsData {
    pub schema_class: String,
    pub schema_version: String,
    pub status: String,
    /// Deepens at Layer 9 (the cost crate's consumer), as do the four
    /// Value payloads below.
    pub effort_signals: Value,
    pub tier_effort_defaults: Vec<Value>,
    pub mutation_floor: Value,
    pub token_budgets: Vec<Value>,
    pub calibration_bands: Vec<Value>,
    pub presets: Vec<Preset>,
}

// --- dispatch-data -----------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct DispatchData {
    pub schema_class: String,
    pub schema_version: String,
    pub status: String,
    /// Deepens at Layer 8 (recorded dispatch), as do the preflight and
    /// manifest payloads below; the branch grammar's refs query joins at
    /// Layer 2.
    pub branch_grammar: Value,
    pub preflight_members: Vec<Value>,
    pub preflight_semantics: Value,
    pub fencing_rule: String,
    pub manifest_fields: Vec<Value>,
}

// --- state-schema (the data set describing the state artifact) ---------------

#[derive(Debug, Clone, Deserialize)]
pub struct StateFieldDecl {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub required: bool,
    pub semantics: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StateSchemaSet {
    pub schema_class: String,
    pub schema_version: String,
    pub status: String,
    // Typed so the State struct's claimed mirror is testable one-for-one
    // (vsdd-cli #725).
    pub state_fields: Vec<StateFieldDecl>,
    /// Deepens at Layer 7 (mdatron's state-consistency family).
    pub declared_constraints: Vec<Value>,
}

// --- snapshot-schema ----------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct SnapshotSchemaSet {
    pub schema_class: String,
    pub schema_version: String,
    pub status: String,
    /// Deepens at Layer 2 (the snapshot acquisition's consumer).
    pub snapshot_fields: Vec<Value>,
    /// Deepens at Layer 2.
    pub audit: Value,
}
