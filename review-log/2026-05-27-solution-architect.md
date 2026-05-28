---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-05-27
phase: phase-5
scope: Purity Boundary Audit — DESIGN-doc scope-boundary verification across the 5 canonical artifacts (README + DESIGN-METHODOLOGY + DESIGN-SCHEMA + DESIGN-OBSERVABILITY + DESIGN-VERIFICATION)
lens: spec-stage Phase 5 Purity Boundary Audit (adapted from canonical primer 5 — pure-function purity claim verification → DESIGN-doc scope-boundary verification; no implementation surface exists yet)
source: domain-raised
session_note: cold-context — first Phase 5 round on vsdd-cli; baseline establishment against the trio + README post Phase 4 routing of 13 prior findings + naming-discipline sweep (commit a0a4987)
model: claude-opus-4-7
execution_method: inline main session (cluster-batched with QE + Security per operator directive 2026-05-27)
sycophancy_compensation: I authored the 5 canonical docs under review; the bias is toward declaring the scope boundaries clean. The audit pressure is to find at least one boundary leak per doc OR cite the specific table row that confirmed the boundary held — silent pass without evidence is the failure mode.
---

# Solution Architect Review 1 — 2026-05-27

**Phase 5 surface:** A.0 — Purity Boundary Audit (DESIGN-doc scope-boundary verification adaptation)
**Cold-session shape:** N/A — inline-run from the main session. Trade-off declared per the bounded-judgment-surface rubric: spec-stage purity-boundary audit is mechanical scope-leak detection against declared "owns / does not own" tables in each DESIGN doc. Cold-session cluster spawn would be over-investment per the queued AI-Engineer-domain cost concern (token-expensive at 10-agent scale, low marginal value over inline for first-run-against-fresh-author).

## Scope

The 5 canonical docs each declare a `Scope + boundary` section with "owns / does not own" lists and a `Cross-DESIGN-doc coordination` section with "produces for / forward-references" tables. The audit checks (a) every "owns" claim has corresponding content authored in that doc; (b) every "does not own" claim is honored (no scope leakage); (c) every cross-DESIGN-doc consume/produce table entry has a matching entry in the cited sibling (symmetric coordination).

## Findings

### Finding 1 — Cross-DESIGN-doc count drift in DESIGN-VERIFICATION (Dim: cross-source consistency) — Open

DESIGN-VERIFICATION cites stale 14-class counts in 3 sites after the 15→13 reconciliation in commit a0a4987:

- `DESIGN-VERIFICATION.md:68` — `Per DESIGN-METHODOLOGY's reconciled dual-mode declaration: 14 frontmatter-based classes + 1 structural class (CHANGELOG).`
- `DESIGN-VERIFICATION.md:717` — workspace tree comment `# 14 frontmatter classes`
- `DESIGN-VERIFICATION.md:832` — cross-doc coordination row `DESIGN-SCHEMA | 14 frontmatter JSON Schemas + 1 structural rule file + error catalog file format`

DESIGN-METHODOLOGY's reconciled declaration is **12 frontmatter + 1 structural = 13 total** (Pre-phase composition declaration folded into PhaseCompositionDeclared event payload; MCP tool I/O folded into MCP protocol's native schema validation). My letter-label + 15→13 sweep in the Phase 4 commit missed these three sites. Boundary leak: DESIGN-VERIFICATION's authoritative claim about how many schemas it consumes contradicts DESIGN-METHODOLOGY's authoritative claim about how many classes exist.

**Why this matters:** the Rust mirror in DESIGN-VERIFICATION compiles `vsdd-core/src/schemas/` per the count it claims to consume — if it counts 14, the build will fail when only 12 frontmatter Rust types exist. A future implementer reading DESIGN-VERIFICATION would author against the stale count.

**Routing:** Phase 4 → Phase 1a (DESIGN-VERIFICATION spec revision). Owning artifact: DESIGN-VERIFICATION.md lines 68, 717, 832. Gate: project-wide grep returns clean for `14 frontmatter` + `14 classes` across all canonical docs; DESIGN-VERIFICATION's count matches DESIGN-METHODOLOGY's reconciled declaration.

