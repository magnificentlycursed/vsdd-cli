# Binary-First Refactor — Plan

**Status:** design accepted 2026-06-02; queued for Phase 2a authoring.
**Scope:** both `mdatron` and `vsdd` repositories.
**Phase:** 1c (acceptance criteria settled); 2a Red Gate pending per phase.

## Local install status

As of 2026-06-02, `mdatron` is installed at `~/.cargo/bin/mdatron` via
`cargo install --path ../mdatron/mdatron-cli --locked`. The pre-commit
hook at `.githooks/pre-commit` is active and runs `mdatron verify` on
every commit that stages markdown / schema / pattern changes.

## Provenance

This plan synthesizes 24 cold-session domain reviews + inline lens-applications
authored between 2026-06-01 and 2026-06-02. The 18 cluster-batched Phase 3
reviews for crosslink #12 (`*-phase-1-bundle.md`) live at
`mdatron/review-log/` (per the routing correction made during crosslink #13
Phase 4); the cross-repo planning reviews (`*-binary-first-refactor.md`,
`*-init-drift.md`, `*-mdatron-consistency.md`) remain under
`vsdd-cli/review-log/` as cross-cutting design artifacts. Each disposition
below cites the review entries that informed it.

## Operator directives recorded

This session accumulated multiple operator directives that did not previously
land as `OperatorDirectiveApplied` events (see M5 F3+F9 audit-trail finding).
The directives in effect at the time of this plan:

| Date | Directive | Source |
|---|---|---|
| 2026-06-02 | Phase 3 cluster-batched cold-session per milestone is the methodology default; inline shape requires explicit operator authorization per invocation | session correction following 6 inline-Phase-3-skips |
| 2026-06-02 | `mdatron` is a standalone project. `vsdd` consumes `mdatron` as a dependency. `mdatron` has its own `init` command | session |
| 2026-06-02 | Both `mdatron` and `vsdd` are consumed AS BINARIES. `vsdd` shells out to `mdatron verify` rather than calling `mdatron-core` library APIs | session |
| 2026-06-02 | Structure takes cues from `crosslink` (binary-first, `anyhow` at binary edges, `src/commands/<verb>/mod.rs` organization, `include_str!` for embedded resources) | session |
| 2026-06-02 | Workspace shape: full collapse — single `mdatron` / `vsdd` crate per repo; no published `-core` siblings | session disposition |
| 2026-06-02 | `mdatron init` v0.1 scope: empty skeleton + `.mdatron/config.yaml` stub + `verify` reads config (B.2+B.3) | session disposition |
| 2026-06-02 | Drift / upgrade flag surface: adopt `crosslink`'s verbatim on both tools — `--force` / `--update` / `--no-prompt` / `--dry-run` | session disposition |
| 2026-06-02 | DSL `defined()` empty-string carve-out: drop; strict not-Null semantic | session disposition |
| 2026-06-02 | `vsdd init` templates deployment: implement now alongside the 42 markdown artifacts | session disposition |
| 2026-06-02 | mdatron and vsdd error-code namespaces stay strictly separate; no proxy/intercept | DR convergence across multiple cold-sessions |

Methodology amendment owed: capture directives going forward as
`OperatorDirectiveApplied` events at the point of operator-input, not in
retrospect (M5 F3+F9 audit-trail discipline).

## Architectural shape (settled)

| Concern | Decision |
|---|---|
| Distribution model | Each repo publishes one binary crate to crates.io: `mdatron` and `vsdd` |
| Workspace shape | Single crate per repo. No published `mdatron-core` / `vsdd-core` siblings. Internal organization via `src/<topic>/` modules. |
| Subcommand layout | `src/main.rs` dispatches to `src/commands/<verb>/mod.rs` (crosslink-cued) |
| Error paradigm | `anyhow` at the binary edges (main.rs); typed-variant errors at module boundaries |
| Embedded resources | `include_str!` constants in dedicated `src/artifacts.rs` (vsdd) / `src/embedded.rs` (mdatron) |
| Cross-process seam | `vsdd` spawns `mdatron verify --json` as subprocess; parses versioned JSON output object on stdout; human-readable diagnostics on stderr |
| Error codes | `MDATRON-Exxxx` and `VSDD-Exxxx` namespaces strictly separate; no proxy/intercept across the seam |
| Output-format versioning | Output carries `mdatron_output_version` field; vsdd asserts compatibility |
| Drift discipline | Three-way classification (manifest_hash, current_hash, new_template_hash → UpdateAction enum) per crosslink; same flag surface on both `mdatron init` and `vsdd init` |
| Config files | `.mdatron/config.yaml` + `.vsdd/config.yaml`; managed-section markers per crosslink discipline |

