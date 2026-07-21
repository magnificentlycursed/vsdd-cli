//! The nine versioned data sets as loadable artifacts (Layer 1 of the
//! decomposition). Each loader validates its artifact against the
//! `.mdatron/schemas/<class>.json` pair at read — the trust boundary's
//! read-time validation, vsdd self-validating until mdatron's
//! state-consistency family lands (vsdd supplies schemas, mdatron
//! executes them, per the boundary preamble).
//!
//! Registry-surface diagnostics are developer-facing prose: the
//! registered token vocabulary (the statusline data set's) governs the
//! state artifact's surfaces; the registry loader names files,
//! locations, and repairs in plain words.

pub mod frontmatter;
pub mod sets;

use std::fs;
use std::path::{Path, PathBuf};

use mdatron_core::Schema;
use serde::de::DeserializeOwned;

use crate::diagnostics::Diagnostic;
use sets::{
    ActToAffordanceMap, CompositionScopeAndActions, DispatchData, EconomicsData, GateData,
    InstalledArtifactManifest, SnapshotSchemaSet, StateSchemaSet, StatuslineData,
};

/// All nine sets, loaded and validated.
#[derive(Debug)]
pub struct Registry {
    pub installed_artifact_manifest: InstalledArtifactManifest,
    pub state_schema: StateSchemaSet,
    pub composition_scope_and_actions: CompositionScopeAndActions,
    pub statusline_data: StatuslineData,
    pub gate_data: GateData,
    pub dispatch_data: DispatchData,
    pub act_to_affordance_map: ActToAffordanceMap,
    pub economics_data: EconomicsData,
    pub snapshot_schema: SnapshotSchemaSet,
}

fn artifact_diagnostic(
    file: PathBuf,
    kind: &str,
    message: String,
    location: Option<(usize, usize)>,
) -> Box<Diagnostic> {
    Box::new(Diagnostic {
        file,
        kind: kind.to_string(),
        machine_token: kind.to_string(),
        location,
        message,
        recovery_action: "repair-registry-artifact".to_string(),
        recovery_text: "repair the artifact to match its schema pair, then re-run".to_string(),
    })
}

/// Load one set from `templates/registry/<class>.md`, validating the
/// frontmatter against its schema pair. A missing file, a frontmatter
/// parse failure (with location), a `schema_class` mismatch, or a
/// schema-pair violation each yield a diagnostic naming the file —
/// never a panic (the registry is adopter-edited).
pub fn load_set<T: DeserializeOwned>(repo_root: &Path, class: &str) -> Result<T, Box<Diagnostic>> {
    let path = repo_root
        .join("templates/registry")
        .join(format!("{class}.md"));
    let text = fs::read_to_string(&path).map_err(|e| {
        let kind = if e.kind() == std::io::ErrorKind::NotFound {
            "absent"
        } else {
            "permission-or-io"
        };
        artifact_diagnostic(
            path.clone(),
            kind,
            format!("cannot read the registry artifact: {e}"),
            None,
        )
    })?;

    let (fm, _body) = frontmatter::split_frontmatter(&text).map_err(|e| {
        artifact_diagnostic(
            path.clone(),
            "malformed",
            e.message.clone(),
            Some((e.line, e.column)),
        )
    })?;
    // Frontmatter begins on line 2 of the file; parser locations are
    // relative to the frontmatter slice.
    let fm_line = |line: usize| line + 1;

    let value: serde_yaml_ng::Value = serde_yaml_ng::from_str(fm).map_err(|e| {
        let location = e.location().map(|l| (fm_line(l.line()), l.column()));
        artifact_diagnostic(
            path.clone(),
            "malformed",
            format!("frontmatter parse failure: {e}"),
            location,
        )
    })?;

    let found = value
        .get("schema_class")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if found != class {
        return Err(artifact_diagnostic(
            path,
            "schema-class-mismatch",
            format!("expected schema_class `{class}`, found `{found}`"),
            None,
        ));
    }

    let schema_path = repo_root
        .join(".mdatron/schemas")
        .join(format!("{class}.json"));
    let schema_text = fs::read_to_string(&schema_path).map_err(|e| {
        artifact_diagnostic(
            schema_path.clone(),
            "absent",
            format!("cannot read the schema pair: {e}"),
            None,
        )
    })?;
    let schema_json: serde_json::Value = serde_json::from_str(&schema_text).map_err(|e| {
        artifact_diagnostic(
            schema_path.clone(),
            "malformed",
            format!("the schema pair is not valid JSON: {e}"),
            Some((e.line(), e.column())),
        )
    })?;
    let schema = Schema::compile(&schema_json).map_err(|e| {
        artifact_diagnostic(
            schema_path.clone(),
            "malformed",
            format!("the schema pair does not compile: {e}"),
            None,
        )
    })?;
    let violations = schema.validate(&value);
    if !violations.is_empty() {
        return Err(artifact_diagnostic(
            path,
            "schema-violation",
            format!("the artifact violates its schema pair: {violations:?}"),
            None,
        ));
    }

    serde_yaml_ng::from_str::<T>(fm).map_err(|e| {
        let location = e.location().map(|l| (fm_line(l.line()), l.column()));
        artifact_diagnostic(
            path,
            "malformed",
            format!("typed decode failure: {e}"),
            location,
        )
    })
}

/// Load all nine sets from the repo; the first failure reports with its
/// file per `load_set`'s discipline.
pub fn load_all(repo_root: &Path) -> Result<Registry, Box<Diagnostic>> {
    Ok(Registry {
        installed_artifact_manifest: load_set(repo_root, "installed-artifact-manifest")?,
        state_schema: load_set(repo_root, "state-schema")?,
        composition_scope_and_actions: load_set(repo_root, "composition-scope-and-actions")?,
        statusline_data: load_set(repo_root, "statusline-data")?,
        gate_data: load_set(repo_root, "gate-data")?,
        dispatch_data: load_set(repo_root, "dispatch-data")?,
        act_to_affordance_map: load_set(repo_root, "act-to-affordance-map")?,
        economics_data: load_set(repo_root, "economics-data")?,
        snapshot_schema: load_set(repo_root, "snapshot-schema")?,
    })
}
