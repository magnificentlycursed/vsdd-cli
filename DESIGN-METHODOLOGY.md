# DESIGN-METHODOLOGY.md

Design document for the methodology subsystem of the VSDD Suite. Specifies the methodology spec section list, phase primer + domain prompt authoring shape, phase-domain composition mechanisms, Exacting Mentor stance operationalization, per-feature axes activation, cluster-batching defaults, memory isolation discipline, cold-session budget per axis combination, MCP server tool surface, auth method declaration discipline, and acceptance criteria for deliverables.

For positioning: see [`README.md`](./README.md). For sibling subsystem designs: [`DESIGN-OBSERVABILITY.md`](./DESIGN-OBSERVABILITY.md), [`DESIGN-VERIFICATION.md`](./DESIGN-VERIFICATION.md), [`DESIGN-SCHEMA.md`](./DESIGN-SCHEMA.md).

---

## Scope + boundary

The methodology subsystem owns:

- The methodology spec (concise governing prose; ~250-350 lines target)
- 10 phase primers per whitepaper-canonical taxonomy (1a, 1b, 1c, 2a, 2b, 2c, 3, 4, 5, 6)
- 18 domain prompts (16 role + 2 meta: VSDD Methodology + Sanity Check)
- 14 language/interface supplements
- Phase-domain composition mechanisms (matrix + per-primer instruction + pre-phase declaration + check-phase-composition hook)
- Exacting Mentor stance operationalization across primers + domains
- Tone-flex policy (Mentor default + Formal for attestations + schema declarations)
- Per-feature axes → domain activation matrix
- Cluster-batching shape for Phase 3 cycles
- Memory isolation discipline for cold-session reviews
- Cold-session budget per per-feature-axis combination
- MCP server tool surface + content discipline
- Auth method declaration + Security disciplines (operator-facing)
- Cross-DESIGN-doc authoring order + change authority

The methodology subsystem does NOT own:

- Hook implementation details (DESIGN-VERIFICATION)
- Event log sink format + metrics derivation + trace assembly + OTel collector internals (DESIGN-OBSERVABILITY)
- Frontmatter + JSON Schema definitions (DESIGN-SCHEMA)
- Rust crate internals + CLI implementation + cargo workspace structure (DESIGN-VERIFICATION + DESIGN-OBSERVABILITY)
- CI workflow templates + GitHub Secrets pattern + Rust hook-runner mirror (DESIGN-VERIFICATION)
- Anthropic Usage and Cost API integration (DESIGN-OBSERVABILITY; deferred to v1+)

### Platform requirement (v1 scope)

v1 is **GitHub-only**. The methodology's CI-side teeth — bypass-approval label gate (`DESIGN-VERIFICATION § Bypass-marker enforcement`), CODEOWNERS auto-routing for TW + DR composition, SARIF emission for GitHub Code Scanning, CHANGELOG cooperation with `crosslink close`, the dependency-approval discipline's PR-description structure — are all GitHub-API-specific. Adopting projects must be GitHub-hosted to receive the full methodology surface.

Non-GitHub platforms (GitLab, Bitbucket, Forgejo, Codeberg, self-hosted Gitea, sourcehut) are **not supported at v1** and **no commitment is made to support them in v1+**. Future platform support is contingent on adoption evidence and operator-directive. The methodology's `vsdd init --check` pre-flight detects non-GitHub remotes and refuses deployment with explicit operator-facing error.

---

## Cross-DESIGN-doc authoring order + change authority

**Authoring order:** DESIGN-SCHEMA first (foundational; underpins event variants + frontmatter + per-artifact-class schemas) → DESIGN-OBSERVABILITY + DESIGN-VERIFICATION in parallel (each consume schemas) → DESIGN-METHODOLOGY revalidates against the trio at the cross-DESIGN-doc closure boundary.

This document is authored as a working artifact ahead of DESIGN-SCHEMA per operator-time pragmatism — the methodology decisions surface artifact classes; DESIGN-SCHEMA fixes schemas to the surfaced classes; this document re-validates at that boundary.

**DESIGN.md change authority:** A single Solution Owner holds change authority over all 4 DESIGN docs. Per-doc primary author may differ for cognitive ownership (e.g., AI Engineer co-authors DESIGN-OBSERVABILITY; SA co-authors DESIGN-VERIFICATION), but final authority + spec-contract-change discipline lives with one SO. Per SO domain Dim 0 ("DESIGN.md is the contract; the SO holds change authority"), single change authority preserves audit-trail integrity for "Raise to SO" findings across the doc set.

---

## Methodology spec section list

The methodology spec is the toolkit's concise governing prose. Target ~250-350 lines. Lives at `methodology.md` at project root (for vsdd-using-projects) and at `vsdd-cli` repo root (for the toolkit's own canonical spec).

| Section | Purpose | Target lines |
|---|---|---|
| Opening + scope | What VSDD is; what this spec governs; relationship to whitepaper | 20-30 |
| Four governing design goals | Concise restatement; no historical context | 30-40 |
| Phase taxonomy | 10 whitepaper-canonical phases; per-phase one-line summary | 25-35 |
| Phase-domain composition | The composition matrix + 4 enforcement mechanisms | 30-40 |
| Adversarial review stance | Exacting Mentor stance core + 5 lenses + tone-flex policy | 25-35 |
| Domain set | 16 role + 2 meta domains; activation criteria pointer | 20-30 |
| Per-feature axes | Each axis + its downstream activation | 20-30 |
| Forward-only disciplines | Event-log append-only + documentation narrative-preservation | 15-20 |
| Bypass-marker mechanism | HTML comment + frontmatter hybrid | 10-15 |
| Two-audience principle | Humans + agents; structural properties | 15-20 |
| Two cooperating audit-trail layers | Suite-side event log + crosslink-side audit trail | 15-20 |
| Schema versioning | Per-class semantic versioning + forward-only | 10-15 |
| MVR / Exit Signal convergence | Maximum Viable Refinement (per-round) + Exit Signal (project-terminal) | 15-20 |
| Auth method | Plan vs API key per use case + credential disciplines | 15-20 |
| Domain change authority | Single SO; "Raise to SO" routing discipline | 10-15 |
| Closing + cross-references | Pointer to DESIGN docs + whitepaper + crosslink | 10-15 |

Total: ~285-380 lines. Target achievable.

### Methodology version pin discipline

The methodology spec carries a top-level `methodology_version: <semver>` frontmatter field (independent of per-section frontmatter declared by the Methodology spec section artifact class). Per Phase 5 round 1 finding routing (Security F6): the canonical toolkit copy and the per-project deployed copy can drift after toolkit upgrades; without version-pinning the project's audit trail loses the methodology snapshot it was authored under.

**Version-pin mechanism:**

- vsdd-cli's own `methodology.md` declares `methodology_version: <semver>` matching the toolkit semver
- `vsdd init` deploys the toolkit-canonical `methodology.md` to adopting projects with the version stamped in frontmatter
- `vsdd verify check` includes the `check-methodology-version-drift` hook (DESIGN-VERIFICATION § Per-hook deployment matrix) which compares project `methodology_version` against the installed toolkit's bundled version
- Drift fires `VSDD-W0200: methodology-version-drift` (warning; allows commit but surfaces in CI as PR comment)
- `vsdd init --update-methodology` subcommand refreshes the project's `methodology.md` to the toolkit-canonical version + emits `OperatorDirectiveApplied{directive: methodology-version-updated, from: <semver>, to: <semver>}` event

**Why warning not error:** projects may legitimately stay pinned to an older methodology version for stability reasons (mid-cycle upgrade is disruptive; project may prefer to upgrade between cycles). The warning surfaces the drift for operator awareness without blocking commit.

**Acceptance criterion for "methodology spec is complete":**
- Every section in the list has a non-empty body
- Every architectural decision in this DESIGN doc is reflected in at least one section
- Every event variant declared is defined
- Every domain referenced has a corresponding prompt file
- Every phase has its primer
- The `check-methodology-semantics.py` hook validates these mechanically

---

## Phase taxonomy

Strict whitepaper-canonical 10 sub-phases:

| Phase | Name | Governing primer |
|---|---|---|
| 1a | Behavioral Specification | `vsdd-phase-1a` |
| 1b | Verification Architecture | `vsdd-phase-1b` |
| 1c | Spec Review Gate (Decomposition) | `vsdd-phase-1c` |
| 2a | Test Suite Generation (Red Gate) | `vsdd-phase-2a` |
| 2b | Minimal Implementation | `vsdd-phase-2b` |
| 2c | Refactor | `vsdd-phase-2c` |
| 3 | Adversarial Refinement (The VDD Roast) | `vsdd-phase-3` |
| 4 | Feedback Integration Loop | `vsdd-phase-4` |
| 5 | Formal Hardening | `vsdd-phase-5` |
| 6 | Convergence (The Exit Signal) | `vsdd-phase-6` |

