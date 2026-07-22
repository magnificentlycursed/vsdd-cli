//! Project bootstrap: file emission + manifest + idempotent re-init.
//!
//! Implements the 9-step v0.1 scope agreed in the multi-domain review (SA + SO + SE
//! + PE + QE + DR consensus on Option A):
//!
//!   1. refuse a non-git directory (PE tightening)
//!   2. emit 4 schemas + 1 pattern + 10 phase primers + 18 domain prompts + 14 supplements
//!   3. create `.vsdd/` skeleton (events.jsonl + config.yaml)
//!   4. write init-manifest.json with sha256 per deployed file
//!   5. emit ProjectInitialized event (first init only)
//!   6. idempotent on unchanged state
//!   7. refuse drifted managed file with clear error (DR caveat)
//!
//! Templates deployment (step 6 in the original spec enumeration) is deferred to
//! a follow-up iteration — the Phase 2a Red Gate did not cover it.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Serialize;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{artifacts, patterns, schemas};

/// Toolkit version stamped into deployed artifacts.
const VSDD_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Caller-provided knobs for the deployment.
#[derive(Debug, Default)]
pub struct InitOptions {
    /// CI bootstrap mode: skip operator prompts; use defaults; produce
    /// CI-runtime-shaped outputs.
    pub ci_mode: bool,
}

/// Per-run deployment outcome.
#[derive(Debug)]
pub struct InitReport {
    /// Files written (or rewritten) on this invocation.
    pub deployed: Vec<PathBuf>,
    /// Files left untouched because their manifest hash already matched.
    pub skipped: Vec<PathBuf>,
    /// Path to the canonical `<project>/.vsdd/init-manifest.json`.
    pub manifest_path: PathBuf,
}

/// Errors arising during init orchestration. Distinct from per-file IO errors,
/// which are wrapped in [`InitError::Io`].
#[derive(Debug, Error)]
pub enum InitError {
    #[error("not a git repository: {path}; run `git init` first")]
    NotGitRepository { path: PathBuf },

    #[error(
        "managed file drifted at {path}; resolve with --keep-operator-edits or \
         --accept-managed-defaults (expected sha256 {expected_sha256}, got {actual_sha256})"
    )]
    ManagedFileDrifted {
        path: PathBuf,
        expected_sha256: String,
        actual_sha256: String,
    },

    #[error("io error at '{path}': {error}")]
    Io { path: PathBuf, error: String },
}

impl InitError {
    fn io(path: &Path, e: std::io::Error) -> Self {
        Self::Io {
            path: path.to_path_buf(),
            error: e.to_string(),
        }
    }
}

/// Run the init pipeline against `project_root`. Returns a [`InitReport`] on success.
pub fn init(project_root: &Path, options: &InitOptions) -> Result<InitReport, InitError> {
    let _ = options;

    // Step 1: PE-tightened git-repository check on the deploy path itself.
    let git_path = project_root.join(".git");
    if !git_path.exists() {
        return Err(InitError::NotGitRepository {
            path: project_root.to_path_buf(),
        });
    }

    // Step 2: load prior manifest. Absence means first init (drives event emission).
    let manifest_path = project_root.join(".vsdd/init-manifest.json");
    let prior_manifest = load_manifest(&manifest_path)?;
    let is_first_init = prior_manifest.is_none();

    // Step 3: build the deployment plan: (relative-path, content-bytes).
    let plan = build_deployment_plan();

    // Step 4: walk the plan, detecting drift, deploying changed files, recording hashes.
    let mut deployed: Vec<PathBuf> = Vec::new();
    let mut skipped: Vec<PathBuf> = Vec::new();
    let mut current_entries: BTreeMap<String, ManifestEntry> = BTreeMap::new();

    for (rel_path, source_bytes) in &plan {
        let dest = project_root.join(rel_path);
        let source_sha = sha256_hex(source_bytes);

        if dest.exists() {
            let actual_bytes = std::fs::read(&dest).map_err(|e| InitError::io(&dest, e))?;
            let actual_sha = sha256_hex(&actual_bytes);

            // DR caveat: if the prior manifest claimed a different hash than what's
            // on disk now, an operator edited a managed file outside the discipline.
            // Refuse rather than overwrite their work.
            if let Some(prior_entry) = prior_manifest
                .as_ref()
                .and_then(|m| m.files.get(rel_path.as_str()))
            {
                if prior_entry.sha256 != actual_sha {
                    return Err(InitError::ManagedFileDrifted {
                        path: dest,
                        expected_sha256: prior_entry.sha256.clone(),
                        actual_sha256: actual_sha,
                    });
                }
            }

            if actual_sha == source_sha {
                skipped.push(dest);
                current_entries.insert(rel_path.clone(), ManifestEntry { sha256: source_sha });
                continue;
            }
            // No drift but content differs from source = toolkit upgrade. Overwrite.
        }

        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).map_err(|e| InitError::io(parent, e))?;
        }
        std::fs::write(&dest, source_bytes).map_err(|e| InitError::io(&dest, e))?;
        deployed.push(dest);
        current_entries.insert(rel_path.clone(), ManifestEntry { sha256: source_sha });
    }

    // Step 5: ensure .vsdd/ skeleton (events.jsonl + config.yaml). Operator-owned;
    // not recorded in manifest, not subject to drift discipline.
    let events_path = project_root.join(".vsdd/events.jsonl");
    let config_path = project_root.join(".vsdd/config.yaml");
    if let Some(parent) = events_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| InitError::io(parent, e))?;
    }
    if !events_path.exists() {
        std::fs::write(&events_path, "").map_err(|e| InitError::io(&events_path, e))?;
    }
    if !config_path.exists() {
        let config = format!("vsdd_version: {VSDD_VERSION}\n");
        std::fs::write(&config_path, config).map_err(|e| InitError::io(&config_path, e))?;
    }

    // Step 6: write manifest only if its content actually changes.
    let manifest_value = Manifest {
        vsdd_version: VSDD_VERSION.to_string(),
        files: current_entries,
    };
    let manifest_json = serde_json::to_string_pretty(&manifest_value)
        .expect("Manifest serializes (no non-serializable fields)");
    let manifest_changed = match std::fs::read_to_string(&manifest_path) {
        Ok(existing) => existing.trim() != manifest_json.trim(),
        Err(_) => true,
    };
    if manifest_changed {
        if let Some(parent) = manifest_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| InitError::io(parent, e))?;
        }
        std::fs::write(&manifest_path, &manifest_json)
            .map_err(|e| InitError::io(&manifest_path, e))?;
        deployed.push(manifest_path.clone());
    }

    // Step 7: emit ProjectInitialized event on first init only. Subsequent runs
    // (idempotent or upgrade) don't append; that would inflate the audit trail.
    if is_first_init {
        let event = ProjectInitializedEvent {
            event: "ProjectInitialized",
            vsdd_version: VSDD_VERSION,
            deployed_artifact_count: plan.len(),
        };
        let line = format!(
            "{}\n",
            serde_json::to_string(&event)
                .expect("ProjectInitializedEvent serializes (no non-serializable fields)")
        );
        std::fs::write(&events_path, line).map_err(|e| InitError::io(&events_path, e))?;
        if !deployed.iter().any(|p| p == &events_path) {
            deployed.push(events_path);
        }
    }

    Ok(InitReport {
        deployed,
        skipped,
        manifest_path,
    })
}

