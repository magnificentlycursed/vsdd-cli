//! Layer 1 red gate — the nine data sets as loadable artifacts.
//!
//! Phase 2a suite (vsdd-cli #716): loads run against the live repo tree,
//! so the artifacts themselves are the fixtures; assertions pin the
//! operator-adopted values the consuming layers depend on. Corruption
//! cases run on temp copies. Fails executed against the
//! pre-implementation stubs; phase 2b turns it green.

use std::fs;
use std::path::{Path, PathBuf};

use vsdd_core::registry::{self, sets::*, Registry};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

#[test]
fn all_nine_sets_load_from_the_live_tree() {
    let reg: Registry = registry::load_all(&repo_root()).expect("all nine sets load");
    assert_eq!(reg.gate_data.schema_class, "gate-data");
    assert_eq!(reg.statusline_data.schema_class, "statusline-data");
    assert_eq!(
        reg.installed_artifact_manifest.schema_class,
        "installed-artifact-manifest"
    );
    assert_eq!(reg.state_schema.schema_class, "state-schema");
    assert_eq!(
        reg.composition_scope_and_actions.schema_class,
        "composition-scope-and-actions"
    );
    assert_eq!(reg.dispatch_data.schema_class, "dispatch-data");
    assert_eq!(
        reg.act_to_affordance_map.schema_class,
        "act-to-affordance-map"
    );
    assert_eq!(reg.economics_data.schema_class, "economics-data");
    assert_eq!(reg.snapshot_schema.schema_class, "snapshot-schema");
}

#[test]
fn gate_data_carries_the_adopted_doctrine() {
    let gate: GateData = registry::load_set(&repo_root(), "gate-data").expect("gate data loads");
    assert_eq!(
        gate.failure_kinds.len(),
        12,
        "twelve kinds after the round-1 additions"
    );
    let conditional: Vec<_> = gate
        .failure_kinds
        .iter()
        .filter(|k| k.red_validity == "conditional")
        .collect();
    assert_eq!(conditional.len(), 1, "exactly one conditional kind");
    assert_eq!(conditional[0].kind, "compile-failure");
    assert!(
        gate.failure_kinds
            .iter()
            .all(|k| k.scope == "per-test" || k.scope == "whole-suite"),
        "every kind carries the scope axis"
    );
    assert_eq!(
        gate.flake_policy.runs_per_gate_execution, 3,
        "the adopted flake count"
    );
}

#[test]
fn statusline_data_carries_the_glance_and_recovery_rulings() {
    let sl: StatuslineData =
        registry::load_set(&repo_root(), "statusline-data").expect("statusline data loads");
    assert_eq!(sl.read_failure_kinds.len(), 3);
    assert_eq!(
        sl.display_fields.len(),
        4,
        "the session segment is demoted (#679)"
    );
    assert!(
        sl.display_fields.iter().all(|f| f.field != "session"),
        "no session field on the glance surface"
    );
    let milestone = sl
        .display_fields
        .iter()
        .find(|f| f.field == "milestone-with-count")
        .expect("milestone field");
    assert_eq!(milestone.width_budget_chars, 34, "the #680 width ruling");
    for kind in &sl.read_failure_kinds {
        assert!(!kind.machine_token.is_empty() && !kind.human_recovery.is_empty());
    }
    let markers: Vec<&str> = sl
        .degraded_kinds
        .iter()
        .map(|d| d.marker_word.as_str())
        .collect();
    assert_eq!(markers, ["tracker offline", "tracker degraded"]);
    assert_eq!(
        sl.wiring_outcomes.len(),
        6,
        "the Install requirement's fixed six"
    );
}

