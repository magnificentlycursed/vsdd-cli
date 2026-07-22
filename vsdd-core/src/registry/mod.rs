//! The nine versioned data sets as loadable artifacts (Layer 1 of the
//! decomposition). Each loader validates its artifact against the
//! `.mdatron/schemas/<class>.json` pair at read — the trust boundary's
//! read-time validation, vsdd self-validating until mdatron's
//! state-consistency family lands (vsdd supplies schemas, mdatron
//! executes them, per the boundary preamble).
//!
//! Registry-surface diagnostics name files, locations, and repairs in
//! plain words. Their recovery action is the registered
//! `repair-registry-artifact` vocabulary member, carried here as a
//! declared bootstrap mirror: the loader cannot load the vocabulary
//! before loading — the permanent exception — so the constant below
//! mirrors the registered member and a fidelity test pins the two
//! together (operator registration 2026-07-21, vsdd-cli #724).

pub mod frontmatter;
pub mod sets;

use std::path::{Path, PathBuf};

use crate::schema_check::Schema;
use serde::de::DeserializeOwned;

use crate::diagnostics::{yaml_location, Diagnostic};
use sets::{
    ActToAffordanceMap, CompositionScopeAndActions, DispatchData, EconomicsData, GateData,
    InstalledArtifactManifest, SnapshotSchemaSet, StateSchemaSet, StatuslineData,
};

/// The registered recovery member this loader mirrors (vsdd-cli #724).
pub(crate) const REGISTRY_REPAIR_ACTION: &str = "repair-registry-artifact";

/// The registered member for schema-pair failures (vsdd-cli #739's
/// rider) — the same declared bootstrap mirror as its sibling above.
pub(crate) const SCHEMA_PAIR_RESTORE_ACTION: &str = "restore-schema-pair";

enum BoundedTextError {
    Io(std::io::Error),
    Oversize,
    NotUtf8(std::string::FromUtf8Error),
}

/// The loader's text reads ride the bounded reader (the #726 ruling's
/// capped reader, landed per vsdd-cli #732).
fn read_bounded_utf8(path: &Path) -> Result<String, BoundedTextError> {
    let bounded = crate::bounded_read::read_bounded(path).map_err(BoundedTextError::Io)?;
    if bounded.oversize {
        return Err(BoundedTextError::Oversize);
    }
    String::from_utf8(bounded.bytes).map_err(BoundedTextError::NotUtf8)
}

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

/// A failure in the adopter's artifact: the repair is theirs.
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
        recovery_action: REGISTRY_REPAIR_ACTION.to_string(),
        recovery_text: "repair the registry artifact to match its schema pair, then re-run"
            .to_string(),
    })
}

/// A failure in the schema pair itself: the artifact is not the broken
/// party, and the recovery names the pair file (vsdd-cli #727).
fn schema_pair_diagnostic(
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
        recovery_action: SCHEMA_PAIR_RESTORE_ACTION.to_string(),
        recovery_text:
            "restore or repair the schema pair file named in the diagnostic, then re-run — the artifact is not the broken party"
                .to_string(),
    })
}

