---
schema_class: review-entry
schema_version: 1.0.0
review_number: 2
date: 2026-05-28
phase: phase-3
scope: Comprehensive audit of vsdd-cli downstream methodology additions against dollspace's upstream VSDD whitepaper (gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00). Classifies each downstream element as (a) operationalization within upstream framing, (b) downstream specialization of upstream concept, or (c) downstream invention without upstream basis. Output is the audit-trail surface; not amendment proposals.
lens: VSDD Methodology meta-domain (upstream-conformance audit) + Documentation Reviewer (cross-source consistency)
source: director-raised (operator-directive 2026-05-28 "fetch the gist and verify" → "Yes" to audit-other-downstream-additions scope; "comprehensive" + review-log routing)
session_note: Filename slug `upstream-audit` improvised pending approved-artifact-name registry resolution (operator-flagged 2026-05-28 — none of the existing review-log slugs are approved either).
model: claude-opus-4-7
execution_method: inline main session
sycophancy_compensation: The author of this review is the same identity that authored the downstream methodology. The natural bias is to classify inventions as "specialization" rather than "invention" to soften the surface — granting upstream-grounding credit where none was earned. This audit must resist that urge. Honest reading: most downstream structure is invention motivated by tooling necessity + the upstream's Linear Accountability principle; not all of it has direct upstream-text grounding. Class (c) is the unflattering default when no upstream sentence covers the surface; the author resisted softening (c) → (b) on convenient interpretations.
---

# VSDD Upstream Conformance Audit — 2026-05-28

## TL;DR

The upstream VSDD whitepaper (Section II Pipeline + Section IV Core Principles + Section V AI Orchestration + Section VI When to Use) is ~3500 words. The downstream methodology (methodology.md + 4 DESIGN docs + 18 domain prompts + 14 supplements + templates + README) is ~150,000 words. The ratio reflects operationalization mass — most downstream content is "how do we actually do this with our tools?" — but a non-trivial subset is invention without upstream-text grounding. This audit classifies 34 surfaces into three buckets:

- **Class (a) operationalization within upstream framing**: 15 surfaces — the upstream describes the concept; downstream implements it
- **Class (b) downstream specialization of upstream concept**: 5 surfaces — the upstream names the concept loosely; downstream extends/structures it
- **Class (c) downstream invention without upstream basis**: 22 surfaces — no upstream sentence covers this surface

The (c) inventions are not necessarily wrong — some are necessary tooling operationalization (`.vsdd/config.yaml`; event variant taxonomy); some are defensible methodology extensions (forward-only discipline; earned-by-recurrence). But they are inventions, not upstream-derived. The audit-trail honesty surface is naming them as such.

## Methodology source consulted

Upstream gist `gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00`, fetched 2026-05-28 via `gh api gists/d8d3bc3ecf4188df049d7a4726bb2a00`. Single file `VSDD.md`. Sections I (Toolchain), II (Pipeline — Phase 1 through Phase 6 with Steps 1a/1b/1c + 2a/2b/2c), III (Contract Chain), IV (Core Principles 1-7), V (AI Orchestration Notes), VI (When to Use VSDD).

## Classification key

- **(a)** Operationalization within upstream framing — upstream sentence covers the concept; downstream is tool-substitution or structural detail within that framing
- **(b)** Downstream specialization of upstream concept — upstream names the concept loosely; downstream extends, structures, or formalizes it
- **(c)** Downstream invention without upstream basis — no upstream sentence covers this surface; necessary or defensible inventions still classified (c) for audit-trail honesty

---

## Surface category 1: Pipeline structure

### Phase 1a Behavioral Specification — Class (a)

Upstream Section II Phase 1 § Step 1a explicitly names "Behavioral Specification" with Behavioral Contract + Interface Definition + Edge Case Catalog + Non-Functional Requirements. Downstream maps directly.

### Phase 1b Verification Architecture — Class (a)

