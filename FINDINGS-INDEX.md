# FINDINGS-INDEX.md — vsdd-cli

Canonical aggregation surface for findings across cycles. Per primer 4 § Manual mode completion criterion #4: "Suite findings are filed in `FINDINGS-INDEX.md`, not collapsed into project-phase routes."

This document was **authored retroactively** on 2026-05-27 after operator-directive surfaced the methodology-spirit gap: prior Phase 3 + Phase 5 cycles produced 26 findings + routed them via commit messages but never aggregated them to a single source. Per primer 4's routing table, this is a "Process gap → Phase 4 itself" finding (documented in the Process-gap section below).

**Operational mode:** Manual mode (`crosslink issue list` returns no issues; findings tracked in review-log entries + commit messages + this index). Adopting projects using `crosslink swarm review --file-issues` would file findings as crosslink issues automatically; vsdd-cli's own development has been operator-orchestrated without that flag.

---

## Source taxonomy

Per primer 3 § Source field discipline + primer 4 § Source column extensions:

- `domain-raised` — finding raised within the domain's normal review pass
- `director-raised` — operator surfaced the finding outside a review pass
- `regression-replay` — finding from a prior cycle's regression check
- `external-feedback` — finding from outside the project (user report, etc.)
- `mixed` — multiple sources

Phase tag per finding: `phase:3` / `phase:4` / `phase:5` / `phase:6` / `process-gap`.

---

## Round 1 — Phase 3 IAR (scope: DESIGN trio) (2026-05-27; commits 607a075..713aeeb → resolution in a0a4987)

**Scope:** DESIGN-SCHEMA + DESIGN-OBSERVABILITY + DESIGN-VERIFICATION; DESIGN-METHODOLOGY revalidation against the trio.

**Method:** Inline-clustered per-domain review (NOT cold-session — methodology violation surfaced retroactively in the later Phase 5 round; tracked here as pg-1).

**Routing closure:** Commit `a0a4987` — "Phase 4 routing: integrate 13 Phase 3 review findings + dependency approval directive + naming-discipline sweep"

