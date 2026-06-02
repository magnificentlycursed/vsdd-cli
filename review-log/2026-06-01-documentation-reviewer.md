---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-06-01
phase: phase-3
scope: >-
  Adversarial sweep across the canonical 5-doc corpus (methodology.md + DESIGN-METHODOLOGY
  + DESIGN-SCHEMA + DESIGN-OBSERVABILITY + DESIGN-VERIFICATION + README) + 3 registries
  (vocabulary.yaml + canonical-patterns.yaml + anonymization-patterns.yaml). Subject —
  ALL load-bearing concept-anchors, coinages, namings, vocabulary. Lens-mix from DR + TW
  + SO + AIE + VSDD-methodology + Sanity-Check.
lens: 5-lens application weighted to Consistency (5) + Maintainability (4) + Usability (3). Attacker (1) + Edge cases (2) lenses light-touch for a prose-surface sweep.
source: director-raised
session_note: >-
  Inline single-session composition; NOT the cluster-batched cold-session shape Phase 3
  ordinarily requires. Operator-directive — "Use all appropriate domains... You can run
  this inline." Methodology-deviation declared in pre-phase composition block below; raised
  as a VSDD-methodology meta finding (F12) for honest audit-trail.
model: claude-opus-4-7
execution_method: >-
  inline main session (single-agent multi-domain composition; six domain primers loaded
  sequentially — vsdd-phase-3 + DR + TW + SO + AIE + vsdd-methodology + sanity-check)
sycophancy_compensation: >-
  I (the inline-running reviewer identity) am also the same identity that authored the 5
  canonical docs across the recent commit chain (5ccf740 → 1229cda). The bias is to read
  my own coinages as load-bearing-by-construction and to dismiss novel-term proliferation
  as "earned." The compensation — every concept-anchor finding is grounded in a mechanical
  citation (line number; count; broken-link path) so the finding holds whether or not the
  reviewer accepts the author's intent. Where rationale is judgment-only (e.g., F7 Goal N),
  the finding Raises to SO with explicit dismissal_rationale-required disposition rather
  than self-validating.
filename_note: >-
  This review log is filed under the documentation-reviewer domain slug per the canonical
  `<date>-<domain-slug>.md` convention. The composition spans 6 domains; DR is the primary
  author-identity domain (7 of 15 findings rest on DR-dim citations — cold-context
  discoverability, cross-reference resolution, naming-discipline cold-read, three-audience
  effectiveness). Findings routing to non-DR domains carry their domain label in the
  per-finding heading.
---

# Documentation Reviewer Review 1 — 2026-06-01

**Phase 3 cycle round:** 1 (this is the opening round of a new IAR cycle scoped to the naming surface; not a continuation of the Phase 3 round-1 cluster reviews that landed in commit 538677a)

---

## Pre-phase composition declaration

```yaml
phase: phase-3
composed_domains: [documentation-reviewer, technical-writer, solution-owner, ai-engineer, vsdd-methodology, sanity-check]
composition_mode: inline-single-agent-multi-domain
memory_isolation: NONE (single main-session; no worktree isolation; no --no-memory flag)
operator_confirmation: confirmed (operator-directive: "Use all appropriate domains. I'd like to evaluate all load-bearing concept-anchors, coinages, namings, and vocabulary" + "You can run this inline")
cluster_shape: deviation-from-4-cluster-default (operator-directive-justified; flagged in F12)
declared_at: 2026-06-01T17:30Z
methodology_deviation: |
  Phase 3 primer (vsdd-phase-3.md) declares cluster-batched cold-session reviewer mode as the
  canonical shape. This cycle runs inline single-agent multi-domain composition by operator
  directive. The deviation costs cold-session-isolation discipline + adversarial-pair separation
  (DR ↔ TW co-located; SO + vsdd-methodology co-located; Red Team absent). Trade-off accepted by
  operator-directive for a bounded-surface sweep where the findings are evidence-grounded
  (line counts; broken links; grep-evidenced site enumeration) rather than judgment-heavy. F12
  formalizes the deviation as a recurring-pattern finding for the vsdd-methodology meta-domain.
sycophancy_compensation: see frontmatter sycophancy_compensation field
```

---

## Scope

The naming surface enumerated for this review:

**Load-bearing concept-anchors (vocabulary-registered):** VSDD, MVR, Exit Signal, Exacting Mentor, IAR, Red Gate, capture-source, Raise to SO, phase-domain composition, always-on baseline (10 terms in `templates/registry/vocabulary.yaml`).

**Load-bearing concept-anchors (NOT registered; spec-asserted):** bypass-marker mechanism, sycophancy compensation, narrative-preservation, stability commitment, earned-by-recurrence trigger, organizational scaffolding, concept-anchor, two-audience principle, three pillars / observability signal surfaces, per-feature axes, axis-driven activation, Goal 1–4 (the four governing design goals), shift-left, validator pair, dimension, classification universe, role domain, meta domain, cold-session reviewer mode, skill mode, Mentor voice, Formal voice, tone-flex policy, methodology-stabilization, operator-directive, methodology-evolution coherence, methodology-spirit, methodology-amendment governance, status-tier (candidate / accepted / deprecated), 13 artifact classes, 18 methodology event variants, ~19 methodology hooks, four-dimensional convergence, four-cluster default, adversarial-pair separation invariant, two cooperating audit-trail layers, EventEnvelope, anchor-ID, 7-value capture-source enum, dual-mode validator, primer (phase primer vs domain prompt), domain primer / prompt, two-audience principle, three-audience principle (suite developers / users / agents), substrate, "the rebuild," prose-surface composition discipline, layer-cycle PR discipline (~50+ unregistered terms).

