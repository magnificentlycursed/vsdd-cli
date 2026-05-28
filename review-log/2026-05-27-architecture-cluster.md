---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-05-27
phase: phase-3
scope: Tier 1 spec artifact pass — README + methodology.md + DESIGN-METHODOLOGY + DESIGN-SCHEMA + DESIGN-OBSERVABILITY + DESIGN-VERIFICATION — applying SA + PE + DE lenses against decomposition, dependency-management, schema discipline, persistence, supply-chain, and CI-pipeline surfaces
lens: Architecture cluster — SA + PE + DE lenses applied to Tier 1 spec artifacts
source: domain-raised
session_note: cold-context — first Phase 3 IAR round on the spec set; Architecture cluster composed via 3-domain skill-mode aggregation in cold-context Agent spawn (worktree-isolated by harness; no operator-memory poisoning beyond the harness-provided cwd + supplied artifacts)
model: claude-opus-4-7
execution_method: Agent-tool subagent spawn (cold-context approximation; harness-isolated; no prior-cycle memory mounted)
sycophancy_compensation: I am Claude Opus 4.7 reviewing artifacts authored by another instance of Claude Opus 4.7 in the same conversational session that dispatched me — substrate-shared author. The bias is to accept the surface-coherence of the spec ("this all fits") and to accept the named numbers as load-bearing ("18 events; 19 hooks; 13 artifact classes — these must be right because they're declared three times"). I specifically resisted: (1) accepting DESIGN-VERIFICATION's 19-hook count as load-bearing-and-coherent when the post-DESIGN.md auto-scaffolding entry isn't a validator and shares no error-code surface with the other 18, (2) accepting the "single-binary-with-subcommands" decomposition without testing whether the `vsdd-core` / `vsdd` split actually constitutes one binary or two crates with a re-export pattern, (3) accepting the Python-subprocess-to-Rust-binary hook architecture without auditing the cold-start cost claim, and (4) accepting `.vsdd/events.jsonl` as a coherent persistence story when the methodology asserts both "append-only structural property" AND "git-tracked per cycle with `git checkout` recovery" which are different durability stories.
---

# Architecture cluster Review 1 — 2026-05-27

**Phase 3 surface:** Adversarial Refinement (Exacting Mentor stance) — Architecture cluster (SA + PE + DE composed)
**Cold-session shape:** Agent-tool spawn from main session; harness provides cwd-isolated artifact tree; no prior-cycle memory mounted; no operator-feedback carried; per the cluster-batching default per DESIGN-METHODOLOGY § Cluster-batching shape.

## Scope

The Tier 1 artifact set declares the methodology, the schema discipline, the observability surface, and the verification surface as four sibling DESIGN docs orbiting `methodology.md` + `README.md`. The Architecture cluster's job is to apply 3 lenses (Solution Architect: decomposition + seams + purity-boundary + hard-to-undo decisions; Platform Engineer: reproducibility + supply-chain + CI discipline; Data Engineer: schema discipline + migration + persistence + backup-recovery) against the spec as authored and surface findings the in-author review didn't catch. Sibling clusters (Implementation, Communication, Adversarial) will surface their own; coordination notes flag overlap candidates.

The Phase 5 round 1 reviews already landed substantial cross-doc consistency fixes (count drift, orphan error codes, auth × CI cross-field validation, always-on baseline, methodology-version-pin). This Phase 3 pass works against the current state and tries to find what's *still* drifted after those fixes — plus structural / architectural concerns that Phase 5 didn't enumerate.

## Findings

### Finding 1 — `vsdd-suite_version` vs `vsdd_toolkit_version` payload field name drift (Dim SA-5 + DE-1) — Open

The `ProjectInitialized` event variant has its payload field declared inconsistently across docs:

- `DESIGN-SCHEMA.md:319` — `ProjectInitialized { vsdd_toolkit_version, axes_declared, auth_method, deployed_artifacts_manifest }`
- `DESIGN-OBSERVABILITY.md:255` — `ProjectInitialized` carries `vsdd_toolkit_version, axes_declared, auth_method, deployed_artifacts_manifest`
- `DESIGN-OBSERVABILITY.md:284` (cardinality table) — `vsdd_toolkit_version` (dimension field)
- `README.md:336` — "carries fields: `vsdd_suite_version`, `axes_declared`, `auth_method`, `deployed_artifacts_manifest`"
- `DESIGN-METHODOLOGY.md:861` — "auth_method + axes_declared + vsdd_suite_version + deployed_artifacts"