Phase 5 + Phase 6 are first-class methodology phases per the whitepaper; projects choose whether to execute them.

---

## Phase-domain composition matrix (detailed)

Each phase composes with its relevant domains. The matrix is load-bearing methodology, enforced at four layers.

| Phase | Composed domains (skill mode unless noted) | Composition rationale |
|---|---|---|
| 1a | SO (primary); UX, Accessibility, Privacy, Localization per per-feature axes | Spec-contract authoring; UX/A11y/Privacy/L10n shape spec for downstream-user-facing projects |
| 1b | SO + SA + QE | Verification architecture needs SA's architecture lens + QE's test-strategy lens + SO's spec-contract authority |
| 1c | SA (primary); SO co-stewards for spec-gate close | Decomposition is architectural; SA-owned; SO validates spec-gate at close |
| 2a | QE (primary) | Red Gate authoring is QE's primary territory; falsifiability check |
| 2b | SE (primary); TW + DR (project-state staleness); QE (test-pyramid maintenance); DE / AI Eng / etc. per per-feature axes | Implementation needs TW + DR composed to prevent README/CHANGELOG/PROCESS staleness during Phase 2b |
| 2c | SE + SA | Refactor needs architectural lens (SA) + implementation lens (SE) |
| 3 | All active domains (cold-session reviewer mode; NOT skill mode) | Canonical Phase 3 multi-domain adversarial pass |
| 4 | None specific (operator-orchestrated routing) | Routing is operator-orchestrated; no specific domain composition |
| 5 | QE + Security + SA | Formal hardening: QE for mutation/fuzz/proptest; Security for adversarial-hardening; SA for purity-boundary-audit |
| 6 | None specific (operator-orchestrated attestation) | Convergence is operator-orchestrated; Exit Signal attestation is single-author |

Phase 3 is the only phase where domains compose as **reviewers** (cold-session sub-agents producing structured findings). All other phases use **skill mode** (operator-interactive lenses applied during work).

### Pre-phase composition declaration template

Per the 4 enforcement mechanisms, each phase entry produces a declaration:

```yaml
phase: <phase-id>
composed_domains: [<domain-slug>, ...]
composition_mode: skill-interactive  # or reviewer-cold-session for Phase 3
operator_confirmation: confirmed
memory_isolation: worktree-no-memory  # or container-isolated for Phase 3 high-stakes
declared_at: <iso-timestamp>
```

Declaration emits a `PhaseCompositionDeclared` observability event. Absent declaration at a phase-boundary commit is itself a finding for the VSDD Methodology meta-domain.

### Layer-cycle PR discipline (operationalization)

Each layer's work lands in a single PR opened early + accumulated incrementally. Closes the existing-suite Layer 1 anti-pattern (CI/CD work concentrated at layer-close).

**Draft PR opens at Phase 2a commit.** The PR accumulates the layer's commits + receives early review feedback; closes when layer-gate criteria are met. Hook `check-draft-pr-presence.py` fires at Phase 2a commits + emits `VSDD-E0070: draft-pr-missing` if no draft PR exists for the layer.

**PE composes with Phase 2b.** When Phase 2b adds a dependency, the corresponding PE artifact (lockfile, audit gate, env pin) lands in the same commit. Phase-domain composition matrix entry for Phase 2b includes PE explicitly.

**PR template artifact class.** PR description follows a templated structure (PR template class schema):

```yaml
required_fields:
  - layer_scope                      # one-sentence scope statement
  - phase_coverage_checklist         # phases this PR closes
  - composed_domains_per_phase       # phase-domain composition declarations
  - co_authors                       # TW + DR trailers for prose surfaces
  - manual_tests_section             # auto-generated by `vsdd observe pr-body --layer N`
  - exit_signal_pointer              # required when layer closes; pointer to ExitSignaled event
excluded_fields:
  - credential-shaped patterns       # per Security disciplines; mechanically enforced
```

**TW + DR cross-phase composition.** `.github/CODEOWNERS` auto-routes:
- TW for prose-surface paths (`README.md`, `DESIGN-*.md`, `PROCESS.md`, `CHANGELOG.md`, `manual-tests/`)
- DR for PR description review (cold-reader pass)

**Commit-level domain co-authorship (parallel to PR co-authorship).** Same discipline extends to commit-level via git `Co-authored-by:` trailers. Convention:

```
# Prose-surface commit message:
<subject>

<body>

Composed-domains: <comma-separated domain slugs for the phase>

Co-authored-by: Technical Writer <tw@vsdd-domains>
Co-authored-by: Documentation Reviewer <dr@vsdd-domains>
[other domain co-authors per phase composition]
```

Synthetic `@vsdd-domains` email signals domain-lens attribution (not a real person). The `check-prose-surface-tw-dr-composition.py` hook validates either form (`Composed-domains:` trailer OR `Co-authored-by:` trailers); the `Co-authored-by:` form is preferred — richer audit trail (surfaces in `git log` / `git shortlog` / `git blame`); discoverable via standard git tooling; aligns with the existing AI-co-authorship convention (`Co-Authored-By: Claude Opus 4.7 <noreply@anthropic.com>`).

Phase-domain matrix extends to commit-level. Each phase's composed-domains attribute to commits in that phase via Co-authored-by trailers:

| Phase | Co-authorship trailers required for commits touching prose/code |
|---|---|
| 1a / 1b spec authoring | SO (primary) + TW + DR (prose surfaces) + axes-activated domains |
| 1c decomposition | SA + SO + DR (cold-reader spec-gate) |
| 2a Red Gate | QE |
| 2b implementation | SE + TW + DR (prose-surface updates) + QE + axes-activated |
| 2c refactor | SE + SA |
| 3 adversarial refinement | (cold-session reviewer dispatch; not commit-time authorship) |
| 4 routing | operator-orchestrated (no domain co-authors) |
| 5 formal hardening | QE + Security + SA |
| 6 convergence | operator-single-author (no domain co-authors; attestation is operator-attributed only) |

Hook dispatch: for each touched file in commit, identify phase from `.vsdd/config.yaml` + active-cycle state, look up composed_domains, validate trailers present. Missing trailers fire `VSDD-W0180: prose-surface-commit-without-tw-dr-composition` warning OR bypass-marker required with rationale.

**Manual-test checklist (auto-generation).** `vsdd observe pr-body --layer N` reads `manual-tests/layer-N.md` + embeds checkbox items into PR description. Single source of truth; no copy-paste drift. Operator marks items checked in PR UI as they execute manual testing. `check-pr-manual-test-completion.py` validates all items checked or deferred-with-rationale per primer 1c discipline; fires `VSDD-E0090: pr-manual-tests-incomplete` if violated.

**PR-lifecycle events:** `DraftPROpened` (Phase 2a boundary) → `PRReadyForReview` (layer-gate close; manual tests complete) → `PRMerged` (final). Two new hooks: `check-draft-pr-presence.py` + `check-pr-template-conformance.py`.

---

## Adversarial review stance operationalization

The Exacting Mentor stance operationalized across primer 3 + per-domain prompts.

### Primer 3 framing prompt (~40-line target)

Primer 3 opens with the Exacting Mentor stance description + the 5 lenses + the sycophancy-check failure modes specific to Phase 3 cold-session review.

```markdown
# Phase 3 Primer: Adversarial Refinement (The VDD Roast)

You are an experienced reviewer who has seen this defect class before. You hold the work
to the standard you know it can meet — because you believe the author can reach that
standard, not because you're suspicious of them. Your tone is direct, specific, and
exacting. You don't pull punches; you also explain why something is wrong + what the
better version looks like + what corrective pattern applies. Sycophancy resistance is
rooted in standards: letting a defect slide because the author tried hard would be the
failure mode.

[5 lenses elaborated: attacker, edge cases, usability, maintainability, consistency]

[Phase 3-specific sycophancy guards: cold-context discipline; no prior-cycle memory;
 structured-finding output; per-cycle pre-declaration check]

[Cluster-batching shape: 4-cluster default; adversarial-pair-separation invariant
 (Security ↔ Red Team on different agents; TW ↔ DR on different agents)]

[Confidentiality-aware citation discipline]
```

### Per-domain sycophancy-check failure modes

Each domain's frontmatter declares `sycophancy_failure_modes: [<list>]`. The list is Mentor-voice prose elaborations for the specific failure classes the domain owns.

**Worked examples (QE + Security):**

QE `sycophancy_failure_modes`:
- Tests that pass against an empty function body
- Coverage as a substitute for falsifiability
- Smoke tests masquerading as integration tests
- The mutation survivor you can rationalize away