**Coinages (Phase 3 primer / DESIGN-METHODOLOGY cluster-batching shape):** Implementation cluster, Architecture cluster, Communication cluster, Adversarial cluster (4 cluster names from vsdd-phase-3.md; load-bearing for Phase 3 cluster-batching but unregistered).

**Letter-label anti-pattern surviving sites on main:** Tier A/B (13 sites); Pattern B (2 sites); Pillar / Three pillars (7+ sites); Surface A/B/C/D (1 site — teaching reference, acceptable); Goal 1/2/3/4 (25 sites — deferred per open issue #111).

**Registries:** `templates/registry/vocabulary.yaml` (10 terms; managed-section + operator_extensions); `canonical-patterns.yaml` (7 letter-label patterns; 5 acceptable patterns; 2 synthesis-review-citation patterns; 4 acceptable evidence-reference patterns); `anonymization-patterns.yaml` (separate surface; not reviewed this round).

---

## Findings

### Finding 1 — Broken cross-reference: `methodology.md § Naming + coinage governance` cited from 3 surfaces, section does not exist on main (Dim: cross-reference resolution; DR) — Open

**Evidence:**
- `templates/registry/vocabulary.yaml:5`: comment cites "Per methodology.md § Naming + coinage governance + DESIGN-VERIFICATION § check-naming-discipline.py"
- `templates/registry/canonical-patterns.yaml:5`: same comment
- `DESIGN-VERIFICATION.md:200`: "Per the naming + coinage governance discipline:" (ambiguous referent — could be reading as the DESIGN-METHODOLOGY section)
- `methodology.md` section list (lines 27–407): Opening + scope / Four governing design goals / Phase taxonomy / Phase-domain composition / Adversarial review stance / Domain set / Per-feature axes / Forward-only disciplines / Bypass-marker mechanism / Two-audience principle / Two cooperating audit-trail layers / Schema versioning / MVR and Exit Signal convergence / Auth method / Domain change authority / Closing — **no `## Naming + coinage governance` heading**
- The actual `## Naming + coinage governance` section lives at `DESIGN-METHODOLOGY.md:796`. The registry comments cite the wrong file.

**Why it matters:** The two managed-section registries are the operator-facing canonical artifacts that survive `vsdd init` deployment. Their first-line comments are the cold reader's entrypoint for "where do I learn the governance rule?" They point at a non-existent anchor. Fires `VSDD-E0010: cross-reference-broken` (per DESIGN-SCHEMA error catalog, candidate).

**Routing:** Phase 4 → Phase 1a (registry comment surface revision). Two-line mechanical fix: change `methodology.md` to `DESIGN-METHODOLOGY.md` in both registry files. Composes with F2.

**Classification:** Resolved-pending (mechanical fix).

---

### Finding 2 — Vocabulary registry coverage gap: 10 registered terms vs. ~50+ spec-asserted load-bearing concept-anchors (Dim: vocabulary registry conformance; TW) — Open

**Evidence:**
- `templates/registry/vocabulary.yaml` `terms:` list: VSDD, MVR, Exit Signal, Exacting Mentor, IAR, Red Gate, capture-source, Raise to SO, phase-domain composition, always-on baseline (10 entries).
- Methodology spec (scope section above) enumerates ~50+ load-bearing concept-anchors that are asserted-but-not-registered. Sample: bypass-marker, sycophancy compensation, narrative-preservation, stability commitment, earned-by-recurrence, two-audience principle, three pillars, shift-left, validator pair, dimension, classification universe, role domain, meta domain, cold-session reviewer mode, skill mode, Mentor voice, Formal voice, tone-flex policy, EventEnvelope, anchor-ID, dual-mode validator, primer, substrate.
- `DESIGN-METHODOLOGY.md:806`: "The `check-naming-discipline.py` hook (consolidated; covers letter-labels + suite-internal-terminology + vocabulary-registry conformance) scans documents for ... novel-term-without-registry-entry (single-recurrence: candidate code)."

**The fuzz signal:** When `check-naming-discipline.py` ships with the third rule active, it will fire `VSDD-W0001` (or the candidate-tier successor) against ~50+ terms in the canonical docs themselves. The methodology spec violates the registry-conformance discipline it asserts. Two resolution paths:
1. **Bulk-register the spec-asserted terms** in `vocabulary.yaml` before the hook activates the novel-term rule, OR
2. **Narrow the novel-term rule** to operator-project-local prose (excluding the toolkit-canonical docs), OR
3. **Defer novel-term-rule activation** to a post-stability-commitment phase + ship the hook with rule 3 dormant.

**Routing:** Phase 4 → Raise to SO (methodology-amendment governance). SO disposition required: which of the three resolution paths is canonical. Composes with F1 (registry comments first need to point at the right doc), F11 (recursive violation), F15 (registry-vs-spec asymmetry).

**Classification:** Deferred-pending-SO.

---

### Finding 3 — Letter-label anti-pattern sweep incomplete on `main`: 22+ surviving sites (Dim: spec-vs-implementation consistency; SO + vsdd-methodology) — Open

**Evidence:**
- Tier A/B: 13 sites (DESIGN-METHODOLOGY:851, 853, 855, 887, 890, 891, 941, 942; DESIGN-VERIFICATION:679, 872; README:868, 869) — excluding 2 teaching-reference sites at DESIGN-METHODOLOGY:824 + :840.
- Pattern B: 2 sites (DESIGN-OBSERVABILITY:492 heading "### Auto-generated PR body (Pattern B)"; DESIGN-METHODOLOGY:218 "Manual-test checklist (Pattern B auto-generation)").
- Pillar / Three pillars (excluding `Pillar \d` which IS in the regex): 7+ sites — `methodology.md:412` "three pillars" in DESIGN-OBSERVABILITY cross-reference; `README.md:316` "### Three pillars" heading + `:318` "| Pillar | Source | ..." table column; `DESIGN-OBSERVABILITY.md:3` "three-pillars + dashboard ladder"; `:30` "Three pillars (logs / metrics / traces)"; `:309` "## Three pillars"; `:311` "| Pillar | Source | ..." column header; `:610` "Three pillars + dashboard ladder + FinOps applied to IAR".
- The companion sweep on `wip-archive` (commits `82a0ccb` + `ad6ee80`) DID resolve these. The `wip-archive` branch carries the sweep + the extended Pillar regex; `main` has neither.

**Why it matters:** The methodology spec asserts at multiple layers that the anti-pattern is mechanically enforced (`check-naming-discipline.py` fires `VSDD-E0160`). The registry's regex on `main` is `\bPillar \d\b` — which DOES NOT MATCH bare "Pillar" or "Three pillars." On `main`, both the prose corpus AND the regex are incomplete. The toolkit cannot dogfood its own discipline.

**Routing:** Phase 4 → Phase 1a (spec revision; mechanical sweep) + Phase 1b (extend canonical-patterns.yaml Pillar regex). Two options: (a) port `wip-archive` commits `82a0ccb` + `ad6ee80` + `1229cda` forward to `main`; (b) re-author the sweep clean on `main`. Composes with F4 (cleanroom-restart branch state).

**Classification:** Resolved-pending (port-forward or re-author; SO disposition on path).

---

### Finding 4 — Cleanroom-restart branch state surfaces methodology-evolution coherence gap (Dim: methodology-evolution coherence; vsdd-methodology) — Open

**Evidence:**
- `main` HEAD: `2f2f7c2` (Phase 1a: 14 supplements + CI workflow templates).
- `wip-archive` HEAD: `7be9105` (vdd-iar-alignment Review 1) — 7 commits ahead of `main`.
- `wip-archive` includes the naming-discipline sweep (`82a0ccb` + `ad6ee80`), the methodology.md content additions (`1229cda` — added Conventions section + capture-source enum + MCP server section + CHANGELOG discipline), the error-catalog cleanup (`c75a4c2`), and the "WIP session 2026-05-28 — preserved for cleanroom restart" marker (`76ab1eb`).
- `main` has none of those resolutions.
- The methodology declares (`methodology.md:235` + `README.md:712–723`) that pre-stability-commitment history is malleable and only post-stability-commitment is forward-only. `main` is pre-stability — so a force-reset is permitted in principle. But the resolutions on `wip-archive` close 8+ routed Phase 4 findings; discarding them silently is methodology-evolution-coherence drift.

**Why it matters:** The cleanroom-restart marker (`76ab1eb`) is undocumented in the canonical docs. A cold reader of `main` cannot derive that the naming-discipline sweep on `wip-archive` was a real resolution path that needs to be re-merged. The forward-only-discipline section names four stability-commitment triggers but does NOT name a "cleanroom restart" pattern as a permitted operator-mode.

**Routing:** Phase 4 → Raise to SO. Two paths: (a) port-forward the resolutions + retire `wip-archive`; (b) document the cleanroom-restart pattern as a forward-only-discipline-section amendment + reconcile main. Composes with F3.

**Classification:** Deferred-pending-SO.

---

### Finding 5 — "rebuild" is suite-internal terminology in operator-facing prose (Dim: cold-context discoverability; DR) — Open

**Evidence:**
- `README.md:113` "The rebuild's product is:"
- `README.md:122` "**The rebuild's own development as canonical dogfood.** The rebuild applies its own methodology to itself"
- `README.md:569` "The rebuild's adversarial reviewer adopts an **Exacting Mentor** stance"
- `README.md:837` "Operators of the existing suite are not auto-migrated. The rebuild's adoption is a new-project decision."
- `README.md:872` table footer "Goal 4 end-to-end demonstration via rebuild's own CI"
- `DESIGN-METHODOLOGY.md:459` table cell "The rebuild's own methodology spec + DESIGN docs"
- `DESIGN-METHODOLOGY.md:948` same as README:872 footer

**Why it matters:** A cold reader landing on README.md cannot derive what "the rebuild" denotes. The toolkit is called "vsdd" / "the toolkit" (referenced ~50+ times). "The rebuild" is an undefined synonym that signals suite-internal provenance — the toolkit was authored as a rebuild of a prior suite (existing-suite / bookmark-cli-manual). The cold reader sees a load-bearing referent ("the rebuild's adversarial reviewer adopts...") and has nowhere to ground it. Fires `VSDD-W0030: stale-claim-suspicion` (a referent without a definition is a stale-claim shape) + the candidate `VSDD-W0001: suite-internal-terminology-bleed`.

**Routing:** Phase 4 → Phase 1a (prose revision). Two paths: (a) replace "the rebuild" with "the toolkit" / "vsdd-cli" throughout; (b) introduce a single explicit definition early in README ("This toolkit is referred to internally as 'the rebuild' because it re-implements ...") + leave subsequent uses. Path (a) reduces vocabulary; path (b) preserves audit-trail. SO + DR + TW co-author on resolution.

**Classification:** Resolved-pending (mechanical sweep).

---

### Finding 6 — Quantitative claim drift: hook count cited as ~18, ~19, 19, "= ~24 hooks total" across 6 sites (Dim: stale-claim suspicion; TW) — Open

**Evidence:**
- `README.md:13` "~18 methodology hooks + 13 schema-validated artifact classes"
- `methodology.md:413` "~19 methodology hooks + CI workflow templates"
- `DESIGN-METHODOLOGY.md:859` "Deploys ~19 methodology hooks in `.claude/hooks/`"
- `DESIGN-METHODOLOGY.md:964` "per-hook deployment matrix (~19 hooks ...)"
- `DESIGN-VERIFICATION.md:29` "~19 methodology hooks (Python operator-side; Rust mirror CI-side)"
- `DESIGN-VERIFICATION.md:170` "## Per-hook deployment matrix (~19 hooks)"
- `DESIGN-VERIFICATION.md:697` "# ... (19 hooks total)"
- `DESIGN-VERIFICATION.md:745` "19 methodology hooks (Python thin wrappers)"
- `DESIGN-VERIFICATION.md:899` "19 methodology hooks composing with crosslink's 5 = ~24 hooks total"

**Why it matters:** Three distinct quantities visible (~18, ~19, 19, ~24-total). One uses approximate operator (~), one uses exact (19), one is a sum (~24-total). Fires `VSDD-W0030: stale-claim-suspicion` against the README site (~18 conflicts with the verification-doc 19). DR cold-read fails: the cold reader cannot derive whether the toolkit ships 18, 19, or 24 hooks. The README is the document an adopting-project operator reads first; its ~18 is the stalest figure.

**Routing:** Phase 4 → Phase 1a (TW + DR co-authored sweep). Decide on canonical count (almost certainly 19); update README + methodology.md cross-reference. Note also that DESIGN-VERIFICATION:899's "= ~24 hooks total" is misleading without context (the +5 is from crosslink, not vsdd-canonical).

**Classification:** Resolved-pending (mechanical sweep; canonical count = 19).

---

### Finding 7 — "Goal 1 / 2 / 3 / 4" letter-label-adjacent in 25 sites; rationale-required disposition deferred indefinitely (Dim: load-bearing-concept-anchor vs. letter-label tension; SO) — Open / Raise-to-SO

**Evidence:**
- Site counts: methodology.md (4) + README.md (9) + DESIGN-METHODOLOGY.md (7) + DESIGN-OBSERVABILITY.md (3) + DESIGN-VERIFICATION.md (2) + DESIGN-SCHEMA.md (0) = 25 total.
- Open issue #111 (adv-f5) deferred this in commit `82a0ccb` (sweep-retro) with "DEFERRED (adv-f5; rationale-required per operator-directive)" — but no rationale has been recorded; the issue has been open since 2026-05-28.
- The methodology asserts (`DESIGN-METHODOLOGY.md:824`) that author-introduced cognitive-scaffolding terms (Tier A/B/C; Pillar N; Mechanism A/B/C) are NOT methodology terms and are "replaced with descriptive prose in canonical docs." The Goal N pattern is structurally identical: an organizational-scaffolding prefix attached to descriptive content ("Goal 1 — Absorbability-ready patterns"). The methodology's own discipline, applied recursively, says Goal N should be replaced by the descriptive name.
- BUT: the methodology spec also asserts that each governing goal is referenced by its number across other docs ("Goal 2 operationalization", "Goal 3 flagship", "Goal 4 CI/CD shift-left") — the number IS doing concept-anchor work in the sense that it provides a stable referent for cross-doc citation.

**The tension:** Goal N is BOTH letter-label-adjacent (matches the anti-pattern's structural shape: prefix-word + index, no concept-word in the identifier) AND load-bearing-concept-anchor (the four governing goals are a methodology-spec foundation, and `Goal 2 operationalization` is a real cross-doc invariant).

**Resolution options (Raise to SO):**
1. **Accept Goal N as load-bearing-concept-anchor** — add the rationale to `DESIGN-METHODOLOGY.md § Naming + coinage governance` as a documented carve-out from the anti-pattern. The 4 goals get vocabulary-registry entries. This formalizes what the deferred-pending-rationale disposition tacitly is.
2. **Rename to descriptive concept-anchors** — replace "Goal 1" with "Absorbability goal," "Goal 2" with "Auditability goal" (or "machine-enforceability goal"), "Goal 3" with "Observability goal," "Goal 4" with "Shift-left goal." 25-site mechanical sweep + cross-doc citation updates.
3. **Hybrid** — keep "Goal N" in operator-facing prose (README) where the operator's mental model maps "Goal 1 + Goal 2 + Goal 3 + Goal 4" to a stable four-tuple; replace "Goal N" with descriptive form in DESIGN docs where each goal is operationalized in isolation.

**Routing:** Phase 4 → Raise to SO. `OperatorDirectiveApplied{directive: spec-contract-amended OR spec-contract-amendment-rejected, rationale: <text>}` required. Closes #111.

**Classification:** Deferred-pending-SO.

---

### Finding 8 — "discipline" overloaded across 141 sites with 12+ semantic categories (Dim: terminology consistency; DR + TW) — Open

**Evidence:**
- Total usage: methodology.md (12) + DESIGN-METHODOLOGY.md (46) + DESIGN-SCHEMA.md (15) + DESIGN-OBSERVABILITY.md (12) + DESIGN-VERIFICATION.md (18) + README.md (38) = 141.
- Semantic categories observed (non-exhaustive):
  - **data discipline** — event-log append-only (`methodology.md:233`)
  - **prose discipline** / **documentation narrative-preservation** (`methodology.md:235`)
  - **authoring discipline** — TW + DR co-authorship (`DESIGN-METHODOLOGY.md`)
  - **schema discipline** — per-artifact-class schemas (`DESIGN-SCHEMA.md:838`)
  - **hook discipline** / **per-hook test discipline**
  - **audit-trail discipline** / **two-cooperating-audit-trail-layers discipline**
  - **sycophancy compensation discipline** (`methodology.md:158`)
  - **naming + coinage governance discipline** (`DESIGN-VERIFICATION.md:200`)
  - **cold-session discipline** / **memory isolation discipline**
  - **classification discipline** / **routing discipline** / **Raise-to-SO routing discipline**
  - **methodology-spirit discipline** (vsdd-methodology meta dim 2)
  - **shift-left discipline** (`DESIGN-VERIFICATION.md:679`)
  - **status-tier discipline** (`DESIGN-METHODOLOGY.md:940`)
  - **forward-only discipline** / **narrative-preservation discipline**

**Why it matters:** The word is the most-overloaded noun in the canonical corpus. A cold reader encountering "per the X discipline" cannot disambiguate without retrieving the local context. The mechanical-enforcement claim ("methodology rule has a hook OR schema validator OR crosslink workflow check") becomes hard to validate when "discipline" is the binding term that connects rule → mechanism, and the term is reused with different semantic weights. Per the DR dim "Cold-context discoverability."

**Why it's a finding rather than acceptable repetition:** The methodology's `DESIGN-METHODOLOGY.md § Naming + coinage governance` explicitly proscribes "Pattern A/B" / "Tier A/B" / "Pillar N" as scaffolding shorthand. "Discipline" is a different failure mode — semantic-overload rather than letter-label — but functionally the same: a cold reader cannot derive the referent from the identifier alone.

**Routing:** Phase 4 → Phase 1a (sweep). Two paths: (a) introduce a typology — "X discipline" where X is one of {data, prose, authoring, schema, hook, audit-trail, sycophancy, naming, cold-session, classification, routing, forward-only} — codified in vocabulary registry, with each canonical use cross-referenced to its category; (b) replace "discipline" with category-specific words in low-traffic sites (e.g., "narrative-preservation rule"). Likely combination: do both.

**Classification:** Resolved-pending (sweep). Composes with F2 (registry-coverage-gap).

---

### Finding 9 — `substrate` is an unregistered load-bearing coinage with 11+ uses across docs (Dim: vocabulary-registry conformance; DR + TW) — Open

**Evidence:**
- `methodology.md:6` frontmatter: `substrate_anchors: [https://github.com/forecast-bio/crosslink, https://code.claude.com/docs/en/agent-sdk/overview]` — load-bearing schema field.
- `methodology.md:412` "MCP server brings substrate-doc + methodology lookup into every Claude Code session" + `:414` "the observability + execution substrate the toolkit composes against" + `:415` "the operational substrate authored by dollspace."
- `DESIGN-METHODOLOGY.md`: `vsdd init --check pre-flight validates substrate prerequisites` (`:851`)
- `DESIGN-OBSERVABILITY.md`: `:610` "substrate-doc + methodology lookup"
- `templates/registry/canonical-patterns.yaml` comments reference "substrate" implicitly
- The user's own opening message in this session ("substrate-grounded naming") uses substrate as a load-bearing concept-anchor.

**Why it matters:** `substrate` denotes "the underlying platform / runtime / tool we compose against" — crosslink + Agent SDK + (implicitly) Claude Code itself + (implicitly) Anthropic API. The methodology composes-against-substrate as a foundational design discipline (Goal 1's absorbability is substrate-relative; Goal 3's observability composes against the substrate's OTel export). But:
- No vocabulary registry entry
- No definition in methodology.md
- The cold reader has to infer the boundary (crosslink IS substrate; the toolkit's own code is NOT substrate; the operator's project code IS substrate from the toolkit's view? — undefined)

**Routing:** Phase 4 → Phase 1a (introduce substrate as a registered term in vocabulary.yaml + add a one-paragraph definition to methodology.md's opening section).

**Classification:** Resolved-pending.

---

### Finding 10 — MVR is used in 3 nested scopes; vocabulary entry names only the per-round scope (Dim: terminology consistency; sanity-check) — Open

**Evidence:**
- `vocabulary.yaml:23-28`: definition reads "Maximum Viable Refinement; the per-round closure signal when all active domains produce only Hallucinated findings."
- `methodology.md:335`: "Maximum Viable Refinement (MVR) is the per-round closure signal: an IAR cycle (Phase 3) reaches implementation-MVR when all active domains produced only Hallucinated findings (or no findings) on the final round. **Per-domain MVR feeds the layer-MVR signal which gates layer-close.**"
- `methodology.md:337`: "Exit Signal is the project-terminal four-dimensional convergence (Phase 6): **Spec MVR** (cold SO review across final layers produced no Phase 1a/1b-routed findings); **Test MVR** (Phase 5 Mutation Testing per layer with per-mutant disposition); **Implementation MVR** (Phase 3 final round per active domain produced only Hallucinated findings); **Formal-verification MVR** (Phase 5 Proof Execution harnesses each have recorded outcomes OR explicitly declared not-applicable with rationale)."

**The fuzz signal:** MVR is denoting (a) per-round closure (Phase 3 cycle round), (b) per-layer closure (layer-MVR gates layer-close), and (c) per-dimension convergence (Spec MVR / Test MVR / Implementation MVR / Formal-verification MVR feed Exit Signal). Three nested scopes; cold reader has to infer which one is being referenced at each site. The vocabulary registry definition names only scope (a).

**Why it matters:** This is a **load-bearing concept-anchor** by the user's own framing. The methodology's terminal-convergence shape rests on the four dimensions reaching MVR independently and then converging. If MVR-the-per-round-signal and MVR-the-per-dimension-signal are the same word with different referents, the convergence-attestation discipline gets ambiguous. Per the sanity-check meta-domain's "Rubber-ducking discipline" dim: rubber-duck the question "what does it mean for the toolkit to reach MVR?" — answer is undefined without qualifier.

**Routing:** Phase 4 → Raise to SO (vocabulary amendment). Options: (a) extend vocabulary.yaml MVR entry to enumerate the three scopes; (b) introduce three distinct registered terms (round-MVR / layer-MVR / dimension-MVR) with cross-references; (c) keep MVR as the umbrella + always qualify ("per-round MVR" / "layer MVR" / "Spec MVR").

**Classification:** Deferred-pending-SO.

---

### Finding 11 — Methodology spec violates its own self-declared concept-anchor invariant (Dim: methodology-spirit adherence; vsdd-methodology) — Open

**Evidence:**
- `methodology.md:31` asserts: "Every architectural decision in the DESIGN docs surfaces here in at least one section; **every event variant and artifact class declared by the toolkit is named here**; every phase has a corresponding primer; every active domain has a corresponding prompt."
- Actual state: methodology.md names 0 of the 18 event variants explicitly; methodology.md names 0 of the 13 artifact classes explicitly. The numbers (18 event variants, 13 artifact classes) are cited but the variant names + class names are NOT enumerated in methodology.md.
- The 18 event variants ARE enumerated in DESIGN-OBSERVABILITY.md; the 13 artifact classes ARE enumerated in DESIGN-SCHEMA.md. But the invariant says they're also named in methodology.md.
- The `check-methodology-semantics.py` hook is asserted (`methodology.md:31`) to "mechanically validate these invariants." If the hook were live and the invariant were the literal "named here," the hook would fire on every commit of methodology.md.

**Why it matters:** Recursive self-violation pattern. The methodology spec is the canonical referent for the discipline; the discipline declares "X is named in methodology.md"; methodology.md does not name X. Per the vsdd-methodology meta-domain's dim "Methodology-spirit adherence" — the spec follows the letter (it cites counts: 18 variants, 13 classes) but violates the spirit (the load-bearing concept-anchors aren't named in the spec, defeating the dual-audience-readable principle).

**Routing:** Phase 4 → Raise to SO. Options: (a) amend methodology.md to enumerate the 18 variants + 13 classes (likely +30–40 lines); (b) amend the invariant statement to "every variant + class is cross-referenced from methodology.md to its canonical DESIGN-doc home" — softer assertion that the cross-reference resolution exists; (c) split the difference — methodology.md names the canonical 5–6 most load-bearing variants/classes inline + cross-references the rest.

**Classification:** Deferred-pending-SO.

---

### Finding 12 — Phase 3 cluster-batched cold-session discipline waived for this cycle; deviation undocumented in canonical-patterns or methodology-deviation registry (Dim: methodology-spirit adherence + sycophancy compensation; vsdd-methodology) — Open

**Evidence:**
- Phase 3 primer (`vsdd-phase-3.md`) declares: "This is the only phase that runs domains in cold-session reviewer mode, NOT skill mode... Per the cluster-batching shape, the active domain set ... spawns into 4 clusters with adversarial-pair separation... Adversarial-pair separation invariant: Security ↔ Red Team on different clusters; TW ↔ DR on different clusters."
- This cycle (the present review) runs inline single-agent multi-domain composition: DR + TW co-located (violates DR ↔ TW separation invariant), SO + vsdd-methodology co-located, Red Team absent, Security absent.
- Operator-directive: "You can run this inline" — explicit authorization for the deviation.
- BUT: the methodology has no `inline-single-agent-multi-domain` composition_mode enumerated as a valid Phase 3 shape. The valid shapes per the primer are `4-cluster-default` and `per-domain` (the high-stakes 18-agent alternative).
- The Adversarial-cluster spec (DR + AIE + SO + vsdd-methodology + sanity-check + UX + Red Team) is operationally what got composed here MINUS UX + Red Team + the cluster-separation; PLUS TW pulled in from Communication cluster.

**Why it matters:** Per vsdd-methodology meta-domain dim 2 ("methodology-spirit adherence"): a cycle that follows the letter of Phase 3 (right phase, right primer loaded, findings produced) but violates the spirit (adversarial-pair separation; cold-session-isolation) IS the failure mode the meta-domain catches. The operator-directive authorizes the deviation; the deviation does not retroactively become methodology-compliant.

**Recurrence evidence:** This is the SECOND inline-Phase-3 cycle in the toolkit's history (the first being the inline Phase 5 round-1 in `9b85504` — explicitly noted as inline-clustered with SA + QE in the security review-log frontmatter at `:11`). Two recurrences → earned-by-recurrence trigger fires.

**Routing:** Phase 4 → Raise to SO + Raise to VSDD-methodology meta. Options:
1. **Amend the Phase 3 primer** to enumerate `inline-single-agent-multi-domain` as a third valid composition_mode (with explicit cost-discipline: which dims it sacrifices, when it's appropriate). Earned-by-recurrence trigger has now fired (2 cases); amendment is in-scope.
2. **Treat inline composition as a methodology deviation**, require explicit `OperatorDirectiveApplied{directive: phase-3-inline-composition-authorized, rationale: <text>}` events on every such cycle, mark such cycles as not-MVR-eligible.
3. **Hybrid:** authorize inline for bounded-surface sweeps (evidence-grounded, no judgment-heavy claims) but disallow for layer-close or pre-MVR rounds.

**Classification:** Deferred-pending-SO. Composes with F11 (recursive self-violation pattern) + F4 (cleanroom restart).

---

### Finding 13 — 4 Phase-3 cluster names (Implementation / Architecture / Communication / Adversarial) are load-bearing but unregistered coinages (Dim: vocabulary-registry conformance; AIE — sub-agent cost-discipline coordination) — Open

**Evidence:**
- `vsdd-phase-3.md` lines 8–11 declare the 4 cluster names + their composed-domain memberships. These names are the operationalization of the "cluster-batching shape" the methodology-cost-discipline rests on (~60% agent-count reduction per AIE primer).
- The cluster names are referenced in:
  - DESIGN-METHODOLOGY.md (cluster-batching shape section)
  - Phase 4 routing logs (each routed finding is tagged with its origin cluster)
  - The Phase 3 primer itself (canonical referent)
- BUT: zero of the 4 cluster names appear in `templates/registry/vocabulary.yaml`. They have no first_introduced_in version pin; no domain_scope declaration; no deprecated_aliases pointers.
- These names ARE descriptive (not letter-labels — "Implementation cluster" vs "Cluster A"). The naming-discipline anti-pattern doesn't fire on them. The gap is **vocabulary-registry conformance**, not letter-label.

**Why it matters from AIE lens:** The cluster-batching shape is a cost-discipline decision (cluster vs. per-domain = 4 vs. 18 agents per cycle). The names are referenced by Phase 4 routing + sub-agent dispatch decisions. An adopting project that extends the cluster set (operator-extensible per `canonical-patterns.yaml`'s operator_extensions surface) has no canonical place to register a 5th cluster. The registry-conformance discipline is silently bypassed for the methodology's own load-bearing coinages.

**Routing:** Phase 4 → Phase 1a (add 4 cluster-name entries to vocabulary.yaml managed-section + define `cluster_name_extensions` schema for operator-side registration). Mechanical fix.

**Classification:** Resolved-pending. Composes with F2 (registry-coverage-gap).

---

### Finding 14 — `Pillar` regex on main is `\bPillar \d\b`; does NOT match "Pillar" as table column header or "Three pillars" as section heading (Dim: spec-vs-implementation alignment; vsdd-methodology) — Open

**Evidence:**
- `templates/registry/canonical-patterns.yaml:44-46`:
  ```yaml
  - pattern: "\\bPillar \\d\\b"
    description: "Pillar <number> organizing structure"
    corrective_pattern: "Use descriptive name for the pillar's concept"
  ```
- Surviving sites that the regex does NOT match: `README.md:316` `### Three pillars`, `README.md:318` `| Pillar | Source | ...`, `DESIGN-OBSERVABILITY.md:309` `## Three pillars`, `DESIGN-OBSERVABILITY.md:311` `| Pillar | Source | ...`, `DESIGN-OBSERVABILITY.md:30` `Three pillars (logs / metrics / traces)`, `DESIGN-OBSERVABILITY.md:3` `three-pillars + dashboard ladder`, `DESIGN-OBSERVABILITY.md:610` `Three pillars + dashboard ladder + FinOps applied to IAR`, `methodology.md:412` `three pillars + dashboard ladder`.
- `wip-archive` commit `82a0ccb` extended the regex to `\\bPillar \\d\\b|\\bThree pillars\\b|\\bpillar(s)?\\b` (with negative-lookahead for structural references). Main has the narrower form.

**Why it matters:** The mechanical-enforcement layer that the methodology asserts (Goal 2 — "every methodology rule has a hook OR schema validator OR crosslink workflow check") is incomplete here. The regex enforces letter-labels for `Pillar 1` / `Pillar 2` (zero sites match) but fails to catch the actual surviving anti-patterns. Recursive failure mode in line with F11 (methodology spec violating its own invariant).

**Routing:** Phase 4 → Phase 1b (canonical-patterns.yaml extension). Port the `wip-archive` regex extension to `main`. Composes with F3 + F4.

**Classification:** Resolved-pending (mechanical regex extension).

---

### Finding 15 — Project-intent enum is asymmetric between SO domain prompt + methodology.md + DESIGN.md template (Dim: spec-contract specificity; SO) — Open

**Evidence:**
- SO domain prompt (`vsdd-domain-solution-owner.md` line 22): "DESIGN.md § Project intent declares one of the four intents (learning-exercise / portfolio / capstone / production). Intent calibrates Phase 5 + Phase 6 strategy + axes activation."
- `methodology.md` (415 lines): no section declares the 4-intent enum. Project intent IS referenced (in the auth-method section + the closing cross-references) but the enum values are not surfaced in the canonical spec.
- `templates/DESIGN.md.vsdd-template`: not read this round — uncertain whether the template declares the enum or simply has a `Project intent:` field.
- `README.md` does not enumerate the 4 intents.

**Why it matters:** The SO domain prompt cites 4 enum values as load-bearing-for-Phase-5/Phase-6-calibration. If the methodology spec doesn't declare them, the SO prompt's enum becomes the de-facto canonical source — but per the methodology's own naming-discipline (each load-bearing concept-anchor lives in vocabulary.yaml + is referenced from methodology.md), the enum should be in methodology.md too.

**Routing:** Phase 4 → Phase 1a (add a one-paragraph project-intent enum to methodology.md's auth-method-adjacent section OR a new sub-section). Composes with F11 (self-declared concept-anchor invariant).

**Classification:** Resolved-pending.

---

## Round-close summary

**15 findings raised this round. None Hallucinated; none Dismissed. Round MUST continue (Phase 3 round-trigger: any active domain produced real findings).**

| Finding | Domain | Classification | Routing | Composes with |
|---|---|---|---|---|
| F1 | DR | Resolved-pending | Phase 4 → 1a | F2 |
| F2 | TW | Deferred-pending-SO | Phase 4 → SO | F1, F11, F15 |
| F3 | SO + vsdd-methodology | Resolved-pending | Phase 4 → 1a + 1b | F4 |
| F4 | vsdd-methodology | Deferred-pending-SO | Phase 4 → SO | F3 |
| F5 | DR | Resolved-pending | Phase 4 → 1a | — |
| F6 | TW | Resolved-pending | Phase 4 → 1a | — |
| F7 | SO | Deferred-pending-SO (closes #111) | Phase 4 → SO | — |
| F8 | DR + TW | Resolved-pending | Phase 4 → 1a | F2 |
| F9 | DR + TW | Resolved-pending | Phase 4 → 1a | F2 |
| F10 | sanity-check | Deferred-pending-SO | Phase 4 → SO | — |
| F11 | vsdd-methodology | Deferred-pending-SO | Phase 4 → SO | F2, F15 |
| F12 | vsdd-methodology | Deferred-pending-SO | Phase 4 → SO + vsdd-methodology | F4, F11 |
| F13 | AIE | Resolved-pending | Phase 4 → 1a | F2 |
| F14 | vsdd-methodology | Resolved-pending | Phase 4 → 1b | F3, F4 |
| F15 | SO | Resolved-pending | Phase 4 → 1a | F11 |

**MVR signal:** NOT YET. 9 Resolved-pending findings + 6 Deferred-pending-SO findings; zero Hallucinated. Phase 3 cycle continues; next round requires at minimum the F4 + F11 + F12 SO dispositions to land before re-running the sweep (otherwise the same findings re-surface).

**Phase 4 routing recommendation:**
1. **Mechanical-fix bundle (F1 + F6 + F14):** cross-reference fix in 2 registry comments + hook-count canonicalization in 6 sites + canonical-patterns.yaml Pillar regex extension. Single-commit-able, sub-30-min.
2. **Cleanroom-restart resolution (F4 + cascades):** SO disposition required on port-forward vs. re-author path. Blocks F3 mechanical sweep until decided.
3. **SO-disposition bundle (F2 + F7 + F10 + F11 + F12 + F15):** six Raise-to-SO findings requiring operator-directive + `OperatorDirectiveApplied` event chain. Recommend a single operator session, sequenced.
4. **Sweep bundle (F3 + F5 + F8 + F9 + F13):** mechanical prose sweeps + vocabulary-registry additions. Dependent on F4 resolution (port-forward avoids re-doing F3).

**Cross-finding coherence (sanity-check dim 2):** the 15 findings are mutually consistent. F11 (self-violation of concept-anchor invariant) + F12 (inline-Phase-3 deviation) + F4 (cleanroom-restart drift) form a coherent meta-pattern: the methodology's own discipline is mid-flight on `main`, and the toolkit's authoring is operating ahead of the canonical-doc surface that the discipline rests on. The compensation is exactly what this review-log captures — surface the gaps before stability-commitment fires.

**Sycophancy-compensation reflection:** the bias I resisted is dismissing F11 + F12 as "I authored these in good faith; the gap is just velocity-vs-coherence trade-off." The discipline says velocity is acceptable pre-stability-commitment AND that the gap must be surfaced in the audit trail. This file is the audit-trail surface. Failure mode would be to file F1 + F6 + F14 (the mechanical fixes) without F11 + F12 (the methodology-spirit findings).

---

## Cross-references

- `templates/registry/vocabulary.yaml` (10-term registry — surface for F2, F9, F10, F13 amendments)
- `templates/registry/canonical-patterns.yaml` (regex registry — surface for F1 comment fix, F14 regex extension)
- `methodology.md § Naming + coinage governance` (referenced by registries; section does NOT exist; F1)
- `DESIGN-METHODOLOGY.md § Naming + coinage governance` at line 796 (actual canonical home)
- `.claude/commands/vsdd-phase-3.md` (cluster-batching shape; F12 + F13 source)
- `wip-archive` branch (resolves F3 + F14; subject of F4)
- Open crosslink issue #111 (adv-f5; closed by F7 SO disposition)
- Open crosslink issue #128 (this review; round 1 close pending)
