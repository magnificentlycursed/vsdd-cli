---
schema_class: design-doc
schema_version: 1.0.0
doc_class: design
version: 0.1.0
consumes_from: [README.md, methodology.md, DESIGN-METHODOLOGY.md, DESIGN-SCHEMA.md, DESIGN-OBSERVABILITY.md, DESIGN-VERIFICATION.md]
produces_for: []
last_revision_trigger: "Phase 1a + Phase 1b spec authoring per Architecture cluster Finding 9 (#90) routing — toolkit's own DESIGN.md was missing despite being the foundational artifact every Phase 1c decomposition + Phase 2a Red Gate + Phase 2b implementation references"
---

# DESIGN.md — vsdd-cli

The toolkit's own behavioral spec + verification architecture + decomposition. This is the artifact that vsdd-cli's own Phase 2a Red Gate, Phase 2b implementation, and Phase 3 IAR rounds reference. The methodology spec (`methodology.md`) + per-subsystem DESIGN docs (`DESIGN-METHODOLOGY.md`, `DESIGN-SCHEMA.md`, `DESIGN-OBSERVABILITY.md`, `DESIGN-VERIFICATION.md`) describe what the methodology IS + how each subsystem is architected; this DESIGN.md describes what the **vsdd-cli toolkit project** does, ships, and verifies.

This document is authored Phase 1a (Behavioral Spec) + Phase 1b (Verification Architecture) jointly per the methodology's allowance.

---

vsdd-cli is **one collaborator's interpretation + operationalization** of [@dollspace.gay](https://bsky.app/profile/dollspace.gay)'s [VSDD methodology](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) as a Rust CLI. It is **not** the methodology itself; the methodology is authored by dollspace. The toolkit's purpose: provide adoption-ready mechanical enforcement + observability + verification for projects following VSDD.

---

## Phase 5 + Phase 6 strategy (REQUIRED)

