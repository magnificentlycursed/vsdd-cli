<!-- hook-bypass[check-no-letter-clusters]: documentation of the letter-label antipattern by name; Defect 1 lens citations -->
<!-- hook-bypass[check-document-staleness]: review of pre-event drafts; quotes their stale claims as evidence -->
---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-05-28
phase: phase-3
scope: Adversarial review of fresh vsdd-cli baseline `README.md` + `DESIGN-METHODOLOGY.md` (pre-event drafts inherited at `fdb10d1`) against vsdd-suite Phase 3 IAR domains, per retrospective lessons from `vsdd-cli-wip/review-log/2026-05-28-vsdd-methodology.md` (Review 1 — 12 defect classes) + the upstream conformance audit at `vsdd-cli-wip/review-log/2026-05-28-upstream-audit.md` (Review 2 — 42 surfaces classified).
lens: VDD-IAR Alignment (lead — methodology-spec semantic coherence) + Solution Owner (contract specificity + scope discipline) + Documentation Reviewer (cold-reader cross-source consistency) + Technical Writer (prose quality + reference navigability)
source: director-raised
session_note: Inline-composed multi-lens review against the baseline pre-event drafts. Vsdd-suite primer 3 + 4 domain prompts (SOLUTION-OWNER-REVIEW, DOCUMENTATION-REVIEWER-REVIEW, TECHNICAL-WRITER-REVIEW, VDD-IAR-ALIGNMENT-REVIEW) loaded at session-start from local vsdd-suite at `guild-projects/guild-portfolio/vsdd-suite/` (operator-pointed at session). The vsdd-suite cold-context-per-domain dispatch gold standard is NOT met — this is skill-mode inline review per the retrospective's Defect 5 framing (composition discipline acknowledged; not cold-session-per-domain). Adversary cognitive-diversity gap noted (Claude-as-Builder + Claude-as-Adversary; operator-confirmed deliberate deviation pending vsdd-cli COMPATIBILITY.md authoring).
model: claude-opus-4-7
execution_method: inline main session
sycophancy_compensation: The same identity that authored the baseline drafts at commit `5ccf740` is reviewing them now. The same-identity sycophancy failure mode applies. Mitigation: every finding cross-checked against the retrospective's 12 defect classes + the upstream conformance audit; the test is "did the author rationalize this away when warm, that a cold reader holding the upstream gist + vsdd-suite Layer 1 spec would catch?" Inline-multi-domain-composed review additionally accumulates context across domains and reconciles findings, which softens adversarial pressure compared to cold-session-per-domain — flagged as a second sycophancy vector. The review's findings prioritize concrete evidence (line numbers, exact quotes) over judgment claims; rationalization-as-dismissal is the failure mode the format resists.
---

# VDD-IAR Alignment Review 1 — Baseline drafts vs retrospective lessons — 2026-05-28

**Tested against:** baseline commit `fdb10d1` (pre-design-positioning state inherited at fresh clone; no Phase 2c applicable — both files under review are pre-event drafts authored at initial commit `5ccf740`)

**Phase 5 surface:** not applicable (review is pre-implementation-phase)

## TL;DR

Baseline `README.md` (~1100 lines) + `DESIGN-METHODOLOGY.md` (~930 lines) are pre-event drafts inherited from the initial commit, before the session-defect retrospective. Reviewed against the retrospective's 12 defect classes + the upstream audit's 22 Class (c) invention list, the baseline drafts already exhibit a substantial subset of the defects the retrospective catalogued. The fresh authoring needs to systematically apply lessons rather than incrementally edit these drafts forward — most of the load-bearing framing (intent, Phase 5/6 optional, schema-validation-asserted-as-deployed, vsdd-cli-as-direct-implementation-of-gist) is wrong at the framing level, not the prose level.

**17 findings across 4 domain lenses, organized by lens then by retrospective-defect mapping.** (Initial 14 surfaced from partial-read + targeted grep; 3 additional surfaced from full-read pass per operator-directive 2026-05-28 "please read both in full." The full-read pass confirmed all initial findings with additional line-evidence + surfaced the 3 listed below.)

The strongest recurring pattern: the baseline drafts collapse the 3-layer model (gist → vsdd-suite → vsdd-cli) into a 2-layer framing (gist → vsdd-cli implementation), erasing vsdd-suite's role as the intermediate operationalization layer that vsdd-cli is supposed to cleanroom-implement. This is the same defect class as the Layer-0-vs-Layer-1 mis-measurement in the upstream audit; the framing fix is structural, not editorial.

## Governing references (per VDD-IAR-Alignment § Governing References)

