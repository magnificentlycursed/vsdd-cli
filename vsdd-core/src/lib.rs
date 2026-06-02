//! vsdd-core — library for the VSDD methodology toolkit.
//!
//! VSDD-specific JSON Schemas, error catalog data, and registry files live here.
//! Validation is performed by `mdatron-core` consuming this crate's data.
//!
//! Currently provides:
//! - `schemas::PHASE_PRIMER` — JSON Schema for vsdd-phase-*.md frontmatter
//! - `schemas::DOMAIN_PROMPT` — JSON Schema for vsdd-domain-*.md frontmatter
//! - `schemas::SUPPLEMENT` — JSON Schema for vsdd-cli/supplements/*.md frontmatter

pub mod schemas {
    //! Bundled VSDD JSON Schemas. Embedded at compile time via `include_str!`.

    /// JSON Schema for phase primer frontmatter (vsdd-phase-*.md files).
    pub const PHASE_PRIMER: &str = include_str!("../schemas/phase-primer.json");

    /// JSON Schema for domain prompt frontmatter (vsdd-domain-*.md files).
    pub const DOMAIN_PROMPT: &str = include_str!("../schemas/domain-prompt.json");

    /// JSON Schema for supplement frontmatter (vsdd-cli/supplements/*.md files).
    pub const SUPPLEMENT: &str = include_str!("../schemas/supplement.json");

    /// JSON Schema for review-entry frontmatter (review-log/<date>-<slug>.md files).
    pub const REVIEW_ENTRY: &str = include_str!("../schemas/review-entry.json");
}

pub mod patterns {
    //! Bundled VSDD mdatron-DSL patterns. Embedded at compile time via `include_str!`.

    /// Cross-file referential integrity rules across phase primers, domain prompts,
    /// supplements, and review entries.
    pub const CROSS_REFERENCES: &str = include_str!("../patterns/cross-references.yaml");
}

pub mod artifacts {
    //! Bundled deployable markdown artifacts (phase primers, domain prompts, supplements).
    //!
    //! Phase 2a Red Gate state: arrays are empty stubs. Phase 2b populates them with
    //! `include_str!` entries per artifact.

    /// Phase primer markdown files. Entries are `(filename, content)`; deployed by
    /// `vsdd_core::init` to `<project>/.claude/commands/<filename>`.
    pub const PHASE_PRIMERS: &[(&str, &str)] = &[];

    /// Domain prompt markdown files. Deployed to `<project>/.claude/commands/<filename>`.
    pub const DOMAIN_PROMPTS: &[(&str, &str)] = &[];

    /// Supplement markdown files. Deployed to `<project>/supplements/<filename>`.
    pub const SUPPLEMENTS: &[(&str, &str)] = &[];
}

pub mod init {
    //! Project bootstrap: file emission + manifest + idempotent re-init.
    //!
    //! Phase 2a Red Gate state: `init` is a `todo!()` stub. Phase 2b implements the
    //! 9-step v0.1 scope agreed in the multi-domain review (SA + SO + SE + PE + QE + DR).

    use std::path::{Path, PathBuf};

    use thiserror::Error;

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
        #[error("substrate is not a git repository: {path}; run `git init` first")]
        SubstrateNotGit { path: PathBuf },

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

    /// Run the init pipeline against `project_root`. Returns a [`InitReport`] on
    /// success.
    pub fn init(_project_root: &Path, _options: &InitOptions) -> Result<InitReport, InitError> {
        todo!("Phase 2b: implement the 9-step v0.1 init scope")
    }
}
