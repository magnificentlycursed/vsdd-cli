//! Project bootstrap: three-way drift classification + template deployment +
//! idempotent re-init (Slice 3, Install; `.design/install-slice.md`).
//!
//! Pipeline:
//!   1. refuse a non-git directory (PE tightening; REQ-9)
//!   2. classify each managed artifact three-way from the triple
//!      (recorded-manifest sha × current-disk sha × new-template sha) into
//!      Unchanged / ToolkitUpgrade / Conflict / Missing — mirroring crosslink's
//!      `classify_update` (REQ-1)
//!   3. act per the classification, the per-artifact management class (REQ-16),
//!      and the flag surface (`--force` / `--update` / `--no-prompt` /
//!      `--dry-run` / per-file resolutions; REQ-2..6)
//!   4. deploy the 15 `templates/*` artifacts (REQ-11/13) and GENERATE the
//!      adopter's `.vsdd/registry/installed-artifact-manifest.md` from the
//!      deployed set (REQ-15)
//!   5. write `init-manifest.json` LAST (re-run-to-converge atomicity; REQ-10)
//!   6. emit `ProjectInitialized` on first init only (REQ-8)
//!
//! The core is deterministic and non-interactive: it never reads stdin. The
//! interactive Conflict prompt (REQ-6) lives in the binary, which discovers
//! conflicts via [`plan_conflicts`], prompts per file, and re-invokes with the
//! operator's [`ConflictChoice`]s in [`InitOptions::resolved_conflicts`].

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{artifacts, patterns, schemas};

/// Toolkit version stamped into deployed artifacts and manifest entries.
const VSDD_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The section-managed region delimiters (REQ-16). The tool owns everything up
/// to the End anchor; the operator owns the extension region below it.
const MANAGED_START_ANCHOR: &str = "# === vsdd managed ===";
const MANAGED_END_ANCHOR: &str = "# === End vsdd managed ===";

/// Caller-provided knobs for the deployment.
#[derive(Debug, Default)]
pub struct InitOptions {
    /// CI bootstrap mode: skip operator prompts; use defaults. Implies
    /// `--no-prompt` (REQ-5).
    pub ci_mode: bool,

    /// `--force`: overwrite Conflict (operator-edited) files (REQ-3).
    pub force: bool,
    /// `--update`: apply ToolkitUpgrade files only (REQ-4).
    pub update: bool,
    /// `--no-prompt`: non-interactive; skip Conflict files unless `force` (REQ-5).
    pub no_prompt: bool,
    /// `--dry-run`: print the plan and write nothing (REQ-2).
    pub dry_run: bool,
    /// The operator's per-file Conflict resolutions — the non-interactive
    /// representation of the interactive prompt's choices (REQ-6). Keyed by the
    /// artifact's project-relative destination path.
    pub resolved_conflicts: BTreeMap<String, ConflictChoice>,
}

/// The per-file three-way classification outcome (REQ-1 / AC-1), mirroring
/// crosslink's `classify_update`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Classification {
    /// All three hashes equal — skip.
    Unchanged,
    /// disk == manifest != template — update.
    ToolkitUpgrade,
    /// disk != manifest — the operator-edited case; do not silently overwrite.
    Conflict,
    /// Destination absent — deploy.
    Missing,
}

/// The operator's resolution of a single Conflict file (REQ-6) — the
/// non-interactive representation of the interactive prompt's per-file choice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictChoice {
    /// Keep the operator's edit; leave the file byte-unchanged.
    KeepOperatorEdit,
    /// Accept the new template; overwrite the file.
    AcceptNewTemplate,
}

/// Classify a managed file from the triple `(recorded manifest sha, current
/// disk sha, new-template sha)` — REQ-1 / AC-1.
///
/// This is the sha-first migration point (REQ-7): a pre-field manifest entry
/// simply supplies its recorded `sha256` as `manifest_sha`, so a matching disk
/// hash adopts (Unchanged/ToolkitUpgrade) and a differing one is a Conflict
/// (never a silent overwrite). A file present on disk with no manifest entry
/// (`manifest_sha == None`, e.g. a pre-existing unmanaged file at a template
/// destination) is a Conflict unless it already equals the template (REQ-9).
pub fn classify(
    manifest_sha: Option<&str>,
    disk_sha: Option<&str>,
    template_sha: &str,
) -> Classification {
    match disk_sha {
        None => Classification::Missing,
        Some(disk) => match manifest_sha {
            Some(recorded) if recorded == disk => {
                if disk == template_sha {
                    Classification::Unchanged
                } else {
                    Classification::ToolkitUpgrade
                }
            }
            Some(_) => Classification::Conflict,
            None => {
                if disk == template_sha {
                    Classification::Unchanged
                } else {
                    Classification::Conflict
                }
            }
        },
    }
}

