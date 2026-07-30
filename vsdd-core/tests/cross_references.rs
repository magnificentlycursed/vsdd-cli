//! Phase 2a Red Gate for vsdd-core/patterns/cross-references.yaml extensions.
//!
//! Each test builds a temp project laid out the way mdatron expects
//! (.mdatron/schemas + .mdatron/patterns + fixture markdown files),
//! declares its jurisdiction with `mdatron init` (v0.3.0 requires a
//! config.yaml or verify fails with MDATRON-E0080; vsdd-cli #816), runs
//! the mdatron BINARY's verify over it (tool-to-tool, the #739 boundary;
//! the library pipeline died with the seam, vsdd-cli #764), and asserts
//! that the relevant finding code appears (or does not appear) for the
//! fixture. mdatron on PATH is the estate's own baseline tooling requirement —
//! the preflight names it and the pre-commit gate enforces it — so its
//! absence here is a loud failure, never a skip.

use std::fs;
use std::path::PathBuf;

struct TempProject {
    /// Owns the unpredictable, exclusively-created directory; dropping
    /// it cleans up. The hand-rolled pid-named path this replaces had
    /// a delete-then-create race at a predictable shared-temp location
    /// (vsdd-cli #769) — tempfile is the sibling suite's pattern.
    _dir: tempfile::TempDir,
    root: PathBuf,
}

/// The parsed shape of a `verify --json` run the tests assert against.
/// Carries `pipeline_status` so a silent test can prove the pipeline
/// actually RAN (an E0080 failure is `failed`, never `ok`) rather than pass
/// vacuously on an empty `findings` array — the flip-fallout class this
/// suite fell to (vsdd-cli #822 F1; the #818 false-assurance family).
struct VerifyRun {
    codes: Vec<String>,
    pipeline_status: String,
}

impl TempProject {
    fn new(name: &str) -> Self {
        let dir = tempfile::Builder::new()
            .prefix(&format!("vsdd-xref-{name}-"))
            .tempdir()
            .unwrap();
        let root = dir.path().to_path_buf();
        let proj = Self { _dir: dir, root };
        proj.seed_schemas_and_patterns();
        // v0.3.0 requires a declared jurisdiction (config.yaml) or verify
        // fails with MDATRON-E0080 before any file is checked. Seed it the
        // adopter way -- `mdatron init` -- rather than hand-rolling a
        // config.yaml: hand-rolling the .mdatron layout is exactly what left
        // this harness (and the real tree, vsdd-cli #816) brittle to the
        // 0.1.x -> 0.3.0 jurisdiction requirement. init stays forward-
        // compatible with whatever layout a future mdatron requires.
        proj.mdatron_init();
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

    /// Declare the project's jurisdiction the way a real adopter does, so
    /// the temp project is a faithful v0.3.0 layout and stays valid as the
    /// tool's required layout evolves (vsdd-cli #816). init is idempotent
    /// and preserves the adopter-seeded schemas/patterns above; it adds the
    /// config.yaml (default `**/*.md`, which covers every fixture path these
    /// tests write) and the managed manifest.
    fn mdatron_init(&self) {
        let status = std::process::Command::new("mdatron")
            .args(["init", "-q", "--project-root"])
            .arg(&self.root)
            .status()
            .expect("mdatron runs from PATH — the estate's tooling requirement");
        assert!(
            status.success(),
            "mdatron init seeds the project jurisdiction"
        );
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

    /// Run the mdatron binary's verify; returns the finding codes AND the
    /// pipeline status. Tests assert on `pipeline_status` so a run that
    /// never completed (E0080: `failed`) cannot satisfy a "code absent"
    /// assertion vacuously (vsdd-cli #822 F1).
    fn run(&self) -> VerifyRun {
        let output = std::process::Command::new("mdatron")
            .args(["verify", "--json", "-q", "--project-root"])
            .arg(&self.root)
            .output()
            .expect("mdatron runs from PATH — the estate's tooling requirement");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let parsed: serde_json::Value = serde_json::from_str(stdout.trim())
            .unwrap_or_else(|e| panic!("mdatron --json output parses: {e}"));
        let codes = parsed["findings"]
            .as_array()
            .expect("the output object carries a findings array")
            .iter()
            .map(|f| {
                f["code"]
                    .as_str()
                    .expect("every finding carries a code")
                    .to_string()
            })
            .collect();
        let pipeline_status = parsed["pipeline_status"]
            .as_str()
            .expect("the envelope carries a pipeline_status")
            .to_string();
        VerifyRun {
            codes,
            pipeline_status,
        }
    }
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

    let run = proj.run();
    assert_eq!(run.pipeline_status, "ok", "the pipeline ran to completion");
    assert!(
        run.codes.iter().any(|c| c == "VSDD-E0207"),
        "expected VSDD-E0207 finding; got {:?}",
        run.codes
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
    // Positive control (vsdd-cli #822 F1): a same-run sentinel that MUST
    // fire a sibling cross-reference code, so "E0207 absent" cannot pass
    // vacuously over an inert or failed pipeline. A domain paired with
    // itself trips E0208 while resolving E0201 (it is its own index entry).
    proj.write(
        ".claude/commands/vsdd-domain-quality-engineer.md",
        "---\n\
         schema_class: domain-prompt\n\
         domain_slug: quality-engineer\n\
         role_titles: [Quality Engineer]\n\
         tier: core\n\
         classification_universe: [resolved]\n\
         validator_pair: quality-engineer\n\
         supplements_applied: []\n\
         ---\n# body\n",
    );

    let run = proj.run();
    assert_eq!(run.pipeline_status, "ok", "the pipeline ran to completion");
    assert!(
        run.codes.iter().any(|c| c == "VSDD-E0208"),
        "sentinel: the cross-reference evaluator actually ran this corpus; got {:?}",
        run.codes
    );
    assert!(
        !run.codes.iter().any(|c| c == "VSDD-E0207"),
        "expected no VSDD-E0207 finding; got {:?}",
        run.codes
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

    let run = proj.run();
    assert_eq!(run.pipeline_status, "ok", "the pipeline ran to completion");
    assert!(
        run.codes.iter().any(|c| c == "VSDD-E0208"),
        "expected VSDD-E0208 finding; got {:?}",
        run.codes
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
    // Positive control (vsdd-cli #822 F1): a same-run sentinel firing a
    // sibling code, so "E0208 absent" cannot pass vacuously. A phase primer
    // whose primer_id and phase disagree trips E0207.
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

    let run = proj.run();
    assert_eq!(run.pipeline_status, "ok", "the pipeline ran to completion");
    assert!(
        run.codes.iter().any(|c| c == "VSDD-E0207"),
        "sentinel: the cross-reference evaluator actually ran this corpus; got {:?}",
        run.codes
    );
    assert!(
        !run.codes.iter().any(|c| c == "VSDD-E0208"),
        "expected no VSDD-E0208 finding; got {:?}",
        run.codes
    );
}