| Finding ID | Domain | Dim | Title | Classification | Source | Routing | State |
|---|---|---|---|---|---|---|---|
| 1-f1 | SA | architecture-coherence | Pre-phase composition declaration class consolidation | Resolved | domain-raised | phase-1a (DESIGN-SCHEMA) | ✅ Resolved in a0a4987 (class folded into PhaseCompositionDeclared event) |
| 1-f2 | SO | scope-consolidation | MCP tool I/O class redundant with MCP protocol native validation | Resolved | domain-raised | phase-1a (DESIGN-SCHEMA) | ✅ Resolved in a0a4987 (class dropped; 15 → 13 classes) |
| 1-f3 | SA | architecture-coherence | OTel collector lifecycle undefined (persistent daemon vs on-demand) | Resolved | domain-raised | phase-1a (DESIGN-OBSERVABILITY) | ✅ Resolved in a0a4987 (on-demand spawn declared) |
| 1-f4 | SA | drift-prevention-by-construction | Python hooks reimplement validation; drift between operator-local + CI possible | Resolved | domain-raised | phase-1a (DESIGN-VERIFICATION) | ✅ Resolved in a0a4987 (Python subprocess to Rust binary; one canonical source) |
| 1-f5 | AIE | cardinality-discipline | Per-variant cardinality classification missing for external-backend cost-at-scale | Resolved | domain-raised | phase-1a (DESIGN-OBSERVABILITY) | ✅ Resolved in a0a4987 |
| 1-f6 | AIE | cache-invalidation | Methodology lookup cache TTL doesn't handle mid-session file edits | Resolved | domain-raised | phase-1a (DESIGN-OBSERVABILITY) | ✅ Resolved in a0a4987 (file-mtime-aware) |
| 1-f7 | Security | redaction-config-location | Anonymization patterns inline-declared vs registry-file | Resolved | domain-raised | phase-1a (DESIGN-OBSERVABILITY) | ✅ Resolved in a0a4987 (config_source: .vsdd/registry/anonymization-patterns.yaml) |
| 1-f8 | Security | processor-ordering | Redaction processor ordering not declared as structural invariant | Resolved | domain-raised | phase-1a (DESIGN-OBSERVABILITY) | ✅ Resolved in a0a4987 |
| 1-f9 | Red Team | external-backend-confirmation | New external backend endpoint silently forwards events; data-exfiltration surface | Resolved | domain-raised | phase-1a (DESIGN-OBSERVABILITY) | ✅ Resolved in a0a4987 (operator-confirmation gate) |
| 1-f10 | Red Team | bypass-self-approval | Bypass-approved label could be self-applied by PR author | Resolved | domain-raised | phase-1a (DESIGN-VERIFICATION) | ✅ Resolved in a0a4987 (label-applier ≠ PR-author check) |
| 1-f11 | Red Team | schema-injection-attack-surface | Validator could hot-load PR-modifiable schemas | Resolved | domain-raised | phase-1a (DESIGN-VERIFICATION) | ✅ Resolved in a0a4987 (canonical-schema-path discipline) |
| 1-f12 | QE | validator-test-suite-blocks-release | Validator test-suite failure didn't block toolkit release | Resolved | domain-raised | phase-1a (DESIGN-VERIFICATION) | ✅ Resolved in a0a4987 |
| 1-f13 | PE + Security | pre-built-binary-signing | Pre-built binaries listed v1+ but supply-chain + CI ergonomics gap justify v1.0 | Resolved | domain-raised | phase-1a (DESIGN-VERIFICATION) | ✅ Resolved in a0a4987 (track 5n promoted to v1.0 ship-blocker) |
| 1-process-gap-1 | VSDD Methodology | methodology-spirit-adherence | Phase 3 ran inline, not cold-session, without explicit operator-directive | Documented | regression-replay | phase-4 self-routing | 📝 Documented retroactively in 9b85504 + this index |

**Round closure:** 13 findings Resolved + 1 process-gap documented. Implementation-MVR signal asserted but compromised by the inline-execution methodology violation (pg-1).

---

## Round 1 — Phase 5 (scope: spec-stage adapted surfaces) (2026-05-27; commit 9b85504 → resolution in 6fb9bcb)

**Scope:** Spec-stage adapted Phase 5 (Purity Boundary Audit + Invariant Property Check + Adversarial-Input Fuzz on the 5 canonical docs).

**Method:** Inline-clustered (SA + QE + Security composed in main session; declared in pre-phase composition declaration). The cold-session-vs-inline rubric was honored — bounded judgment surface + explicit trade-off declaration per per-round preamble.

**Routing closure:** Commit `6fb9bcb` — "Phase 4 routing for Phase 5 round 1: 12 findings resolved (8 mechanical fixes + 4 methodology amendments) + 1 deferred to v1+"