/// Per-run deployment outcome.
#[derive(Debug)]
pub struct InitReport {
    /// Files written (or rewritten) on this invocation.
    pub deployed: Vec<PathBuf>,
    /// Files left untouched because their classification prescribed no write.
    pub skipped: Vec<PathBuf>,
    /// Path to the canonical `<project>/.vsdd/init-manifest.json`.
    pub manifest_path: PathBuf,
}

/// A Conflict the interactive layer (the binary) must resolve — surfaced by
/// [`plan_conflicts`] so the caller can prompt per file (REQ-6).
#[derive(Debug, Clone)]
pub struct ConflictInfo {
    /// Project-relative destination path (the [`InitOptions::resolved_conflicts`] key).
    pub rel_path: String,
    /// Absolute destination path on disk.
    pub dest: PathBuf,
    /// The current template bytes (for a diff against the operator's copy).
    pub template: Vec<u8>,
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

/// The per-artifact management class (REQ-16 / D3): how REQ-1's classification
/// applies to a deployed artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ManagementClass {
    /// Drift-tracked whole-file under the three-way classification.
    Managed,
    /// Deploy-if-absent; never overwritten once present; exempt from
    /// ToolkitUpgrade/Conflict.
    Scaffold,
    /// Only the tool-owned region above the End anchor is drift-tracked; the
    /// operator-extension region below it is preserved.
    SectionManaged,
}

/// One deployment-plan entry: a destination, its bytes, and its class.
struct PlanEntry {
    rel: String,
    bytes: Vec<u8>,
    class: ManagementClass,
}

/// The action decided for one plan entry (pure; no IO).
enum Decision {
    /// Write the template bytes verbatim.
    DeployTemplate,
    /// Write these bytes (a section-managed merge of template + operator region).
    DeployBytes(Vec<u8>),
    /// No write; record the current-template manifest entry (Unchanged / present scaffold).
    Skip,
    /// No write; carry the prior manifest entry forward (skipped drift with a prior record).
    Preserve,
    /// No write; record nothing (a foreign file with no prior manifest entry).
    Omit,
    /// A managed-file drift that, in a non-interactive default run, is refused
    /// (`ManagedFileDrifted`) rather than silently overwritten.
    Drift,
}

