---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-06-02
phase: phase-3
scope: >-
  Milestone bf99abe (patterns: cross-reference integrity). Subject -- the six
  rules of vsdd-core/patterns/cross-references.yaml (VSDD-E0201..E0206), the DSL
  surface they exercise (key()/every/defined/{{$self.field}} interpolation), the
  indices design (markdown glob -> frontmatter -> indexed_by), and the
  sentinel-expansion side effect in .claude/commands/vsdd-phase-3.md
  (relevant_domains: [all-active-domains] -> literal 18-slug enumeration).
  E0207/E0208 (added in 4e39baf, post-milestone) out of scope.
lens: >-
  Primary QE (test falsifiability dim 1; edge enumeration dim 3; mutation
  readiness dim 6). Supporting SA (DSL surface coherence; index design).
  TW (rule-ID + message-text quality). Sanity-Check (sentinel-expansion
  design call).
source: director-raised
session_note: >-
  Cold-session reviewer mode per Phase 3 primer; composed inline by operator
  directive (4 domains in one agent: QE primary + SA + TW + sanity-check
  supporting). Adversarial-pair invariant respected. Falsifiability discipline
  per QE dim 1: for each rule asked 'what fixture would make it fire that
  doesn't fire now?' before recording.
model: claude-opus-4-7
execution_method: >-
  inline single-session multi-domain cold reviewer; primer + 4 domain prompts +
  review-entry schema loaded sequentially; no prior-cycle memory.
sycophancy_compensation: >-
  Claude authored both the rules (bf99abe) and the sentinel-expansion edit in
  the same commit. The bias: read the literal 18-slug list as 'right because
  mdatron verified clean' -- green-run-as-sufficient is QE failure mode 2.
  Compensation: F1 grounds in the primer's own prose (line 34, 'axes-activated')
  which the literal list contradicts; F4 grounds in symmetry vs phase-1a..2c.
---

# Quality Engineer Review 1 -- 2026-06-02

**Phase 3 cycle round:** 1 (opening cold-session review of milestone bf99abe).

## Pre-phase composition declaration

```yaml
phase: phase-3
composed_domains: [quality-engineer, solution-architect, technical-writer, sanity-check]
composition_mode: inline-single-agent-multi-domain-cold-session
memory_isolation: cold-session-no-prior-context
operator_confirmation: confirmed (director-raised)
cluster_shape: deviation-from-4-cluster-default (4-domain inline; adversarial pairs separated)
declared_at: 2026-06-02T00:00Z
```

## Findings

### F1 -- Sentinel expansion destroyed semantic information; literal 18-slug list contradicts the primer's own prose (sanity-check + TW) -- Accepted-with-remediation

**Evidence:** Pre-edit `[all-active-domains]` (1 sentinel) -> post-edit 18 literal slugs == every domain that exists (`ls vsdd-domain-*.md | wc -l == 18`). Primer prose at `vsdd-phase-3.md:34` says: "the active domain set (always-on baseline + per-feature axes-activated) spawns into 4 clusters." The intended semantics of `[all-active-domains]` was the *dynamic* set determined at swarm-invocation time by axes activation. The literal list says "every domain, statically, every time." Comparison: `vsdd-phase-1a.md:8` uses 5 axes-conditional slugs; phase-3's literal-18 breaks that pattern.

**The fuzz signal:** Add a 19th domain prompt -> E0204 stays green (all 18 still resolve) AND the 19th is silently absent from "all-active". Commit message admits this: "When the domain roster changes, this list must move with it -- a tradeoff accepted."

**Why "expand the sentinel" was wrong:** Three options existed. (a) Expand the sentinel (chosen). (b) Whitelist the literal in E0204 -- bypasses integrity for the one primer that needs it most. (c) Add `relevant_domains_strategy: enumerated | all-active | axes-driven` to phase-primer schema; make E0204 conditional on `strategy == enumerated`. Path (c) preserves the sentinel's meaning, keeps the rule property-shaped, surfaces the architectural choice in the schema. The chosen path optimized for "verify clean today" at the cost of "future-maintainer rediscovers semantics from scratch."

**Routing:** Phase 4 -> Raise to SO (schema amendment) + SA. Composes with F5.

### F2 -- E0201's `"sanity-check"` literal disjunct is dead code (QE + SA) -- Resolved-pending

**Evidence:** `assert: $self.validator_pair == "sanity-check" or defined(key("domains", $self.validator_pair))`. But `.claude/commands/vsdd-domain-sanity-check.md` exists with `domain_slug: sanity-check` and is indexed by the `domains` key. Therefore `key("domains", "sanity-check")` IS defined; the left disjunct never matters.

**Why it matters:** *Falsifiability* -- no fixture distinguishes rule-with-disjunct from rule-without against any plausible corpus. *Maintainability* -- the literal hardcodes "sanity-check is the universal validator-pair-terminator" in rule logic; if a second terminator is added (e.g. `vsdd-methodology`), hand-edit needed. *Author-intent reading:* the disjunct likely predates sanity-check becoming a registered domain; it's a fossil the milestone added but didn't notice was obsolete.

**Routing:** Phase 4 -> Phase 1b (drop disjunct; rule becomes `defined(key("domains", $self.validator_pair))`). Add Phase 2a fixture: domain with `validator_pair: ghost-domain` -- expect E0201.

### F3 -- All six rules are property-shaped; no falsifying fixtures in tree (QE) -- Resolved-pending

**Evidence:** E0201..E0206 are all `every(...)` / `defined(key(...))` shapes -- universal quantification only. Commit message: "mdatron verify clean across all 48 artifacts + 6 new rules." Green-run-on-real-corpus is QE failure mode 1: the corpus was constructed *to* pass. No `tests/fixtures/` or `mdatron-tests/` directory contains a deliberately-broken artifact the rules must reject. Falsifiability is asserted, not demonstrated.

