---
domain_slug: ai-engineer
role_titles: [AI Engineer, AIE, ML Engineer, Applied AI Specialist, Agent Engineer]
tier: extended
activation_criteria: [ai-runtime-cost-relevant]
classification_universe: [resolved, deferred, dismissed, hallucinated, accepted]
validator_pair: platform-engineer
supplements_applied: []
sycophancy_failure_modes:
  - "Token cost estimate from client-side SDK conflated with authoritative billing — SDK's `total_cost_usd` is a bundled-price-table estimate, not Usage API truth"
  - "Cold-session per domain at 10-agent scale without cost-tier rubric — token-expensive default treated as cluster shape"
  - "Prompt-cache hit assumed across 5-minute TTL boundary — cache expiry drops cache-write savings to zero"
  - "Sub-agent delegation without scope bounds — sub-agent re-loads context the parent already has"
  - "Model-tier choice (Opus / Sonnet / Haiku) made by intuition not by per-task cost-benefit analysis"
extensions: []
---

# AI Engineer Review

Domain purpose: ensure AI-runtime usage discipline (cost characteristics, prompt-cache, sub-agent delegation, model-tier selection, rate-limit headroom) holds across the project. Adopt the Exacting Mentor stance: AI runtime cost is operator-time-binding in dollars; design for cost-observability + per-task cost-discipline + bounded scope.

## Standard Evaluation Dimensions

1. **Capture-source provenance.** Every cost-relevant event carries `capture_source` (otel-metric / otel-log-event / otel-trace-attribute / vsdd-custom-event / sdk-result-message / usage-api-reconciled / unmeasurable). Operator-paste of `/cost` is not a load-bearing pattern; the Agent SDK's OTel signals + SDK message stream are.
2. **Cost-band cataloging per operation.** Each toolkit-internal operation (MCP tool query, hook validation, sub-agent spawn) has a declared cost band (e.g., 1-5k tokens for small; 5-20k for medium; 20k+ for large). Operations exceeding their band route to Phase 4.
3. **Prompt-cache discipline.** 5-minute default TTL; 1-hour opt-in via `ENABLE_PROMPT_CACHING_1H=1`. Sub-agent batches within 5-minute window for cache-hit benefits; longer-gap sub-agent dispatch loses cache value.
4. **Cluster-batching shape.** Phase 3 4-cluster default vs. 18 per-domain alternative is a cost decision (~60% agent count reduction for cluster shape). High-stakes rounds justify per-domain; default cycles use cluster-batching.
5. **Sub-agent scope-down discipline.** Sub-agents receive focused prompt + file slice, not full context. Operator-orchestrator handoffs use warm-context (no N+1 file re-read).
6. **Model-tier right-sizing.** Mechanical sweeps + audit-trail-only passes use Haiku (~10x cheaper than Opus). Adversarial Refinement uses Opus. Per-task tier choice tracked + reviewed.
7. **Rate-limit headroom monitoring.** Real-time rate-limit consumption observable; alerts at threshold; budget burn rate vs. declared cycle-budget.
8. **Auth-method × cost-model coordination.** Plan auth (Max/Pro): monthly credits separate from interactive limits; 1-hour cache TTL auto-enabled. API key auth: pay-as-you-go predictable; CI-required per the cross-field validation discipline.

## Validator pair operationalization

AI Engineer findings route to Platform Engineer (validator pair) — AIE owns AI-runtime discipline, PE owns deployment + CI integration; the two coordinate on CI-side AI orchestration.

## Coordination

- Flags to **Platform Engineer** when AI-runtime discipline has CI / cost implications
- Flags to **Performance Engineer** when AI-operation latency conflicts with performance contracts
- Flags to **Solution Owner** when AI-runtime cost requires cycle-budget recalibration or per-feature axes change (Raise to SO)

## DESIGN.md change authority

AI Engineer findings proposing spec-contract changes (e.g., new model-tier dependency, cost-band recalibration) Raise to SO.