/// Run the init pipeline against `project_root`. Returns an [`InitReport`] on success.
pub fn init(project_root: &Path, options: &InitOptions) -> Result<InitReport, InitError> {
    // Step 1: git-repository check on the deploy path itself.
    ensure_git_repo(project_root)?;

    // Step 2: load prior manifest. Absence means first init (drives event emission).
    let manifest_path = project_root.join(".vsdd/init-manifest.json");
    let prior_manifest = load_manifest(&manifest_path)?;
    let is_first_init = prior_manifest.is_none();

    // Step 3: build the deployment plan (62 artifacts: 47 bundled + 15 templates).
    let plan = build_deployment_plan();

    // Step 4a (dry-run): classify, print the plan, write nothing (REQ-2).
    if options.dry_run {
        for entry in &plan {
            let dest = project_root.join(&entry.rel);
            let disk_bytes = read_if_present(&dest)?;
            let prior_entry = prior_manifest
                .as_ref()
                .and_then(|m| m.files.get(entry.rel.as_str()));
            let template_sha = sha256_hex(&entry.bytes);
            let (cls, decision) = decide(
                entry.class,
                &entry.bytes,
                &template_sha,
                disk_bytes.as_deref(),
                prior_entry,
                &entry.rel,
                options,
            );
            println!(
                "vsdd init [dry-run] {}: {} -> {}",
                entry.rel,
                classification_word(cls),
                decision_word(&decision)
            );
        }
        return Ok(InitReport {
            deployed: Vec::new(),
            skipped: Vec::new(),
            manifest_path,
        });
    }

    // Step 4b: walk the plan; deploy / skip / preserve / refuse per the decision.
    let mut deployed: Vec<PathBuf> = Vec::new();
    let mut skipped: Vec<PathBuf> = Vec::new();
    let mut current_entries: BTreeMap<String, ManifestEntry> = BTreeMap::new();

    for entry in &plan {
        let dest = project_root.join(&entry.rel);
        let disk_bytes = read_if_present(&dest)?;
        let prior_entry = prior_manifest
            .as_ref()
            .and_then(|m| m.files.get(entry.rel.as_str()));
        let template_sha = sha256_hex(&entry.bytes);
        let (_cls, decision) = decide(
            entry.class,
            &entry.bytes,
            &template_sha,
            disk_bytes.as_deref(),
            prior_entry,
            &entry.rel,
            options,
        );

        match decision {
            Decision::DeployTemplate => {
                write_file(&dest, &entry.bytes)?;
                deployed.push(dest);
                current_entries.insert(entry.rel.clone(), managed_entry(&template_sha, entry.class));
            }
            Decision::DeployBytes(bytes) => {
                write_file(&dest, &bytes)?;
                deployed.push(dest);
                current_entries.insert(entry.rel.clone(), managed_entry(&template_sha, entry.class));
            }
            Decision::Skip => {
                skipped.push(dest);
                current_entries.insert(entry.rel.clone(), managed_entry(&template_sha, entry.class));
            }
            Decision::Preserve => {
                skipped.push(dest);
                let pe = prior_entry.expect("Decision::Preserve implies a prior manifest entry");
                current_entries.insert(entry.rel.clone(), preserve_entry(pe, entry.class));
            }
            Decision::Omit => {
                skipped.push(dest);
            }
            Decision::Drift => {
                let actual_sha256 = disk_bytes.as_deref().map(sha256_hex).unwrap_or_default();
                let expected_sha256 = prior_entry.map(|e| e.sha256.clone()).unwrap_or_default();
                return Err(InitError::ManagedFileDrifted {
                    path: dest,
                    expected_sha256,
                    actual_sha256,
                });
            }
        }
    }

    // Step 4c: generate the adopter's installed-artifact manifest from the
    // deployed set (REQ-15). Not a deployed-template manifest entry; written
    // only when its (deterministic) content changes, so a converged re-run
    // rewrites nothing (REQ-8).
    let generated_path = project_root.join(".vsdd/registry/installed-artifact-manifest.md");
    let generated = generate_installed_manifest(&plan);
    write_if_changed(&generated_path, generated.as_bytes())?;

    // Step 5: ensure the `.vsdd/` skeleton (events.jsonl + config.yaml).
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

    // Step 6: write the manifest LAST (REQ-10), and only if it actually changes (REQ-8).
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

    // Step 7: emit ProjectInitialized on first init only (REQ-8). The count is
    // the plan size (62): the 15 templates are counted; the generated manifest
    // is not a deployed template.
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

/// Discover the Conflicts a run would face, without writing anything — the
/// binary's interactive layer prompts over these and re-invokes [`init`] with
/// the operator's [`ConflictChoice`]s (REQ-6).
pub fn plan_conflicts(
    project_root: &Path,
    options: &InitOptions,
) -> Result<Vec<ConflictInfo>, InitError> {
    ensure_git_repo(project_root)?;
    let manifest_path = project_root.join(".vsdd/init-manifest.json");
    let prior_manifest = load_manifest(&manifest_path)?;
    let plan = build_deployment_plan();

    let mut out = Vec::new();
    for entry in &plan {
        let dest = project_root.join(&entry.rel);
        let disk_bytes = read_if_present(&dest)?;
        let prior_entry = prior_manifest
            .as_ref()
            .and_then(|m| m.files.get(entry.rel.as_str()));
        let template_sha = sha256_hex(&entry.bytes);
        let (cls, _decision) = decide(
            entry.class,
            &entry.bytes,
            &template_sha,
            disk_bytes.as_deref(),
            prior_entry,
            &entry.rel,
            options,
        );
        if matches!(cls, Classification::Conflict) {
            out.push(ConflictInfo {
                rel_path: entry.rel.clone(),
                dest,
                template: entry.bytes.clone(),
            });
        }
    }
    Ok(out)
}

// ── Decision logic (pure) ────────────────────────────────────────────────────

/// Decide the action for one plan entry from its class, the classification, and
/// the flag surface. Pure: no IO, no stdin, no error — the caller turns a
/// [`Decision::Drift`] into `ManagedFileDrifted` (non-dry-run) or a printed line
/// (dry-run).
fn decide(
    class: ManagementClass,
    template_bytes: &[u8],
    template_sha: &str,
    disk_bytes: Option<&[u8]>,
    prior_entry: Option<&ManifestEntry>,
    rel: &str,
    options: &InitOptions,
) -> (Classification, Decision) {
    match class {
        ManagementClass::Scaffold => match disk_bytes {
            None => (Classification::Missing, Decision::DeployTemplate),
            // Deploy-if-absent: a present scaffold is left untouched and exempt
            // from ToolkitUpgrade/Conflict (REQ-16).
            Some(_) => (Classification::Unchanged, Decision::Skip),
        },

        ManagementClass::Managed => {
            let disk_sha = disk_bytes.map(sha256_hex);
            let cls = classify(
                prior_entry.map(|e| e.sha256.as_str()),
                disk_sha.as_deref(),
                template_sha,
            );
            let decision = match cls {
                Classification::Missing => Decision::DeployTemplate,
                Classification::Unchanged => Decision::Skip,
                Classification::ToolkitUpgrade => {
                    if options.update || options.force {
                        Decision::DeployTemplate
                    } else {
                        preserve_or_omit(prior_entry)
                    }
                }
                Classification::Conflict => {
                    resolve_conflict(options, rel, prior_entry, Decision::DeployTemplate)
                }
            };
            (cls, decision)
        }

        ManagementClass::SectionManaged => match disk_bytes {
            None => (Classification::Missing, Decision::DeployTemplate),
            Some(disk) => {
                let disk_str = String::from_utf8_lossy(disk);
                let tmpl_str = String::from_utf8_lossy(template_bytes);
                // The tool owns only the region above the End anchor; an
                // operator extension below it is not drift (REQ-16).
                if managed_region(disk_str.as_ref()) == managed_region(tmpl_str.as_ref()) {
                    (Classification::Unchanged, Decision::Skip)
                } else {
                    let merged = merge_section(tmpl_str.as_ref(), disk_str.as_ref());
                    let decision =
                        resolve_conflict(options, rel, prior_entry, Decision::DeployBytes(merged));
                    (Classification::Conflict, decision)
                }
            }
        },
    }
}

/// The flag-gated resolution of a Conflict. `accept` is the write to perform on
/// `--force` or an `AcceptNewTemplate` resolution.
fn resolve_conflict(
    options: &InitOptions,
    rel: &str,
    prior_entry: Option<&ManifestEntry>,
    accept: Decision,
) -> Decision {
    if options.force {
        return accept;
    }
    if let Some(choice) = options.resolved_conflicts.get(rel) {
        return match choice {
            ConflictChoice::AcceptNewTemplate => accept,
            ConflictChoice::KeepOperatorEdit => preserve_or_omit(prior_entry),
        };
    }
    if options.no_prompt || options.ci_mode {
        return preserve_or_omit(prior_entry);
    }
    // Default, non-interactive core: a drifted managed file with a prior record
    // is refused (`ManagedFileDrifted`); a foreign file with no record is left
    // untouched (REQ-9). The interactive prompt is the binary's job.
    if prior_entry.is_some() {
        Decision::Drift
    } else {
        Decision::Omit
    }
}

fn preserve_or_omit(prior_entry: Option<&ManifestEntry>) -> Decision {
    if prior_entry.is_some() {
        Decision::Preserve
    } else {
        Decision::Omit
    }
}

// ── Manifest entry construction ──────────────────────────────────────────────

fn section_anchors() -> Option<Vec<String>> {
    Some(vec![
        MANAGED_START_ANCHOR.to_string(),
        MANAGED_END_ANCHOR.to_string(),
    ])
}

fn anchors_for(class: ManagementClass) -> Option<Vec<String>> {
    match class {
        ManagementClass::SectionManaged => section_anchors(),
        _ => None,
    }
}

/// The manifest entry for a freshly deployed / unchanged file: the current
/// template's whole-file sha and the current toolkit version (REQ-7).
fn managed_entry(template_sha: &str, class: ManagementClass) -> ManifestEntry {
    ManifestEntry {
        sha256: template_sha.to_string(),
        template_version_at_deploy: VSDD_VERSION.to_string(),
        managed_section_anchors: anchors_for(class),
    }
}

/// Carry a prior entry forward for a skipped drift, backfilling
/// `template_version_at_deploy` when migrating a pre-field manifest (REQ-7).
fn preserve_entry(prior: &ManifestEntry, class: ManagementClass) -> ManifestEntry {
    let version = if prior.template_version_at_deploy.is_empty() {
        VSDD_VERSION.to_string()
    } else {
        prior.template_version_at_deploy.clone()
    };
    ManifestEntry {
        sha256: prior.sha256.clone(),
        template_version_at_deploy: version,
        managed_section_anchors: anchors_for(class),
    }
}

// ── Section-managed helpers ──────────────────────────────────────────────────

/// The tool-owned region: everything above the End anchor (or the whole content
/// when the anchor is absent).
fn managed_region(content: &str) -> &str {
    match content.find(MANAGED_END_ANCHOR) {
        Some(idx) => &content[..idx],
        None => content,
    }
}

/// Merge the fresh template's managed region with the operator's extension
/// region (everything from the End anchor onward). Falls back to the full
/// template when either side lacks the anchor.
fn merge_section(template: &str, disk: &str) -> Vec<u8> {
    match (template.find(MANAGED_END_ANCHOR), disk.find(MANAGED_END_ANCHOR)) {
        (Some(ti), Some(di)) => format!("{}{}", &template[..ti], &disk[di..]).into_bytes(),
        _ => template.as_bytes().to_vec(),
    }
}

// ── Generated installed-artifact manifest (REQ-15) ───────────────────────────

/// Generate the adopter's `installed-artifact-manifest.md` deterministically
/// over the deployed set. It enumerates the deployed artifacts that belong to
/// the installed-artifact-manifest closed-world class vocabulary (the vsdd
/// command set as `command-listing`, the statusline script as
/// `statusline-wiring`); the toolkit's schema, pattern, registry-data, and
/// supplement payloads are drift-tracked by `init-manifest.json` and their
/// schema pairs, not this environment-integrity manifest. Conforms to the
/// `installed-artifact-manifest` schema (REQ-15 / AC-15).
fn generate_installed_manifest(plan: &[PlanEntry]) -> String {
    let mut entries = String::new();
    for e in plan {
        if let Some(name) = e.rel.strip_prefix(".claude/commands/") {
            let id = name.strip_suffix(".md").unwrap_or(name);
            entries.push_str(&format!(
                "  - {{id: {id}, path: {rel}, class: command-listing, source: vsdd-source, \
                 lifetime: tracked-payload, referenced_by: [command-listing], pairs_with: [], \
                 resolution: exists, fail_mode: undefined}}\n",
                rel = e.rel
            ));
        }
    }
    entries.push_str(
        "  - {id: statusline-script, path: .vsdd/statusline/vsdd-statusline.sh, \
         class: statusline-wiring, source: vsdd-source, lifetime: tracked-payload, \
         referenced_by: [statusline-command-path], pairs_with: [], resolution: exists, \
         fail_mode: fail-open-guarded}\n",
    );

    format!(
        "---\n\
schema_class: installed-artifact-manifest\n\
schema_version: 0.3.1\n\
status: draft-proposal\n\
reference_surfaces:\n\
  - {{id: command-listing, path: .claude/commands/, scope: repo}}\n\
  - {{id: statusline-command-path, path: settings statusLine entry, scope: repo-or-host}}\n\
entries:\n\
{entries}\
---\n\
\n\
# Installed-artifact manifest — generated by `vsdd init`\n\
\n\
Generated by `vsdd init` from the artifacts it deployed into this project. It\n\
enumerates the deployed artifacts that belong to the installed-artifact-manifest\n\
closed-world class vocabulary (the vsdd command set and the statusline script);\n\
the toolkit's schema, pattern, registry-data, and supplement payloads are\n\
drift-tracked by `.vsdd/init-manifest.json` and their schema pairs instead.\n\
Regenerated whenever the deployed set changes; not hand-edited.\n"
    )
}

// ── Deployment plan ──────────────────────────────────────────────────────────

/// Build the (relative-path, content-bytes, management-class) deployment plan:
/// 47 bundled artifacts + 15 `templates/*` files (REQ-11).
fn build_deployment_plan() -> Vec<PlanEntry> {
    let mut plan: Vec<PlanEntry> = Vec::new();

    let managed = |rel: &str, bytes: &[u8]| PlanEntry {
        rel: rel.to_string(),
        bytes: bytes.to_vec(),
        class: ManagementClass::Managed,
    };

    // Schemas (4) + pattern (1) — managed whole-file.
    plan.push(managed(
        ".mdatron/schemas/phase-primer.json",
        schemas::PHASE_PRIMER.as_bytes(),
    ));
    plan.push(managed(
        ".mdatron/schemas/domain-prompt.json",
        schemas::DOMAIN_PROMPT.as_bytes(),
    ));
    plan.push(managed(
        ".mdatron/schemas/supplement.json",
        schemas::SUPPLEMENT.as_bytes(),
    ));
    plan.push(managed(
        ".mdatron/schemas/review-entry.json",
        schemas::REVIEW_ENTRY.as_bytes(),
    ));
    plan.push(managed(
        ".mdatron/patterns/cross-references.yaml",
        patterns::CROSS_REFERENCES.as_bytes(),
    ));

    // Phase primers (10) + domain prompts (18) → .claude/commands/ — managed.
    for (name, content) in artifacts::PHASE_PRIMERS {
        plan.push(managed(
            &format!(".claude/commands/{name}"),
            content.as_bytes(),
        ));
    }
    for (name, content) in artifacts::DOMAIN_PROMPTS {
        plan.push(managed(
            &format!(".claude/commands/{name}"),
            content.as_bytes(),
        ));
    }
    // Supplements (14) → supplements/ — managed.
    for (name, content) in artifacts::SUPPLEMENTS {
        plan.push(managed(&format!("supplements/{name}"), content.as_bytes()));
    }

    // ── The 15 `templates/*` artifacts (REQ-11/13/16). ──────────────────────

    // 2 CI workflows — managed.
    plan.push(managed(
        ".github/workflows/vsdd-verify.yml",
        include_bytes!("../../templates/.github/workflows/vsdd-verify.yml"),
    ));
    plan.push(managed(
        ".github/workflows/vsdd-observe-pr-body.yml",
        include_bytes!("../../templates/.github/workflows/vsdd-observe-pr-body.yml"),
    ));

    // DESIGN scaffold — deploy-if-absent (REQ-16).
    plan.push(PlanEntry {
        rel: "DESIGN.md".to_string(),
        bytes: include_bytes!("../../templates/DESIGN.md.vsdd-template").to_vec(),
        class: ManagementClass::Scaffold,
    });

    // Statusline script — managed.
    plan.push(managed(
        ".vsdd/statusline/vsdd-statusline.sh",
        include_bytes!("../../templates/statusline/vsdd-statusline.sh"),
    ));

    // 8 `.md` registry data sets — managed whole-file.
    plan.push(managed(
        ".vsdd/registry/act-to-affordance-map.md",
        include_bytes!("../../templates/registry/act-to-affordance-map.md"),
    ));
    plan.push(managed(
        ".vsdd/registry/composition-scope-and-actions.md",
        include_bytes!("../../templates/registry/composition-scope-and-actions.md"),
    ));
    plan.push(managed(
        ".vsdd/registry/dispatch-data.md",
        include_bytes!("../../templates/registry/dispatch-data.md"),
    ));
    plan.push(managed(
        ".vsdd/registry/economics-data.md",
        include_bytes!("../../templates/registry/economics-data.md"),
    ));
    plan.push(managed(
        ".vsdd/registry/gate-data.md",
        include_bytes!("../../templates/registry/gate-data.md"),
    ));
    plan.push(managed(
        ".vsdd/registry/snapshot-schema.md",
        include_bytes!("../../templates/registry/snapshot-schema.md"),
    ));
    plan.push(managed(
        ".vsdd/registry/state-schema.md",
        include_bytes!("../../templates/registry/state-schema.md"),
    ));
    plan.push(managed(
        ".vsdd/registry/statusline-data.md",
        include_bytes!("../../templates/registry/statusline-data.md"),
    ));

    // 3 `.yaml` registry files — section-managed (REQ-16).
    let section_managed = |rel: &str, bytes: &[u8]| PlanEntry {
        rel: rel.to_string(),
        bytes: bytes.to_vec(),
        class: ManagementClass::SectionManaged,
    };
    plan.push(section_managed(
        ".vsdd/registry/anonymization-patterns.yaml",
        include_bytes!("../../templates/registry/anonymization-patterns.yaml"),
    ));
    plan.push(section_managed(
        ".vsdd/registry/canonical-patterns.yaml",
        include_bytes!("../../templates/registry/canonical-patterns.yaml"),
    ));
    plan.push(section_managed(
        ".vsdd/registry/vocabulary.yaml",
        include_bytes!("../../templates/registry/vocabulary.yaml"),
    ));

    plan
}

// ── IO + hashing helpers ─────────────────────────────────────────────────────

fn ensure_git_repo(project_root: &Path) -> Result<(), InitError> {
    if project_root.join(".git").exists() {
        Ok(())
    } else {
        Err(InitError::NotGitRepository {
            path: project_root.to_path_buf(),
        })
    }
}

fn read_if_present(path: &Path) -> Result<Option<Vec<u8>>, InitError> {
    if path.exists() {
        Ok(Some(std::fs::read(path).map_err(|e| InitError::io(path, e))?))
    } else {
        Ok(None)
    }
}

fn write_file(dest: &Path, bytes: &[u8]) -> Result<(), InitError> {
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent).map_err(|e| InitError::io(parent, e))?;
    }
    std::fs::write(dest, bytes).map_err(|e| InitError::io(dest, e))
}

