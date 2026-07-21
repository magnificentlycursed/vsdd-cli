---
schema_class: gate-data
schema_version: 0.3.0
status: draft-proposal
failure_kinds:
  - {kind: assertion-failure, red_validity: valid-red, scope: per-test, meaning: "an executed test's assertion failed"}
  - {kind: panic, red_validity: valid-red, scope: per-test, meaning: "an executed test terminated in a panic or unhandled error"}
  - {kind: error-exit, red_validity: valid-red, scope: per-test, meaning: "an executed test returned an error result"}
  - {kind: timeout, red_validity: valid-red, scope: per-test, meaning: "an executed test exceeded its per-test time limit"}
  - {kind: abort, red_validity: valid-red, scope: per-test, meaning: "an executed test's process aborted (signal, out-of-memory kill, hard abort)"}
  - {kind: doctest-compile-failure, red_validity: valid-red, scope: per-test, meaning: "a doctest failed to compile; the runner reports it as that doctest failing — the per-test exception to compile failure's whole-suite scope (operator ruling 2026-07-21, vsdd-cli #676)"}
  - {kind: compile-failure, red_validity: conditional, scope: whole-suite, meaning: "the suite failed to build; red only under a validator-approved compile-defect declaration, otherwise the run routes to the cannot-run family"}
  - {kind: build-script-failure, red_validity: neither, scope: whole-suite, meaning: "a build script failed before compilation; the run routes to the cannot-run family (operator ruling 2026-07-21, vsdd-cli #676)"}
  - {kind: not-collected, red_validity: neither, scope: per-test, meaning: "the named test is absent from the runner report; satisfies neither the red nor the green half"}
  - {kind: skipped, red_validity: neither, scope: per-test, meaning: "the test was collected and reported skipped at runtime; neither half"}
  - {kind: ignored, red_validity: neither, scope: per-test, meaning: "the test carries an ignore attribute; reported but not executed; neither half"}
  - {kind: filtered, red_validity: neither, scope: per-test, meaning: "the test was excluded by runner filtering; neither half"}
pin_kind_declaration:
  declared_shape: "the finding issue declares expected_kinds — a non-empty set drawn from the valid-red kinds, or compile-failure under its approved compile-defect declaration"
  red_rule: "valid red: the named test fails in every run with each run's observed kind a member of the declared set; an observed kind outside the set is wrong-reason, decided by set membership with no approval lane (operator ruling 2026-07-21, vsdd-cli #675)"
  whole_suite_rule: "a declared set containing compile-failure validates its red at the whole-suite grain: every run's build fails under the approved compile-defect declaration — no named test exists, so the per-test rules do not apply and the aggregation unit is the run (vsdd-cli #700)"
  delta_rule: "the standing-suite delta fails closed: a baseline-passing test that aggregates flaky at HEAD fails the delta (same ruling)"
flake_policy:
  runs_per_gate_execution: 3
  per_test_aggregation: "red only if the named test fails in all runs with every observed kind a member of the finding's declared expected set; green only if it passes in all runs; a consistent neither-kind outcome across all runs aggregates to that recorded neither state, not flaky (vsdd-cli #701); any genuinely mixed outcome is the flaky recorded state, which satisfies neither half and is reported with its per-run kinds"
  scope: "covers the pin runs and the standing-suite delta alike; the suite delta compares per-test aggregated outcomes at baseline and HEAD, and flaky-at-HEAD fails the delta for a baseline-passing test (fail-closed, operator ruling 2026-07-21, vsdd-cli #675)"
cannot_run_predicate:
  command_binding: "the repo's declared test command at workspace scope — the same command the chassis's gate resolves (crosslink hook-config agent_test_commands at this estate; the adopter's declared equivalent elsewhere)"
  report_binding: "the verdict binds to the declared runner's report contract — parseable-per-contract decides present versus unparseable; the runner's process model changes nothing, the predicate reads only the report and the exit status (vsdd-cli #677)"
  cannot_run_when: "the command produces no runner report at all; or exits nonzero with zero tests collected in the report; or produces a report the declared runner's contract cannot parse — present-but-unparseable fails closed to cannot-run (a report truncated mid-write is neither absence nor population)"
  is_not_cannot_run: "a suite that runs and fails tests, however many; a slow suite that completes; a nonzero exit with a populated, parseable report"
  timeout_seconds: 600
  timeout_semantics: "a run exceeding the timeout is cannot-run whether it produced no report or a partial one — a partial report from a run that never finished is not a population; the timeout firing is recorded on the gate record"