Security `sycophancy_failure_modes`:
- Input validation that trusts the trust boundary
- Catch-all error handlers that swallow security-relevant errors
- The mitigation you adopted without verifying the attack class still applies
- Threat modeling as a checklist exercise

Per-domain elaborations across 18 domains (~3-5 failure modes each = ~60-90 entries). Each is Mentor-voice: names the failure mode specifically + the corrective pattern.

### Tone-flex policy applied per surface

| Surface | Tone | Owner |
|---|---|---|
| Per-finding bodies | Mentor | Reviewer (cold-session sub-agent or operator-skill-mode) |
| Phase 3 Round close summaries | Mentor | Reviewer |
| Layer-gate close narratives | Mentor | Operator + Reviewer |
| Sycophancy-check failure modes | Mentor | Domain-prompt author |
| Hook output messages on failure | Mentor | DESIGN-VERIFICATION (hook authors) |
| Phase 6 Exit Signal record | Formal (signed attestation) + optional Mentor retrospective | Operator |
| Methodology amendment landing | Formal | Operator (SO authority) |
| Schema definitions + JSON Schemas | Formal (typed declarations) | DESIGN-SCHEMA |
| Methodology spec opening | Formal positioning + Mentor examples | Methodology spec author |

---

## Substrate composition

The rebuild composes against the Claude Agent SDK as the primary integration substrate. The Agent SDK runs the Claude Code CLI as a subprocess and emits OpenTelemetry telemetry + programmatic per-message cost data. Methodology decisions on substrate composition:

**v1 scope: Claude Code CLI (via Agent SDK).** Direct Anthropic Messages API integration (bypassing the CLI) is out of scope for v1. Both Plan auth and API-key auth work through the Agent-SDK-runs-CLI path. The architecture is designed extensibly so Messages-API-direct can be added in v1+ if operator-time permits without redesign.

**Agent SDK + OpenTelemetry as observability primitive.** Methodology-specific event variants augment the SDK's built-in OTel signals; the toolkit does not reinvent token/cost/cache/tool/hook capture. This closes the "substrate observability gap" framing that prior pre-design iterations carried — when OTel export is enabled, full observability is built in.

**OTel collector deployment is a core rebuild feature.** `vsdd init` deploys a default OTel collector configuration (`.vsdd/otel-collector.yaml`). Default sinks: `.vsdd/events.jsonl` (suite-side audit trail) + crosslink hub (when crosslink in use). External-backend endpoints declared as commented examples; operator-extensible via single-config-edit. Goal 3's flagship status depends on bundled observability being default-on.

**Cost-tally tier discipline retired; replaced with capture-source provenance.** Every cost-relevant event carries `capture_source` (otel-metric / otel-log-event / otel-trace-attribute / vsdd-custom-event / sdk-result-message / usage-api-reconciled / unmeasurable). Operator-paste of `/cost` is not load-bearing once Agent SDK OTel export is the canonical capture pattern.

**Authoritative billing via Anthropic Usage and Cost API is deferred to v1+.** SDK's `total_cost_usd` (client-side estimate from bundled price table) drives in-cycle decisions in v1. Per-project reconciliation against the Usage API is a v1+ scope-expansion that builds on the data-source abstraction in DESIGN-OBSERVABILITY.

---

## Auth method declaration

Auth method declared explicitly in `.vsdd/config.yaml` per project; no implicit default. Operator declares at `vsdd init` time + may change per `AuthMethodChanged` event.

### Per-use-case recommendation

| Use case | Auth method | Rationale |
|---|---|---|
| Operator-local exploration + Phase 1a-2c primer skill mode | Plan (Max/Pro) + Agent SDK | Agent SDK credits don't consume interactive limits; 1h prompt-cache TTL auto-enabled; appropriate for individual operator pace |
| **Goal 4 CI/CD pipeline + Phase 5 hardening tool runs + scheduled cron sweeps** | **API key + Agent SDK** | Predictable pay-as-you-go billing per Anthropic's own recommendation for automation; full per-token visibility; no plan-credit boundary risk |
| Phase 3 swarm reviews (operator-orchestrated, multi-agent compounding cost) | API key OR Plan based on operator preference + cycle scale | Small cycles can fit Plan; large cycles (10+ parallel agents) likely overflow into API |
| Methodology drift sweeps (scheduled via CronCreate) | API key + Agent SDK | Automated; no operator-in-loop; API-direct fits the automation pattern |

### Prompt-cache TTL behavior

- **Plan auth (Max/Pro):** Anthropic automatically grants 1-hour prompt cache TTL. Operator-local cycles with sub-1-hour-gap multi-session work benefit from cache reuse without configuration.
- **API key auth:** default 5-minute prompt cache TTL. Operator opts into 1-hour TTL via `ENABLE_PROMPT_CACHING_1H=1` env var when cycle-shape benefits from longer TTL. 1h TTL writes cost more per write but enable longer cache reuse — favorable for multi-cycle / multi-session work.

The methodology spec surfaces this auth-method × caching-behavior delta. Operators choose auth method per use case with caching implications named explicitly.

### Security disciplines for credential handling

- **`.vsdd/config.yaml`** carries auth-method-name + credential-source-reference only (e.g., `auth_method_credential_source: env:ANTHROPIC_API_KEY`); NEVER credential value. Schema validator rejects credential-shaped fields.
- **Anonymization hook** (`check-anonymization.sh`) detects API-key formats: `sk-ant-api03-...`, generic `Bearer <token>` headers, env-var-assignment-with-credential-shaped-value. Pattern table maintained in DESIGN-VERIFICATION.
- **Event-variant credential exclusion:** all 18 methodology event variants exclude credential-shaped fields structurally. The schema validator rejects any event-emission attempt that includes API key material.
- **`OTEL_LOG_RAW_API_BODIES`** opt-in env var (which would include API request bodies in OTel exports) stays default-off; the methodology recommends this stays off + names it as security-relevant in operator-facing docs.
- **OTel collector forwarding to external backends** (Honeycomb, Datadog, Grafana, Langfuse, etc.): collector config explicitly redacts credential-shaped values before forwarding. Each backend's bearer token follows the same env-var-only storage discipline as the primary API key.
- **Audit-trail discipline:** `AuthMethodDeclared` event variant carries auth_method + credential_source (env-var name; never value) at init. `AuthMethodChanged` + `AuthFailureObserved` retired/consolidated per the variant-proliferation governance audit; rate-limit + invalid-credential events covered by Agent SDK OTel signals natively (no methodology-specific variant needed). Auth-rotation events route through `OperatorDirectiveApplied{directive: auth-method-rotation}` consolidated variant.
- **CI integration:** GitHub Secrets pattern (or equivalent CI platform); never echo key value in logs.
- **Per-operator vs shared-organizational keys:** per-operator default (clear attribution); shared-organizational extension activated by `auth_attribution_pattern: shared-organizational` in config.
- **Compromised credential procedure:** documented in operational runbook (revoke at credential issuer → audit event log via Agent SDK OTel signals for cycle activity post-compromise + pre-revocation → emit `OperatorDirectiveApplied{directive: credential-rotation, rationale: <text>}` → reissue + update `.vsdd/config.yaml` env-var-name reference → anonymization regression-check ensures compromised credential not latent in repo history).

---

## Always-on domain baseline

The methodology declares an explicit always-on domain set independent of per-feature axes. Per Phase 5 round 1 finding routing (Security F1 + QE F4): a project that declares all 9 axes as `no` must still have non-empty composed domains for every phase the project executes; the axis matrix is additive over the baseline rather than replacing it.

**Always-on for every project:**

- Software Engineer (SE)
- Quality Engineer (QE)
- Solution Architect (SA)
- Solution Owner (SO)

**Always-on when the project ships code (any source file in `src/`, `lib/`, or equivalent language-conventional directory):**

- Platform Engineer (PE)
- Performance Engineer (PerfE)

**Activation rule for Phase 3 (cold-session reviewer mode):** baseline domains + axis-activated domains, deduplicated. A zero-axes project that ships code activates 6 domains (SE + QE + SA + SO + PE + PerfE); cluster-batching collapses the 6 into 2 clusters (Implementation: SE+QE+PerfE; Architecture: SA+SO+PE) or runs per-domain at high-stakes rounds. The methodology never permits Phase 3 with zero composed domains.

**Rationale:** the per-feature axes matrix encodes incremental specialty activation (Privacy, Accessibility, Localization, etc.); the baseline encodes the load-bearing methodology floor any non-trivial software project requires. Separating the two clarifies what's always-on vs. axis-driven and closes the zero-axes degenerate state surfaced by Phase 5.

---

## Per-feature axes → domain activation matrix

Projects declare axes in `.vsdd/config.yaml`. Each axis activates one downstream calibration **additive over the always-on baseline** (see preceding section).

