---
schema_class: review-entry
schema_version: 1.0.0
review_number: 3
date: 2026-06-02
phase: phase-1a
scope: mdatron + vsdd refactor to binary-first, crosslink-cued layout (subprocess boundary, anyhow at edges, src/commands/, embedded artifacts)
lens: software-engineer
source: operator-directive
session_note: Cold-session SE proposal under settled binary-first directive. SE+sanity-check baseline. No code modified. Re-clusters the prior mdatron-consistency findings under the new model.
model: claude-opus-4-7
execution_method: claude-code-agent
sycophancy_compensation: SE-lens-bias-toward-implementation-defensible-structure; weighed refactor cost against named maintainability gain per minimal-implementation Dimension 2; risk of crosslink-shaped-but-mdatron-empty cargo-cult layout flagged.
supplements_loaded: []
---

# SE proposal: mdatron + vsdd binary-first refactor

## Headline

**Keep mdatron workspace (core + cli), collapse vsdd-core into vsdd, anyhow at binary edges only, src/commands/ per verb, subprocess IPC via `mdatron verify --json` on stdout + exit codes distinguishing not-found / pipeline-broke / findings-found.** JSON envelope + exit-code contract is the load-bearing decision; everything else mechanical. Net: mdatron ~+200 code, +150 test; vsdd ~+250 code (vsdd-core deletion offsetting), +280 test.

## 1. File-tree mockups

### mdatron post-refactor

```
mdatron/
├── Cargo.toml                  # workspace: [mdatron-core, mdatron-cli]
├── mdatron-core/               # UNCHANGED — library + typed errors stay
│   └── src/{lib.rs, diagnostic.rs, error.rs, frontmatter.rs, schema.rs,
│            verify.rs, dsl/{mod,expr,expr_parser,index,parser,types}.rs}
└── mdatron-cli/
    ├── Cargo.toml              # anyhow + clap + mdatron-core path-dep
    ├── src/
    │   ├── main.rs             # ~80 LoC: Cli + global args (--json, --quiet) + dispatch
    │   ├── output.rs           # ~120 LoC: JsonEnvelope struct + tty printer + exit map
    │   └── commands/
    │       ├── mod.rs          # ~10 LoC: pub mod {init, verify, explain}
    │       ├── verify/mod.rs   # ~180 LoC: calls mdatron_core::verify; emits envelope
    │       │                   #   or tty; maps VerifyError → anyhow w/ Context
    │       ├── init/mod.rs     # ~140 LoC: writes .mdatron/{schemas,patterns} from
    │       │                   #   embedded include_str! + sha256 manifest; non-git refuse
    │       └── explain/mod.rs  # ~80 LoC: maps MDATRON-EXXXX → include_str! blurb
    ├── resources/              # NEW — embedded artifacts (minimal defaults)
    │   ├── schemas/example-schema.json    # ~30 LoC
    │   ├── patterns/example-pattern.yaml  # ~20 LoC
    │   └── explain/MDATRON-E0001.md ...   # ~10 stub blurbs
    └── tests/cli_integration.rs           # ~150 LoC, ~8 cases (incl. --json shape)
```

mdatron-core stays private because the engine (~3000 LoC, 130 inline tests) benefits from clap-free unit-test isolation. Embedded artifacts intentionally **minimal** — mdatron is the engine; vsdd ships the rich set.

### vsdd post-refactor

```
vsdd-cli/
├── Cargo.toml                  # workspace: [vsdd]  (vsdd-core deleted)
└── vsdd/
    ├── Cargo.toml              # anyhow + clap + sha2 + serde; NO mdatron-core dep
    ├── src/
    │   ├── main.rs             # ~90 LoC: Cli + global args + dispatch
    │   ├── mdatron.rs          # ~140 LoC: subprocess wrapper — discover, spawn,
    │   │                       #   parse envelope, typed MdatronError enum
    │   ├── preflight.rs        # ~220 LoC UNCHANGED (already subprocess-shaped)
    │   ├── resources.rs        # ~80 LoC: embedded 4 schemas + 1 pattern + 42 markdown
    │   │                       #   via include_str! groups (PHASE_PRIMERS / DOMAINS /
    │   │                       #   SUPPLEMENTS) — same shape as today's vsdd-core lib.rs
    │   └── commands/
    │       ├── mod.rs          # ~10 LoC: pub mod {init, verify, observe}
    │       ├── init/
    │       │   ├── mod.rs      # ~200 LoC: orchestration (was vsdd-core/init.rs)
    │       │   ├── manifest.rs # ~80 LoC: Manifest + sha256 (mirrors crosslink)
    │       │   ├── plan.rs     # ~60 LoC: build_deployment_plan from resources::*
    │       │   └── drift.rs    # ~60 LoC: load_manifest + drift classification
    │       ├── verify/mod.rs   # ~120 LoC: calls mdatron::run_verify, surfaces findings
    │       └── observe/mod.rs  # DEFERRED — see §7 cargo-cult risk
    ├── resources/              # NEW location for embedded artifacts
    │   ├── schemas/ patterns/  # 4 + 1
    │   └── markdown/{phase-primers, domain-prompts, supplements}/  # 10+18+14
    └── tests/
        ├── init_integration.rs    # ~280 LoC migrated from vsdd-core/tests/init.rs
        └── verify_subprocess.rs   # ~180 LoC NEW — boundary tests
```