### Finding 2 — Orphan anchor-ID rules for dropped classes (Dim: cross-source consistency) — Open

DESIGN-SCHEMA's Anchor-ID generation conventions table (`DESIGN-SCHEMA.md:631-644`) still has rows for two dropped artifact classes:

- `| Pre-phase declaration | phase-comp-{phase}-{declared_at-date} |` — class folded into PhaseCompositionDeclared event payload
- `| MCP tool I/O | mcp-tool-{tool_name-kebab} |` — class delegated to MCP protocol's native validation

Both anchor-ID rules are now orphaned — they reference classes that no longer exist in the artifact-class set. A future implementer authoring `vsdd-core/src/anchor.rs` would emit anchor-ID derivation rules for classes that have no schema file. Mechanical inconsistency.

**Why this matters:** the anchor-ID derivation discipline (deterministic anchor-IDs from frontmatter; no hand-authored HTML anchors) depends on the anchor-rule table being the authoritative source. Orphan rules in the table contradict the dropped-class decision in Phase 3 review SO+SA consolidation.

**Routing:** Phase 4 → Phase 1a (DESIGN-SCHEMA spec revision). Owning artifact: DESIGN-SCHEMA.md:631-644. Gate: anchor-ID rule table matches the 13-class set; orphan rules either removed OR annotated as "folded into <target>; no standalone anchor-ID needed."

### Finding 3 — DESIGN-SCHEMA implementation-order tracks reference Rust crate internals (Dim: scope-boundary leak) — Accepted

`DESIGN-SCHEMA.md:800-808` (tracks 3a-3h) reference Rust crate file paths (`vsdd-core/src/anchor.rs`, `vsdd-core/src/bypass.rs`, `vsdd-core/src/migration.rs`). DESIGN-VERIFICATION declares ownership of "Rust crate workspace structure + Binary distribution" (`DESIGN-VERIFICATION.md:40-41`). Surface tension: which doc owns the file-paths-inside-vsdd-core decisions?

**Resolution rationale:** schema source format + code generation pipeline IS DESIGN-SCHEMA's territory; file paths for the schema sources (`vsdd-core/schemas/<class>.json`) and the utility modules that consume them (anchor.rs, bypass.rs, migration.rs) are scope-coordinate cross-references, not boundary violations. DESIGN-VERIFICATION owns the *workspace shape* (Cargo.toml, crate boundaries, binary entry); DESIGN-SCHEMA owns the *content shape* (schemas + utility module purposes). Boundary held tightly but did not leak.

**Note:** if a future revision adds substantive Rust-crate-internals decisions to DESIGN-SCHEMA (e.g., specific dependency choices for `vsdd-core`), that crosses the boundary and routes to DESIGN-VERIFICATION.

**Classification:** Accepted (boundary tight but coherent; documented here so future revisions can detect drift).

## Summary

3 findings — 2 Open + 1 Accepted. Two real cross-source consistency defects (count drift; orphan anchor-ID rules) — both consequences of the Phase 4 routing commit incomplete grep coverage. One scope-tension surface accepted-with-rationale.

The Phase 4 commit (a0a4987) was claimed clean against project-wide grep for `15 artifact|15 classes|15 frontmatter` patterns — but didn't sweep `14 frontmatter` (the intermediate count after one class dropped from each pair) nor the anchor-ID derivation table. Per primer 4 anti-pattern "Site-specific fix declared closure": even a project-wide grep can miss adjacent defect sites when the grep pattern doesn't anticipate the residual intermediate state. The lesson for future sweeps: grep both the old count AND the intermediate counts AND the per-class names that should be removed.

## Coordination

- Finding 1 + Finding 2 route to Phase 1a (DESIGN-doc spec revision). Suggest bundling in a single Phase 4 → Phase 1a commit alongside the rest of this Phase 5 round's findings.
- Finding 3 informs future cross-DESIGN-doc-boundary decisions; no current action required.
- Cross-domain: QE's invariant-property-check (this round) and Security's adversarial-input-fuzz (this round) may surface additional sites that compound with Finding 1's count-drift class.