**Per-rule falsifying-fixture catalog (none exist):**

| Rule | Falsifying fixture |
|---|---|
| E0201 | domain prompt with `validator_pair: nonexistent-domain` |
| E0202 | domain prompt with `supplements_applied: [made-up]` |
| E0203 | supplement with `domains_in_scope: [made-up]` |
| E0204 | phase primer with `relevant_domains: [ghost]` |
| E0205 | phase primer with `supplements_in_scope: [ghost]` |
| E0206 | review-entry with `phase: phase-99` (also caught by enum -- see F4) |

**Mutation posture (QE dim 6):** which mutants of `defined(key(...))` -> `not defined(...)` survive? *All*, against the current corpus, until a real artifact violates by accident.

**Routing:** Phase 4 -> Phase 2a (red-gate fixture authoring for cross-references.yaml).

### F4 -- E0206 partly redundant with review-entry.json `phase` enum; dispatch model otherwise coherent (SA) -- Resolved-pending

**Evidence:** `review-entry.json` declares `phase` as enum `[phase-0..phase-6]` (11 values). The phase-primer corpus contains primers for 1a, 1b, 1c, 2a, 2b, 2c, 3, 4, 5, 6 -- no `phase-0` primer file. A review-entry with `phase: phase-0` passes schema but fails E0206. E0206's residual job IS real ("enum is broader than primer corpus") but the *message* doesn't say so; operator wonders if the schema enum is wrong.

**Composition observation (positive):** dispatch by `schema_class` (per ddbb8d0) is the right call. The rule-level `context: domain-prompt | supplement | phase-primer | review-entry` maps 1:1 to `schema_class`. Clean model, no implicit dispatch.

**Routing:** Phase 4 -> Phase 1a (E0206 message: name the file-vs-enum gap explicitly).

### F5 -- The literal 18-slug list drifts under roster change; no compensating rule (SA + vsdd-methodology) -- Accepted-with-remediation

**Evidence:** Today: 18 domain prompts; 18 slugs listed. No rule asserts the equality. Adding a 19th domain prompt produces passing E0204 + silently absent 19th-domain semantics. Commit acknowledges but doesn't enforce.

**Compensating rule (suggestion):** `context: phase-primer`, asserting `every(d in <every-domain-slug>, d in $self.relevant_domains)` -- requires iterating *over the `key()` index* as `every`'s source. If mdatron's `every` supports that, the rule is writable today; if not, this milestone surfaces a *missing DSL feature*.

**Hardness-to-undo:** today the schema fix (F1 path c) is one field + one rule branch. With multiple primers using the literal-list pattern, migration grows non-trivial.

**Routing:** Phase 4 -> Raise to SO (decide: schema strategy field, OR compensating rule, OR explicit accepted drift in prose).

### F6 -- Message-text quality + rule-IDs (TW) -- Resolved-pending

**Evidence:** Rule-IDs (`domain-validator-pair-resolves`, etc.) are positive-form descriptions of what the rule asserts. Load-bearing not noise: the catalog (DESIGN-VERIFICATION convention) references rules by both code AND id. `{{$self.field}}` interpolation correctly surfaces offending artifact + value (E0201 names `validator_pair '{{$self.validator_pair}}'`).

**Weakness:** E0202/E0203/E0204/E0205 say "one or more slugs ... do not resolve." Operator gets the list-owner but not the offending slug. Cold operator must re-read the array and visually diff against the index -- exactly the cognitive work the message should do.

**Routing:** Phase 4 -> Phase 1b (extend interpolation: a `{{$first_unresolved}}` template variable; needs DSL surface decision via SA).

## Round-close summary

**6 findings; 0 Hallucinated; 0 Dismissed. Round MUST continue.**

| F | Domain | Classification | Routing |
|---|---|---|---|
| F1 | sanity-check + TW | Accepted (remediation) | Phase 4 -> SO + SA |
| F2 | QE + SA | Resolved-pending | Phase 4 -> Phase 1b |
| F3 | QE | Resolved-pending | Phase 4 -> Phase 2a |
| F4 | SA | Resolved-pending | Phase 4 -> Phase 1a |
| F5 | SA + vsdd-methodology | Accepted (remediation) | Phase 4 -> SO |
| F6 | TW | Resolved-pending | Phase 4 -> Phase 1b |

**Meta-pattern (F1 + F5):** the sentinel-expansion surfaced an architectural absence (`relevant_domains_strategy`), not an implementation defect. The operator-author absorbed DSL pressure by literalizing data instead of formalizing strategy. QE failure mode 2 in design space: coverage rose without the underlying property strengthening. Corrective: when a sentinel forces rule expansion, ask whether it carried load-bearing semantics; if yes, schema-formalize before data-expand.

**Sycophancy compensation:** F2 + F3 are findings I would have missed by accepting "verify clean" as proof-of-correctness. The QE-dim-1 question produced both.

## Cross-references

- `vsdd-core/patterns/cross-references.yaml` (subject; E0201..E0206 in scope; E0207..E0208 added in 4e39baf out of scope)
- `.mdatron/patterns/cross-references.yaml` (mirror; F1..F6 apply equally)
- `.claude/commands/vsdd-phase-3.md` (sentinel-expansion site; F1 + F5)
- `vsdd-core/schemas/phase-primer.json` (F1 schema-amendment surface)
- `vsdd-core/schemas/review-entry.json` (F4 enum-vs-corpus overlap)
- Commit bf99abe (milestone under review)