#[test]
fn manifest_cross_references_resolve() {
    // The declared cross-field constraints, executed as a test: every
    // pairs_with id resolves to an entry; every referenced_by id resolves
    // to a reference surface; entry ids unique.
    let m: InstalledArtifactManifest =
        registry::load_set(&repo_root(), "installed-artifact-manifest").expect("manifest loads");
    let ids: Vec<&str> = m.entries.iter().map(|e| e.id.as_str()).collect();
    let mut unique = ids.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(ids.len(), unique.len(), "entry ids unique");
    let surfaces: Vec<&str> = m.reference_surfaces.iter().map(|s| s.id.as_str()).collect();
    for entry in &m.entries {
        for pair in &entry.pairs_with {
            assert!(ids.contains(&pair.as_str()), "pairs_with `{pair}` resolves");
        }
        for r in &entry.referenced_by {
            assert!(
                surfaces.contains(&r.as_str()),
                "referenced_by `{r}` resolves"
            );
        }
    }
    let host_wired: Vec<&str> = m
        .entries
        .iter()
        .filter(|e| e.lifetime == "host-wiring")
        .map(|e| e.id.as_str())
        .collect();
    assert!(host_wired.contains(&"plugin-set"), "the #703 axis value");
    assert!(host_wired.contains(&"statusline-wiring"), "the #709 sweep");
}

#[test]
fn composition_scope_and_vocabulary_hold_their_shape() {
    let c: CompositionScopeAndActions =
        registry::load_set(&repo_root(), "composition-scope-and-actions")
            .expect("composition loads");
    assert_eq!(c.scope_members.len(), 11, "ten phases plus the fix lane");
    let lane = c
        .scope_members
        .iter()
        .find(|m| m.id == "fix-lane")
        .expect("fix-lane member");
    assert_eq!(lane.kind, "lane", "the fix lane is never called a phase");
    let recovery: Vec<&str> = c
        .action_vocabulary
        .iter()
        .filter(|a| a.family == "recovery")
        .map(|a| a.id.as_str())
        .collect();
    for token in [
        "restore-state-file",
        "fix-state-content",
        "fix-state-permissions",
        "reconcile-toward-artifact",
    ] {
        assert!(
            recovery.contains(&token),
            "recovery family carries `{token}`"
        );
    }
}

#[test]
fn affordance_map_carries_the_adopted_bindings() {
    let map: ActToAffordanceMap =
        registry::load_set(&repo_root(), "act-to-affordance-map").expect("map loads");
    assert!(map.entries.len() >= 14);
    assert!(
        map.entries.iter().any(|e| e.kind == "session-surface"),
        "the Workflow adoption's binding kind"
    );
    assert!(!map.rules.is_empty());
}

#[test]
fn economics_presets_hold_the_verified_counts() {
    let e: EconomicsData =
        registry::load_set(&repo_root(), "economics-data").expect("economics loads");
    assert_eq!(e.presets.len(), 3);
    let standard = e
        .presets
        .iter()
        .find(|p| p.id == "standard")
        .expect("standard preset");
    for domain in [
        "software-engineer",
        "security",
        "ux",
        "quality-engineer",
        "solution-owner",
        "solution-architect",
    ] {
        assert!(
            standard.active_domains.contains(domain),
            "the six core domains are named inline (#710): missing `{domain}`"
        );
    }
}

#[test]
fn snapshot_schema_pins_the_four_renderer_fields() {
    let s: SnapshotSchemaSet =
        registry::load_set(&repo_root(), "snapshot-schema").expect("snapshot schema loads");
    assert!(s.snapshot_fields.len() >= 4);
    let renderer_fields = s
        .audit
        .get("renderer_display_fields")
        .and_then(|v| v.as_sequence())
        .map(|v| v.len());
    assert_eq!(
        renderer_fields,
        Some(4),
        "exactly four renderer display fields"
    );
}