## Re-clustered dispositions

All in-flight items from this session, dispositioned:

| # | Item | Disposition | Phase |
|---|---|---|---|
| 1 | Error-code reallocation (M1 F1, M2 F1, M6 F2) | Resolved | Phase 1 |
| 2 | DSL `Field`-on-Object-missing-key returns `Null` (symmetric with `Field`-on-Null) — M4 PE F1 | Resolved | Phase 1 |
| 3a | `should-fire` fixtures for cross-reference rules | Resolved | v0.1.x |
| 3b | vsdd init drift handling | (iii) adopt crosslink flag surface verbatim | Phase 3 |
| 4 | Phase 3 methodology amendment | Accepted with action — memory + methodology.md amendment | Phase 0 + ongoing |
| 5 | Cross-doc consistency (concat() in DESIGN-MDATRON.md; ProjectInitialized in DESIGN-OBSERVABILITY) | Resolved | Phase 0 (specs) + Phase 2 (impl) |
| 6 | Audit-trail tightening (events.jsonl truncation; OperatorDirectiveApplied retroactive; branch protection) | Mixed — events.jsonl bug Resolved; retroactive directive emission Accepted with action; branch-protection Accepted |
| 7 | CI / install reconciliation | Resolved | Phase 6 |
| 8 | vsdd templates deployment | Implement now | Phase 3 |
| 9 | Stdlib undertest (join silent debug-format; set-ops; concat edges) | Resolved | v0.1.x |
| 10 | `defined()` empty-string carve-out | Drop the carve-out | Phase 1 |
| 11 | DSL style cleanups (M7 F7-F10) | Accepted as v0.1 | v0.1.x drive-by with #9 |
| 12 | Author `mdatron` README | Resolved | Phase 2 |
| 13 | `--quiet` / `--json` / `--log-level` global flags | Resolved | Phase 0 |
| 14 | Implement `mdatron explain CODE` + retain `= explain:` diagnostic line [^so-2026-06-02] | Implement explain catalog + retain line | Phase 2 |

[^so-2026-06-02]: Row 14 originally read "Strip `= explain:` diagnostic line".
Reversed per the SO disposition recorded 2026-06-02 at
[`phase-0-output-format/DESIGN.md:566-575`](./phase-0-output-format/DESIGN.md):
"implement explain for v0.1.0; the line is retained because the surface it
promises is built." Amendment applied during crosslink #13 Phase 4 routing
per the M5 F3+F9 audit-trail-on-act discipline; honest record of the
disposition chain preserved in
[`phase-2-mdatron-json/phase-4-routing.md`](./phase-2-mdatron-json/phase-4-routing.md)
§ Operator-directive housekeeping.
| 15 | `mdatron-cli/tests/cli_integration.rs` (now `tests/cli_integration.rs` post-collapse) | Resolved | Phase 2 |
| 16 | anyhow paradigm scope | At binary edges only | Phase 4 |
| 17 | `mdatron init` v0.1 scope | B.2+B.3 (empty skeleton + config.yaml stub) | Phase 5 |
| 18 | Publish `mdatron` 0.1.0 to crates.io | Resolved | Phase 6 |
| 19 | Vocabulary discipline (Finding vs Diagnostic vs Error; schema_class onboarding; mdatron init prose) | Accepted with action | Phase 2 + Phase 6 (README) |
| 20 | Code-allocation lint + DESIGN-MDATRON spec amendment + E0001/E0002 rename | Resolved | Phase 1 |

## 7-phase work plan

Ordered by hard-to-undo dependency. Each phase has its own VSDD 2a→2b→3 cycle
unless noted.

### Phase 0 — Output-format contract design + freeze

**v0.1.0 blocker.** Must land + freeze before any other phase's implementation
work touches the seam.

