---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-06-01
phase: phase-3
scope: TW validator-pair follow-up to DR Review 1 F2 (vocabulary registry coverage gap). Drafts the retain/add/retire filter against the ~50 unregistered load-bearing concept-anchors enumerated in 2026-06-01-documentation-reviewer.md § Scope.
lens: Consistency (5) + Maintainability (4). Operator-facing prose ergonomics — cold reader's memorization load.
source: domain-raised
session_note: Inline single-session continuation of the same Phase 3 cycle that produced 2026-06-01-documentation-reviewer.md. Methodology-deviation (inline single-agent multi-domain composition) re-declared by reference; see prior review-log for the full pre-phase composition block.
model: claude-opus-4-7
execution_method: inline main session
sycophancy_compensation: >-
  Same author identity as the canonical docs under review (5ccf740 → 1229cda). Bias to
  defend my own coinages as load-bearing-by-construction. Compensation — every RETIRE
  candidate must have its replacement prose drafted at point-of-use; every ADD candidate
  must have its structural binding (CLI output / error code / event variant / schema field
  / hook ID / artifact-class name) cited. No term is moved without mechanical evidence.
filename_note: TW domain slug per canonical `<date>-<domain-slug>.md` convention. This entry is the validator-pair follow-up; DR cold-read pass at round close.
---

# Technical Writer Review 1 — 2026-06-01

**Phase 3 cycle round:** 1 (same cycle as 2026-06-01-documentation-reviewer.md; TW validator-pair pass)

---

## The filter

**Criterion (operator-directive 2026-06-01):** keep a term as a registered methodology coinage if it appears in at least one of:

- CLI output (visible to operator via `vsdd <subcommand>` stdout/stderr or `vsdd verify explain <code>`)
- Error catalog (a `VSDD-E####` or `VSDD-W####` or `VSDD-L####` code or its message text)
- Event variant name (one of the 18 methodology event variants in `.vsdd/events.jsonl`)
- Schema field (any frontmatter key, payload key, or YAML schema key in the 13 artifact-class schemas)
- Hook ID (a `check-*.py` filename or a `check-id` reported by a hook)
- Artifact-class name (one of the 13 registered classes)

Otherwise: retire to descriptive prose at point-of-use.

**Why this criterion is sound:** the methodology's `Goal 2` (audit-trail + machine-enforceability) operationalizes via these 6 surfaces. A term that doesn't appear in any of them is by construction not load-bearing for mechanical enforcement — only for prose convenience. Per the methodology's own `DESIGN-METHODOLOGY.md:822` ("methodology favors descriptive prose over named-mechanism shorthand"), prose-only coinages are exactly the candidates the methodology already proscribes.

---

## Applying the filter to the ~50 candidates

### Keep unchanged (already registered, structurally bound, no scope ambiguity)

| Term | Structural binding | Notes |
|---|---|---|
| VSDD | Toolkit name; error-code prefix (`VSDD-E####`); `schema_class:` prefix in artifact frontmatter | No change |
| capture-source | `capture_source` field on every `EventEnvelope`; 7-value enum | No change |
| Raise to SO | Routing concept + `OperatorDirectiveApplied{directive: spec-contract-amended OR spec-contract-amendment-rejected}` event payload | No change |
| phase-domain composition | `PhaseCompositionDeclared` event variant | No change |

### Keep with definition amendment (registered, but registry entry needs scope/definition extension)