| Axis | Activates | Cold-session budget impact |
|---|---|---|
| `ships-to-users-other-than-developer: yes` | Documentation Reviewer + Technical Writer (default) | Baseline |
| `network-exposed: yes` | Red Team + Security (extended) | Scale up |
| `persists-managed-schema-data: yes` | Data Engineer | Scale up slightly |
| `handles-user-data: yes` | Privacy | Scale up slightly |
| `safety-critical: yes` | Phase 5 Mutation Testing + Purity Boundary Audit recommended | Scale up substantially (Phase 5 expands) |
| `formal-verification-candidates: yes` | Phase 5 Proof Execution recommended | Scale up further (Phase 5 expands more) |
| `ui-surface: yes` | UX + Accessibility | Scale up slightly |
| `localized: yes` | Localization | Baseline |
| `ai-runtime-cost-relevant: yes` | AI Engineer | Baseline |

Combined budgets scale per axis combination. The methodology spec declares the matrix as load-bearing. `vsdd init` reads `.vsdd/config.yaml` + activates relevant domain skills + emits the per-feature-axes + activated-domains list as an observability event at init time.

---

## Cluster-batching shape for Phase 3 cycles

**Default 4-cluster shape with adversarial-pair separation:**

| Cluster | Domains (multi-domain per agent) | Adversarial-pair separation |
|---|---|---|
| Implementation cluster | SE + QE + Performance Engineer | none (no pair member in this cluster) |
| Architecture cluster | SA + Platform Engineer + Data Engineer | none |
| Communication cluster | Security + TW + Accessibility + Privacy + Localization | Security ↔ Red Team separated; TW ↔ DR separated |
| Adversarial cluster | Red Team + DR + UX + AI Engineer + Solution Owner + VSDD Methodology + Sanity Check | adversarial-pair members + SO-as-validator-routing-target co-located with VSDD Methodology |

Per-cluster agent receives: primer 3 + the cluster's domain prompts + relevant supplements + the project under review.

**Per-domain spawn alternative** (high-stakes rounds): 18 agents, one per domain. Used at layer-close OR MVR-approach rounds where marginal finding matters most.

**Cluster naming:** descriptive (Implementation / Architecture / Communication / Adversarial), not letter-coded. Avoids the letter-cluster anti-pattern where the reader has to look up what each letter means.

---

## Memory isolation discipline

Cold-session sub-agents spawned via `crosslink swarm review` use worktree-isolation + an explicit memory exclusion flag.

**Pattern 1 — worktree + `--no-memory` (default):**

```bash
crosslink swarm review \
  --agents 4 \
  --mandate-from-knowledge vsdd-domain \
  --cluster-shape phase-3-default \
  --no-memory \
  --worktree-isolation strict
```

`--no-memory` excludes `~/.claude/projects/*/memory/` from the worktree. Sub-agent sees only project artifacts + cluster's domain prompts + primer 3 + relevant supplements. No operator-feedback memory poisoning.

**Pattern 2 — container-isolated (high-stakes):**

```bash
crosslink swarm review \
  --agents 4 \
  --container-isolated \
  --mandate-from-knowledge vsdd-domain \
  --cluster-shape phase-3-default
```

Operator runs from fresh container; no operator-memory mount; maximum cold-context discipline. Used for project-terminal Phase 6 cycles + Phase 5 mutation/fuzz/proof-execution cycles.

Memory-isolation choice declared in the pre-phase composition declaration.

---

## Cold-session budget per per-feature-axis combination

Base bands per cycle phase. These are first-pass estimates; refined per evidence as the toolkit's own development cycles produce actual data (the toolkit dogfoods on itself; no separate reference example).

| Phase | Per-round budget (base) | Per-axis multiplier | Notes |
|---|---|---|---|
| 1a / 1b / 1c | Small (skill mode; operator-interactive) | Minor | Spec authoring; not compounding-cost |
| 2a | Small (skill mode + QE composition) | Minor | Red Gate; QE-primary |
| 2b | Moderate (skill mode + multi-domain composition) | Per-feature axes scale | Implementation + TW + DR + QE all in skill mode |
| 2c | Small (skill mode) | Minor | Refactor; SE + SA |
| 3 | Substantial per round (4-cluster cold-session) | Per-feature axes scale | Compounding-cost cycle; pre-cycle declaration required |
| 4 | Small (operator-orchestrated routing) | Minor | Routing |
| 5 | Substantial per round (cold-session + tool runs) | Scale per safety-critical / formal-verification axes | Mutation + fuzz + proof-execution tool runs measured separately |
| 6 | Small (operator-orchestrated attestation) | Minor | Single-author attestation |

Specific token-count thresholds are intentionally absent until the toolkit's own observability subsystem produces cycle telemetry from real data. Pre-cycle declaration discipline (per primer 3) names the cycle's expected shape; after-action cost report closes the loop.

---

## MCP server tool surface

A single MCP server (`vsdd mcp-serve` subcommand) exposes 4 tools to agents in every Claude Code session:

| Tool | Scope |
|---|---|
| `vsdd.methodology.lookup(query, scope?)` | The rebuild's own methodology spec + DESIGN docs + supplements + domain prompts. Scopes: "spec" \| "design-docs" \| "supplements" \| "domains" \| "all" |
| `claude_code.docs.search(query, page?)` | Claude Code documentation (`code.claude.com/docs`) |
| `crosslink.docs.search(query, page?)` | Crosslink documentation |
| `anthropic.api.docs.search(query)` | Anthropic API documentation (`platform.claude.com/docs`); v1 stub returns "not yet implemented"; full implementation in v1+ when Usage API integration lands |

### Internal architecture (DESIGN-OBSERVABILITY owns the implementation; methodology owns the content discipline)

The server has three layers:

1. **Tool dispatch** — handles MCP protocol; routes incoming tool calls to handlers
2. **Cache layer** — local cache at `.vsdd/mcp-cache/` (gitignored; ephemeral); TTL-bounded
3. **Fetch primitive** — WebFetch-equivalent for cache misses

### Acceptance criteria for the MCP server

Each tool needs falsifiable tests:
- Query produces expected reference (e.g., "What is the cluster-batching default?" → returns reference to 4-cluster shape + adversarial-pair separation)
- Cache TTL behavior tested (stale-cache + fresh-fetch paths)
- WebFetch error handling (network failures; URL changes)
- Malformed query handling
- Empty result handling

`manual-tests/vsdd-mcp-serve.md` documents per-tool acceptance criteria + the standard run-through pattern.

---

## VSDD Methodology meta-domain

The meta-domain renamed from VDD-IAR Alignment. Validates semantic coherence of methodology application.

**4 surviving dimensions:**

1. **Spec-vs-implementation semantic alignment** — does the implementation faithfully match DESIGN.md's spec contracts? Not just code-correctness; semantic-coherence. Catches drift between what the spec asserts + what the implementation does.

2. **Methodology-spirit adherence** — does the cycle's discipline-application match the methodology's intent? Catches cycles that follow the letter of the methodology (right hook count, right phase sequencing) but violate the spirit (sycophantic reviews, performative pre-cycle declarations).

3. **Cross-session semantic continuity** — does this cycle's terminology, classification, and routing match prior cycles' conventions? Catches naming drift, classification universe extensions, routing target divergence across cycles.

4. **Methodology-evolution coherence** — does the toolkit's own methodology change cohere with prior versions? Catches methodology amendments that break narrative-preservation or contradict prior decisions without explicit acknowledgement.

**Activation:** On-demand (not gate-criterion). Operator activates when methodology drift is suspected OR at periodic intervals.

**Validator pair:** SO (when finding raises to spec-change); Sanity Check otherwise.

**Sycophancy_failure_modes** (Mentor voice):
- Methodology violations rationalized as "methodology evolution"
- Classification-universe drift accepted without explicit operator-policy decision
- Pre-cycle declarations that performatively check boxes without actual cycle-shape commitment
- Cross-cycle inconsistency dismissed as "this cycle's specific context"

---

## Sanity Check meta-domain

Validator-of-last-resort + rubber-ducking surface.

**Activation:** Automatic via hook when `validator: sanity-check` declared in a finding's frontmatter. The hook fires at finding-close commits + triggers the Sanity Check skill (operator-interactive review).

**Hook config:** DESIGN-VERIFICATION specifies the mechanical implementation.

**Sycophancy_failure_modes:**
- Validator-of-last-resort role not invoked when other validators have clear conflict-of-interest
- Sanity-check finding-validation perfunctory (checkbox-style) when substantive cross-domain coherence is at stake

---

## Phase primer authoring guidelines

Each phase primer is a `.claude/commands/vsdd-phase-<id>.md` file deployed by `vsdd init`. Per-primer shape:

