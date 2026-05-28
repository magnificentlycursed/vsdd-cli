# DESIGN-SCHEMA.md

Design document for the schema enforcement subsystem of the `vsdd` toolkit. Defines per-artifact-class schemas, the type-system source-of-truth, the dual validation modes (frontmatter + structural), the auth_method credential-exclusion structural property, the anchor-ID generation conventions, the error catalog structure, schema versioning, and cross-DESIGN-doc coordination.

```yaml
# Pre-authoring composition declaration (per phase-domain composition discipline)
authoring_target: DESIGN-SCHEMA.md
composed_domains: [solution-architect, solution-owner, ai-engineer]
composition_mode: skill-interactive
operator_confirmation: confirmed (operator directive 2026-05-27)
declared_at: 2026-05-27
substrate:
  crosslink: initialized in vsdd-cli (committed)
  legacy_reference: https://github.com/magnificentlycursed/guild-portfolio/tree/main/vsdd-suite/
  methodology_anchors: [README.md, DESIGN-METHODOLOGY.md, primer 3]
```

For positioning: see [`README.md`](./README.md). For methodology subsystem design: [`DESIGN-METHODOLOGY.md`](./DESIGN-METHODOLOGY.md). Sibling subsystem designs: [`DESIGN-OBSERVABILITY.md`](./DESIGN-OBSERVABILITY.md) (consumes event-variant schemas + collector + sink design), [`DESIGN-VERIFICATION.md`](./DESIGN-VERIFICATION.md) (consumes schemas for hook implementation + Rust mirror).

---

## Scope + boundary

DESIGN-SCHEMA owns:

- Per-artifact-class JSON Schemas + structural-pattern schemas (13 classes)
- Schema source format + file conventions + code generation pipeline
- Auth_method declaration schema with explicit NO-key-material structural property
- Credential-exclusion structural property for event-variant schemas
- Anchor-ID deterministic generation conventions
- Error catalog file format + code-range conventions + status-tier discipline (candidate / accepted / deprecated)
- Per-class semantic versioning + migration discipline
- Bypass-marker schema (HTML comment + frontmatter forms)

DESIGN-SCHEMA does NOT own:

- Hook implementation (DESIGN-VERIFICATION)
- Validator execution (DESIGN-VERIFICATION)
- Event log sink format + OTel collector internals + dashboards (DESIGN-OBSERVABILITY)
- Methodology spec section content (DESIGN-METHODOLOGY)
- Per-phase primer + per-domain prompt content (DESIGN-METHODOLOGY)

---

## Schema source + tooling

### Schema file conventions

Per-artifact-class schemas live at `vsdd-core/schemas/<class>.{json,yaml}`. The file format depends on validation mode:

- **Frontmatter-based** (12 classes): JSON Schema files at `vsdd-core/schemas/<class>.json` validate YAML frontmatter at top of markdown files. JSON Schema draft 2020-12.
- **Structural** (CHANGELOG class): YAML rule file at `vsdd-core/schemas/changelog.yaml` declares structural rules (top-structure patterns, section-structure regexes, canonical-category enums, entry-pattern regex). Validator dispatches per-rule.

Schemas carry their own metadata frontmatter (yes, schemas-about-schemas):

```yaml
---
schema_class: <class-slug>
schema_version: <semver>
validation_mode: frontmatter | structural
status: stable | draft | deprecated
last_breaking_change: <semver | null>
---
```

### Code generation pipeline

