---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-05-27
phase: phase-3
scope: Implementation cluster (SE + QE + PerfE) on Tier 1 spec artifacts — testability, falsifiability, implementation seams, performance characteristics
lens: Implementation cluster — SE + QE + PerfE lenses applied to Tier 1 spec artifacts (methodology.md + 5 DESIGN docs + 10 primers + 18 domain prompts)
source: domain-raised
session_note: cold-context — first Phase 3 IAR round on the spec set; Implementation cluster composed via 3-domain skill-mode aggregation in cold-context Agent spawn
model: claude-opus-4-7
execution_method: Agent-tool subagent spawn (cold-context approximation; cluster-batched per primer 3 default)
sycophancy_compensation: The artifacts under review were authored by another instance of claude-opus-4-7 in the same conversational session that dispatched this cluster. I share substrate with the author. The Phase 5 round-1 inline reviews (QE, SA, Security) already harvested cross-doc count-drift + zero-axes + assumption-violation findings; the bias from inside this session is to treat those rounds as having caught the main surface and assume the residue is hallucinated. I specifically resisted: (a) accepting the "validator < 1s wall-clock" claim as a bounded performance contract when no workload is named; (b) accepting the "v1+ deferred" stamps on falsifiability surfaces (LSP, migrate, usage-api-reconcile) as legitimate scope-bounding when those deferrals mask testability gaps for the v1 ship-state; (c) accepting that "Goal 2 makes machine-enforceable mechanically true" without auditing whether each consolidated multi-rule hook is actually falsifiable per-rule rather than per-hook; (d) accepting the Python-thin-wrapper-to-Rust-subprocess dispatch pattern as cost-neutral when the spec itself names ~50ms subprocess overhead per hook firing.
---

# Implementation Cluster Review 1 — 2026-05-27

**Phase 3 cluster:** Implementation (Software Engineer + Quality Engineer + Performance Engineer composed via cold-context Agent spawn)
**Cluster-batching shape:** per primer 3 default — 3-domain aggregation in one cold-session sub-agent
**Lens applied:** testability + falsifiability + implementation seams + performance characteristics of the spec

## Scope

Tier 1 artifacts under review: `methodology.md` (415 lines) + `README.md` (886 lines) + `DESIGN-METHODOLOGY.md` (1007 lines) + `DESIGN-SCHEMA.md` (842 lines) + `DESIGN-OBSERVABILITY.md` (614 lines) + `DESIGN-VERIFICATION.md` (903 lines) + 10 phase primers + 18 domain prompts. Spec-stage — no Rust code yet committed; the cluster reviews whether the spec carries enough detail for Phase 2a Red Gate tests + Phase 2b minimal implementation + performance-contract verification.

## Findings

### Finding 1 — VSDD-E0021 declared twice with conflicting summaries (Dim: SE-1 spec-implementation alignment + QE-2 acceptance-criteria coverage) — Open

```yaml
finding_id: 1-f1
domain: software-engineer
dim: 1
owner: software-engineer
status: open
validator: sanity-check
classification: open
source: domain-raised
routing:
  target_phase: phase-1a
  target_artifact: README.md
  target_section: error-catalog-accepted
dismissal_rationale: null
```

`README.md:502` (Accepted codes table) declares:

> `VSDD-E0021: auth-method-plan-incompatible-with-ci` (Plan auth declared for CI; structurally invalid …)

`README.md:509` (Candidate codes table) declares:

> `VSDD-E0011: unverified-citation`, `VSDD-E0012: missing-source-attribution`, `VSDD-E0013: validator-pair-mismatch`, `VSDD-E0021: findings-registry-orphan-row`

Same code (`VSDD-E0021`) declared with two different summaries, different status tiers (Accepted vs Candidate), different rules. `DESIGN-SCHEMA.md:388,396` consistently uses `VSDD-E0021: auth-method-plan-incompatible-with-ci`; the Candidate-table usage at README:509 is the drift.

**Why this matters (SE lens):** the error-catalog is a single keyed registry — every code resolves to exactly one summary + detail + help via `vsdd verify explain <code>`. The forward-only governance the spec asserts (`DESIGN-SCHEMA.md:744-750`: "Codes never reused once retired") presumes uniqueness from coinage onward; a double-declaration at coinage time means the Rust enum `VsddErrorCode::E0021` cannot deserialize cleanly — `serde` chokes on duplicate-key registry, and either YAML load drops the second or the implementer arbitrarily picks one. Either way, one of the rules silently becomes unenforceable.

**Why this matters (QE lens):** falsifiability gap. Per-code fixture pairs at `manual-tests/error-catalog/VSDD-E0021/{should-fire,should-not-fire}/` would have to satisfy both summaries simultaneously, which is impossible — the should-fire fixture for `auth-method-plan-incompatible-with-ci` is a `.vsdd/config.yaml` with `auth_method.ci: plan`; the should-fire fixture for `findings-registry-orphan-row` is a findings-registry with an orphan row. These are disjoint inputs. The `vsdd verify test-error-catalog` regression suite cannot pass against both.