| Finding ID | Domain | Dim | Title | Classification | Source | Routing | State |
|---|---|---|---|---|---|---|---|
| 2-f1 | SA | cross-source-consistency | Count drift in DESIGN-VERIFICATION (3 stale "14 frontmatter classes" sites) | Resolved | domain-raised | phase-1a (DESIGN-VERIFICATION) | ✅ Resolved in 6fb9bcb |
| 2-f2 | SA | cross-source-consistency | Orphan anchor-ID rules for dropped Pre-phase + MCP tool I/O classes | Resolved | domain-raised | phase-1a (DESIGN-SCHEMA) | ✅ Resolved in 6fb9bcb |
| 2-f3 | SA | scope-boundary-leak | DESIGN-SCHEMA implementation-order references Rust-crate paths | Accepted | domain-raised | (no action) | 📋 Accepted-with-rationale; documented for future-drift detection |
| 2-q1 | QE | falsifiability | Orphan VSDD-W0140 code in check-naming-discipline hook (not declared in catalog) | Resolved | domain-raised | phase-1a (DESIGN-VERIFICATION + DESIGN-SCHEMA) | ✅ Resolved in 6fb9bcb (consolidated to E0160 + W0001) |
| 2-q2 | QE | cross-source-consistency | DESIGN-OBSERVABILITY header "(3)" vs 4-row table | Resolved | domain-raised | phase-1a (DESIGN-OBSERVABILITY) | ✅ Resolved in 6fb9bcb |
| 2-q3 | QE | error-code-ownership | VSDD-E0050 fires from two hooks without ownership rationale | Resolved | domain-raised | phase-1a (DESIGN-SCHEMA + DESIGN-VERIFICATION) | ✅ Resolved in 6fb9bcb (split into E0050 + E0051) |
| 2-q4 | QE | cross-source-consistency | Implicit always-on domain baseline undeclared | Resolved | domain-raised | phase-1a (DESIGN-METHODOLOGY + README) | ✅ Resolved in 6fb9bcb (always-on baseline section authored; operator-directive option (a)) |
| 2-q5 | QE | cross-source-consistency | Hook count ~18 declared but table has 17 rows | Resolved | domain-raised | phase-1a (DESIGN-VERIFICATION) | ✅ Resolved in 6fb9bcb (rows 18 + 19 added) |
| 2-s1 | Security | input-validation-completeness | Zero-axes project edge state undefined (composes with 2-q4) | Resolved | domain-raised | phase-1a (DESIGN-METHODOLOGY + README) | ✅ Resolved in 6fb9bcb (always-on baseline covers) |
| 2-s2 | Security | portability / threat-model-assumption | GitHub-platform assumption baked into adoption flow | Resolved | domain-raised | phase-1a (DESIGN-METHODOLOGY + README) | ✅ Resolved in 6fb9bcb (operator-directive option (a): v1 GitHub-only declared) |
| 2-s3 | Security | operator-experience-predictability | Init-order silence (vsdd init before crosslink init undeclared) | Resolved | domain-raised | phase-1a (README) | ✅ Resolved in 6fb9bcb |
| 2-s4 | Security | input-validation-cross-field | Plan auth + CI structurally permitted but operationally impossible | Resolved | domain-raised | phase-1a (DESIGN-SCHEMA + README) | ✅ Resolved in 6fb9bcb (auth_method.operator_local + .ci split + cross-field validation) |
| 2-s5 | Security | identity-attribution-discipline | Multi-machine operator identity continuity unaddressed | Deferred | domain-raised | v1+ pending earned-by-recurrence | ⏸️ Deferred per operator-directive |
| 2-s6 | Security | cross-source-consistency | Methodology spec drift between toolkit-canonical and project-local copies | Resolved | domain-raised | phase-1a (DESIGN-METHODOLOGY + README + DESIGN-VERIFICATION) | ✅ Resolved in 6fb9bcb (operator-directive option (a): version-pin + drift warning + W0200) |

**Round closure:** 12 Resolved + 1 Deferred-to-v1+ + 1 Accepted-with-rationale. Phase-5-MVR for this round reached.

---

## Round 1 — Phase 3 IAR (scope: full spec set, cluster-batched hybrid) (2026-05-27; cluster reviews complete)

**Scope:** All ~49 spec artifacts (Tier 1 cold-session cluster spawn for canonical docs + primers + domain prompts; Tier 2-3 inline pending).

**Method:** Hybrid per operator-directive — 4 cluster sub-agents spawned cold-context via Agent tool (Implementation + Architecture + Communication + Adversarial); Tier 2-3 inline pass from main session after cluster reviews land. Security + Red Team activated on operator-directive (credential-handling + supply-chain grounds); Data Engineer activated on operator-directive (persists-managed-schema-data axis).

**Active domain set (12 + 2 meta):** SE + QE + SA + SO + PE + PerfE (always-on baseline) + DR + TW + AIE (axis-activated) + Security + Red Team + DE (operator-directive); VSDD Methodology + Sanity Check (on-demand).

**Status:** Sub-agents running. Findings will be appended to this index post-cluster-completion + post-synthesis.

