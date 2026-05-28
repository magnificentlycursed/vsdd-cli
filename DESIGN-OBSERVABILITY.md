# DESIGN-OBSERVABILITY.md

Design document for the observability subsystem of the `vsdd` toolkit. Defines the OTel collector design, sink wiring, 18 methodology event variant payloads + capture-source provenance, three-pillars + dashboard ladder + FinOps surfaces, the MCP server architecture, and the `vsdd observe` CLI surface.

```yaml
# Pre-authoring composition declaration
authoring_target: DESIGN-OBSERVABILITY.md
composed_domains: [solution-architect, ai-engineer, platform-engineer, security]
composition_mode: skill-interactive
operator_confirmation: confirmed (operator directive 2026-05-27)
declared_at: 2026-05-27
substrate:
  crosslink: initialized in vsdd-cli
  legacy_reference: https://github.com/magnificentlycursed/guild-portfolio/tree/main/vsdd-suite/
  methodology_anchors: [README.md, DESIGN-METHODOLOGY.md, DESIGN-SCHEMA.md, primer 3]
  schema_dependency: DESIGN-SCHEMA.md (per-event-variant payload schemas + credential-exclusion property)
```

For positioning: see [`README.md`](./README.md). For methodology subsystem: [`DESIGN-METHODOLOGY.md`](./DESIGN-METHODOLOGY.md). For schema enforcement: [`DESIGN-SCHEMA.md`](./DESIGN-SCHEMA.md). Sibling: [`DESIGN-VERIFICATION.md`](./DESIGN-VERIFICATION.md) (consumes hook-fire events; emits validation events).

---

## Scope + boundary

DESIGN-OBSERVABILITY owns:

- OTel collector design + default config + sink wiring
- 18 methodology event variant payload mapping to OTel signals
- Capture-source provenance discipline
- Three pillars (logs / metrics / traces) — sources + sinks + query mechanisms
- FinOps applied to IAR cycles
- Dashboard ladder (v1 CLI / v2 HTML / v3 Grafana absorption-pitch)
- MCP server (`vsdd mcp-serve`) — tool dispatch + cache + fetch primitive
- `vsdd observe` CLI subcommand surface
- Credential redaction at collector forwarding boundary
- Cost-relevant signal capture-source assignment (replaces the retired cost-tally tier discipline)
- Usage and Cost API extensibility (v1+; not v1 deliverable)

DESIGN-OBSERVABILITY does NOT own:

- Schemas themselves (DESIGN-SCHEMA — per-event-variant payload definitions)
- Hook implementations (DESIGN-VERIFICATION — hooks emit observability events; this doc owns what gets emitted)
- Methodology spec content (DESIGN-METHODOLOGY)
- CLI binary distribution + cargo workspace (DESIGN-VERIFICATION)
- Per-domain prompt content (DESIGN-METHODOLOGY)

---

## Substrate composition