- **Primary:** [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) (Layer 0; canonical methodology)
- **Intermediate (Layer 1):** vsdd-suite at the pinned cleanroom-ref commit `5789ad48f41e241b00b49cdbc82aa19370659a06` of `github.com/magnificentlycursed/guild-portfolio/vsdd-suite/`; locally available at `guild-projects/guild-portfolio/vsdd-suite/`
- **Project under review:** fresh vsdd-cli at baseline `fdb10d1`; lessons-source at `vsdd-cli-wip/review-log/2026-05-28-vsdd-methodology.md` + `vsdd-cli-wip/review-log/2026-05-28-upstream-audit.md` + `vsdd-cli-wip/review-log/2026-05-28-security.md` + `vsdd-cli-wip/CLEANROOM-RESTART-PROPOSAL.md`

The cleanroom restart's stated posture: vsdd-cli is a Layer 2 implementation of Layer 1 (vsdd-suite), not a direct implementation of Layer 0 (gist). The baseline drafts pre-date this clarification.

---

## Findings — VDD-IAR Alignment lens

### Finding 1 — Layer-model collapse (Dim 8 role integrity + Dim 7 cross-session consistency)

**Open**

`README.md:11`: "**This is not my methodology.** VSDD is dollspace's. This repo is one collaborator's *interpretation and implementation* of that methodology as a Rust toolkit."

`README.md:25-32`: "The relationship to the methodology + upstream: ... Methodology authorship is dollspace's. This toolkit implements + interprets the methodology; it does not author it. The canonical methodology lives in the VSDD whitepaper + its predecessor VDD whitepaper + the crosslink repo."

The framing positions vsdd-cli as direct implementation of dollspace's gist (Layer 0), with crosslink as substrate. vsdd-suite is mentioned only as evidence-source (`R## F##` / `G-###` references at line 51) and as a "sibling, not successor" (line 806) — not as the intermediate operationalization layer vsdd-cli is supposed to cleanroom-implement.

The retrospective's project-memory framing (`project_vsdd_cli_is_cleanroom_of_vsdd_suite.md`): "vsdd-cli is a cleanroom implementation of vsdd-suite ... Layer 0 dollspace gist → Layer 1 vsdd-suite (methodology operationalization) → Layer 2 vsdd-cli (cleanroom Rust impl)."

The upstream conformance audit re-classified most of vsdd-cli's "Class (c) inventions" measured against Layer 0 as likely (a) operationalizations of Layer 1 once measured against vsdd-suite. The README's Layer-0-direct framing is the root cause of mis-measurement.

**Proposed:** README's "Project identity" section names the 3-layer model explicitly: vsdd-cli implements vsdd-suite's operationalization of dollspace's gist; vsdd-cli is cleanroom-of-vsdd-suite, not direct-implementation-of-gist. Cite the pinned vsdd-suite commit. Reference the COMPATIBILITY.md (to be authored) for the deliberate deviations from vsdd-suite (intent removal; FINDINGS-INDEX absence; etc.).

**Classification:** Open. Raised to SO (DESIGN.md change authority surface).

### Finding 2 — Phase 5 + Phase 6 framed as optional (Dim 13 + Dim 14)

**Open**

`README.md:611`: "Phase 5 + Phase 6 are first-class methodology phases; projects choose whether to execute them."

`DESIGN-METHODOLOGY.md:101`: "Phase 5 + Phase 6 are first-class methodology phases per the whitepaper; projects choose whether to execute them."

Upstream gist Section IV Core Principle 7: "The system isn't done until specs, tests, implementation, *and* formal proofs have all independently survived adversarial review." Four-Dimensional Convergence is non-negotiable for VSDD. Phase 6 IS the Exit Signal; calling it optional is methodology-spirit violation.

The retrospective's Phase 5/6 amendments (applied to vsdd-cli-wip but not fresh vsdd-cli): both phases mandatory; partial-application is allowed per upstream Section VI ("use the parts that make sense") but is not VSDD conformance.

**Proposed:** strip "projects choose whether to execute" from both surfaces. Replace with "Phase 5 + Phase 6 are mandatory methodology phases per the upstream whitepaper (Core Principle 7, Four-Dimensional Convergence). Projects that opt out are running partial VSDD per the upstream's prototyping allowance and do not reach the Exit Signal."

**Classification:** Open. Raised to SO.

### Finding 3 — Phase 5 surface enumeration incomplete (4 listed; upstream lists 5)

**Open**

`README.md:607`: "5 Formal Hardening (Mutation Testing / Fuzz Testing / Purity Boundary Audit / Proof Execution)"

Upstream gist Phase 5 lists **5 surfaces**: Proof Execution + Fuzz Testing + **Security Hardening** (Wycheproof + Semgrep) + Mutation Testing + Purity Boundary Audit. The baseline omits Security Hardening as a discrete surface — the only upstream-named surface that lists specific tooling exemplars (Wycheproof for cryptographic edge cases; Semgrep for static analysis).