```markdown
---
primer_id: vsdd-phase-<id>
phase: <phase-id>
version: <semver>
frequency: <per-layer | per-project | etc.>
governing_skill: true
relevant_domains: [<domain-slug>, ...]
supplements_in_scope: [<supplement-slug>, ...]
---

# Phase <id> Primer: <name>

[Composition instruction — explicit; names the relevant_domains as skill-mode loads]
[Phase-specific discipline — what the operator is doing in this phase]
[Pre-phase declaration template — what to record before phase work begins]
[Composition with the Exacting Mentor stance — where applicable]
[Phase-completion criteria — what closes the phase + emits PhaseExited]
[Cross-references to other primers + domains + supplements]
```

Per-primer target: ~50-80 lines (primer 3 may be longer to carry the full stance + cluster-batching discipline; other primers shorter).

Total primer content: ~600-800 lines across 10 primers.

**Acceptance criterion for "phase primer is done":**
- Frontmatter validates against the primer JSON Schema
- Composition instruction names relevant_domains explicitly
- Pre-phase declaration template present
- Phase-completion criteria specified
- Cross-references resolve mechanically (no dead links)
- The `check-methodology-semantics.py` hook validates

---

## Domain prompt authoring guidelines

Each domain prompt is a knowledge page + a `.claude/commands/vsdd-domain-<slug>.md` skill. Per-domain shape:

```markdown
---
domain_slug: <slug>
role_titles: [<title>, ...]
tier: core | extended | meta
activation_criteria: [<axis-dependency>, ...]
classification_universe: [resolved, deferred, dismissed, hallucinated]
validator_pair: <domain-slug | sanity-check>
supplements_applied: [<supplement-slug>, ...]
sycophancy_failure_modes:
  - <failure-mode-1>
  - <failure-mode-2>
extensions: []  # supplements may extend with per-language sub-dimensions
---

# <Domain Name> Review

[Opening: domain purpose + the Exacting Mentor stance applied to this domain's lens]
[Standard Evaluation Dimensions — numbered list; per-dim Mentor-voice prose]
[Validator pair operationalization]
[Coordination — flag findings to which other domains]
[DESIGN.md change authority — Raise-to-SO discipline]
```

Per-domain target: ~80-150 lines. Vestigial patterns retired: job-title variants → frontmatter; sycophancy-check failure modes → frontmatter + methodology spec; coordination matrix → methodology spec once; validator-pair paragraph → methodology spec mapping table; three-audience lens section retires; classification universe extensions → single universe with rationale-required `Accepted`.

Total domain prompt content: ~1,500-2,000 lines across 18 domains.

**Acceptance criterion for "domain prompt is done":**
- Frontmatter validates against the domain JSON Schema
- All standard evaluation dimensions in Mentor voice
- Sycophancy_failure_modes list non-empty + Mentor-voice
- Validator pair declared + maps to a real validator
- Cross-references resolve mechanically

---

## Document artifact validation surface — design

The schema/hook/error-catalog system is **Goal 2's primary operationalization**: auditable + machine-enforceable + dual-audience-readable. Treats document artifacts the way Rust treats source code — schema validation at commit-time (hooks); PR-time (CI Rust mirror); authoring-time (LSP — v1+).

### 13 artifact classes with schema discipline

| Class | Drift patterns caught |
|---|---|
| Review entry | Source-field defaulting, sycophancy-compensation absence, machine-readability drift, audit-trail format |
| Finding | Classification-universe extension, validator-pair mismatch, hallucinated-without-dismissal-rationale, routing-target ambiguity |
| Phase primer | Composition instruction absence, frontmatter completeness |
| Domain prompt | Tier/activation drift, sycophancy_failure_modes absence |
| Supplement | Frontmatter completeness |
| Methodology event variants | Credential-exclusion structural property, capture-source provenance |
| `.vsdd/config.yaml` | Auth-method declaration shape, credential-shaped field rejection |
| **DESIGN doc** | Cross-doc reference resolution, stale claims, structural compliance |
| **Methodology spec section** | Section non-empty body, cross-references resolve, architectural-decisions reflected |
| **Manual-test** | Preamble completeness (R74), test-class declaration, falsifiability check |
| **Exit Signal record** | Attestation completeness, per-dimension status, signature presence |
| **PR template** | Required fields present, credential-shaped fields excluded, manual_tests_section auto-generated |
| **CHANGELOG** (structural; not frontmatter-based) | Keep-a-Changelog structural compliance (header + disclaimer + format-ref + `[Unreleased]`), canonical categories (Added/Changed/Deprecated/Removed/Fixed/Security), entry presence for substantive commits, version-section date format, file integrity, candidate rules (category-label alignment, issue-reference presence, unreleased overflow, breaking-vs-semver, entry format consistency) |

Pre-phase composition declaration folds into the `PhaseCompositionDeclared` event variant payload (event-variant schema validates the declaration shape at phase-boundary commit time). MCP tool I/O is validated by the MCP protocol natively (each tool registration carries input + output JSON Schemas as part of the protocol contract); cost characteristics live in DESIGN-OBSERVABILITY.

CI workflow template meta-schema class reserved as `candidate`; promotion to `accepted` requires a second recurrence case beyond the single existing-suite evidence case. v1 ships without it; CI workflow YAML validates against GitHub's own schema only.

### Error catalog (~25 accepted + ~15 candidate at v1)

Registered at `vsdd-core/error-catalog.yaml`. Forward-only; retired codes deprecated with migration pointer; per-major-release catalog versioning. Each code carries `status: candidate | accepted | deprecated`. Candidate codes ship as warnings pending second-recurrence promotion to accepted; accepted codes block per declared severity.

Each code carries:

```yaml
error_code: VSDD-E0040
severity: error  # error | warning | lint
summary: "promised-artifact-missing"   # Mentor voice; one-line
detail: |                              # multi-line explanation
  TODO.md:138 commits to manual-tests/layer-3.md but file does not exist.
note:                                  # contextual facts
  - "per Manual-test class schema"
  - "layer-3 phase entry in .vsdd/config.yaml"
help: |                                # corrective pattern (Mentor voice)
  Author manual-tests/layer-3.md following the Manual-test class frontmatter template.
  Or, if Layer 3 manual-tests are intentionally deferred, annotate with rationale.
explain_ref: docs/error-codes/VSDD-E0040.md
```

Code-range conventions:
- `VSDD-E0001`-`E0099`: DESIGN-SCHEMA frontmatter validators
- `VSDD-E0100`-`E0199`: DESIGN-VERIFICATION hook violations
- `VSDD-E0200`-`E0299`: Phase-domain composition violations
- W codes parallel; L codes for style/lint

### Validator architecture

**Three layers (separation):**
1. **Schema definitions** (DESIGN-SCHEMA owns): the type system at `vsdd-core/schemas/<artifact-class>.{json,yaml}` per class; two validation modes:
   - **Frontmatter-based** (14 classes): JSON Schema validates YAML frontmatter at top of markdown files
   - **Structural pattern-based** (CHANGELOG class): rule-based whole-file validation; regex + section-structure checks; canonical enum constraints
2. **Validators** (DESIGN-VERIFICATION owns): hooks + Rust mirror execute schemas; one rule, two enforcement surfaces (operator-local Python + CI-side Rust mirror); per-class dispatch routes to frontmatter or structural validator
3. **Emission/aggregation** (DESIGN-OBSERVABILITY owns): `HookFired` + `ValidationPassed` / `ValidationFailed` events to OTel collector + event log

**One schema source; multiple consumers:** the Python hook + the Rust mirror + the LSP (v1+) all read the same schema files. Operator-local + CI cannot drift in what they enforce. Both validation modes share the error-catalog + Mentor-voice output + observability emission.

### How the system interfaces

**Schema ↔ Hooks:** schemas are the rules (type system); hooks are the executors. Hooks emit `HookFired` events at every invocation; validation failures additionally emit `ValidationFailed{error_code, location, summary, detail, help}`.

**Schema ↔ Reviews:** schemas catch mechanical drift at commit-time; reviews catch judgment-bearing concerns at cycle-time. Phase 3 reviewers operate on artifacts where schema-validation has already passed — attention shifts from mechanical-defect-finding to semantic-coherence + sycophancy + threat-modeling. Review-log entries are themselves schema-validated.

**Schema ↔ Goal 2:** the three-mechanism enforcement triad (schema validator OR hook OR crosslink workflow check) converges at the user-facing error catalog. Operator sees `VSDD-E0040` regardless of which mechanism caught the rule. Auditable (every validation emits events); machine-enforceable (rule → mechanical enforcement); dual-audience-readable (humans see Mentor-voice errors + `vsdd verify explain <code>` extended docs; agents consume structured frontmatter + SARIF + events).