DESIGN-SCHEMA is the type-system source-of-truth and uses `vsdd_toolkit_version`. README + DESIGN-METHODOLOGY use `vsdd_suite_version` (a residual name from the existing-suite era — "suite" predates the "toolkit" reframing). This is a hard-to-undo decision for a *persisted* schema: once events ship in `.vsdd/events.jsonl` with the field name fixed, every downstream consumer (crosslink hub's events.rs; future Grafana dashboards; the per-variant cardinality classification at DESIGN-OBSERVABILITY:284) is pinned to that name. Migrating later requires the schema-version-bump dance DESIGN-SCHEMA documents — and that hasn't been built yet.

**Why this matters (DE lens):** persisted-data field names outlive the code that writes them. Per Data Engineer dim 1 (schema discipline) + dim 4 (data validation at boundaries), the field name has to be authoritative before any event is emitted. The drift means the Rust-types-as-source-of-truth pipeline (`vsdd-core/src/schemas/event_variants.rs` → `schemars` → JSON Schema) doesn't know which name to generate. Whichever name lands in the first commit becomes the v1.0.0 baseline.

**Why this matters (SA lens):** hard-to-undo decisions named (SA dim 5). This is one — and it's not yet resolved.

**Routing:** Phase 4 → Phase 1a (DESIGN-SCHEMA-authoritative — pick one; sweep three other sites). Suggest `vsdd_toolkit_version` (matches the README's own "vsdd is a Rust toolkit" framing; "suite" is the residual existing-suite vocabulary that the rebuild explicitly disavows per `README.md:106`).

- **finding_id:** 1-f1
- **domain:** solution-architect (primary) + data-engineer (composed)
- **dim:** SA-5 (hard-to-undo decisions named) + DE-1 (schema discipline)
- **classification:** Open
- **routing:** target_phase: phase-1a; target_artifact: DESIGN-SCHEMA.md as authority; sweep README.md:336 + DESIGN-METHODOLOGY.md:861

---

### Finding 2 — `HookFired` / `ValidationPassed` / `ValidationFailed` are load-bearing events with no payload schema (Dim DE-1 + SA-2) — Open

The trio of validation-emission events is referenced repeatedly as the canonical mechanism that connects schemas → hooks → observability:

- `methodology.md:287` — section frontmatter declares `event_variants_referenced: [..., ValidationPassed, ValidationFailed, HookFired]`
- `DESIGN-SCHEMA.md:97` — "Both modes share the error-catalog + Mentor-voice output + observability emission (`HookFired` + `ValidationPassed` / `ValidationFailed` events)"
- `DESIGN-OBSERVABILITY.md:326-327` — "**Hook fire counts** — count `HookFired` events from DESIGN-VERIFICATION emission" + "**Validation failure rate per error code** — `ValidationFailed` events grouped by error code"
- `DESIGN-VERIFICATION.md:837` — cross-doc table entry produces "OTel emission convention for `HookFired` + `ValidationPassed` / `ValidationFailed` events"
- `README.md:340` — "Schema-validation events (`ValidationPassed` / `ValidationFailed` / `HookFired`) augment via the document artifact validation surface (described below)"

But: the 18-event-variant list at `DESIGN-SCHEMA.md:305-323` does NOT include these three. Per DESIGN-OBSERVABILITY:208 "**18 methodology event variant payloads**" — the per-variant payload schemas are claimed to be authoritative there + in DESIGN-SCHEMA. Neither doc declares a payload schema for HookFired, ValidationPassed, or ValidationFailed.

**The architecture gap:** these events are simultaneously (a) the load-bearing carriers of the `vsdd verify check` output stream, (b) the source of the per-cycle "Hook fire counts" + "Validation failure rate per error code" metrics, (c) the events consumed by crosslink hub (per the "events.rs schema-compatible" claim), and (d) NOT in the 18-variant catalog that DESIGN-SCHEMA promises to author payload schemas for.

**Why this matters (DE lens):** Per Data Engineer dim 4 (data validation at boundaries — write-time validation matches declared schema): if there's no declared schema for `HookFired{?}` then write-time validation can't run; the redaction processor at the OTel collector can't validate the event's credential-exclusion structural property; the downstream consumer reading `.vsdd/events.jsonl` can't know what fields are guaranteed-present.

**Why this matters (SA lens):** Per Solution Architect dim 2 (architectural seam clarity) — the schemas-to-hooks-to-events seam runs through these three events; the seam is invisible at the spec layer because the events aren't typed there. A future implementer reading DESIGN-VERIFICATION cannot author the hook's emission code without back-referencing DESIGN-OBSERVABILITY's metric derivation expectations.

**Routing:** Phase 4 → Phase 1a (DESIGN-SCHEMA authors the payload schemas; DESIGN-OBSERVABILITY updates per-variant cardinality classification table; methodology.md and README catalogs reconcile to "18 methodology variants + 3 validation-event variants = 21 total" OR consolidate validation events as augmentation outside the methodology-variant count). Three resolution paths:
- (a) Promote the three to first-class methodology variants — bumps count to 21; requires payload schema authoring per DESIGN-SCHEMA discipline.
- (b) Document them as "validation-emission augmentation events" distinct from "methodology event variants" — clarifies which catalog they live in; still requires payload schemas.
- (c) Drop the references entirely and surface their data through OTel's native log-event shape — loses the per-cycle metric derivation that DESIGN-OBSERVABILITY claims.

Suggest (b) — preserves the methodology-variant count discipline while typing the validation events.

- **finding_id:** 1-f2
- **domain:** data-engineer (primary) + solution-architect (composed)
- **dim:** DE-1 (schema discipline) + SA-2 (architectural seam clarity)
- **classification:** Open
- **routing:** target_phase: phase-1a; target_artifact: DESIGN-SCHEMA.md (event variants section)

---

### Finding 3 — `MCP tool I/O artifact class` reference is stale in DESIGN-OBSERVABILITY after class drop (Dim SA-2 cross-source consistency) — Open

DESIGN-SCHEMA dropped the MCP tool I/O artifact class per the 15→13 consolidation, with explicit rationale at `DESIGN-SCHEMA.md:573-577`:

> ### MCP tool I/O — DROPPED as standalone class
> Per Phase 3 multi-domain review consolidation: MCP protocol natively validates tool schemas (each tool registration carries input + output JSON Schemas as part of the MCP protocol contract). vsdd's 4 tools are defined as Rust code in `vsdd/src/mcp_serve/` with derive-macro-generated schemas...

But `DESIGN-OBSERVABILITY.md:429` still asserts:

> Each tool's input + output schemas declared per the MCP tool I/O artifact class (DESIGN-SCHEMA).

This is a residual reference to a class that no longer exists in the artifact-class set. A future implementer authoring `vsdd/src/mcp_serve/` reads DESIGN-OBSERVABILITY for tool-cost characteristics + follows the MCP-tool-I/O-class breadcrumb to DESIGN-SCHEMA where the class is declared dropped — confusion + wasted spec-trace.

**Why this matters (SA lens):** Per SA dim 7 (abstraction altitude) — the toolkit deliberately chose to delegate MCP tool validation to the MCP protocol itself rather than adding a vsdd artifact-class layer on top. That decision is load-bearing for the "minimum-viable-surface" framing. A stale cross-reference contradicts the consolidation decision.

This is the same defect class as Phase 5 SA F1+F2 (count drift; orphan anchor-ID rules) — a Phase 4 sweep that didn't catch a residual mention.

**Routing:** Phase 4 → Phase 1a (DESIGN-OBSERVABILITY:429 spec revision; trivial fix — replace the reference with "MCP protocol's native input/output schema declaration" + cross-link to the Rust source-of-truth at `vsdd/src/mcp_serve/`).

- **finding_id:** 1-f3
- **domain:** solution-architect
- **dim:** SA-2 (architectural seam clarity) — same defect class as Phase 5 SA F1+F2
- **classification:** Open
- **routing:** target_phase: phase-1a; target_artifact: DESIGN-OBSERVABILITY.md:429

---

### Finding 4 — README declares "~18 methodology hooks" while every other doc says "~19" (Dim SA-2 cross-source consistency) — Open

`README.md:13` declares:

> - adds a verification subsystem (~18 methodology hooks + 13 schema-validated artifact classes + Rust-like error catalog)

Every other site says 19:
- `README.md:117` — "**The verification subsystem** — ~19 methodology hooks composing with crosslink's 5 enforcement hooks (~24 total in a VSDD project)"
- `README.md:149` — "Deploys ~19 methodology hooks..."
- `README.md:248` — "`.claude/hooks/*.py` | ~19 methodology hooks deployed by `vsdd init`"
- `DESIGN-VERIFICATION.md:3, 29, 170, 697, 745, 899` — `~19` consistently
- `DESIGN-METHODOLOGY.md:859, 964` — `~19`
- `methodology.md:413` — `~19`

The README opening paragraph is the project's authoritative positioning statement (the first thing anyone sees). The single `~18` is exactly the kind of drift Phase 5 QE F5 surfaced for the deployment-matrix table — an isolated number that didn't get swept when the count moved.

**Why this matters (SA lens):** Per SA dim 6 (cross-cutting concerns applied uniformly) — count claims are cross-cutting; one wrong count makes a future reader question all of them. The README opening is high-prominence; the defect is high-visibility.

**Why this matters (PE lens):** PE dim 4 (CI workflow discipline) — the CI workflow that runs `vsdd verify check` deploys exactly N hooks; the count is operational. If a future toolkit version legitimately moves to 18 or 20, the operator should be able to trust that every doc reflects the same answer.

**Routing:** Phase 4 → Phase 1a (README.md:13 fix). Bundle with any other count-drift sweep this cycle surfaces.

- **finding_id:** 1-f4
- **domain:** solution-architect
- **dim:** SA-6 (cross-cutting concerns) + SA-2 (cross-source consistency)
- **classification:** Open
- **routing:** target_phase: phase-1a; target_artifact: README.md:13

---

### Finding 5 — `cargo install vsdd --locked` from source in every CI job is a Goal-4-shift-left footgun (Dim PE-6 + PE-1) — Open

`DESIGN-VERIFICATION.md:254` + `:283` declare the canonical CI workflow templates run:

```yaml
- name: Install vsdd
  run: cargo install vsdd --locked
```

Two CI workflows ship by default: `vsdd-verify.yml` + `vsdd-observe-pr-body.yml`. Both compile vsdd from source on every workflow invocation. README:31 claims pre-built binaries are a "v1.0 ship-blocker (promoted from v1+)" via DESIGN-VERIFICATION:800-815, but the actual workflow templates `vsdd init` deploys (the spec-authored YAML) use `cargo install` from source — not the pre-built-binary `curl-pipe-tar-pipe-bin` pattern documented in the binary-distribution section.

The pre-built-binary path is described as a fallback at DESIGN-VERIFICATION:826 ("Fallback: `cargo install vsdd --locked`") — but the templates themselves use the fallback as the primary path.

**Why this matters (PE lens):**
- **PE dim 6 (build performance):** README:30 estimates `cargo install` from source at ~60s per CI job. With 2 default workflows × N jobs × every PR push = real CI cost. The pre-built-binary path is supposed to address exactly this — but the template doesn't use it.
- **PE dim 1 (reproducible builds):** `cargo install vsdd --locked` from crates.io is reproducible only as long as crates.io serves the same `Cargo.lock`-anchored source. Pre-built binaries from GitHub Releases would let CI consume signed cosigned-attested binaries — more reproducible AND faster.
- **PE dim 3 (supply-chain attestation):** the entire SLSA-provenance + cosign-signing infrastructure documented at DESIGN-VERIFICATION:803-808 only adds value if CI actually consumes the signed binaries. Templates that go through `cargo install` from source bypass the attestation surface that the v1.0-ship-blocker promotion exists to provide.

**Why this matters (SA lens):** SA dim 5 (hard-to-undo decisions named) — the CI workflow templates are what `vsdd init` writes to adopting projects. Once shipped, every adopter's `.github/workflows/vsdd-*.yml` carries this pattern. Changing later is a managed-section-update path with operator-confirmation friction.

**Routing:** Phase 4 → Phase 1a (DESIGN-VERIFICATION workflow templates revision). Two paths:
- (a) v1.0 templates use the pre-built-binary path with `cargo install` as documented fallback; gate v1.0 release on the pre-built-binary pipeline being functional (per the "v1.0 ship-blocker" framing).
- (b) v1.0 templates use `cargo install` per current state; pre-built-binary path documented as v1.1 deliverable; cosign + SLSA work re-routes to v1.1.

Suggest (a) — preserves the supply-chain-attestation surface that DESIGN-VERIFICATION already claims as ship-blocker. If the pre-built-binary pipeline isn't ready, the methodology should treat that as a v1.0 blocker rather than ship templates that bypass the attestation surface.

- **finding_id:** 1-f5
- **domain:** platform-engineer
- **dim:** PE-6 (build performance) + PE-1 (reproducible builds) + PE-3 (supply-chain attestation)
- **classification:** Open
- **routing:** target_phase: phase-1a; target_artifact: DESIGN-VERIFICATION.md workflow templates + binary-distribution coordination

---

### Finding 6 — GitHub Actions are pinned by major-version tag, not SHA (Dim PE-3 + PE-1) — Open

`DESIGN-VERIFICATION.md:252, 281, 293, 372` use GitHub Actions like:

```yaml
- uses: actions/checkout@v4
- uses: github/codeql-action/upload-sarif@v3
- uses: actions/github-script@v7
```

Major-version tags (`@v4`, `@v3`, `@v7`) are mutable — the maintainer can move the tag to a new commit without changing the version label. Per Platform Engineer dim 1 (reproducible builds: "Future-developer + CI-from-3-years-from-now must produce the same bytes") + PE dim 3 (supply-chain attestation: "Pre-built binaries signed... release infrastructure compromise scope bounded by attestation") — tag-pinning is the well-documented supply-chain attack surface (the Codecov bash-uploader compromise pattern; the tj-actions/changed-files Mar 2025 incident).

GitHub's own [supply-chain hardening guidance](https://docs.github.com/en/actions/security-guides/security-hardening-for-github-actions#using-third-party-actions) explicitly recommends pinning to the full commit SHA:

```yaml
- uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11   # v4.1.1
```

The methodology authors a dependency-approval discipline (DESIGN-VERIFICATION:619-674) requiring SO + PE + Security investigation for every new Cargo.toml entry — but the CI workflow templates that ship to every adopting project depend on 4+ GitHub Actions pinned by mutable tag with no equivalent investigation surface. The "Cargo.lock-pin-discipline-but-CI-action-tag-pin-laxity" asymmetry is structural.

**Why this matters (PE lens):** the same attacker who can't compromise vsdd via a Cargo.toml addition (because the dependency-approval hook blocks it) can compromise vsdd via an upstream GitHub Action that vsdd deploys to every adopting project's CI. The dependency-approval discipline is defense-in-depth that lives on one surface but not the other.

**Why this matters (DE lens):** Per Data Engineer dim 8 (append-only patterns) — the `.vsdd/events.jsonl` audit trail captures `HookFired` events from the CI runs. If a compromised action exfiltrates the event log or modifies it before commit, the append-only structural property is breached at the CI layer rather than the application layer.

**Routing:** Phase 4 → Phase 1a (DESIGN-VERIFICATION CI workflow template revision). Pin every `uses:` to a full commit SHA with the version label in a `# v4.1.1`-style comment. Optionally extend the dependency-approval hook (`check-dependency-approval.py`) to detect tag-pinned actions in `.github/workflows/vsdd-*.yml` + fire a candidate code like `VSDD-W0210: github-action-tag-pinned-not-sha` (earned-by-recurrence: this finding + any future PR that adds a tag-pinned action).

- **finding_id:** 1-f6
- **domain:** platform-engineer (primary) + data-engineer (composed)
- **dim:** PE-3 (supply-chain attestation) + PE-1 (reproducible builds)
- **classification:** Open
- **routing:** target_phase: phase-1a; target_artifact: DESIGN-VERIFICATION.md workflow templates

---

### Finding 7 — `.vsdd/events.jsonl` durability story conflates append-only-structural with git-tracked-recovery (Dim DE-3 + DE-8) — Open

The methodology asserts `.vsdd/events.jsonl` is both:

1. **Append-only by structural property** (methodology.md:295; DESIGN-OBSERVABILITY:152): "local NDJSON append-only file" — structural invariant enforced by the producer (collector + hook emission code).
2. **Git-tracked per cycle with `git checkout` recovery** (methodology.md:233; DESIGN-OBSERVABILITY:152): "disaster recovery via `git checkout`" — durability via committing the file at cycle close.

These are two different durability stories layered on the same file:

- **Story A (append-only structural):** events emitted between commits accumulate in the file; the file grows monotonically; the in-memory or producer-side discipline prevents in-place edits.
- **Story B (git-tracked recovery):** commits create restore points; `git checkout` returns to the prior commit's state — which would *truncate* events emitted between commits.

The OTel collector config at DESIGN-OBSERVABILITY:99-105 declares a rotation policy:

```yaml
file/events-jsonl:
  path: .vsdd/events.jsonl
  rotation:
    max_megabytes: 100
    max_days: 30
```

So the *third* durability story is the collector's own rotation — files older than 30 days OR larger than 100 MB get rotated. Rotated-out events are presumably moved to `.vsdd/events.jsonl.1` (or similar) — but the methodology doesn't document the rotation file naming, git-tracking of rotated files, or recovery from rotated state.

**The architecture gap (DE lens):** Per Data Engineer dim 3 (backup + recovery discipline: "Backups are taken at declared intervals + tested via restore-to-staging. RTO + RPO declared..."):
- The RPO (Recovery Point Objective — how much data loss is acceptable on recovery) for the event log is unstated.
- The RTO (Recovery Time Objective) is unstated.
- The rotation discipline interacts with the git-tracking discipline in a way the spec doesn't enumerate: a commit-then-rotate sequence loses pre-rotation events from the committed state; a rotate-then-commit sequence captures them but in a separate file.
- "Disaster recovery via `git checkout`" assumes the disaster is local-file-corruption-with-clean-git-history; it doesn't cover disaster scenarios like force-push-overwrites, git-objects-corruption, or accidental `.gitignore` of the event log file.

**Why this matters (DE lens):** Per DE sycophancy_failure_modes — "Backup strategy claimed but never exercised — recovery untested; data loss path silent." The recovery story is asserted without operational testing; the rotation + git-tracking interaction is silent.

**Why this matters (SA lens):** Per SA dim 5 (hard-to-undo decisions named) — the choice of "git is the backup store" is a hard-to-undo decision. Migrating to a different backup discipline later (S3? a managed log service?) requires retroactively rebuilding the audit-trail integrity guarantees.

**Routing:** Phase 4 → Phase 1a (DESIGN-OBSERVABILITY persistence-discipline revision). Needs:
- (a) Declared RTO + RPO for the event log per the toolkit's `safety-critical` axis (currently unstated for the toolkit itself).
- (b) Explicit rotation × git-tracking interaction documented: do rotated files get committed? Are they tracked separately? Does cycle-close commit also commit the rotation?
- (c) `vsdd verify recover` candidate subcommand (v1+) that exercises the recovery path against a corrupted-state fixture, proving the recovery story.
- (d) An explicit "the event log is best-effort durable, not safety-critical durable" framing if the toolkit's intent is below safety-critical.

- **finding_id:** 1-f7
- **domain:** data-engineer
- **dim:** DE-3 (backup + recovery discipline) + DE-8 (append-only patterns) + DE-5 (query workload characterization downstream)
- **classification:** Open
- **routing:** target_phase: phase-1a; target_artifact: DESIGN-OBSERVABILITY.md persistence + rotation + recovery sections; methodology.md forward-only-disciplines

---

### Finding 8 — Schema migration utility is foundational track 3h but `vsdd verify migrate` is v1+ candidate (Dim DE-2 + SA-5) — Open

DESIGN-SCHEMA declares two contradictory things about migration:

1. `DESIGN-SCHEMA.md:815` — track 3h "Author schema-versioning + migration utility (`vsdd-core/src/migration.rs`)" is in the **Foundational** column.
2. `DESIGN-SCHEMA.md:773` — "`vsdd verify migrate <class>` subcommand (v1+ candidate) walks existing artifacts of that class; reports drift; offers auto-fix patterns for trivial migrations"
3. `DESIGN-VERIFICATION.md:558` — `vsdd verify migrate <artifact-class>` is listed in the CLI surface as "(v1+ candidate)"
4. `DESIGN-VERIFICATION.md:877` — track 5p "`vsdd verify migrate` subcommand (schema-version migration)" is in the v1+ list

**The contradiction:** the *utility module* is foundational (must land in v1 per track 3h). The *subcommand surface* is v1+. Per Data Engineer dim 2 (migration path completeness: "Every breaking schema change has a migration path with a tested forward path + tested rollback"):

- If `vsdd-core/src/migration.rs` exists at v1 but `vsdd verify migrate` doesn't ship as a subcommand, there's no operator-facing path to invoke migration.
- The first breaking schema change post-v1 (which DESIGN-SCHEMA's per-class semver discipline says will happen — "Review entry schema can hit 2.0.0 while Domain prompt stays at 1.0.0") arrives without a tested migration command.
- Per DE sycophancy_failure_modes: "Schema migration path declared in spec but never tested — migration breaks when actually applied."

**The DE gap:** the spec asserts migration is forward-only and per-class semver-bumped, but the migration tooling that operators would invoke isn't a v1 deliverable. The methodology bets that no breaking schema change happens between v1.0 and v1+ — without explicit reasoning about why that's tractable.

**Why this matters (SA lens):** SA dim 5 (hard-to-undo decisions) — schema-versioning discipline is hard-to-undo. If v1 ships without a migration command, the first cycle that needs migration is the cycle that proves the migration story.

**Routing:** Phase 4 → Phase 1a (DESIGN-SCHEMA + DESIGN-VERIFICATION coordinate revision). Two resolution paths:
- (a) Promote `vsdd verify migrate` from v1+ to v1; track 5p moves to v1 deliverables; the per-class semver discipline is end-to-end testable at v1.
- (b) Defer the v1.0 ship to a v0.x phase where schemas are explicitly not stable; v1.0 is the schema-stability commitment AND the moment migration tooling ships; document the deferral.

Suggest (a) — the migration utility module already lands; promoting the CLI subcommand is incremental.

- **finding_id:** 1-f8
- **domain:** data-engineer (primary) + solution-architect (composed)
- **dim:** DE-2 (migration path completeness) + SA-5 (hard-to-undo decisions)
- **classification:** Open
- **routing:** target_phase: phase-1a; target_artifact: DESIGN-SCHEMA implementation order + DESIGN-VERIFICATION CLI surface

---

### Finding 9 — Per-feature axis `persists-managed-schema-data` excludes the toolkit itself from DE composition — but the toolkit clearly persists schema-bearing data (Dim DE-1 + SA-1) — Open

Per `README.md:692` + DESIGN-METHODOLOGY:370 the `persists-managed-schema-data: yes` axis activates Data Engineer. The toolkit's own `.vsdd/config.yaml` for this very repo would presumably declare per-feature axes; but the methodology doesn't show what the toolkit's own axis declarations are.

The toolkit clearly persists schema-bearing data:
- `.vsdd/events.jsonl` (NDJSON; per-event-variant payload schema — 18+ variants)
- `.vsdd/config.yaml` (vsdd-config artifact class schema)
- `.vsdd/init-manifest.json` (per-file SHA-256 hashes)
- `.vsdd/mcp-cache/` (TTL-bounded; some structural discipline)
- `.vsdd/registry/vocabulary.yaml` + `canonical-patterns.yaml` + `anonymization-patterns.yaml` (operator-extensible registries)
- 13 schema-validated artifact classes' instances across `review-log/`, `.claude/commands/`, `supplements/`, `docs/error-codes/`, `manual-tests/`, etc.

By any reasonable read of "persists managed schema data" — the toolkit does. So DE activates for the toolkit's own development. So the Architecture cluster (per DESIGN-METHODOLOGY:389) includes DE for vsdd-cli's own Phase 3 cycles — which is what produced this very review entry.

But: nowhere does the spec explicitly declare the toolkit's own axis manifest. The toolkit dogfoods on itself per README:122 ("The rebuild's own development as canonical dogfood") — but the axes declaration that drives composition for the rebuild's own cycles is unstated.

**Why this matters (SA lens):** Per SA dim 1 (decomposition coherence: "Each layer's acceptance criteria are independently buildable + verifiable") — Phase 3 cluster composition depends on axes; axes drive DE activation; DE activation determines whether Architecture cluster includes DE; DE inclusion determines whether persistence findings (like Finding 7 above) surface from a domain-with-the-lens. If the toolkit's own axes are unstated, the cluster composition for the toolkit's own reviews is unstated.

**Why this matters (DE lens):** Per DE dim 1 (schema discipline) — the methodology asks adopting projects to declare schema-data axes; the toolkit silently asserts the axis without declaring it. The dogfooding-on-itself framing claims the toolkit applies its own discipline; the axis-declaration discipline is the first surface where that's testable.

**Routing:** Phase 4 → Phase 1a (DESIGN-METHODOLOGY + README revision; the toolkit's own DESIGN.md is the missing artifact). Two resolution paths:
- (a) Author `DESIGN.md` for the toolkit itself at repo root (separate from DESIGN-METHODOLOGY which is the methodology subsystem design) declaring the toolkit's axes — `persists-managed-schema-data: yes`, `network-exposed: no` (the toolkit doesn't network; it composes against Claude Code which does), `handles-user-data: no` (no end-user data; operator-only), `safety-critical: no` (operator-tool, not life-critical), `formal-verification-candidates: yes` (for pure-function pure validators), `ui-surface: no` (CLI; no UI), `localized: no`, `ai-runtime-cost-relevant: yes` (the methodology is cost-relevant by design).
- (b) Add a section to DESIGN-METHODOLOGY explicitly enumerating the toolkit's own axes-as-applied-to-itself.

Suggest (a) — matches the DESIGN.md.vsdd-template's own structure; consistent with the methodology's adopting-project expectations.

- **finding_id:** 1-f9
- **domain:** data-engineer (primary) + solution-architect (composed)
- **dim:** DE-1 (schema discipline) + SA-1 (decomposition coherence)
- **classification:** Open
- **routing:** target_phase: phase-1a; target_artifact: new DESIGN.md at toolkit repo root (or new section in DESIGN-METHODOLOGY)

---

### Finding 10 — Python hook subprocess overhead "~50ms × 19 hooks × typical 3 files" is undeclared cumulative cost (Dim PE-6 + SA-7) — Open

`DESIGN-VERIFICATION.md:115` claims:

> Operator-local subprocess overhead: ~50ms per hook firing (negligible at typical commit-touches-3-files scale).

Per the dispatch design, each Python hook subprocess-shells to the Rust binary. With 19 methodology hooks deployed + the `git diff --name-only` scoping at DESIGN-VERIFICATION:564 ("hook chain runs only on files in `git diff --name-only` for the commit"), the worst-case overhead is:

- 19 hooks × 50ms = 950ms per commit minimum (single-file commit; every hook fires once per commit, not per file).
- For multi-file commits: per-file dispatch is a separate consideration; the spec doesn't clarify whether the hook fires once per commit or once per file in the chain.

950ms per commit is at the threshold where operator perception shifts from "instant" to "perceptible pause." If hook-firing scales per-file (19 hooks × 3 files × 50ms = 2850ms = ~3 seconds per commit), the operator-local experience degrades materially.

Per the DESIGN-VERIFICATION:888 deferred decision ("Python hook dispatch (subprocess to vsdd-core Rust binary vs pure-Python validation)") — the design is undecided between subprocess-to-Rust and pure-Python; the deferred decision punts the cost analysis.

**Why this matters (PE lens):** Per PE dim 6 (build performance) — `cargo install` from source ~60s budget is the v1.0-ship-blocker for the pre-built-binary path. Operator-local commit-time budget is the parallel-axis concern; un-budgeted hook overhead degrades the dogfooding-on-itself surface (every spec commit by the toolkit's own author runs the same 19 hooks).

**Why this matters (SA lens):** Per SA dim 7 (abstraction altitude: "neither too low nor too high") — the Python-thin-wrapper-to-Rust-binary pattern is a deliberate abstraction choice. Its load-bearing cost (subprocess startup × hook count) isn't measured against the alternative (pure-Python validators that ship with vsdd-core's schemas as static JSON files).

**Routing:** Phase 4 → Phase 1a (DESIGN-VERIFICATION hook architecture revision). Needs:
- (a) Explicit per-hook cost budget (e.g., "individual hook < 100ms wall-clock; full chain < 1s for typical commits") declared as a v1 acceptance criterion.
- (b) Pre-v1 benchmark fixture exercising the 19-hook chain against a representative commit set (e.g., the toolkit's own commit history); reports cumulative time.
- (c) Tiered execution: per-file hooks run only on the file types that fire them (frontmatter-schema fires only on `.md` files; check-changelog-discipline fires only when CHANGELOG.md changes; etc.) — partially declared at DESIGN-VERIFICATION:564 but not fully operationalized.

- **finding_id:** 1-f10
- **domain:** platform-engineer (primary) + solution-architect (composed)
- **dim:** PE-6 (build performance) + SA-7 (abstraction altitude)
- **classification:** Open
- **routing:** target_phase: phase-1a; target_artifact: DESIGN-VERIFICATION.md hook architecture section + acceptance criteria

---

### Finding 11 — `vsdd-core` crate vs `vsdd` crate split contradicts "single-crate-single-binary" framing (Dim SA-2 + SA-7) — Open

The spec repeatedly asserts a "one crate, one binary" decomposition:

- `README.md:212` — "Single `vsdd` crate with a single `vsdd` binary that dispatches subcommands (matching cargo / rustup / git ecosystem convention)"
- `README.md:215` — "`vsdd` Rust binary (subcommands: `init`, `verify`, `observe`, `mcp-serve`)"
- `DESIGN-OBSERVABILITY.md:418` — "(preserves single-crate-single-binary workspace)"

But DESIGN-VERIFICATION's workspace layout (`DESIGN-VERIFICATION.md:710-762`) shows a **two-crate cargo workspace**:

```
vsdd-cli/
├── Cargo.toml                          # workspace root
├── vsdd-core/                          # crate 1 (library)
│   └── ...
├── vsdd/                               # crate 2 (binary)
│   └── ...
```

with `Cargo.toml` `[workspace] members = ["vsdd-core", "vsdd"]`.

**The architectural reality:** the toolkit ships *one binary* (`vsdd`) but is composed of *two cargo crates* (`vsdd-core` + `vsdd`). The "single-crate-single-binary" framing is inaccurate — it's a "two-crate-workspace-with-one-binary" pattern. Per Solution Architect dim 2 (architectural seam clarity: "Function signatures + module boundaries + type contracts make the seam visible") — the `vsdd-core` / `vsdd` seam is invisible at the README + methodology layer; only DESIGN-VERIFICATION surfaces it.

**Why this matters (SA lens):** Per SA dim 7 (abstraction altitude) — the choice between single-crate-with-modules vs two-crate-workspace has real consequences:
- **Two-crate-workspace pros:** clean separation between schema/event/error-catalog types (vsdd-core) and CLI dispatch + business logic (vsdd); enables vsdd-core to be reused (e.g., by an LSP binary or external Rust consumers) without depending on clap or the binary's CLI surface.
- **Two-crate-workspace cons:** more cargo build configuration; per-crate dependency surface; downstream consumers of vsdd-core (if any) require independent crates.io publishing or workspace-internal-only usage.

The choice is reasonable but the framing isn't accurate. A future maintainer reading the README sees "single crate" and is surprised by the workspace.

**Why this matters (PE lens):** Per PE dim 2 (dependency approval discipline) — the dependency-approval hook scope is per-Cargo.toml; with two crates, both `vsdd-core/Cargo.toml` and `vsdd/Cargo.toml` are dependency-manifest files. The hook needs to scope to both; DESIGN-VERIFICATION:660 only mentions `Cargo.toml` (singular).

**Routing:** Phase 4 → Phase 1a (README + methodology framing revision; DESIGN-VERIFICATION dependency-approval hook scope clarification). Two paths:
- (a) Reframe consistently as "two-crate workspace publishing one binary on crates.io" everywhere it's mentioned; preserve the workspace structure.
- (b) Collapse to actually single-crate (with `lib.rs` modules instead of separate vsdd-core crate); update DESIGN-VERIFICATION workspace layout.

Suggest (a) — the workspace structure is sound; the framing is what's drifted.

- **finding_id:** 1-f11
- **domain:** solution-architect (primary) + platform-engineer (composed)
- **dim:** SA-2 (architectural seam clarity) + SA-7 (abstraction altitude) + PE-2 (dependency approval discipline)
- **classification:** Open
- **routing:** target_phase: phase-1a; target_artifact: README.md:212 + DESIGN-OBSERVABILITY.md:418 + DESIGN-VERIFICATION dependency-approval scope

---

### Finding 12 — Operator-extensible `.vsdd/registry/anonymization-patterns.yaml` is a trust-boundary inversion (Dim PE-3 + SA-4) — Open

The redaction processor at DESIGN-OBSERVABILITY:91-96 sources its patterns from operator-extensible config:

```yaml
redaction:
  config_source: .vsdd/registry/anonymization-patterns.yaml
  # The registry file declares api_key_patterns + bearer_token_patterns + credential_attribute_names
  # Operator extends by editing the registry file; collector reloads on next cycle start
```

DESIGN-VERIFICATION:346 confirms: "Operator-extensible at `.vsdd/registry/anonymization-patterns.yaml` (deployed by vsdd init; operator adds project-specific patterns)."

DESIGN-VERIFICATION:62 explicitly contrasts this with schema-injection-defense: "Operator-extension paths (`.vsdd/registry/`) exist for operator-configurable patterns (anonymization patterns; canonical-vocabulary registry); schemas themselves are not operator-extensible."

**The trust-boundary problem (SA + PE lens):**

- The anonymization patterns determine what gets redacted *before forwarding to external backends*. An operator (or an attacker who can write to `.vsdd/registry/`) can *narrow* the redaction pattern set — removing a pattern means credentials matching that pattern flow through to the external backend (Honeycomb / Datadog / Grafana / etc.).
- The schema-injection defense (canonical-schema-path discipline) explicitly distinguishes "schemas are NOT operator-extensible" — for exactly the same reason: PR-modifiable validation rules are the attack surface.
- But the anonymization patterns are operator-extensible. The same attacker model that justifies non-extensible schemas also justifies non-extensible-but-additive-only anonymization patterns. The current spec permits operators to *delete* patterns, not just add them.

**Why this matters (PE lens):** Per PE dim 3 (supply-chain attestation: "release infrastructure compromise scope bounded by attestation") — the redaction-pattern config is part of the trust boundary at the OTel collector forwarding step. A compromised `.vsdd/registry/anonymization-patterns.yaml` exfiltrates credentials by allowlist-reduction.

**Why this matters (SA lens):** Per SA dim 4 (trust boundary placement: "Where does input from outside the process enter? Each entry point is named in DESIGN.md + has Phase 5 Fuzz Testing scope. Untrusted-input-treated-as-trusted is the load-bearing security failure mode") — the anonymization-patterns file is an entry point. It's operator-controlled but PR-modifiable; that's an attack surface that should be in Phase 5 Fuzz Testing scope.

**Routing:** Phase 4 → Phase 1a (DESIGN-OBSERVABILITY + DESIGN-VERIFICATION coordinate revision). Two resolution paths:
- (a) Operator-extensible anonymization patterns are **additive-only** — the toolkit's canonical pattern set (bundled in the Rust binary; not file-loadable) is always applied; operator-extensions extend but cannot subtract. Add a hook (`check-anonymization-pattern-completeness.py`) verifying the canonical patterns are still present.
- (b) Anonymization patterns are NOT operator-extensible at all — pattern updates require a toolkit-version-bump; operators receive new patterns via `cargo install vsdd --force`. Removes the attack surface entirely but reduces per-project flexibility for site-specific credential formats (e.g., a company-internal API key prefix).

Suggest (a) — preserves operator flexibility while closing the subtraction attack surface. Bundle with Finding 6 — both are trust-boundary findings in the verification subsystem.

- **finding_id:** 1-f12
- **domain:** platform-engineer (primary) + solution-architect (composed)
- **dim:** PE-3 (supply-chain attestation) + SA-4 (trust boundary placement)
- **classification:** Open
- **routing:** target_phase: phase-1a; target_artifact: DESIGN-OBSERVABILITY.md anonymization-patterns extension model + DESIGN-VERIFICATION pattern-completeness hook

---

### Finding 13 — Phase-domain matrix declares Phase 5 hard-coded to QE + Security + SA but Phase 5 round 1 already composed Data Engineer (Dim SA-1 + SA-2) — Accepted

Per `DESIGN-METHODOLOGY.md:139` + README:644, Phase 5 composes:

> | 5 | QE + Security + SA |

Phase 5 round 1 (this very repo, the reviews I'm reading as priors) explicitly composed SA + QE + Security per `review-log/2026-05-27-solution-architect.md` + `2026-05-27-security.md` + `2026-05-27-quality-engineer.md`. The cluster-batched composition was per operator-directive 2026-05-27 — three domain reviewers running inline.

But: the toolkit persists schema-bearing data (Finding 9 above asserts DE should be active). DE was not composed for Phase 5 round 1. The omission is consistent with the hard-coded Phase 5 composition matrix — but inconsistent with the per-feature-axes-additivity discipline Phase 5 round 1 Security F1 + QE F4 *both* surfaced.

This is the same defect class Security F1 + QE F4 identified: phase-domain composition has two mechanisms (phase-fixed + axes-additive); they're not unified; Phase 5 ignores axes. If DE is axis-activated, it should compose with Phase 5 for projects with persists-managed-schema-data: yes.

**Why this is Accepted not Open:** Phase 5 round 1 Security F1 + QE F4 already routed this concern to operator-directive for the always-on-baseline decision. The current spec landed the always-on-baseline (DESIGN-METHODOLOGY:340-358) but the Phase 5 axes-additivity question is implicit in that fix. The Architecture cluster's contribution: confirm the question landed at the right routing target; surface the specific case (DE × Phase 5) for the operator-directive scope.

**Rationale for Accepted:** the finding *is* substantive but it's already in the operator-directive routing queue from Phase 5 round 1. Re-raising at Phase 3 doesn't add information; it confirms the prior routing.

**Why this matters (SA lens):** Per SA dim 1 (decomposition coherence) — phase-domain composition is the load-bearing decomposition mechanism. The hard-coded-vs-axes-additive split is the consistency gap. Surfaced for the record + acknowledgment of cross-cycle continuity.

- **finding_id:** 1-f13
- **domain:** solution-architect
- **dim:** SA-1 (decomposition coherence) + SA-2 (architectural seam clarity)
- **classification:** Accepted (already routed via Phase 5 round 1 Security F1 + QE F4 to operator-directive; surfaced for continuity)
- **routing:** None — confirms prior routing

---

## Summary

13 findings — 12 Open + 1 Accepted. The Tier 1 spec artifacts are *substantially coherent* after Phase 5 round 1's sweep landed (auth × CI cross-field validation, always-on baseline, methodology-version-pin); the Architecture cluster's contribution is finding the *next layer* of drifts:

**By lens:**
- **Solution Architect (decomposition + seams + hard-to-undo decisions):** Findings 1, 2, 3, 4, 8, 9, 11, 13 — primarily cross-source-consistency drift surviving Phase 5's sweep, plus the toolkit's own-axes-undeclared gap and the two-crate-vs-single-crate framing.
- **Platform Engineer (reproducibility + supply-chain + CI):** Findings 5, 6, 10, 12 — the CI workflow templates' actual-vs-aspirational gap (`cargo install` vs pre-built binaries), GitHub Actions tag-pinning vs SHA-pinning, hook-chain cost budget, anonymization-patterns trust-boundary inversion.
- **Data Engineer (schema + migration + persistence + recovery):** Findings 1, 2, 7, 8, 9 — `ProjectInitialized` field name drift, validation-events-without-payload-schemas, event-log durability story, migration tooling deferral, axis-declaration gap.

**By classification:**
- 12 Open (route to Phase 4 → Phase 1a for spec revisions; mostly bounded mechanical fixes; some operator-directive substantive amendments)
- 1 Accepted (cross-cycle continuity acknowledgment; no new action)
- 0 Deferred, Dismissed, or Hallucinated — the spec held against this round in the sense that nothing surfaced was clearly out-of-scope or based on a misreading.

**Cluster routing buckets:**

- **Bounded mechanical fixes** (route to Phase 4 → Phase 1a; bundle in this cycle's spec-revision commit): F1 (toolkit-version vs suite-version), F3 (stale MCP class reference), F4 (~18 vs ~19 hooks in README opening), F11 (two-crate workspace framing).
- **Architecture spec amendments** (require coordinate revision across docs): F2 (validation events + payload schemas), F5 (CI templates use pre-built binaries), F6 (GitHub Actions SHA-pinning), F7 (event-log durability story), F8 (migration tooling promote to v1), F10 (hook-chain cost budget), F12 (anonymization-patterns additive-only).
- **Operator-directive routing** (require methodology amendment): F9 (toolkit's own axes declaration) — also surfaces a DESIGN.md authoring need for the toolkit itself.
- **Accepted-with-cross-cycle-continuity:** F13.

Per primer 3 "Continue if any active domain produced real findings": this round produced 12 Open findings; the cycle continues. The "implementation-MVR-reached" exit signal is not yet earned.

## Coordination

- **Implementation cluster (SE + QE + Performance Engineer) coordination:**
  - F8 (migration tooling) routes to QE for the regression-suite implications (per-class migration testing).
  - F10 (hook-chain cost budget) routes to Performance Engineer for the benchmark-fixture design.
  - F2 (validation event payload schemas) routes to QE for falsifiability fixtures (`manual-tests/error-catalog/HookFired/{should-fire,should-not-fire}/` pattern).

- **Communication cluster (Security + TW + Accessibility + Privacy + Localization) coordination:**
  - F6 (GitHub Actions SHA-pinning) is primarily PE but has clear Security overlap — Communication cluster's Security may surface the same finding from the threat-model angle.
  - F12 (anonymization-patterns trust-boundary) is PE-primary but Security-overlap; Communication cluster's Security may extend with attack-scenario enumeration.
  - F1 (field name drift) is TW-relevant — the rebuild's "naming + coinage governance" discipline (suite → toolkit) is exactly the kind of vocabulary-registry concern TW + DR own.

- **Adversarial cluster (Red Team + DR + UX + AI Engineer + Solution Owner + VSDD Methodology + Sanity Check) coordination:**
  - F9 (toolkit's own axes declaration) routes to SO via the meta-domain — declaring axes is a spec-contract-change-class decision per "Raise to SO" discipline.
  - F5 (CI templates) routes to SO for the v1.0-ship-blocker scope question; the binary-distribution pipeline readiness is operator-directive.
  - F13 (Accepted; Phase 5 round 1 continuity) routes to VSDD Methodology meta-domain for cross-session semantic continuity audit.

- **Sycophancy resistance check:** I specifically did *not* surface findings against the Phase 5 round 1 fixes themselves (auth × CI cross-field validation; always-on baseline; methodology-version-pin) — those landed; re-raising them would be reviewer-loop-without-new-evidence. The findings here are net-new from the Architecture cluster's lens, not duplications of prior findings under different cluster-naming.

The Tier 1 spec set is approaching coherence. The remaining gaps are mostly the "second sweep after the first sweep" pattern — fixes that landed in Phase 5 round 1 cleared the load-bearing structural issues; what's left is finer-grained consistency, supply-chain hardening, persistence-discipline tightening, and the toolkit-meta-applies-to-itself axis declaration. None of the findings here block forward motion; all are routable.