| Term | Structural binding | Amendment |
|---|---|---|
| MVR | Phase 3 round-close signal; per-layer signal; per-dimension signal (Exit Signal feeds) | Extend definition to enumerate 3 scopes (per F10 in DR Review 1). Recommended phrasing: "round-MVR (Phase 3 per-round closure) / layer-MVR (gates layer-close) / dimension-MVR (Spec MVR + Test MVR + Implementation MVR + Formal-verification MVR feeding Exit Signal)." |
| Exit Signal | `ExitSignaled` event variant + Phase 6 attestation record class | Drop the `deprecated_aliases: ["four-dimensional convergence"]` line — that alias is still load-bearing in `methodology.md:337` and DESIGN-METHODOLOGY; either retire the alias from prose OR remove the deprecation marker. Resolved-pending operator pick. |
| IAR | Phase 3 cycle pattern; appears in `.vsdd/events.jsonl` round-context labels + Phase 3 primer + dashboard ladder | No definition change. |
| Red Gate | Phase 2a artifact; `manual-tests/error-catalog/` fixture-tree references; `Phase 2a Red Gate test stubs` in DESIGN-METHODOLOGY auto-scaffold spec | No definition change. |
| Exacting Mentor | Stance name; Mentor voice schema requirement (DESIGN-VERIFICATION:154 `summary: <Mentor voice one-line>`) flows from this stance; appears in every Phase 3 primer | No definition change. |
| always-on baseline | Domain-set composition rule; referenced from `PhaseCompositionDeclared` payload validation | Recommend definition tightening: "SE + QE + SA + SO active in every project; PE + Performance Engineer active when project ships code (i.e., `methodology_anchors:` includes implementation surfaces beyond docs)." Current definition (`vocabulary.yaml:79-84`) is correct but operator-facing prose elsewhere drops "when project ships code" condition. |

### Add to registry (currently unregistered; structural binding confirmed)

| Term | Structural binding | Proposed `domain_scope` |
|---|---|---|
| **bypass-marker** | `VSDD-E0016: bypass-rationale-missing` + `VSDD-W0070: bypass-marker-scope-mismatch` + frontmatter `bypass: [{hook_id, rationale, pr_approval_label}]` + HTML comment pattern `<!-- hook-bypass[<hook-id>]: <rationale> -->` | [methodology, verification] |
| **sycophancy compensation** | `sycophancy_compensation:` schema field on Review-entry class (DESIGN-SCHEMA:133); `check-sycophancy-compensation.py` hook ID; `SycophancySelfAudit` event variant | [methodology, phase-3, phase-5] |
| **operator-directive** | `OperatorDirectiveApplied` event variant + directive enumeration in payload | [methodology, solution-owner] |
| **substrate** | `substrate_anchors:` schema field on methodology-spec frontmatter (`methodology.md:6`) | [methodology] |
| **earned-by-recurrence** | `OperatorDirectiveApplied{trigger: earned-by-recurrence, recurrences: [<evidence-refs>]}` payload pattern (asserted in DESIGN-METHODOLOGY:802); methodology-amendment governance trigger | [methodology] |
| **validator pair** | `validator_pair:` schema field on Domain-prompt artifact class (DESIGN-SCHEMA:246) | [methodology, all-domains] |
| **classification universe** | `classification_universe:` schema field on Domain-prompt artifact class (DESIGN-SCHEMA:245); `check-classification-universe.py` hook | [methodology, all-domains] |
| **status-tier** | `status: candidate \| accepted \| deprecated` schema field on error catalog + artifact-class registry + vocabulary registry | [methodology, schema] |
| **per-feature axes** | `axes_declared` field in `ProjectInitialized` event payload (DESIGN-OBSERVABILITY:255); 9 axis field names in `.vsdd/config.yaml` | [methodology, project-init] |
| **Mentor voice / Formal voice** | `summary: <Mentor voice one-line>` schema requirement in hook output spec (DESIGN-VERIFICATION:154); Formal voice required for Phase 6 attestation record + methodology amendments + schema definitions | [methodology, all-domains, phase-6] |
| **anchor-ID** | `<anchor-id>` schema type + `target_section: <anchor-id \| null>` field (DESIGN-SCHEMA:175); anchor-ID generation conventions section in DESIGN-SCHEMA | [schema] |
| **methodology-stabilization** | `OperatorDirectiveApplied{directive: methodology-stabilization}` directive name — fires the forward-only narrative-preservation rule | [methodology, solution-owner] |
| **Implementation cluster / Architecture cluster / Communication cluster / Adversarial cluster** | Phase 3 cluster-batching shape; appear in Phase 3 routing-log frontmatter + `composed_domains` payload tags | [methodology, phase-3] |

