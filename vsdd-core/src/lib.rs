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
    //! Each constant is a `&[(filename, content)]` slice. `vsdd_core::init` deploys each
    //! entry to its conventional location:
    //!   PHASE_PRIMERS  -> `<project>/.claude/commands/<filename>`
    //!   DOMAIN_PROMPTS -> `<project>/.claude/commands/<filename>`
    //!   SUPPLEMENTS    -> `<project>/supplements/<filename>`

    /// Phase primer markdown files (10 entries).
    pub const PHASE_PRIMERS: &[(&str, &str)] = &[
        ("vsdd-phase-1a.md", include_str!("../../.claude/commands/vsdd-phase-1a.md")),
        ("vsdd-phase-1b.md", include_str!("../../.claude/commands/vsdd-phase-1b.md")),
        ("vsdd-phase-1c.md", include_str!("../../.claude/commands/vsdd-phase-1c.md")),
        ("vsdd-phase-2a.md", include_str!("../../.claude/commands/vsdd-phase-2a.md")),
        ("vsdd-phase-2b.md", include_str!("../../.claude/commands/vsdd-phase-2b.md")),
        ("vsdd-phase-2c.md", include_str!("../../.claude/commands/vsdd-phase-2c.md")),
        ("vsdd-phase-3.md", include_str!("../../.claude/commands/vsdd-phase-3.md")),
        ("vsdd-phase-4.md", include_str!("../../.claude/commands/vsdd-phase-4.md")),
        ("vsdd-phase-5.md", include_str!("../../.claude/commands/vsdd-phase-5.md")),
        ("vsdd-phase-6.md", include_str!("../../.claude/commands/vsdd-phase-6.md")),
    ];

    /// Domain prompt markdown files (18 entries).
    pub const DOMAIN_PROMPTS: &[(&str, &str)] = &[
        ("vsdd-domain-accessibility.md", include_str!("../../.claude/commands/vsdd-domain-accessibility.md")),
        ("vsdd-domain-ai-engineer.md", include_str!("../../.claude/commands/vsdd-domain-ai-engineer.md")),
        ("vsdd-domain-data-engineer.md", include_str!("../../.claude/commands/vsdd-domain-data-engineer.md")),
        ("vsdd-domain-documentation-reviewer.md", include_str!("../../.claude/commands/vsdd-domain-documentation-reviewer.md")),
        ("vsdd-domain-localization.md", include_str!("../../.claude/commands/vsdd-domain-localization.md")),
        ("vsdd-domain-performance-engineer.md", include_str!("../../.claude/commands/vsdd-domain-performance-engineer.md")),
        ("vsdd-domain-platform-engineer.md", include_str!("../../.claude/commands/vsdd-domain-platform-engineer.md")),
        ("vsdd-domain-privacy.md", include_str!("../../.claude/commands/vsdd-domain-privacy.md")),
        ("vsdd-domain-quality-engineer.md", include_str!("../../.claude/commands/vsdd-domain-quality-engineer.md")),
        ("vsdd-domain-red-team.md", include_str!("../../.claude/commands/vsdd-domain-red-team.md")),
        ("vsdd-domain-sanity-check.md", include_str!("../../.claude/commands/vsdd-domain-sanity-check.md")),
        ("vsdd-domain-security.md", include_str!("../../.claude/commands/vsdd-domain-security.md")),
        ("vsdd-domain-software-engineer.md", include_str!("../../.claude/commands/vsdd-domain-software-engineer.md")),
        ("vsdd-domain-solution-architect.md", include_str!("../../.claude/commands/vsdd-domain-solution-architect.md")),
        ("vsdd-domain-solution-owner.md", include_str!("../../.claude/commands/vsdd-domain-solution-owner.md")),
        ("vsdd-domain-technical-writer.md", include_str!("../../.claude/commands/vsdd-domain-technical-writer.md")),
        ("vsdd-domain-ux.md", include_str!("../../.claude/commands/vsdd-domain-ux.md")),
        ("vsdd-domain-vsdd-methodology.md", include_str!("../../.claude/commands/vsdd-domain-vsdd-methodology.md")),
    ];

    /// Supplement markdown files (14 entries).
    pub const SUPPLEMENTS: &[(&str, &str)] = &[
        ("bash.md", include_str!("../../supplements/bash.md")),
        ("browser-app.md", include_str!("../../supplements/browser-app.md")),
        ("claude-code-cli.md", include_str!("../../supplements/claude-code-cli.md")),
        ("cli.md", include_str!("../../supplements/cli.md")),
        ("css.md", include_str!("../../supplements/css.md")),
        ("github-actions.md", include_str!("../../supplements/github-actions.md")),
        ("html.md", include_str!("../../supplements/html.md")),
        ("javascript-typescript.md", include_str!("../../supplements/javascript-typescript.md")),
        ("json.md", include_str!("../../supplements/json.md")),
        ("markdown.md", include_str!("../../supplements/markdown.md")),
        ("python.md", include_str!("../../supplements/python.md")),
        ("rust.md", include_str!("../../supplements/rust.md")),
        ("toml.md", include_str!("../../supplements/toml.md")),
        ("yaml.md", include_str!("../../supplements/yaml.md")),
    ];
}

pub mod init;
