---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-05-27
phase: phase-4
scope: Phase 4 Feedback Integration Loop routing of 58 findings from Phase 3 IAR Round 1 (scope: full spec set; 4-cluster cold-session hybrid spawn)
lens: operator-orchestrated routing per primer 4 — per-finding trace + earliest-phase-that-fixes-it assignment
source: domain-raised
session_note: cold-context — primer 4 loaded; routing applied per the routing table; sycophancy-resistance against "everything routes to Phase 2b" anti-pattern
model: claude-opus-4-7
execution_method: inline main session (operator-orchestrated; no domain composition required per primer 4 § Composition)
sycophancy_compensation: I authored the artifacts under review by the cluster sub-agents whose findings I am now routing. The temptation is to route findings to phases I can fix quickly (Phase 1a spec amendments) vs the phases that would actually close them (Phase 2a Red Gate tests would surface impl-f6 better than spec-revision; Phase 1c re-decomposition is more honest than spec-patching for arch-f9 / arch-f11). Cold-context resistance: every Phase 1a-routed finding got asked "would Phase 1c re-decomposition or Phase 2a Red Gate authoring fix this more durably?" before locking the route.
---

# Phase 4 Routing Review 1 — 2026-05-27

**Pre-phase composition declaration:**
```yaml
phase: phase-4
composed_domains: []
composition_mode: operator-orchestrated
operator_confirmation: confirmed (operator directive: "Phase 4 routing of 58 findings first")
cold_session_shape: N/A — Phase 4 is operator-orchestrated per primer 4 § Composition; no domain composition declared
declared_at: 2026-05-27
```

## Scope

58 findings from Phase 3 IAR Round 1 (scope: full spec set):
- 13 Implementation cluster (`review-log/2026-05-27-implementation-cluster.md`)
- 13 Architecture cluster (`review-log/2026-05-27-architecture-cluster.md`)
- 12 Communication cluster (`review-log/2026-05-27-communication-cluster.md`)
- 20 Adversarial cluster (`review-log/2026-05-27-adversarial-cluster.md`)

## Routing summary

| Route target | Count | Note |
|---|---|---|
| **Phase 1a (spec revision)** | 52 | Spec at spec-stage; majority of findings route here — appropriate for the phase, not Phase-2b-collapse anti-pattern |
| **Phase 2a (Red Gate / fixture authoring)** | 1 | adv-f10 (anonymization-hook fixture; tracks-to-implementation) |
| **Operator-directive (methodology-amendment)** | included in the 52 above; 7 carry `methodology-amendment` label | impl-f12, arch-f9, comm-f9, adv-f11, adv-f12, adv-f14, adv-f17 |
| **Deferred to v1+** | 1 | comm-f11 (multi-machine identity; constrained — Exit Signal cycle must retire or declare explicitly) |
| **Closed at routing time** | 4 metadata findings | impl-f13 Hallucinated; arch-f13 Accepted; adv-f18 Resolved-with-evidence; comm-f11 left open as deferred per above |

**Phase-2b-collapse anti-pattern check (per primer 4 § Primary failure mode):** 0 findings routed to Phase 2b. The toolkit has no implementation yet — Phase 2b is unreachable until Phase 1c + Phase 2a complete. Anti-pattern not triggered (good).

**Spec-defect bias check:** 52/58 findings = 90% route to Phase 1a. This is appropriate for spec-stage projects; the spec IS the artifact under review. A future Phase 3 round on the implementation (post Phase 2b) would shift the distribution toward Phase 2b. If Phase 3 Round 2 on the spec also routes 90% to Phase 1a, that signals the spec has not stabilized.

## Crosslink issue assignments

All 58 findings filed at crosslink issue IDs #69-126 (some duplicates from earlier filing attempts left gaps; see FINDINGS-INDEX.md). Per-cluster mapping:

- Implementation: #69-80 + (impl-f13 Hallucinated closed) — `crosslink issue list --label scope:full-spec-set --label domain:software-engineer` returns the SE subset
- Architecture: #81-93 (with arch-f13 Accepted-closed) — `--label domain:solution-architect`
- Communication: #95-106 (with comm-f11 deferred-open) — `--label domain:technical-writer` / `--label domain:security`
- Adversarial: #107-126 (with adv-f18 Resolved-closed) — `--label domain:documentation-reviewer` / `--label domain:vsdd-methodology` / `--label domain:red-team`

## Coordination clusters (Phase 1a work that benefits from coherent bundling)

Per primer 4 § Multi-phase routes + the Architecture cluster's coordination notes:

### Cluster I — Error catalog cleanup (single Phase 1a commit)
- impl-f1 (VSDD-E0021 collision) + adv-f6 (same) + adv-f19 (VSDD-W0080 collision) + impl-f2 (VSDD-E0220 conflicting semantics)
- One commit fixes 4 findings via the per-code-one-source contract restored.

### Cluster II — Naming-discipline sweep retro (single Phase 1a commit)
- adv-f2 (Tier A/B) + adv-f3 (Surface A/B/C/D in Phase 5 primer + QE prompt) + adv-f4 (Pattern A/B + Pillar N) + adv-f5 (Goal 1/2/3/4 — rationale-required)
- One commit fixes ~30+ sites; also re-validates the canonical-patterns.yaml registry against current state.

### Cluster III — Toolkit's own DESIGN.md + Cargo.toml + docs/dependencies/ bootstrap (single Phase 1a + 1c + 2a coordinated commit)
- arch-f9 (toolkit's own DESIGN.md missing) + adv-f7 (docs/dependencies/ doesn't exist) + arch-f11 (vsdd-core vs vsdd crate split framing)
- This is the "vsdd-cli dogfoods its own methodology" inflection — author the toolkit's DESIGN.md with proper Project intent + axes + behavioral contracts; then track 2a (Rust crate) becomes properly-scoped.

### Cluster IV — methodology.md content additions (single Phase 1a commit)
- comm-f1 (first-use expansion) + comm-f2 (vocab registry domain abbreviations) + comm-f4 (CHANGELOG discipline) + comm-f5 (MCP server) + comm-f6 (capture-source enumeration)
- methodology.md grows from 415 lines to ~500-550 lines covering the cited gaps.

### Cluster V — Threat model + Security operational runbooks (operator-directive coordination)
- comm-f7 (threat model) + adv-f17 (threat model never authored) + comm-f8 (credential-rotation runbook missing) + comm-f10 (OTEL_LOG_RAW_API_BODIES enforcement gap) + arch-f12 (anonymization-patterns trust-boundary inversion) + adv-f9 (bypass-marker single-maintainer degradation) + adv-f16 (vsdd-domains email registration)
- Substantial security-discipline authoring; could land as standalone THREAT-MODEL.md + runbooks/ directory.

