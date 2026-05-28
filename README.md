# vsdd

A Rust toolkit that interprets + implements [Verified Spec-Driven Development (VSDD)](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) — a software methodology authored by **[@dollspace.gay](https://bsky.app/profile/dollspace.gay)** ([GitHub](https://github.com/dollspace-gay)) and operationalized in **[crosslink](https://github.com/forecast-bio/crosslink)** (also by dollspace).

**Credits + sources:**
- [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) — the methodology this toolkit implements
- [VDD whitepaper](https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25) — predecessor; introduces the hyper-critical-adversary discipline
- [crosslink](https://github.com/forecast-bio/crosslink) — the canonical operational substrate this toolkit composes against
- [@dollspace.gay](https://bsky.app/profile/dollspace.gay) on Bsky + [dollspace-gay](https://github.com/dollspace-gay) on GitHub — methodology + substrate author

**This is not my methodology.** VSDD is dollspace's. This repo is one collaborator's *interpretation and implementation* of that methodology as a Rust toolkit. The toolkit:
- adds an observability subsystem (OTel + FinOps surfaces) on top of the methodology
- adds a verification subsystem (~18 methodology hooks + 13 schema-validated artifact classes + Rust-like error catalog)
- ships standalone via `cargo install vsdd`
- is designed for clean absorption back into crosslink at the upstream maintainer's discretion

Repo: [`vsdd-cli`](https://github.com/magnificentlycursed/vsdd-cli) (Rust CLI implementation). Published crate: [`vsdd`](https://crates.io/crates/vsdd). Binary installed: `vsdd`.

This document is the positioning. Per-subsystem design docs ([`DESIGN-METHODOLOGY.md`](./DESIGN-METHODOLOGY.md), [`DESIGN-OBSERVABILITY.md`](./DESIGN-OBSERVABILITY.md), [`DESIGN-VERIFICATION.md`](./DESIGN-VERIFICATION.md), [`DESIGN-SCHEMA.md`](./DESIGN-SCHEMA.md)) fill in the architecture.

---

## Project identity

The `vsdd` toolkit is an **independent implementation** of [@dollspace.gay](https://bsky.app/profile/dollspace.gay)'s VSDD methodology — one collaborator's interpretation and operationalization of the methodology as a Rust CLI + observability subsystem + verification subsystem. It extends and enhances both the methodology and the [crosslink](https://github.com/forecast-bio/crosslink) tool that operationalizes it.

The relationship to the methodology + upstream:

- **Methodology authorship is dollspace's.** This toolkit implements + interprets the methodology; it does not author it. The canonical methodology lives in the [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) + its predecessor [VDD whitepaper](https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25) + the [crosslink](https://github.com/forecast-bio/crosslink) repo.
- **Designed for clean absorption** if upstream wants it. Patterns ship in shapes that minimize integration friction.
- **Stands on its own merits.** The observability + verification surfaces deliver value as a standalone toolkit independent of any absorption decision.
- **Absorption is upstream's discretion.** dollspace decides whether to absorb each pattern. The toolkit proposes when each pattern is mature enough to demonstrate; the toolkit doesn't pre-suppose absorption.

---

## Audience + role

**Working artifact upstream of Phase 1.** This document is the input to Phase 1 design authoring + Phase 1c decomposition. Its readers:

1. **The operator authoring the DESIGN docs** (future-me or a collaborator). The document is design-rationale for the work that follows. Bias toward concrete decisions, named constraints, deferred decisions surfaced.
2. **AI agents composed against Phase 1 primers + relevant domains** when authoring DESIGN docs from this positioning. The document is machine-readable as well as human-readable; greppable headers, schema-shaped frontmatter when added, navigable cross-references.

Both readers want clear positioning, named trade-offs, deferred decisions surfaced. The doc is a working artifact — bold, anti-slop — not polished for an external evaluator audience.

### Conventions used in this doc

**Domain abbreviations** (expanded on first use; reference table here for the shorthand used throughout): SO = Solution Owner, SA = Solution Architect, SE = Software Engineer, QE = Quality Engineer, TW = Technical Writer, DR = Documentation Reviewer, UX = User Experience, PE = Platform Engineer, AIE = AI Engineer, Security, Red Team. See the [Domain set](#domain-set) section for the full 18-domain list + activation criteria.

**Substrate abbreviations:** OTel = OpenTelemetry; OTLP = OpenTelemetry Protocol; MCP = Model Context Protocol; SDK = Software Development Kit; SARIF = Static Analysis Results Interchange Format (machine-readable CI output); LSP = Language Server Protocol (IDE integration).

**Evidence references:** `R## F##` (e.g., `R78 F4`, `R91 F1`) refers to the **existing-suite** Review N Finding M — historical methodology-evolution evidence from [`guild-projects/guild-portfolio/vsdd-suite/suite-development/review-log/`](https://github.com/magnificentlycursed/guild-portfolio/tree/main/vsdd-suite/suite-development/review-log). `G-###` (e.g., `G-156`) refers to existing-suite governing findings at [`vsdd-suite/suite-development/FINDINGS-INDEX.md`](https://github.com/magnificentlycursed/guild-portfolio/blob/main/vsdd-suite/suite-development/FINDINGS-INDEX.md). `PR #N` refers to existing-suite pull requests. Bookmark-cli-manual references like `TW R1 F2` are findings from the existing reference-example project's per-domain review logs.

**Error codes:** `VSDD-E####` blocks commit/merge; `VSDD-W####` warns but allows; `VSDD-L####` informational lint. See [Document artifact validation + error surface](#document-artifact-validation--error-surface) for the catalog.

---

## Four governing design goals

These four goals are the toolkit's architectural constraints. Every subsystem must serve all four; every artifact must demonstrably advance at least one.

### Goal 1 — Absorbability-ready patterns

Every enhancement the suite ships is designed in shapes that absorb cleanly into crosslink IF upstream chooses. Absorbability is a quality property of the design — not a strategic dependency.

What this means in practice:

- Schemas + event types + skill manifests + hook contracts are crosslink-compatible at the boundary
- Pattern documentation includes "absorption-target" + "suite-side fallback" sections
- The rebuild prototypes patterns standalone; absorption proposals follow when each is mature enough to demonstrate
- No pattern blocks on upstream decisions

### Goal 2 — Auditable + machine-enforceable + dual-audience

Every artifact serves two audiences (**humans + agents**) with three structural properties (**auditable + machine-enforceable + dual-audience-readable**):

- **Auditable:** every action emits structured audit-trail evidence. Two cooperating layers — suite-side event log (always present) + crosslink-side audit trail (when crosslink is in use, via compatible event schema).
- **Machine-enforceable:** every methodology rule has a hook OR a schema validator OR a crosslink workflow check. Not honor-system; not authoring discipline alone.
- **Dual-audience-readable:** narrative prose for humans + structured frontmatter / labels / events for agents. Same artifact, two surfaces.

Human role-flavor (developer vs. user) is captured by the Solution Owner ↔ Documentation Reviewer adversarial-pair pattern, not by a separate audience.

### Goal 3 — Observability + Monitoring + Alerting + Traces + Dashboarding as first-class

Observability Engineering + FinOps as design dimension. Every artifact is **born observable**. Every action emits. Default-on, not opt-in.

The rebuild composes against the [Claude Agent SDK's built-in OpenTelemetry export](https://code.claude.com/docs/en/agent-sdk/observability) as the observability primitive — the CLI emits metrics (tokens, cost, sessions, tool decisions), log events (prompts, API requests, errors, tool results), and traces (interactions, model requests, tool calls, hooks) to any OTLP-compatible backend. Methodology-specific event variants augment the SDK's signals.

### Goal 4 — Shift VSDD left into CI/CD pipelines

The methodology runs in CI/CD pipelines, not only operator-local terminals. Verification checks shift left to PR-time mechanical enforcement: phase-transition provability, phase-composition declaration, schema validation, cost-tally bounds, finding-classification universe, drift + staleness sweeps, anonymization + identity-correlation enforcement.

Methodology violations caught in CI/CD before merge cost ~minutes of CI runtime; the same violations caught in Phase 3 cost hours of cold-session adversarial review. Pre-merge mechanical enforcement frees reviewer time for findings that require judgment.

---

## Non-goals

The rebuild explicitly does NOT:

- **Maintain backwards compatibility with the existing suite.** No artifact migrates by copy. The existing suite continues to exist as a sibling project; the toolkit starts fresh.
- **Ship orchestration tooling.** Crosslink owns dispatch (`swarm review`, `swarm gate`, `swarm fix`), session lifecycle, milestones-as-layers, issues-as-findings, knowledge management. Claude Code owns in-session experience (skills, subagents, hooks). The suite composes against both; does not compete.
- **Support a manual operational mode at first-class parity.** Crosslink-primary mode only.
- **Build a parallel observability system to crosslink's.** Crosslink has events + tui + serve + token_usage + context measure + intervene + heartbeats — the bones. The suite contributes the FinOps + Observability-Engineering surface on top via crosslink-compatible event schema. Absorption is upstream's call.
- **Contribute to claude-code upstream.** The rebuild is not absorbable into claude-code; will not file upstream there.
- **Integrate directly with the Anthropic Messages API.** v1 ships against the Claude Code CLI (via the Agent SDK as programmatic wrapper). Direct Messages API integration is out of scope for v1; architecture extensible to v1+ if operator-time permits.
- **Treat the existing suite as a predecessor to migrate from.** Sibling, not successor.
- **Ship a curated reference example.** The rebuild dogfoods on itself; the existing `bookmark-cli-manual` (in the existing suite) serves as historical reference for the methodology applied to a non-methodology project.

---

## Center of gravity

The rebuild's product is:

1. **The methodology spec** — concise governing prose (~250-350 lines) at `methodology.md` (project root for vsdd-using-projects; `vsdd-cli` repo root for the toolkit's own spec) that captures the load-bearing disciplines.
2. **The observability subsystem** — flagship. OTel collector + sink wiring + 18 methodology-specific event variants + `vsdd observe` subcommand + FinOps-applied-to-IAR dashboards. Standalone-valuable; crosslink-compatible event schema; designed for absorption.
3. **The verification subsystem** — ~19 methodology hooks composing with crosslink's 5 enforcement hooks (~24 total in a VSDD project).
4. **The schema enforcement layer** — YAML frontmatter + per-artifact-class JSON Schema with semantic versioning.
5. **The domain prompt set** — 16 role-domain prompts + 2 meta-domain prompts (VSDD Methodology + Sanity Check).
6. **The phase primers** — 10 phase primers per whitepaper-canonical taxonomy.
7. **The language and interface supplements** — 14 supplements with vestigial-pattern cuts applied.
8. **The rebuild's own development as canonical dogfood.** The rebuild applies its own methodology to itself; the repo is the worked example. No curated `-v2` reference example is committed.
9. **The adoption companion** — `vsdd init` subcommand that composes with `crosslink init` to deploy suite-specific skills, hooks, knowledge pages, schemas, MCP server, and OTel collector config.

---

## Adoption + distribution

A project adopts the suite via:

```sh
cargo install vsdd              # one-time: install the toolkit
cd <project-root>
crosslink init                  # crosslink's own setup
vsdd init                       # deploys toolkit assets
```

**Init order is required, not advisory.** `crosslink init` deploys substrate (hooks, MCP servers, rules) that `vsdd init` composes against. Running `vsdd init` first fires `VSDD-E0220: existing-file-malformed-refuse-to-overwrite` on the missing crosslink artifacts during the pre-flight check (`vsdd init --check` detects crosslink-init-manifest absent and refuses deployment). The reverse order is not order-independent by construction; if attempted, `vsdd init` halts with explicit operator-facing error directing the operator to run `crosslink init` first.

**Platform requirement: v1 is GitHub-only.** The methodology's CI-side teeth — bypass-approval label gate, CODEOWNERS auto-routing, SARIF emission, CHANGELOG cooperation, dependency-approval PR-description structure — are GitHub-API-specific. `vsdd init --check` detects non-GitHub remotes and refuses deployment. No commitment to support GitLab / Bitbucket / Forgejo / Codeberg / self-hosted Gitea / sourcehut in v1 or v1+; revisit only with adoption evidence + operator-directive.

`vsdd init` (subcommand of the single `vsdd` Rust binary distributed via `cargo install vsdd`). The shift-left discipline: every defect class preventable at adoption-time gets caught at adoption-time, not at first commit or first cycle.

**Pre-flight validation (`vsdd init --check`):** runs before deployment — validates git repo present + claude-code installed (substrate version pinned) + crosslink installed (if axis declared) + cargo toolchain + Python version. Reports OK/missing before any artifacts deploy. Prevents mid-init inconsistent state.

**Deployment steps:**

1. Deploys 10 phase-primer skills + 16 per-domain skills + VSDD Methodology + Sanity Check meta-skills as `.claude/commands/vsdd-*.md` files (VSDD-prefix discipline so they cluster in `/help` and don't collide with crosslink's 14 skills). Per-domain skills (`vsdd-domain-<slug>`) are operator-interactive entry points; phase-primer skills compose against per-domain skills per the phase-domain composition matrix
2. Deploys ~19 methodology hooks (Python — matches Claude Code's hook convention + crosslink's existing 5 hooks) alongside crosslink's 5 (extends `.claude/settings.json`; composes, doesn't replace) + runs `pre-commit install` automatically so the first commit lands under hook enforcement (closes first-commit-without-hook-enforcement defect class)
3. Registers the 16 role-domain prompts + VSDD Methodology + Sanity Check meta as `vsdd-domain` knowledge pages via `crosslink knowledge import`
4. Registers all 14 supplements as `vsdd-supplement` knowledge pages
5. **Interactive per-feature axes prompt** — asks operator to confirm each axis (`ships-to-users-other-than-developer?`, `network-exposed?`, etc.); writes `.vsdd/config.yaml` with declared axes; emits `ProjectAxesDeclared` event (closes axes-undeclared-drift defect class)
6. **Interactive auth method prompt** — Plan vs API key; writes `auth_method` + `auth_method_credential_source` to `.vsdd/config.yaml` (NO key value); emits `AuthMethodDeclared` event
7. Creates `.vsdd/events.jsonl` for the suite-side audit-trail sink
8. Deploys default OTel collector configuration (`.vsdd/otel-collector.yaml`) — local collector with `.vsdd/events.jsonl` + crosslink-hub sinks default-on; external-backend endpoints declared as commented examples; sets `CLAUDE_CODE_ENABLE_TELEMETRY=1` + OTLP exporter env vars
9. Deploys `.claude/mcp.json` registering the methodology + substrate-docs MCP server (`vsdd mcp-serve` invocation) — single server exposes 4 tools: methodology lookup, Claude Code docs search, crosslink docs search, Anthropic API docs search (stub in v1); pre-warms cache with substrate doc snapshots
10. Deploys `DESIGN.md.template` (operator authors Phase 1a from a schema-validated structural template — closes DESIGN-doc structural drift class)
11. Deploys `.vsdd/registry/vocabulary.yaml` (canonical methodology terms registry) + `.vsdd/registry/canonical-patterns.yaml` + `.vsdd/registry/anonymization-patterns.yaml` (per-project-extensible credential-detection patterns)
12. Deploys `.github/PULL_REQUEST_TEMPLATE.md` (PR template artifact class) + `.github/CODEOWNERS` (auto-routes TW + DR co-authorship for prose surfaces) + `.github/ISSUE_TEMPLATE/*.md`
13. Deploys CI workflow templates to `.github/workflows/` — schema-validated against CI workflow template meta-schema; `vsdd verify` runs as PR merge gate; auth-method-conditional steps for API-key-using CI
14. Emits `ProjectInitialized` event with full deployment manifest (vsdd toolkit version, list of artifacts deployed, axes declared, auth method, hooks installed, MCP server registered) — the project's audit trail begins at init, not at first commit

Single command; operator's adoption cost is the interactive prompts (axes + auth) — typically ~2 minutes.

**Phase 1a/1b-time auto-scaffolding (post-DESIGN.md commit hook):** when DESIGN.md commits to a Layer N's behavioral contracts, the `post-design-md-modification` hook auto-scaffolds:
- `manual-tests/layer-N.md` skeleton with checkable items derived from each behavioral contract — closes the bookmark-cli-manual SO R1 F1 + DR R1 F3 promised-artifact-missing recurrence
- Phase 2a Red Gate test stubs (failing-by-default) for each behavioral contract — closes the QE R8 F1-F3 falsifiability gap pattern (every contract starts with at least one test)
- Emits `ArtifactScaffolded` event with layer + contract count

Operator fills in test bodies + manual-test expected outcomes; doesn't author skeletons from scratch.

### Collision handling with existing project content

vsdd init plays nicely with existing projects. Patterns inherited from crosslink's own `init` collision-handling discipline: managed-section markers + JSON object merge + side-by-side templates + refuse-malformed-file. Operator content is never clobbered.

**Files vsdd composes with (merges or appends managed sections):**

| File | Pattern |
|---|---|
| `.gitignore` | Managed-section markers (`# === vsdd managed ===` / `# === End vsdd managed ===`); idempotent in-place replacement |
| `.claude/mcp.json` | JSON object merge — adds `vsdd` entry to `mcpServers`; warns on key collision; preserves operator entries |
| `.claude/settings.json` | UNION-merge `allowedTools` (operator + crosslink + vsdd); preserves existing hooks; adds vsdd-* hook entries alongside crosslink's |
| `.github/CODEOWNERS` | Managed-section markers; TW + DR routing rules appended; operator rules preserved |
| `.pre-commit-config.yaml` | Managed-section markers OR YAML object merge; vsdd-* hooks added to repo list |

**Files vsdd never touches (operator-owned):**

`DESIGN.md` · `README.md` · `CHANGELOG.md` (managed by crosslink's `close` command per [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) convention; see [CHANGELOG discipline](#changelog-discipline) below) · `PROCESS.md` · `Cargo.toml` / `package.json` / `pyproject.toml` · `src/` / `tests/` / `lib/` · existing `manual-tests/*.md` · operator-authored `.github/workflows/*.yml`

**Files deployed alongside (side-by-side templates when can't safely merge):**

| If exists | vsdd deploys |
|---|---|
| `DESIGN.md` | `DESIGN.md.vsdd-template` (operator reviews + adopts manually) |
| `.github/PULL_REQUEST_TEMPLATE.md` | `.github/PULL_REQUEST_TEMPLATE/vsdd-layer-pr.md` (GitHub multi-template directory; selectable from dropdown) |

**Files vsdd owns entirely (vsdd-prefixed; no collision possible):**

`.claude/hooks/vsdd-*.py` · `.claude/commands/vsdd-*.md` · `.claude/agents/vsdd-*.md` · `.github/workflows/vsdd-*.yml` · `.github/ISSUE_TEMPLATE/vsdd-*.md` · `.vsdd/` directory tree

**Refusal disciplines (matches crosslink):** if `.mcp.json` / `.claude/settings.json` / `.pre-commit-config.yaml` / `.vsdd/init-manifest.json` is malformed, vsdd-init bails with `error[VSDD-E0220]: existing-file-malformed-refuse-to-overwrite` — operator fixes or removes + retries. Never destroys operator state.

**Idempotent re-init:** `.vsdd/init-manifest.json` tracks per-file SHA-256 hashes of vsdd-deployed files. Re-running `vsdd init` after a toolkit upgrade (`cargo install vsdd --force`) replaces managed sections + merges new entries; operator-edits outside managed sections preserved; operator-edits inside managed sections detected via manifest-SHA mismatch + explicit resolution flag (`--keep-operator-edits` or `--accept-managed-defaults`).

**Pre-flight detection:** `vsdd init --check` reports the deployment plan before any writes — which files would be CREATED, MERGED, SKIPPED-WITH-TEMPLATE, or SKIPPED-OPERATOR-OWNED. Operator confirms or aborts.

**Operator-action queue:** files needing operator-merging (the `*.vsdd-template` side-by-side files) listed in vsdd-init's exit output + the `ProjectInitialized` event manifest.

### Binary surface

**Principle: any CLI the suite ships is Rust.** Single `vsdd` crate with a single `vsdd` binary that dispatches subcommands (matching cargo / rustup / git ecosystem convention). Library modules shared internally; single version number; one release artifact.

| Component | Language | Why |
|---|---|---|
| `vsdd` Rust binary (subcommands: `init`, `verify <check\|explain\|test-error-catalog\|hook>`, `observe <cycle\|layer\|project\|metrics\|pr-body>`, `mcp-serve`) | Rust | Matches crosslink's toolchain; `cargo install vsdd` distribution; type safety; single-binary CI bootstrap; matches cargo / git / rustup ecosystem convention |
| Deployed `.claude/hooks/*.py` | Python | Claude Code's hook convention; matches crosslink's existing 5 hooks; substrate match wins for the deployed-into-projects layer |
| Deployed `.claude/commands/vsdd-*.md` | Markdown | Claude Code convention |
| Schemas (`schemas/*.json`) | JSON | Cross-language; generated from Rust types via [`schemars`](https://github.com/GREsau/schemars) or equivalent |
| Methodology spec, primers, domain prompts, supplements | Markdown + YAML frontmatter | Cross-language |
| `.vsdd/otel-collector.yaml` | YAML | OpenTelemetry collector standard |

The hook architecture: pure-Python hooks operator-side; the same enforcement logic exists as Rust binary (`vsdd verify hook <hook-id>`) for CI execution. Two surfaces; one JSON Schema source.

**Subcommand surface:**

```
vsdd init                              # adoption companion (operator-local OR --ci-mode for CI bootstrap)
vsdd verify check                      # run hooks + schema validation; emit errors/warnings/lints
vsdd verify explain <error-code>       # extended documentation for any VSDD-E####/W####/L#### code
vsdd verify test-error-catalog         # regression suite for the error catalog
vsdd verify hook <hook-id>             # Rust hook-runner mirror (CI-side)
vsdd observe cycle|layer|project       # tabular reports
vsdd observe metrics                   # event-log + OTel signal aggregation
vsdd observe pr-body --layer <N>       # auto-generate PR description with manual-test checklist
vsdd mcp-serve                         # MCP server (long-running stdio loop)
```

### Claude Code substrate features leveraged

The rebuild explicitly leverages these features rather than treating the substrate generically:

| Feature | Suite usage |
|---|---|
| **[Claude Agent SDK](https://code.claude.com/docs/en/agent-sdk/observability)** | Primary integration substrate; runs claude-code CLI as subprocess; emits OTel telemetry + SDK message stream cost data |
| **OpenTelemetry export** (`CLAUDE_CODE_ENABLE_TELEMETRY=1` + exporter env vars) | Metrics (tokens, cost, sessions, tool decisions); log events (prompts, API requests, errors); traces (interactions, llm_requests, tools, hooks) → vsdd-deployed OTel collector → `.vsdd/events.jsonl` + crosslink hub |
| **SDK message stream cost data** (`message.usage`, `modelUsage`, `total_cost_usd`) | Per-step + per-model + cumulative SDK estimate; consumed by `vsdd observe` for in-cycle reports (caveat: client-side estimate, not authoritative) |
| **W3C trace context propagation** | SDK auto-injects TRACEPARENT into CLI subprocess + Bash/PowerShell tool calls; full delegation chain visible in single trace |
| `.claude/hooks/*.py` | ~19 methodology hooks deployed by `vsdd init` |
| `.claude/commands/*.md` | 10 phase-primer + 16 per-domain + 2 meta skills |
| `.claude/agents/*.md` | Per-domain cold-session reviewer agents pre-configured |
| `.claude/mcp.json` | Methodology + substrate-docs MCP server (`vsdd mcp-serve`) |
| Cron triggers (`CronCreate`) | Scheduled drift sweeps + cycle-close reminders + methodology-staleness checks |
| Notifications (`PushNotification`, `RemoteTrigger`) | Operator alerts: budget breach, rate-limit headroom, scheduled-cycle reminders |
| LSP integration | Real-time frontmatter validation during methodology + project authoring |
| Background tasks (`Bash run_in_background`) | CI-side compositions; long-running aggregations; cold-session dispatch primitives |
| Plan mode | Substantive methodology-spec change discipline |
| Permission modes | Hook-bypass-marker enforcement at PR-time |
| Plan auth (Max/Pro) | Default operator-local auth; Agent SDK credits separate from interactive limits; 1-hour prompt-cache TTL auto-enabled |
| API key auth (`ANTHROPIC_API_KEY` env var) | CI/automation auth per Anthropic's guidance; required for shared-org-key + per-operator-key patterns; pay-as-you-go billing; opt-in 1-hour TTL via `ENABLE_PROMPT_CACHING_1H=1` |

---

## Auth method + Security disciplines

Auth method declared explicitly in `.vsdd/config.yaml` per project; **separate fields for operator-local and CI contexts** (per Phase 5 round 1 Security F4 — Plan auth structurally permits CI declaration but operationally fails at runtime; cross-field validation now rejects). Operator declares at `vsdd init` time + may change per `AuthMethodChanged` event (rotation, scale-shift, plan-credit-exhaustion fallback).

```yaml
auth_method:
  operator_local: plan | api_key
  operator_local_credential_source: "plan-auth-no-key" | "env:<VAR_NAME>"
  ci: api_key | "none"        # "none" allowed when no CI workflows deployed; "plan" rejected by schema validator
  ci_credential_source: "env:<VAR_NAME>" | null
```

| Auth method | Context | Cost model |
|---|---|---|
| **Plan (Max/Pro) + Agent SDK** | Operator-local Phase 1a-2c skill mode + small Phase 3 cycles (operator-interactive session required — NOT CI) | Monthly Agent SDK credits ($20-$200 by tier); separate from interactive limits; 1-hour prompt-cache TTL auto-enabled |
| **API key + Agent SDK** | Operator-local OR Goal 4 CI/CD + Phase 5 hardening tool runs + scheduled cron sweeps | Pay-as-you-go per-token; predictable for automation per Anthropic's own guidance; 1-hour cache TTL opt-in |

**Security disciplines:**

- **Credential storage:** `.vsdd/config.yaml` carries auth-method-name + credential-source-reference only; NEVER credential value. Schema validator rejects credential-shaped fields
- **Anonymization hook:** detects API-key formats (`sk-ant-api03-...`, generic Bearer headers, env-var-assignment-with-credential-shaped-value)
- **Event-variant credential exclusion:** all 18 methodology event variants exclude credential-shaped fields structurally; `OTEL_LOG_RAW_API_BODIES` stays default-off; OTel collector config redacts credential-shaped values before forwarding to external backends
- **Audit trail:** AuthMethodDeclared / AuthMethodChanged / AuthFailureObserved event variants provide forensic record
- **CI integration:** GitHub Secrets pattern (or equivalent); key rotation procedure documented (monthly cadence recommended; ad-hoc on compromise)
- **Per-operator vs shared-organizational keys:** per-operator default (clear attribution); shared-organizational extension activated by `auth_attribution_pattern: shared-organizational` in config
- **Compromised credential procedure:** revoke → audit event log → emit AuthFailureObserved → reissue → anonymization regression-check (lives in operational runbook)
- **Backend bearer tokens** (Honeycomb, Datadog, Grafana, Langfuse, etc.): same disciplines apply — env-var-only storage; anonymization hook detection; no backend tokens in committed config

---

## Two-audience principle

Every artifact serves humans and agents simultaneously:

| Audience | Consumes via | Enforcement mechanism |
|---|---|---|
| **Humans** (operators, contributors, future-me) | Linear prose, markdown anchors, navigable cross-references | Authoring discipline; cold-reader pair validation (Documentation Reviewer ↔ Technical Writer); SO scope review |
| **Agents** (parallel cold-context reviewers + main-session orchestrators) | Greppable frontmatter, schema-validated fields, event log, crosslink typed labels | Hooks (mechanical); JSON Schema validation; observability event schema |

Role-flavor distinction (developer-extending-methodology vs. user-following-methodology) is captured by the Solution Owner ↔ Documentation Reviewer adversarial-pair pattern.

---

## Observability subsystem

Detailed design in [`DESIGN-OBSERVABILITY.md`](./DESIGN-OBSERVABILITY.md). Outline below.

### Agent SDK + OpenTelemetry as primitive

The Claude Agent SDK emits three independent OTel signals (metrics, log events, traces) to any OTLP-compatible backend (Honeycomb, Datadog, Grafana, Langfuse, self-hosted collector) when telemetry is enabled. The rebuild composes against this primitive — methodology-specific event variants augment the SDK's signals rather than reinventing capture.

`vsdd init` deploys a default OTel collector configuration (`.vsdd/otel-collector.yaml`). Default sinks: `.vsdd/events.jsonl` (suite-side audit trail) + crosslink hub (when crosslink in use). External-backend endpoints declared as commented examples; operator-extensible via single-config-edit.

### Observability signal surfaces (logs / metrics / traces)

| Signal | Source | Sink (v1) | Sink (absorption path) |
|---|---|---|---|
| **Logs (events)** | Agent SDK OTel log events + 18 methodology-specific event variants | `.vsdd/events.jsonl` per project | Pitched as new event variants in crosslink's `events.rs`; emits compatible records into crosslink hub |
| **Metrics** | Agent SDK OTel metrics + derived from event log at query time | `vsdd observe` subcommand | Pitched as `crosslink metrics` command extension |
| **Traces** | Agent SDK OTel traces (`claude_code.interaction`, `.llm_request`, `.tool`, `.hook` spans) + finding-lifecycle as span tree (raised → classified → routed → resolved → validated) | Query-time derivation from event log | Pitched as `crosslink trace` command extension |

### Methodology-specific event variants (18)

```
PhaseEntered             PhaseExited            PhaseTransitionAttested
FindingRaised            FindingClassified      FindingRouted
ExitSignaled             SycophancySelfAudit    OperatorDirectiveApplied
VerificationMiniCycleSpawned                    PhaseCompositionDeclared
ProtectiveDisciplineEnforced                    AuthMethodDeclared
ProjectInitialized       ArtifactScaffolded
DraftPROpened            PRReadyForReview       PRMerged
```

`ProjectInitialized` carries fields: `vsdd_suite_version`, `axes_declared`, `auth_method`, `deployed_artifacts_manifest`. `ArtifactScaffolded` is a generic event covering manual-tests + Phase 2a Red Gate skeleton + other auto-scaffolding outputs.

Each carries the same `EventEnvelope` shape crosslink uses (`agent_id`, `agent_seq`, `timestamp`, `signed_by`, `signature`). Total ordering preserved. Crosslink-compatible by construction. All event-variant schemas exclude credential-shaped fields structurally.

Schema-validation events (`ValidationPassed` / `ValidationFailed` / `HookFired`) augment via the document artifact validation surface (described below). Auth-method change events (`AuthMethodChanged`, `AuthFailureObserved`) are reserved for v1+ adoption pending operator-rotation or rate-limit recurrence evidence (Agent SDK OTel signals cover the failure cases natively in v1).

### Capture-source provenance

Every cost-relevant event carries `capture_source`:

```
otel-metric              — emitted by Agent SDK as OTel metric
otel-log-event           — emitted by Agent SDK as OTel log event
otel-trace-attribute     — emitted by Agent SDK as span attribute
vsdd-custom-event        — methodology-specific event
sdk-result-message       — SDK's per-query total_cost_usd + modelUsage + cumulative usage
usage-api-reconciled     — Anthropic Usage and Cost API (reserved; v1+ scope)
unmeasurable             — explicitly absent (rare; rationale + closure ETA required)
```

Operator-paste of `/cost` is not a load-bearing pattern. Capture is automated via OTel + SDK message stream. SDK's `total_cost_usd` is a client-side estimate from a bundled price table; authoritative cost via [Anthropic Usage and Cost API](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) is **deferred to v1+**; the toolkit ships extensibly for that integration.

### Monitoring + Alerting

- **Real-time signals:** agent heartbeats (rides crosslink's existing heartbeat); rate-limit headroom; budget burn rate
- **SLO declarations** in `.vsdd/config.yaml` per project; derived from per-feature axes
- **Anomaly alerts:** 3σ above rolling median per metric
- **Threshold alerts:** declared per SLO; configurable breach action

### Dashboards

| Version | Surface |
|---|---|
| **v1** | Tabular CLI reports (`vsdd observe cycle`, `vsdd observe layer`, `vsdd observe project`) |
| **v2** | HTML output (`vsdd observe cycle --html`) |
| **v3** | Grafana dashboard JSON + Prometheus rules YAML if absorption justifies |

### FinOps applied to AI-driven IAR

Standard FinOps disciplines applied: cost-per-finding × per-domain × per-cycle × per-layer × per-project (aggregable bottom-up); budget-vs-actual per intent-axis; anomaly detection; right-sizing recommendations (model-tier mix vs. defect density); showback (per-domain cost breakdown); unit economics (cost-per-Exit-Signal-attestation).

---

## Verification subsystem

Detailed design in [`DESIGN-VERIFICATION.md`](./DESIGN-VERIFICATION.md). Pre-commit hooks for hard gates + on-demand `vsdd verify check` aggregator. Composes with crosslink's 5 enforcement hooks; does not replace.

### Hook composition

| Hook layer | Count | Owner | Discipline |
|---|---|---|---|
| Crosslink enforcement hooks | 5 | crosslink upstream | Tracking discipline (session-start, prompt-guard, work-check, post-edit-check, pre-web-check + heartbeat) |
| Suite methodology hooks | ~19 | suite-repo | Frontmatter schema validation, citation resolution, classification universe, naming-discipline (incl. letter-label anti-pattern + suite-internal terminology), anonymization (incl. API-key detection), identity-correlation, document staleness, phase-transition provability (consolidated 9-transition matrix), phase-domain composition, draft-PR presence, PR-template conformance, PR-manual-tests-completion, DESIGN.md template conformance, post-DESIGN.md auto-scaffolding (manual-tests + Phase 2a Red Gate skeleton), prose-surface TW + DR composition, CHANGELOG-discipline (consolidated: entry-presence + Keep-a-Changelog structure + version-date + canonical-categories + file-integrity + 5 candidate rules), dependency approval (SO + PE supply-chain + Security investigation for new dependencies), methodology-version-drift (project methodology.md vs toolkit-canonical drift detection) |

Total deployed in a VSDD project: ~24 hooks. Hook count growth governed by earned-by-recurrence trigger; new hooks require 2+ documented drift cases or explicit operator-directive. Hooks are consolidated where logic overlaps (e.g., `check-changelog-discipline.py` covers 10 rules in one hook with multi-rule dispatch). The dependency-approval hook is operator-directive triggered (2026-05-27 directive: any new crate / npm package / pip package requires SO approval + PE supply-chain investigation + Security CVE / threat-model review; living investigation record at `docs/dependencies/<crate>.md`). The methodology-version-drift hook is operator-directive triggered (2026-05-27 directive following Phase 5 round 1 Security F6: project `methodology.md` `methodology_version` is compared against installed toolkit's bundled version; drift fires `VSDD-W0200` warning; refresh via `vsdd init --update-methodology`).

### Hook architecture

Pure-Python hooks operator-side (Claude Code substrate convention preserved); the same enforcement logic exists as Rust binary mirror for CI execution. Two surfaces share the JSON Schema source. No Python-wrapper-invokes-Rust pattern.

Every methodology hook runs in both contexts (operator-local + CI). Bypass-marker requires rationale + PR-approval label.

CI bootstrap: every CI job starts with `vsdd init --ci-mode` to deploy verification artifacts before checks run.

### Soft-audit aggregator

`vsdd verify check` runs drift / staleness / dangling-anchor / cross-cutting integrity checks on demand. Each check emits an observability event regardless of whether invoked via pre-commit or aggregator. The error catalog (`vsdd verify explain <code>`) provides extended documentation for any error/warning/lint code.

---

## Schema enforcement

Detailed design in [`DESIGN-SCHEMA.md`](./DESIGN-SCHEMA.md). YAML frontmatter + per-artifact-class JSON Schema with **semantic versioning per class**. **13 artifact classes** carry the schema discipline; each has structural compliance + cross-reference + credential-exclusion properties mechanically enforced. Most are frontmatter-based; CHANGELOG is structural (whole-file pattern validation).

### Artifact classes (13)

| Class | Frontmatter fields |
|---|---|
| Review entry | review_number, date, phase, scope, lens, source, session_note, model, execution_method, sycophancy_compensation (conditionally required) |
| Finding | finding_id (deterministic from {review, finding-number}), domain, dim, owner, status, blocked_by, validator, classification, source, routing, dismissal_rationale (when Hallucinated/Dismissed) |
| Phase primer | primer_id, phase, version, frequency, governing_skill, relevant_domains, supplements_in_scope |
| Domain prompt | domain_slug, role_titles ([list]), tier (core/extended/meta), activation_criteria, classification_universe, validator_pair, supplements_applied, sycophancy_failure_modes |
| Supplement | supplement_slug, languages_or_interfaces, domains_in_scope |
| Methodology event variants | event_type, capture_source, credential-exclusion structural property |
| `.vsdd/config.yaml` | per-feature axes, SLO declarations, signing config, auth_method, auth_method_credential_source |
| **DESIGN doc** | doc_class (positioning/design/runbook), version, consumes_from (cross-doc refs), produces_for, last_revision_trigger |
| **Methodology spec section** | section_name, required (bool), target_lines, event_variants_referenced, domains_referenced, phases_referenced |
| **Manual-test** | test_class (install-verification/binary/mcp-tool/integration-cycle), layer, target_artifact, tested_against, prerequisites, expected_outcomes, falsifiability_check |
| **Exit Signal record** | attestation_class (exit-signal), project, attestation_commit, attested_by, signature, per_dimension (status, evidence_pointer), cross_dimension_consistency_check, install_verification |
| **PR template** | pr_template_version, required_fields (scope, phase_coverage, composed_domains, co_authors, manual_tests_section, exit_signal_pointer?), excluded_fields (credential-shaped patterns) |
| **CHANGELOG** (structural; not frontmatter-based) | required_top_structure (header + disclaimer + Keep-a-Changelog link), required_sections (`[Unreleased]` + per-release pattern), canonical_categories enum, entry_pattern, sub_section_grouping_allowed |

Pre-phase composition declaration folds into the `PhaseCompositionDeclared` event variant payload (event-variant schema validates the declaration at phase-boundary commit). MCP tool I/O is validated by the MCP protocol natively (each tool registration carries input + output JSON Schemas in the protocol contract); a separate artifact-class validator added redundancy without proportional safety.

CI workflow template meta-schema class reserved as a candidate; promotion to accepted requires a second recurrence case beyond the single existing-suite evidence case. v1 ships without it; CI workflow YAML validates against GitHub's own schema only.

### Anchor IDs derived from frontmatter

`{review_number}-{finding_number}` for findings; `{phase}-{primer_slug}` for primers; etc. Single anchor scheme. No hand-authored `<a id="...">` HTML anchors.

### Schema versioning + forward-only

Per-class semantic versioning. **Additions only** (no breaking changes); deletions or renames require an explicit methodology Review entry + a major-version bump that consuming hooks honor.

---

## Document artifact validation + error surface

The rebuild treats document artifacts the way Rust treats source code. Schema validation runs at commit-time (hooks); at PR-time (CI); at authoring-time (LSP — deferred to v1+). Validators emit errors, warnings, and lints with code identifiers, structured output, and corrective-pattern guidance.

This is **Goal 2's primary operationalization** — the schema/hook/error-catalog system makes "auditable + machine-enforceable + dual-audience-readable" mechanically true rather than aspirational.

### Severity levels

- **Errors** (`VSDD-E####`) block commit/merge
- **Warnings** (`VSDD-W####`) surface but allow commit; surface in CI as PR comment
- **Lints** (`VSDD-L####`) informational; surface in IDE; not in CI gates

### Error catalog

Registered at `vsdd-core/error-catalog.yaml`. Forward-only; retired codes deprecated with migration pointer; per-major-release catalog versioning. Each code carries: severity, summary (Mentor voice), detail, note(s), help (corrective pattern), explain_ref, `status` (candidate | accepted | deprecated).

Error code namespacing convention:
- `VSDD-E0001`-`E0099`: DESIGN-SCHEMA frontmatter validators
- `VSDD-E0100`-`E0199`: DESIGN-VERIFICATION hook violations
- `VSDD-E0200`-`E0299`: Phase-domain composition violations
- W codes parallel ranges with same convention
- L codes for style/lint

**Candidate vs accepted status:** error codes start as `candidate` and graduate to `accepted` when 2+ documented drift recurrences ground the trigger. Candidate codes are authored + tested (validators exist; fixtures present) but emit warnings rather than blocking commits. Accepted codes block per their declared severity. Forward-only governance: codes never reused once retired; deprecated codes carry migration pointers to replacements.

**Strict earned-by-recurrence:** new codes added via methodology amendment citing 2+ recurrence cases. Single-recurrence codes ship as candidate-status pending second-case evidence. Operator-directive-triggered codes (e.g., the PR-discipline codes from the Layer-cycle PR amendment) ship as accepted if the operator directive cites multiple defect cases.

### Output formats

- **TTY:** color-coded human-readable (rustc convention)
- **`--format sarif`:** machine-readable for CI integration (GitHub Code Scanning, GitLab, etc.)
- **`--format json`:** programmatic consumers
- **`--format compact`:** terse single-line for high-density scenarios

### Operator UX

`vsdd verify explain VSDD-E0040` opens extended documentation for any code (matches Rust's `--explain` pattern). LSP integration (deferred to v1+) provides real-time squiggles + quick-fix actions. Error-density throttling consolidates repeated codes (`VSDD-E0010 [×42 in this commit; see first 3 below]`).

### Coverage at v1 — ~25 accepted codes + ~15 candidate codes

Catches drift patterns documented in existing-suite + bookmark-cli-manual review evidence. Accepted codes block per their severity; candidate codes ship as warnings pending second-recurrence promotion.

**Accepted (multi-recurrence + operator-directive triggered):**

| Pattern category | Codes |
|---|---|
| Cross-doc reference resolution | `VSDD-E0010: unresolved cross-reference` |
| Promised artifact missing | `VSDD-E0040: promised-artifact-missing` (TW Layer-2→Layer-3 recurrence) |
| Phase-composition not declared | `VSDD-E0050: phase-composition-not-declared` (bookmark-cli-manual recurrence) |
| Vestigial pattern detection | `VSDD-W0001: vestigial-pattern-detected` (R88 F3 + multiple cycles) |
| Stale-claim suspicion (quantitative) | `VSDD-W0030: stale-claim suspicion` |
| Sycophancy discipline | `VSDD-W0010: sycophancy-compensation-absent` (R83 + multiple cycles) |
| Wall-clock fabrication | `VSDD-W0040: fabricated-time-estimate` (R91 incident — 16x discrepancy) |
| Hook bypass discipline | `VSDD-E0016: bypass-rationale-missing`, `VSDD-W0070: bypass-marker-scope-mismatch` (PR #44 + bookmark-cli-manual recurrence) |
| Manual-test discipline | `VSDD-E0018: manual-test-preamble-incomplete` (R74), `VSDD-W0080: manual-test-checkbox-without-specificity` (G-132) |
| Letter-label anti-pattern | `VSDD-E0160: letter-label-anti-pattern` (R78 F4 + R94 + PR #38/44/52 — 4 recurrences) |
| Prose-surface composition | `VSDD-W0180: prose-surface-commit-without-tw-dr-composition` (TW Layer-2→Layer-3 recurrence) |
| CHANGELOG discipline (operator-directive — adopt crosslink's Keep-a-Changelog pattern) | `VSDD-W0190: changelog-entry-missing` · `VSDD-W0191: changelog-structure-malformed` · `VSDD-W0194: changelog-version-section-missing-date` · `VSDD-W0195: changelog-non-canonical-category` · `VSDD-E0240: changelog-deleted` (all cooperate with `crosslink close` auto-management) |
| PR-discipline (operator-directive) | `VSDD-E0070: draft-pr-missing`, `VSDD-E0080: pr-template-malformed`, `VSDD-W0041: pr-co-authorship-missing`, `VSDD-E0090: pr-manual-tests-incomplete` |
| Dependency approval (operator-directive 2026-05-27) | `VSDD-E0100: dependency-approval-missing` (new entry in `Cargo.toml` / `package.json` / `pyproject.toml` / `requirements.txt` without SO + PE + Security investigation in PR description and corresponding `docs/dependencies/<crate>.md` investigation entry) |
| Auth × CI cross-field validation (Phase 5 Round 1 Security F4) | `VSDD-E0021: auth-method-plan-incompatible-with-ci` (Plan auth declared for CI; structurally invalid — Plan requires operator-interactive session CI cannot provide), `VSDD-W0022: ci-workflows-present-without-ci-auth-declared` (CI workflow files exist without `auth_method.ci` declared) |
| Anonymization | `VSDD-E0110: credential-shaped-value-detected-in-committed-text` (check-anonymization hook fires when API-key / Bearer / env-var-credential patterns appear in committed text) |
| Methodology version pin (Phase 5 round 1 Security F6) | `VSDD-W0200: methodology-version-drift` (project `methodology.md` `methodology_version` < installed toolkit's bundled version; refresh via `vsdd init --update-methodology`) |

**Candidate (single-recurrence; promote on second case):**

| Pattern category | Codes |
|---|---|
| Audit-trail integrity (single-recurrence) | `VSDD-E0011: unverified-citation`, `VSDD-E0012: missing-source-attribution`, `VSDD-E0013: validator-pair-mismatch`, `VSDD-E0023: findings-registry-orphan-row` |
| Cycle discipline (single-recurrence) | `VSDD-E0014: round-scope-not-reduced`, `VSDD-E0015: phase-2a-evidence-shape-missing`, `VSDD-E0017: classification-not-in-universe`, `VSDD-W0060: routing-target-ambiguous` |
| External-review handle | `VSDD-E0019: external-review-handle-inconsistent` |
| Director-raised capture | `VSDD-W0100: director-raised-finding-not-captured` |
| CHANGELOG candidate rules | `VSDD-W0192: changelog-category-label-mismatch` · `VSDD-W0193: changelog-entry-without-issue-reference` · `VSDD-W0196: changelog-unreleased-overflow` · `VSDD-W0197: changelog-breaking-without-semver-major` · `VSDD-L0050: changelog-entry-format-inconsistent` |

The catalog grows forward-only via earned-by-recurrence trigger. Candidate codes can promote without major-version bump; major-version bump required for breaking changes (renaming codes; removing codes; changing severity).

### Validator falsifiability

Each error code has test fixtures at `manual-tests/error-catalog/<code>/{should-fire,should-not-fire}/`. `vsdd verify test-error-catalog` runs the regression suite.

### How the system interfaces

**Schema ↔ Hooks:** schemas are rules (the type system); hooks are executors. One JSON Schema; two enforcement surfaces (Python hook operator-side + Rust mirror CI-side). All validation outcomes emit `HookFired` + `ValidationFailed`/`ValidationPassed` events to the OTel collector + `.vsdd/events.jsonl`.

**Schema ↔ Reviews:** schemas catch mechanical drift at commit-time; reviews catch judgment-bearing concerns at cycle-time. Phase 3 reviewers operate on artifacts where schema-validation has already passed — reviewer attention shifts from mechanical-defect-finding to semantic-coherence + sycophancy + threat-modeling judgment. Review-log entries are themselves schema-validated (Review entry artifact class); reviewer output mechanically structured.

**Schema ↔ Goal 2:** the triad of enforcement mechanisms (schema validator OR hook OR crosslink workflow check per Goal 2) converges at the user-facing error catalog. Operator sees `VSDD-E0040` regardless of which mechanism caught the rule. Auditable (every validation emits events), machine-enforceable (rule → mechanical enforcement), dual-audience-readable (humans see Mentor-voice errors + explain pages; agents consume structured frontmatter + SARIF + events).

---

## Finding management

Crosslink issues + typed labels (when crosslink is in use). The suite's `domain:/layer:/round:/finding:/classification:/source:` label schema is implementation-shape-pitched upstream as `crosslink label declare --namespace <ns>` + AND-filter `issue list -l a -l b`.

Until those upstream items land, the suite uses untyped namespaced string labels (`domain:quality-engineer`) + jq pipelines over `crosslink issue list --json`.

When crosslink is not in use (rare; not the supported launch method, but acknowledged as fallback), findings live as YAML-frontmatter markdown files in `.vsdd/findings/`. Schema-identical to the crosslink-issue case.

---

## Domain set

**16 role + 2 meta = 18 domains.**

### Core role domains (6)

Software Engineer, Quality Engineer, UX, Security, Solution Architect, Solution Owner.

### Extended role domains (10)

Platform Engineer, Data Engineer, Red Team, Performance Engineer, Technical Writer, Documentation Reviewer, Accessibility, Privacy, Localization, AI Engineer.

### Meta domains (2)

**VSDD Methodology** — semantic-coherence reviewer of methodology application. Surviving dimensions: spec-vs-implementation semantic alignment; methodology-spirit adherence; cross-session semantic continuity; methodology-evolution coherence. Activation: on-demand (not gate-criterion).

**Sanity Check** — validator-of-last-resort + rubber-ducking surface. Activates automatically via hook when `validator: sanity-check` declared in a finding's frontmatter.

### Per-domain prompt shape

Each of the 18 domain prompts is ~80-150 lines. Vestigial-pattern cuts applied: job-title variants → frontmatter; sycophancy-check failure modes → frontmatter + methodology spec; coordination matrix → methodology spec once; validator-pair paragraph → methodology spec mapping table; three-audience lens section retires; classification universe extensions → single universe with rationale-required `Accepted`.

Total domain-prompt content: ~1,500-2,000 lines across 18 files.

---

## Adversarial review stance: The Exacting Mentor

The rebuild's adversarial reviewer adopts an **Exacting Mentor** stance — a sustainable, multi-domain framing that scales across many reviews without reviewer fatigue. Unnamed stance; not personified. Primer 3 retains the whitepaper-canonical "Adversarial Refinement (The VDD Roast)" subtitle for source-material continuity; the stance the reviewer adopts is the Exacting Mentor.

### Core stance

You are an experienced reviewer who has seen this defect class before. You hold the work to the standard you know it can meet — because you believe the author can reach that standard, not because you're suspicious of them. Your tone is direct, specific, and exacting. You don't pull punches; you also explain why something is wrong + what the better version looks like + what corrective pattern applies. Sycophancy resistance is rooted in standards, not paranoia: letting a defect slide because the author tried hard would be the failure mode, not the kindness.

### Five lenses

Every finding answers at least one of these lenses. Reviewers carry all five in peripheral attention; apply the lens that matches the surface.

1. **Attacker's mindset** — "If I were trying to break this, where would I attack?" Probe injection vectors, authentication bypass, race conditions, resource exhaustion, deserialization, supply-chain insertion. Frame findings as defensive opportunities, not gotchas.
2. **Edge cases** — "What about the conditions the happy-path code skips?" Probe empty input, null, max-size, off-by-one, unicode normalization, concurrent access, partial failure, timeout, signal interrupts. Name *which* edge cases for *this* surface.
3. **Usability** — "Will the operator know what to do when they see this?" Think from the seat of the person using the system. Is the error helpful? Is the affordance discoverable? Is the API ergonomic? Is the failure recoverable?
4. **Maintainability** — "Will future-developer understand and modify this?" Think six months out. Right level of abstraction? Hard-to-undo decisions named? Seam visible for the next feature?
5. **Consistency** — "Does this match the spec? Does the doc match the code? Does this match how the rest of the project does it?" Catch drift between surfaces — small per instance, large in aggregate.

### Tone-flex policy

Mentor voice is the default across operator-facing surfaces. Formal voice is the exception for legally-record-style attestations + schema declarations.

| Surface | Tone |
|---|---|
| Per-finding bodies | Mentor (direct + specific + growth-framed) |
| Phase 3 Round close (per-domain) | Mentor |
| Layer-gate close summary | Mentor |
| Sycophancy-check failure-mode descriptions | Mentor (naming the class as a growth opportunity) |
| Hook output messages on failure | Mentor (name the violation + the corrective pattern) |
| Phase 6 Exit Signal record | Formal (signed attestation with timestamp + table-structured per-dimension status); Mentor-voice retrospective paragraph optional |
| Methodology amendment landing | Formal |
| Schema definitions + JSON Schemas | Formal (typed declarations; no prose) |
| Methodology spec opening | Formal positioning + Mentor-voice examples |

### Domain-as-skill vs domain-as-reviewer

Domain prompts compose with the Exacting Mentor stance in two invocation modes. Same substance; different session contexts + output shapes. Both modes are first-class methodology surfaces.

| Property | Domain-as-skill (operator-interactive) | Domain-as-reviewer (cold-session sub-agent) |
|---|---|---|
| Entry point | `/vsdd-domain-<slug>` slash command | `crosslink swarm review` with knowledge-page mandate |
| Session context | Operator-interactive; prior context present | Cold; worktree-isolated; no prior context |
| Output shape | Free-form conversational; Mentor voice | Structured review-entry per primer 3 schema |
| Cardinality | One domain at a time (operator-driven) | Multi-domain parallel cluster with adversarial-pair separation |
| Use cases | Rubber-ducking with a lens; pre-Phase-3 self-review; design-question lens; per-phase composition (Phase 1a-2c, 5) | Phase 3 Adversarial Refinement round (canonical multi-domain pass) |
| Output destination | Operator's terminal (not persisted unless captured) | Review-log markdown + crosslink issues + events.jsonl |

Skill-mode is conversational-only by design. Phase 3 review-entries must come from cold-session reviewer dispatch to preserve audit-trail integrity.

---

## Phase taxonomy

Strict whitepaper-canonical 10 sub-phases:

1a Behavioral Spec → 1b Verification Architecture → 1c Spec Review Gate → 2a Test Suite Generation → 2b Minimal Implementation → 2c Refactor → 3 Adversarial Refinement (The VDD Roast) → 4 Feedback Integration Loop → 5 Formal Hardening (Mutation Testing / Fuzz Testing / Purity Boundary Audit / Proof Execution) → 6 Convergence (The Exit Signal)

Operators can author Phase 1a and 1b in a single session if they choose; the methodology spec lists them as distinct phases per the whitepaper.

Phase 5 + Phase 6 are first-class methodology phases; projects choose whether to execute them. The methodology spec describes when each surface is useful + what each looks like.

---

## Phase-domain composition

Each phase composes with its relevant domains. The matrix is load-bearing methodology, enforced at four layers (matrix declaration; per-primer instruction; pre-phase declaration; commit-time hook).

| Phase | Primer | Relevant domains (composed via skill mode unless noted) |
|---|---|---|
| 1a Behavioral Spec | `vsdd-phase-1a` | SO (primary); UX, Accessibility, Privacy, Localization per per-feature axes |
| 1b Verification Architecture | `vsdd-phase-1b` | SO + SA + QE (test strategy) |
| 1c Spec Review Gate | `vsdd-phase-1c` | SA (primary); SO co-stewards for spec-gate close |
| 2a Test Suite Generation | `vsdd-phase-2a` | QE (primary) |
| 2b Minimal Implementation | `vsdd-phase-2b` | SE (primary); TW + DR (project-state staleness); QE (test-pyramid maintenance); DE / AI Eng / etc. per per-feature axes |
| 2c Refactor | `vsdd-phase-2c` | SE + SA |
| 3 Adversarial Refinement | `vsdd-phase-3` | All active domains (cold-session reviewer mode) |
| 4 Feedback Integration | `vsdd-phase-4` | None specific (operator-orchestrated routing) |
| 5 Formal Hardening | `vsdd-phase-5` | QE + Security + SA |
| 6 Convergence (Exit Signal) | `vsdd-phase-6` | None specific (operator-orchestrated attestation) |

Phase 3 is the only phase where domains compose as reviewers (cold-session sub-agents producing structured findings). Every other phase uses skill mode (operator-interactive).

### Four enforcement mechanisms

1. **Matrix declaration** — load-bearing methodology section. Operators see the composition as structural.
2. **Per-primer composition instruction** — each non-Phase-3 primer's opening prose explicitly instructs domain-as-skill loading. Primer frontmatter carries `relevant_domains: [<list>]` as a machine-readable declaration.
3. **Pre-phase composition declaration** — symmetric with primer 3's pre-cycle methodology check. Emits a `PhaseCompositionDeclared` observability event. Absent declaration is itself a finding.
4. **`check-phase-composition.py` hook** — fires at phase-boundary commits. Missing declaration requires bypass-marker with rationale.

### Layer-cycle PR discipline

Each layer's work lands in a single PR opened early + accumulated incrementally. Closes the existing-suite Layer 1 anti-pattern (CI/CD work concentrated at layer-close generating a cluster of late-cycle PE findings).

- **Draft PR opens at Phase 2a commit.** The PR accumulates the layer's commits + receives early review feedback; closes when layer-gate criteria are met.
- **PE tooling lands incrementally.** When Phase 2b adds a dependency, the corresponding PE artifact (lockfile, audit gate, env pin) lands in the same commit — not deferred to layer-close. Phase-domain composition matrix accordingly adds **PE** to Phase 2b composed-domains (incremental tooling).
- **PR description follows the templated structure** (PR template artifact class). Required fields: layer scope, phase coverage checklist, composed-domains declaration per phase, TW + DR co-authorship trailers, manual-test checklist section (auto-generated by `vsdd observe pr-body --layer N`), Exit Signal pointer (when layer closes).
- **TW + DR compose with PR-description authoring** (cross-phase composition). TW for prose-surface updates (README, DESIGN, PROCESS, CHANGELOG, `manual-tests/layer-N.md`); DR for cold-reader review of the PR description itself.
- **Commit-level domain co-authorship.** Same TW + DR discipline applies at commit-level via git `Co-authored-by:` trailers. Identifier convention: `Co-authored-by: Technical Writer <tw@vsdd-domains>` + `Co-authored-by: Documentation Reviewer <dr@vsdd-domains>`. The synthetic `@vsdd-domains` email signals domain-lens attribution (not a real person); standard `git log` / `git shortlog` / `git blame` tooling surfaces the lens composition. The `check-prose-surface-tw-dr-composition.py` hook validates either `Composed-domains:` trailer OR `Co-authored-by:` trailers (the latter preferred — richer audit trail; standard git tooling integration). Other phases extend the convention via the phase-domain composition matrix: Phase 2b commits add SE + QE co-authorship; Phase 2a commits add QE; Phase 1c commits add SA + SO.
- **Manual-test checklist** items embedded in PR description as GitHub markdown task list. Operator checks items as they execute manual testing. PR merge gate (`VSDD-E0090: pr-manual-tests-incomplete`) validates all items checked or deferred-with-rationale per primer 1c discipline.
- **PR ready-for-review when layer-gate criteria met.** Phase 3 multi-domain review runs against the PR.

Three new methodology event variants track the PR lifecycle: `DraftPROpened` (Phase 2a boundary) → `PRReadyForReview` (layer-gate close) → `PRMerged` (final). Two new hooks enforce: `check-draft-pr-presence.py` + `check-pr-template-conformance.py`.

---

## Intent calibration via per-feature axes

Per-feature axes in `.vsdd/config.yaml` per project. Each axis drives one downstream calibration.

```yaml
ships-to-users-other-than-developer: yes
network-exposed: yes
persists-managed-schema-data: yes
handles-user-data: yes
safety-critical: no
formal-verification-candidates: no
ui-surface: no
localized: no
ai-runtime-cost-relevant: no
```

| Axis | Activates |
|---|---|
| `ships-to-users-other-than-developer: yes` | Documentation Reviewer + Technical Writer (default) |
| `network-exposed: yes` | Red Team + Security (extended) |
| `persists-managed-schema-data: yes` | Data Engineer |
| `handles-user-data: yes` | Privacy |
| `safety-critical: yes` | Phase 5 Mutation Testing + Purity Boundary Audit recommended |
| `formal-verification-candidates: yes` | Phase 5 Proof Execution recommended |
| `ui-surface: yes` | UX (default if not already activated) + Accessibility |
| `localized: yes` | Localization |
| `ai-runtime-cost-relevant: yes` | AI Engineer |

Each axis is independent. No tier vocabulary. Cold-session budget bands per axis-combination land in [`DESIGN-METHODOLOGY.md`](./DESIGN-METHODOLOGY.md).

**Always-on domain baseline (additive over axes):** SE + QE + SA + SO activate regardless of axes. PE + PerfE activate when the project ships code (any source file in `src/` / `lib/` / equivalent). The axes matrix above extends from this baseline — a zero-axes project that ships code still has 6 composed domains (not zero). See [`DESIGN-METHODOLOGY.md` § Always-on domain baseline](./DESIGN-METHODOLOGY.md#always-on-domain-baseline).

---

## Forward-only disciplines

Two distinct names; two distinct disciplines:

**Event-log append-only (data discipline).** Structural property of the event log. Records are append-only; no in-place updates. Rides crosslink's existing append-only NDJSON pattern + the suite's own `.vsdd/events.jsonl` v1 sink. Disaster recovery: `.vsdd/events.jsonl` committed to git per cycle; `git checkout` recovers from prior commit.

**Documentation narrative-preservation (prose discipline).** Authoring discipline for the methodology spec + primers + domain prompts + supplements. Pre-rebuild artifacts in the existing suite stay as authored per the original semantics.

**The forward-only restriction kicks in at a stability commitment — not from the toolkit's first commit.** Pre-stability (early development, pre-design synthesis, pre-v1.0): git history can be rewritten freely (rebase, amend, drop commits); dated review-log entries can be re-authored as the discipline evolves. The trigger conditions for forward-only application (any of):

- **v1.0 release** (semver-stability commitment to public consumers)
- **First push to a public remote** (external visibility boundary; observers may depend on history)
- **First downstream adoption** (another project's CI/audit-trail references this repo's history)
- **Operator-declared methodology-stabilization milestone** (explicit `OperatorDirectiveApplied{directive: methodology-stabilization}` event emission; consolidates with the existing variant per the variant-proliferation governance discipline)

Whichever fires first locks narrative-preservation forward-only for the toolkit's own history. Before any of these fires, the methodology benefits from history-malleability — fixing initial-commit attribution, consolidating early commits, refining discipline retroactively. After the trigger: append-only forward-only applies; redactions only via append-only mechanisms; retroactive edits to dated entries are findings for VSDD Methodology meta-domain.

The existing suite's narrative-preservation discipline continues to apply to its own history (the existing suite passed its stability commitment long ago).

---

## Bypass-marker mechanism

Hybrid: HTML-comment `<!-- hook-bypass[hook-id]: rationale -->` works mechanically across markdown rendering pipelines. Frontmatter `bypass: [hook-id]` works cleanly for typed artifacts. Both supported. Bypass without rationale fails the hook; PR-approval label required at merge-gate.

---

## CHANGELOG discipline

The toolkit adopts crosslink's CHANGELOG management pattern verbatim. CHANGELOG.md follows [Keep a Changelog 1.0.0](https://keepachangelog.com/en/1.0.0/) format with crosslink-compatible conventions:

- `[Unreleased]` section at top; accumulates between releases
- Per-release sections with semver + date: `## [0.8.0] - 2026-04-17`
- Canonical categories: Added / Changed / Deprecated / Removed / Fixed / Security (Keep-a-Changelog order)
- Sub-section thematic clusters within categories for substantial-feature releases (e.g., `#### Sentinel — Autonomous Maintenance Agent` inside Added)
- Entries reference issues/PRs/findings in brackets: `([#443])`, `([GH-650])`, `([XL-N])` per the issue tracker's convention

### CHANGELOG is a first-class artifact class

CHANGELOG.md joins the 14 frontmatter-based classes as the 15th class with a **structural schema** (whole-file pattern validation rather than frontmatter parsing). The `check-changelog-discipline.py` hook is consolidated multi-rule:

| Rule | Error code | Status | What it catches |
|---|---|---|---|
| Entry presence | `VSDD-W0190: changelog-entry-missing` | Accepted | Substantive commit without CHANGELOG entry |
| Structural compliance | `VSDD-W0191: changelog-structure-malformed` | Accepted | Missing header / disclaimer / Keep-a-Changelog ref / `[Unreleased]` section |
| Version-date format | `VSDD-W0194: changelog-version-section-missing-date` | Accepted | `## [N.M.P]` without ` - YYYY-MM-DD` |
| Canonical categories | `VSDD-W0195: changelog-non-canonical-category` | Accepted | Categories outside {Added, Changed, Deprecated, Removed, Fixed, Security} |
| File integrity | `VSDD-E0240: changelog-deleted` | Accepted | CHANGELOG.md existed at last manifest snapshot; now missing |
| Category-label alignment | `VSDD-W0192: changelog-category-label-mismatch` | Candidate | Entry's source-commit label doesn't match section category |
| Entry issue reference | `VSDD-W0193: changelog-entry-without-issue-reference` | Candidate | Entry without `([#NNN])` or equivalent |
| Unreleased overflow | `VSDD-W0196: changelog-unreleased-overflow` | Candidate | `[Unreleased]` accumulates beyond operator-set threshold |
| Breaking-vs-semver | `VSDD-W0197: changelog-breaking-without-semver-major` | Candidate | Changed-with-BREAKING without major-version bump at release |
| Entry format consistency | `VSDD-L0050: changelog-entry-format-inconsistent` | Candidate (lint) | Entries mixing `-` / `*`; inconsistent issue-ID format |

5 accepted (operator-directive triggered by adoption of crosslink pattern); 5 candidate (await recurrence evidence OR operator-set thresholds).

### Cooperation with crosslink

When crosslink is the project's issue tracker, `crosslink close` auto-manages CHANGELOG.md entries:
- Creates CHANGELOG.md with the Keep-a-Changelog template if absent
- Categorizes entries by issue label (`bug`/`fix` → Fixed; `feature`/`enhancement` → Added; `breaking` → Changed; `deprecated` → Deprecated; `removed` → Removed; `security` → Security; default → Changed)
- Appends to `[Unreleased]` under the appropriate category

The toolkit's hook **detects entries already added by crosslink** + passes validation. Never duplicate-writes. When operator runs `vsdd verify check`, validates Keep-a-Changelog structure of whatever's there.

### When crosslink is not in use

Rare fallback. `vsdd init` deploys `CHANGELOG.md.vsdd-template` as a side-by-side template (NOT as CHANGELOG.md) if file absent. Operator adopts manually. Template is verbatim Keep-a-Changelog skeleton:

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security
```

Auto-creation tooling: `vsdd verify changelog --create` candidate v1+ subcommand replicates crosslink's auto-create behavior for operators not using crosslink.

### vsdd's own CHANGELOG

The `vsdd-cli` repo's own CHANGELOG.md follows this pattern verbatim. The toolkit dogfoods its own discipline.

---

## Naming + coinage governance

The methodology applies disciplines to its own evolution. New terms, event variants, error codes, hooks, and artifact classes do not enter the methodology speculatively.

### Earned-by-recurrence trigger

Methodology amendments require 2+ documented drift recurrences OR explicit operator-directive citing equivalent evidence. Single-recurrence additions ship as `status: candidate` (validators exist; emit warnings; do not block) and graduate to `accepted` on the second case. Forward-only: codes never reused once retired; deprecated entries carry migration pointers.

### Vocabulary registry

Canonical methodology terms live at `.vsdd/registry/vocabulary.yaml` (deployed by `vsdd init`). Documents using deprecated aliases fire `VSDD-W0001: vestigial-pattern-detected`. Adding a term requires a methodology-amendment commit. TW co-authors term-introductions; DR cold-reader review asks "is this term necessary?" before merge.

### Letter-label anti-pattern enforcement

Per multi-recurrence evidence (R78 F4 Surface A/B/C/D + R94 + PR #38/44/52 cluster-letter recurrences), `check-naming-discipline.py` fires `VSDD-E0160: letter-label-anti-pattern` for label patterns like `Surface [A-Z]`, `Cluster [A-Z]`, `Mode [A-Z]`, `Path [A-Z]`, `Tier [A-Z]`, `Pillar [N]`. Acceptable: `Dim N`, `Layer N`, `Round N`, `Finding N`, `Phase Na` — the concept-word is in the identifier.

### Coinage discipline

Author-introduced cognitive scaffolding terms (terms invented for organizational convenience rather than as methodology load-bearing concepts) do not enter the methodology spec. Adding a term requires recurrence evidence OR operator-directive. The methodology favors descriptive prose over named-mechanism shorthand.

### Artifact-class + event-variant + hook governance

- Adding a 15th artifact class requires methodology amendment + earned-by-recurrence record
- Adding a 19th event variant requires same — consolidate to generic variants first (e.g., `ArtifactScaffolded` covers manual-tests + Red Gate skeleton + future scaffolding outputs; resists per-outcome variant proliferation)
- Adding a hook requires earned-by-recurrence record; existing hooks consolidated where logic overlaps (the naming-discipline hook covers letter-labels + suite-internal-terminology + vocabulary-registry conformance in one)

---

## Migration from the existing suite

**Sibling, not successor.** The existing suite at [`guild-projects/guild-portfolio/vsdd-suite/`](https://github.com/magnificentlycursed/guild-portfolio/tree/main/vsdd-suite) continues to exist as a record of methodology evolution. The toolkit starts fresh; no artifact migrates by copy.

- Operators of the existing suite are not auto-migrated. The rebuild's adoption is a new-project decision.
- The existing `bookmark-cli-manual` reference example stays where it is; no parallel `bookmark-cli-v2` is committed (the toolkit's own development cycles serve as canonical dogfood).
- The existing suite's forward-only narrative-preservation discipline continues to apply to its own history (passed stability commitment long ago). The toolkit's documentation-narrative-preservation discipline applies forward-only AFTER stability commitment (v1.0 release / first public push / first downstream adoption / operator-declared methodology-stabilization directive); pre-stability history is malleable. See [Forward-only disciplines](#forward-only-disciplines).

---

## License

**MIT** matching crosslink. Minimizes friction for any future absorption + signals continuity with upstream ecosystem.

---

## Implementation order

| Track | Goal-4 surface? |
|---|---|
| 1a — Author DESIGN-SCHEMA.md (foundational; unblocks 1b + 1c) | Foundational |
| 1b — Author DESIGN-OBSERVABILITY.md | Yes |
| 1c — Author DESIGN-VERIFICATION.md | Yes |
| 1d — Re-validate DESIGN-METHODOLOGY.md against 1a-1c | Coordination |
| 2a — Implement vsdd crate (1 crate, 1 binary with subcommands: init / verify / observe / mcp-serve) | Yes |
| 2b — Implement ~17 Python hooks + Rust mirror (incl. post-DESIGN.md auto-scaffolding hook + PR-discipline hooks + consolidated CHANGELOG-discipline hook + naming-discipline hook + prose-surface TW+DR composition hook) | Yes |
| 2c — Author 10 phase primers | No |
| 2d — Author 18 domain prompts (with vestigial-pattern cuts) | No |
| 2e — Author 14 supplements (with cuts) | No |
| 2f — Author methodology spec | No |
| 2g — Author CI workflow templates | Goal-4 specific |
| 2h — Implement methodology + substrate-docs MCP server (full v1 deliverable) | No |
| 2i — Deploy default OTel collector config + sink wiring | Yes |
| 2j — Auth-method declaration UX + event variants + anonymization hook API-key detection | Cross-cutting |
| 2k — Implement error catalog (~30 codes) + validator falsifiability fixtures + `vsdd verify explain` | Yes (Goal 2 operationalization) |
| 2l — Author DESIGN.md template + vocabulary registry + canonical-patterns registry + anonymization-patterns registry (shift-left) | No |
| 2m — Implement post-DESIGN.md auto-scaffolding hook (manual-tests + Phase 2a Red Gate skeleton; shift-left) | No |
| 2n — Author 13 artifact-class JSON Schemas | Foundational |
| 2o — Implement PR template + CODEOWNERS deployment + PR-discipline hooks (draft-pr-presence + pr-template-conformance + pr-manual-tests-completion) | Cross-cutting |
| 3 — Goal 4 end-to-end demonstration via rebuild's own CI | Goal-4 specific |

Time estimates intentionally absent. The operator-time constraint is binding regardless of whether it's quantified; sequencing + scope-discipline are the calibration until the toolkit's own observability subsystem produces cycle telemetry that derives future-cycle estimates from real data.

This is the high-level implementation order; methodology-subsystem-specific tracks (per-domain prompt authoring; per-class JSON Schema authoring; methodology spec section drafting) live in [`DESIGN-METHODOLOGY.md` § Implementation order](./DESIGN-METHODOLOGY.md#implementation-order) and resolve as DESIGN-METHODOLOGY is revalidated against DESIGN-SCHEMA + DESIGN-OBSERVABILITY + DESIGN-VERIFICATION.

Upstream coordination activity (filing bugs upstream; pitching absorption candidates) is operator-activity outside the toolkit's scope.

---

## Closing

The rebuild composes the methodology + observability + verification + adoption against crosslink + Claude Code. Each surface has one owner; each interface is versioned; each subsystem is independently reasonable-about; each is small enough to read end-to-end in an afternoon.

The toolkit's success criterion is the methodology applied to itself: when an adversarial reviewer runs Phase 3 against the `vsdd-cli` repository and produces only hallucinated findings, the Exit Signal is reached. The toolkit is just another project under the methodology.