Upstream Section II Phase 1 § Step 1b explicitly names "Verification Architecture" with Provable Properties Catalog + Purity Boundary Map + Verification Tooling Selection + Property Specifications. Downstream maps directly. Upstream is emphatic: "This must happen in Phase 1, not Phase 5."

### Phase 1c Spec Review Gate — Class (a)

Upstream Section II Phase 1 § Step 1c explicitly names "Spec Review Gate" with both-human-and-Adversary review of both behavioral contract + verification architecture. Downstream extends with "Decomposition" sub-name (layer decomposition); decomposition framing has no upstream text but is consistent with "Sub-issues are generated for each behavioral contract item" (Linear Accountability principle).

### Phase 2a Test Suite Generation (Red Gate) — Class (a)

Upstream Section II Phase 2 § Step 2a explicitly names "Test Suite Generation" with Unit + Edge Case + Integration + Property-Based Tests + "The Red Gate: All tests must *fail* before any implementation begins." Downstream maps directly. Property-Based Tests are upstream Phase 2a — downstream had previously misplaced them in Phase 5 (corrected this session).

### Phase 2b Minimal Implementation — Class (a)

Upstream Section II Phase 2 § Step 2b explicitly names. Downstream maps directly.

### Phase 2c Refactor — Class (a)

Upstream Section II Phase 2 § Step 2c explicitly names. Downstream maps directly.

### Phase 3 Adversarial Refinement (The VDD Roast) — Class (a) for the phase; Class (c) for cluster-batching

Upstream Section II Phase 3 explicitly names. Upstream Adversary reviews 5 surfaces (Spec Fidelity, Test Quality, Code Quality, Security Surface, Spec Gaps Revealed by Implementation) + "Fresh context window on every adversarial pass." Single Adversary per pass — upstream does not enumerate multi-cluster batching shape. Downstream 4-cluster batching (Implementation + Architecture + Communication + Adversarial) is **Class (c) invention** motivated by cost optimization at multi-agent scale.

### Phase 4 Feedback Integration Loop — Class (a)

Upstream Section II Phase 4 explicitly names + describes routing of spec-level / test-level / implementation-level / new-edge-case flaws. Downstream maps directly.

### Phase 5 Formal Hardening — Class (a) for the phase; (a) for all 5 surfaces

Upstream Section II Phase 5 explicitly names + lists Proof Execution + Fuzz Testing + Security Hardening (Wycheproof + Semgrep) + Mutation Testing + Purity Boundary Audit. Downstream now matches all 5 surfaces (corrected this session — Security Hardening was previously omitted).

### Phase 6 Convergence (The Exit Signal) — Class (a)

Upstream Section II Phase 6 explicitly names + tabulates 4-dimensional convergence (Spec / Tests / Implementation / Verification). Downstream maps directly. Core Principle 7 "Four-Dimensional Convergence" reinforces.

---

## Surface category 2: Tooling substitutions

### crosslink (vs Chainlink) — Class (a)

Upstream Section I names "Chainlink — Hierarchical issue decomposition." Downstream substitutes crosslink as the Tracker role. Substitution within upstream-named role.

### Claude as Builder — Class (a)

Upstream Section I + V: "Claude (or similar)" as Builder. Downstream uses Claude. Direct.

### The Adversary substrate — Class (a)

Upstream Section I names "Sarcasmotron (Gemini Gem or equivalent)" as Adversary. Section V: "The Adversary benefits from a *different* model or configuration to avoid shared blind spots." Downstream uses Claude cold-session as Adversary (not a different model family). This violates upstream Section V's "genuine cognitive diversity" recommendation, but the role-substitution itself is upstream-permitted ("or similar").

### MCP server registration — Class (a)

Upstream-agnostic on integration mechanism. MCP is downstream tool-integration detail within upstream's "AI orchestration" framing.

---

## Surface category 3: Role + domain set

### 18 domain prompts vs upstream 3 roles — Class (c)