Rust types are source-of-truth; JSON Schemas generated via [`schemars`](https://github.com/GREsau/schemars) at build time.

```rust
// vsdd-core/src/schemas/review_entry.rs
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ReviewEntryFrontmatter {
    pub review_number: u32,
    pub date: NaiveDate,
    pub phase: Phase,
    pub scope: String,
    pub lens: String,
    pub source: SourceKind,
    // ... 
}
```

At build time: `cargo build` runs `schemars` derive macros → produces `vsdd-core/schemas/review-entry.json`. Python hooks + Rust mirror + LSP (v1+) all read the generated JSON.

**One type system; multiple consumers:** the Python hook + the Rust mirror + the LSP read the same generated schemas. Operator-local + CI cannot drift in what they enforce.

For the structural CHANGELOG class: rules are hand-authored in YAML (no Rust-type generation; rule-based dispatch in the validator).

### Validation modes

| Mode | Classes | Source format | Validator dispatch |
|---|---|---|---|
| **Frontmatter-based** | Review entry, Finding, Phase primer, Domain prompt, Supplement, Methodology event variants, `.vsdd/config.yaml`, DESIGN doc, Methodology spec section, Manual-test, Exit Signal record, PR template | JSON Schema draft 2020-12 | Parse frontmatter → validate against schema |
| **Structural** | CHANGELOG | Hand-authored YAML rule file | Apply rule-set; emit per-rule errors |

Both modes share the error-catalog + Mentor-voice output + observability emission (`HookFired` + `ValidationPassed` / `ValidationFailed` events).

---

## Per-artifact-class schemas

**13 classes** (revised down from 15 by Phase 3 multi-domain review — two classes consolidated away):

- MCP tool I/O class **dropped** — MCP protocol natively validates tool schemas; vsdd's 4 tools are documented in DESIGN-OBSERVABILITY § MCP server tool surface + the methodology spec; the schemas live as Rust code in `vsdd/src/mcp_serve/` without needing a separate artifact-class validation layer.
- Pre-phase composition declaration class **dropped** — folded into the `PhaseCompositionDeclared` methodology event variant's payload schema. The declaration IS the event payload; the standalone class was redundant. Phase-boundary commits emit the event; the event itself is the declaration; the event-variant schema validates.

For each remaining class: purpose, validation mode, frontmatter shape (frontmatter classes) or structural rules (CHANGELOG), required + optional fields, enum constraints, cross-field validation, drift patterns caught, error codes that fire, semantic version baseline.

### Review entry

**Purpose:** Per-domain review-log entry within a Phase 3 cycle. The canonical audit-trail artifact.

**Validation mode:** Frontmatter-based.

**File pattern:** `vsdd-suite/review-log/YYYY-MM-DD-<domain-slug>.md` (in projects using vsdd) OR `review-log/YYYY-MM-DD-<domain-slug>.md` (at vsdd-cli's own repo).

**Frontmatter shape:**

```yaml
---
schema_class: review-entry
schema_version: 1.0.0
review_number: <u32>             # cycle review number; deterministic per cycle
date: <ISO 8601 date>            # 2026-05-27
phase: <Phase enum>              # phase-1a | phase-1b | ... | phase-6
scope: <string>                  # one-line scope statement
lens: <string>                   # what dimension(s) this review applies
source: <SourceKind enum>        # domain-raised | director-raised | regression-replay | external-feedback | mixed
session_note: <string>           # cold-context / warm-handoff / etc.
model: <string>                  # claude-opus-4-7 / claude-sonnet-4-6 / claude-haiku-4-5
execution_method: <string>       # inline main session / 4-cluster cold-session spawn / etc.
sycophancy_compensation: <string | null>  # required when reviewer authored artifact under review; null otherwise
---
```

**Enum constraints:**

- `phase`: enumeration of 10 whitepaper-canonical phases
- `source`: 5-element enum per primer 3 § Source field discipline

**Cross-field validation:**

- `sycophancy_compensation` REQUIRED if the commit's git-author overlaps with the review's authoring identity (hook detects via git log + frontmatter)

**Drift patterns caught:**

- Source-field defaulting silently (`VSDD-E0012`, candidate)
- Sycophancy-compensation absence when applicable (`VSDD-W0010`, accepted)
- Machine-readability format drift (per primer 3 + the audit-trail discipline)

**Semantic version baseline:** 1.0.0 (v1 ship-state).

### Finding

**Purpose:** Individual finding within a Review entry. Carries classification + routing.

**Validation mode:** Frontmatter-based (per-finding frontmatter block within review-entry body).

**Frontmatter shape:**

```yaml
finding_id: <string>             # deterministic: "{review_number}-f{finding_number}"
domain: <string>                 # domain-slug
dim: <u32>                       # which dimension of the domain applies
owner: <string>                  # domain-slug — who fixes
status: <FindingStatus enum>     # open | in-progress | resolved | dismissed | hallucinated | accepted-rationale
blocked_by: [<finding_id>...]    # cross-finding dependencies
validator: <string>              # domain-slug from canonical validator-pair map | sanity-check
classification: <ClassificationKind enum>  # resolved | deferred | dismissed | hallucinated | accepted | ...
source: <SourceKind enum>        # inherits from Review entry; redeclaration per-finding for cross-source mixes
routing:
  target_phase: <Phase enum | null>
  target_artifact: <path | null>
  target_section: <anchor-id | null>
dismissal_rationale: <string | null>   # required when classification ∈ {dismissed, hallucinated, accepted}
```

**Enum constraints:**

- `classification`: per-domain `classification_universe` declared in the domain prompt
- `status`: 6-element FindingStatus enum
- `source`: 5-element SourceKind enum (matches Review entry)

**Cross-field validation:**

- `classification` MUST be in the domain's `classification_universe` (declared in domain-prompt frontmatter); extension fires `VSDD-E0017` (candidate)
- `dismissal_rationale` REQUIRED when `classification` ∈ {dismissed, hallucinated, accepted}
- `validator` MUST match the domain's `validator_pair` declaration; mismatch fires `VSDD-E0013` (candidate)
- `routing` REQUIRED structure when finding routes to another phase; ambiguity fires `VSDD-W0060` (candidate)

**Drift patterns caught:**

- Classification universe extension (`VSDD-E0017`)
- Validator-pair mismatch (`VSDD-E0013`)
- Hallucinated-without-dismissal-rationale (folds into `VSDD-E0017` enum check)
- Routing-target ambiguity (`VSDD-W0060`)

**Semantic version baseline:** 1.0.0.

### Phase primer

**Purpose:** Per-phase primer markdown file at `.claude/commands/vsdd-phase-<id>.md`.

**Validation mode:** Frontmatter-based.

**Frontmatter shape:**

```yaml
primer_id: vsdd-phase-<phase-id>
phase: <Phase enum>
version: <semver>
frequency: <string>              # per-layer | per-project | per-cycle | etc.
governing_skill: true            # always true for phase primers
relevant_domains: [<domain-slug>...]   # per phase-domain composition matrix
supplements_in_scope: [<supplement-slug>...]
```

**Enum constraints:** `phase` ∈ 10-phase enum.

**Cross-field validation:**

- `relevant_domains` MUST match phase-domain composition matrix entry for the declared phase; mismatch fires composition-violation candidate code

**Drift patterns caught:**

- Composition instruction absence (validated via prose-pattern check inside body)
- Frontmatter completeness

**Semantic version baseline:** 1.0.0.

### Domain prompt

**Purpose:** Per-domain methodology prompt at `.claude/commands/vsdd-domain-<slug>.md` + knowledge page registered via `crosslink knowledge import`.

**Validation mode:** Frontmatter-based.

**Frontmatter shape:**

```yaml
domain_slug: <string>            # lowercase-kebab-case
role_titles: [<string>...]       # human-readable role names
tier: <DomainTier enum>          # core | extended | meta
activation_criteria: [<axis-dependency>...]  # per-feature axis names
classification_universe: [<ClassificationKind>...]   # per-domain valid classifications
validator_pair: <string>         # domain-slug | sanity-check
supplements_applied: [<supplement-slug>...]
sycophancy_failure_modes: [<string>...]  # Mentor-voice prose elaborations
extensions: []                   # supplements may extend with per-language sub-dimensions
```

**Enum constraints:** `tier` ∈ {core, extended, meta}; `classification_universe` items ∈ ClassificationKind canonical set + per-domain extensions (require `accepted_rationale` field).

**Cross-field validation:**

- `validator_pair` MUST be a valid domain-slug from the registered domain set OR `sanity-check`
- If `tier: meta`, `activation_criteria` typically empty (meta-domains activate on-demand or via hook)

**Drift patterns caught:**

- Tier/activation drift
- Sycophancy_failure_modes absence (folds into structural completeness check)

**Semantic version baseline:** 1.0.0.

### Supplement

**Purpose:** Per-language or per-interface supplement markdown at `vsdd-cli/supplements/<slug>.md` + knowledge page.

**Validation mode:** Frontmatter-based.

**Frontmatter shape:**

```yaml
supplement_slug: <string>
languages_or_interfaces: [<string>...]
domains_in_scope: [<domain-slug>...]
extensions: []
```

**Drift patterns caught:** Frontmatter completeness.

**Semantic version baseline:** 1.0.0.

### Methodology event variants

**Purpose:** The 18 methodology-specific OTel event variants. Each variant has its own sub-schema; all share the `EventEnvelope` shape.

**Validation mode:** Frontmatter-based (per-event schema; emitted at runtime + validated by the collector before sink).

**Common envelope (all variants):**

```yaml
event_type: <EventType enum>     # 18-variant enum
agent_id: <string>               # crosslink agent identity
agent_seq: <u64>                 # monotonic per-agent sequence
timestamp: <ISO 8601 with timezone>
signed_by: <string>              # SSH key fingerprint
signature: <string>              # SSH signature over canonical event bytes
capture_source: <CaptureSource enum>   # otel-metric | otel-log-event | otel-trace-attribute | vsdd-custom-event | sdk-result-message | usage-api-reconciled | unmeasurable
payload: { ... }                 # per-variant payload (typed)
```

**Per-variant payload schemas:**

- `PhaseEntered { phase, layer, started_at, composed_domains }`
- `PhaseExited { phase, layer, completed_at, exit_status }`
- `PhaseTransitionAttested { from_phase, to_phase, attestation_ref }`
- `FindingRaised { finding_id, domain, dim }`
- `FindingClassified { finding_id, classification, dismissal_rationale? }`
- `FindingRouted { finding_id, target_phase, target_artifact }`
- `ExitSignaled { project, dimension_status_map, install_verification }`
- `SycophancySelfAudit { reviewer_id, compensation_text }`
- `OperatorDirectiveApplied { directive, rationale }`
- `VerificationMiniCycleSpawned { trigger_finding_id }`
- `PhaseCompositionDeclared { phase, composed_domains, composition_mode, memory_isolation, operator_confirmation }`
- `ProtectiveDisciplineEnforced { discipline_id, target_artifact }`
- `AuthMethodDeclared { auth_method, auth_method_credential_source }` — **NO credential value**
- `ProjectInitialized { vsdd_toolkit_version, axes_declared, auth_method, deployed_artifacts_manifest }`
- `ArtifactScaffolded { artifact_class, target_path, source_trigger }`
- `DraftPROpened { pr_number, layer, opened_at }`
- `PRReadyForReview { pr_number, layer, gate_criteria_met }`
- `PRMerged { pr_number, layer, merged_at, exit_signal_pointer? }`

**Credential-exclusion structural property:**

Every variant's payload schema declares:

```yaml
not:
  properties:
    auth_method_credential_value: { }    # forbidden
    api_key: { }                          # forbidden
    bearer_token: { }                     # forbidden
    secret: { }                           # forbidden
    password: { }                         # forbidden
    token: { }                            # forbidden (with regex match for `\\w*_?token` patterns)
```

Schema-validator rejects emission attempts with credential-shaped fields. The AuthMethodDeclared variant explicitly carries `auth_method_credential_source` (env-var name) but NEVER `auth_method_credential_value`.

**Drift patterns caught:**

- Credential leakage in event payload (structural property)
- Capture-source enum drift
- Per-variant payload mismatch

**Semantic version baseline:** 1.0.0.

### `.vsdd/config.yaml`

**Purpose:** Project-level configuration written by `vsdd init`.

**Validation mode:** Frontmatter-based (the whole file is a YAML object; treated as frontmatter equivalent).

**Shape:**

```yaml
schema_class: vsdd-config
schema_version: 1.0.0
per_feature_axes:
  ships_to_users_other_than_developer: <bool>
  network_exposed: <bool>
  persists_managed_schema_data: <bool>
  handles_user_data: <bool>
  safety_critical: <bool>
  formal_verification_candidates: <bool>
  ui_surface: <bool>
  localized: <bool>
  ai_runtime_cost_relevant: <bool>
slo_declarations: [<SLO declaration>...]
signing_config:
  signing_key_fingerprint: <string>
  trust_model_ref: <string>
auth_method:
  operator_local: <AuthMethod enum>             # plan | api_key
  operator_local_credential_source: <string>    # "plan-auth-no-key" | "env:<VAR_NAME>"
  ci: <AuthMethod enum | "none">                # api_key (typical for CI) | "none" (no CI workflows deployed)
  ci_credential_source: <string | null>         # "env:<VAR_NAME>" — env-var name only; null when ci: "none"
auth_attribution_pattern: <AttributionPattern enum>   # per-operator | shared-organizational
```

**Cross-field validation (load-bearing — closes the credential-leakage-via-config attack surface + auth × CI operational impossibility):**

- If `auth_method.operator_local: api_key`, then `auth_method.operator_local_credential_source` MUST match pattern `env:[A-Z][A-Z0-9_]*` (env-var name only; NEVER the credential value)
- If `auth_method.ci: api_key`, then `auth_method.ci_credential_source` MUST match the same env-var-name pattern
- If `auth_method.operator_local: plan`, then `auth_method.operator_local_credential_source` MUST be `plan-auth-no-key`
- The schema validator **rejects `auth_method.ci: plan`** — Plan auth requires operator-interactive session that CI runners cannot provide; declaring plan-auth for CI is structurally invalid (fires `VSDD-E0021: auth-method-plan-incompatible-with-ci`)
- If any `.github/workflows/vsdd-*.yml` exists in the repo + `auth_method.ci: "none"`, fires `VSDD-W0022: ci-workflows-present-without-ci-auth-declared`
- The schema validator **rejects any field matching credential-shaped patterns** (regex against `sk-ant-api03-`, generic Bearer tokens, `sk-[A-Za-z0-9]+`, etc.)

**Drift patterns caught:**

- Auth-method declaration shape violation
- Credential-shaped value in config rejected; fires `VSDD-E0020: invalid-auth_method_credential_source`
- Plan auth declared for CI rejected; fires `VSDD-E0021: auth-method-plan-incompatible-with-ci`
- CI workflows present without CI auth declared; fires `VSDD-W0022: ci-workflows-present-without-ci-auth-declared`

**Semantic version baseline:** 1.0.0.

### DESIGN doc

**Purpose:** Per-DESIGN-doc frontmatter for README, DESIGN-METHODOLOGY, DESIGN-OBSERVABILITY, DESIGN-VERIFICATION, DESIGN-SCHEMA, UPSTREAM-COORDINATION (when authored).

**Validation mode:** Frontmatter-based.

**Shape:**

```yaml
schema_class: design-doc
schema_version: 1.0.0
doc_class: <DocClass enum>       # positioning | design | runbook
version: <semver>
consumes_from: [<doc-name>...]   # DESIGN docs this consumes
produces_for: [<doc-name>...]    # DESIGN docs this produces inputs for
last_revision_trigger: <string>
```

**Cross-field validation:**

- `consumes_from` entries MUST be valid DESIGN-doc names (cross-reference validation)
- `produces_for` entries MUST be valid DESIGN-doc names

**Drift patterns caught:**

- Cross-doc reference resolution (`VSDD-E0010`)
- Stale claims (frontmatter-version vs claimed-content)
- Structural compliance (required sections present)

**Semantic version baseline:** 1.0.0.

### Methodology spec section

**Purpose:** Per-section frontmatter within `methodology.md`.

**Validation mode:** Frontmatter-based (per-section frontmatter blocks).

**Shape:**

```yaml
section_name: <string>
required: <bool>
target_lines: <u32>              # e.g., 25-35
event_variants_referenced: [<EventType>...]
domains_referenced: [<domain-slug>...]
phases_referenced: [<Phase>...]
```

**Cross-field validation:**

- `event_variants_referenced` entries MUST be valid EventType enum values
- `domains_referenced` entries MUST exist as registered domain prompts
- `phases_referenced` entries MUST be in 10-phase enum

**Drift patterns caught:**

- Section non-empty body (folds into structural completeness)
- Cross-reference resolution (`VSDD-E0010`)
- Architectural-decisions reflected (folds into completeness check)

**Semantic version baseline:** 1.0.0.

### Manual-test

**Purpose:** Per-layer manual test artifact at `manual-tests/layer-N.md` OR per-binary at `manual-tests/<binary>.md` OR error-catalog fixtures at `manual-tests/error-catalog/<code>/{should-fire,should-not-fire}/`.

**Validation mode:** Frontmatter-based.

**Shape:**

```yaml
test_class: <TestClass enum>     # install-verification | binary | mcp-tool | integration-cycle | error-catalog-fixture
layer: <u32 | null>              # null for non-layer tests
target_artifact: <path>          # what's being tested
tested_against: <string>         # commit hash or semver
prerequisites: [<string>...]     # tools / env vars / external state
expected_outcomes: [<string>...]
falsifiability_check: <string>   # how do we know this test catches what it claims?
```

**Cross-field validation (per R74):**

- If `test_class: per-layer`, `layer` MUST be declared
- `tested_against` MUST match a valid git ref or semver string

**Drift patterns caught:**

- Preamble completeness (`VSDD-E0018` — `manual-test-preamble-incomplete`)
- Test-class declaration absence
- Falsifiability check absent

**Semantic version baseline:** 1.0.0.

### Exit Signal record

**Purpose:** Phase 6 attestation artifact recording project-terminal convergence.

**Validation mode:** Frontmatter-based.

**Shape:**

```yaml
attestation_class: exit-signal
project: <string>
attestation_commit: <git-sha>
attested_by: <string>            # operator identity (SSH key fingerprint OR github handle)
signature: <string>              # SSH signature over canonical attestation bytes
per_dimension:
  spec: { status: <ExitStatus>, evidence_pointer: <path> }
  test: { status: <ExitStatus>, evidence_pointer: <path> }
  implementation: { status: <ExitStatus>, evidence_pointer: <path> }
  formal_verification: { status: <ExitStatus>, evidence_pointer: <path> }
cross_dimension_consistency_check: <ConsistencyStatus enum>   # pass | fail | not-applicable
install_verification: { status: <ExitStatus>, evidence_pointer: <path> }
retrospective_note: <string | null>
```

**Cross-field validation:**

- `per_dimension` MUST include all 4 dimensions (spec, test, implementation, formal_verification)
- Each `evidence_pointer` MUST resolve to a real file in the project tree
- `signature` MUST verify against `attested_by`'s registered key

**Drift patterns caught:**

- Attestation completeness
- Per-dimension status presence
- Signature verification failure
- Evidence-pointer resolution

**Semantic version baseline:** 1.0.0.

### Pre-phase composition declaration — DROPPED as standalone class

Per Phase 3 multi-domain review consolidation: the pre-phase composition declaration's structure folds into the `PhaseCompositionDeclared` methodology event variant payload (defined under § Methodology event variants above). The standalone class was redundant — phase-boundary commits emit the event; the event payload IS the declaration; the event-variant schema validates.

Cross-field validation (composed_domains matches phase-domain composition matrix; composition_mode: reviewer-cold-session only valid for phase-3) carries forward as event-variant payload schema constraints. Error code `VSDD-E0050: phase-composition-not-declared` continues to fire when no `PhaseCompositionDeclared` event exists at phase-boundary commit.

### PR template

**Purpose:** GitHub PR template at `.github/PULL_REQUEST_TEMPLATE.md` OR `.github/PULL_REQUEST_TEMPLATE/vsdd-layer-pr.md`.

**Validation mode:** Frontmatter-based.

**Shape:**

```yaml
pr_template_version: <semver>
required_fields:
  - scope                          # one-line layer scope
  - phase_coverage                 # phases this PR closes
  - composed_domains               # per-phase composition declarations
  - co_authors                     # TW + DR trailers + other phase-domain trailers
  - manual_tests_section           # auto-generated by `vsdd observe pr-body --layer N`
  - exit_signal_pointer            # required when layer closes
excluded_fields:
  - credential-shaped-patterns     # no auth_token / api_key / bearer fields allowed
```

**Cross-field validation:**

- `manual_tests_section` content MUST follow `manual-tests/layer-N.md` checklist shape
- Body MUST NOT match credential-shaped patterns (anonymization hook scope extends to PR templates)

**Drift patterns caught:**

- Required fields absent (`VSDD-E0080: pr-template-malformed`)
- Credential-shaped fields present (anonymization hook)
- Manual-tests section incomplete (`VSDD-E0090: pr-manual-tests-incomplete`)

**Semantic version baseline:** 1.0.0.

### MCP tool I/O — DROPPED as standalone class

Per Phase 3 multi-domain review consolidation: MCP protocol natively validates tool schemas (each tool registration carries input + output JSON Schemas as part of the MCP protocol contract). vsdd's 4 tools are defined as Rust code in `vsdd/src/mcp_serve/` with derive-macro-generated schemas; cost characteristics are documented in DESIGN-OBSERVABILITY § MCP server tool surface. Layering a vsdd artifact-class validator on top of MCP's native validation added redundancy without proportional safety.

The 4 tool definitions stay; the standalone class drops. Schema validation of MCP tool signatures happens via the MCP protocol itself + `cargo build` of the Rust implementation. Tool signature stability tracked via standard Rust public-API discipline (breaking-change requires semver-major bump).

### CHANGELOG (structural — not frontmatter)

**Purpose:** Project CHANGELOG.md following Keep a Changelog 1.0.0 format with crosslink-cooperation conventions.

**Validation mode:** Structural (whole-file pattern validation).

**Structural rule file** (`vsdd-core/schemas/changelog.yaml`):

```yaml
schema_class: changelog
schema_version: 1.0.0
validation_mode: structural
required_top_structure:
  - header_pattern: '^# Changelog'
  - disclaimer_pattern: 'All notable changes'
  - format_reference_pattern: 'Keep a Changelog'
required_sections:
  - unreleased: '^## \[Unreleased\]$'
  - per_release: '^## \[\d+\.\d+\.\d+\] - \d{4}-\d{2}-\d{2}$'
canonical_categories:
  - Added
  - Changed
  - Deprecated
  - Removed
  - Fixed
  - Security
entry_pattern: '^- .* ?(\(\[?[#A-Za-z]+-?\d+\]?\)?)?$'
sub_section_grouping_allowed: true
forbidden_categories:        # non-canonical category names
  - Improved
  - Updated
  - Misc
  - Other
```

**Rule dispatch** (10 rules; codes per the error catalog):

| Rule | Error code | Status |
|---|---|---|
| Entry presence for substantive commits | `VSDD-W0190` | Accepted |
| Top-structure compliance | `VSDD-W0191` | Accepted |
| Version-section date format | `VSDD-W0194` | Accepted |
| Canonical categories only | `VSDD-W0195` | Accepted |
| File integrity | `VSDD-E0240` | Accepted |
| Category-label alignment | `VSDD-W0192` | Candidate |
| Entry issue reference | `VSDD-W0193` | Candidate |
| Unreleased overflow | `VSDD-W0196` | Candidate |
| Breaking-vs-semver | `VSDD-W0197` | Candidate |
| Entry format consistency | `VSDD-L0050` | Candidate (lint) |

**Drift patterns caught:** see Rule dispatch table above.

**Semantic version baseline:** 1.0.0.

---

## Anchor-ID generation conventions

Anchor IDs are derived deterministically from frontmatter — no hand-authored `<a id="...">` HTML anchors.

| Artifact class | Anchor pattern |
|---|---|
| Finding | `{review_number}-f{finding_number}` (e.g., `5-f3`) |
| Phase primer | `{phase}-{primer_slug}` (e.g., `phase-2a-red-gate`) |
| Domain prompt | `{domain_slug}` (e.g., `quality-engineer`) |
| Supplement | `supplement-{supplement_slug}` |
| Event variant | `event-{event_type}` (e.g., `event-PhaseCompositionDeclared`) |
| Methodology spec section | `section-{section_name}` (kebab-cased) |
| Manual-test | `manual-test-{test_class}-{layer | target_artifact}` |
| Exit Signal record | `exit-signal-{project}-{attestation_commit-short-sha}` |
| PR template | `pr-template-{pr_template_version}` |
| CHANGELOG | (structural; no anchor-ID — whole-file artifact) |

Pre-phase composition declarations: no standalone anchor-ID; the `PhaseCompositionDeclared` event variant payload IS the declaration; events are addressed by `(agent_id, agent_seq)` not anchor-IDs. MCP tool I/O: no anchor-ID; tools are addressed by MCP protocol tool name in the tool registry.

**Validation:** `check-anchor-id-derivation.py` validates anchor-IDs in cross-references match the deterministic pattern. Hand-authored anchors fire `VSDD-W0081: anchor-rename-stale-references` if they drift.

---

## Bypass-marker schema

Two forms supported (hybrid pattern per README + DESIGN-METHODOLOGY):

**HTML comment form:**

```html
<!-- hook-bypass[<hook-id>]: <rationale> -->
```

Pattern: `<!-- hook-bypass\[(?<hook_id>[\w-]+)\]: (?<rationale>.+) -->`

Cross-field validation:
- `rationale` MUST be non-empty (`VSDD-E0016` — `bypass-rationale-missing`)
- `hook_id` MUST match a registered hook (otherwise the bypass is silent + scope-mismatched per `VSDD-W0070` — `bypass-marker-scope-mismatch`)

**Frontmatter form** (for typed artifacts):

```yaml
bypass:
  - hook_id: <hook-slug>
    rationale: <non-empty string>
    pr_approval_label: <github-label-name>      # required at merge-gate
```

**PR-approval label** (per `VSDD-E0030`): CI gate validates that PRs with bypass-marker present carry the operator-declared approval-label. Missing label + bypass present = CI fail.

---

## Error catalog structure

### Catalog file format

`vsdd-core/error-catalog.yaml`:

```yaml
---
schema_class: error-catalog
schema_version: 1.0.0
catalog_version_pinned: 1.0.0
---

codes:
  - code: VSDD-E0040
    severity: error
    status: accepted
    summary: "promised-artifact-missing"
    detail: |
      Cited artifact (typically declared in TODO.md or DESIGN.md commitments)
      does not exist in the project tree.
    note:
      - "per Manual-test class schema"
      - "layer-N phase entry in .vsdd/config.yaml"
    help: |
      Author the missing file following the relevant class frontmatter template.
      OR annotate the deferral with explicit rationale.
    explain_ref: docs/error-codes/VSDD-E0040.md
    evidence_base:
      - "bookmark-cli-manual SO R1 F1"
      - "bookmark-cli-manual DR R1 F3"
    introduced_in: 1.0.0
    deprecated_in: null
    replacement: null
  # ... ~30 codes total at v1
```

### Code-range conventions

| Range | Owner |
|---|---|
| `VSDD-E0001` — `E0099` | DESIGN-SCHEMA frontmatter validators |
| `VSDD-E0100` — `E0199` | DESIGN-VERIFICATION hook violations |
| `VSDD-E0200` — `E0299` | Phase-domain composition violations |
| `VSDD-E0240` — `E0249` | Reserved: CHANGELOG-discipline structural codes |

`W####` codes parallel each range. `L####` codes for style/lint.

### Status-tier discipline

Three tiers per the naming + coinage governance:

- **candidate** — code authored + tests present; validator emits warning rather than blocking; awaits 2nd recurrence to graduate
- **accepted** — multi-recurrence evidence OR operator-directive trigger; validator blocks per severity
- **deprecated** — retired forward-only; carries `replacement` field with migration pointer; never reused

### Forward-only governance

Codes never reused once retired (matches Rust's E0000-series stability). Major-version bump required for:
- Renaming codes
- Removing codes (vs. deprecating)
- Changing severity (e.g., `W####` → `E####`)

Candidate → accepted promotion does NOT require major-version bump (additive only).

---

## Schema versioning

### Per-class semantic versioning

Each artifact class carries its own `schema_version`. Independent evolution: Review entry schema can hit 2.0.0 while Domain prompt stays at 1.0.0.

### Major-version-bump triggers

- Required field added (breaking — old artifacts fail validation)
- Required field removed (breaking)
- Field type changed (breaking)
- Enum value removed (breaking)
- Cross-field validation tightened (potentially breaking)

### Migration mechanism

When a class hits a major-version bump:

1. New schema version published in `vsdd-core/schemas/<class>.json` (v1 stays for backward-compat read)
2. `vsdd verify migrate <class>` subcommand (v1+ candidate) walks existing artifacts of that class; reports drift; offers auto-fix patterns for trivial migrations
3. Existing artifacts retain their declared `schema_version`; validator dispatches per declared version
4. Old version flagged as `deprecated` in the schema file's metadata frontmatter; deprecation deadline declared
5. Eventual removal of the old version after deprecation period

### Catalog-level versioning

The error catalog (`vsdd-core/error-catalog.yaml`) versions independently. Catalog v1.0.0 ships ~30 codes; v1.1.0 adds new candidate codes; v2.0.0 retires deprecated codes.

---

## Cross-DESIGN-doc coordination

### What this doc produces

| Consumer | Consumes from this doc |
|---|---|
| **DESIGN-VERIFICATION** | Per-class JSON Schema files + structural rule files + anchor-ID derivation conventions + bypass-marker schema + error catalog file format + schema-versioning discipline. DESIGN-VERIFICATION implements the validators + hooks against these schemas |
| **DESIGN-OBSERVABILITY** | Event-variant payload schemas + credential-exclusion structural property + capture-source enum. DESIGN-OBSERVABILITY designs the collector + sink wiring + per-event observability emission |
| **DESIGN-METHODOLOGY** | Class enumeration + version baseline + governance discipline (status-tier). DESIGN-METHODOLOGY re-validates against this doc's class list when authoring methodology spec sections |

### What this doc forward-references

| Sibling | This doc forward-references |
|---|---|
| **DESIGN-VERIFICATION** | Hook implementation details + Rust mirror dispatch + LSP integration + `vsdd verify` subcommand surface |
| **DESIGN-OBSERVABILITY** | Event sink format + OTel collector config + dashboard ladder + Usage API extensibility (v1+) |
| **DESIGN-METHODOLOGY** | Methodology spec section authoring + per-domain prompt content + cluster-batching shape + memory isolation discipline |

---

## Implementation order

| Track | Goal-4 surface? |
|---|---|
| 3a — Author 12 per-class JSON Schemas as Rust types via `schemars` (Review entry, Finding, Phase primer, Domain prompt, Supplement, Methodology event variants, `.vsdd/config.yaml`, DESIGN doc, Methodology spec section, Manual-test, Exit Signal record, PR template) | Foundational |
| 3b — Author CHANGELOG structural rule file (`vsdd-core/schemas/changelog.yaml`) | Foundational |
| 3c — Author error catalog v1.0.0 (~30 codes) at `vsdd-core/error-catalog.yaml` with `status` tier discipline | Foundational |
| 3d — Code-generation pipeline: `cargo build` → `schemars` → emit `vsdd-core/schemas/<class>.json` | Foundational |
| 3e — Author per-error-code fixture pairs at `manual-tests/error-catalog/<code>/{should-fire,should-not-fire}/` | QE coverage |
| 3f — Author anchor-ID derivation utility (`vsdd-core/src/anchor.rs`) | Foundational |
| 3g — Author bypass-marker parsing utility (`vsdd-core/src/bypass.rs`) | Foundational |
| 3h — Author schema-versioning + migration utility (`vsdd-core/src/migration.rs`) | Foundational |

Tracks 3a-3h are foundational for DESIGN-VERIFICATION authoring (which consumes the schemas) + DESIGN-OBSERVABILITY (which consumes event-variant payload schemas).

Time estimates intentionally absent per the toolkit's operator-time-binding-without-quantification discipline.

---

## Open decisions deferred

| Decision | Routing |
|---|---|
| `vsdd verify migrate <class>` subcommand surface (v1+ candidate) | DESIGN-VERIFICATION (when migration becomes load-bearing) |
| LSP authoring-time validation server protocol details | DESIGN-VERIFICATION (deferred to v1+) |
| Per-class schema file format inside `vsdd-core/schemas/` — JSON vs YAML for non-Rust-derived schemas | Implementer's call during track 3a |
| Catalog file format extension for `i18n` (translated error messages) | Deferred to v1+ |
| Cross-doc cross-reference resolution rule shape (when references include section-anchors that don't yet exist because the target doc isn't authored) | DESIGN-VERIFICATION (hook tolerates forward-references in dated-pre-stability state per the forward-only-trigger-discipline) |
| Schema validator handling of mixed-language repos (e.g., a project with Rust + Python + JS sources) | DESIGN-VERIFICATION |

---

## Closing

DESIGN-SCHEMA defines the type system the rest of the toolkit's discipline rests on. 13 artifact classes with frontmatter-based + structural validation modes. Error catalog with status-tier discipline. Anchor-ID derivation conventions. Bypass-marker schema. Schema versioning + migration. Credential-exclusion structural property.

The schema layer is small in surface area (~12 JSON Schemas + 1 structural rule file + 1 error catalog file + 4 utility modules) but high in leverage — every downstream validator, every hook, every observability event, every methodology amendment validates against this layer.

**Next:** Author DESIGN-OBSERVABILITY.md + DESIGN-VERIFICATION.md in parallel (both consume from this doc). Then DESIGN-METHODOLOGY revalidates against the trio at the cross-DESIGN-doc closure boundary.
