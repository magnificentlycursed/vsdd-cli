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
pub struct BranchForm {
    pub id: String,
    pub pattern: String,
    pub meaning: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BranchGrammar {
    pub forms: Vec<BranchForm>,
    /// The #688-adopted exemption set as structured data (vsdd-cli #738).
    pub exempt_refs: Vec<String>,
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DispatchData {
    pub schema_class: String,
    pub schema_version: String,
    pub status: String,
    /// Typed at Layer 2, its refs-query consumer.
    pub branch_grammar: BranchGrammar,
    /// Deepens at Layer 8 (recorded dispatch), as do the semantics and
    /// manifest payloads below.
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

/// A post-deserialization hook the registry loader runs on every set
/// (vsdd-cli #794): the one place adopter-authored, terminal-destined
/// display strings are cleaned, so every downstream surface receives
/// terminal-safe data. Sets with no terminal-destined free-form string
/// take the no-op default.
pub trait PostLoad {
    fn post_load(&mut self) {}
}

impl PostLoad for StatuslineData {
    fn post_load(&mut self) {
        use crate::text::clean_for_terminal;
        self.truncation_mark = clean_for_terminal(&self.truncation_mark);
        self.broken_state_mark = clean_for_terminal(&self.broken_state_mark);
        for f in &mut self.display_fields {
            f.absence_text = clean_for_terminal(&f.absence_text);
        }
        for k in &mut self.degraded_kinds {
            k.marker_word = clean_for_terminal(&k.marker_word);
            k.next_step_text = clean_for_terminal(&k.next_step_text);
        }
        for k in &mut self.read_failure_kinds {
            k.human_diagnostic = clean_for_terminal(&k.human_diagnostic);
            k.human_recovery = clean_for_terminal(&k.human_recovery);
        }
    }
}

impl PostLoad for GateData {}
impl PostLoad for InstalledArtifactManifest {}
impl PostLoad for CompositionScopeAndActions {
    fn post_load(&mut self) {
        use crate::text::clean_for_terminal;
        // Latent-safe (vsdd-cli #801): these human strings have no
        // terminal sink today, but the clean-at-source invariant means
        // a later layer that prints them cannot inherit an unclean one.
        for a in &mut self.action_vocabulary {
            a.human = clean_for_terminal(&a.human);
        }
        for s in &mut self.scope_members {
            s.whitepaper_name = clean_for_terminal(&s.whitepaper_name);
        }
    }
}
impl PostLoad for ActToAffordanceMap {}
impl PostLoad for EconomicsData {}
impl PostLoad for DispatchData {}
impl PostLoad for StateSchemaSet {}
impl PostLoad for SnapshotSchemaSet {}

#[cfg(test)]
mod post_load_tests {
    use super::*;

    #[test]
    fn statusline_post_load_cleans_every_registry_display_string() {
        // The registry-string class cleaned once at load (vsdd-cli
        // #794): a hostile character in any terminal-destined field is
        // gone after post_load — the surfaces never see it.
        let mut d = StatuslineData {
            schema_class: "statusline-data".to_string(),
            schema_version: "0.0.0".to_string(),
            status: "test".to_string(),
            read_failure_kinds: vec![ReadFailureKind {
                kind: "malformed".to_string(),
                machine_token: "malformed".to_string(),
                recovery_action: "x".to_string(),
                human_diagnostic: "diag\u{202e}".to_string(),
                human_recovery: "rec\u{200b}".to_string(),
            }],
            degraded_kinds: vec![DegradedKind {
                kind: "tracker-absent".to_string(),
                marker_word: "mark\u{2060}".to_string(),
                benign: true,
                next_step_text: "step\u{e0069}".to_string(),
            }],
            wiring_outcomes: vec![],
            display_fields: vec![DisplayField {
                field: "work-item".to_string(),
                width_budget_chars: 24,
                absence_text: "none\u{2028}".to_string(),
            }],
            truncation_mark: "(cut)\u{1b}".to_string(),
            truncation_rule: String::new(),
            substrate_findings_visibility: String::new(),
            broken_state_mark: "broken\u{feff}".to_string(),
            wall_clock_budget_ms: 250,
            timing_check: serde_yaml_ng::Value::Null,
            repo_set_config: serde_yaml_ng::Value::Null,
            composition_instruction_conduct: String::new(),
        };
        d.post_load();
        assert_eq!(d.truncation_mark, "(cut)");
        assert_eq!(d.broken_state_mark, "broken");
        assert_eq!(d.display_fields[0].absence_text, "none");
        assert_eq!(d.degraded_kinds[0].marker_word, "mark");
        assert_eq!(d.degraded_kinds[0].next_step_text, "step");
        assert_eq!(d.read_failure_kinds[0].human_diagnostic, "diag");
        assert_eq!(d.read_failure_kinds[0].human_recovery, "rec");
    }
}