**Corrective pattern:** assign one of the two rules a different unused code. Suggest: keep `VSDD-E0021: auth-method-plan-incompatible-with-ci` (the more recently coined; appears in Phase 5 Security F4 evidence base) at Accepted status; rename the candidate to `VSDD-E0022: findings-registry-orphan-row` (or move it to a different unused number in the E0001-E0099 DESIGN-SCHEMA range). Update README.md:509 + the corresponding spot in DESIGN-VERIFICATION.md if any. The error-catalog file format (`DESIGN-SCHEMA.md:687-722`) needs a `check-error-catalog-uniqueness.py` hook (candidate) that fires at every error-catalog.yaml commit to prevent this class of double-coinage.

**Routing:** Phase 4 → Phase 1a (README revision; possibly DESIGN-SCHEMA addition for the uniqueness hook).

### Finding 2 — VSDD-E0220 declared with conflicting semantics across two hook contexts (Dim: SE-3 error handling specificity) — Open

```yaml
finding_id: 1-f2
domain: software-engineer
dim: 3
owner: software-engineer
status: open
validator: sanity-check
classification: open
source: domain-raised
routing:
  target_phase: phase-1a
  target_artifact: DESIGN-VERIFICATION.md
  target_section: per-hook-deployment-matrix
dismissal_rationale: null
```

`README.md:138` and `README.md:201` declare `VSDD-E0220: existing-file-malformed-refuse-to-overwrite` — fired by `vsdd init` pre-flight when `.mcp.json` / `.claude/settings.json` / `.pre-commit-config.yaml` / `.vsdd/init-manifest.json` is malformed.

`DESIGN-VERIFICATION.md:182` (per-hook deployment matrix, row 5) declares:

> | 5 | check-anonymization | Pre-commit on all committed text files | $HOME / git user.name / git user.email / API-key formats … | **VSDD-E0220 (existing-file-malformed-refuse-to-overwrite)**, redaction patterns | Both |

The check-anonymization hook fires `VSDD-E0220` for credential-shaped content detection — but the code's summary says `existing-file-malformed-refuse-to-overwrite`, which is a `vsdd init` pre-flight concern with nothing to do with credential redaction.

**Why this matters (SE lens):** when CI reports `VSDD-E0220 fired`, the operator reads the explain page and gets the init-pre-flight summary. The actual cause (credential-shaped string detected in a committed file) is the opposite of the documented meaning. This isn't a doc-typo — it's a documented mismatch between the hook table's "what fires this code" and the catalog's "what this code means."

**Why this matters (QE lens):** the per-error-code fixture pair at `manual-tests/error-catalog/VSDD-E0220/should-fire/` per `DESIGN-VERIFICATION.md:499-510` can only test one of the two semantics. A fixture exercising the init-pre-flight path doesn't exercise the anonymization-hook path; the regression suite has a false-coverage gap.

**Corrective pattern:** check-anonymization fires a distinct code (suggest `VSDD-E0230: credential-shaped-content-detected` in the unused E0200-E0299 range). Update DESIGN-VERIFICATION.md:182 row 5 + the error catalog stub. Alternatively, the check-anonymization hook fires the per-pattern code from the anonymization-patterns registry (`.vsdd/registry/anonymization-patterns.yaml` per DESIGN-VERIFICATION.md:333-344); declare that explicitly.

**Routing:** Phase 4 → Phase 1a (DESIGN-VERIFICATION row 5 revision + error catalog code addition).

### Finding 3 — Validator wall-clock budget under-specified: per-validator <1s declared, hook-chain aggregate undeclared (Dim: PerfE-1 performance-characteristic declaration + PerfE-2 workload characterization) — Open

```yaml
finding_id: 1-f3
domain: performance-engineer
dim: 1
owner: performance-engineer
status: open
validator: solution-architect
classification: open
source: domain-raised
routing:
  target_phase: phase-1a
  target_artifact: DESIGN-METHODOLOGY.md
  target_section: cost-discipline
dismissal_rationale: null
```

`DESIGN-METHODOLOGY.md:690` declares:

> Per AI Engineer dim 9: each validator < 1s wall-clock for typical artifact size. Selective execution (only validate files in `git diff --name-only`). Cumulative cost across the hook chain tractable for ~50-file commits.

The per-validator < 1s is workload-bounded ("typical artifact size") but the typical artifact size is not named — is it the methodology.md 415 lines? a 50-line frontmatter block? a 5MB CHANGELOG accumulating two years of releases? Per-validator latency depends on artifact-size distribution, and the spec gives no anchor for "typical."

The aggregate is even worse: "tractable for ~50-file commits" is not a latency budget. 19 hooks × 50 files × <1s each ≤ 950s = 15.8 minutes worst case. That's not tractable for pre-commit (operators don't wait 15 minutes between `git commit` and the prompt returning).

**Why this matters (PerfE lens):** the spec asserts Goal 2 ("machine-enforceable") + Goal 4 ("Shift VSDD left into CI/CD"). Both depend on the hook chain being fast enough that operator-local commits + CI pre-merge gates run in reasonable time. Without a declared aggregate budget + workload-characterization, an implementer cannot author Phase 2a Red Gate tests asserting "the hook chain completes within X seconds for an N-file commit at artifact-size distribution Y." The performance contract has no falsifiable shape.

**Why this matters (QE lens):** Phase 5 surface B Mutation Testing readiness is bound to a measurable contract. If a mutation replaces a hook's fast-path with `sleep(5)`, no test catches it because no per-hook-chain latency test exists.