/// Load one set from `templates/registry/<class>.md`, validating the
/// frontmatter against its schema pair. A missing file, a frontmatter
/// parse failure (with location), a `schema_class` mismatch, or a
/// schema-pair violation each yield a diagnostic naming the file —
/// never a panic (the registry is adopter-edited).
pub fn load_set<T: DeserializeOwned>(repo_root: &Path, class: &str) -> Result<T, Box<Diagnostic>> {
    // One safe path segment only: the traversal cannot exist even if a
    // future caller passes untrusted input (vsdd-cli #727).
    if class.is_empty()
        || !class
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        // A caller fault, not the adopter's artifact: the empty recovery
        // action is the explicit no-adopter-act signal (the #723 pattern;
        // vsdd-cli #734).
        return Err(Box::new(Diagnostic {
            file: repo_root.join("templates/registry"),
            kind: "invalid-class".to_string(),
            machine_token: "invalid-class".to_string(),
            location: None,
            message: format!(
                "registry class `{class}` is not a single lowercase path segment — a toolkit or caller defect, not an artifact defect"
            ),
            recovery_action: String::new(),
            recovery_text: String::new(),
        }));
    }

    let path = repo_root
        .join("templates/registry")
        .join(format!("{class}.md"));
    let text = read_bounded_utf8(&path).map_err(|e| match e {
        BoundedTextError::Io(e) => {
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
        }
        BoundedTextError::Oversize => artifact_diagnostic(
            path.clone(),
            "malformed",
            format!(
                "the artifact exceeds the reader's {} byte limit and was not parsed",
                crate::bounded_read::MAX_ARTIFACT_BYTES
            ),
            None,
        ),
        BoundedTextError::NotUtf8(e) => artifact_diagnostic(
            path.clone(),
            "malformed",
            format!("the artifact is not valid UTF-8: {e}"),
            None,
        ),
    })?;

    let (fm, _body) = frontmatter::split_frontmatter(&text).map_err(|e| {
        artifact_diagnostic(
            path.clone(),
            "malformed",
            e.message.clone(),
            Some((e.line, e.column)),
        )
    })?;
    // Frontmatter begins on line 2 of the file, hence the offset of 1
    // on parser locations relative to the frontmatter slice.
    let value: serde_yaml_ng::Value = serde_yaml_ng::from_str(fm).map_err(|e| {
        artifact_diagnostic(
            path.clone(),
            "malformed",
            format!("frontmatter parse failure: {e}"),
            yaml_location(&e, 1),
        )
    })?;

    match value.get("schema_class").and_then(|v| v.as_str()) {
        None => {
            return Err(artifact_diagnostic(
                path,
                "schema-class-mismatch",
                format!(
                    "the frontmatter has no string schema_class key; expected schema_class `{class}`"
                ),
                None,
            ));
        }
        Some(found) if found != class => {
            return Err(artifact_diagnostic(
                path,
                "schema-class-mismatch",
                format!("expected schema_class `{class}`, found `{found}`"),
                None,
            ));
        }
        Some(_) => {}
    }

    let schema_path = repo_root
        .join(".mdatron/schemas")
        .join(format!("{class}.json"));
    let schema_text = read_bounded_utf8(&schema_path).map_err(|e| {
        let (kind, detail) = match e {
            BoundedTextError::Io(e) => ("absent", format!("cannot read the schema pair: {e}")),
            BoundedTextError::Oversize => (
                "malformed",
                format!(
                    "the schema pair exceeds the reader's {} byte limit",
                    crate::bounded_read::MAX_ARTIFACT_BYTES
                ),
            ),
            BoundedTextError::NotUtf8(e) => (
                "malformed",
                format!("the schema pair is not valid UTF-8: {e}"),
            ),
        };
        schema_pair_diagnostic(schema_path.clone(), kind, detail, None)
    })?;
    let schema_json: serde_json::Value = serde_json::from_str(&schema_text).map_err(|e| {
        schema_pair_diagnostic(
            schema_path.clone(),
            "malformed",
            format!("the schema pair is not valid JSON: {e}"),
            Some((e.line(), e.column())),
        )
    })?;
    let schema = Schema::compile(&schema_json).map_err(|e| {
        schema_pair_diagnostic(
            schema_path.clone(),
            "malformed",
            format!("the schema pair does not compile: {e}"),
            None,
        )
    })?;
    let violations = schema.validate(&value);
    if !violations.is_empty() {
        let rendered: Vec<String> = violations
            .iter()
            .map(|v| {
                let at = if v.instance_path.is_empty() {
                    "root"
                } else {
                    v.instance_path.as_str()
                };
                format!("{at}: {}", v.message)
            })
            .collect();
        return Err(artifact_diagnostic(
            path,
            "schema-violation",
            format!(
                "the artifact violates its schema pair:\n  {}",
                rendered.join("\n  ")
            ),
            None,
        ));
    }

    // The artifact just PASSED its pair, so a decode failure here is a
    // disagreement between the toolkit's typed model and the pair — a
    // toolkit defect, never the adopter's artifact (vsdd-cli #723). No
    // registered recovery action applies: the empty action is the
    // explicit no-adopter-act-exists signal.
    serde_yaml_ng::from_str::<T>(fm).map_err(|e| {
        Box::new(Diagnostic {
            file: path,
            kind: "loader-pair-disagreement".to_string(),
            machine_token: "loader-pair-disagreement".to_string(),
            location: yaml_location(&e, 1),
            message: format!(
                "the artifact passed its schema pair but the toolkit's typed model refused it: {e} — this is a toolkit defect (the typed model and the pair disagree), not an artifact defect; report it upstream"
            ),
            recovery_action: String::new(),
            recovery_text: String::new(),
        })
    })
}

/// Load all nine sets from the repo. One pass names every failing
/// artifact — a bulk defect (an editor re-encoding, a wrong repo root)
/// reports all its casualties at once rather than one per re-run
/// (vsdd-cli #727).
pub fn load_all(repo_root: &Path) -> Result<Registry, Vec<Diagnostic>> {
    fn keep<T>(
        slot: &mut Option<T>,
        result: Result<T, Box<Diagnostic>>,
        errors: &mut Vec<Diagnostic>,
    ) {
        match result {
            Ok(v) => *slot = Some(v),
            Err(e) => errors.push(*e),
        }
    }

    let mut errors = Vec::new();
    let mut manifest = None;
    let mut state_schema = None;
    let mut composition = None;
    let mut statusline = None;
    let mut gate = None;
    let mut dispatch = None;
    let mut map = None;
    let mut economics = None;
    let mut snapshot = None;

    keep(
        &mut manifest,
        load_set(repo_root, "installed-artifact-manifest"),
        &mut errors,
    );
    keep(
        &mut state_schema,
        load_set(repo_root, "state-schema"),
        &mut errors,
    );
    keep(
        &mut composition,
        load_set(repo_root, "composition-scope-and-actions"),
        &mut errors,
    );
    keep(
        &mut statusline,
        load_set(repo_root, "statusline-data"),
        &mut errors,
    );
    keep(&mut gate, load_set(repo_root, "gate-data"), &mut errors);
    keep(
        &mut dispatch,
        load_set(repo_root, "dispatch-data"),
        &mut errors,
    );
    keep(
        &mut map,
        load_set(repo_root, "act-to-affordance-map"),
        &mut errors,
    );
    keep(
        &mut economics,
        load_set(repo_root, "economics-data"),
        &mut errors,
    );
    keep(
        &mut snapshot,
        load_set(repo_root, "snapshot-schema"),
        &mut errors,
    );

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(Registry {
        installed_artifact_manifest: manifest.expect("checked by errors gate"),
        state_schema: state_schema.expect("checked by errors gate"),
        composition_scope_and_actions: composition.expect("checked by errors gate"),
        statusline_data: statusline.expect("checked by errors gate"),
        gate_data: gate.expect("checked by errors gate"),
        dispatch_data: dispatch.expect("checked by errors gate"),
        act_to_affordance_map: map.expect("checked by errors gate"),
        economics_data: economics.expect("checked by errors gate"),
        snapshot_schema: snapshot.expect("checked by errors gate"),
    })
}