vsdd-core **deleted** — its only consumer (cross_references.rs → `mdatron_core::verify`) becomes a subprocess call. Library shape with zero external consumers = premature abstraction.

## 2. Migration plan

1. **Keep mdatron-core + mdatron-cli split** — 0 LoC, low risk. *Workspace asymmetry justified:* engine has independent clap-free test surface earning the boundary; vsdd-core has zero remaining external consumers.
2. **mdatron-cli: anyhow + global args; core keeps thiserror** — +60 cli, 0 core, low risk.
3. **mdatron `commands/init/` new** (embedded resources + manifest) — +140 code, +30 test, medium (mirrors vsdd init; §7).
4. **Move vsdd-core schemas/patterns/artifacts → vsdd/src/resources.rs** — +80 vsdd, -100 vsdd-core, low (mechanical).
5. **vsdd-core/src/init.rs → vsdd/src/commands/init/{mod,manifest,plan,drift}.rs**; thiserror→anyhow at boundary, internal `InitError` typed — +400/-295 = +105, medium (test migration).
6. **Build vsdd/src/mdatron.rs subprocess wrapper** — +140, **high (contract, §3)**.
7. **vsdd/tests/verify_subprocess.rs** — +180 test, medium (needs CARGO_BIN_EXE_mdatron).

## 3. Cross-process IPC contract

### CLI shape

```
mdatron verify [--project-root DIR] [--schemas DIR] [--patterns DIR]
               [--files GLOB...] [--json] [--quiet]
```

### vsdd invocation skeleton (vsdd/src/mdatron.rs)

```rust
pub fn run_verify(project_root: &Path) -> Result<VerifyOutcome, MdatronError> {
    let bin = discover_mdatron()?; // $VSDD_MDATRON > config > PATH
    let output = Command::new(&bin)
        .args(["verify", "--json", "--project-root"]).arg(project_root)
        .output().map_err(|e| MdatronError::SpawnFailed { bin, source: e })?;
    match output.status.code() {
        Some(0) => Ok(parse_envelope(&output.stdout)?.into_clean()),
        Some(1) => Ok(parse_envelope(&output.stdout)?.into_findings()),
        Some(2) => Err(MdatronError::PipelineFailed {
            stderr: String::from_utf8_lossy(&output.stderr).into_owned() }),
        other => Err(MdatronError::UnexpectedExitCode(other)),
    }
}
```

### JSON envelope (stdout when --json)

```json
{ "schema_version": "1.0.0", "mdatron_version": "0.1.0",
  "summary": {"errors": 2, "warnings": 1, "lint": 0, "files_checked": 42},
  "findings": [ { "code": "MDATRON-E0001", "severity": "error",
    "summary": "frontmatter-parse-failed", "message": "...",
    "help": null, "location": {"file": "x.md", "line": 3, "column": 5},
    "explain_ref": "MDATRON-E0001" } ] }
```

`Finding` already derives `Serialize`; envelope adds 3 fields + summary struct (~30 LoC in mdatron-cli/output.rs).

### Stdout/stderr discipline

- `--json`: findings → stdout envelope; stderr **silent** except exit-2 path (`VerifyError` Display, plain text, not parsed by vsdd).
- `--json` unset: findings → stderr rustc-style (current behavior); stdout used for summary line.

No mixed-format guessing — vsdd only checks exit code + parses stdout (if json) or forwards stderr verbatim (if pipeline failed).

### Error distinction (load-bearing)

```rust
pub enum MdatronError {
    NotFound { searched: Vec<PathBuf> },           // install: cargo install mdatron-cli
    SpawnFailed { bin: PathBuf, source: io::Error },
    PipelineFailed { stderr: String },              // mdatron's pipeline broke
    EnvelopeParseError { stdout_excerpt: String, parse_error: serde_json::Error },
    UnexpectedExitCode(Option<i32>),
}
```

**"Findings exist" is NOT MdatronError.** Exit-1 with parseable envelope → `Ok(VerifyOutcome::Findings(_))`. Lets vsdd `init` invoke verify as post-deploy check without conflating "couldn't run mdatron" with "mdatron found problems".

### Path discovery

