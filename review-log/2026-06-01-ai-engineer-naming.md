---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-06-01
phase: phase-3
scope: AIE domain review of the recurring naming/sequencing-vocabulary problem and effective mitigations. Subject: three parallel sequencing vocabularies that accumulated in this conversation (VSDD's canonical 10 phases + invented "Phase A-F" + invented "Step 0.5/2.1/2.2/2.3" overlay on operator's 5-step bootstrap plan) and the cost the multiplication has imposed on this and prior conversations.
lens: AIE Dimensions 1-8 weighted on Dim 3 (prompt-cache discipline) + Dim 5 (sub-agent scope-down) + Dim 6 (model-tier right-sizing) + Dim 1 (capture-source provenance). Supplement applied: claude-code-cli.md.
source: operator-directive
session_note: Inline single-domain AIE consultation per operator directive ("AI Engineer review using the claude code supplement about the recurring naming problems and effective mitigations"). InlineMultiDomain composition shape (per the just-landed M1 amendment) — single-domain variant.
model: claude-opus-4-7
execution_method: inline main session; AIE primer + claude-code-cli supplement both loaded; vsdd-cli/supplements/claude-code-cli.md AI Engineer extensions in scope
sycophancy_compensation: |
  I authored every instance of the recurring naming problem under review (the "Phase A-F"
  invention in V1-SHIP-CRITERIA; the "Step 0.5" + sub-step layering; the 5-step plan ←→ VSDD
  phase ad-hoc mappings; the parallel-design-tree-with-its-own-vocabulary). Bias: declare
  "the naming proliferation was reasonable given novel design surface." Compensation:
  every finding cites either (a) a specific AIE dimension I can ground in the claude-code-cli
  supplement, or (b) a concrete cost figure (token estimates; cache-hit-rate impact). Where
  judgment is "I think the vocabulary was useful," I raise to Platform Engineer (validator
  pair) for adjudication rather than self-validating.
---

# AI Engineer Review — Recurring naming problems and effective mitigations (2026-06-01)

**Operator directive:** "Please write an AI Engineer review using the claude code supplement about the recurring naming problems and effective mitigations."

**Predecessor context:** Operator named the recurring problem explicitly: "I have lost track of what the phases and steps even mean which you may recall is a recurring problem I have been trying to solve."

---

## Pre-phase composition declaration

```yaml
phase: phase-3
composed_domains: [ai-engineer]
composition_mode: inline-single-domain
memory_isolation: NONE
operator_confirmation: confirmed
declared_at: 2026-06-01
supplements_loaded: [claude-code-cli]
methodology_alignment: |
  Operates under M1 InlineMultiDomain amendment landed at vsdd-cli commit 60eb150.
  Single-domain variant — operator-directive activation; bounded scope; evidence-grounded
  findings (cost estimates; cache impact); judgment-only findings raised to PE per
  AIE → PE → SO escalation chain.
sycophancy_compensation: see frontmatter
```

---

## Scope

The recurring naming problem as it manifests in vsdd-cli + the wider methodology + this conversation specifically:

**Three parallel sequencing vocabularies active simultaneously:**

| Vocabulary | Source | Lifespan |
|---|---|---|
| VSDD's 10 canonical phases (1a / 1b / 1c / 2a / 2b / 2c / 3 / 4 / 5 / 6) | methodology.md + phase primers | Persistent across all VSDD-using projects |
| "Phase A-F" (Foundational / CLI / Agent / MCP / LSP / Onboarding) | mdatron V1-SHIP-CRITERIA.md (authored 2026-06-01) | Invented this conversation |
| "Step 0.5 / Step 1 / Step 2 / sub-step 2.1 / 2.2 / 2.3" | Layered on operator's 5-step bootstrap plan | Operator's original framing + my invented sub-step overlay |