**Scope:**
- JSON output object shape for `mdatron verify --json` (top-level: `mdatron_output_version`, `findings: [Finding]`, `summary: {error_count, warning_count, ...}`, `pipeline_status`)
- Exit-code semantics (SE's three-state model): `0` = pipeline OK + findings may exist; `1` = pipeline OK + at least one error-severity finding; `2` = pipeline failed (configuration / IO / parse); `3` = binary unavailable / version mismatch
- Global flag wiring: `--quiet`, `--json`, `--log-level`, `--log-format`, `--dry-run` (where applicable)
- Namespace-separation rule for `MDATRON-Exxxx` ↔ `VSDD-Exxxx`
- Methodology amendment: Phase 3 cluster-batched cold-session as the default; inline requires explicit operator-directive event

**Acceptance criteria:**
- Output object JSON schema documented in DESIGN-MDATRON.md
- Exit-code semantics documented; lint-checked
- methodology.md amended with Phase 3 default-shape rule

### Phase 1 — Reserved-code drift + DSL Field-access symmetry + defined() fix

**v0.1.0 blocker.** Internal fixes that change error-emission contracts.

**Scope:**
- Rename `MDATRON-E0001` / `E0002` per spec (currently swapped); reserve E0070, E0080
- Amend `DESIGN-MDATRON.md:116-117` + `:506-514` reserved-codes table
- Add code-allocation lint (compile-time check or test that every emission site emits a reserved code)
- Fix `Field`-on-Object-missing-key to return `Null` (symmetric with existing `Field`-on-Null behavior at `mdatron-core/src/dsl/expr.rs:224`)
- Revert the schema-tightening on `supplements_in_scope` / `supplements_applied`. These fields were originally optional in the schemas. They were promoted to required-but-empty-allowed as a workaround for the pre-Phase-1 DSL behavior (Field-on-missing-key raised FieldNotFound, which crashed any pattern referencing the optional field). After the Phase 1 DSL change makes Field-on-Object-missing-key return `Null`, that workaround is unnecessary; revert to the original optional shape. Per crosslink #12 TW/F2 (the term "reactive fix" was undefined shorthand).
- Drop `defined()` empty-string carve-out (`mdatron-core/src/dsl/expr.rs:322-330`)

**Acceptance criteria:**
- Every error code emitted by mdatron resolves to a reserved-codes entry
- Field access on missing keys returns `Null` (test fixture exercises both branches)
- `defined("")` returns `true` (test)
- vsdd corpus still passes `mdatron verify` after schema reverts

### Phase 2 — mdatron --json + cross-process emission

**v0.1.0 blocker.**

**Scope:**
- Implement `mdatron verify --json`: emit Phase 0 output object on stdout
- Keep stderr emission as rustc-shaped diagnostics (operator-readable when --json absent)
- Strip the dead `= explain:` line from diagnostic format (or implement `mdatron explain CODE` — operator-decision pending; my lean: strip for v0.1.0)
- Author `mdatron` README per DR-F1 (install + first run + schema/pattern example + relationship to vsdd)
- Add `tests/cli_integration.rs` per SE's earlier finding
- Add `--quiet` / `--json` / `--log-level` global args connected through clap

**Acceptance criteria:**
- `mdatron verify --json` emits parseable output object
- README covers the three audiences (VSDD user, mdatron-only adopter, tool-author composer)
- CLI integration tests cover: clean run, finding emission, pipeline failure, missing schema directory

### Phase 3 — vsdd subprocess client + drift handling

**v0.1.0 blocker.**

**Scope:**
- Add `src/mdatron.rs` (or `src/commands/verify/subprocess.rs`) to vsdd: subprocess invocation wrapper for `mdatron verify --json` + output object parsing + exit-code mapping
- Remove `mdatron-core = { path = ... }` from `vsdd-core/Cargo.toml`
- Migrate `vsdd-core/tests/init.rs` + `tests/cross_references.rs`: integration tests now spawn `mdatron` binary against fixture projects rather than calling library APIs
- Migrate `vsdd init` from two-way (manifest vs current) to three-way (manifest vs current vs new_template) classification per crosslink/src/commands/init/manifest.rs:classify_update
- Add `--force` / `--update` / `--no-prompt` / `--dry-run` to vsdd's init CLI
- Extend `ManifestEntry` with `template_version_at_deploy` field (breaking manifest change — land before v0.1.0 publishes)
- Implement interactive prompt on `Conflict` action (per crosslink)
- vsdd init template deployment (item #8): extend artifact registry to include `templates/*` files

**Acceptance criteria:**
- vsdd's tests pass without depending on mdatron-core library API
- vsdd init re-run after a toolkit version bump exercises Conflict path; `--no-prompt` skips; `--force` overwrites; `--dry-run` previews
- Templates deploy alongside 42 markdown artifacts to expected locations

### Phase 4 — Workspace re-org

**v0.1.0 blocker.**

**Scope:**
- mdatron: collapse `mdatron-core` + `mdatron-cli` into single `mdatron` crate; reorganize into `src/main.rs` + `src/commands/{init,verify,explain}/mod.rs` + `src/dsl/` + `src/schema.rs` + `src/diagnostic.rs` + `src/frontmatter.rs` + `src/embedded.rs`
- vsdd: collapse `vsdd-core` + `vsdd` into single `vsdd` crate; reorganize into `src/main.rs` + `src/commands/{init,verify,observe,...}/mod.rs` + `src/artifacts.rs` (the embedded 42-markdown + 4-schema + 1-pattern + 6-template artifacts) + `src/preflight.rs` + `src/mdatron.rs` (subprocess wrapper from Phase 3)
- Apply `anyhow` at binary edges (`main.rs`); keep typed-variant errors at module boundaries
- Single Cargo.toml per repo; drop `[workspace]` section
- Update `cargo install` discovery convention

**Acceptance criteria:**
- `cargo build` produces single binary per repo
- All existing tests pass after migration
- `cargo clippy --all-targets -- -D warnings` clean

### Phase 5 — mdatron init + .mdatron/config.yaml

**v0.1.0 OR v0.1.x.** Operator decision pending on whether Phase 5 ships in
v0.1.0 or is the first v0.1.x release after publish.

**Scope:**
- `mdatron init` subcommand: creates `.mdatron/schemas/` + `.mdatron/patterns/` empty; writes `.mdatron/config.yaml` with documented stub fields (`schemas_dir`, `patterns_dir`, `file_globs`)
- `mdatron verify` reads `.mdatron/config.yaml` first; CLI flags override config; absent config is fine (uses defaults)
- Three-way classification + flag surface (`--force` / `--update` / `--no-prompt` / `--dry-run`) same shape as vsdd init's Phase 3 work
- Managed-section markers in config.yaml per DR

**Acceptance criteria:**
- `mdatron init` on fresh project produces working empty scaffold + parseable config.yaml
- `mdatron verify` honors config.yaml; CLI overrides win
- Re-run after toolkit upgrade triggers `--update` path correctly

### Phase 6 — Publish mdatron 0.1.0 + vsdd CI migration

**v0.1.0 release gate.**

**Scope:**
- crates.io publish for `mdatron` (after Phases 0-4 land; Phase 5 optional per above)
- `vsdd-cli/.github/workflows/mdatron-verify.yml` migrated: `cargo install mdatron --locked --version "0.1.x"` replaces sibling-checkout
- `templates/.github/workflows/vsdd-verify.yml` similarly updated
- `vsdd-cli/.githooks/pre-commit` install hint switches to `cargo install mdatron --locked`
- README install instructions updated for both repos
- CHANGELOG entries for both repos

**Acceptance criteria:**
- `cargo install mdatron --locked --version "0.1.0"` works in a clean environment
- vsdd-cli's CI passes against the published crate (sibling repo no longer required for build)
- Pre-commit hook activates without local cargo install --path

## Deferred to v0.1.x or v0.2 (named, not silent)

Items dispositioned but explicitly scheduled later:
- `should-fire` fixtures for cross-reference rules (#3a) — v0.1.x
- Stdlib undertest (#9) — v0.1.x
- DSL style cleanups (#11) — v0.1.x drive-by with #9
- Cross-doc consistency for `ProjectInitialized` event payload (#5 second half) — v0.1.x after Phase 2 lands
- Audit-trail full tightening (#6) beyond Phase 0 methodology amendment — v0.1.x
- Vocabulary discipline second-pass (DR-F3 / DR-F4 / DR-F5 from mdatron consistency review) — v0.1.x once code surface settles
- M3 F1 sentinel-expansion alternative (`relevant_domains_strategy` schema field) — v0.2
- M5 F8 concat() variadic extension — v0.2 if needed
- Templates flag-surface for vsdd init (currently behind `--update`/`--force` from Phase 3; managed-section markers per file-class are v0.2 if needed)

## Supporting reviews

The 24 review-log entries informing this plan, by topic cluster:

**Phase 3 reviews of completed work (six milestones, 2026-06-02):**
- `2026-06-02-solution-architect.md` — mdatron verify pipeline
- `2026-06-02-software-engineer.md` — mdatron-cli wiring + glob + E0002
- `2026-06-02-quality-engineer.md` — cross-references pattern v1
- `2026-06-02-platform-engineer.md` — pre-commit + CI + template
- `2026-06-02-sanity-check.md` — E0207 + E0208 + concat
- `2026-06-02-solution-owner.md` — vsdd init v0.1
- `2026-06-02-software-engineer-mdatron-dsl-catchup.md` — mdatron-core DSL catch-up

**Init-drift weigh-in cluster:**
- `2026-06-02-{solution-architect,solution-owner,software-engineer,platform-engineer,documentation-reviewer}-init-drift.md`

**mdatron consistency cluster:**
- `2026-06-02-{solution-architect,solution-owner,software-engineer,platform-engineer,documentation-reviewer}-mdatron-consistency.md`

**Binary-first refactor cluster:**
- `2026-06-02-{solution-architect,solution-owner,software-engineer,platform-engineer,documentation-reviewer}-binary-first-refactor.md`
