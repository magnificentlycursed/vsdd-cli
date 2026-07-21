//! The nine versioned data sets as loadable artifacts (Layer 1 of the
//! decomposition). Each loader validates its artifact against the
//! `.mdatron/schemas/<class>.json` pair at read — the trust boundary's
//! read-time validation, vsdd self-validating until mdatron's
//! state-consistency family lands (vsdd supplies schemas, mdatron
//! executes them, per the boundary preamble).

pub mod frontmatter;
pub mod sets;

use std::path::Path;

use serde::de::DeserializeOwned;

use crate::diagnostics::Diagnostic;
use sets::{
    ActToAffordanceMap, CompositionScopeAndActions, DispatchData, EconomicsData,
    InstalledArtifactManifest, SnapshotSchemaSet, StateSchemaSet, StatuslineData,
};

/// All nine sets, loaded and validated.
#[derive(Debug)]
pub struct Registry {
    pub installed_artifact_manifest: InstalledArtifactManifest,
    pub state_schema: StateSchemaSet,
    pub composition_scope_and_actions: CompositionScopeAndActions,
    pub statusline_data: StatuslineData,
    pub gate_data: sets::GateData,
    pub dispatch_data: DispatchData,
    pub act_to_affordance_map: ActToAffordanceMap,
    pub economics_data: EconomicsData,
    pub snapshot_schema: SnapshotSchemaSet,
}

/// Load one set from `templates/registry/<class>.md`, validating the
/// frontmatter against its schema pair. A missing file, a frontmatter
/// parse failure (with location), a schema-pair violation, or a
/// `schema_class` mismatch each yield a diagnostic naming the file —
/// never a panic (the registry is adopter-edited).
pub fn load_set<T: DeserializeOwned>(repo_root: &Path, class: &str) -> Result<T, Box<Diagnostic>> {
    let _ = (repo_root, class);
    todo!("2b: frontmatter split, schema-pair validation, typed decode")
}

/// Load all nine sets from the repo.
pub fn load_all(repo_root: &Path) -> Result<Registry, Box<Diagnostic>> {
    let _ = repo_root;
    todo!("2b: nine load_set calls, first failure reported with its file")
}