#[test]
fn corrupted_frontmatter_yields_a_located_diagnostic() {
    let dir = tempfile::tempdir().unwrap();
    let reg_dir = dir.path().join("templates/registry");
    fs::create_dir_all(&reg_dir).unwrap();
    let schema_dir = dir.path().join(".mdatron/schemas");
    fs::create_dir_all(&schema_dir).unwrap();
    for entry in fs::read_dir(repo_root().join("templates/registry")).unwrap() {
        let p = entry.unwrap().path();
        if p.is_file() {
            fs::copy(&p, reg_dir.join(p.file_name().unwrap())).unwrap();
        }
    }
    for entry in fs::read_dir(repo_root().join(".mdatron/schemas")).unwrap() {
        let p = entry.unwrap().path();
        if p.is_file() {
            fs::copy(&p, schema_dir.join(p.file_name().unwrap())).unwrap();
        }
    }
    // Break the gate-data frontmatter mid-block.
    let target = reg_dir.join("gate-data.md");
    let original = fs::read_to_string(&target).unwrap();
    let corrupted = original.replace("flake_policy:", "flake_policy: [unclosed");
    assert_ne!(original, corrupted, "the corruption must have taken");
    fs::write(&target, corrupted).unwrap();

    let diag = registry::load_set::<GateData>(dir.path(), "gate-data")
        .expect_err("corrupted frontmatter is a diagnostic");
    assert!(
        diag.file.ends_with("gate-data.md"),
        "the diagnostic names the file"
    );
    assert!(
        diag.location.is_some(),
        "a parse failure carries its location"
    );
}

#[test]
fn schema_class_mismatch_yields_a_diagnostic() {
    let dir = tempfile::tempdir().unwrap();
    let reg_dir = dir.path().join("templates/registry");
    fs::create_dir_all(&reg_dir).unwrap();
    let schema_dir = dir.path().join(".mdatron/schemas");
    fs::create_dir_all(&schema_dir).unwrap();
    for entry in fs::read_dir(repo_root().join(".mdatron/schemas")).unwrap() {
        let p = entry.unwrap().path();
        if p.is_file() {
            fs::copy(&p, schema_dir.join(p.file_name().unwrap())).unwrap();
        }
    }
    // A statusline artifact filed under the gate-data name.
    fs::copy(
        repo_root().join("templates/registry/statusline-data.md"),
        reg_dir.join("gate-data.md"),
    )
    .unwrap();

    let diag = registry::load_set::<GateData>(dir.path(), "gate-data")
        .expect_err("class mismatch is a diagnostic");
    assert!(
        diag.message.contains("gate-data") && diag.message.contains("statusline-data"),
        "the diagnostic names expected and found classes"
    );
}

// ── Round-1 fix-pass additions (vsdd-cli #723, #724, #725, #727, #728) ────────

#[test]
fn state_schema_set_mirrors_the_state_struct() {
    // The one-for-one mirror pin (vsdd-cli #725): the artifact's
    // enumeration against State's serde surface — names in order,
    // presence-required flags matching Option-ness (nullable fields stay
    // presence-required; last_gate_result and published are the two
    // absent-until-written members).
    let s: StateSchemaSet =
        registry::load_set(&repo_root(), "state-schema").expect("state-schema set loads");
    let declared: Vec<(&str, bool)> = s
        .state_fields
        .iter()
        .map(|f| (f.name.as_str(), f.required))
        .collect();
    assert_eq!(
        declared,
        vec![
            ("schema_version", true),
            ("current_phase", true),
            ("current_layer", true),
            ("open_findings_pointer", true),
            ("last_gate_result", false),
            ("active_composition", true),
            ("published", false),
        ],
        "state-schema's enumeration matches vsdd_core::state::State (schema.rs) one-for-one"
    );
}

#[test]
fn dispatch_data_pins_its_adopted_values() {
    // Value pins for the set that had only liveness coverage (vsdd-cli #728).
    let d: DispatchData =
        registry::load_set(&repo_root(), "dispatch-data").expect("dispatch data loads");
    assert_eq!(d.preflight_members.len(), 5, "the five preflight members");
    assert_eq!(d.manifest_fields.len(), 15, "the fifteen manifest fields");
    assert!(!d.fencing_rule.is_empty());
    let result_values = d
        .preflight_semantics
        .get("result_values")
        .and_then(|v| v.as_sequence())
        .map(|v| v.len());
    assert_eq!(result_values, Some(3), "pass, fail, inconclusive");
}

