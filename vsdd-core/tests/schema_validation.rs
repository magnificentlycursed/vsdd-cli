//! Integration tests: vsdd-core's JSON Schemas validate the existing
//! primer / domain / supplement files in vsdd-cli/.claude/commands/ and supplements/.
//!
//! These are the load-bearing contracts that `mdatron-core::schema::Schema` consumes;
//! the tests confirm both the schemas and the existing files conform.

use std::path::Path;

use mdatron_core::{frontmatter, Schema};

fn load_schema(json_str: &str) -> Schema {
    let json: serde_json::Value =
        serde_json::from_str(json_str).expect("schema is valid JSON");
    Schema::compile(&json).expect("schema compiles")
}

fn frontmatter_of(path: &str) -> serde_yaml::Value {
    let full = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(path);
    let content = std::fs::read_to_string(&full)
        .unwrap_or_else(|e| panic!("read {}: {e}", full.display()));
    let parsed = frontmatter::parse(&content)
        .unwrap_or_else(|e| panic!("parse frontmatter at {}: {e}", full.display()));
    let (fm, _body) =
        parsed.unwrap_or_else(|| panic!("no frontmatter at {}", full.display()));
    fm
}

#[test]
fn phase_primer_schema_validates_phase_1a() {
    let schema = load_schema(vsdd_core::schemas::PHASE_PRIMER);
    let primer = frontmatter_of(".claude/commands/vsdd-phase-1a.md");
    let errors = schema.validate(&primer);
    assert!(
        errors.is_empty(),
        "phase-1a primer failed validation: {errors:?}"
    );
}

#[test]
fn phase_primer_schema_validates_phase_2b() {
    let schema = load_schema(vsdd_core::schemas::PHASE_PRIMER);
    let primer = frontmatter_of(".claude/commands/vsdd-phase-2b.md");
    let errors = schema.validate(&primer);
    assert!(
        errors.is_empty(),
        "phase-2b primer failed validation: {errors:?}"
    );
}

#[test]
fn domain_prompt_schema_validates_software_engineer() {
    let schema = load_schema(vsdd_core::schemas::DOMAIN_PROMPT);
    let domain = frontmatter_of(".claude/commands/vsdd-domain-software-engineer.md");
    let errors = schema.validate(&domain);
    assert!(
        errors.is_empty(),
        "software-engineer domain failed validation: {errors:?}"
    );
}

#[test]
fn domain_prompt_schema_validates_ai_engineer() {
    let schema = load_schema(vsdd_core::schemas::DOMAIN_PROMPT);
    let domain = frontmatter_of(".claude/commands/vsdd-domain-ai-engineer.md");
    let errors = schema.validate(&domain);
    assert!(
        errors.is_empty(),
        "ai-engineer domain failed validation: {errors:?}"
    );
}

#[test]
fn supplement_schema_validates_rust() {
    let schema = load_schema(vsdd_core::schemas::SUPPLEMENT);
    let supplement = frontmatter_of("supplements/rust.md");
    let errors = schema.validate(&supplement);
    assert!(
        errors.is_empty(),
        "rust supplement failed validation: {errors:?}"
    );
}

#[test]
fn supplement_schema_validates_json() {
    let schema = load_schema(vsdd_core::schemas::SUPPLEMENT);
    let supplement = frontmatter_of("supplements/json.md");
    let errors = schema.validate(&supplement);
    assert!(
        errors.is_empty(),
        "json supplement failed validation: {errors:?}"
    );
}

#[test]
fn review_entry_schema_validates_recent_ai_engineer_review() {
    let schema = load_schema(vsdd_core::schemas::REVIEW_ENTRY);
    let entry = frontmatter_of("review-log/2026-06-01-ai-engineer-naming.md");
    let errors = schema.validate(&entry);
    assert!(
        errors.is_empty(),
        "ai-engineer-naming review failed validation: {errors:?}"
    );
}

#[test]
fn review_entry_schema_rejects_invalid_source_enum() {
    let schema = load_schema(vsdd_core::schemas::REVIEW_ENTRY);
    let bad: serde_yaml::Value = serde_yaml::from_str(
        "schema_class: review-entry\n\
         schema_version: 1.0.0\n\
         review_number: 1\n\
         date: 2026-06-01\n\
         phase: phase-3\n\
         scope: x\n\
         lens: y\n\
         source: invented-source-not-in-enum\n\
         session_note: ''\n\
         model: claude-opus\n\
         execution_method: ''\n",
    )
    .unwrap();
    let errors = schema.validate(&bad);
    assert!(
        !errors.is_empty(),
        "invalid source enum should produce errors"
    );
}

#[test]
fn phase_primer_schema_rejects_missing_required_field() {
    let schema = load_schema(vsdd_core::schemas::PHASE_PRIMER);
    let bad: serde_yaml::Value = serde_yaml::from_str(
        "primer_id: vsdd-phase-2a\nphase: phase-2a\nversion: 0.1.0\n",
    )
    .unwrap();
    let errors = schema.validate(&bad);
    assert!(
        !errors.is_empty(),
        "missing required fields should produce errors"
    );
}