**Corrective pattern:** DESIGN-METHODOLOGY's cost-discipline section (and/or DESIGN-OBSERVABILITY's cost characteristics table at lines 192-204) declares per-workload latency budgets — e.g.:

| Workload | Aggregate hook-chain budget (P95) |
|---|---|
| Pre-commit on 1-file change, typical artifact size (avg 200 lines) | <2s |
| Pre-commit on 10-file change | <10s |
| CI pre-merge on 50-file PR | <60s |

Plus the criterion benchmark suite (rust supplement names `criterion`) gets a track in DESIGN-VERIFICATION's implementation order, so the regression-against-baseline discipline (PerfE Dim 8) has a target.

**Routing:** Phase 4 → Phase 1a (DESIGN-METHODOLOGY cost-discipline section + DESIGN-OBSERVABILITY cost-characteristics table revision; possibly DESIGN-VERIFICATION implementation-order addition for criterion benchmarks).

### Finding 4 — Python-subprocess-to-Rust-binary dispatch costs ~50ms × 19 hooks = ~950ms per commit, contradicts <1s per-validator claim (Dim: PerfE-3 hot-path identification + SE-4 API ergonomics) — Open

```yaml
finding_id: 1-f4
domain: performance-engineer
dim: 3
owner: performance-engineer
status: open
validator: solution-architect
classification: open
source: domain-raised
routing:
  target_phase: phase-1a
  target_artifact: DESIGN-VERIFICATION.md
  target_section: one-source-two-enforcement-surfaces
dismissal_rationale: null
```

`DESIGN-VERIFICATION.md:115` declares:

> Operator-local subprocess overhead: ~50ms per hook firing (negligible at typical commit-touches-3-files scale).

The "3-files scale" caveat is hidden. With 19 hooks (`DESIGN-VERIFICATION.md:170` declares ~19), every commit pays 19 × 50ms = 950ms of subprocess overhead alone — regardless of how many files actually need validation. Selective execution (`DESIGN-VERIFICATION.md:564`: "hook chain runs only on files in `git diff --name-only`") reduces per-file work but does NOT reduce hook-spawn count; the Python thin wrapper still subprocess-spawns the Rust binary for every hook ID, even when the diff is empty for that hook's file pattern.

Worse: 950ms of fixed overhead saturates the `< 1s wall-clock per validator` budget before any actual validation occurs. The validator-budget claim is structurally incompatible with the dispatch architecture.