Property-based testing is upstream Phase 2a (Step 2a Test Suite Generation), not Phase 5. Baseline does not conflate the two, which is correct.

**Proposed:** update both surfaces to enumerate all 5 upstream Phase 5 surfaces. Property-based testing remains Phase 2a.

**Classification:** Open. Raised to SO.

### Finding 4 — Phase-domain composition matrix omits Security Hardening + lists 4 vs 5 enforcement layers (consistency)

**Open**

`README.md:629`: "5 Formal Hardening | `vsdd-phase-5` | QE + Security + SA"

`README.md:617`: "The matrix is load-bearing methodology, enforced at four layers (matrix declaration; per-primer instruction; pre-phase declaration; commit-time hook)."

Retrospective process-gap 6 + the subsequent methodology amendment added a 5th enforcement layer (prior-phase exit-signal enforcement) — addressed in `vsdd-cli-wip/methodology.md` after Defect 3 surfaced. The baseline draft predates this amendment.

`DESIGN-METHODOLOGY.md:105` § Phase-domain composition matrix: similar pre-amendment count expected (not fully read; likely matches).

**Proposed:** update enforcement-layer count to 5 + name the prior-phase exit-signal enforcement layer per retrospective Defect 3 amendment.

**Classification:** Open. Raised to SO.

---

## Findings — Solution Owner lens

### Finding 5 — Scope: vsdd-cli described as "implementation" but lists substantial extensions (Dim 4 over-engineering + Dim 9 assignment compliance)

**Open**

`README.md:111-123` § Center of gravity lists 9 surfaces, including:
- Observability subsystem (OTel + FinOps; line 116)
- Verification subsystem (~17 hooks; line 117)
- Schema enforcement layer (15 artifact classes; line 118)
- VSDD-EXXXX error catalog (~25 codes; line 442)
- 18 methodology event variants (line 312-322)

Upstream conformance audit (`vsdd-cli-wip/review-log/2026-05-28-upstream-audit.md`) measured these against Layer 0 as Class (c) inventions. Even when re-measured against Layer 1 (vsdd-suite), several remain vsdd-cli specializations:
- VSDD-EXXXX error catalog (vsdd-suite uses G-XXX finding refs, not error-code catalog)
- OTel + FinOps observability stack (vsdd-suite has no observability subsystem)
- 18 methodology event variants (vsdd-suite has no enumerated event variants)
- 15 artifact-class schema-validation registry (vsdd-suite has artifact classes but not the schema-validation infrastructure)

The "implementation" framing in README:11 mis-characterizes the actual posture. Per SO Dim 4 (over-engineering) + Dim 9 (assignment compliance): vsdd-cli adds substantial scope beyond what vsdd-suite ships. This is operator-directive-permissible — the operator chose this scope explicitly — but the audit-trail honesty discipline requires naming the extensions as extensions, not framing them as implementation.

**Proposed:** rename README § Center of gravity to "Center of gravity (vsdd-cli specializations over vsdd-suite)" and explicitly list which surfaces are Layer 1 implementations vs Layer 2 extensions. The COMPATIBILITY.md (to be authored) tracks the deliberate extensions + their motivating rationale.

**Classification:** Open. Raised to SO.

### Finding 6 — Intent enum still present (Dim 1 spec coverage + Dim 9 assignment compliance)

**Open**

`README.md:363`: "Standard FinOps disciplines applied: ... budget-vs-actual per intent-axis ..."

