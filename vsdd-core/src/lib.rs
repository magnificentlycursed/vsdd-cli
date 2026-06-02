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