Upstream Section I names 3 roles: Architect (human), Builder (Claude), Adversary (Sarcasmotron), plus Tracker (Chainlink as tooling). Downstream introduces 18 domain prompts (Solution Owner, Solution Architect, Software Engineer, Quality Engineer, Platform Engineer, Performance Engineer, Technical Writer, Documentation Reviewer, AI Engineer, Data Engineer, Privacy, Security, Red Team, UX, Accessibility, Localization, Sanity Check, VSDD Methodology).

Mapping the 18 onto the 3 upstream roles:
- **Architect (human)** maps loosely to **Solution Owner** (DESIGN.md change authority) + **Solution Architect** (architectural decomposition)
- **Builder (Claude)** maps to **Software Engineer** + **Quality Engineer** + **Technical Writer** + **Platform Engineer** + **Performance Engineer** + **Documentation Reviewer** + **AI Engineer** + **Data Engineer** + **Privacy** + **Security** + **UX** + **Accessibility** + **Localization** — all "specialized lenses Claude applies"
- **Adversary** maps to **Red Team** + (in Phase 3 cold-session) the cluster-batched reviewer composition
- **Sanity Check** + **VSDD Methodology** are downstream meta-domains with no upstream analog

The 18-domain expansion is downstream invention. It is defensible as "specialized lenses within Architect/Builder/Adversary roles" but the upstream does not enumerate them. Activation criteria (axis-based) is also downstream invention. Class (c) overall.

### Always-on baseline (6 domains) — Class (c)

Downstream declares Software Engineer + Quality Engineer + Solution Architect + Solution Owner + Platform Engineer + Performance Engineer as always-on regardless of axes. Upstream has no baseline concept; Architect + Builder + Adversary are always-on by being the roles themselves. Downstream invention.

### VSDD Methodology meta-domain — Class (c)

Downstream invention. Reviews methodology-application semantic coherence. Upstream Section IV Core Principles assume the methodology is applied; doesn't have a meta-domain to audit application. Class (c) but defensible as audit-trail discipline.

### Sanity Check meta-domain — Class (c)

Downstream invention. Operates as validator-pair for SO (which has no peer). Upstream has no validator-pair concept.

### Validator pair operationalization — Class (c)

Downstream invention. Each domain has a `validator_pair:` declared in the frontmatter; findings route to the pair for validation. Upstream has the Adversary as the sole validator (with human as final authority). Downstream's pair structure has no upstream basis.

---

## Surface category 4: Audit-trail / observability

### 18 methodology event variants — Class (c)

Downstream taxonomy: PhaseEntered, PhaseExited, PhaseCompositionDeclared, FindingRaised, FindingClassified, FindingRouted, RaiseToSO, OperatorDirectiveApplied, ProjectInitialized, HookFired, ValidationPassed, ValidationFailed, ExitSignalAttested, DraftPROpened, PRReadyForReview, PRMerged, AuthMethodChanged, MethodologyAmended.

Upstream has no event-variant enumeration. Linear Accountability principle ("every spec item, test, and line of code has a corresponding tracked unit of work") motivates the audit-trail surface but doesn't specify event types. Class (c) invention. Defensible as operationalization of Linear Accountability + the toolkit's observability goal.

### 13 artifact classes — Class (b)

Downstream taxonomy: design-doc, methodology-spec, methodology-spec-section, phase-primer, domain-prompt, supplement, review-entry, deployment-manifest, error-catalog, anonymization-patterns-registry, canonical-patterns-registry, registry-entry, audit-event.

Upstream names specific artifacts: "formal specification document," "Behavioral Contract," "Verification Strategy," "Property Specifications," "test suite," "Chainlink Issue / Sub-issue ('beads')." Distinct artifact concepts exist upstream; downstream formalizes into 13 classes with schema discipline. Specialization with explicit upstream basis for several classes (design-doc maps to formal specification document; review-entry maps to Adversary review output).

### `.vsdd/events.jsonl` audit-trail file — Class (c)

Downstream invention. Upstream Linear Accountability mentions Chainlink beads as tracking; not a separate event log. Defensible as observability operationalization.