**Add-to-registry subtotal:** 16 entries (the 4 cluster names counted as 4).

### Retire to descriptive prose (no structural binding)

| Coinage to retire | Replacement at point-of-use |
|---|---|
| narrative-preservation | "after stability commitment, the methodology spec + primers + domain prompts + supplements are append-only" |
| stability commitment | Inline the four triggers where the term currently appears: "v1.0 release, first push to a public remote, first downstream adoption, or `OperatorDirectiveApplied{directive: methodology-stabilization}`" |
| organizational scaffolding | "letter-label anti-pattern: a prefix word + index with no concept-word in the identifier (e.g., `Cluster A`, `Tier B`)" |
| concept-anchor | "load-bearing methodology term" (a meta-vocabulary word that doesn't earn its own registry slot) |
| axis-driven activation | "axis-additive domain activation" or just describe at point-of-use ("declaring `network-exposed: yes` activates Red Team + Security") |
| role domain / meta domain | Just say "domain" / "meta-domain" inline; no need for "role domain" as a category name since the meta-domain set is enumerated (vsdd-methodology + sanity-check) and everything else is a domain by default |
| cold-session reviewer mode | "cold-context review (worktree-isolated; no operator memory; no prior-cycle conversation)" |
| skill mode | "skill-loaded composition" or just "the default phase composition (skill primer + domain prompts)" |
| tone-flex policy | Drop the policy name; state the rule at the one site that needs it: "Mentor voice by default; Formal voice for Phase 6 attestations + methodology amendments + schema definitions" |
| methodology-evolution coherence | "methodology coherence across versions" |
| methodology-spirit | "spirit of the methodology" (drop the hyphenated coinage) |
| methodology-amendment governance | Describe at point-of-use: "amendments require earned-by-recurrence evidence OR explicit operator-directive citing equivalent evidence; SO holds change authority" |
| two-audience principle | Keep the concept; drop the proper-noun coinage. Replace "per the two-audience principle" with "every artifact serves humans and agents simultaneously" — describe the rule, don't name it. |
| three-audience principle | Already absent from spec; was an unwritten-but-implied tension. Resolve by stating in DR-prompt + TW-prompt: "the developer-extending-methodology vs. user-following-methodology distinction is the SO ↔ DR adversarial-pair pattern, not a separate audience." |
| "the rebuild" | "the toolkit" / "vsdd-cli" (closes F5 in DR Review 1) |
| prose-surface composition discipline | "TW + DR co-authorship on commits touching README, DESIGN docs, manual-tests, primers, or CHANGELOG" |
| layer-cycle PR discipline | Describe inline at the one site that needs it (the Layer-cycle PR discipline section in README) |
| dual-mode validator | Describe at point-of-use: "validators run in frontmatter-mode (parse + check schema fields) OR structural-mode (parse + check whole-document patterns)"  |
| four-cluster default | Keep the 4 cluster names (in the add-to-registry group) — the "four-cluster default" descriptor is then derivable; drop the umbrella coinage |
| adversarial-pair separation invariant | Keep the rule (Security ↔ Red Team on different clusters; TW ↔ DR on different clusters) — describe it at point-of-use; drop the "invariant" coinage |
| four-dimensional convergence | Already Exit Signal's deprecated_alias; finish the deprecation by removing the term from prose (currently surviving at `methodology.md:337`) |

**Retire-to-prose subtotal:** 21 candidates.

### Numerics (NOT vocabulary; auditable separately)

These are counts, not coinages — they don't belong in the registry. They DO require quantitative-claim discipline (per F6 in DR Review 1) — single canonical value per count.

- **13 artifact classes** (DESIGN-SCHEMA is canonical)
- **18 methodology event variants** (DESIGN-OBSERVABILITY is canonical)
- **19 methodology hooks** (DESIGN-VERIFICATION is canonical; resolve F6 drift by adopting 19 everywhere)
- **7-value capture-source enum** (defined in vocabulary.yaml capture-source entry)
- **5 lenses** (Attacker / Edge cases / Usability / Maintainability / Consistency; Phase 3 primer is canonical)
- **4 cluster default** (the four cluster names in the add-to-registry group; the "4" is derived)
- **16 role + 2 meta = 18 domains** (methodology.md § Domain set is canonical)
- **10 phases** (1a / 1b / 1c / 2a / 2b / 2c / 3 / 4 / 5 / 6; methodology.md § Phase taxonomy is canonical)
- **4 governing design goals** (deferred per F7 / open issue #111)

### Goal 1–4 (separately tracked under F7 / #111)

Out of scope for this filter pass — SO disposition required.

---

## Target final state

| Group | Count | Cumulative registry size |
|---|---|---|
| Keep unchanged | 4 | 4 |
| Keep with amendment | 6 | 10 |
| Add (newly-structurally-bound) | 16 | **26** |
| Retire to prose | 21 removed from spec | — |
| Numerics (not vocabulary) | 0 | — |
| Goal 1–4 (separate disposition) | 0–4 | 26–30 |

**Cold-reader memorization load:** ~26 registered terms (with 4 cluster names treated as a single concept-anchor "the Phase 3 cluster names"). Roughly half are operationally inescapable (VSDD, MVR, IAR, Exit Signal, Red Gate, capture-source, phase-domain composition, bypass-marker, operator-directive, status-tier) and half are mechanical-enforcement bound (anchor-ID, validator pair, classification universe, sycophancy compensation, methodology-stabilization, etc.) where the cold reader meets them via the schema or hook output, not via the spec prose.

**Compared to current ~50+ candidate surface:** roughly half the cold-reader load. The retired 21 terms become descriptive prose at point-of-use — the reader still encounters the concept, but does not need to memorize the proper-noun handle.

---

## Sequencing for adoption

1. **Comment-level fix (sub-5-min):** correct the 2 broken cross-refs from F1 (`methodology.md` → `DESIGN-METHODOLOGY.md` in `vocabulary.yaml:5` + `canonical-patterns.yaml:5`). Unblocks everything else.

2. **Keep-unchanged pass:** no work — entries already correct.

3. **Amendment pass (single TW + DR co-authored commit):** extend MVR entry to enumerate 3 scopes; resolve Exit Signal deprecated_alias; tighten always-on baseline definition. Update `vocabulary.yaml` managed-section + run a project-wide grep to find Exit-Signal-alias sites for the prose sweep.

4. **Add-to-registry pass (single TW + DR co-authored commit per 4–5 entries; ~3–4 commits total):** add 16 new entries to `vocabulary.yaml` managed-section. Each entry carries: `term`, `definition`, `first_introduced_in: 0.1.0`, `deprecated_aliases: []`, `abbreviation`, `domain_scope`. Cluster-name entries share a single `domain_scope: [methodology, phase-3]`.

5. **Retire-to-prose pass (largest single commit; SO + TW + DR co-authored):** project-wide prose sweep replacing 21 retired coinages with their descriptive-prose substitutes. Sites enumerated by grep per term. This is the largest authoring surface — likely ~15–25 file-touches.

6. **Numerics pass (TW + DR co-authored; mechanical):** canonicalize each count to a single value across docs (closes F6).

7. **Goal-1-4 disposition (SO-disposition only):** F7 / #111 closes here OR keeps Goal 1–4 as a documented carve-out from the letter-label anti-pattern. Independent of the other 6 passes.

---

## SO disposition requests

1. **Filter criterion approval.** Does the 6-surface criterion (CLI output / error catalog / event variant / schema field / hook ID / artifact-class name) match the SO's intent for "load-bearing"? If not, propose alternative criterion.

2. **Exit Signal alias (amendment group).** Retire `four-dimensional convergence` from prose at `methodology.md:337` and elsewhere, OR remove the `deprecated_aliases:` line and keep both terms in active use?

3. **Retire-group ambiguity flags:**
   - **two-audience principle** — retire as a proper-noun coinage but keep the rule it describes. Confirm.
   - **role domain / meta domain** — retire the "role domain" category entirely (every domain is a domain; the 2 meta-domains are enumerated). Confirm.
   - **dual-mode validator** — retire the umbrella coinage; describe `frontmatter-mode` + `structural-mode` at point-of-use. Confirm.

4. **Goal-1-4 disposition sequencing.** F7 / #111 disposition required to close the naming sweep cleanly. Three options enumerated in DR Review 1 F7.

---

## DR cold-read pass (validator-pair)

This entry is filed for DR cold-read pass at Phase 3 round close. DR-side questions to validate at cold read:

- **Cold-reader test:** does a fresh reader landing on the 16 add-to-registry entries understand each term's referent from the registry definition alone (no spec lookup)? Each entry needs a 1-sentence definition that stands alone.
- **Cross-reference resolution:** every amendment + add entry carries cross-references to its structural-binding surface — does the cold reader navigate cleanly?
- **Replacement-prose currency:** does each retire-group replacement read better than the retired coinage at the *site that needs it most* (i.e., the highest-traffic uses)? Spot-check sites:
  - "narrative-preservation" at `methodology.md:235` + `README.md:712`
  - "two-audience principle" at `methodology.md § Two-audience principle` heading itself (heading might need to become "Every artifact serves humans and agents")
  - "the rebuild" at `README.md:113` (operator's first encounter)

---

## Recurrence note (added post-author correction)

First-draft of this file used "Bucket A" through "Bucket F" as group labels. Caught by operator-directive within minutes of write. Corrective edits applied — group labels are now descriptive ("Keep unchanged", "Keep with amendment", "Add to registry", "Retire to descriptive prose", "Numerics", "Goal 1–4").

This is the **third recurrence** of the letter-label anti-pattern in this session, by the same author, after explicitly authoring DR Review 1 F3 (sweep incomplete) + F11 (methodology spec violates its own concept-anchor invariant) + F12 (inline Phase 3 deviation). Recurrence pattern: the same identity authors anti-pattern instances **immediately after** authoring the anti-pattern's catch surface — the exact sycophancy failure mode the Phase 3 primer's cluster-batched cold-session shape is designed to prevent (cold-session reviewers carry no author-context; the inline reviewer is the author).

**Canonical-patterns.yaml extension proposal:** add `\bBucket [A-Z]\b` to `letter_label_anti_patterns`. Earned-by-recurrence trigger fired (this is now the 5th recurrence in the toolkit's history if existing-suite R78 F4 + R94 cluster-letter cases are included; 3 in this session alone). Resolved-pending — awaiting operator confirmation before committing the registry change.

---

## Cross-references

- DR Review 1 — 2026-06-01 (`review-log/2026-06-01-documentation-reviewer.md`): source of F2 (this filter implements F2); F5 (the rebuild); F6 (hook count); F7 (Goal N); F10 (MVR scope); F13 (cluster names)
- `templates/registry/vocabulary.yaml` (10-term registry — target of amendment + add-to-registry passes)
- DESIGN-METHODOLOGY.md § Naming + coinage governance (line 796 — governs amendment trigger)
- Open crosslink issue #128 (this Phase 3 cycle; round 1 close pending)
- Open crosslink issue #111 (Goal N disposition; Goal-1-4 group)