Three-tier: (1) `$VSDD_MDATRON` env override, (2) `mdatron_path` in `.vsdd/config.yaml` (deferred to v0.2), (3) `which::which("mdatron")` default. v0.1 ships tiers 1 + 3. Refactor preflight.rs to share the discovery function.

## 4. Test discipline

- `vsdd-core/tests/init.rs` → `vsdd/tests/init_integration.rs` via `assert_cmd`. 7 cases survive; assertions shift from `matches!(_, Err(InitError::SubstrateNotGit{..}))` to `status.code() == Some(1) && stderr.contains(...)`. ~+30%.
- `vsdd-core/tests/cross_references.rs` → `vsdd/tests/verify_subprocess.rs` (~250 LoC, biggest test-cost item). Adds harness `mdatron_verify_in(temp) -> Envelope`.
- `vsdd-core/tests/schema_validation.rs` → keep as vsdd unit test on `resources::SCHEMAS` via direct `jsonschema` dev-dep; no subprocess needed (asserts bundled JSON is internally valid). ~150 LoC mostly unchanged.
- New boundary tests (`verify_subprocess.rs`, ~180 LoC): `propagates_exit_2_as_pipeline_error`, `returns_findings_outcome_on_exit_1`, `returns_clean_on_exit_0`, `not_found_when_PATH_lacks_mdatron`, `envelope_parse_error_when_stdout_corrupt` (mocked stub binary), `respects_VSDD_MDATRON_override`.
- `mdatron-cli/tests/cli_integration.rs` (prior rec) — ~150 LoC, 8 cases now asserting **JSON envelope shape** when `--json` set.
- **No `mdatron-test` helper crate.** Only one downstream; `assert_cmd` is the well-trodden pattern. Extract when a second consumer appears.

## 5. Idiomatic Rust

- **clap derive** in both (matches crosslink). Builder API unwarranted.
- **Cargo.toml**: vsdd workspace shrinks to one member; keep `[workspace]` for forward-compat (likely `vsdd-events` extractor v0.2).
- **Workspace-private vs `src/<module>/`**: split keeps mdatron-core evaluator tests clap-free. Crosslink modules share Database coupling; mdatron-core has none. Keep the split.

## 6. Disposition re-clustering (vs prior mdatron-consistency review)

- **anyhow in mdatron-cli**: was Reject → **Accept (cli only)**; directive moves the boundary, core keeps typed errors. *Design.*
- **`--quiet` + `--json` global args**: was Accept (~15) → **Accept + escalate (~30)**; `--json` now load-bearing (vsdd parses it). *Design.*
- **Reserved-code spec + lint**: unchanged, still Raise-to-SO. *Design.*
- **No shared diagnostic crate**: **hardened** — JSON envelope formalizes boundary as data. *Mechanical.*
- **Defer subcommand-module refactor**: was Defer → **Reverse, do it now** — directive is the third-verb trigger. *Design.*
- **Defer fuzz/proptest**: unchanged; orthogonal. *Mechanical.*
- **mdatron-cli/tests/cli_integration.rs**: Add now, **upgrade assertions to JSON envelope shape**. *Mechanical.*

## 7. Risk callouts

- **crosslink-shaped cargo-cult.** Falsifiability: if `vsdd/src/commands/verify/mod.rs` is under 50 LoC and just `mdatron::run_verify(...)?`, directory ceremony is dead weight. **`observe` defers its mod.rs** until it has substance.
- **Two-impl drift on init.** mdatron init + vsdd init both grow sha256 + drift. **Flag to Solution Architect** — extraction trigger for shared `init-manifest` workspace crate (~80 LoC consolidation), reinforcing prior SE init-drift deferral.
- **Subprocess perf.** Matters if verify becomes a hot loop. v0.1 isn't; **flag to Performance Engineer for v0.2**.

## Coordination

- **Solution Architect**: workspace decision (keep mdatron-core split, collapse vsdd-core); init-manifest extraction trigger.
- **Quality Engineer**: subprocess boundary test plan; init test migration to assert_cmd.
- **Platform Engineer**: `which mdatron` + `$VSDD_MDATRON` semantics; CI matrix for binary-on-PATH install.
- **Solution Owner** (Raise-to-SO, carried forward): reserved-codes spec amendment still owed.

## Recommendation, restated

Binary-first refactor is correct under the directive. Load-bearing decision: JSON envelope on stdout + exit-code-distinguishes-class (§3). Keep mdatron-core split, collapse vsdd-core, anyhow at binary edges only, src/commands/ now (directive is the third-verb trigger), envelope versioned, three-tier path discovery, assert_cmd for boundary tests, no shared subprocess helper crate. Net delta positive — vsdd-core deletion offsets most new wrapper LoC; win is contract clarity at the process boundary.
