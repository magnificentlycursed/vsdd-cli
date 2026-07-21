---
schema_class: economics-data
schema_version: 0.1.0
status: draft-proposal
effort_signals:
  up_signals:
    - {id: rework-ratio, computed_from: "tracker + git history", computation: "findings filed against freshly written text in the following round, over fixes landed — joins finding citations to fix commits' changed lines"}
    - {id: cross-lens-convergence, computed_from: "round manifests + tracker", computation: "distinct lenses converging on one defect within a round — convergence density per finding cluster"}
    - {id: stalled-finding-series, computed_from: tracker, computation: "a finding series holding at stable scope across consecutive rounds without disposition movement"}
  down_signals:
    - {id: repeated-clean-rounds, computed_from: "round manifests + tracker", computation: "consecutive clean rounds from the same lens at unchanged scope"}
    - {id: dismissal-heavy-yield, computed_from: tracker, computation: "a lens's dismissed-plus-hallucinated share of filed findings over recent rounds"}
    - {id: mechanical-residue, computed_from: "git history", computation: "keys on the commit evidence sections' exclusion statements — a lens whose findings resolve into mechanical passes"}
  attribution_grain: "declared per auth mode, never promised beyond it: per-dispatch token usage under API auth; window and operator-time consumption attributable per round at best under subscription auth"
tier_effort_defaults:
  - {stage: draft, tier: session-model, effort: session-dial, note: "the draft stage shares the implementing session's dial, per the contract"}
  - {stage: cold-review-round, tier: session-model, effort: medium, note: "per-lens model and effort are settable at dispatch on the Workflow orchestration surface (operator-adopted 2026-07-21, vsdd-cli #597) and every dispatch sets both explicitly — this row is the declared default the dispatch names, never a silent inheritance; round-1 actuals (observed claude-opus-4-8 at half the session model's per-token weight, vsdd-cli #674) feed the advisory loop's first tier revision"}
  - {stage: fix-pass, tier: session-model, effort: medium, note: ""}
  - {stage: terminal-verify-round, tier: session-model, effort: high, note: "the round that decides the stop signal gets the higher dial"}
mutation_floor:
  kill_ratio_percent: 80
  scope: "changed code, when the review config declares the floor; the thorough preset declares it"
token_budgets:
  - {artifact_class: session-skill, budget_tokens: 5000}
  - {artifact_class: domain-prompt, budget_tokens: 3000}
  - {artifact_class: phase-primer, budget_tokens: 2500}
  - {artifact_class: supplement-section, budget_tokens: 2000}
  - {artifact_class: always-on-core, budget_tokens: 1500}
calibration_bands:
  - {operation: cold-spec-review-round, band: "3 to 50 findings pre-dedup, 5 to 25 minutes wall-clock, 200,000 to 600,000 downstream tokens per round", provenance: "findings and wall-clock seeded from this respec's 40+ recorded rounds (series 9-7-7-30-45-13-3-0 and the statusline/lane series); token dimension seeded from round 1's observed actuals — 46 findings pre-dedup, ~13 minutes, 480,118 downstream tokens across five lenses, capture source pinned to subagent transcript usage records (harness notification figures differ and are not band inputs) — operator ruling 2026-07-21, vsdd-cli #674"}
  - {operation: fix-pass, band: "one round's full disposition, 10 to 40 minutes wall-clock", provenance: "seeded from the amendment cycles' fix passes; seeds pending ledger actuals; no token dimension yet — the recorded fix passes ran attended in-session, where per-operation token capture awaits the Layer 9 adapters"}
  - {operation: cold-code-review-round, band: "2 to 10 findings", provenance: "unseeded — no code-shaped cold rounds recorded yet in this estate; first phase-2a cycle calibrates"}
presets:
  - id: thorough
    active_domains: "the full default domain set (18: the 16 role domains plus the 2 meta domains; count verified against the installed prompts 2026-07-21, vsdd-cli #684)"
    round_budget: 8
    stop_sensitivity: "stop on a fully hallucinated round; rounds past the signal require named new evidence"
    mutation_floor_declared: true
    note: "this project's declared preset — the toolkit governs other projects, so its defects propagate"
  - id: standard
    active_domains: "the six core role domains plus sanity-check (count verified against the installed prompts 2026-07-21, vsdd-cli #684; reference resolved per #705)"
    round_budget: 5
    stop_sensitivity: "stop on a fully hallucinated round"
    mutation_floor_declared: false
    note: ""
  - id: minimal
    active_domains: "software-engineer, quality-engineer, documentation-reviewer as the cold reader, sanity-check as validator of last resort"
    round_budget: 3
    stop_sensitivity: "stop on the first clean round"
    mutation_floor_declared: false
    note: "pair co-activation and validator-differs-from-owner hold at any set size, per the contract"
---

# Economics data set

The cost machinery's data (contract: Cost is knowable; Deterministic
composition's preset clause; part of the phase-1c data-authoring
package, vsdd-cli #598). Every number is an operator-owned proposal
until adoption is recorded (vsdd-cli #671). Round-1 rulings folded in
2026-07-21: vsdd-cli #674, #684.

`effort_signals`: each signal computable from the tracker, the manifests,
and git history — the audit trail the contract already names, no new
capture. Advisories follow decision routing: absorbable ones execute;
tier and effort defaults reach the operator with the math.

`tier_effort_defaults`: initial defaults only — the advisory loop
proposes revisions from ledger evidence once cycles complete. Mechanized
gates run no model and take no dial.

`calibration_bands`: seeds, honestly marked — bands update from actuals,
and a band with no linked actuals after a completed cycle is a reported
condition, so these seeds are the thing that check fires against until
real cycles link.

`presets`: suggested starting points with the integrity rules holding at
any size; the operator customizes freely and the customization is a
declared, git-tracked, schema-validated input. The preset fields here
normalize the contract's prose enumeration into one shape — a stated
normalization, not a divergence (vsdd-cli #697).

Authored under phase-1c data authoring (vsdd-cli #598, set issue #671).
Draft vocabulary under the maturity lifecycle until first publish.