Plus ad-hoc mappings between them (e.g. "Step 3 = Phase 2b"; "Step 2.2 = Phase 1a") that don't hold consistently — Step 2.2 includes a Phase 4 routing pass + a Phase 1a revision; Step 3 includes Phase 2a Red Gate + Phase 2b Green + Rust supplement PE work + dependency approval per VSDD-E0100.

Per the operator's own framing, this is a **recurring** problem ("you may recall is a recurring problem I have been trying to solve"). AIE's job in this review: what does the AI-runtime cost surface tell us about effective mitigations specifically?

---

## Findings

### AIE-NAME-F1 — Vocabulary instability busts prompt-cache hits (Open; load-bearing)

**Per claude-code-cli supplement § AI Engineer extensions + AIE Dim 3 (prompt-cache discipline).** The supplement names prompt-cache 5-minute default TTL (API key auth) and 1-hour opt-in TTL via `ENABLE_PROMPT_CACHING_1H=1`. Cache hits depend on **prefix stability** — the leading content of a prompt being identical across turns.

Multi-vocabulary conversations generate vocabulary-disambiguation tokens on every turn ("by 'Step 3' here I mean the operator's bootstrap plan step 3, which is Phase 2b in VSDD's taxonomy and overlaps with mdatron Phase A.1+B implementation"). These tokens vary turn-to-turn because the operator's reference resolves differently per turn. Net: every turn imports vocabulary-disambiguation into its prefix; prefix differs from prior turn's prefix; cache-write cost paid; cache-hit savings lost.