### Output formats

- TTY: color-coded human-readable (rustc convention)
- `--format sarif`: machine-readable for CI integration (GitHub Code Scanning)
- `--format json`: programmatic consumers
- `--format compact`: terse single-line

### Validator falsifiability

Per-code test fixtures at `manual-tests/error-catalog/<code>/{should-fire,should-not-fire}/`. `vsdd verify test-error-catalog` runs the regression suite. Per QE dim 2: every error code's positive + negative fixture proves the validator catches what it claims + doesn't fire spuriously.

### Cost discipline

Per AI Engineer dim 9: each validator < 1s wall-clock for typical artifact size. Selective execution (only validate files in `git diff --name-only`). Cumulative cost across the hook chain tractable for ~50-file commits. Matches Rust's incremental compilation model.

### Security disciplines for validator output

Per Security pass: all validator output runs through credential-redaction pass before emitting to any surface (TTY / CI logs / event log / SARIF / LSP). Validators sanitize their own error output — a malformed credential in `.vsdd/config.yaml` is `[REDACTED-VALUE-MATCHING-CREDENTIAL-PATTERN]` in the error message, not echoed raw.

---

## CHANGELOG discipline (design)

The toolkit adopts crosslink's CHANGELOG management pattern verbatim. CHANGELOG.md follows [Keep a Changelog 1.0.0](https://keepachangelog.com/en/1.0.0/) format with crosslink-compatible conventions.

### CHANGELOG as 15th artifact class

CHANGELOG.md is the only class with a **structural schema** (rule-based whole-file validation) rather than frontmatter-based. The validator architecture must handle two modes:

- **Frontmatter-based validation** (14 classes): JSON Schema validates YAML frontmatter at top of markdown files
- **Structural pattern-based validation** (CHANGELOG class): rule-based whole-file validation; regex + section-structure checks; canonical category enumeration

Both modes share the same error-catalog + observability emission + Mentor-voice output. Hooks dispatch to the appropriate validator based on artifact class.

### Consolidated `check-changelog-discipline.py` hook

10-rule multi-dispatch:

| Rule | Error code | Status |
|---|---|---|
| Entry presence for substantive commits | `VSDD-W0190: changelog-entry-missing` | Accepted (operator-directive) |
| Keep-a-Changelog structural compliance | `VSDD-W0191: changelog-structure-malformed` | Accepted |
| Version-section date format | `VSDD-W0194: changelog-version-section-missing-date` | Accepted |
| Canonical categories only | `VSDD-W0195: changelog-non-canonical-category` | Accepted |
| File integrity (no deletion post-init) | `VSDD-E0240: changelog-deleted` | Accepted |
| Category-label alignment | `VSDD-W0192: changelog-category-label-mismatch` | Candidate |
| Entry issue/PR reference | `VSDD-W0193: changelog-entry-without-issue-reference` | Candidate |
| Unreleased overflow threshold | `VSDD-W0196: changelog-unreleased-overflow` | Candidate |
| Breaking-vs-semver alignment | `VSDD-W0197: changelog-breaking-without-semver-major` | Candidate |
| Entry format consistency | `VSDD-L0050: changelog-entry-format-inconsistent` | Candidate (lint) |

5 accepted (operator-directive trigger via Keep-a-Changelog pattern adoption); 5 candidate (await recurrence evidence OR operator-set thresholds).

### Cooperation with crosslink

When crosslink is the project's issue tracker, `crosslink close` auto-manages CHANGELOG.md (creates template if absent; categorizes by label; appends to `[Unreleased]`). The toolkit's hook detects entries already added by crosslink + passes validation. Never duplicate-writes.

Cross-toolkit-handoff: the validator reads CHANGELOG.md as it exists at commit-time. Authorship is operator-directed (manual entries) OR crosslink-auto-generated. Both flow through the same validator.

### Auto-creation deferral

`vsdd verify changelog --create` candidate v1+ subcommand replicates crosslink's auto-create behavior for projects not using crosslink. Trigger condition for graduation: second VSDD-adopting project running without crosslink + needing CHANGELOG bootstrap. Until then, side-by-side `CHANGELOG.md.vsdd-template` deployment at `vsdd init` covers the non-crosslink path.

### Anonymization scope coverage

The check-anonymization.sh hook scans CHANGELOG.md among other prose-surfaces; credential leakage in CHANGELOG entries (e.g., security-fix entry echoing the credential) is covered structurally by the existing anonymization hook. Explicit cross-reference: CHANGELOG-discipline validator + anonymization hook compose at commit-time; either failure blocks.

---

## Adoption into existing projects — collision handling (design)

vsdd init plays nicely with existing projects. Patterns inherited from crosslink's own `init` collision-handling discipline: managed-section markers + JSON object merge + side-by-side templates + refuse-malformed-file.

### Per-file collision matrix

| File | Strategy | Implementation pattern |
|---|---|---|
| `.gitignore` | Managed-section markers (`# === vsdd managed ===` / `# === End vsdd managed ===`); idempotent in-place replacement on re-init | Inherits crosslink's `write_root_gitignore` pattern; sections preserved outside markers |
| `.claude/mcp.json` | JSON object merge — adds `vsdd` entry to `mcpServers`; warns on key collision; preserves operator entries | Inherits crosslink's `write_mcp_json_merged` pattern |
| `.claude/settings.json` | UNION-merge `allowedTools`; preserves existing hooks block; adds vsdd-* hook entries alongside crosslink's | Inherits crosslink's `write_settings_json_merged` pattern; composes with crosslink (which manages its own hooks) |
| `.github/CODEOWNERS` | Managed-section markers; TW + DR routing rules appended; operator rules preserved | New pattern for vsdd; matches gitignore-managed-section discipline |
| `.pre-commit-config.yaml` | Managed-section markers OR YAML object merge; vsdd-* hooks added to repo list | New pattern for vsdd |

### Files vsdd never touches (operator-owned)

`DESIGN.md` · `README.md` · `CHANGELOG.md` (crosslink-managed when in use; never overwritten by vsdd) · `PROCESS.md` · `Cargo.toml` / `package.json` / `pyproject.toml` · `src/` / `tests/` / `lib/` · existing `manual-tests/*.md` · operator-authored `.github/workflows/*.yml`

### Side-by-side template deployment

| If exists | vsdd deploys |
|---|---|
| `DESIGN.md` | `DESIGN.md.vsdd-template` (operator reviews + adopts manually) |
| `CHANGELOG.md` | (skipped; cooperate with crosslink-managed file OR existing content) |
| `.github/PULL_REQUEST_TEMPLATE.md` | `.github/PULL_REQUEST_TEMPLATE/vsdd-layer-pr.md` (GitHub multi-template directory; selectable from dropdown) |

If `CHANGELOG.md` absent + crosslink not in use: deploy `CHANGELOG.md.vsdd-template` side-by-side.

### Files vsdd owns entirely (vsdd-prefixed; no collision possible)

`.claude/hooks/vsdd-*.py` · `.claude/commands/vsdd-*.md` · `.claude/agents/vsdd-*.md` · `.github/workflows/vsdd-*.yml` · `.github/ISSUE_TEMPLATE/vsdd-*.md` · `.vsdd/` directory tree

### Refusal disciplines

If `.mcp.json` / `.claude/settings.json` / `.pre-commit-config.yaml` / `.vsdd/init-manifest.json` is malformed, vsdd-init bails with `error[VSDD-E0220]: existing-file-malformed-refuse-to-overwrite` — operator fixes or removes + retries. Never destroys operator state. Matches crosslink's refusal-on-malformed discipline verbatim.

### Idempotent re-init via init-manifest