### Cluster VI — Methodology amendments (single methodology-amendment commit)
- adv-f12 (earned-by-recurrence trigger violations) + adv-f11 (methodology_version cadence) + adv-f13 (Composed-domains trailer semantics) + adv-f14 (Phase 5 before Phase 3 — composes with pg-6 loophole) + impl-f12 (cluster-batching for low-axis) + comm-f9 (cluster-shape divergence between primer + dispatcher) + arch-f9 (toolkit's own DESIGN.md / DE composition) + arch-f8 (track 3h vs vsdd verify migrate v1+ scope)
- Substantial methodology amendments; could ship as v0.2.0 methodology-version bump.

### Cluster VII — CI workflow + supply-chain hardening (single Phase 1a + 2a commit)
- arch-f5 (cargo install bypasses pre-built-binary attestation) + arch-f6 (GitHub Actions tag-pinning vs SHA) + arch-f10 (Python hook subprocess cumulative cost) + arch-f7 (events.jsonl durability) + impl-f4 (subprocess ~950ms cost) + impl-f3 (validator wall-clock budget undeclared) + impl-f11 (dependency-approval PR-edit bypass)
- CI workflow templates rewrite + per-PR cost measurement; needs PE + Security composition.

### Cluster VIII — Hook + validator implementation prep (Phase 1c → 2a → 2b)
- impl-f5 (Manual-test falsifiability_check schema) + impl-f6 (consolidated hooks fixture per-rule) + impl-f7 (purity boundary uncatalogued) + impl-f8 (vsdd verify exit codes) + impl-f10 (unknown hook-id handling) + adv-f10 (anonymization fixture; Phase 2a route)
- These are the implementation prerequisites; Phase 1c decomposes the hook implementation layer; Phase 2a authors fixtures; Phase 2b implements.

## Sequencing recommendation (per the operator's prior directive "proper Phase 1c → 2a → 2b for hooks")

1. **Cluster II (naming-discipline retro)** — small, mechanical, blocking nothing; can land any time. Recommend first because it cleans the canonical artifacts the next clusters will reference.
2. **Cluster I (error catalog cleanup)** — small, mechanical; restores the per-code-one-source contract before Phase 2a fixtures land.
3. **Cluster IV (methodology.md content additions)** — methodology spec gaps; bounded scope.
4. **Cluster VI (methodology amendments)** — substantive; needs SO disposition + earned-by-recurrence-evidence trail for each amendment. May warrant a fresh Phase 3 round before locking.
5. **Cluster III (toolkit DESIGN.md + Cargo.toml bootstrap)** — opens Phase 1c proper for hook implementation.
6. **Cluster VIII (hook + validator implementation prep)** — Phase 1c → 2a → 2b sequence per primer 4 § After Phase 4.
7. **Cluster V (threat model + runbooks)** — can run parallel with Cluster VIII (independent concerns).
8. **Cluster VII (CI workflow hardening)** — can run after Cluster III (which establishes Cargo.toml).

## Composes-with (cross-finding coordination)

- Cluster I (error catalog cleanup) composes with Cluster II (naming sweep) — both touch DESIGN-VERIFICATION
- Cluster VI (methodology amendments) composes with pg-6 (phase-skip loophole already filed as critical-priority process-gap)
- Cluster III (DESIGN.md + Cargo.toml bootstrap) composes with pg-4 (DE composition missing from prior DESIGN docs)
- Cluster VIII (hook implementation prep) composes with pg-5 (leanness — domain prompts need expansion) AND pg-6 (the phase-skip loophole prevention)

## Completion criteria check (per primer 4 § Completion criteria)

1. ✅ Every real finding from Phase 3 has a recorded route — 58/58 filed with route: label
2. 🟡 Every route names the gate at each phase — partial; cluster-level gates named above; per-finding gates implicit in route:phase-1a / route:phase-2a labels
3. 🟡 Every blocking relationship recorded — cluster-level coordination above names blocking; per-finding `crosslink issue block` not yet applied
4. ✅ Suite findings filed in crosslink (not collapsed into project-phase routes); FINDINGS-INDEX.md aggregates
5. ✅ Phase-2b-collapse anti-pattern not triggered (0/58 to Phase 2b; appropriate for spec-stage)

Phase 4 routing exit signal: **PhaseExited{phase: phase-4, exit_status: complete, scope: full-spec-set, routed_count: 58}**.

## After Phase 4 (per primer 4 § After Phase 4)

The next pass begins. Per the sequencing recommendation above, the recommended next entry is **Phase 1a primer** for Cluster II (naming-discipline retro) — smallest unblock-everything-else surface. The operator-directed alternative is **Phase 1c primer** for hook implementation layer decomposition (Cluster III + Cluster VIII).

## Coordination

- Routed findings will spawn per-cluster Phase 1a commits OR Phase 1c → 2a → 2b cycles per the sequencing above. Each commit references the originating crosslink issue IDs in its body.
- FINDINGS-INDEX.md gets updated post-each-commit with state transitions (Open → Resolved with citation).
- pg-2 (manual-mode → crosslink-mode cutover) is closed; pg-6 (phase-skip loophole) is the next methodology-amendment surface.