The toolkit composes against the [Claude Agent SDK](https://code.claude.com/docs/en/agent-sdk/observability) as the observability primitive. The SDK runs the Claude Code CLI as subprocess + emits three OTel signals (metrics + log events + traces) to any OTLP-compatible backend when telemetry is enabled.

**Enabled by** `CLAUDE_CODE_ENABLE_TELEMETRY=1` + per-signal exporter env vars. `vsdd init` deploys these env vars in `.vsdd/env-vars` for operator to source.

**The toolkit augments, doesn't reinvent:** methodology-specific event variants (the 18 listed below) emit alongside the SDK's built-in signals; both flow through the same collector + sink.

### SDK OTel signal inventory (consumed; not reinvented)

| Signal | Source | What's captured |
|---|---|---|
| Metrics | Agent SDK CLI subprocess | Token counts, cost (client-side estimate), session count, lines of code, tool decisions |
| Log events | Agent SDK CLI subprocess | Prompts, API requests, errors, tool results |
| Traces | Agent SDK CLI subprocess + W3C trace context propagation | `claude_code.interaction`, `claude_code.llm_request`, `claude_code.tool`, `claude_code.hook` spans |

Sub-agent spawns via the Task tool nest under parent's span tree; full delegation chain visible as one trace.

### vsdd-specific augmentation layer

Methodology event variants emit as OTel custom log events with `service.name=vsdd` resource attribute. The collector configures dual-routing: SDK signals + custom events flow to the same backend(s); per-event filtering at sink time.

---

## OTel collector design

### Default config deployment

`vsdd init` deploys `.vsdd/otel-collector.yaml` with default sinks + commented external-backend stubs:

```yaml
# .vsdd/otel-collector.yaml (default deployment)
receivers:
  otlp:
    protocols:
      http:
        endpoint: 127.0.0.1:4318

processors:
  batch:
    timeout: 5s
  # Credential redaction — MUST run before any exporter in every pipeline (processor ordering invariant)
  # Patterns sourced from .vsdd/registry/anonymization-patterns.yaml (operator-extensible per-project)
  redaction:
    config_source: .vsdd/registry/anonymization-patterns.yaml
    # The registry file declares api_key_patterns + bearer_token_patterns + credential_attribute_names
    # Operator extends by editing the registry file; collector reloads on next cycle start
    # See DESIGN-VERIFICATION § check-anonymization scope for the canonical pattern catalog

exporters:
  # Default: write to .vsdd/events.jsonl (suite-side audit trail)
  file/events-jsonl:
    path: .vsdd/events.jsonl
    rotation:
      max_megabytes: 100
      max_days: 30
  # Default: forward to crosslink hub when available
  otlphttp/crosslink-hub:
    endpoint: http://127.0.0.1:8443/v1/events
    headers:
      X-Crosslink-Source: vsdd
    sending_queue:
      enabled: true
      retry_on_failure: true

  # Commented examples for external backends — operator uncomments + sets endpoint + token env var
  # otlphttp/honeycomb:
  #   endpoint: https://api.honeycomb.io
  #   headers:
  #     x-honeycomb-team: ${HONEYCOMB_API_KEY}
  # otlphttp/datadog:
  #   endpoint: https://otlp.datadoghq.com
  #   headers:
  #     DD-API-KEY: ${DATADOG_API_KEY}
  # otlphttp/grafana:
  #   endpoint: ${GRAFANA_OTLP_ENDPOINT}
  #   headers:
  #     Authorization: Basic ${GRAFANA_AUTH}

service:
  pipelines:
    logs:
      receivers: [otlp]
      processors: [redaction, batch]
      exporters: [file/events-jsonl, otlphttp/crosslink-hub]
    metrics:
      receivers: [otlp]
      processors: [redaction, batch]
      exporters: [file/events-jsonl, otlphttp/crosslink-hub]
    traces:
      receivers: [otlp]
      processors: [redaction, batch]
      exporters: [file/events-jsonl, otlphttp/crosslink-hub]
```

**Operator extension:** uncomment external-backend entries + set credential env vars. The collector config itself is operator-editable; vsdd-init never overwrites operator changes via the managed-section pattern.

**External-backend operator-confirmation:** adding a new external backend endpoint to the collector config is a substantive change — event-log forwarding to a new external destination is a data-exfiltration attack surface if the endpoint is malicious. `vsdd observe collect start` detects new external-backend endpoints (vs. last-known-good state via init-manifest SHA) and prompts operator-confirmation before forwarding begins. Confirmation emits `OperatorDirectiveApplied{directive: external-backend-added, endpoint: <hostname-only>, rationale: <text>}` event (hostname only — never the auth token). Operator can pre-confirm via `vsdd observe collect start --confirm-external-backends` flag for non-interactive contexts (CI).

### Sink wiring

Default sinks (always-on):

- **`.vsdd/events.jsonl`** — local NDJSON append-only file; git-tracked per cycle; disaster recovery via `git checkout`. Schema-validated by event-variant payload schemas (DESIGN-SCHEMA).
- **crosslink hub** — when crosslink is in use; HTTP OTLP endpoint at crosslink's hub URL. Crosslink's `seam.rs` consumes; events.rs schema-compatible.

Operator-extensible sinks:

- Honeycomb / Datadog / Grafana / Langfuse / self-hosted Jaeger / any OTLP-compatible backend
- Per-backend credential discipline: env-var-only storage; collector config references via `${VAR_NAME}` substitution; never inline values

### Credential redaction at forwarding boundary

The redaction processor (per Security composition) runs **before any export**. Every signal passing through the collector is scanned for:

- API key patterns (`sk-ant-api03-`, `sk-[A-Za-z0-9]+`, GitHub `ghp_` tokens)
- `Bearer <token>` HTTP header patterns
- Credential-shaped attribute names (auth_method_credential_value, api_key, bearer_token, password, secret, etc.)

Matches are redacted to `[REDACTED-VALUE-MATCHING-CREDENTIAL-PATTERN]` placeholder.

**`OTEL_LOG_RAW_API_BODIES` stays default-off** — enabling it would route full API request bodies (including credential-shaped values in headers) through the OTel pipeline, undermining the redaction discipline. Operator can enable for debugging in private contexts but the methodology recommends keeping it off.

### Collector lifecycle (on-demand)

The collector is **not** a persistent daemon. Spawned on-demand per cycle or per `vsdd observe` invocation. Avoids platform-specific systemd/launchd wrapper complexity at single-operator scale.

| Phase | Lifecycle action |
|---|---|
| `vsdd init` | Deploy config + env vars to `.vsdd/env-vars` for operator to source |
| Operator-local cycle start | `vsdd observe collect start` spawns collector subprocess; SDK emits to localhost:4318; collector PID tracked at `.vsdd/.cache/collector.pid` |
| Operator-local cycle close | `vsdd observe collect stop` (or auto-stop on `vsdd verify check` cycle-close detection); collector flushes pending events + exits cleanly |
| CI cycle | Collector spawned per-job OR endpoints point directly to crosslink hub (collector-less mode); per-job lifecycle bounded |
| Cycle commit | Operator commits `.vsdd/events.jsonl` |

**On-demand pattern wins for single-operator scale.** Persistent-daemon pattern revisits if multi-operator concurrent sessions or always-on dashboard rendering surfaces evidence-of-need.

### Processor ordering invariant

The redaction processor **must** run before any exporter in every pipeline. The collector config snippet's `processors: [redaction, batch]` declares this ordering; validation hook (deferred to v1+ per the candidate-track discipline) confirms no pipeline exists with batch-before-redaction.

This is a structural invariant — events containing credential-shaped values never reach a sink without passing through redaction first.

### Cost characteristics

Per AI Engineer dim 9 (validator wall-clock budget):

| Operation | Expected cost |
|---|---|
| Collector startup | ~200ms |
| Per-event ingest | <1ms |
| Per-event redaction pass | <1ms |
| Batch export to file sink | <10ms per batch |
| Batch export to crosslink hub | <100ms per batch (network-bound) |

Cumulative collector overhead < 1% of cycle wall-clock at typical event volumes (~1000 events/cycle).

---

## 18 methodology event variant payloads

Per DESIGN-SCHEMA's variant payload table, each emitted as OTel custom log event with `service.name=vsdd` + `event_type=<variant>` resource attributes. Payload schemas live in DESIGN-SCHEMA; this doc maps them to OTel emission patterns.

### Phase lifecycle variants (5)

| Variant | When emitted | Capture-source |
|---|---|---|
| `PhaseEntered` | Phase-boundary commit detected by hook OR operator-invoked `vsdd verify hook phase-transition` | vsdd-custom-event |
| `PhaseExited` | Phase-completion commit detected | vsdd-custom-event |
| `PhaseTransitionAttested` | check-phase-transitions.py validates transition + records attestation | vsdd-custom-event |
| `PhaseCompositionDeclared` | check-phase-composition.py validates declaration at phase entry | vsdd-custom-event |
| `ArtifactScaffolded` | post-design-md-modification hook auto-scaffolds manual-tests OR Phase 2a Red Gate stubs | vsdd-custom-event |

### Finding lifecycle variants (3)

| Variant | When emitted | Capture-source |
|---|---|---|
| `FindingRaised` | Per-Finding entry committed to review-log | vsdd-custom-event |
| `FindingClassified` | Finding's `classification` field set/changed | vsdd-custom-event |
| `FindingRouted` | Finding's `routing` field populated | vsdd-custom-event |

### Cycle convergence variants (1)

| Variant | When emitted | Capture-source |
|---|---|---|
| `ExitSignaled` | Phase 6 attestation committed; per-dimension status + signature recorded | vsdd-custom-event |

### Discipline-enforcement variants (3)

| Variant | When emitted | Capture-source |
|---|---|---|
| `SycophancySelfAudit` | Review-log entry with `sycophancy_compensation` field declared | vsdd-custom-event |
| `OperatorDirectiveApplied` | Operator-directive recorded in event log (methodology amendments, scope decisions, methodology-stabilization milestone, etc.) | vsdd-custom-event |
| `ProtectiveDisciplineEnforced` | Anonymization / identity-correlation / permission-policy hook fires + redacts | vsdd-custom-event |
| `VerificationMiniCycleSpawned` | Round 3+ verification mini-cycle triggered | vsdd-custom-event |

### Auth + identity variants (1)

| Variant | When emitted | Capture-source |
|---|---|---|
| `AuthMethodDeclared` | At `vsdd init` (folded into ProjectInitialized) OR when operator changes auth method | vsdd-custom-event |

### Project lifecycle variants (1)

| Variant | When emitted | Capture-source |
|---|---|---|
| `ProjectInitialized` | At `vsdd init` completion; carries `vsdd_toolkit_version`, `axes_declared`, `auth_method`, `deployed_artifacts_manifest` | vsdd-custom-event |

### PR lifecycle variants (3)

| Variant | When emitted | Capture-source |
|---|---|---|
| `DraftPROpened` | check-draft-pr-presence.py detects new draft PR at Phase 2a | vsdd-custom-event |
| `PRReadyForReview` | PR transitions from draft to ready; layer-gate criteria met | vsdd-custom-event |
| `PRMerged` | PR merge detected; Exit Signal pointer captured if applicable | vsdd-custom-event |

Total: 18 variants. Each carries the common envelope (`agent_id`, `agent_seq`, `timestamp`, `signed_by`, `signature`, `capture_source`) + per-variant payload (defined in DESIGN-SCHEMA).

### Per-variant cardinality classification

For external-backend cost-at-scale planning, each payload field is classified as:

- **dimension** — low-cardinality; safe to use as a backend tag/label; permanent in time-series indexes
- **metric** — high-cardinality OR continuous value; emitted as numeric measurement, not as label
- **attribute** — variable-cardinality; available for query but not pre-indexed; per-record only

Cardinality classification per variant (excerpt — full table lives in `vsdd-core/schemas/<variant>.json` schema metadata):

| Variant | Dimension fields (low cardinality; safe to index) | Metric fields | Attribute fields (variable cardinality; per-event only) |
|---|---|---|---|
| `PhaseEntered` | `phase`, `layer` | `started_at` (timestamp) | `composed_domains` (array; cardinality bounded by 18-domain enum) |
| `FindingRaised` | `domain`, `dim` | (none) | `finding_id` (high cardinality per project) |
| `FindingClassified` | `classification`, `domain` | (none) | `finding_id`, `dismissal_rationale` (free text) |
| `ExitSignaled` | `project` | `attested_at` (timestamp) | `attestation_commit` (high cardinality), `dimension_status_map` (object) |
| `AuthMethodDeclared` | `auth_method` | (none) | `auth_method_credential_source` (env-var name; bounded but project-variable) |
| `ProjectInitialized` | `vsdd_toolkit_version`, `auth_method` | (none) | `deployed_artifacts_manifest` (object; high cardinality) |

External-backend operator chooses which fields to index (incur per-cardinality storage cost) vs query-time-only (attribute fields). Per-variant schemas declare the classification; collector tags signals appropriately for downstream backend consumption.

---

## Capture-source provenance (replaces retired cost-tally tier discipline)

Every cost-relevant event carries `capture_source` enum. The cost-tally tier discipline (`agent_self_verifiable` / `operator_verifiable` / `operator_confirmable` / `derived`) retires. Replaced by:

```
capture_source values:
  otel-metric              — Agent SDK OTel metric (tokens, cost, sessions, tool decisions)
  otel-log-event           — Agent SDK OTel log event (prompts, API requests, errors, tool results)
  otel-trace-attribute     — Agent SDK OTel span attribute
  vsdd-custom-event        — methodology-specific event variant
  sdk-result-message       — SDK's per-query total_cost_usd + modelUsage + cumulative usage
  usage-api-reconciled     — Anthropic Usage and Cost API (deferred to v1+)
  unmeasurable             — explicitly absent (rare; rationale + closure ETA required)
```

`vsdd observe` reports surface the capture-source per-field so cost data carries provenance into reports. Operator-paste of `/cost` is not a load-bearing pattern.

---

## Three pillars

| Pillar | Source | Sink (v1) | Query mechanism |
|---|---|---|---|
| **Logs (events)** | Agent SDK OTel log events + 18 methodology custom events | `.vsdd/events.jsonl` + crosslink hub | `vsdd observe` reads file; SQL via jq pipelines OR via crosslink hub's queryable endpoint |
| **Metrics** | Agent SDK OTel metrics + derived from event log at query time | `vsdd observe` Rust aggregator | Pulled at command time; no time-series database in v1 |
| **Traces** | Agent SDK OTel traces (span trees) + finding-lifecycle as span tree | Query-time derivation from event log | `vsdd observe trace --finding-id <id>` walks event log to reconstruct |

### Metrics derivation

Methodology-relevant metrics derived at query time from event log:

- **Cost-per-finding** — sum tokens emitted between `FindingRaised` and `FindingClassified` events; divide by classified-non-hallucinated count
- **Cost-per-Exit-Signal-attestation** — total tokens emitted in cycle / 1 (per project)
- **Per-domain cost breakdown** — sum tokens within `PhaseEntered{phase:3}` / `PhaseExited{phase:3}` spans, group by cluster-membership
- **Per-cycle agent count** — count distinct `agent_id` in events between PhaseEntered + PhaseExited for phase-3
- **Per-cycle wall-clock** — first event timestamp to last event timestamp
- **Hook fire counts** — count `HookFired` events from DESIGN-VERIFICATION emission
- **Validation failure rate per error code** — `ValidationFailed` events grouped by error code

### Trace assembly

Finding lifecycle as span tree:

```
finding.lifecycle (root span)
├── FindingRaised (commit hash + author + dim)
├── FindingClassified (classification + dismissal_rationale if applicable)
├── FindingRouted (target_phase + target_artifact)
├── Resolution-commit-trace (if Resolved; cross-reference to commit-author + co-authors)
└── ValidatorClosure (when validator confirms close)
```

Query: `vsdd observe trace --finding-id 5-f3`.

W3C trace context auto-propagates from Agent SDK; sub-agent spawns nest under parent. Finding-lifecycle spans correlate via finding-id attribute.

---

## Dashboard ladder

| Version | Surface | Audience |
|---|---|---|
| **v1** | Tabular CLI reports via `vsdd observe cycle`, `vsdd observe layer`, `vsdd observe project` | Operator at cycle / layer / project close |
| **v2** | HTML output via `vsdd observe cycle --html` | Operator-experience-test reveals desired surfaces |
| **v3** | Grafana dashboard JSON + Prometheus rules YAML | Cross-project FinOps; absorption-pitch material |

### v1 CLI report shapes

```
$ vsdd observe cycle --layer 3 --round 1
═══════════════════════════════════════════════════════════════
Cycle: bookmark-cli Layer 3 Round 1
═══════════════════════════════════════════════════════════════
Phase: 3 (Adversarial Refinement)
Duration: 2h14m (Bash-instrumented; agent did NOT count time)
Agents: 4 (cluster-batched; adversarial-pair separation preserved)
Cluster shape: Implementation + Architecture + Communication + Adversarial
─── Findings ──────────────────────────────────────────────────
Total: 14 (3 Resolved / 5 Deferred / 4 Dismissed / 2 Hallucinated)
Per-domain: SE 3 | SA 2 | QE 3 | TW 2 | DR 2 | Security 1 | UX 1
─── Cost (SDK-estimated; capture-source: sdk-result-message) ──
Tokens: 1.2M total
  - Input: 800k
  - Output: 350k
  - Cache read: 50k
Estimated cost (USD): $4.20 (caveat: client-side estimate; not authoritative)
─── Hooks fired ───────────────────────────────────────────────
HookFired: 47 (47 pass / 0 violation)
ValidationFailed: 0
═══════════════════════════════════════════════════════════════
```

### v2 HTML output

Same data as v1 but rendered as standalone HTML with embedded charts. `--html` flag emits `cycle-report.html` next to the event log.

### v3 Grafana absorption-pitch

Per-cycle metrics exported to Prometheus-compatible format; Grafana dashboards rendered from declarative JSON. v3 lands when absorption-pitch material requires it.

---

## FinOps applied to IAR cycles

Standard FinOps disciplines applied to AI-driven cycles:

- **Cost-per-finding** × per-domain × per-cycle × per-layer × per-project (aggregable bottom-up)
- **Budget-vs-actual** per intent-axis (per-feature axes activate domain set + cold-session budget multiplier)
- **Anomaly detection** — 3σ above rolling median per metric; alerts via `PushNotification`
- **Right-sizing recommendations** — model-tier mix vs. defect density; per AI Engineer tuning-lever catalog
- **Showback** — per-domain cost breakdown surfaced in cycle reports
- **Unit economics** — cost-per-Exit-Signal-attestation tracked per project; allows cross-project comparison

### Tuning levers (carried forward from AI Engineer methodology amendments)

| Lever | Cost delta | When to apply |
|---|---|---|
| Model-tier right-sizing (Opus 4.7 → Sonnet 4.6 → Haiku 4.5) | ~10x reduction (Haiku vs Opus) | Mechanical sweeps; audit-trail-only passes |
| Prompt-cache discipline (5-min TTL respect) | ~10x reduction on cache-hit | Sub-agent batches within 5-min window |
| Cluster-batching (4-cluster vs 18 per-domain) | ~60% agent count reduction | Default for Phase 3 cycles |
| Sub-agent scope-down (focused prompt + file slice) | Reduces re-load cost | All sub-agent spawns |
| N+1 file-reread elimination (warm-context handoff) | Eliminates redundant reads | Operator-orchestrator handoffs |
| Plan vs API auth selection | Plan: $0 marginal within credits; API: predictable per-token | Per use case (operator-local: Plan; CI: API) |

---

## MCP server (`vsdd mcp-serve`)

Per DESIGN-METHODOLOGY's MCP server section. Single server; 4 tools. Implemented as `vsdd mcp-serve` subcommand (preserves single-crate-single-binary workspace).

### Tool surface (4 tools)

| Tool | Scope | Cost characteristic (per-query token band) |
|---|---|---|
| `vsdd.methodology.lookup(query, scope?)` | Methodology spec + DESIGN docs + supplements + domain prompts | 1-5k tokens per query (small) |
| `claude_code.docs.search(query, page?)` | Claude Code docs at code.claude.com/docs | 5-20k tokens (WebFetch + extract) |
| `crosslink.docs.search(query, page?)` | Crosslink documentation | 5-20k tokens |
| `anthropic.api.docs.search(query)` | Anthropic API documentation (stub in v1; full in v1+) | v1: <1k tokens (stub return) |

Each tool's input + output schemas declared per the MCP tool I/O artifact class (DESIGN-SCHEMA).

### Internal architecture

Three layers:

1. **Tool dispatch** — MCP protocol handling; routes incoming tool calls to per-tool handlers
2. **Cache layer** — `.vsdd/mcp-cache/` (gitignored; TTL-bounded); pre-warmed at `vsdd init` time
3. **Fetch primitive** — for cache misses, WebFetch-equivalent (HTTP GET + markdown extract)

### Cache strategy

| Cache type | TTL | Refresh trigger |
|---|---|---|
| Methodology lookup (own repo content) | File-mtime-aware (no TTL) | Cache invalidated when source markdown's mtime changes; revalidation per-query against filesystem; methodology spec changes propagate immediately |
| Claude Code docs (external) | 24 hours | Operator runs `vsdd observe mcp-cache refresh` OR scheduled cron task |
| Crosslink docs (external) | 24 hours | Same |
| Anthropic API docs (external) | 24 hours (v1+) | Same |

Methodology lookup uses file-mtime-aware invalidation rather than TTL because the source can change mid-session — operator edits methodology.md or DESIGN docs; cache must reflect immediately. External-substrate-doc caches use TTL because their source changes infrequently + WebFetch is the costly path.

Stale-cache fetch (TTL-expired) triggers async refresh; serves stale during refresh. Stale-cache served logged via `MCPCacheStaleServed` (proposed candidate event; deferred under variant-proliferation governance until recurrence evidence).

### Cooperation with crosslink knowledge subsystem

Per the substrate-docs cataloging discipline (cooperate with existing substrate affordances rather than parallel-reinvent): the MCP server cooperates with `crosslink knowledge`:

- Crosslink knowledge pages registered for vsdd-domain + vsdd-supplement at `vsdd init`
- MCP `vsdd.methodology.lookup` first queries crosslink knowledge (registered pages); falls back to direct file read if knowledge not registered
- No duplicate-storage

### Absorbability framing

The doc-search-via-MCP pattern is a Tier 2 cold-pitch candidate for crosslink absorption (per UPSTREAM-COORDINATION future-authored doc). Generic pattern — extensible beyond VSDD methodology to any-doc-search use case.

---

## `vsdd observe` CLI subcommand surface

### Subcommand list

```
vsdd observe cycle              # cycle-level report
vsdd observe layer              # layer-level report
vsdd observe project            # project-level report
vsdd observe metrics            # event-log + OTel aggregation
vsdd observe metrics --auto     # auto-detect cycle from active state
vsdd observe trace --finding-id <id>   # finding-lifecycle span tree
vsdd observe pr-body --layer <N>       # auto-generate PR description with manual-test checklist
vsdd observe mcp-cache refresh         # refresh substrate doc cache
vsdd observe mcp-cache status          # show cache hit rates + TTL state
vsdd observe explain <metric>          # explain a metric's derivation
vsdd mcp-serve                         # top-level subcommand for MCP server (long-running stdio loop)
```

### Output formats

- **TTY:** color-coded human-readable (rustc convention; uses ANSI escape codes per terminal capability)
- **`--format json`** — programmatic consumers
- **`--format yaml`** — operator-pipeline-friendly
- **`--format compact`** — terse single-line for high-density scenarios
- **`--html`** (cycle / layer / project only) — emits HTML report next to event log

### Auto-generated PR body (Pattern B)

`vsdd observe pr-body --layer <N>` reads `manual-tests/layer-N.md` + project state + emits PR description:

```markdown
## Scope
Layer 3: bm export + bm import (round-trip workflow)

## Phase coverage
- [x] Phase 1a — Behavioral spec (DESIGN.md § Layer 3 contracts)
- [x] Phase 1b — Verification architecture
- [x] Phase 1c — Spec review gate
- [x] Phase 2a — Red Gate (15 failing tests at commit 878d3b6)
- [x] Phase 2b — Implementation (commit fd21900)
- [x] Phase 2c — Refactor (commit 78bd3cf)
- [ ] Phase 3 — Adversarial Refinement (in progress)
- [ ] Phase 4 — Feedback Integration
- [ ] Phase 5 — Formal Hardening
- [ ] Phase 6 — Convergence

## Composed-domains per phase
[per phase-domain composition matrix; auto-populated]

## Co-authors
- Technical Writer <tw@vsdd-domains>
- Documentation Reviewer <dr@vsdd-domains>
- Solution Engineer <se@vsdd-domains>
- Quality Engineer <qe@vsdd-domains>

## Manual tests checklist
[auto-generated from manual-tests/layer-3.md]
- [ ] bm export emits all bookmarks as storage-format JSON
- [ ] bm export against absent store emits {"bookmarks":[]}
- [ ] bm export --tag OR-union
- [ ] bm export --tag "" rejected
- [ ] ... (per-AC checkbox list)

## Exit Signal pointer
[populated when layer closes; references ExitSignaled event]
```

---

## Auto-create CHANGELOG cooperation (v1+ scope)

Per the CHANGELOG discipline (DESIGN-METHODOLOGY): when crosslink is in use, `crosslink close` auto-manages CHANGELOG.md. The toolkit cooperates — never duplicate-writes.

For projects not using crosslink: candidate `vsdd verify changelog --create` subcommand replicates crosslink's auto-create behavior. Promotion trigger: second VSDD-adopting project running without crosslink + needing CHANGELOG bootstrap. Until then, side-by-side `CHANGELOG.md.vsdd-template` deployment at `vsdd init` covers the gap.

Routing: this is `vsdd verify` subcommand surface (lives in DESIGN-VERIFICATION), but the CHANGELOG-format definition + Keep-a-Changelog adherence lives here (observability + audit-trail concern).

---

## Usage and Cost API extensibility (v1+ scope)

SDK's `total_cost_usd` is client-side estimate from bundled price table — not authoritative.

For per-cycle reports in v1: SDK estimate is used. `vsdd observe` reports surface the caveat: "client-side estimate; not authoritative billing."

For v1+: `vsdd observe metrics --reconcile-usage-api` calls Anthropic's [Usage and Cost API](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) + reconciles. Authoritative cost per cycle / per month / per project. Capture-source `usage-api-reconciled`.

**Architecture extensibility for v1+:** the metric derivation layer already takes `capture_source` as a discriminator. Adding `usage-api-reconciled` as a source is additive — no breaking changes to existing event-log consumers.

---

## Cross-DESIGN-doc coordination

### What this doc produces

| Consumer | Consumes from this doc |
|---|---|
| **DESIGN-VERIFICATION** | OTel emission convention for `HookFired` + `ValidationPassed` / `ValidationFailed` events from hook execution; collector + sink wiring; redaction processor expectations for credential-shaped output |
| **DESIGN-METHODOLOGY** | Capture-source enum + per-variant payload mapping informs methodology spec section on event log discipline; FinOps tuning lever catalog informs cost-discipline section |
| **DESIGN-SCHEMA** | OTel emission constraints (what shape the collector expects) inform event-variant payload schemas — coordinate-loop with DESIGN-SCHEMA |

### What this doc forward-references

| Sibling | This doc forward-references |
|---|---|
| **DESIGN-SCHEMA** | Per-variant payload schemas + credential-exclusion structural property (consumed for variant emission validation) |
| **DESIGN-VERIFICATION** | Hook implementation details — what fires when, what's emitted, what's blocked |
| **DESIGN-METHODOLOGY** | Methodology spec section on observability + FinOps + dashboard discipline |

---

## Implementation order

| Track | Goal-4 surface? |
|---|---|
| 4a — Author `.vsdd/otel-collector.yaml` template + redaction processor config | Yes (Goal 3 flagship; required for cost/usage capture) |
| 4b — Implement `vsdd observe` CLI subcommands (`cycle`, `layer`, `project`, `metrics`, `trace`, `pr-body`) | Yes (FinOps reports; PR body auto-generation) |
| 4c — Implement `vsdd mcp-serve` (4 tools; cache layer; fetch primitive) | No (but agents leverage in all phases) |
| 4d — Implement per-cycle metric derivation pipeline (event-log → metrics) | Yes |
| 4e — Implement HTML report renderer (v2 dashboard ladder) | No (v1 ships TTY; HTML in v1+ if operator-experience tests reveal demand) |
| 4f — Implement W3C trace context propagation handling for finding-lifecycle spans | No |
| 4g — Implement collector lifecycle wrapper (systemd/launchd OR operator-managed) | Yes (Goal 4 CI needs collector startup) |
| 4h — Author Grafana dashboard JSON + Prometheus rules YAML (v3 absorption-pitch material) | No (v1+ scope) |
| 4i — Implement Anthropic Usage API reconciliation (`vsdd observe metrics --reconcile-usage-api`) | No (v1+ scope; architecture extensibility ensured) |

Tracks 4a-4d + 4g are v1 deliverables; 4e + 4f + 4h + 4i are v1+ scope.

---

## Open decisions deferred

| Decision | Routing |
|---|---|
| Collector binary distribution (use upstream `otelcol-contrib` OR ship a `vsdd-otelcol` build with pre-bundled redaction processor) | DESIGN-VERIFICATION (binary distribution scope) |
| Per-tool MCP cost-budget enforcement at server side (e.g., reject queries that would exceed cost-band) | DESIGN-METHODOLOGY (operator-policy decision) |
| Methodology dashboard ladder v3 design (Grafana JSON schema; metric naming convention) | DESIGN-OBSERVABILITY (this doc, future iteration) |
| Trace assembly performance at high event volumes (does query-time derivation scale to 100k+ events per project?) | DESIGN-OBSERVABILITY (this doc, future iteration after evidence) |
| `vsdd observe mcp-cache` pre-warm schedule + sources | DESIGN-OBSERVABILITY (this doc, future iteration) |
| External-backend recommendation matrix (which backend for which use case) | Operator-runbook (out of methodology scope) |

---

## Closing

DESIGN-OBSERVABILITY operationalizes Goal 3 (observability as first-class) + Goal 4 (CI/CD shift-left observability surfaces). The architecture composes against the Agent SDK + OTel as primitive — augmentation rather than reinvention. 18 methodology event variants surface methodology-lifecycle telemetry alongside the SDK's built-in signals. Three pillars + dashboard ladder + FinOps applied to IAR. MCP server brings substrate-doc + methodology lookup into every Claude Code session.

The flagship status holds: every artifact born observable; every action emits; default-on, not opt-in. The collector + redaction processor make credential-exclusion structural rather than aspirational.

**Next:** DESIGN-VERIFICATION authoring (in parallel with this doc; consumes from this doc's emission conventions + collector design).