### `.vsdd/config.yaml` — Class (c)

Downstream invention. Necessary tooling operationalization.

### `methodology_version` semver pinning — Class (c)

Downstream invention. Upstream has no methodology-versioning concept — VSDD-the-methodology is treated as stable. Class (c).

---

## Surface category 5: Project calibration

### 7 per-feature axes — Class (c)

Downstream axes (after this session's drops): ships-to-users-other-than-developer, network-exposed, persists-managed-schema-data, handles-user-data, ui-surface, localized, ai-runtime-cost-relevant.

Upstream has no axis concept. Section VI "When to Use VSDD" is qualitative ("Correctness is non-negotiable; codebase will be maintained long-term; multiple AI models available; security is primary; project complexity justifies formal spec work") not enumerated as binary flags. Class (c) invention. Replaced the earlier (also-invented) four-intent enum.

The axes work mechanically (each domain prompt's `activation_criteria:` matches axes); the structure is functional. But no upstream sentence specifies "projects declare an axis profile that drives domain activation."

### Phase-domain composition matrix + 5 enforcement mechanisms — Class (c)

Downstream invention. The matrix declares which domains compose at which phase. Five enforcement mechanisms: matrix declaration in methodology.md; per-primer instruction; pre-phase composition declaration; check-phase-composition hook; prior-phase exit-signal enforcement.

Upstream has the Adversary as the single Phase 3 reviewer + human as Architect-throughout. No multi-domain composition matrix. Class (c) — defensible as multi-agent operationalization but not upstream-derived.

---

## Surface category 6: Quality / verification

### Classification universe (5 values) — Class (b)

Downstream universe: resolved, deferred, dismissed, hallucinated, accepted.

Upstream Phase 6 has "Hallucinated findings" as the convergence signal (Adversary forced to invent problems). Downstream extends to 5-value classification universe applicable at every Phase 3 cycle. Specialization with explicit upstream basis (Hallucinated) + downstream extension (Resolved, Deferred, Dismissed, Accepted).

### Cross-dimension consistency check — Class (a)

Upstream Section IV Core Principle 7 "Four-Dimensional Convergence: The system isn't done until specs, tests, implementation, *and* formal proofs have all independently survived adversarial review." Phase 6 dimension table explicitly enumerates the 4 dimensions. Downstream cross-dimension consistency check is the operational form of upstream Principle 7. (a) operationalization.

### Per-mutant disposition — Class (b)

Upstream Phase 5: "Tools like mutmut or Stryker mutate the code to verify the test suite actually catches real bugs. If a mutation survives, the test suite has a gap." Downstream extends to per-mutant audit-trail discipline (each surviving mutant gets a disposition: test-added / spec-amended / mutation-equivalent / accepted-as-loss-of-fidelity). Specialization.

### Capture-source provenance enum — Class (c)

Downstream observability discipline (capture_source: otel-metric / otel-log-event / otel-trace-attribute / vsdd-custom-event / sdk-result-message / usage-api-reconciled / unmeasurable). Upstream has no observability provenance concept.

### Cluster-batching Phase 3 shape (4-cluster) — Class (c)

Downstream cost-optimization for multi-agent Phase 3. Upstream describes single-Adversary-per-pass. Class (c) invention.

---

## Surface category 7: Methodology meta-discipline

### Forward-only / pre-post stability discipline — Class (c)

Downstream invention. The methodology has a stability commitment (project-defined; v1.0-rc or operator-declared); pre-stability history is malleable, post-stability is append-only narrative-preserved. Upstream has no narrative-preservation lifecycle concept.

### Earned-by-recurrence trigger — Class (c)

Downstream invention. Methodology amendments require 2+ documented drift cases OR explicit operator-directive. Upstream has no amendment-governance concept (methodology is treated as stable).

### Operator-directive override pattern — Class (b)

Upstream Section V: "The Human is not a bottleneck — they're the strategic layer. They approve specs, resolve disputes, and make judgment calls that AI can't. The human's role is *elevated*, not diminished, by the AI orchestration." Concept of human-as-final-authority is upstream. Downstream operationalizes as `OperatorDirectiveApplied` event with structured rationale + audit-trail. Specialization.

### Layer-cycle PR discipline — Class (c)

Downstream invention. Layer decomposition + per-layer PR cycle with DraftPROpened → PRReadyForReview → PRMerged events + check-draft-pr-presence.py + check-pr-template-conformance.py hooks. Upstream has no PR-workflow concept (linear accountability via Chainlink beads, not git-PR-workflow).

### Synthesis-review-citation discipline — Class (c)

Downstream prose-discipline rule (synthesis reviews cite findings by descriptive title not finding-ID shorthand). Upstream-agnostic on review prose form. Class (c).

---

## Surface category 8: Verification infrastructure

### ~24 hooks — Class (b)

Upstream Phase 5: "Suites like Wycheproof (cryptographic edge cases) and Semgrep (static analysis) are run as CI/CD gates." Two specific hooks (Wycheproof + Semgrep) are upstream-named. Downstream extends to ~24 hooks covering: pre-commit anonymization, methodology-version-drift, dependency-approval, prior-phase-exit-signal, pre-stability-discipline, target-overshoot-recurrence, operational-mode-declared, spec-state-honest, naming-discipline, methodology-semantics, pre-phase-composition-loading, draft-pr-presence, pr-template-conformance, sycophancy-compensation, etc.

Specialization with limited explicit upstream basis. The hook taxonomy + per-hook semantics are downstream operationalization of the upstream "CI/CD gates" framing extended to multiple methodology-discipline surfaces.

### Error catalog (VSDD-EXXXX / VSDD-WXXXX codes) — Class (c)

Downstream invention. ~30 error codes enumerated across DESIGN-SCHEMA + DESIGN-VERIFICATION + README. Upstream has no error code taxonomy.

---

## Surface category 9: Toolkit scope additions

### `.vsdd/` substrate directory — Class (c)

Downstream invention. Necessary tooling operationalization.

### `vsdd init` command + interactive prompts — Class (c)

Downstream invention. Necessary tooling for axis declaration + auth method + deployment manifest.

### Auth method cross-field validation — Class (c)

Downstream invention from Phase 5 round 1 Security F4. No upstream basis.

### Cost-band observability (OTel collector, FinOps disciplines) — Class (c)

Downstream invention. Upstream is agnostic on observability beyond Linear Accountability + Chainlink beads.

---

## Tally

| Class | Count |
|---|---|
| (a) Operationalization within upstream framing | 15 |
| (b) Downstream specialization of upstream concept | 5 |
| (c) Downstream invention without upstream basis | 22 |
| **Total surfaces audited** | **42** |

The (a) cluster is dominated by Phase 1a/1b/1c + 2a/2b/2c + 3/4/5/6 (10 phase sub-units explicitly named upstream) + tool substitutions (Chainlink→crosslink, Builder=Claude, Adversary substrate, MCP) + cross-dimension consistency check. The substantial pipeline-structure grounding is real; the operationalization beyond pipeline structure is (b) + (c).

The (c) inventions cluster in: meta-discipline (forward-only, earned-by-recurrence, operator-directive, layer-cycle PR, synthesis-review-citation), audit-trail infrastructure (event variants, .vsdd/events.jsonl, methodology_version), project-calibration (7 axes, composition matrix), verification infrastructure (~24 hooks beyond Wycheproof + Semgrep; error catalog), toolkit scope (`.vsdd/`, `vsdd init`, FinOps).

The (a) operationalization cluster: Phase 1-6 sub-phasing (upstream-grounded directly), tooling role substitutions (Chainlink → crosslink; Builder = Claude), cross-dimension consistency check (Core Principle 7).

The (b) specialization cluster: 13 artifact classes (upstream has distinct artifacts); classification universe (upstream has Hallucinated); per-mutant disposition (upstream has Mutation Testing); operator-directive (upstream has human-as-final-authority); hooks (upstream has Wycheproof + Semgrep; downstream extends).

---

## What this means

The downstream methodology is **mostly Class (c) invention** with a Class (a) backbone of upstream-grounded phase structure. This is honest: vsdd-cli is an *operationalization of VSDD for adopting projects + the toolkit's own development*; that operationalization required substantial invention (taxonomies, schemas, hooks, event variants, axes, composition mechanisms) that upstream doesn't specify.

The inventions are not inherently illegitimate — operationalization is necessary work — but the audit-trail honesty surface is naming them as inventions, not claiming they derive from upstream. Several have the shape of "the upstream principle motivates this, but the specific structure is downstream":

- Linear Accountability → 18 event variants + .vsdd/events.jsonl
- Architect/Builder/Adversary roles → 18 domain prompts + composition matrix
- Wycheproof + Semgrep CI gates → ~24 hooks
- Hallucinated convergence signal → 5-value classification universe
- Four-Dimensional Convergence → cross-dimension consistency check + Exit Signal attestation discipline

These motivated-by-upstream inventions should be acknowledged as such — not framed as upstream-derived.

Several Class (c) inventions are more loosely motivated:
- Forward-only / pre-post stability discipline — no upstream sentence covers; defensible as methodology-evolution lifecycle
- Earned-by-recurrence trigger — no upstream sentence covers
- Layer-cycle PR discipline — no upstream concept of layers OR PRs
- Methodology-version semver pinning — upstream treats methodology as stable
- Operator-directive override formalization — upstream has human authority concept, downstream's event-emit structure is invented

These are downstream methodology decisions that extend or complement upstream rather than derive from it. The operator should know they're downstream when making methodology amendments.

## Open questions for operator routing

1. **Should the downstream-vs-upstream class be declared in each artifact?** A `upstream_conformance: a | b | c` field on methodology-spec-section frontmatter would make the audit-trail surface visible at every section. Adds discipline overhead.

2. **Should Class (c) inventions surface their motivation explicitly?** A "Motivated by upstream principle: <quote>" line for inventions that map to upstream principles (Linear Accountability, Four-Dimensional Convergence, etc.) would surface the implicit grounding.

3. **Should Class (c) inventions without upstream motivation be reviewed for retention?** Forward-only discipline, earned-by-recurrence, layer-cycle PR, methodology-version pinning — each is an invention with no upstream motivation. They may be earning their keep on operational grounds; the audit is the prompt to verify each.

4. **Should the operator file each surfaced (c) invention as a crosslink finding for individual disposition?** This audit names 22 inventions in one pass; per-invention routing would surface each for explicit operator decision.

5. **Should the Adversary substrate be revisited?** Upstream Section V recommends "a different model or configuration to avoid shared blind spots" + "Using a different model family (e.g., Gemini as Adversary when Claude is Builder) introduces genuine cognitive diversity." Downstream uses Claude cold-session as Adversary — same model family as Builder, violating the cognitive-diversity recommendation. Defensible on tooling-availability grounds but should be acknowledged.

## Cross-references

- `review-log/2026-05-28-vsdd-methodology.md` (Review 1) — session-level methodology-spirit drift defects; this audit (Review 2) is the upstream-conformance surface adjacent to it
- Crosslink issue #127 — SO domain prompt staleness (this audit found other staleness patterns; not filed individually pending operator direction on per-invention routing)
- Upstream gist: `gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00`

## Sycophancy compensation declaration

The author of this audit is the same identity that authored the 22 Class (c) inventions enumerated above. The natural bias is to soften (c) → (b) by claiming upstream motivation for inventions that have only-loose upstream connection. The audit resisted this by reading the upstream gist for explicit text covering each surface; surfaces without explicit text were classified (c) regardless of how defensible the invention seems.

The honest read: downstream vsdd-cli is *substantially more inventive than its upstream source* — most of the toolkit's methodology surface is downstream extension. This is reasonable for a methodology operationalization toolkit, but the audit-trail honesty is naming it as extension, not as derivation.