/// Write only when the destination's bytes differ (idempotence; REQ-8/REQ-15).
fn write_if_changed(dest: &Path, bytes: &[u8]) -> Result<(), InitError> {
    if let Ok(existing) = std::fs::read(dest) {
        if existing == bytes {
            return Ok(());
        }
    }
    write_file(dest, bytes)
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
            // Corrupt manifest: treat as first init (REQ-9), never a false drift.
            Err(_) => Ok(None),
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(InitError::io(path, e)),
    }
}

fn classification_word(cls: Classification) -> &'static str {
    match cls {
        Classification::Unchanged => "unchanged",
        Classification::ToolkitUpgrade => "toolkit-upgrade",
        Classification::Conflict => "conflict",
        Classification::Missing => "missing",
    }
}

fn decision_word(decision: &Decision) -> &'static str {
    match decision {
        Decision::DeployTemplate | Decision::DeployBytes(_) => "deploy",
        Decision::Skip => "skip",
        Decision::Preserve => "keep (recorded drift preserved)",
        Decision::Omit => "keep (unmanaged; not recorded)",
        Decision::Drift => "conflict (needs --force / --update / a resolution)",
    }
}

// ── Serde shapes ───────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
struct Manifest {
    vsdd_version: String,
    /// path-relative-to-project -> entry record.
    files: BTreeMap<String, ManifestEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ManifestEntry {
    sha256: String,
    /// The toolkit version whose template produced the deployed content (REQ-7).
    /// `#[serde(default)]` lets a pre-field manifest parse for the sha-first
    /// migration; the classifier backfills it on adopt.
    #[serde(default)]
    template_version_at_deploy: String,
    /// The section-managed region delimiters for a `section-managed` artifact
    /// (REQ-16); absent (skipped) for whole-file `managed`/`scaffold` entries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    managed_section_anchors: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct ProjectInitializedEvent<'a> {
    event: &'a str,
    vsdd_version: &'a str,
    deployed_artifact_count: usize,
}