**Phase 5 strategy:** planned — Proof Execution (kani-based bounded model checking on the schema-validator's accept/reject decision over bounded input space + YAML round-trip purity verification on the parser core) + Fuzz Testing (cargo-fuzz against the schema validator + YAML parser + bypass-marker parser) + Security Hardening (cargo-audit + cargo-deny on dependencies; Semgrep on Rust source; Wycheproof not applicable in initial scope — no cryptographic surfaces) + Mutation Testing (cargo-mutants against the Rust mirror) + Purity Boundary Audit (final check that the pure-core in `vsdd-core` has no I/O dependencies). Property-based testing (proptest) lives in Phase 2a Test Suite Generation, not Phase 5.

**Phase 6 strategy:** planned — project-terminal four-dimensional convergence attestation when the toolkit reaches Exit Signal (no real findings across active domains in Phase 3 IAR final round; Phase 5 surfaces complete; cross-dimension consistency held).

---

## Per-feature axes (REQUIRED)

Authoritative declaration; consumed by `vsdd init` for adopting projects + by Phase 3 cluster-batching for vsdd-cli's own IAR rounds.

```yaml
per_feature_axes:
  ships-to-users-other-than-developer: yes  # toolkit installed via `cargo install vsdd` by adopting projects
  network-exposed: no                       # toolkit is local CLI; per the operator-directive 2026-05-27, Security + Red Team still activate on credential-handling + supply-chain grounds
  persists-managed-schema-data: yes         # .vsdd/events.jsonl (append-only event log), .vsdd/config.yaml, 3 registry YAML files all carry schema discipline; operator-directive 2026-05-28 activated Data Engineer
  handles-user-data: no                     # no user PII; toolkit handles methodology-relevant project metadata + operator-attribution + git commit data
  ui-surface: no                            # CLI only; treating per methodology framing as not-GUI
  localized: no                             # English-only at v1
  ai-runtime-cost-relevant: yes             # the toolkit IS about AI-runtime cost observability; AI Engineer composes with every Phase 5 + Phase 3 round
```

**Active domain set per Phase 3 IAR composition (always-on baseline + axis-driven + operator-directive):**
- Always-on baseline (always): Software Engineer, Quality Engineer, Solution Architect, Solution Owner, Platform Engineer, Performance Engineer
- Axis-activated: Documentation Reviewer + Technical Writer (ships-to-users-other-than-developer), Data Engineer (persists-managed-schema-data), AI Engineer (ai-runtime-cost-relevant)
- Operator-directive activated (2026-05-27): Security + Red Team (credential-handling + supply-chain attack surface justifies despite network-exposed: no)

Total active: **12 role domains** + 2 meta domains (VSDD Methodology + Sanity Check) on-demand.

**Auth method:**

```yaml
auth_method:
  operator_local: plan
  operator_local_credential_source: "plan-auth-no-key"
  ci: api_key
  ci_credential_source: "env:ANTHROPIC_API_KEY"
auth_attribution_pattern: per-operator
```

Operator-local runs use Claude Plan auth (Max/Pro tier). CI runs use API key via GitHub Secrets. Operator identity is the `git config user.name` value (per-operator attribution pattern). Plan auth structurally rejected for CI per the cross-field validation (per methodology.md § Auth method).

---

## Behavioral contracts (REQUIRED — Phase 1a)

The toolkit's command surface. Per-subcommand: input → output transition + error conditions + edge cases + invariants. The Phase 2a Red Gate will author failing tests per behavior; Phase 2b implements minimum to turn the Red Gate green.

### `vsdd init`

**Contract:** deploys the toolkit's substrate to the current directory (an adopting project). When passed `--check`, only validates prerequisites + reports a deployment plan without writing any files. When passed `--ci-mode`, deploys CI-specific subset (no interactive prompts; reads from `.vsdd/config.yaml` if present). When passed `--update-methodology`, refreshes the project's `methodology.md` to the installed toolkit's canonical version + emits `OperatorDirectiveApplied{directive: methodology-version-updated, from: <semver>, to: <semver>}` event.

**Deployment manifest (per the methodology spec adoption flow):**
- 10 phase-primer skills + 18 per-domain skills + 2 meta-skills to `.claude/commands/vsdd-*.md`
- ~19 methodology hooks to `.claude/hooks/vsdd-*.py`
- 16 role-domain knowledge pages + 14 supplements via `crosslink knowledge import`
- `.vsdd/events.jsonl` (empty)
- `.vsdd/config.yaml` (interactive prompts populate axes + auth method)
- `.vsdd/otel-collector.yaml` (default collector config)
- `.vsdd/registry/{vocabulary,canonical-patterns,anonymization-patterns}.yaml`
- `.vsdd/env-vars` (CLAUDE_CODE_ENABLE_TELEMETRY + OTLP exporter env vars)
- `.claude/mcp.json` (MCP server registration)
- `.claude/settings.json` (UNION-merged with crosslink's allowedTools)
- `methodology.md` (the canonical spec; same `methodology_version` as the toolkit)
- `DESIGN.md.template` (the deployable template; preserves operator-authored DESIGN.md)
- `.github/workflows/vsdd-{verify,observe-pr-body}.yml`
- `.github/PULL_REQUEST_TEMPLATE.md` + `.github/CODEOWNERS` (managed-section)
- `.github/ISSUE_TEMPLATE/vsdd-*.md`
- `.pre-commit-config.yaml` (managed-section) + runs `pre-commit install`

**Emits `ProjectInitialized` event** with full deployment manifest at completion.

**Error conditions:**
- `VSDD-E0220: existing-file-malformed-refuse-to-overwrite` — malformed `.claude/mcp.json` / `.claude/settings.json` / `.pre-commit-config.yaml` / `.vsdd/init-manifest.json`; bail without writing
- Pre-flight failures (git repo absent; Claude Code unavailable; crosslink-init-manifest absent — order-required per README; cargo toolchain absent; Python absent; non-GitHub remote at v1) — report all + exit non-zero
- Interactive-prompt cancellation — clean exit code; no partial state

**Edge cases:**
- Re-init after toolkit upgrade — managed sections replaced in place per init-manifest SHA-tracking; operator-edits outside markers preserved; operator-edits inside managed markers fire `--keep-operator-edits` vs `--accept-managed-defaults` resolution prompt
- Concurrent `vsdd init` invocations (rare; operator races) — first writer wins via file-lock on `.vsdd/init-manifest.json.tmp`
- `--check` with missing prerequisites — reports all gaps; exit non-zero; no writes

### `vsdd verify check`

**Contract:** runs all ~19 methodology hooks (Python subprocess to Rust binary OR pure-Python validation) against the current commit's staged + working-tree files. Emits per-hook `HookFired` event; per-finding `ValidationPassed` or `ValidationFailed` events with error code + location + summary + detail + help.

**Output formats:**
- TTY default — color-coded human-readable per rustc convention; OSC-8 hyperlinks where supported
- `--format sarif` — SARIF 2.1.0 for GitHub Code Scanning
- `--format json` — programmatic consumers
- `--format compact` — terse single-line per finding

**Flags:**
- `--hook <hook-id>` — run a single hook
- `--files <path>...` — selective execution (validates only listed files; otherwise uses `git diff --name-only`)

**Error conditions:**
- Any blocking error code fired by any hook — exit non-zero with error count in stderr
- Hook-internal panic (Rust mirror crash) — fail the hook; record `ValidationFailed{error_code: VSDD-E0300: validator-internal-error}` event; continue to next hook
- Bypass-marker present but rationale empty — fire `VSDD-E0016: bypass-rationale-missing`
- Bypass-marker present + non-namespaced — fire `VSDD-W0070: bypass-marker-scope-mismatch`

**Edge cases:**
- Large commits (>50 files) — selective execution narrows by hook + file; per-hook wall-clock < 1s budget honored
- Binary files — skipped (no methodology hooks apply to binaries)
- Symlinks — followed once; cycle-detected paths skipped with warning

### `vsdd verify explain <error-code>`

**Contract:** opens the per-code documentation page (Rust `--explain` pattern). Page lives at `docs/error-codes/VSDD-X####.md` (deployed alongside the toolkit). Returns markdown to stdout when piped; opens in `$PAGER` when TTY.

**Error conditions:**
- Unknown code — exit non-zero with "code not found; see vsdd verify check --list-codes for the catalog"

### `vsdd verify test-error-catalog`

**Contract:** runs the per-error-code falsifiability fixture pairs at `manual-tests/error-catalog/<code>/{should-fire,should-not-fire}/`. For each accepted code: confirms hook fires on should-fire fixture + does NOT fire on should-not-fire fixture. Candidate codes ship without fixtures (warning only).

**Error conditions:**
- Any accepted-code fixture failure — exit non-zero; release pipeline blocks
- Missing fixture for accepted code — exit non-zero; promotion from candidate to accepted requires fixture

### `vsdd verify hook <hook-id>`

**Contract:** Rust mirror invocation; direct invocation of the canonical validation logic for the named hook. Used by CI (no Python intermediate) + by `check-prior-phase-exit-signal.py` (candidate hook for the phase-skip prevention discipline).

### `vsdd observe cycle | layer | project`

**Contract:** tabular CLI report aggregating event-log + OTel signals for the named scope. `--auto` detects active cycle from session state; `--layer <N>` scopes to layer; `--project` aggregates across all layers.

**Output:** human-readable TTY by default; `--format html` emits standalone HTML next to event log; `--format json` for programmatic consumers.

**Error conditions:**
- Missing `.vsdd/events.jsonl` — clean error: "No event log found; run `vsdd init` first"
- Malformed event-log line — skip + warn with line number; do not abort

### `vsdd observe pr-body --layer <N>`

**Contract:** generates PR description text from `manual-tests/layer-N.md` + project state + phase-domain composition matrix. Used by the `vsdd-observe-pr-body.yml` CI workflow.

### `vsdd observe metrics`

**Contract:** event-log + OTel aggregation. `--auto` detects cycle from active state. `--reconcile-usage-api` (v1+) reconciles SDK estimate with Anthropic Usage and Cost API.

### `vsdd observe trace --finding-id <id>`

**Contract:** walks the event log to assemble the finding's lifecycle span tree (FindingRaised → FindingClassified → FindingRouted → resolution-commit-trace).

### `vsdd observe mcp-cache {refresh | status}`

**Contract:** `refresh` triggers async refresh of external-substrate-doc caches (Claude Code docs + crosslink docs + Anthropic API docs); `status` shows cache hit rates + TTL state.

### `vsdd observe explain <metric>`

**Contract:** documents the named metric's derivation (e.g., cost-per-finding = sum tokens between FindingRaised + FindingClassified events / classified-non-hallucinated count).

### `vsdd mcp-serve`

**Contract:** long-running MCP server (stdio loop); responds to MCP protocol messages; exposes 4 tools (vsdd.methodology.lookup + claude_code.docs.search + crosslink.docs.search + anthropic.api.docs.search per methodology.md § MCP server tool surface). Cache at `.vsdd/mcp-cache/`.

**Error conditions:**
- MCP protocol parse error — log + skip; continue stdio loop
- WebFetch failure (cache miss + network failure) — fail tool call with retry-after hint; preserve stale cache

---

## Verification architecture (REQUIRED — Phase 1b)

### Purity boundary

**Pure functions (`vsdd-core/src/`):**
- `schemas/` modules — Rust types are source-of-truth; serialization + deserialization via `serde` + `schemars` is deterministic
- `anchor.rs` — anchor-ID derivation from frontmatter (`{review_number}-f{finding_number}` etc.); deterministic
- `bypass.rs` — bypass-marker parsing (HTML comment + frontmatter forms); deterministic
- `migration.rs` — schema-version migration utility; deterministic given input + migration rule set
- `error_catalog.rs` — error catalog loader from YAML; deterministic given input
- `methodology_version_drift.rs` — methodology_version comparison; deterministic
- `naming_discipline.rs` (when implemented) — regex match against canonical-patterns registry; deterministic given pattern set + input

**Effectful surface (thin shell around pure core):**
- File I/O (`.vsdd/events.jsonl` reads + writes; `.vsdd/config.yaml` reads + writes; deployment to `.claude/`)
- Subprocess (`crosslink` invocations; Python hook subprocess)
- Network (MCP server fetches via WebFetch; OTel collector forwarding to operator-configured backends)
- Time (timestamps for events; mtime comparison for cache invalidation)
- Environment (env-var reads for credentials; `NO_COLOR`, `CLAUDE_CODE_*` settings)
- Random (UUID generation for events; cryptographic random for signatures)

### Automation classification per behavior

| Behavior | Automatable | Manual-test scope |
|---|---|---|
| `vsdd init` deployment manifest | Yes (file presence assertions) | `vsdd init` interactive prompts (operator confirms axes + auth method) |
| `vsdd init --check` pre-flight | Yes (substrate-prerequisite checks return predictable enum) | — |
| `vsdd verify check` hook chain | Yes (per-hook Red Gate fixtures) | TTY color-coding (operator observes terminal output) |
| `vsdd verify explain <code>` | Yes (markdown rendering) | `$PAGER` interaction |
| `vsdd verify test-error-catalog` | Yes (recursive regression suite) | — |
| `vsdd observe <scope>` | Yes (output-shape regression) | HTML report rendering at `--format html` |
| `vsdd observe pr-body` | Yes (markdown output regression) | GitHub PR UI rendering of the body |
| `vsdd mcp-serve` | Yes (MCP protocol fixtures) | Interactive session with Claude Code |

### Phase 5 candidates

**Property-based testing (proptest):**
- Anchor-ID derivation roundtrip: `parse(anchor_id(frontmatter)) == frontmatter`
- Bypass-marker parsing roundtrip
- Per-class semver comparison ordering invariants
- Methodology-version-drift detection: `drift(toolkit_v, project_v) == (toolkit_v > project_v)`
- Schema validation idempotence: `validate(validate(input))` produces same result

**Mutation Testing (cargo-mutants):**
- Every Rust mirror validator in `vsdd-core/src/`
- Anchor-ID derivation
- Bypass-marker parser
- Per-error-code catalog loader

**Fuzz Testing (cargo-fuzz):**
- Schema validator — arbitrary YAML/JSON inputs (most load-bearing parser surface; v1 ship-blocker)
- Bypass-marker parser — arbitrary markdown/HTML comment inputs
- Error catalog YAML loader — arbitrary YAML inputs
- Event log NDJSON parser — arbitrary JSONL streams

**Proof Execution:** not declared for v1. Schema validator core may become a Proof Execution candidate at v1+ if kani-bounded proofs become tractable + the validator's invariants justify the effort.

### Trust boundaries

External inputs (each carries validation discipline at the boundary):
- **`.vsdd/config.yaml`** (operator-authored OR `vsdd init` populated) — validated against vsdd-config artifact-class schema; cross-field validation rejects credential-shaped values + Plan-auth-for-CI
- **Operator-extensible registry YAML files** (`.vsdd/registry/*.yaml`) — schema-validated; operator-extension sections distinguished from managed sections by structural markers
- **PR description text** (extracted via `gh pr view`) — bypass-marker pattern scanning; CR-LF normalization; control-character filtering before display
- **MCP server tool input** (arbitrary text queries from Claude Code session) — sanitized + length-bounded before query execution
- **Downloaded artifacts** (pre-built binaries via curl-pipe-tar) — cosign signature verified before extraction (v1.0 ship-blocker)
- **Anthropic API responses** (via Agent SDK) — credential-shaped fields rejected by OTel collector redaction processor before any sink

---

## Decomposition (REQUIRED — Phase 1c — per-layer breakdown)

Layer-by-layer build order. Each layer has acceptance criteria + Phase 2a Red Gate scope + manual-tests checklist. Per the methodology, each layer opens a draft PR at Phase 2a + closes when layer-gate criteria met.

### Layer 1: Foundational schemas + utilities

**Scope:** `vsdd-core` crate skeleton; 13 artifact-class JSON Schemas as Rust types; anchor-ID derivation + bypass-marker parser + migration utility + error catalog loader.

**Acceptance criteria:**
- `vsdd-core/Cargo.toml` declares dependencies (schemars, serde, serde_json, serde_yaml, jsonschema, thiserror)
- 13 artifact-class types in `vsdd-core/src/schemas/` with `#[derive(Serialize, Deserialize, JsonSchema)]`
- `cargo build` emits `vsdd-core/schemas/<class>.json` via `schemars` derive at build time
- `vsdd-core/error-catalog.yaml` populated with ~25 accepted + ~15 candidate codes
- `vsdd-core/src/anchor.rs` + `bypass.rs` + `migration.rs` + `error_catalog.rs` + `methodology_version_drift.rs`
- Phase 2a Red Gate: failing tests per behavior; all turn green at Layer 1 close

**Manual-tests:** `manual-tests/layer-1.md` (auto-scaffolded by `post-design-md-modification` hook).

**Depends on:** none (foundational).

### Layer 2: `vsdd init` subcommand

**Scope:** `vsdd init`, `vsdd init --check`, `vsdd init --ci-mode`, `vsdd init --update-methodology` per the behavioral contract. Interactive prompts (axes + auth method). Deployment manifest writing + idempotent re-init.

**Acceptance criteria:**
- All deployment artifacts deploy correctly to a fresh repo
- Re-init preserves operator-edits outside managed sections; reports conflicts inside markers
- `--check` reports deployment plan without writing
- `--ci-mode` skips interactive prompts; reads from `.vsdd/config.yaml`
- `--update-methodology` refreshes project methodology.md + emits OperatorDirectiveApplied event
- Phase 2a Red Gate covers happy path + each error condition + each edge case

**Depends on:** Layer 1 (schemas + utilities).

### Layer 3: `vsdd verify` subcommand + ~19 methodology hooks

**Scope:** `vsdd verify check`, `vsdd verify explain`, `vsdd verify test-error-catalog`, `vsdd verify hook <hook-id>`. ~19 Python hooks at `.claude/hooks/vsdd-*.py` (thin wrappers subprocessing to Rust binary). Per-error-code falsifiability fixtures at `manual-tests/error-catalog/<code>/`.

**Acceptance criteria:**
- All ~19 hooks fire correctly on per-code should-fire fixtures
- No hook fires spuriously on should-not-fire fixtures
- `vsdd verify check --format sarif` emits valid SARIF 2.1.0
- `vsdd verify explain <code>` opens per-code markdown page
- `vsdd verify test-error-catalog` blocks release on any accepted-code fixture failure
- candidate `check-prior-phase-exit-signal.py` hook fires VSDD-W0210 on missing prior-phase exit

**Depends on:** Layer 1 (schemas + utilities) + Layer 2 (vsdd binary for subcommand dispatch).

### Layer 4: `vsdd observe` subcommand + OTel collector deployment

**Scope:** `vsdd observe cycle | layer | project | metrics | trace | pr-body | mcp-cache | explain`. OTel collector config deployment via `vsdd init`. Event-log query + metric derivation + trace assembly.

**Acceptance criteria:**
- All subcommands produce expected output shapes for representative event logs
- `--format json` valid; `--format html` renders standalone; `--format compact` terse
- Event-log query handles malformed lines (skip + warn; do not abort)
- Trace assembly walks finding lifecycle correctly

**Depends on:** Layer 1 + Layer 2.

### Layer 5: `vsdd mcp-serve` subcommand + MCP server

**Scope:** Long-running stdio loop responding to MCP protocol; 4 tools (vsdd.methodology.lookup + claude_code.docs.search + crosslink.docs.search + anthropic.api.docs.search). Cache layer + fetch primitive.

**Acceptance criteria:**
- MCP protocol fixtures pass for each tool
- Cache hit/miss behavior matches spec (file-mtime-aware for methodology lookup; 24-hour TTL for external docs)
- Fetch failures preserve stale cache + return retry-after

**Depends on:** Layer 1 + Layer 2.

### Layer 6: Pre-built binary release pipeline (v1.0 ship-blocker)

**Scope:** GitHub Actions workflow for per-platform builds (Linux x86_64 + aarch64; macOS x86_64 + aarch64). cosign / sigstore signing. SLSA provenance generation. Reproducible builds (Cargo.lock + rust-toolchain.toml + runner pinning).

**Acceptance criteria:**
- `cargo install vsdd` works
- `curl -L <release-url>/<binary>.tar.gz | tar xz` works + cosign verification passes
- SLSA provenance attached as release asset
- Build reproducible across two independent CI runs (byte-identical binaries)

**Depends on:** All prior layers + a stable v1.0-rc.

### Layer 7 (v1+): CHANGELOG cooperation subcommand + LSP server

**Scope:** `vsdd verify changelog --create` (CHANGELOG bootstrap for non-crosslink projects). `vsdd lsp-serve` (LSP server; real-time frontmatter validation).

**Depends on:** prior layers; deferred to v1+.

---

## Out of scope (v1)

- **LSP server** — deferred to v1+ candidate
- **Anthropic Usage and Cost API reconciliation** — `vsdd observe metrics --reconcile-usage-api` deferred to v1+
- **Multi-machine operator identity continuity** — deferred per Phase 5 Round 1 Security finding; revisit on adoption evidence
- **Non-GitHub platforms** (GitLab / Bitbucket / Forgejo / Codeberg / sourcehut) — v1 is GitHub-only per operator-directive 2026-05-27; no commitment for v1+
- **Direct Anthropic Messages API integration** — v1 ships against Claude Code CLI via Agent SDK; Messages API direct in v1+ if operator-time permits
- **Windows pre-built binaries** — Linux + macOS only at v1; WSL or `cargo install vsdd` from source on Windows
- **HTML dashboard ladder v2 / Grafana absorption-pitch v3** — v1 ships tabular CLI reports only
- **Curated reference example project** — vsdd-cli dogfoods on itself; the legacy `bookmark-cli-manual` (in the existing-suite) serves as historical reference
- **Schema migration utility v1 scope** — operator-directive deferral; revisit at v1.0-rc with concrete migration needs
- **methodology_version cadence amendments** — operator-directive deferral; methodology_version stays 0.1.0 through spec-stage; amendments fold into v1.0 baseline

---

## Open decisions deferred

The 48 open crosslink findings represent open decisions. Per the Phase 4 routing log (`review-log/2026-05-27-phase-4-routing.md`), these route across 8 coordination bundles. Substantive deferrals per operator-directive:

| Decision | Routing | Crosslink |
|---|---|---|
| Cluster-batching shape for low-axis projects | Defer per operator "too early" | #80, #103 |
| Schema migration utility v1 vs v1+ scope | Defer per operator "I don't know" | #89 |
| Earned-by-recurrence demotion of 3 amendments | Defer per pre-stability discipline | #118 |
| methodology_version cadence at v0.x | Stay 0.1.0 until v1.0-rc | #117 |
| Composed-domains trailer semantics clarification | Defer to v1.0-rc | #119 |
| Phase 5 before Phase 3 spec-stage adaptation | Partially addressed by prior-phase-exit-signal enforcement | #120 |
| Goal 1/2/3/4 letter-label-adjacent decision | Rationale-required; defer pending op-directive | #111 |
| Toolkit CHANGELOG.md content + cadence | Defer to standalone bundle | #106 |
| THREAT-MODEL.md authoring | Defer to standalone bundle | #101, #123 |
| Operational runbooks (credential-rotation, etc.) | Defer to standalone bundle | #102 |
| Domain prompts + supplements expansion to methodology line-count target | Defer per operator "I haven't even finished writing the specs" | #67 (pg-5) |

Plus ~10 small cross-doc consistency cleanup findings + ~10 implementation-class findings (deferred until layer 1+ implementation begins).

---

## Cross-references

- [`methodology.md`](./methodology.md) — canonical methodology spec; deployed to adopting projects by `vsdd init`
- [`README.md`](./README.md) — toolkit positioning + adoption flow + four governing design goals
- [`DESIGN-METHODOLOGY.md`](./DESIGN-METHODOLOGY.md) — methodology subsystem design; phase primer authoring guidelines; domain prompt authoring guidelines
- [`DESIGN-SCHEMA.md`](./DESIGN-SCHEMA.md) — 13 artifact-class schemas + error catalog file format + schema versioning + cross-field validation
- [`DESIGN-OBSERVABILITY.md`](./DESIGN-OBSERVABILITY.md) — OTel collector + 18 event variants + dashboard ladder + MCP server architecture
- [`DESIGN-VERIFICATION.md`](./DESIGN-VERIFICATION.md) — validator architecture + ~19 methodology hooks + CI workflow templates + dependency approval + methodology version pin
- [`FINDINGS-INDEX.md`](./FINDINGS-INDEX.md) — cross-cycle findings aggregation + per-cycle archive
- [`review-log/`](./review-log/) — per-domain Phase 3 + Phase 5 review entries
- [Claude Agent SDK](https://code.claude.com/docs/en/agent-sdk/overview) — observability + execution substrate
- [`crosslink`](https://github.com/forecast-bio/crosslink) — operational substrate by dollspace + the absorption target for toolkit patterns
- [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) — the methodology this toolkit implements
