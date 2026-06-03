---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-06-02
phase: phase-3
scope: >-
  Crosslink #12 bundled changes per docs/refactor/phase-1-codes-and-dsl/DESIGN.md
  — reserved-code drift fix (E0001/E0002/E0050/E0070/E0080), DSL Field-access
  symmetry (Err(FieldNotFound) -> Ok(Null) on Object-missing-key), defined()
  empty-string carve-out drop. Hot-path surface: mdatron-core/src/dsl/expr.rs
  expression evaluator + pattern-rule throughput across the vsdd corpus.
lens: Performance Engineer — dim 3 (hot-path identification), dim 4 (allocation discipline), dim 7 (scaling boundaries), with attention to throughput-delta from previously-crashing pattern rules now evaluating to completion.
source: operator-directive
session_note: >-
  Cold-session Phase 3 review. No prior-session memory. Subject is the design
  as-written; mdatron-core implementation lives in a sibling repo (not on disk
  in vsdd-cli). Reasoning operates against the DESIGN.md contract surface +
  the operator-supplied lens framing for the four hot-path questions.
model: claude-opus-4-7
execution_method: >-
  Single-domain cold-session review (PerfE primary). No code modifications.
  Operator-supplied prompt enumerated the four candidate perf surfaces and
  invited dismissal if nothing load-bearing surfaces.
sycophancy_compensation: >-
  PerfE bias is "every change deserves a microbenchmark." Counter-bias for
  v0.1 corpus scale (6 cross-reference rules, sub-second verify) is "perf
  contracts that don't bind are spec gaps, not findings." I am resisting
  the bias by declining to manufacture a finding where the changes are
  net-neutral-to-positive at the workload that actually exists.
supplements_loaded: []
---

# Performance Engineer Review — Phase 1 codes-and-dsl bundle (crosslink #12)

## Headline

Dismissed. No PerfE-load-bearing finding. The bundle's allocation profile
strictly improves; throughput delta is bounded by corpus size (6 rules);
no declared latency budget binds.

## Rationale (per operator-supplied surface)

**1. Field-access change (expr.rs:221-236).** Confirmed at DESIGN
line 99: `ok_or_else(|| EvalError::FieldNotFound { ... })` is replaced by
the implicit `Ok(Value::Null)` branch. `ok_or_else` allocates an
`EvalError::FieldNotFound { field: name.clone(), on: "object" }` —
a `String::clone` on every miss. The replacement allocates nothing on
the miss path. Net allocation reduction on the optional-field-reference
hot path. No finding.

**2. defined() simplification (expr.rs:322-330).** The removed arm
(`Value::Str(s) => !s.is_empty()`) eliminates a discriminant test +
an `is_empty()` call per `defined()` invocation on a string. The
replacement is a single `matches!(v, Value::Null)`. Hot-path-relevant
for pattern rules that call `defined($self.field)` per node, but the
gain is sub-microsecond per call. No declared latency budget binds;
no finding.

**3. is_reserved_mdatron_code().** Called by the spec-amendment lint
+ code-allocation lint at compile-test time. Not on any runtime hot
path. No PerfE concern. No finding.

**4. all_emitted_codes_are_reserved source-scan test.** Test-time
filesystem walk over mdatron-core/src/**. Estimated 10-20ms per run.
Runs once per `cargo test` invocation, not per verify. Not load-bearing
for build cost; not load-bearing for verify throughput. No finding.

**5. Pattern-rule re-execution (the one real throughput delta).**
Operator correctly named the inversion: rules that previously
short-circuited on `FieldNotFound` now evaluate the remaining predicate
graph against `Null`. For the vsdd corpus's 6 cross-reference rules,
this is a bounded multiplier — each rule that used to bail at the
first missing optional field now walks its full AST. Worst case: ~6x
predicate evaluations per pattern with an optional-field reference,
times sub-microsecond per evaluation, times corpus size. Still
sub-millisecond. The DESIGN's framing ("schema-tightening reverts
become possible") makes this an intentional trade — paying nanoseconds
to recover ergonomic optional-field semantics. No latency budget in
DESIGN.md § Behavioral contracts binds; no finding.

## Classification

- finding_id: n/a (no findings)
- classification: dismissed
- dismissal_rationale: >-
    The four candidate surfaces resolve to (a) net allocation reduction
    on Field-miss, (b) sub-microsecond defined() speedup, (c) non-hot-path
    test-only code, (d) bounded throughput delta at corpus scale (6 rules,
    sub-millisecond pattern phase). No declared performance contract in
    DESIGN.md § Behavioral contracts binds; no finding reaches PerfE
    load-bearing.
- routing target: none

## Phase 5 deferral note

If/when mdatron-core grows a corpus-scale workload (10k+ documents or
100+ pattern rules), the Field-symmetry change's "rules-that-used-to-crash-
now-evaluate-fully" delta becomes worth benchmarking. Proptest seeds in
DESIGN § Phase 5 candidates are the natural attachment point; a
criterion bench over `evaluate(Expr::Field(...))` at that scale would
bind the contract. Not v0.1 work.