mapping_schema:
  report_set_default: confirmed
  entry_fields: "{from: <non-empty set of test paths present in the baseline report and absent from the HEAD report>, to: <non-empty set of test paths present in the HEAD report and absent from the baseline report>, reason: <text>, approval: <validator approval record handle>}"
  rules:
    - "mappings are set-valued: splits (one source, several targets) and merges (several sources, one target) are lawful mappings; a path appears in at most one mapping's from set and at most one mapping's to set (operator ruling 2026-07-21, vsdd-cli #678)"
    - "every to-set member of a split inherits the dormancy scrutiny — no split target escapes it (same ruling)"
    - "targets are drawn from the HEAD runner report (the adopted report-set default, confirmed): a target the runner cannot report is not a mapping target, and that relocation degrades to a source-path disablement with the relocation linkage a validator-trust item"
    - "the approval record enumerates every from-to pair; the gate checks the approved pair set equals the declared pair set"
    - "a mapped target arriving reported-but-non-executing carries the composed disablement declaration on the same record"
    - "mappings are declared in the commit's structured evidence section"
---

# Gate data set

The fix-scale and layer-scale gate machinery's data (contract: Phase exit
by gate; part of the phase-1c data-authoring package, vsdd-cli #598).
Every number and the
report-set confirmation are operator-owned proposals until adoption is
recorded (vsdd-cli #668). Round-1 rulings folded in 2026-07-21:
vsdd-cli #675, #676, #677, #678.

`failure_kinds`: red and green are recorded with test identity and failure
kind, making wrong-reason a mechanical comparison. The `red_validity`
column carries the pin doctrine: six per-test kinds are valid reds
(doctest compile failure the compile-time exception among five run-time
failures — the runner reports it against the named doctest);
compile-failure is conditional on the
validator-approved compile-defect declaration; the five remaining kinds
are the neither-red-nor-green recorded states. The `scope` column
separates per-test kinds from whole-suite ones — a whole-suite kind
routes to the cannot-run family rather than impersonating a per-test
failure.

`pin_kind_declaration`: a defect whose kind varies by platform or run
order pins as a declared set, and wrong-reason stays a set-membership
check — mechanical under kind instability, no approval lane added.

`flake_policy`: red must be reliable to demonstrate the defect, so a red
that wavers is flaky, not red — and a green that wavers is flaky, not
green. Kind-set agreement across runs keeps wrong-reason detection honest
under repetition, and a baseline-passing test arriving flaky at HEAD
fails the suite delta. The run counts are operator-adopted seeds; the
quality-engineer lens's round-1 assessment stands recorded — aggregation
shape principled, the counts themselves unjustified by evidence, with
detection power inverted relative to stakes against the timing check's
seven runs — and count revision routes through the advisory loop once
ledger actuals accumulate (vsdd-cli #697).

`cannot_run_predicate`: the mechanical discriminator backing the
broken-verification-surface form — the gate's own run attempt rejects a
cannot-run claim when the suite in fact runs. Validator approval remains
the primary discriminator; this predicate is the additional mechanical
check the contract keeps on the two suite-cannot-run forms. The three
arms of `cannot_run_when` close the truncated-report gap: absent,
empty, and present-but-unparseable all fail toward cannot-run, and the
report contract named in `report_binding` decides parseability.

`mapping_schema`: relocation-as-fix has a lawful lane; the report-set
default is confirmed (targets are the report set), retaining the
cfg-vanished degradation and the mapping fixtures adopted with the
report-set confirmation (vsdd-cli #668).

Authored under phase-1c data authoring (vsdd-cli #598, set issue #668).
Draft vocabulary under the maturity lifecycle until first publish.

Member adoptions recorded on the set issue do not advance this
artifact's status: the status field advances by the phase-exit
adoption act, then first publish (vsdd-cli #715, executing the #697
item-5 standing disposition at the cold pass's finding).
