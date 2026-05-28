---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-05-27
phase: phase-5
scope: Invariant-Property Check — cross-DESIGN-doc consistency invariants across the 5 canonical artifacts (error code declarations, event variant declarations + counts, hook-to-error-code mappings, axis → activation completeness)
lens: spec-stage Phase 5 surface B (Mutation Testing analog) — applied to the spec rather than to a test suite. The invariant-property checks ARE the spec's mutation-test surface: each invariant is a property the spec asserts that should hold structurally across docs; counterexamples are mutation-testing-equivalent test-gap signals.
source: domain-raised
session_note: cold-context — first Phase 5 round on vsdd-cli; inline-clustered with SA + Security per operator directive 2026-05-27
model: claude-opus-4-7
execution_method: inline main session (cluster-batched with SA + Security)
sycophancy_compensation: I authored the 5 canonical docs under review; the bias is to declare invariants hold trivially. The audit pressure is to name a specific table row that violates each invariant OR cite the structural reason the invariant cannot fail (the property test must fail against a stub of the spec, not pass against an empty cross-doc table).
---

# Quality Engineer Review 1 — 2026-05-27

**Phase 5 surface:** B — Invariant Property Check (Mutation Testing analog adapted to spec-stage: each invariant is a property the spec asserts that should hold structurally across docs; the test is whether the spec's cross-doc tables would catch a mutation)
**Cold-session shape:** N/A — inline-run from the main session. Trade-off declared per the bounded-judgment-surface rubric: cross-doc consistency invariants are mechanical/empirical checks against the 5 canonical docs (grep + count + cross-reference). Cold-session cluster spawn would be over-investment.

## Scope

5 cross-DESIGN-doc invariants the spec asserts that should hold structurally:

1. **Every error code mentioned in any DESIGN doc exists in README's catalog** (orphan-code check)
2. **Every event variant count claim matches the table** (count-vs-content consistency)
3. **Every hook in the deployment matrix has a unique error code OR documented shared-code rationale** (error-code ownership)
4. **Every artifact class in the validation modes table has the same count as DESIGN-METHODOLOGY's reconciled declaration** (count-vs-list consistency)
5. **Every domain in the 18-domain set has either an axis-driven activation OR an explicit always-on declaration** (no-implicit-baseline check)

## Findings

### Finding 1 — Orphan error code reference: VSDD-W0140 (Dim: falsifiability — every declared code must be discoverable) — Open

`DESIGN-VERIFICATION.md:181` declares the check-naming-discipline hook fires three codes: `VSDD-E0160, VSDD-W0001, VSDD-W0140`. Project-wide grep:

- `VSDD-E0160` — declared in README catalog as `letter-label-anti-pattern` (Accepted)
- `VSDD-W0001` — declared in README catalog as `vestigial-pattern-detected` (Accepted)
- `VSDD-W0140` — **declared nowhere**

The check-naming-discipline hook is documented as consolidated 4-rule dispatch: letter-label anti-pattern (E0160) + vocabulary registry conformance (W0001 vestigial-pattern-detected) + suite-internal terminology + first-use expansion. W0140 corresponds to one of the latter two rules but has no catalog entry. Mutation-testing analog: a project violating "first-use expansion" or "suite-internal terminology" would fire a hook that emits an unrecognized error code; the validator's error-catalog test-suite would not have a fixture for it; `vsdd verify explain VSDD-W0140` would return "code not found."

**Why this matters:** the catalog's status-tier discipline (candidate / accepted / deprecated) requires every code to be authored + tested. An orphan code declaration in the hook table is a falsifiability gap — the rule fires a code that doesn't formally exist.

**Routing:** Phase 4 → Phase 1a (DESIGN-doc spec revision). Owning artifact: either declare W0140 in README's catalog AND DESIGN-SCHEMA's error-code-range conventions (suite-internal-terminology-violation? first-use-expansion-missing?), OR consolidate the hook to use only E0160 + W0001 + a future earned-by-recurrence-derived code. Gate: every hook-cited error code resolves in `vsdd verify explain`.

### Finding 2 — Event-variant count inconsistency: Discipline-enforcement variants header says 3 but table has 4 (Dim: cross-source consistency) — Open

`DESIGN-OBSERVABILITY.md:236` declares `### Discipline-enforcement variants (3)` but the table immediately below (`DESIGN-OBSERVABILITY.md:238-243`) lists 4 entries: `SycophancySelfAudit`, `OperatorDirectiveApplied`, `ProtectiveDisciplineEnforced`, `VerificationMiniCycleSpawned`. Header-vs-content drift; total reconciles to 18 only if the 4-count is correct (counted by category: 5 phase + 3 finding + 1 cycle + 4 discipline + 1 auth + 1 project + 3 PR = 18 ✓).

**Why this matters:** mutation-testing analog — if a future implementer mutates the table to remove one of the 4 entries (e.g., drops `VerificationMiniCycleSpawned`), the header still says "(3)" and the structural property "header count matches row count" wouldn't catch the mutation. The invariant has no defender.

**Routing:** Phase 4 → Phase 1a (DESIGN-OBSERVABILITY spec revision; trivial fix). Owning artifact: DESIGN-OBSERVABILITY.md:236. Gate: header parenthetical matches row count.

### Finding 3 — Same error code (VSDD-E0050) fires from two different hooks without ownership rationale (Dim: error-code-ownership) — Open

`DESIGN-VERIFICATION.md:185-186`:

```
| 8 | check-phase-transitions ... | VSDD-E0050 + phase-specific codes | Both |
| 9 | check-phase-composition | ... | VSDD-E0050 | Both |
```

Both hooks fire `VSDD-E0050`. When a CI run reports `VSDD-E0050 fired`, the operator cannot tell which hook is the trigger source without re-reading the per-hook output (assuming the hook output names itself, which is hook-implementation-discipline not error-code-discipline). The error catalog (`DESIGN-SCHEMA.md:692-712`) declares each code carries one summary + one detail + one help — implying one source.

**Why this matters:** the error-code design contract is per-code-one-source (Rust's `--explain` pattern; one canonical explanation per code). Shared codes across multi-hook firing surfaces violate that contract or require explicit shared-code rationale.

**Routing:** Phase 4 → Phase 1a (DESIGN-SCHEMA + DESIGN-VERIFICATION coordinate revision). Either: (a) split into E0050 (phase-composition-not-declared) + E0051 (phase-transition-not-attested) — preserves per-code-one-source; (b) document shared-code rationale (e.g., "E0050 fires when EITHER hook detects phase-boundary discipline violation; per-hook output names the specific surface") — extends the catalog contract to explicitly permit shared codes. Suggest (a) for clarity.

### Finding 4 — Implicit always-on domain baseline is undeclared (Dim: cross-source consistency) — Open

The per-feature axes → domain activation matrix (README.md:674-684) only specifies extended/specialty domain activation per axis. Domains never explicitly activated by any axis:

- Software Engineer (SE)
- Quality Engineer (QE)
- Solution Architect (SA)
- Solution Owner (SO)
- Platform Engineer (PE)
- Performance Engineer (PerfE)

These are presumed always-on for any non-trivial project but the methodology never says so. A project that declares all 9 axes as `no` would activate **zero domains** per the matrix — leaving Phase 3 with no composed_domains; the cluster-batching shape (which assumes 4 clusters × multi-domain) collapses; Phase 5 (which composes QE + Security + SA) has no domains to compose.

**Why this matters:** invariant: every methodology phase has a non-empty composed_domains for any project that exists. Current spec violates the invariant for the all-no-axes edge case. The phase-domain composition matrix would emit `composed_domains: []` and the `check-phase-composition.py` hook has no failure path for empty composition.

**Routing:** Phase 4 → Phase 1a (DESIGN-METHODOLOGY + README coordinate revision). Three options:

- (a) Declare an explicit always-on baseline: SE + QE + SA + SO activate regardless of axes; PE + PerfE activate when the project ships code. The methodology declares the floor; the matrix extends from there.
- (b) Make at least one axis required (the `ships-to-users-other-than-developer` axis defaults yes; a project must explicitly declare it no with rationale). Avoids the zero-axes state entirely.
- (c) Document the zero-axes path explicitly as "research-only / spike project; Phase 3 skipped; methodology surfaces are advisory not gating."

Operator-directive needed to pick the option.

### Finding 5 — Hook table 17 entries; declared count 18 (Dim: cross-source consistency) — Open

`DESIGN-VERIFICATION.md:170` declares `## Per-hook deployment matrix (~18 hooks)`. The table at lines 178-194 has 17 numbered rows (1-17). The 18th hook (`check-dependency-approval`) is described in a separate section at `DESIGN-VERIFICATION.md:653-660` with declaration `~18th methodology hook; brings total to ~18` but doesn't have a row in the deployment matrix table itself.

The dependency-approval hook trigger / rules / error codes / mode are described but the table that operationalizes per-hook deployment timing + surface (Operator-local vs CI) has 17 entries.

**Why this matters:** the per-hook deployment matrix is the load-bearing table for `vsdd init` + CI bootstrap. A future implementer authoring `.claude/hooks/vsdd-*.py` reads the table to enumerate which hooks to deploy. Missing row = missing deployment.

**Routing:** Phase 4 → Phase 1a (DESIGN-VERIFICATION spec revision; trivial fix). Owning artifact: DESIGN-VERIFICATION.md:178-194 (extend table with row 18 for check-dependency-approval; trigger: commits touching dependency manifests; rule: SO + PE + Security investigation present + investigation doc at `docs/dependencies/<crate>.md`; error code: VSDD-E0100; mode: Both).

## Summary

5 findings — all Open. Cross-doc consistency invariants are more drift-prone than the in-author review surface caught:

- 1 orphan code (W0140) hiding inside a 4-rule consolidated hook
- 1 header-vs-table count inconsistency (3 vs 4 discipline-enforcement variants)
- 1 multi-hook-shared-error-code ambiguity (E0050 fires from 2 hooks)
- 1 implicit-always-on-domain-baseline gap (zero-axes edge state undefined)
- 1 hook-count-vs-table-rows drift (~18 declared; 17 rows; dep-approval described separately)

Per primer 5 anti-pattern "writing a property whose only assertion is doesn't panic": these 5 invariants assert structural cross-doc consistency, not just liveness. A property test for invariant 4 (always-on baseline) would fail against a stub spec that declares zero axes — which is exactly what the current spec asserts, so the property test IS failing against the spec as authored. That's the right behavior for a mutation-testing-analog finding.

## Coordination

- Findings 1-3, 5 route to Phase 1a (DESIGN-doc spec revision); all are bounded-disposition spec-coordinate edits. Bundle with SA's Findings 1-2 for a single Phase 4 → Phase 1a commit.
- Finding 4 (implicit always-on baseline) requires an **operator-directive decision** — picking (a)/(b)/(c) is a methodology-amendment-class decision, not a mechanical fix. Route to operator.
- Cross-domain: Security's adversarial-input-fuzz round may surface adjacent zero-axes / methodology-assumption findings that compound with Finding 4.