// ── Helpers ────────────────────────────────────────────────────────────────────

/// Build the (relative-path, content-bytes) deployment plan from the bundled artifacts.
fn build_deployment_plan() -> Vec<(String, Vec<u8>)> {
    let mut plan: Vec<(String, Vec<u8>)> = vec![
        (
            ".mdatron/schemas/phase-primer.json".into(),
            schemas::PHASE_PRIMER.as_bytes().to_vec(),
        ),
        (
            ".mdatron/schemas/domain-prompt.json".into(),
            schemas::DOMAIN_PROMPT.as_bytes().to_vec(),
        ),
        (
            ".mdatron/schemas/supplement.json".into(),
            schemas::SUPPLEMENT.as_bytes().to_vec(),
        ),
        (
            ".mdatron/schemas/review-entry.json".into(),
            schemas::REVIEW_ENTRY.as_bytes().to_vec(),
        ),
        (
            ".mdatron/patterns/cross-references.yaml".into(),
            patterns::CROSS_REFERENCES.as_bytes().to_vec(),
        ),
    ];

    for (name, content) in artifacts::PHASE_PRIMERS {
        plan.push((
            format!(".claude/commands/{name}"),
            content.as_bytes().to_vec(),
        ));
    }
    for (name, content) in artifacts::DOMAIN_PROMPTS {
        plan.push((
            format!(".claude/commands/{name}"),
            content.as_bytes().to_vec(),
        ));
    }
    for (name, content) in artifacts::SUPPLEMENTS {
        plan.push((format!("supplements/{name}"), content.as_bytes().to_vec()));
    }

    plan
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn load_manifest(path: &Path) -> Result<Option<Manifest>, InitError> {
    match std::fs::read_to_string(path) {
        Ok(content) => match serde_json::from_str::<Manifest>(&content) {
            Ok(m) => Ok(Some(m)),
            // Corrupt manifest: treat as no prior init. Operator can recover by
            // running init fresh; refuse-on-drift only fires for legitimate
            // managed-file edits, not parse failures.
            Err(_) => Ok(None),
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(InitError::io(path, e)),
    }
}

// ── Serde shapes ───────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, serde::Deserialize)]
struct Manifest {
    vsdd_version: String,
    /// path-relative-to-project -> entry record. Nested rather than a flat
    /// `String` so future fields (`deployed_at`, `vsdd_version_at_deploy`,
    /// `managed_section_anchors`) can be added without breaking the format.
    files: BTreeMap<String, ManifestEntry>,
}

#[derive(Debug, Serialize, serde::Deserialize)]
struct ManifestEntry {
    sha256: String,
}

#[derive(Debug, Serialize)]
struct ProjectInitializedEvent<'a> {
    event: &'a str,
    vsdd_version: &'a str,
    deployed_artifact_count: usize,
}