**Estimated cost** (rough; client-side estimate per claude-code-cli supplement's `capture_source: sdk-result-message` convention; not Usage-API-reconciled):

- Per-turn vocabulary disambiguation: ~200-500 tokens
- This conversation has ~150 turns
- If 80% of turns had vocabulary-disambiguation tokens that busted cache: ~150 × 0.8 × 350 ≈ 42,000 tokens of cache-miss overhead

Cache misses cost the cache-write multiplier (typically ~1.25× input cost for the first occurrence) AND the missed savings on subsequent reads (~0.1× input cost for hits). For ~42k cache-busting tokens, the unrecovered cost is in the ~$0.30-1.00 range per conversation. Small per conversation; load-bearing over many conversations.

**Mitigation:** vocabulary stability discipline (see § Effective mitigations § M1 below).

**Routes to:** Platform Engineer (validator pair) for cost-observability instrumentation that would surface this in OTel signals.

### AIE-NAME-F2 — Sub-agent delegation bloats prompts when vocabulary unstable (Open)

**Per claude-code-cli supplement § "Sub-agent delegation via Task tool" + AIE Dim 5 (sub-agent scope-down discipline).** When the canonical vocabulary is stable, sub-agent prompts are short and focused ("review this artifact per Phase 3 cluster-batched adversarial discipline"). When unstable, sub-agent prompts must include disambiguation glossaries OR risk sub-agent confusion.

Concrete example from this conversation: the DSL falsifiability sub-agent (spawned for Mitigation 4) received ~21k input tokens with elaborate context-establishment. A stable-vocabulary version of the same task could have been ~5-8k tokens with the same task quality, per AIE's sub-agent scope-down rubric.

**Estimated cost** of vocabulary-instability sub-agent overhead, per sub-agent spawn: ~10-15k extra input tokens. This conversation spawned 1 sub-agent so the overhead is ~$0.05-0.20 in absolute terms. Becomes load-bearing when sub-agent spawns scale.

**Routes to:** PE for sub-agent prompt template improvements (see § M2 below); SO for budget-implication recognition.

### AIE-NAME-F3 — Multi-vocabulary makes capture-source provenance ambiguous (Open)

**Per claude-code-cli supplement § "Capture-source provenance" + AIE Dim 1.** The supplement names the 7-value `capture_source` enum (otel-metric / otel-log-event / otel-trace-attribute / vsdd-custom-event / sdk-result-message / usage-api-reconciled / unmeasurable). The enum classifies WHERE cost-relevant events come from.

When a finding emerges in this conversation, asking "which phase/step did this emerge in?" has multiple valid answers (Phase 4 routing? Step 2.2 substantive work? mdatron Phase A planning?). The audit-trail event payload (`phase` field in `PhaseCompositionDeclared`, `FindingRaised`, etc.) becomes ambiguous because phase resolution depends on which vocabulary the recording agent had loaded.

Forensic audit at v1+ (when adopters investigate "where did this drift come from?") faces extra disambiguation cost per audit query: "which vocabulary was active when this event was recorded?" Operator-time-binding cost; not AI-runtime cost directly but adjacent.

**Mitigation:** see § M3 below (canonical-vocabulary-only in event payloads).

**Routes to:** vsdd-methodology meta domain for event-schema review.

### AIE-NAME-F4 — Model-tier downgrade blocked by vocabulary instability (Open; recurrent)

**Per claude-code-cli supplement § AI Engineer extensions implicitly (via SDK message stream cost discipline) + AIE Dim 6 (model-tier right-sizing) + sycophancy_failure_mode #5.** Mechanical work (file edits; cargo iteration; simple template fills) is the canonical downgrade-candidate-class — typical mechanical task suits Sonnet or Haiku at ~3-10× cheaper than Opus.

But downgrade requires confidence that the cheaper model understands task context. When task context requires negotiating three sequencing vocabularies + the meta-mapping between them, smaller-model context-understanding is at risk. Per-task downgrade decisions default to "stay on Opus to be safe" because the disambiguation overhead requires Opus-class context handling.

**Concrete recurrence:** every Phase 2a / Phase 2b / Step-X commit message in this conversation runs on Opus 4.7. Per AIE-S3-F3 (recurrent finding from prior reviews this conversation), much of that work is mechanical. Downgrade not attempted partly because the per-task scope is "implement the validator engine, including its Phase 2a / 2b / Step-3 / Phase A.1 sequencing" — that compound description exceeds what a cheaper model can confidently parse.

Stable vocabulary unlocks the downgrade. "Implement Phase 2b Green for the schema module" reads cleanly to Haiku. "Implement Step 3 sub-step Phase A.1 iteration B" requires Opus.

**Routes to:** SO via AIE → PE → SO escalation (cost-discipline methodology amendment).

### AIE-NAME-F5 — Vocabulary-debt-finding-class is not first-class (Open; foundational gap)

When invented vocabulary lands without explicit deprecation of prior variants, the variants don't accumulate in a tracked debt list — they accumulate in the operator's working memory + in conversation context. There is no canonical "Vocabulary debt" finding class in vsdd-cli's catalog.

The recently-landed M1 amendment formalizes InlineMultiDomain composition shape as first-class. Analogous formalization for the "non-canonical sequencing vocabulary" pattern doesn't exist. Without it, every conversation re-discovers the recurring problem.

**Proposed:** new finding category — `vocabulary-drift` — fires when (a) work introduces sequencing/taxonomy vocabulary not in the canonical methodology spec, OR (b) ad-hoc mappings between canonical and non-canonical vocabularies appear in artifacts. Severity: warning. Promotes to error when accumulated count exceeds threshold per project.

**Routes to:** vsdd-methodology meta domain for catalog amendment (parallel structure to M1's vocabulary-drift codification).

### AIE-NAME-F6 — Skill prompt vocabulary stability is an unmonitored surface (Open)

**Per claude-code-cli supplement § "Slash command discipline" + § AI Engineer extensions.** The supplement names `.claude/commands/<name>.md` as the slash-command surface. vsdd-cli's domain skills (`/vsdd-domain-quality-engineer`, etc.) use canonical VSDD phase vocabulary throughout. Good.

But ad-hoc work in conversations (this one included) introduces non-canonical vocabulary that the operator then has to translate when invoking a domain skill ("when I say Step 3 below, the skill's Phase 2a discipline applies"). The translation happens in the operator's prompt to the skill, not in the skill itself. The skill output may use canonical vocabulary; the operator's input mixes vocabularies; the resulting context has both.

**Per AIE Dim 5 (sub-agent scope-down):** every conversation that mixes vocabularies before invoking a skill imports the disambiguation cost into the skill's working context. Skill-output quality degrades when its input context includes non-canonical vocabulary it must mentally translate.

**Mitigation:** vocabulary check at session start (see § M5 below).

**Routes to:** PE for skill-deployment discipline; AIE retains for prompt-cost accounting.

---

## Effective mitigations (AIE-grounded)

### M1: Vocabulary stability discipline (operator-chosen 2026-06-01; ratify)

The operator's just-made choice — "VSDD's 10 phases + plain language; retire 'Phase A-F' and 'Step 0.5/2.1/2.2/2.3'" — is **the AIE-optimal mitigation**. It maximizes prompt-cache prefix stability (AIE-NAME-F1), unblocks model-tier downgrade (AIE-NAME-F4), and ends the skill-input-translation cost (AIE-NAME-F6).

AIE supports unconditionally. Recommend formalizing as methodology discipline: every new sequencing/taxonomy vocabulary requires explicit predecessor-deprecation declaration; otherwise warning fires at next conversation start.

### M2: Sub-agent prompt template with canonical-vocabulary glossary

**Per claude-code-cli supplement § "Sub-agent delegation via Task tool".** Sub-agent prompts for VSDD-context work should include a brief (~200-token) canonical-vocabulary glossary at the top of the prompt:

```
You are operating in a VSDD-methodology context. Canonical phase vocabulary:
  Phase 1a — Behavioral specification    Phase 2a — Red Gate (failing tests first)
  Phase 1b — Verification architecture   Phase 2b — Implementation (Red → Green)
  Phase 1c — Decomposition                Phase 2c — Refactor
  Phase 3  — Adversarial Refinement       Phase 4  — Routing
  Phase 5  — Purity boundary audit        Phase 6  — Exit Signal attestation

Do NOT use non-canonical sequencing vocabulary. If asked about "Step 0.5", "Step 2.2",
"Phase A", or similar, ask for canonical-phase mapping before proceeding.
```

The glossary is **stable across all sub-agent invocations** — so it cache-warms cleanly. Sub-agent prompts after the glossary can be terse + canonical-vocabulary-only.

### M3: Canonical-vocabulary-only in event payloads (AIE-NAME-F3 resolution)

Per the M1 InlineMultiDomain amendment's per-finding annotation discipline: the `phase` field on `PhaseCompositionDeclared` / `FindingRaised` / etc. events must contain canonical VSDD phase values only. Non-canonical sequencing vocabulary appears in human-readable summary fields (e.g. `methodology_deviation_summary`) but does not appear in machine-indexed fields.

Forensic audit queries become unambiguous: "show all events at phase: 2b" returns a clean set, not a vocabulary-resolution problem.

### M4: Vocabulary-drift finding class (AIE-NAME-F5 resolution)

Catalog amendment: add `VSDD-W0210: non-canonical-sequencing-vocabulary` (candidate per the M1 earned-by-recurrence trigger). Fires when commit messages, design docs, or knowledge pages introduce sequencing/taxonomy vocabulary not in the canonical methodology spec.

Implementation: bootstrap-validator.py extension (or its successor when mdatron's engine is operational) checks committed text against a registered canonical-vocabulary list.

### M5: Per-conversation vocabulary check at session start (AIE-NAME-F6 resolution)

At session start (per claude-code-cli supplement § "`.claude/settings.json` discipline" — settings.json's SessionStart hook), emit a 1-line vocabulary check:

```
Active sequencing vocabulary: canonical VSDD 10 phases (per methodology.md).
Non-canonical references in this session will be flagged for translation.
```

This single line at conversation start anchors the vocabulary; subsequent agent inferences cache against it cleanly.

### M6: Cost-observability instrumentation for vocabulary-cost (AIE-NAME-F1 resolution)

Per claude-code-cli supplement § "Capture-source provenance" — extend OTel collector to emit a `VocabularyDriftDetected` event variant when conversation context includes non-canonical sequencing vocabulary. Event carries: vocabulary_terms_used, estimated_cache_impact_tokens, estimated_disambiguation_cost. Surfaces the cost AIE-NAME-F1 names in real-time dashboards.

Reserved as v1+ deliverable; not load-bearing at v1.0.

---

## Recommendations (priority-ranked)

1. **Adopt the operator's vocabulary-stability choice unconditionally** (M1). This is the highest-leverage mitigation — it addresses 4 of 6 findings simultaneously at zero implementation cost. The choice has been made; this review ratifies it.

2. **Add the sub-agent prompt template** (M2) when next sub-agent is spawned. Use the glossary block above (or similar). Don't bloat further.

3. **Land `VSDD-W0210: non-canonical-sequencing-vocabulary`** as candidate in the next vsdd-cli error-catalog revision (M4). Earned-by-recurrence trigger met (this conversation + prior conversations the operator referenced).

4. **Defer M3 (event-payload vocabulary discipline)** until event-emission tooling is operational (per BOOTSTRAP-MITIGATION § Mitigation 3 bootstrap-period exception). Note in the methodology spec.

5. **Defer M5 (session-start vocabulary check)** until SessionStart hook integration matures.

6. **Defer M6 (cost-observability instrumentation)** to v1+ per Anthropic Usage API integration scope.

---

## Classification summary

6 findings: 6 Open; 1 marked load-bearing (AIE-NAME-F1, prompt-cache impact).

| Code | Severity | Status | Routes to |
|---|---|---|---|
| AIE-NAME-F1 | Warning | Open (load-bearing) | Platform Engineer |
| AIE-NAME-F2 | Warning | Open | Platform Engineer + Solution Owner via escalation |
| AIE-NAME-F3 | Lint | Open | vsdd-methodology meta |
| AIE-NAME-F4 | Warning | Open (recurrent — third instance per AIE-S3-F3 + AIE-MITIG-F5 + this) | Solution Owner via escalation |
| AIE-NAME-F5 | Error | Open (foundational gap) | vsdd-methodology meta |
| AIE-NAME-F6 | Lint | Open | Platform Engineer |

## Coordination

- **Platform Engineer (validator pair):** AIE-NAME-F1 (prompt-cache cost), AIE-NAME-F2 (sub-agent overhead), AIE-NAME-F6 (skill input vocabulary). Single PE consultation closes three.
- **vsdd-methodology meta domain:** AIE-NAME-F3 (event-payload vocabulary), AIE-NAME-F5 (vocabulary-drift finding class). Both land in the same catalog amendment.
- **Solution Owner via AIE → PE → SO escalation:** AIE-NAME-F2 (sub-agent budget recognition), AIE-NAME-F4 (model-tier downgrade rubric — third recurrence).
- **Sanity-Check:** not invoked.

## Recurrence-pattern note

AIE-NAME-F4 (model-tier downgrade blocked) is recurrent — third instance this conversation (AIE-S3-F3 in informal weigh-in earlier; AIE-MITIG-F5 in 2026-06-01 AIE review on mitigation techniques; this finding). Earned-by-recurrence trigger met. SO escalation recommended; per-Phase-2b-commit cost-tier check is the proposed methodology amendment.

The M1 amendment + this AIE-NAME review together address the meta-pattern: VSDD has had ambient confusion about its own sequencing taxonomy that compounds with each conversation that introduces invented variants. The combined load-bearing mitigations (operator's vocabulary stability choice + M2 sub-agent template + M4 vocabulary-drift finding class) form a coherent resolution path.