**Why this matters (PerfE lens):** hot path is hook dispatch, not hook logic. The spec optimizes the wrong loop. Two paths to a faster shape: (a) a single Python entry point (`.claude/hooks/vsdd-dispatch.py`) that subprocess-spawns the Rust binary ONCE with all hook IDs as arguments, amortizing the 50ms; (b) a long-running `vsdd verify daemon` that operator-local commits IPC into (similar to `rust-analyzer`'s daemon pattern). Option (a) is the minimum viable; option (b) is v1+.

**Why this matters (SE lens):** the operator-local hook architecture as declared is the developer-tax surface — every commit slows by ~1s minimum. Operators commit dozens of times per day. The architecture invites operator-side hook-bypass-marker abuse to avoid the friction; that erodes the Goal 2 enforcement floor.

**Why this matters (QE lens):** falsifiability of the "no drift between operator-local + CI" claim depends on both surfaces running the same Rust binary. The 50ms subprocess pattern preserves that property; the corrective pattern (a) preserves it identically. Don't compromise the "one source; two enforcement surfaces" invariant for the speed-up.

**Corrective pattern:** DESIGN-VERIFICATION revises the dispatch shape — declare the single-Python-entry-point + multi-hook-ID args pattern. Update the per-hook table to note that operator-local hooks share dispatch (not 1:1 with `.claude/hooks/vsdd-*.py` files). Re-validate the wall-clock budget against the revised architecture.

**Routing:** Phase 4 → Phase 1a (DESIGN-VERIFICATION dispatch-pattern revision).

### Finding 5 — Manual-test class `falsifiability_check` field is a free-text string with no schema constraint (Dim: QE-1 test falsifiability + QE-4 manual-test preamble completeness) — Open

```yaml
finding_id: 1-f5
domain: quality-engineer
dim: 1
owner: quality-engineer
status: open
validator: software-engineer
classification: open
source: domain-raised
routing:
  target_phase: phase-1a
  target_artifact: DESIGN-SCHEMA.md
  target_section: manual-test
dismissal_rationale: null
```

`DESIGN-SCHEMA.md:470-479` declares the Manual-test class frontmatter shape:

```yaml
falsifiability_check: <string>   # how do we know this test catches what it claims?
```

The field type is `<string>` — any non-empty value passes the schema. The spec's QE Dim 1 (test falsifiability) is the load-bearing methodology floor: every test answers "what would have to be true of the implementation for this test to fail?" But the structural representation of that answer in the Manual-test class is unconstrained prose. "yes" passes. "TBD" passes. "see DESIGN.md" passes. A future implementer authoring `manual-tests/layer-N.md` files can satisfy the schema by typing literally anything in the falsifiability_check field.

**Why this matters (QE lens):** the methodology's sycophancy-failure-mode #4 (QE: "Manual-test checkbox without specificity — 'verify it works' with no observable outcome stated") fires `VSDD-W0080` per `README.md:496`. But the higher-leverage failure mode — the falsifiability claim itself being vacuous — has no hook. The mutation-testing analog: an adversarial author can write a manual test that purports to falsifiably check something, satisfy the schema, and ship; the hook never catches it.

**Why this matters (SE lens):** when Phase 2a Red Gate authoring consumes the manual-test as a behavioral-contract anchor, the SE reads the `falsifiability_check` to decide what the test asserts. Vague text leads to vague tests.

**Corrective pattern:** strengthen the schema. The `falsifiability_check` field declares two sub-fields:

```yaml
falsifiability_check:
  stub_behavior_under_test: <string>      # what behavior would the implementation have to violate for this test to fail?
  observable_failure_signal: <string>     # what does the operator/CI see when this test fails?
```

Both sub-fields required, both non-empty. Plus a hook (candidate, since this is a single-recurrence shape — earned-by-recurrence governance applies) like `check-falsifiability-specificity.py` that pattern-matches vague filler ("TBD", "yes", "see X", <5 words) and fires `VSDD-W0081: falsifiability-check-vague`.

**Routing:** Phase 4 → Phase 1a (DESIGN-SCHEMA Manual-test class revision; candidate hook + code addition in DESIGN-VERIFICATION).

### Finding 6 — Consolidated multi-rule hooks lose per-rule falsifiability when fixtures are per-hook not per-rule (Dim: QE-1 test falsifiability + QE-6 mutation testing readiness) — Open

```yaml
finding_id: 1-f6
domain: quality-engineer
dim: 1
owner: quality-engineer
status: open
validator: software-engineer
classification: open
source: domain-raised
routing:
  target_phase: phase-1a
  target_artifact: DESIGN-VERIFICATION.md
  target_section: per-error-code-falsifiability-fixtures
dismissal_rationale: null
```

`DESIGN-VERIFICATION.md:494-526` declares per-error-code fixture pairs:

```
manual-tests/error-catalog/VSDD-E0040/{should-fire,should-not-fire}/
```

For consolidated multi-rule hooks (`check-naming-discipline` = 4 rules; `check-phase-transitions` = 9-transition matrix; `check-changelog-discipline` = 10 rules), the fixture organization is **per-code**, not per-rule. But `check-naming-discipline` fires `VSDD-W0001` for FOUR different rules (per `DESIGN-VERIFICATION.md:181`):

> letter-label anti-pattern + vocabulary registry compliance + suite-internal terminology + first-use expansion … VSDD-W0001 (vestigial / deprecated-alias / suite-internal-terminology / missing-first-use-expansion — multi-rule shared)

A `should-fire` fixture for `VSDD-W0001` exercises ONE of those four rules. The regression suite passing on `VSDD-W0001` doesn't prove all four sub-rules are exercised. A mutation that disables the first-use-expansion sub-rule survives because the W0001 fixture probably exercises the letter-label sub-rule.

**Why this matters (QE lens):** mutation-testing readiness gap. Per Phase 5 surface B, a surviving mutant signals a test-suite blind spot. The consolidated-hook + per-code-fixture architecture creates surviving-mutants-by-construction for any sub-rule the fixture doesn't exercise.

**Why this matters (SE lens):** when an SE refactors the consolidated hook (e.g., adds a 5th rule to `check-naming-discipline`), the regression suite can't tell whether the refactor preserved the existing 4 rules' semantics. The fixtures don't enforce per-rule contracts.

**Corrective pattern:** fixture organization extends to per-sub-rule:

```
manual-tests/error-catalog/VSDD-W0001/
├── README.md
├── rule-letter-label-anti-pattern/
│   ├── should-fire/
│   └── should-not-fire/
├── rule-deprecated-alias/
│   ├── should-fire/
│   └── should-not-fire/
├── rule-suite-internal-terminology/
│   └── ...
└── rule-first-use-expansion/
    └── ...
```

The `vsdd verify test-error-catalog` regression suite walks the per-sub-rule directories and asserts each rule's fixtures pass. Per-rule disposition becomes the audit signal alongside per-code disposition.

**Routing:** Phase 4 → Phase 1a (DESIGN-VERIFICATION § Per-error-code falsifiability fixtures revision).

### Finding 7 — Property-based testing surface under-named: no pure functions catalogued in DESIGN-VERIFICATION (Dim: QE-7 property-based testing + SE-4 purity boundary) — Open

```yaml
finding_id: 1-f7
domain: quality-engineer
dim: 7
owner: quality-engineer
status: open
validator: solution-architect
classification: open
source: domain-raised
routing:
  target_phase: phase-1a
  target_artifact: DESIGN-VERIFICATION.md
  target_section: validator-architecture
dismissal_rationale: null
```

The QE domain prompt (`vsdd-domain-quality-engineer.md:30`) declares Phase 5 surface A (property-based testing) is bounded by:

> which functions have invariants the spec asserts that should hold across input ranges?

The SE domain prompt (`vsdd-domain-software-engineer.md:27`) declares purity boundary preservation:

> Functions DESIGN.md § Verification architecture declares pure stay pure in implementation.

But DESIGN-VERIFICATION.md — which IS the verification architecture for vsdd-cli itself — does not catalog any pure functions. The four utility modules at `vsdd-core/src/{anchor.rs, bypass.rs, migration.rs, error_catalog.rs}` (`DESIGN-SCHEMA.md:805-808`) are natural property-test candidates (anchor-ID generation is a deterministic function of frontmatter; bypass-marker parsing is a regex application; migration is a YAML-to-YAML transform; error-catalog loading is YAML deserialization) but none are declared pure in DESIGN-VERIFICATION.

**Why this matters (QE lens):** Phase 5 surface A authoring for vsdd-cli itself (the toolkit dogfooding its own methodology) has no target surface. An adopter project's Phase 5 surface A authoring also has no template, because vsdd-cli's own DESIGN docs don't model the discipline.

**Why this matters (SE lens):** the purity boundary is unstated, so a future implementer can't tell which functions must stay pure (anchor.rs MUST be deterministic; bypass.rs MUST not perform I/O during parsing; migration.rs MAY perform I/O at the boundary but the transform-function-itself MUST be pure). The spec leaves this to inference.

**Corrective pattern:** DESIGN-VERIFICATION adds a `§ Purity boundary` section enumerating the toolkit's own pure functions + the properties each holds:

```markdown
## Purity boundary

The following vsdd-core functions are pure (no I/O, no time reads, no RNG, no environment reads) — properties checked via proptest:

| Function | Property invariants |
|---|---|
| `anchor::derive_anchor_id(class, frontmatter) -> AnchorId` | Determinism: same input → same output. Length bounded by class. Reversible-from-fields. |
| `bypass::parse_marker(line) -> Option<BypassMarker>` | Returns Some iff line matches `<!-- hook-bypass[<id>]: <rationale> -->`. Rationale non-empty when Some. |
| `migration::transform_v1_to_v2(v1_doc) -> Result<V2Doc, MigrationError>` | Idempotent: applying twice yields same V2. Loss-free for v1 → v2 round-trip when shape is in the migration matrix. |
| `error_catalog::lookup(code, catalog) -> Option<Entry>` | Returns Some iff catalog contains code. Returns None for retired codes (per forward-only). |
```

Plus a `proptest`-track in DESIGN-VERIFICATION's implementation-order (currently absent).

**Routing:** Phase 4 → Phase 1a (DESIGN-VERIFICATION purity-boundary section addition).

### Finding 8 — `vsdd verify` exit code semantics undeclared (Dim: SE-3 error handling specificity + SE-4 API ergonomics) — Open

```yaml
finding_id: 1-f8
domain: software-engineer
dim: 3
owner: software-engineer
status: open
validator: sanity-check
classification: open
source: domain-raised
routing:
  target_phase: phase-1a
  target_artifact: DESIGN-VERIFICATION.md
  target_section: vsdd-verify-cli-subcommand-surface
dismissal_rationale: null
```

`DESIGN-VERIFICATION.md:544-560` enumerates the `vsdd verify` CLI surface but does not declare exit code semantics. A CI workflow at `DESIGN-VERIFICATION.md:258` runs `vsdd verify check --format sarif > vsdd-verify.sarif` — what exit code signals what to the CI runner?

By inference from `DESIGN-VERIFICATION.md:880-883`:

> `vsdd verify test-error-catalog` runs the regression suite … correct exit codes (0 for pass; non-zero for violation)

So `0 = pass, non-zero = violation` is the contract. But "non-zero" is unstructured — 1 = hook-violation (errors), 2 = malformed-arguments, 3 = infrastructure-failure (vsdd binary missing schema), 4 = bypass-marker-without-approval … these distinctions matter for CI scripting (a wrapper script wants to differentiate "validate found errors → comment on PR" from "infrastructure failed → retry the job").

**Why this matters (SE lens):** API ergonomics. CI authors writing GitHub Actions YAML need to know what `vsdd verify check`'s exit code means for `continue-on-error: true` vs `if: failure()` conditionals. The current spec leaves this implicit; the implementer picks arbitrary codes; CI scripts that worked at v1.0 break when the convention changes.

**Why this matters (QE lens):** falsifiability of CI workflow behavior. Without declared exit-code semantics, the manual-test for `.github/workflows/vsdd-verify.yml` can't assert "when E0040 fires, the workflow step exits with code 1 (not 2)."

**Corrective pattern:** `DESIGN-VERIFICATION.md § vsdd verify CLI subcommand surface` declares the exit-code table:

```markdown
### Exit code semantics

| Code | Meaning | When |
|---|---|---|
| 0 | All validators passed; no findings | All-clean |
| 1 | One or more validators surfaced errors (severity: error) | Per-error-code violations |
| 2 | One or more validators surfaced warnings only (no errors) | Warning-only mode |
| 64 | Usage error (malformed args; unknown subcommand) | per sysexits.h `EX_USAGE` |
| 70 | Internal error (vsdd-binary itself crashed; schema file unreadable) | per sysexits.h `EX_SOFTWARE` |
| 78 | Configuration error (`.vsdd/config.yaml` malformed; schemas-canonical-path missing) | per sysexits.h `EX_CONFIG` |
```

(sysexits.h is the BSD convention crosslink + cargo both honor.)

**Routing:** Phase 4 → Phase 1a (DESIGN-VERIFICATION addition).

### Finding 9 — OTel collector cost characteristics declared without workload (Dim: PerfE-2 workload characterization) — Open

```yaml
finding_id: 1-f9
domain: performance-engineer
dim: 2
owner: performance-engineer
status: open
validator: solution-architect
classification: open
source: domain-raised
routing:
  target_phase: phase-1a
  target_artifact: DESIGN-OBSERVABILITY.md
  target_section: cost-characteristics
dismissal_rationale: null
```

`DESIGN-OBSERVABILITY.md:194-204` declares collector cost characteristics:

| Operation | Expected cost |
|---|---|
| Collector startup | ~200ms |
| Per-event ingest | <1ms |
| Per-event redaction pass | <1ms |
| Batch export to file sink | <10ms per batch |
| Batch export to crosslink hub | <100ms per batch (network-bound) |

Cumulative collector overhead < 1% of cycle wall-clock at typical event volumes (~1000 events/cycle).

No workload is named for what generates the events. Is "~1000 events/cycle" a Phase 3 cluster-batched cycle (4 agents × ~50 events each = 200 events would be small)? Is it counting every hook-fire? Per `DESIGN-VERIFICATION.md`, every hook firing emits `HookFired` + potentially `ValidationPassed` or `ValidationFailed`. A 50-file commit with 19 hooks all firing emits ~950+ events from validation alone, before any methodology event variants. "~1000 events/cycle" is the wrong order of magnitude if every-hook-every-file emits.

**Why this matters (PerfE lens):** the redaction processor is the load-bearing security invariant (`DESIGN-OBSERVABILITY.md:186-190`: "events containing credential-shaped values never reach a sink without passing through redaction first"). If per-event redaction is actually 5ms (not <1ms) at credential-pattern-rich workloads (an event log with many quoted env vars), the cumulative redaction-pass time becomes a hot path. Without a workload-characterization for events containing how-many-pattern-candidates, the <1ms claim is unverifiable.

**Why this matters (QE lens):** Phase 5 surface C Fuzz Testing against the redaction processor needs a workload definition (input event payload shape + size distribution). Without it, the fuzz budget can't be bounded.

**Corrective pattern:** DESIGN-OBSERVABILITY's cost table names the workload column:

| Operation | Workload | Expected cost (P95) |
|---|---|---|
| Per-event ingest | OTel log event, payload ≤ 4KB, ≤ 8 attributes | <1ms |
| Per-event redaction pass | Payload ≤ 4KB, anonymization-patterns.yaml ≤ 20 patterns, no matches | <1ms |
| Per-event redaction pass | Payload ≤ 4KB, ≥ 1 match | <5ms |
| Batch export to file sink | Batch size 100 events, payload sum ≤ 400KB | <10ms |
| Cumulative collector overhead per cycle | ≤ 1000 events/cycle, OR-pattern coverage workload | <1% of cycle wall-clock |

Plus a `criterion`-bench track in DESIGN-OBSERVABILITY's implementation order for the redaction processor (regression-detection per PerfE Dim 8).

**Routing:** Phase 4 → Phase 1a (DESIGN-OBSERVABILITY cost-characteristics section revision).

### Finding 10 — `vsdd verify hook <hook-id>` undefined behavior for unknown hook-id (Dim: SE-3 error handling specificity) — Open

```yaml
finding_id: 1-f10
domain: software-engineer
dim: 3
owner: software-engineer
status: open
validator: sanity-check
classification: open
source: domain-raised
routing:
  target_phase: phase-1a
  target_artifact: DESIGN-VERIFICATION.md
  target_section: vsdd-verify-cli-subcommand-surface
dismissal_rationale: null
```

`DESIGN-VERIFICATION.md:556` declares:

> `vsdd verify hook <hook-id>` Rust hook-runner mirror invocation matches the Python hook's enforcement logic against the same JSON Schema source

What happens when `<hook-id>` is not a registered hook? The spec doesn't say. Possible failure modes:

- Silent exit 0 (no hook ran; nothing to report) — worst for CI (false pass)
- Panic with stack trace (rustc-style) — leaks internals
- Friendly error: `error: unknown hook-id 'foo'. Known hooks: <enumerate>`. exit 64.

By inference from sysexits.h, exit 64 + a friendly listing is right. But the spec doesn't declare this.

**Why this matters (SE lens):** CI workflows reference hook-ids by string. A typo in `.github/workflows/vsdd-verify.yml` (`vsdd verify hook check-fronmatter-schema` instead of `check-frontmatter-schema`) needs to fail loudly. The current spec's silence is consistent with all three failure modes above.

**Why this matters (QE lens):** falsifiability of CI behavior. A Phase 2a Red Gate test for the vsdd CLI needs to assert behavior on typo-hook-id. Without the spec stating the behavior, the test can't be written.

**Corrective pattern:** DESIGN-VERIFICATION's CLI surface section adds an "Unknown hook-id" failure-mode declaration:

```markdown
### Failure modes

| Failure | Exit code | Message |
|---|---|---|
| Unknown hook-id | 64 | `error: unknown hook-id '<id>'. Known hooks: <list of 19>` |
| Files arg list contains paths outside repo | 64 | `error: file '<path>' is outside the git working tree` |
| Schema file unreadable (binary corrupted) | 70 | `error[infrastructure]: schema file '<path>' unreadable: <io-error>` |
```

**Routing:** Phase 4 → Phase 1a (DESIGN-VERIFICATION CLI-surface failure-modes addition).

### Finding 11 — `check-dependency-approval` discipline cannot detect approval bypass via PR-edit (Dim: SE-3 error handling + QE-1 falsifiability) — Open

```yaml
finding_id: 1-f11
domain: software-engineer
dim: 3
owner: software-engineer
status: open
validator: security
classification: open
source: domain-raised
routing:
  target_phase: phase-1a
  target_artifact: DESIGN-VERIFICATION.md
  target_section: dependency-approval-discipline
dismissal_rationale: null
```

`DESIGN-VERIFICATION.md:619-673` declares the dependency-approval hook. The trigger fires on commits that add new dependency-manifest entries; the discipline requires the PR description carries a "Dependency approval" section with SO + PE + Security investigation.

**Attack surface:** the PR description is mutable. A PR author can:

1. Open PR with dependency-approval section present + SO co-authorship trailer + dummy investigation text
2. Wait for `check-dependency-approval.py` hook to validate at commit time + pass
3. Edit the PR description after merge-gate clears, removing the dependency-approval section
4. Merge

The hook validates at COMMIT time (per `DESIGN-VERIFICATION.md:223`: "Pre-commit + Pre-merge on dependency-manifest changes"). Pre-merge re-validates against the PR description AT THAT TIME, not against historical state. If the PR description edit-then-merge sequence happens within the pre-merge re-check window, the bypass succeeds. The bypass-approval-label mechanism (the only protection against self-applied bypass) only fires for explicit `<!-- hook-bypass[...] -->` markers, not for stealth-removal of required PR-body sections.

**Why this matters (SE lens):** the dependency-approval discipline is the operator-directive-triggered hook (per `README.md:501-502`); it carries supply-chain attestation weight. A bypass through stealth-edit defeats the SO + PE + Security investigation gate.

**Why this matters (QE lens):** falsifiability — the manual-test for `check-dependency-approval.py` needs to assert "post-merge edit of PR body that removes the approval section is detected." The current spec has no such assertion path.

**Corrective pattern:** the dependency-approval hook stores its evidence in a commit-tracked file (`docs/dependencies/<crate>.md` is already named in the spec at `README.md:501`). The hook validates the file's existence + content shape, not just the PR-description text. PR-description edits can't retroactively remove a committed file without a follow-up commit, which re-fires the hook. The investigation record IS the load-bearing artifact; the PR description is just a surfacing layer.

Explicit declaration in DESIGN-VERIFICATION:

```markdown
The investigation record at `docs/dependencies/<crate>.md` is the canonical evidence; the PR-description section is a surfacing convention. The hook validates the file's existence + content shape; PR-description-only investigation is rejected with VSDD-E0101: dependency-approval-record-missing-file.
```

**Routing:** Phase 4 → Phase 1a (DESIGN-VERIFICATION dependency-approval section + new code addition).

### Finding 12 — Cluster-batching shape inherits Implementation cluster fragility for low-axis projects (Dim: SE-1 spec-implementation alignment + QE-2 acceptance criteria coverage) — Open

```yaml
finding_id: 1-f12
domain: quality-engineer
dim: 2
owner: solution-owner
status: open
validator: solution-owner
classification: open
source: domain-raised
routing:
  target_phase: phase-1a
  target_artifact: DESIGN-METHODOLOGY.md
  target_section: cluster-batching-shape-for-phase-3-cycles
dismissal_rationale: null
```

`DESIGN-METHODOLOGY.md:382-397` declares the Implementation cluster as SE + QE + Performance Engineer. The always-on baseline (`DESIGN-METHODOLOGY.md:340-358`) activates SE + QE + SA + SO for any project, plus PE + PerfE when the project ships code.

For a project that ships code but has all 9 per-feature axes set `no`: 6 domains active (SE + QE + SA + SO + PE + PerfE). The Implementation cluster is SE + QE + PerfE = 3 of the 6 active. The Architecture cluster is SA + PE + DE — but DE only activates with `persists-managed-schema-data` axis, so for the zero-axes-shipping-code project, Architecture cluster is SA + PE (2 domains). The Communication cluster is Security + TW + Accessibility + Privacy + Localization — ALL axis-activated, so zero domains. The Adversarial cluster includes SO + VSDD Methodology + Sanity Check (and axis-activated Red Team + DR + UX + AI Engineer = all empty) — so 3 domains (SO + 2 metas).

So the 4-cluster shape for a zero-axes-shipping-code project: Implementation 3, Architecture 2, Communication 0, Adversarial 3. The Communication cluster collapses to empty; adversarial-pair separation (Security ↔ Red Team must be different clusters per `vsdd-phase-3.md:21`) becomes vacuous because Red Team isn't activated. The cluster-batching invariant is not preserved across axis combinations.

**Why this matters (QE lens):** the test the `check-phase-composition.py` hook would write — "for any composed_domain set, the 4-cluster shape preserves adversarial-pair separation + non-empty clusters" — fails on the zero-axes-shipping-code case. The methodology asserts this invariant (via the cluster shape table) but the actual composition rule violates it.

**Why this matters (SE lens):** the implementer authoring the cluster-batching dispatch logic needs a deterministic rule for "when a cluster is empty, what does cold-session spawn do?" Currently, no rule. The implementation choice (skip empty clusters? collapse to 3 clusters? always spawn the cluster with a no-op agent?) is left to inference.

**Corrective pattern:** DESIGN-METHODOLOGY's cluster-batching section declares the empty-cluster handling rule:

```markdown
### Empty-cluster handling

When the per-feature-axes activation produces an empty cluster (e.g., zero-axes project with no Communication-cluster domains active), the cluster is omitted from the Phase 3 dispatch. Adversarial-pair separation is checked against the REMAINING clusters; if separation cannot be preserved (e.g., Security is active but no second cluster exists to separate from Red Team), Phase 3 dispatch falls back to per-domain spawn for active adversarial-pair members.
```

This also composes with QE Finding 4 + Security Finding 1 (zero-axes edge state) — a single operator-directive can resolve all three findings via the always-on-baseline declaration + the empty-cluster handling rule.

**Routing:** Phase 4 → operator-directive (composes with the pending zero-axes resolution) → Phase 1a (DESIGN-METHODOLOGY revision).

### Finding 13 — DESIGN-OBSERVABILITY references MCP tool I/O artifact class as still-existing despite removal (Dim: QE-2 acceptance criteria coverage; consistency lens) — Hallucinated

```yaml
finding_id: 1-f13
domain: quality-engineer
dim: 2
owner: quality-engineer
status: open
validator: sanity-check
classification: hallucinated
source: domain-raised
routing:
  target_phase: null
  target_artifact: null
  target_section: null
dismissal_rationale: "DESIGN-OBSERVABILITY.md:429 says 'Each tool's input + output schemas declared per the MCP tool I/O artifact class (DESIGN-SCHEMA).' On closer reading of DESIGN-SCHEMA.md:573-577, the MCP tool I/O artifact class IS dropped — but DESIGN-OBSERVABILITY's reference at line 429 is to the MCP protocol's native validation, not to a vsdd artifact class. The phrasing is ambiguous (a Phase 3 finding could justifiably surface this as a real cross-doc drift) but the actual content is consistent: DESIGN-SCHEMA says MCP tool I/O 'lives as Rust code in vsdd/src/mcp_serve/' (line 105), and DESIGN-OBSERVABILITY's reference is to that same Rust-code-as-schema. The 'MCP tool I/O artifact class' phrase at DESIGN-OBSERVABILITY:429 is residual prose that could be tightened but does not assert the class exists. SA Finding 2 from 2026-05-27-solution-architect.md already catches the orphan-anchor-ID rule for this dropped class; that's the load-bearing finding. The implementation cluster's marginal contribution here is zero."
```

Marked hallucinated; ceding to SA Finding 2's coverage of the dropped-class orphan-reference pattern. Including for transparency — the cluster considered raising this and confirmed it was already covered.

## Summary

13 findings — 12 Open, 1 Hallucinated. Implementation cluster's lens surfaced:

- 2 error-code collisions (E0021 + E0220) — load-bearing for catalog uniqueness invariant
- 3 performance-contract gaps (validator wall-clock workload; Python-subprocess hot path; OTel collector cost workload)
- 3 falsifiability gaps (Manual-test falsifiability_check unconstrained; consolidated-hook per-rule fixture gap; purity boundary uncatalogued for own toolkit)
- 3 API/error-handling gaps (`vsdd verify` exit codes; `vsdd verify hook` unknown-id; dependency-approval bypass via PR-edit)
- 1 cluster-batching shape fragility (empty-cluster handling)

Per primer 3's Exacting Mentor stance: the spec is well-organized and the cross-doc coordination tables are doing real work, but the surfaces this cluster owns — testability + falsifiability + implementation seams + performance — carry the kind of "looks fine, breaks on first implementation cycle" gaps that emerge under cold-context lens.

## Coordination

**Coordinate with Architecture cluster (SA + Platform Engineer + Data Engineer):**

- Finding 4 (Python-subprocess dispatch) routes to SA validator — architectural seam change
- Finding 9 (OTel cost workload) routes to SA — composes with DESIGN-OBSERVABILITY scope-boundary review
- Finding 12 (cluster-batching empty-cluster handling) composes with the pending zero-axes resolution from QE F4 + Security F1

**Coordinate with Communication cluster (Security + TW + Accessibility + Privacy + Localization):**

- Finding 2 (E0220 overload) involves the check-anonymization hook — Security validator-pair appropriate
- Finding 11 (dependency-approval PR-edit bypass) routes to Security — supply-chain attestation surface

**Coordinate with Adversarial cluster (Red Team + DR + UX + AI Engineer + Solution Owner + VSDD Methodology + Sanity Check):**

- Findings 1, 2, 5, 6, 7, 8, 10 all route through Phase 4 → Phase 1a DESIGN-doc revision; bundle for a single round of spec-coordinate edits where possible
- Finding 12 needs operator-directive (composes with the pending zero-axes from Phase 5 round 1 inline findings); route to SO via VSDD Methodology meta-domain validator

**Phase 5 surface preparation (when Phase 5 runs against vsdd-cli itself):**

- Surface A (property-based testing): blocked on Finding 7 resolution (purity boundary not catalogued)
- Surface B (mutation testing): blocked on Finding 6 resolution (per-sub-rule fixtures needed)
- Surface C (fuzz testing): the redaction processor is a natural target; blocked on Finding 9 (workload-characterization needed for fuzz-budget bounding)

**Bundled-edit opportunity:** Findings 1 + 2 + 8 + 10 are all `DESIGN-VERIFICATION` + `README` error-catalog / CLI-surface coordinate edits. A single Phase 4 → Phase 1a commit could close all four with one cross-doc consistency pass.