#[test]
fn registry_repair_token_is_the_registered_member() {
    // The loader's bootstrap mirror (vsdd-cli #724): it cannot load the
    // vocabulary before loading, so its constant mirrors the registered
    // member — and this test is the pin that keeps the two equal.
    let c: CompositionScopeAndActions =
        registry::load_set(&repo_root(), "composition-scope-and-actions")
            .expect("composition set loads");
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("templates/registry")).unwrap();
    fs::create_dir_all(dir.path().join(".mdatron/schemas")).unwrap();
    let diag = registry::load_set::<GateData>(dir.path(), "gate-data")
        .expect_err("an absent artifact is a diagnostic");
    let member = c
        .action_vocabulary
        .iter()
        .find(|a| a.id == diag.recovery_action)
        .expect("the loader's mirrored token resolves to a registered member");
    assert_eq!(member.family, "recovery");
}

#[test]
fn invalid_class_is_rejected_before_any_path_join() {
    // The traversal guard (vsdd-cli #727): a class that is not one safe
    // lowercase segment never reaches the filesystem.
    for bad in ["../escape", "/etc/shadow", "UPPER", "a b", ""] {
        let diag = registry::load_set::<GateData>(&repo_root(), bad)
            .expect_err("an unsafe class is refused");
        assert_eq!(diag.kind, "invalid-class", "class `{bad}`");
    }
}

#[test]
fn load_all_names_every_failing_artifact_in_one_pass() {
    // Bulk corruption reports all casualties at once (vsdd-cli #727).
    let dir = tempfile::tempdir().unwrap();
    let reg_dir = dir.path().join("templates/registry");
    fs::create_dir_all(&reg_dir).unwrap();
    let schema_dir = dir.path().join(".mdatron/schemas");
    fs::create_dir_all(&schema_dir).unwrap();
    for entry in fs::read_dir(repo_root().join("templates/registry")).unwrap() {
        let p = entry.unwrap().path();
        if p.is_file() {
            fs::copy(&p, reg_dir.join(p.file_name().unwrap())).unwrap();
        }
    }
    for entry in fs::read_dir(repo_root().join(".mdatron/schemas")).unwrap() {
        let p = entry.unwrap().path();
        if p.is_file() {
            fs::copy(&p, schema_dir.join(p.file_name().unwrap())).unwrap();
        }
    }
    for victim in ["gate-data.md", "statusline-data.md"] {
        let target = reg_dir.join(victim);
        let original = fs::read_to_string(&target).unwrap();
        let corrupted = format!("broken, not frontmatter\n{original}");
        fs::write(&target, corrupted).unwrap();
    }

    let errors = registry::load_all(dir.path()).expect_err("two broken artifacts fail the load");
    assert_eq!(errors.len(), 2, "one pass names every failing artifact");
    let named: Vec<String> = errors
        .iter()
        .map(|d| d.file.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    assert!(named.contains(&"gate-data.md".to_string()));
    assert!(named.contains(&"statusline-data.md".to_string()));
}

#[test]
fn bom_before_the_fence_still_splits() {
    // Windows-editor provenance (vsdd-cli #727).
    let dir = tempfile::tempdir().unwrap();
    let reg_dir = dir.path().join("templates/registry");
    fs::create_dir_all(&reg_dir).unwrap();
    let schema_dir = dir.path().join(".mdatron/schemas");
    fs::create_dir_all(&schema_dir).unwrap();
    for entry in fs::read_dir(repo_root().join(".mdatron/schemas")).unwrap() {
        let p = entry.unwrap().path();
        if p.is_file() {
            fs::copy(&p, schema_dir.join(p.file_name().unwrap())).unwrap();
        }
    }
    let original = fs::read_to_string(repo_root().join("templates/registry/gate-data.md")).unwrap();
    fs::write(reg_dir.join("gate-data.md"), format!("\u{feff}{original}")).unwrap();

    let loaded: Result<GateData, _> = registry::load_set(dir.path(), "gate-data");
    assert!(
        loaded.is_ok(),
        "a leading byte-order mark does not defeat the fence"
    );
}