`DESIGN-METHODOLOGY.md:418`: "Methodology-spirit adherence — does the cycle's discipline-application match the methodology's intent?" (the "methodology's intent" form is acceptable as "methodology's spirit" framing per the retrospective's earlier fix at the same line in vsdd-cli-wip; baseline draft has not received this fix)

Per operator-directive 2026-05-28: intent enum removed entirely from vsdd-cli; per-feature axes is the sole composition calibration mechanism. The deliberate deviation from vsdd-suite (which still has intent per `G-162`) is captured in COMPATIBILITY.md (to be authored).

The baseline drafts pre-date intent removal. The fresh authoring must strip intent terminology across both files + author COMPATIBILITY.md documenting the deviation.

**Proposed:** strip intent references; replace `intent-axis` (line 363) with `per-feature axis`; replace `methodology's intent` (DESIGN-METHODOLOGY:418) with `methodology's spirit`.

**Classification:** Open. Raised to SO.

### Finding 7 — FINDINGS-INDEX + implementation-order roadmap antipattern (retrospective Defect 12)

**Open**

`DESIGN-METHODOLOGY.md:851` § Implementation order (header exists per the structural grep; full content not read but the section's presence indicates a roadmap-as-doc pattern).

`README.md:847` cross-references the DESIGN-METHODOLOGY implementation-order section.

Per retrospective Defect 12 + operator-directive 2026-05-28: implementation-order roadmaps belong in crosslink-tracked issues + milestones, not in inline markdown bullets. The DESIGN-METHODOLOGY § Implementation order section is a vestigial vsdd-suite-pattern artifact (vsdd-suite uses `suite-development/FINDINGS-INDEX.md` for analogous tracking).

**Proposed:** strip § Implementation order from DESIGN-METHODOLOGY; replace with a brief pointer to crosslink-issue-board (filtered by milestone). The roadmap content becomes a series of crosslink-tracked issues during fresh authoring.

**Classification:** Open. Raised to SO.

---

## Findings — Documentation Reviewer lens (cold-reader)

### Finding 8 — Forward-references to non-existent artifacts (Dim 6 documentation rot + Dim 1 clone-and-follow fidelity)

**Open**

`README.md:116`: "The methodology spec — concise governing prose (~250-350 lines) at `methodology.md` (project root for vsdd-using-projects; `vsdd-cli` repo root for the toolkit's own spec)"

`README.md:132-159`: "vsdd init ... Deploys 10 phase-primer skills + 16 per-domain skills ... Deploys ~17 methodology hooks ... Registers the 16 role-domain prompts ... Registers all 14 supplements ..."

Baseline fresh vsdd-cli at `fdb10d1` has **none of these artifacts**: no `methodology.md`, no `primers/`, no domain prompts, no supplements, no implemented hooks (the 2 hooks at `hooks/` were just copied from vsdd-cli-wip; not yet wired). The README describes a deployed system that does not exist.

Per DR Dim 1 (clone-and-follow): a cold reader following `cargo install vsdd && cd <project> && crosslink init && vsdd init` (README:132-135) would fail at `cargo install vsdd` (crate not published) and `vsdd init` (binary not built). The entire adoption section is aspirational without `spec_state` markers.

Per retrospective Defect 11: spec assertions about validation infrastructure require explicit `spec_state` markers (spec'd / implemented / deployed). Baseline drafts lack these markers throughout.

**Proposed:** add a § Spec state section to README clearly distinguishing what is spec'd vs implemented vs deployed at the current fresh vsdd-cli stage. Each major capability claim carries an explicit state marker (likely all `spec'd` at v0; transitions to `implemented` / `deployed` are visible at commit time via the marker).

**Classification:** Open. Raised to SO (methodology amendment surface).

### Finding 9 — Implicit-knowledge audit failures (Dim 2)

**Open**

`README.md:51`: "Evidence references: `R## F##` (e.g., `R78 F4`, `R91 F1`) refers to the existing-suite Review N Finding M ... `G-###` (e.g., `G-156`) refers to existing-suite governing findings at ... `PR #N` refers to existing-suite pull requests. Bookmark-cli-manual references like `TW R1 F2` are findings from the existing reference-example project's per-domain review logs."

The convention is declared; the references themselves are not clickable per-finding. A cold reader hitting "R78 F4 Surface A/B/C/D + R94 + PR #38/44/52 cluster-letter recurrences" (line 790) cannot click through to the actual findings. The reader must trust the citation without verification — exactly the implicit-knowledge failure mode DR Dim 2 names.

Adjacent: many methodology-specific terms used without first-use expansion (`IAR`, `MVR`, `VDD Roast`, `Hallucinated`, `Exit Signal`, `phase-domain composition matrix`, `four-dimensional convergence`) — some are expanded somewhere in the doc, but in author-order rather than landing-order.

**Proposed:** add per-finding links for cited R## F## / G-### references (e.g., `[R78 F4](https://github.com/.../vsdd-suite/suite-development/review-log/2026-05-19-...)` form). Or: limit citation density in forward-facing prose; relegate finding-evidence-density to a separate appendix.

**Classification:** Open. Routes to fresh-authoring sequencing — addressed as part of the rewrite, not as a separate fix.

### Finding 10 — Cross-source-consistency: `Surface A/B/C/D` documented as antipattern but the same labels used elsewhere

**Open**

`README.md:790` (documenting the antipattern): "Per multi-recurrence evidence (R78 F4 Surface A/B/C/D + R94 + PR #38/44/52 cluster-letter recurrences), `check-naming-discipline.py` fires `VSDD-E0160: letter-label-anti-pattern` for label patterns like `Surface [A-Z]`, `Cluster [A-Z]`, `Mode [A-Z]`, `Path [A-Z]`, `Tier [A-Z]`, `Pillar [N]`. Acceptable: `Dim N`, `Layer N`, `Round N`, `Finding N`, `Phase Na` — the concept-word is in the identifier."

This is documentation OF the antipattern — quoting the labels by name as the worked example of what NOT to do. Per the cleanroom hook deployment: the line is flagged by `hooks/check-no-letter-clusters.py` because the regex matches the literal text. The file-level bypass at the top of THIS review entry handles the same problem for the review-log file.

The README's documentation-of-the-antipattern lines need a file-level `<!-- hook-bypass[check-no-letter-clusters]: documenting the letter-label antipattern by name -->` in the first 5 lines.

**Proposed:** add the file-level bypass to README + a similar bypass to any other file that legitimately documents the antipattern by name (e.g., DR prompt's evaluation-dimension worked example).

**Classification:** Open.

---

## Findings — Technical Writer lens (authorial)

### Finding 11 — Documentation rot: baseline drafts describe a system that has not been built (Dim 2 documentation accuracy)

**Open**

Adjacent to Finding 8 but distinct lens: from the authorial perspective (TW), the baseline drafts were correct-at-authoring (the rebuild was *planned* to have all these surfaces) and are inaccurate-at-reading because nothing has been built yet. The pattern is exactly the rot-by-time TW Dim 2 names: "documentation describes the previous signature" / "function docstrings that describe the previous signature."

The defect class differs from regular rot — the baseline never described a built system; it described a *planned* system without marking the plans as plans. Per retrospective Defect 11 + the proposed `spec_state` discipline: pre-built spec assertions are honest when marked `spec'd`; the baseline drafts assert capabilities without such markers.

**Proposed:** TW + DR co-author the `spec_state`-marked rewrite per the Defect 11 amendment. Every capability claim carries `spec_state: spec'd | implemented | deployed`. Fresh authoring is the natural place to apply this — retrofit is more expensive than upfront discipline.

**Classification:** Open. Cross-cuts to SO Finding 8.

### Finding 12 — Inline-reference navigability (Dim 13)

**Open**

`README.md` has substantial linked references (`[VSDD whitepaper]`, `[crosslink]`, `[`vsdd-cli`]`, etc.) but many references to specific findings, primers, hooks, and event variants are unlinked:

- "R78 F4 Surface A/B/C/D + R94 + PR #38/44/52" (line 790) — no per-citation links
- "TW Layer-2→Layer-3 recurrence" (line 477) — no link
- "the Phase 5 hardening primer" mentioned without naming `primers/5-formal-hardening.md`
- Event variant names (PhaseEntered, FindingRaised, etc. at lines 315-322) — no anchor links to their definitions
- Error-catalog codes referenced extensively without per-code anchor links

Per TW Dim 13 (inline-reference navigability) + the operator wordings the dim cites: "These should be markdown links so that a human can click through" / "Mentions of software, people, documents, etc. should have links too to properly credit the projects."

**Proposed:** systematic link sweep during fresh authoring; per-finding R## F## links to actual vsdd-suite review-log entries; per-event-variant anchor links within the event-variant catalog section; per-error-code anchor links within the error catalog.

**Classification:** Open. Routes to fresh-authoring rewrite scope.

### Finding 13 — AI session independence: methodology context lives in vsdd-cli-wip review-log entries; fresh vsdd-cli's README doesn't reference them (Dim 10)

**Open**

The retrospective + upstream audit + security review + cleanroom restart proposal in `vsdd-cli-wip/review-log/2026-05-28-*.md` + `vsdd-cli-wip/CLEANROOM-RESTART-PROPOSAL.md` together constitute the load-bearing methodology-evolution context for fresh vsdd-cli. Without these references, a future reader cannot understand why fresh vsdd-cli made the decisions it did (intent removal; Phase 5/6 mandatory; vsdd-suite cleanroom posture; 2 vsdd-cli specialization hooks; etc.).

Per TW Dim 10 (AI session independence): "Is the knowledge required to understand and maintain this project documented in the project artifacts, or does it exist only in AI conversation history?" The retrospective is the *artifact* form of the session history; referencing it from README closes the AI-session-independence gap.

**Proposed:** fresh vsdd-cli README links to vsdd-cli-wip's review-log entries + CLEANROOM-RESTART-PROPOSAL.md under a "Historical context" or "Lessons applied" section. The wip remains accessible via the `origin/wip-archive` branch + the directory at `<parent-dir>/vsdd-cli-wip/`.

**Classification:** Open. Routes to fresh-authoring rewrite scope.

### Finding 14 — Vestigial vsdd-suite framing: "intent calibration" / "FINDINGS-INDEX" / "implementation order" survive in the baseline (cross-cutting; TW + SO + VDD-IAR)

**Open**

Catalog of vestigial vsdd-suite patterns surviving in baseline (synthesizing prior findings):
- `intent-axis` / `methodology's intent` (Finding 6)
- `## Implementation order` section in DESIGN-METHODOLOGY:851 (Finding 7)
- `existing-suite Layer 1 anti-pattern` reference (README:643 + DESIGN-METHODOLOGY:141) — descriptive citation of vsdd-suite's history, OK as-is
- 3-layer model not made explicit (Finding 1)
- Phase 5 surfaces (Finding 3 — Security Hardening omitted)
- Phase 5 + Phase 6 optional framing (Finding 2)

Per the retrospective's "vestigial structures that leak in from using vsdd-suite during development must be migrated to vsdd-cli forms as dogfooding progresses" directive: these patterns are migration targets, not edit-in-place fixes.

**Proposed:** the fresh authoring pass IS the migration. Each fresh artifact is authored applying the lessons; vestigial patterns are eliminated by structural rewrite, not by spot-edits to the baseline drafts.

**Classification:** Open. Meta-finding cross-referencing F1, F2, F3, F6, F7.

### Finding 15 — Exit Signal conflation: Phase 3 reaching only-Hallucinated framed as the Exit Signal (Dim 14 four-dimensional convergence)

**Open**

`README.md:857`: "The toolkit's success criterion is the methodology applied to itself: when an adversarial reviewer runs Phase 3 against the `vsdd-cli` repository and produces only hallucinated findings, the Exit Signal is reached."

`DESIGN-METHODOLOGY.md:847`: "When an adversarial reviewer runs Phase 3 against the `vsdd-cli` repository and produces only hallucinated findings, the Exit Signal is reached. Hallucinated classification requires explicit demonstration of non-applicability per primer 3 discipline."

Per upstream gist Phase 6 + Core Principle 7: the Exit Signal is **four-dimensional convergence** (Spec MVR + Test MVR + Implementation MVR + Formal-verification MVR + cross-dimension consistency check) — NOT a single Phase 3 round producing only-Hallucinated findings. Phase 3 only-Hallucinated gives **Implementation MVR for that round** (one of four dimensions, one round of many).

Per VDD-IAR-Alignment Dim 14: "the convergence round's consistency-check table has one row per spec-named behavior and zero inconsistent rows at convergence-declaration time."

The baseline's Exit Signal framing is wrong on two axes: (a) conflates Phase 3 MVR with Phase 6 four-dimensional convergence; (b) reduces Phase 6's attestation discipline to a single Phase 3 outcome.

**Proposed:** rewrite the success criterion in both surfaces to: "The toolkit's success criterion is the methodology applied to itself: when Phase 6 four-dimensional convergence (Spec MVR + Test MVR + Implementation MVR + Formal-verification MVR + cross-dimension consistency) is attested for the `vsdd-cli` repository, the Exit Signal is reached. Per upstream gist Phase 6 + Core Principle 7."

**Classification:** Open. Raised to SO.

### Finding 16 — VSDD Methodology meta-domain rename from "VDD-IAR Alignment" (Layer 1 deviation; vsdd-cli specialization)

**Open**

`DESIGN-METHODOLOGY.md:412`: "The meta-domain renamed from VDD-IAR Alignment."

vsdd-suite Layer 1 names this meta-domain `vdd-iar-alignment` (per the loaded `vsdd-suite-vdd-iar.md` at local path `guild-projects/guild-portfolio/vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md`). vsdd-cli renames it to `vsdd-methodology`. This is a deliberate Layer 2 specialization vs Layer 1.

The DESIGN-METHODOLOGY notes the rename matter-of-factly without naming it as a deviation from vsdd-suite. Per the retrospective's cleanroom-of-vsdd-suite framing + the deliberate-deviations discipline (intent removal precedent), specializations vs Layer 1 should be captured in COMPATIBILITY.md (to be authored) with explicit rationale.

This same review log uses the vsdd-suite slug `vdd-iar-alignment.md` for the file name — the slug IS the Layer 1 name. vsdd-cli's internal domain prompt slug `vsdd-methodology` deviates from vsdd-suite's `vdd-iar-alignment`. The review-log slug discipline + internal-domain-slug discipline cross-validate: this review's slug is the vsdd-suite-canonical form even though vsdd-cli internally calls it something different.

**Proposed:** COMPATIBILITY.md (to be authored) declares the rename as deliberate vsdd-cli specialization + names rationale (likely: "VSDD Methodology" reads cleaner than "VDD-IAR Alignment" once IAR is no longer an internal-vsdd-suite-specific concept). For review-log filename discipline: stick with the vsdd-suite slug `vdd-iar-alignment` (Layer 1 canonical form) since that's the per-approved-set authority.

**Classification:** Open. Routes to COMPATIBILITY.md authoring.

### Finding 17 — Layer-cycle PR discipline + 4-cluster Phase 3 batching + MCP server tool surface: vsdd-cli specializations without upstream basis (Layer 0 audit residue)

**Open**

`DESIGN-METHODOLOGY.md:139-167` § Layer-cycle PR discipline — declares PR-as-layer discipline with DraftPROpened → PRReadyForReview → PRMerged event sequence, PR template artifact class, check-pr-manual-test-completion hook. Upstream gist Section II Phase 4 § Feedback Integration mentions routing back through phases but has NO PR concept (no DraftPROpened, no PR template, no PR-discipline hooks).

`DESIGN-METHODOLOGY.md:307-322` § Cluster-batching shape — declares 4-cluster default (Implementation / Architecture / Communication / Adversarial) with specific cluster compositions + adversarial-pair-separation invariant. Upstream gist describes single-Adversary-per-pass with context reset; the 4-cluster shape is downstream invention motivated by multi-agent cost optimization. The line `Cluster naming: descriptive (Implementation / Architecture / Communication / Adversarial), not letter-coded` (line 322) correctly avoids the letter-label anti-pattern but the 4-cluster shape itself remains downstream.

`DESIGN-METHODOLOGY.md:378-407` § MCP server tool surface — declares 4 MCP tools (vsdd.methodology.lookup; claude_code.docs.search; crosslink.docs.search; anthropic.api.docs.search). Upstream gist + vsdd-suite have no MCP server concept; this is wholly vsdd-cli specialization.

Per the upstream conformance audit at `vsdd-cli-wip/review-log/2026-05-28-upstream-audit.md`: these are Class (c) inventions when measured against Layer 0. When re-measured against Layer 1 (vsdd-suite), Layer-cycle PR discipline likely remains (c) (no vsdd-suite PR-as-layer pattern); 4-cluster batching is likely (c) (vsdd-suite has `crosslink swarm review --agents N` but not the 4-cluster shape with adversarial-pair separation); MCP server is wholly (c).

These are not findings AGAINST these surfaces — they may all be defensible vsdd-cli specializations. The finding is that they are not surfaced as specializations; the DESIGN doc presents them as part of "the methodology" without acknowledging the Layer 2 invention status. The audit-trail honesty discipline (per VSDD Methodology Dim 4 — methodology-evolution coherence) requires naming inventions as inventions.

**Proposed:** COMPATIBILITY.md (to be authored) catalogs these three as deliberate vsdd-cli specializations with rationale (likely: PR-as-layer leverages GitHub native workflow; 4-cluster batching reduces token cost at multi-agent scale; MCP server provides agent-self-serve docs lookup). DESIGN-METHODOLOGY § Cluster-batching shape + § Layer-cycle PR discipline + § MCP server add a one-line "vsdd-cli Layer 2 specialization; see COMPATIBILITY.md for upstream relationship" pointer.

**Classification:** Open. Routes to COMPATIBILITY.md authoring + DESIGN-METHODOLOGY rewrite.

---

## Phase 4 routing

Per primer 3 § Round closing: every Phase 3 round entry's closing block must include a `**Phase 4 routing:** <reference>` field.

**Phase 4 routing:** Round 1 findings inform fresh-authoring sequencing rather than routing to specific phase destinations. All 14 findings are intended to be applied during DESIGN.md + methodology.md + README + COMPATIBILITY.md authoring (Phase 1a + 1b at fresh-vsdd-cli scope). Routing destinations per finding:

| Finding | Domain | Routes to |
|---|---|---|
| F1 (Layer-model collapse) | VDD-IAR | Phase 1a (DESIGN.md § Project identity + methodology.md § 3-layer model) |
| F2 (Phase 5/6 optional) | VDD-IAR | Phase 1a (methodology.md § Phase taxonomy + DESIGN-METHODOLOGY rewrite) |
| F3 (Phase 5 surfaces) | VDD-IAR | Phase 1a (methodology.md Phase 5 enumeration + README adoption section) |
| F4 (composition matrix amendments) | VDD-IAR | Phase 1a (methodology.md § Phase-domain composition matrix) |
| F5 (scope: extensions vs implementation) | SO | Phase 1a (README § Center of gravity restructure) + COMPATIBILITY.md authoring |
| F6 (intent removal) | SO | Phase 1a (strip intent across all surfaces) |
| F7 (FINDINGS-INDEX + implementation-order) | SO | Phase 1a (DESIGN-METHODOLOGY rewrite; crosslink-issue migration) |
| F8 (forward-reference to non-existent artifacts) | DR | Phase 1a (add `spec_state` markers across spec assertions) |
| F9 (implicit-knowledge audit) | DR | Phase 1a (link sweep during rewrite) |
| F10 (Surface A/B/C/D documentation) | DR | Add `<!-- hook-bypass -->` to README + adjacent files |
| F11 (documentation rot — baseline pre-implementation) | TW | Phase 1a (TW + DR co-author per Defect 11 amendment) |
| F12 (inline-reference navigability) | TW | Phase 1a (link sweep) |
| F13 (AI session independence) | TW | Phase 1a (README § Historical context / Lessons applied) |
| F14 (vestigial vsdd-suite framing meta-finding) | TW + SO + VDD-IAR | meta-routing pointer; fresh authoring IS the migration |
| F15 (Exit Signal conflation Phase 3 vs Phase 6) | VDD-IAR | Phase 1a (rewrite success criterion in README + DESIGN-METHODOLOGY) |
| F16 (VSDD Methodology meta-domain rename) | VDD-IAR | COMPATIBILITY.md authoring |
| F17 (PR discipline + cluster batching + MCP server as vsdd-cli specializations) | VDD-IAR + SO | COMPATIBILITY.md authoring + DESIGN-METHODOLOGY rewrite |

Aggregate routing pattern: 14 of 17 findings route to Phase 1a fresh authoring + COMPATIBILITY.md authoring. The remaining 3 (F4, F10, F16) route to local fixes (composition matrix amendment + hook-bypass marker addition + COMPATIBILITY.md entry).

---

## Closing

Round 1 produced 17 real findings across 4 domain lenses (14 surfaced from initial pass; 3 additional surfaced from full-read follow-up per operator-directive). Per primer 3 § Round triggers § Continue trigger (G-131): any round producing new real findings triggers Round N+1. Round 2 would apply against the fresh-authored artifacts after Phase 1a rewrite lands — verifying the findings are actually addressed + checking for adjacent defects the fix may have created.

The full-read pass also confirmed every initial finding with additional line evidence: F2 (Phase 5/6 optional) corroborated at DESIGN-METHODOLOGY:101; F4 (4 vs 5 enforcement layers) corroborated at DESIGN-METHODOLOGY:107; F6 (intent surviving) corroborated at DESIGN-METHODOLOGY:418 + README:656 § Intent calibration via per-feature axes section header + README:665-666 + 678-679 + DESIGN-METHODOLOGY:297-298; F7 (FINDINGS-INDEX + implementation order) corroborated at DESIGN-METHODOLOGY:851; F8 (forward-references to non-existent artifacts) corroborated extensively at DESIGN-METHODOLOGY:769-848 (Acceptance criteria for deliverables describes capabilities none of which exist). The shortcut-via-grep approach undercounted findings + missed the rename-from-VDD-IAR specialization (F16) + the Exit Signal conflation (F15) entirely. The full-read pass was the right discipline per the operator-directive correction.

**Round close trigger** (for Round 2 attestation): N/A — this is Round 1; the trigger discipline applies starting at Round 2.

Per the retrospective sycophancy-compensation discipline: the same-identity overlap is documented; the inline-multi-domain-composition softening is documented; the findings prioritize concrete evidence (line numbers, exact quotes) over judgment. The author's bias toward rationalizing baseline drafts as "this was authored before the lessons were learned, so it gets a pass" was resisted — pre-event authorship is the *origin* of the defect classes, not an exemption from them.

The single load-bearing observation: the baseline drafts encode the wrong layer model (Layer-0-direct rather than Layer-2-via-Layer-1). Every other finding cascades from this structural framing. Fresh authoring that fixes the layer model + adopts the `spec_state` discipline + strips the vestigial vsdd-suite patterns produces a coherent baseline; spot-editing the existing drafts leaves the structural framing wrong.

## Sycophancy compensation declaration

The author of this review is the same identity that authored the baseline drafts under review. The findings here are checked against the retrospective's 12 defect classes + the upstream audit's surface-by-surface classification — not against fresh judgment. The discipline: if a finding's evidence comes from the retrospective + a current grep on the baseline matches, the finding is real regardless of how the warm author rationalizes it. The 14 findings above each cite specific lines + reference specific defect classes; rationalization-as-dismissal is the failure mode the format resists.

The inline-multi-domain-composition is the second documented vector. Mitigated by structuring findings per-lens (VDD-IAR / SO / DR / TW sections) so the per-domain pressure is preserved rather than reconciled across lenses. The cold-context-per-domain dispatch via vsdd-suite's `crosslink swarm review --agents N --mandate adversarial` is the gold standard; this review's inline form does not meet it.

The Adversary cognitive-diversity gap (per upstream gist Section V — different model family recommended for Adversary vs Builder) is unmitigated. Claude is both the Builder (authored baseline) and the Adversary (this review). Operator-confirmed deliberate deviation pending COMPATIBILITY.md authoring.
