//! Phase 2a Red Gate for vsdd-core/patterns/cross-references.yaml extensions.
//!
//! Each test builds a temp project laid out the way mdatron expects
//! (.mdatron/schemas + .mdatron/patterns + fixture markdown files), runs the
//! full mdatron_core::verify pipeline against it, and asserts that the
//! relevant finding code appears (or does not appear) for the fixture.
//!
//! These tests fail by default until the corresponding rules land in
//! vsdd-core/patterns/cross-references.yaml during the Phase 2b session.

use std::fs;
use std::path::PathBuf;

use mdatron_core::diagnostic::Finding;
use mdatron_core::verify::{verify, VerifyConfig};

struct TempProject {
    root: PathBuf,
}

impl TempProject {
    fn new(name: &str) -> Self {
        let root = std::env::temp_dir().join(format!("vsdd-xref-{}-{}", name, std::process::id(),));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let proj = Self { root };
        proj.seed_schemas_and_patterns();
        proj
    }

    fn seed_schemas_and_patterns(&self) {
        let schemas_dir = self.root.join(".mdatron/schemas");
        let patterns_dir = self.root.join(".mdatron/patterns");
        fs::create_dir_all(&schemas_dir).unwrap();
        fs::create_dir_all(&patterns_dir).unwrap();

        fs::write(
            schemas_dir.join("phase-primer.json"),
            vsdd_core::schemas::PHASE_PRIMER,
        )
        .unwrap();
        fs::write(
            schemas_dir.join("domain-prompt.json"),
            vsdd_core::schemas::DOMAIN_PROMPT,
        )
        .unwrap();
        fs::write(
            schemas_dir.join("supplement.json"),
            vsdd_core::schemas::SUPPLEMENT,
        )
        .unwrap();
        fs::write(
            schemas_dir.join("review-entry.json"),
            vsdd_core::schemas::REVIEW_ENTRY,
        )
        .unwrap();
        fs::write(
            patterns_dir.join("cross-references.yaml"),
            vsdd_core::patterns::CROSS_REFERENCES,
        )
        .unwrap();
    }

    fn write(&self, rel: &str, content: &str) {
        let path = self.root.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }

    fn write_minimal_domains(&self, slugs: &[&str]) {
        for slug in slugs {
            self.write(
                &format!(".claude/commands/vsdd-domain-{slug}.md"),
                &format!(
                    "---\n\
                     schema_class: domain-prompt\n\
                     domain_slug: {slug}\n\
                     role_titles: [{slug}]\n\
                     tier: core\n\
                     classification_universe: [resolved]\n\
                     validator_pair: sanity-check\n\
                     supplements_applied: []\n\
                     ---\n# body\n"
                ),
            );
        }
    }

    fn run(&self) -> Vec<Finding> {
        let cfg = VerifyConfig::new(&self.root);
        verify(&cfg).expect("verify pipeline ran without internal error")
    }
}

impl Drop for TempProject {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn finding_codes(findings: &[Finding]) -> Vec<&str> {
    findings.iter().map(|f| f.code.as_str()).collect()
}

// ── VSDD-E0207: phase-primer.primer_id must match "vsdd-" + phase ──────────────

#[test]
fn e0207_fires_when_primer_id_does_not_match_phase() {
    let proj = TempProject::new("e0207-mismatch");
    // primer_id says phase-1a, phase says phase-2a -- schema-valid, semantically wrong.
    proj.write(
        ".claude/commands/vsdd-phase-1a.md",
        "---\n\
         schema_class: phase-primer\n\
         primer_id: vsdd-phase-1a\n\
         phase: phase-2a\n\
         version: 0.1.0\n\
         frequency: per-milestone\n\
         governing_skill: true\n\
         relevant_domains: []\n\
         supplements_in_scope: []\n\
         ---\n# body\n",
    );

    let findings = proj.run();
    let codes = finding_codes(&findings);
    assert!(
        codes.contains(&"VSDD-E0207"),
        "expected VSDD-E0207 finding; got {codes:?}"
    );
}

#[test]
fn e0207_silent_when_primer_id_matches_phase() {
    let proj = TempProject::new("e0207-match");
    proj.write(
        ".claude/commands/vsdd-phase-2a.md",
        "---\n\
         schema_class: phase-primer\n\
         primer_id: vsdd-phase-2a\n\
         phase: phase-2a\n\
         version: 0.1.0\n\
         frequency: per-milestone\n\
         governing_skill: true\n\
         relevant_domains: []\n\
         supplements_in_scope: []\n\
         ---\n# body\n",
    );

    let findings = proj.run();
    let codes = finding_codes(&findings);
    assert!(
        !codes.contains(&"VSDD-E0207"),
        "expected no VSDD-E0207 finding; got {codes:?}"
    );
}

// ── VSDD-E0208: domain.validator_pair must not equal domain_slug ───────────────

#[test]
fn e0208_fires_when_validator_pair_is_self() {
    let proj = TempProject::new("e0208-self");
    // The fixture pairs with itself; the literal "sanity-check" satisfies E0201
    // without needing a real sanity-check domain in the index.
    proj.write(
        ".claude/commands/vsdd-domain-software-engineer.md",
        "---\n\
         schema_class: domain-prompt\n\
         domain_slug: software-engineer\n\
         role_titles: [Software Engineer]\n\
         tier: core\n\
         classification_universe: [resolved]\n\
         validator_pair: software-engineer\n\
         supplements_applied: []\n\
         ---\n# body\n",
    );

    let findings = proj.run();
    let codes = finding_codes(&findings);
    assert!(
        codes.contains(&"VSDD-E0208"),
        "expected VSDD-E0208 finding; got {codes:?}"
    );
}

#[test]
fn e0208_silent_when_validator_pair_differs_from_self() {
    let proj = TempProject::new("e0208-distinct");
    // Only solution-architect is needed in the index; the fixture pairs with it.
    proj.write_minimal_domains(&["solution-architect"]);
    proj.write(
        ".claude/commands/vsdd-domain-software-engineer.md",
        "---\n\
         schema_class: domain-prompt\n\
         domain_slug: software-engineer\n\
         role_titles: [Software Engineer]\n\
         tier: core\n\
         classification_universe: [resolved]\n\
         validator_pair: solution-architect\n\
         supplements_applied: []\n\
         ---\n# body\n",
    );

    let findings = proj.run();
    let codes = finding_codes(&findings);
    assert!(
        !codes.contains(&"VSDD-E0208"),
        "expected no VSDD-E0208 finding; got {codes:?}"
    );
}