---

## Cross-cycle process gaps (the meta-findings)

These findings surface methodology-spirit drift in the very session that authors the methodology. Self-audit per the VSDD Methodology meta-domain.

| Process gap ID | Title | Surfaced by | Date | State |
|---|---|---|---|---|
| pg-1 | Phase 3 cycle 1 ran inline without cold-session declaration | Phase 5 round 1 (cycle 2) sycophancy-compensation discipline | 2026-05-27 | 📝 Documented retroactively in 9b85504 + cycle 1 row 1-process-gap-1 |
| pg-2 | Findings not filed to crosslink + FINDINGS-INDEX.md absent | Operator-directive 2026-05-27 ("Did the previous findings get added to crosslink?") | 2026-05-27 | 🟡 Resolving now via this index commit |
| pg-3 | Manual-mode operation never explicitly declared in DESIGN-METHODOLOGY or methodology.md | Operator-directive surfacing pg-2 | 2026-05-27 | 🔓 Open — route to Phase 4 → Phase 1a (methodology.md needs Manual-mode-vs-crosslink-mode operational declaration) |
| pg-4 | DESIGN-SCHEMA + DESIGN-OBSERVABILITY composed without Data Engineer despite heavy schema content | Operator-directive 2026-05-27 ("DE should be added to relevant designs") | 2026-05-27 | 🔓 Open — route to Phase 4 → DE inline review of DESIGN-SCHEMA + DESIGN-OBSERVABILITY |
| pg-5 | Domain prompts + supplements significantly under methodology line-count target (43-53 lines vs 80-150 target) | Tracked deviation in commit messages; flagged for adversarial assessment | 2026-05-27 | 🔓 Open — route pending leanness assessment + Phase 3 cluster-review findings on the same artifacts |
| pg-6 | Methodology silent on phase-skip prevention | Operator directive 2026-05-27 ('revisit methodology to close this loophole') | 2026-05-27 | 🔓 Open — methodology-amendment-class; agent went from Phase 1a authoring toward Phase 2b implementation without Phase 1c or Phase 2a |

---

## How to use this index

**To check current state of findings:**
1. Scan the "State" column of each cycle's table
2. ✅ = Resolved with citation; 🔓 = Open; ⏸️ = Deferred-with-trigger; 📋 = Accepted-with-rationale; 📝 = Documented; 🟡 = In-progress

**To file a new finding (manual mode):**
1. Author or append to the relevant cycle's review-log entry at `review-log/<date>-<domain-slug>.md`
2. Add a row to the cycle's table in this index OR add a new cycle section if this is a fresh cycle
3. Per primer 4 § Routing output: record finding ID, route, owning artifact, gate, sequencing

**To migrate to crosslink-mode tracking** (future):
1. Run `crosslink issue create` per Open finding with `-l review-finding -l phase:<N> -l route:phase-<dest>` labels
2. This index becomes a per-cycle archive surface; live state lives in crosslink

**Operational-mode declaration (per pg-3 routing):** vsdd-cli's own development operates in **manual mode** by default. The operator may transition to crosslink-mode tracking by running `crosslink issue list` to verify state + filing existing Open findings via `crosslink issue create`. Adopting projects choose their mode independently.

---

## Cross-references

- [methodology.md](./methodology.md) — canonical methodology spec
- [.claude/commands/vsdd-phase-3.md](./.claude/commands/vsdd-phase-3.md) — Phase 3 primer with cluster-batching shape + Exacting Mentor stance
- [.claude/commands/vsdd-phase-4.md](./.claude/commands/vsdd-phase-4.md) — Phase 4 routing primer
- [review-log/](./review-log/) — per-domain review entries (the authoring surface that feeds this index)
- Existing-suite reference: [`FINDINGS-INDEX.md`](https://github.com/magnificentlycursed/guild-portfolio/blob/main/vsdd-suite/suite-development/FINDINGS-INDEX.md) (governing G-### findings tracker for methodology evolution)