`.vsdd/init-manifest.json` tracks per-file SHA-256 hashes of vsdd-deployed files (matches crosslink's `init-manifest.json` pattern). Re-running `vsdd init` after toolkit upgrade replaces managed sections + merges new entries; operator-edits outside managed sections preserved; operator-edits inside managed sections detected via manifest-SHA mismatch + explicit resolution flag (`--keep-operator-edits` or `--accept-managed-defaults`).

### Pre-flight detection

`vsdd init --check` reports the deployment plan before any writes — which files would be CREATED, MERGED, SKIPPED-WITH-TEMPLATE, or SKIPPED-OPERATOR-OWNED. Operator confirms or aborts. Closes mid-init-inconsistent-state defect class.

### Operator-action queue post-init

Files needing operator-merging (the `*.vsdd-template` side-by-side files) listed in vsdd-init's exit output + the `ProjectInitialized` event manifest. Operator can run `vsdd init --review-deployment` post-init to see the full manifest + suggested integration points.

---

## Naming + coinage governance

The methodology applies disciplines to its own evolution. New terms, event variants, error codes, hooks, and artifact classes do not enter the methodology speculatively. Drawn from recurrence evidence (R78 F4 Surface-lettering + R94 cluster-lettering + multi-cycle vocabulary drift + cycle-by-cycle proliferation patterns documented in existing-suite and bookmark-cli-manual reviews).

### Earned-by-recurrence trigger

Methodology amendments require **2+ documented drift recurrences** OR explicit operator-directive citing equivalent evidence. Single-recurrence additions ship as `status: candidate` (validators exist; emit warnings; do not block merge) and graduate to `accepted` on the second case. Forward-only governance: codes/variants/classes never reused once retired; deprecated entries carry migration pointers.

### Vocabulary registry

Canonical methodology terms live at `.vsdd/registry/vocabulary.yaml` (deployed by `vsdd init`). Each term carries: definition, first-introduced version, deprecated_aliases (for migration pointers), abbreviation (if any), domain_scope. The `check-naming-discipline.py` hook (consolidated; covers letter-labels + suite-internal-terminology + vocabulary-registry conformance) scans documents for deprecated aliases (fires `VSDD-W0001`) + novel-term-without-registry-entry (single-recurrence: candidate code).

TW co-authors term-introducing commits (per the Layer-cycle PR discipline cross-phase composition). DR cold-reader review asks "is this term necessary?" before merge. VSDD Methodology meta-domain reviews term-introductions for methodology-spirit coherence at cycle-close.

### Letter-label anti-pattern enforcement (R78 F4 + R94 multi-recurrence)

`check-naming-discipline.py` fires `VSDD-E0160: letter-label-anti-pattern` for patterns:
- `Surface [A-Z]`, `Cluster [A-Z]`, `Mode [A-Z]`, `Path [A-Z]`, `Form [A-Z]`
- `Tier [A-Z]`, `Pillar [N]` (organizational scaffolding patterns)

**Acceptable** (concept-word in the identifier): `Dim N`, `Layer N`, `Round N`, `Finding N`, `Phase Na`.

Closes the load-bearing R78 F4 + R94 recurrence class structurally. 4 cycles of human-authored drift before mechanical enforcement caught the pattern in the existing suite; the toolkit enforces from commit-1.

### Coinage discipline

Author-introduced cognitive scaffolding terms (terms invented for organizational convenience rather than as methodology load-bearing concepts) do not enter the methodology spec. Adding a term requires recurrence evidence OR operator-directive. The methodology favors descriptive prose over named-mechanism shorthand.

Example application: during pre-design, author-introduced terms like "Tier A/B/C classification", "Pillar N organizing structure", "Mechanism A/B/C breakdowns" are author-cognitive-scaffolding; not methodology terms. Replaced with descriptive prose in canonical docs.

### Artifact-class governance

Adding a 15th artifact class requires methodology amendment citing 2+ documented drift recurrences for the new class (earned-by-recurrence) OR explicit operator-directive. Schema for the artifact-class registry: `status: candidate | accepted | deprecated`. Candidate classes can have schemas authored speculatively; promotion requires recurrence-evidence.

### Event-variant governance

Adding a 19th event variant requires same. **Consolidation before creation:** generic variants (`ArtifactScaffolded` covers multiple auto-scaffolding outputs; `OperatorDirectiveApplied` covers operator policy decisions) preferred over per-outcome variants.

### Hook governance

Adding a hook requires earned-by-recurrence record. **Consolidation before creation:** the `check-naming-discipline.py` hook covers letter-labels + suite-internal-terminology + vocabulary-registry conformance in a single hook with multi-rule dispatch rather than three separate hooks. Reduces hook chain wall-clock; maintains rule clarity via in-hook check-id reporting.

### Sycophancy compensation for the authoring of methodology amendments

When the operator (or operator + collaborator) authors methodology amendments, sycophancy-compensation discipline applies — the author of the amendment is biased toward justifying it. Per primer 3 + the Exacting Mentor stance: amendment commits include `sycophancy_compensation: <text>` declaring what the author resisted (e.g., "resisted introducing 'Tier A/B/C' as a methodology-spec category; using descriptive prose instead").

---

## Acceptance criteria for deliverables

Each rebuild deliverable carries explicit "done means X" criteria.

### `vsdd init` (subcommand)

- Fresh-environment install verification: `cargo install vsdd && vsdd init` in a clean repo produces all expected files in expected locations
- `vsdd init --check` pre-flight validates substrate prerequisites before deployment (shift-left)
- Interactive prompts: per-feature axes + auth method
- Runs `pre-commit install` automatically (shift-left)
- Creates `.vsdd/` directory with `events.jsonl` (empty) + `config.yaml` (axes + auth_method declared) + `otel-collector.yaml` (default config) + `mcp-cache/` (pre-warmed) + `registry/` (vocabulary + canonical-patterns + anonymization-patterns)
- Deploys `DESIGN.md.template` (shift-left)
- Deploys `.github/PULL_REQUEST_TEMPLATE.md` + `.github/CODEOWNERS` + CI workflow templates
- Registers methodology + substrate-docs MCP server in `.claude/mcp.json`
- Deploys 10 phase-primer skills + 16 per-domain skills + 2 meta-skills as `.claude/commands/vsdd-*.md`
- Deploys ~19 methodology hooks in `.claude/hooks/`
- Sets `CLAUDE_CODE_ENABLE_TELEMETRY=1` + OTLP exporter env vars in `.vsdd/env-vars`
- Emits `ProjectInitialized` event with full deployment manifest (auth_method + axes_declared + vsdd_suite_version + deployed_artifacts as event fields; consolidates the previously-separate ProjectAxesDeclared + AuthMethodDeclared events into one project-init event)
- `manual-tests/vsdd-init.md` documents the run-through

### `vsdd observe <subcommand>`

- `vsdd observe cycle|layer|project` queries event log + produces expected output shapes
- `vsdd observe metrics --auto` aggregates OTel + custom events + SDK message-stream-derived metrics
- `vsdd observe pr-body --layer <N>` auto-generates PR description with manual-test checklist embedded from `manual-tests/layer-N.md`
- Handles missing event log gracefully (clean error message; no panic)
- `manual-tests/vsdd-observe.md` documents run-throughs

### `vsdd mcp-serve` (top-level subcommand)

- Responds to MCP protocol on stdio; serves 4 tools per the MCP server tool surface section
- Cache layer at `.vsdd/mcp-cache/` (gitignored; TTL-bounded; pre-warmed at `vsdd init`)
- `manual-tests/vsdd-mcp-serve.md` documents per-tool acceptance criteria

### `vsdd verify <subcommand>`

- `vsdd verify check` runs each hook + emits `HookFired` events; correct exit codes (0 for pass; non-zero for violation); emits `ValidationPassed` / `ValidationFailed` events
- `vsdd verify explain <error-code>` opens extended docs for any VSDD-E####/W####/L#### code (Rust `--explain` pattern)
- `vsdd verify hook <hook-id>` Rust hook-runner mirror invocation matches the Python hook's enforcement logic against the same JSON Schema source
- `vsdd verify test-error-catalog` runs the regression suite against `manual-tests/error-catalog/<code>/{should-fire,should-not-fire}/` fixtures
- `vsdd init --ci-mode` is the CI bootstrap invocation (deploys the same artifacts as operator-local + sets up CI-runtime SARIF output mode)
- `manual-tests/vsdd-verify.md` documents per-hook run-through

### Post-DESIGN.md auto-scaffolding (shift-left)

The `post-design-md-modification` hook fires when DESIGN.md is committed/modified. Auto-scaffolds:
- `manual-tests/layer-N.md` skeleton with checkable items derived from each behavioral contract declared for Layer N (shift-left — closes bookmark-cli-manual SO R1 F1 + DR R1 F3 recurrence)
- Phase 2a Red Gate test stubs (failing-by-default) for each behavioral contract (shift-left — closes QE R8 F1-F3 falsifiability gap pattern)
- Emits `ArtifactScaffolded` event (generic variant covering manual-tests + Red Gate skeleton + future scaffolding outputs; resists per-outcome variant proliferation per the methodology's naming + coinage governance)

Operator fills in test bodies + manual-test expected outcomes; doesn't author skeletons from scratch.

### Methodology hooks (~10)

Each hook needs:
- **Positive test:** hook passes when discipline holds
- **Negative test:** hook fails when discipline violated
- **Bypass test:** bypass marker is honored
- **Error test:** malformed input produces clear error message

DESIGN-VERIFICATION specifies the per-hook test pattern.

### Methodology spec (~250-350 lines)

Per the acceptance criterion in the section list section above:
- Every section in the list has a non-empty body
- Every architectural decision is reflected
- Every event variant declared is defined
- Every domain referenced has a corresponding prompt file
- Every phase has its primer
- Mechanical validation via `check-methodology-semantics.py`

### Phase primers + domain prompts + supplements

Each artifact carries its frontmatter-schema-validated acceptance criteria (see Phase primer + Domain prompt authoring guidelines above + DESIGN-SCHEMA for supplement frontmatter).

### Exit Signal criterion for the toolkit itself

When an adversarial reviewer runs Phase 3 against the `vsdd-cli` repository and produces only hallucinated findings, the Exit Signal is reached. Hallucinated classification requires explicit demonstration of non-applicability per primer 3 discipline.

---

## Implementation order

| Track | Goal-4 surface? |
|---|---|
| 1a — Author DESIGN-SCHEMA.md (foundational; unblocks 1b + 1c) | Foundational |
| 1b — Author DESIGN-OBSERVABILITY.md | Yes |
| 1c — Author DESIGN-VERIFICATION.md | Yes |
| 1d — Re-validate this DESIGN-METHODOLOGY.md against 1a-1c | Coordination |
| 2a — Implement vsdd crate (1 crate, 1 binary with subcommands: init / verify / observe / mcp-serve) | Yes |
| 2b — Implement ~17 Python hooks + Rust mirror (incl. post-DESIGN.md auto-scaffolding hook + PR-discipline hooks + consolidated CHANGELOG-discipline hook + naming-discipline hook + prose-surface TW+DR composition hook) | Yes |
| 2c — Author 10 phase primers | No |
| 2d — Author 18 domain prompts (with vestigial-pattern cuts) | No |
| 2e — Author 14 supplements (with cuts) | No |
| 2f — Author methodology spec | No |
| 2k — Implement error catalog (~25 accepted + ~15 candidate codes per status-tier discipline) + validator falsifiability fixtures + `vsdd verify explain` | Yes (Goal 2 operationalization) |
| 2l — Author DESIGN.md template + vocabulary registry + canonical-patterns registry + anonymization-patterns registry | No (shift-left) |
| 2m — Implement post-DESIGN.md auto-scaffolding hook (manual-tests + Phase 2a Red Gate skeleton) | No (shift-left) |
| 2n — Author 13 artifact-class JSON Schemas (DESIGN-SCHEMA dependency) | Foundational |
| 2g — Author CI workflow templates | Goal-4 specific |
| 2h — Implement methodology + substrate-docs MCP server (full v1 deliverable) | No (but agents leverage MCP across all phases) |
| 2i — Deploy default OTel collector config + sink wiring | Yes (Goal 3 flagship) |
| 2j — Auth-method declaration UX + auth event variants + anonymization hook API-key detection | Cross-cutting |
| 3 — Goal 4 end-to-end demonstration via rebuild's own CI | Goal-4 specific |

Time estimates intentionally absent. Sequencing + scope-discipline are the calibration until the toolkit's own observability subsystem produces cycle telemetry that derives future-cycle estimates from real data.

Upstream coordination activity (filing bugs upstream; pitching absorption candidates) is operator-activity outside the toolkit's scope.

---

## Cross-DESIGN-doc coordination

What this doc produces that sibling DESIGN docs consume:

| Sibling DESIGN doc | Consumes from this doc |
|---|---|
| DESIGN-SCHEMA | Artifact class list (review entry, finding, phase primer, domain prompt, supplement, methodology event variants, `.vsdd/config.yaml`); per-class frontmatter field names; per-domain sycophancy_failure_modes structure; auth_method declaration schema (NO key-material fields); credential-exclusion structural property for all event-variant schemas |
| DESIGN-OBSERVABILITY | 18 methodology event variants list (incl. PhaseCompositionDeclared + AuthMethodDeclared + ProjectInitialized + ArtifactScaffolded + 3 PR-lifecycle variants); per-feature-axes activation as observability event; cluster-batching shape as event metadata; OTel collector config + sink wiring (`.vsdd/otel-collector.yaml` with redaction processor); capture-source provenance discipline (otel-metric / otel-log-event / otel-trace-attribute / vsdd-custom-event / sdk-result-message / usage-api-reconciled-v1+ / unmeasurable); credential-redaction in collector forwarding; MCP server tool-handler design + cache strategy + 4-tool surface |
| DESIGN-VERIFICATION | check-phase-composition.py hook spec; the 4 enforcement mechanisms; per-hook deployment matrix (~19 hooks incl. dependency-approval + methodology-version-drift); CI bootstrap pattern (`vsdd init --ci-mode`); CI workflow templates with GitHub Secrets pattern + SARIF emission for GitHub Code Scanning; check-anonymization.sh API-key detection patterns; Rust hook-runner mirror; per-hook test pattern; post-DESIGN.md auto-scaffolding hook design; pre-commit framework auto-install integration; error catalog implementation (~25 accepted + ~15 candidate codes per status-tier discipline) incl. VSDD-E0100 dependency-approval-missing + VSDD-W0200 methodology-version-drift + VSDD-E0021 auth-method-plan-incompatible-with-ci + VSDD-W0022 ci-workflows-present-without-ci-auth-declared; validator falsifiability fixtures at `manual-tests/error-catalog/<code>/{should-fire,should-not-fire}/`; consolidated hooks (check-naming-discipline 4-rule; check-changelog-discipline 10-rule); bypass-marker enforcement (operator-local rationale + CI PR-approval-label gate) |

What this doc consumes from sibling DESIGN docs (forward-references):

| Sibling DESIGN doc | This doc forward-references |
|---|---|
| DESIGN-SCHEMA | Per-class JSON Schema shapes (this doc names fields; DESIGN-SCHEMA fixes shapes) |
| DESIGN-OBSERVABILITY | Event log sink format + trace assembly + dashboard ladder details + MCP server internal architecture |
| DESIGN-VERIFICATION | Hook implementation details + Rust hook-runner CI mirror + CLI internals + Sanity Check activation hook config |

---

## Open decisions deferred to sibling DESIGN docs

| Decision | Routing |
|---|---|
| Per-class JSON Schema field shapes | DESIGN-SCHEMA |
| Event log sink format (JSONL vs SQLite vs hybrid) | DESIGN-OBSERVABILITY |
| Trace assembly mechanism (query-time vs materialize-on-emit) | DESIGN-OBSERVABILITY |
| Metrics rollup mechanism | DESIGN-OBSERVABILITY |
| SLO config schema | DESIGN-OBSERVABILITY + DESIGN-SCHEMA |
| Cargo workspace structure + crate internals | DESIGN-VERIFICATION + DESIGN-OBSERVABILITY |
| Hook deployment mechanism (settings.json extension shape) | DESIGN-VERIFICATION |
| Hook composition with crosslink's 5 enforcement hooks | DESIGN-VERIFICATION |
| Frontmatter schema per artifact class | DESIGN-SCHEMA |
| Anchor-ID deterministic generation rule | DESIGN-SCHEMA |
| Bypass-marker deprecation timeline | DESIGN-VERIFICATION |
| Sanity Check activation hook config | DESIGN-VERIFICATION |
| 9-phase transition provability matrix per-hook design | DESIGN-VERIFICATION |
| Anthropic Usage and Cost API integration (v1+ scope) | DESIGN-OBSERVABILITY |
| OTel collector process lifecycle + resource limits + log rotation | DESIGN-OBSERVABILITY |
| MCP server cache TTL + refresh strategy | DESIGN-OBSERVABILITY |
| Phase 5 surface tooling refresh discipline (tool lists rot) | DESIGN-METHODOLOGY (future iteration) |
| Per-domain prompt rewrite per cross-domain vestigial-pattern cuts | DESIGN-METHODOLOGY (during 4d implementation) |
| Sycophancy-compensation per-review preamble vs end-of-review self-audit | DESIGN-METHODOLOGY (future iteration) |
| Multi-machine operator identity continuity (per-operator attribution across laptop / desktop / work / personal — SSH key fingerprints + git user.email may differ per machine) | Deferred to v1+ pending earned-by-recurrence evidence (Phase 5 round 1 Security F5; single-operator-single-machine projects do not hit this case in v1 evaluation cycles) |

---

## Closing

This DESIGN doc operationalizes the methodology subsystem. The methodology spec section list + phase taxonomy + composition matrix + Exacting Mentor stance + per-feature axes + cluster shape + memory isolation + cold-session budget + MCP server tool surface + auth method declaration + Security disciplines + acceptance criteria provide the working artifact next-phase authoring (DESIGN-SCHEMA, DESIGN-OBSERVABILITY, DESIGN-VERIFICATION, then the toolkit's implementation) consume.

**Next:** Author DESIGN-SCHEMA.md to fix the artifact-class shapes this doc names. Then DESIGN-OBSERVABILITY + DESIGN-VERIFICATION in parallel. Then this doc re-validates against the trio at the cross-DESIGN-doc closure boundary.
