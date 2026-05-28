# DESIGN-VERIFICATION.md

Design document for the verification subsystem of the `vsdd` toolkit. Defines the validator architecture (dual-mode: frontmatter + structural), the per-hook deployment matrix (~17 methodology hooks), the Rust mirror for CI, CI workflow templates with auth-method-conditional steps, bypass-marker enforcement, error catalog implementation, per-error-code falsifiability fixtures, and the `vsdd verify` CLI subcommand surface.

```yaml
# Pre-authoring composition declaration
authoring_target: DESIGN-VERIFICATION.md
composed_domains: [solution-architect, platform-engineer, security, quality-engineer]
composition_mode: skill-interactive
operator_confirmation: confirmed (operator directive 2026-05-27)
declared_at: 2026-05-27
substrate:
  crosslink: initialized in vsdd-cli
  legacy_reference: https://github.com/magnificentlycursed/guild-portfolio/tree/main/vsdd-suite/
  methodology_anchors: [README.md, DESIGN-METHODOLOGY.md, DESIGN-SCHEMA.md, DESIGN-OBSERVABILITY.md, primer 3]
  schema_dependency: DESIGN-SCHEMA.md (per-class JSON Schemas + structural rules + error catalog)
  observability_dependency: DESIGN-OBSERVABILITY.md (HookFired + ValidationPassed/Failed event emission)
```

For positioning: see [`README.md`](./README.md). For methodology: [`DESIGN-METHODOLOGY.md`](./DESIGN-METHODOLOGY.md). For schemas: [`DESIGN-SCHEMA.md`](./DESIGN-SCHEMA.md). For observability: [`DESIGN-OBSERVABILITY.md`](./DESIGN-OBSERVABILITY.md).

---

## Scope + boundary

DESIGN-VERIFICATION owns:

- Validator architecture (dual-mode dispatch: frontmatter + structural)
- ~17 methodology hooks (Python operator-side; Rust mirror CI-side)
- Per-hook deployment matrix (which hooks run at commit-time / CI-time / both)
- CI workflow templates (`.github/workflows/vsdd-*.yml`)
- Auth-method-conditional CI steps (Plan-auth vs API-key-auth)
- Bypass-marker enforcement (operator-local + PR-approval-label gate at CI)
- Error catalog implementation (consumes DESIGN-SCHEMA's catalog file format)
- Per-error-code falsifiability fixtures (manual-tests/error-catalog/<code>/{should-fire,should-not-fire}/)
- `vsdd verify` CLI subcommand surface
- LSP server implementation (v1+ candidate)
- check-anonymization.sh API-key detection patterns extension
- Pre-commit framework integration
- Rust crate workspace structure (single crate; subcommand binary)
- Binary distribution (cargo install + pre-built binaries per platform)

DESIGN-VERIFICATION does NOT own:

- Schema definitions (DESIGN-SCHEMA — type system source-of-truth)
- Event payload schemas (DESIGN-SCHEMA — variant shapes)
- Observability emission internals (DESIGN-OBSERVABILITY — collector + sink wiring)
- Methodology spec content (DESIGN-METHODOLOGY)
- Per-domain prompt content (DESIGN-METHODOLOGY)
- Phase primer body content (DESIGN-METHODOLOGY)

---

## Validator architecture (dual-mode dispatch)

Per DESIGN-METHODOLOGY's reconciled dual-mode declaration: 14 frontmatter-based classes + 1 structural class (CHANGELOG).

### Layered architecture

```
┌─────────────────────────────────────────────────────┐
│  Hooks (.claude/hooks/vsdd-*.py — operator-local)   │
│       │                                             │
│       ├─→ Dispatch per artifact-class               │
│       │                                             │
│       ├─→ Frontmatter mode                          │
│       │     └─→ Parse YAML frontmatter              │
│       │     └─→ Validate against JSON Schema        │
│       │                                             │
│       └─→ Structural mode (CHANGELOG)               │
│             └─→ Apply rule-set                      │
│             └─→ Per-rule dispatch                   │
│                                                     │
│  Rust mirror (vsdd verify hook <hook-id> — CI)      │
│       │                                             │
│       └─→ Same dispatch; same schemas               │
│                                                     │
│  Emission → ValidationPassed / ValidationFailed     │
│              events to .vsdd/events.jsonl           │
│              + OTel collector (DESIGN-OBSERVABILITY)│
└─────────────────────────────────────────────────────┘
```

### One source; two enforcement surfaces

Per A2 (hook architecture decision). Python hooks at `.claude/hooks/vsdd-*.py` cover operator-local; Rust mirror at `vsdd verify hook <hook-id>` covers CI execution. Both consume the same JSON Schemas (frontmatter) + the same YAML structural rule file (CHANGELOG). Operator-local + CI cannot drift in what they enforce.

### Code generation pipeline

Rust types in `vsdd-core/src/schemas/` are source-of-truth. `cargo build` runs `schemars` derive macros → emits `vsdd-core/schemas/<class>.json`. The Python hooks read the generated JSON; the Rust mirror reads the same JSON via `jsonschema` crate.

For CHANGELOG structural rules: hand-authored YAML at `vsdd-core/schemas/changelog.yaml`. Rust mirror uses `serde_yaml` + custom validator. Python hook uses `pyyaml` + matching validator.

### Dispatch logic

```python
# .claude/hooks/vsdd-frontmatter-schema-validator.py (sketch)
def validate(file_path: Path) -> List[ValidationResult]:
    klass = identify_artifact_class(file_path)
    if klass.validation_mode == "frontmatter":
        frontmatter = parse_yaml_frontmatter(file_path)
        schema = load_schema(klass.schema_path)
        return validate_against_schema(frontmatter, schema)
    elif klass.validation_mode == "structural":
        rules = load_structural_rules(klass.schema_path)
        return apply_rules(file_path, rules)
    else:
        raise ValueError(f"Unknown validation mode: {klass.validation_mode}")
```

### Result emission

Each validator emits structured results:

```yaml
# Per-validation result (internal representation)
file: <path>
artifact_class: <class-slug>
schema_version_validated_against: <semver>
result: pass | warning | error | lint
findings:
  - error_code: VSDD-E0040
    severity: error
    location: <file:line>
    summary: <Mentor voice one-line>
    detail: <multi-line explanation>
    note: [<contextual facts>]
    help: <corrective pattern>
    explain_ref: docs/error-codes/VSDD-E0040.md
```

Results stream to:

- stderr (TTY display; color-coded per severity)
- `.vsdd/events.jsonl` via OTel collector (per DESIGN-OBSERVABILITY)
- SARIF file when `--format sarif` flag passed (for CI integration)
- LSP diagnostics when running via LSP server (v1+)

---

## Per-hook deployment matrix (~17 hooks)

Each hook is a thin Python entry point at `.claude/hooks/vsdd-<hook-id>.py` (~5-15 lines) that invokes shared logic via `vsdd-core` (when Rust binary available) OR pure-Python validation logic.

### Hook list

| # | Hook | Trigger | Rule(s) | Error code(s) | Mode |
|---|---|---|---|---|---|
| 1 | check-frontmatter-schema | Pre-commit on `*.md` with frontmatter; pre-merge in CI | Per-class JSON Schema validation | VSDD-E0001-E0099 range (per class) | Both |
| 2 | check-cite-resolution | Pre-commit on `*.md`; pre-merge CI | Citation regex match → audit-trail finding-id registry | VSDD-E0010, VSDD-E0011 (candidate) | Both |
| 3 | check-classification-universe | Pre-commit on Finding entries | Classification ∈ domain's classification_universe | VSDD-E0017 (candidate) | Both |
| 4 | check-naming-discipline (consolidated) | Pre-commit on `*.md` | Letter-label anti-pattern + vocabulary registry compliance + suite-internal terminology + first-use expansion | VSDD-E0160, VSDD-W0001, VSDD-W0140 | Both |
| 5 | check-anonymization | Pre-commit on all committed text files | $HOME / git user.name / git user.email / API-key formats (`sk-ant-api03-`, `Bearer <token>`, etc.) | VSDD-E0220 (existing-file-malformed-refuse-to-overwrite), redaction patterns | Both |
| 6 | check-identity-correlation | Pre-commit on external-author review-log files | Handle-slug consistency across declared platforms | VSDD-E0019 (candidate) | Both |
| 7 | check-document-staleness | Pre-commit on prose surfaces; cron weekly sweep | Cross-doc reference resolution + last-modified-vs-related-modified drift | VSDD-W0030 | Both |
| 8 | check-phase-transitions (consolidated 9-transition matrix) | Phase-boundary commits | Per-R95-F3 9-transition provability matrix | VSDD-E0050 + phase-specific codes | Both |
| 9 | check-phase-composition | Phase-boundary commits | Composed-domains declaration matches phase-domain composition matrix | VSDD-E0050 | Both |
| 10 | check-draft-pr-presence | Phase 2a commits | Draft PR exists for the layer | VSDD-E0070 | CI only |
| 11 | check-pr-template-conformance | PR creation / update | PR description matches PR template artifact class schema | VSDD-E0080 | CI only |
| 12 | check-pr-manual-test-completion | PR merge-gate | Manual-test checklist all checked OR deferred-with-rationale | VSDD-E0090 | CI only |
| 13 | check-design-md-template-conformance | Commits to DESIGN.md | DESIGN.md frontmatter + required sections present | VSDD-E0001 range | Both |
| 14 | post-design-md-modification | Post-commit on DESIGN.md modification | Auto-scaffold `manual-tests/layer-N.md` skeleton + Phase 2a Red Gate test stubs; emit ArtifactScaffolded event | (auto-scaffolding; no error code; emits event) | Operator-local (CI doesn't auto-scaffold; CI validates result) |
| 15 | check-prose-surface-tw-dr-composition | Pre-commit on prose-surface files | Composed-domains trailer OR Co-authored-by trailers for TW + DR present | VSDD-W0180 | Both |
| 16 | check-changelog-discipline (consolidated 10-rule) | Pre-commit on CHANGELOG.md; pre-merge CI | Keep-a-Changelog structural compliance + entry presence + version-date + canonical categories + file integrity + 5 candidate rules | VSDD-W0190, W0191, W0194, W0195, E0240 (accepted); W0192, W0193, W0196, W0197, L0050 (candidate) | Both |
| 17 | check-bypass-marker | Pre-commit on all files | Bypass-marker rationale non-empty + hook-id namespaced + PR-approval-label at CI | VSDD-E0016, VSDD-W0070, VSDD-E0030 | Both |

### Consolidation patterns

Per the naming + coinage governance discipline:

- **check-naming-discipline** consolidates letter-label + suite-internal terminology + vocabulary registry + first-use expansion into ONE hook with multi-rule dispatch (4 rules)
- **check-phase-transitions** consolidates the 9-transition provability matrix into one hook (per R95 F3)
- **check-changelog-discipline** consolidates 10 CHANGELOG-discipline rules into one hook (5 accepted + 5 candidate)

This keeps the operator-facing hook count manageable while preserving per-rule clarity via in-hook check-id reporting.

### Deployment timing

| Hook | Trigger | Surface |
|---|---|---|
| check-frontmatter-schema | Pre-commit + Pre-merge | Operator-local + CI |
| check-cite-resolution | Pre-commit + Pre-merge | Operator-local + CI |
| check-anonymization | Pre-commit | Operator-local (and CI-mirrored on hook run) |
| check-document-staleness | Pre-commit + Cron weekly | Operator-local + scheduled |
| check-phase-transitions | Phase-boundary commits | Both |
| check-phase-composition | Phase-boundary commits | Both |
| check-draft-pr-presence | Phase 2a commit detection | CI only |
| check-pr-template-conformance | PR creation/update | CI only |
| check-pr-manual-test-completion | PR merge-gate | CI only |
| post-design-md-modification | Post-commit on DESIGN.md | Operator-local (CI validates result) |
| check-prose-surface-tw-dr-composition | Pre-commit on prose surfaces | Both |
| check-changelog-discipline | Pre-commit on CHANGELOG.md + Pre-merge | Both |
| check-bypass-marker | Pre-commit | Both |

CI bootstrap: `vsdd init --ci-mode` runs first; deploys hooks + Rust mirror artifacts. All hooks except auto-scaffolding (post-commit hook) run in both contexts.

---

## CI workflow templates

`vsdd init` deploys CI workflow templates to `.github/workflows/vsdd-*.yml` per the CI workflow template artifact class (DESIGN-SCHEMA candidate class; promotes to accepted when second project's evidence justifies).

### Default templates

**`.github/workflows/vsdd-verify.yml`** — runs all methodology hooks via Rust mirror:

```yaml
name: vsdd verify

on:
  pull_request:
  push:
    branches: [main]

jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install vsdd
        run: cargo install vsdd --locked
      - name: Bootstrap toolkit artifacts
        run: vsdd init --ci-mode
      - name: Run methodology hooks
        run: vsdd verify check --format sarif > vsdd-verify.sarif
      - name: Upload SARIF for code scanning
        uses: github/codeql-action/upload-sarif@v3
        if: always()
        with:
          sarif_file: vsdd-verify.sarif
```

**`.github/workflows/vsdd-observe-pr-body.yml`** — auto-generates PR body when DESIGN.md commits modify layer behavioral contracts:

```yaml
name: vsdd observe pr-body

on:
  pull_request:
    paths:
      - 'DESIGN.md'
      - 'manual-tests/**'

jobs:
  generate-pr-body:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install vsdd
        run: cargo install vsdd --locked
      - name: Bootstrap
        run: vsdd init --ci-mode
      - name: Generate PR body
        id: prbody
        run: |
          BODY=$(vsdd observe pr-body --layer $(vsdd observe metrics --current-layer))
          echo "body<<EOF" >> $GITHUB_OUTPUT
          echo "$BODY" >> $GITHUB_OUTPUT
          echo "EOF" >> $GITHUB_OUTPUT
      - name: Update PR description
        uses: actions/github-script@v7
        with:
          script: |
            github.rest.pulls.update({
              owner: context.repo.owner,
              repo: context.repo.repo,
              pull_number: context.issue.number,
              body: process.env.PR_BODY
            })
          env:
            PR_BODY: ${{ steps.prbody.outputs.body }}
```

### Auth-method-conditional steps

Per A11: CI uses API key (predictable pay-as-you-go billing per Anthropic's automation guidance). Templates assume API key via GitHub Secrets:

```yaml
env:
  ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
  CLAUDE_CODE_ENABLE_TELEMETRY: 1
  OTEL_EXPORTER_OTLP_ENDPOINT: ${{ secrets.OTEL_COLLECTOR_ENDPOINT }}
  OTEL_EXPORTER_OTLP_HEADERS: Authorization=Bearer ${{ secrets.OTEL_COLLECTOR_TOKEN }}
```

Operator configures secrets at the GitHub repo level. Templates reference `${{ secrets.<NAME> }}` — never inline credential values.

For Plan-auth-operating projects (rare in CI; typically operator-local only): templates document the conditional path; check-anonymization scope catches any inline credentials.

### SARIF output for GitHub Code Scanning

`vsdd verify check --format sarif` emits [SARIF 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/) — GitHub's [Code Scanning](https://docs.github.com/en/code-security/code-scanning) consumes it natively. Validation findings appear in PR Conversation tab as code-scanning alerts; PR reviewers see them inline alongside code review.

SARIF rule definitions live at `vsdd-core/sarif-rules.json` (generated from error-catalog.yaml at build time).

### check-anonymization.sh API-key detection patterns extension

The existing `check-anonymization.sh` from crosslink/existing-suite covers $HOME + git user.name + git user.email patterns. The toolkit extends with API-key patterns per SEC-F2:

```bash
# Additional patterns (deployed to .vsdd/registry/anonymization-patterns.yaml)
api_key_patterns:
  - "sk-ant-api03-[A-Za-z0-9_\\-]+"          # Anthropic API key
  - "sk-[A-Za-z0-9]{32,}"                    # OpenAI / generic sk-prefix
  - "Bearer [A-Za-z0-9_\\-\\.=]+"            # generic Bearer token in HTTP header
  - "ghp_[A-Za-z0-9]{36}"                    # GitHub Personal Access Token
  - "github_pat_[A-Za-z0-9_]+"               # GitHub fine-grained PAT
  - "[a-zA-Z_]*_?TOKEN=[A-Za-z0-9_\\-\\.=]+" # env-var-assignment with credential-shaped value
  - "[a-zA-Z_]*_?KEY=[A-Za-z0-9_\\-\\.=]+"   # env-var-assignment KEY
  - "[a-zA-Z_]*_?SECRET=[A-Za-z0-9_\\-\\.=]+"# env-var-assignment SECRET
```

Operator-extensible at `.vsdd/registry/anonymization-patterns.yaml` (deployed by vsdd init; operator adds project-specific patterns).

---

## Bypass-marker enforcement

### Operator-local handling

Pre-commit hook (`check-bypass-marker.py`) scans for bypass-marker pattern:

```
<!-- hook-bypass[<hook-id>]: <rationale> -->
```

Rules:
- Bypass-marker present + non-empty rationale → hook SKIPPED for the marked context; emits `BypassMarkerInvoked` event (could be a new variant; deferred under variant-proliferation governance until earned-by-recurrence trigger)
- Bypass-marker present + empty rationale → `VSDD-E0016: bypass-rationale-missing` ERROR
- Bypass-marker present + non-namespaced (`<!-- hook-bypass: ... -->` without hook-id) → `VSDD-W0070: bypass-marker-scope-mismatch` WARNING

### CI-side merge gate

PR with bypass-marker active requires explicit operator approval via GitHub label (configurable; default: `bypass-approved`):

```yaml
# .github/workflows/vsdd-verify.yml (excerpt)
- name: Check bypass-marker PR approval
  if: contains(github.event.pull_request.body, 'hook-bypass')
  uses: actions/github-script@v7
  with:
    script: |
      const labels = await github.rest.issues.listLabelsOnIssue({
        owner: context.repo.owner,
        repo: context.repo.repo,
        issue_number: context.issue.number,
      });
      if (!labels.data.some(l => l.name === 'bypass-approved')) {
        core.setFailed('PR contains bypass-marker without bypass-approved label. Add label after operator-review confirms the bypass rationale.');
      }
```

Operator review → add `bypass-approved` label → merge gate passes. Without label: CI fails → merge blocked.

### Frontmatter form (for typed artifacts)

```yaml
bypass:
  - hook_id: <hook-slug>
    rationale: <non-empty string>
    pr_approval_label: bypass-approved
```

Hook reads frontmatter; same enforcement logic.

---

## Error catalog implementation

### File location

`vsdd-core/error-catalog.yaml` — per DESIGN-SCHEMA's catalog file format. Build-time emission to `vsdd-core/sarif-rules.json` + `docs/error-codes/` (one markdown page per code).

### Per-code documentation

Each code has a markdown page at `docs/error-codes/VSDD-X####.md`:

```markdown
# VSDD-E0040 — promised-artifact-missing

**Severity:** error
**Status:** accepted (multi-recurrence: bookmark-cli-manual SO R1 F1 + DR R1 F3)
**Introduced in:** 1.0.0

## What this means

TODO.md (or DESIGN.md) commits to an artifact that doesn't exist in the project tree.
The commitment is documented; the artifact is absent. Two paths to a clean state:
either author the missing artifact OR annotate the deferral with explicit rationale.

## Example

[example output from the hook]

## How to fix

Author the missing file following the relevant class frontmatter template:

[per-class template snippet]

OR annotate deferral in TODO.md:

```yaml
deferred_artifacts:
  - path: manual-tests/layer-3.md
    deferred_to: <phase | layer>
    rationale: <non-empty>
```

## Related codes

- VSDD-W0080 — manual-test-checkbox-without-specificity
- VSDD-W0030 — stale-claim suspicion

## See also

[methodology references]
```

`vsdd verify explain VSDD-E0040` opens this page in the operator's terminal (TTY) or via the LSP.

### SARIF rule format

```json
{
  "id": "VSDD-E0040",
  "shortDescription": { "text": "promised-artifact-missing" },
  "fullDescription": {
    "text": "TODO.md commits to an artifact that doesn't exist in the project tree."
  },
  "help": {
    "text": "Author the missing file following the relevant class frontmatter template, OR annotate the deferral with explicit rationale.",
    "markdown": "[Mentor-voice corrective pattern; per-code help text]"
  },
  "helpUri": "https://github.com/magnificentlycursed/vsdd-cli/blob/main/docs/error-codes/VSDD-E0040.md",
  "defaultConfiguration": { "level": "error" },
  "properties": {
    "category": "promised-artifact",
    "vsdd_status": "accepted",
    "evidence_base": ["bookmark-cli-manual SO R1 F1", "bookmark-cli-manual DR R1 F3"]
  }
}
```

GitHub Code Scanning consumes SARIF; findings appear in PR Conversation + Security tab.

---

## Per-error-code falsifiability fixtures

Per QE dim 2 + the QE-driven error-catalog-test-discipline: each error code has positive + negative test fixtures at:

```
manual-tests/error-catalog/
├── VSDD-E0040/
│   ├── README.md                       # describes the code + how fixtures demonstrate
│   ├── should-fire/
│   │   ├── todo-with-promised-missing-file.md
│   │   └── expected-output.txt         # what the hook should emit
│   └── should-not-fire/
│       ├── todo-with-deferred-annotation.md
│       └── expected-output.txt
├── VSDD-E0050/
│   └── ... (per-code fixture set)
└── ... (one directory per error code in catalog)
```

`vsdd verify test-error-catalog` runs the regression suite:

```bash
$ vsdd verify test-error-catalog
Running 47 error-code fixture pairs...
✓ VSDD-E0010: cross-ref resolution (positive + negative pass)
✓ VSDD-E0040: promised-artifact-missing (positive + negative pass)
✓ VSDD-E0050: phase-composition-not-declared (positive + negative pass)
✗ VSDD-W0192: changelog-category-label-mismatch (CANDIDATE; should-fire fixture missing)
...
44 pass / 0 fail / 3 candidate-status-missing-fixture
```

Candidate codes can ship without fixtures (warning only); accepted codes MUST have fixtures (regression suite blocks promotion to accepted otherwise).

---

## `vsdd verify` CLI subcommand surface

```
vsdd verify check                         # run all methodology hooks
vsdd verify check --hook <hook-id>        # run a specific hook
vsdd verify check --format sarif          # SARIF output for CI integration
vsdd verify check --format json           # programmatic consumers
vsdd verify check --format compact        # terse single-line per finding
vsdd verify check --files <file>...       # selective execution (only validate listed files)
vsdd verify explain <error-code>          # extended documentation (rustc --explain pattern)
vsdd verify test-error-catalog            # run per-code fixture regression suite
vsdd verify test-error-catalog --code <c> # run specific code's fixtures
vsdd verify hook <hook-id>                # Rust hook-runner mirror invocation (CI-side)
vsdd verify hook <hook-id> --file <path>  # validate specific file via specific hook
vsdd verify migrate <artifact-class>      # schema migration (v1+ candidate)
vsdd verify changelog --create            # auto-create CHANGELOG.md for non-crosslink projects (v1+ candidate)
```

### Selective execution

Per AI Engineer dim 9 (validator wall-clock budget): hook chain runs only on files in `git diff --name-only` for the commit. `vsdd verify check` without `--files` flag uses git-diff scoping by default; explicit `--files` overrides.

### Output formats

| Format | Use case |
|---|---|
| TTY (default) | Operator-local; color-coded; OSC-8 hyperlinks where supported |
| `--format sarif` | CI integration (GitHub Code Scanning, GitLab) |
| `--format json` | Programmatic consumers (custom CI tooling) |
| `--format compact` | High-density scenarios (operator wants tight summary) |

### Error-density throttling

If a single commit fires 50+ instances of the same error code, output consolidates:

```
error[VSDD-E0010]: unresolved cross-reference [×42 in this commit; first 3 shown below]
  --> DESIGN.md:88
   | [first instance]
  --> DESIGN.md:142
   | [second instance]
  --> DESIGN.md:201
   | [third instance]
   = note: 39 additional instances; run `vsdd verify check --format json` for full list
```

---

## LSP integration (v1+ candidate)

`vsdd-lsp` separate binary (OR `vsdd lsp-serve` subcommand) — LSP server protocol implementation.

### Features (v1+ scope)

- **Real-time frontmatter validation** as you type
- **Squiggle rendering** for errors / warnings / lints with appropriate severity
- **Hover tooltips** for error codes (shows summary + help)
- **Quick-fix actions** for common errors:
  - Replace deprecated vocabulary (`IAR` → `Adversarial Refinement`)
  - Auto-add missing required frontmatter field (with default value)
  - Auto-add Composed-domains trailer to commit message
  - Auto-insert first-use expansion for abbreviations
- **Go-to-definition** for cross-references (click `[VSDD-E0040]` → opens docs page)
- **Find-references** for finding-IDs + domain-slugs + phase-IDs

### IDE integrations

- VS Code extension via `@anthropic-ai/claude-code` extension
- JetBrains plugin (via LSP client)
- Vim / Neovim via `coc.nvim` or built-in LSP client

Deferred to v1+. Substantial implementation cost; revisit when v1 ships + operator-experience data accumulates.

---

## Pre-commit framework integration

`vsdd init` runs `pre-commit install` automatically (per Tier A shift-left discipline). Deploys `.pre-commit-config.yaml` with managed-section markers:

```yaml
# .pre-commit-config.yaml (excerpt; managed section)
# === vsdd managed ===
repos:
  - repo: local
    hooks:
      - id: vsdd-frontmatter-schema
        name: vsdd frontmatter schema validation
        entry: .claude/hooks/vsdd-frontmatter-schema.py
        language: python
        files: '\.md$'
      - id: vsdd-cite-resolution
        name: vsdd citation resolution
        entry: .claude/hooks/vsdd-cite-resolution.py
        language: python
        files: '\.md$'
      # ... (17 hooks total)
# === End vsdd managed ===
```

Idempotent re-init: managed section replaced in place; operator-added hooks outside markers preserved.

---

## Rust crate workspace structure

Per A1 + A14: single `vsdd` crate; single `vsdd` binary with subcommand dispatch.

### Workspace layout

```
vsdd-cli/
├── Cargo.toml                          # workspace root
├── vsdd-core/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs                      # shared library modules
│   │   ├── schemas/                    # Rust types (source-of-truth)
│   │   │   ├── review_entry.rs
│   │   │   ├── finding.rs
│   │   │   ├── ...                     # 14 frontmatter classes
│   │   ├── events/                     # 18 event variant types
│   │   ├── anchor.rs                   # anchor-ID derivation utility
│   │   ├── bypass.rs                   # bypass-marker parsing
│   │   ├── migration.rs                # schema-version migration utility
│   │   └── error_catalog.rs            # error catalog loader
│   ├── schemas/                        # JSON Schemas (generated at build time)
│   │   ├── review-entry.json
│   │   ├── finding.json
│   │   ├── ...
│   │   └── changelog.yaml              # structural rule file (hand-authored)
│   ├── error-catalog.yaml              # error catalog source
│   ├── sarif-rules.json                # SARIF rule definitions (generated)
│   └── build.rs                        # cargo build hook for code-gen
├── vsdd/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                     # subcommand dispatch
│       ├── init/                       # vsdd init implementation
│       ├── verify/                     # vsdd verify implementation
│       ├── observe/                    # vsdd observe implementation
│       └── mcp_serve/                  # vsdd mcp-serve implementation
├── .claude/                            # Claude Code substrate (deployed by vsdd init when this is a vsdd-using project)
│   ├── hooks/
│   │   └── vsdd-*.py                   # 17 methodology hooks (Python thin wrappers)
│   ├── commands/
│   │   └── vsdd-*.md                   # 10 phase primers + 16 domain skills + 2 meta
│   └── mcp.json                        # MCP server registration
├── docs/
│   └── error-codes/
│       └── VSDD-*.md                   # per-code documentation
├── manual-tests/
│   └── error-catalog/
│       └── VSDD-*/                     # per-code fixture pairs
└── .github/
    ├── workflows/
    │   ├── vsdd-verify.yml
    │   └── vsdd-observe-pr-body.yml
    ├── CODEOWNERS
    ├── PULL_REQUEST_TEMPLATE.md
    └── ISSUE_TEMPLATE/
```

### Cargo.toml workspace

```toml
[workspace]
members = ["vsdd-core", "vsdd"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.80"
license = "MIT"
authors = ["magnificentlycursed"]
repository = "https://github.com/magnificentlycursed/vsdd-cli"
```

### Binary entry

```toml
# vsdd/Cargo.toml
[[bin]]
name = "vsdd"
path = "src/main.rs"
```

`cargo install vsdd` installs the single `vsdd` binary. Subcommands dispatched via clap.

---

## Binary distribution

### crates.io publication

`cargo publish` from `vsdd/` (vsdd-cli is the GitHub repo name; `vsdd` is the crate name on crates.io). Per A11: `vsdd` reserved on crates.io.

### Pre-built binaries (v1+ optimization)

For CI environments where `cargo install` from source is too slow (~60s compile), pre-built binaries via GitHub Releases:

```
vsdd-0.1.0-x86_64-unknown-linux-gnu.tar.gz
vsdd-0.1.0-aarch64-unknown-linux-gnu.tar.gz
vsdd-0.1.0-x86_64-apple-darwin.tar.gz
vsdd-0.1.0-aarch64-apple-darwin.tar.gz
```

CI workflow installs via:

```yaml
- name: Install vsdd (pre-built)
  run: |
    curl -L https://github.com/magnificentlycursed/vsdd-cli/releases/download/v0.1.0/vsdd-0.1.0-x86_64-unknown-linux-gnu.tar.gz \
      | tar xz -C /usr/local/bin
```

Fallback: `cargo install vsdd --locked` (slower but always-fresh).

---

## Cross-DESIGN-doc coordination

### What this doc consumes

| Source | Consumed |
|---|---|
| **DESIGN-SCHEMA** | 14 frontmatter JSON Schemas + 1 structural rule file + error catalog file format + anchor-ID derivation conventions + bypass-marker schema |
| **DESIGN-OBSERVABILITY** | OTel emission convention for `HookFired` + `ValidationPassed` / `ValidationFailed` events; collector + sink wiring + redaction processor |
| **DESIGN-METHODOLOGY** | Phase-domain composition matrix (informs hook dispatch); Layer-cycle PR discipline (informs CI workflow templates); CHANGELOG cooperation pattern |

### What this doc produces

| Consumer | Produces |
|---|---|
| **DESIGN-METHODOLOGY** | Per-hook deployment matrix (informs methodology spec section on enforcement layer); CI workflow templates (informs operator-runbook surfaces) |
| **DESIGN-OBSERVABILITY** | Hook-fire event emission schema (consumed by collector); validation-result emission schema |

### Forward-references to v1+

| Decision | Routing |
|---|---|
| LSP server implementation details | v1+ scope (post-toolkit-v1 ship) |
| Anthropic Usage and Cost API integration | DESIGN-OBSERVABILITY v1+ |
| `vsdd verify migrate` subcommand surface | This doc v1+ iteration |
| `vsdd verify changelog --create` for non-crosslink projects | This doc v1+ iteration |

---

## Implementation order

| Track | Goal-4 surface? |
|---|---|
| 5a — Implement vsdd-core crate (Rust types + schemars + JSON Schema codegen + error catalog loader + anchor + bypass + migration utilities) | Foundational |
| 5b — Implement vsdd binary subcommand dispatch + `vsdd init` (deployment + interactive prompts + manifest tracking) | Yes |
| 5c — Implement `vsdd verify check` + `vsdd verify hook <hook-id>` + `vsdd verify explain` + `vsdd verify test-error-catalog` | Yes |
| 5d — Author 17 Python hooks (thin wrappers; consume vsdd-core via subprocess OR pure-Python validation logic) | Yes |
| 5e — Author CI workflow templates (`.github/workflows/vsdd-*.yml`) | Yes (Goal-4 specific) |
| 5f — Implement SARIF emission + per-error-code documentation pages | Yes |
| 5g — Author per-error-code fixture pairs at `manual-tests/error-catalog/<code>/` | QE coverage |
| 5h — Implement check-anonymization.sh extension (API-key detection patterns) | Cross-cutting |
| 5i — Implement consolidated check-naming-discipline hook (4-rule dispatch) | Cross-cutting |
| 5j — Implement consolidated check-changelog-discipline hook (10-rule dispatch) | Yes |
| 5k — Implement post-DESIGN.md auto-scaffolding hook (manual-tests + Phase 2a Red Gate skeleton) | No (Tier A/B shift-left) |
| 5l — Implement bypass-marker enforcement (pre-commit + CI label gate) | Cross-cutting |
| 5m — Pre-commit framework integration (`vsdd init` auto-runs `pre-commit install`) | Yes |
| 5n — Pre-built binary release pipeline (GitHub Releases; per-platform builds) | Yes (Goal 4 CI optimization) |
| 5o — LSP server implementation (`vsdd lsp-serve`) | No (v1+ scope) |
| 5p — `vsdd verify migrate` subcommand (schema-version migration) | No (v1+ scope) |
| 5q — `vsdd verify changelog --create` subcommand (non-crosslink-projects CHANGELOG bootstrap) | No (v1+ scope) |

Tracks 5a-5n are v1 deliverables. 5o-5q are v1+.

---

## Open decisions deferred

| Decision | Routing |
|---|---|
| Python hook dispatch (subprocess to vsdd-core Rust binary vs pure-Python validation) | Implementer's call during 5d; recommend pure-Python for simplicity + Rust mirror as the CI-side optimization path |
| LSP server protocol details (which capabilities to implement; quick-fix action surface) | v1+ scope |
| `vsdd verify migrate` mechanism (auto-fix patterns vs operator-prompted) | v1+ scope; needs evidence from first major-version-bump cycle |
| Pre-built binary signing (cosign / sigstore / GPG) | DESIGN-VERIFICATION v1+ iteration |
| Operator-extension hook authoring guidance (when does a project add a custom hook to its own .vsdd/registry/?) | DESIGN-METHODOLOGY (operator-runbook scope) |
| Cross-language hook execution (Python hooks reading non-Python sources; Rust hooks generic over language) | Implementation detail; standard cross-language pattern |

---

## Closing

DESIGN-VERIFICATION operationalizes the mechanical-enforcement layer of Goal 2 (machine-enforceable) + Goal 4 (CI/CD shift-left). 17 methodology hooks composing with crosslink's 5 = ~22 hooks total deployed in a VSDD project. Per-hook deployment matrix declares operator-local vs CI scope. Rust mirror at CI-side preserves "one source; two enforcement surfaces" — operator-local + CI cannot drift in what they enforce.

Bypass-marker enforcement is the operator-escape-valve; PR-approval label is the CI-side teeth. Error catalog with per-code documentation + SARIF emission + fixture-based falsifiability — the Rust-compiler-for-documents discipline made operational.

**Next:** DESIGN-METHODOLOGY revalidation against the trio (DESIGN-SCHEMA + DESIGN-OBSERVABILITY + DESIGN-VERIFICATION) at the cross-DESIGN-doc closure boundary. Implementation tracks 5a-5n become the v1 deliverable surface.
