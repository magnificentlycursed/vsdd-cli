---
schema_class: gate-data
schema_version: 0.1.0
status: draft-proposal
failure_kinds:
  - {kind: assertion-failure, red_validity: valid-red, meaning: "an executed test's assertion failed"}
  - {kind: panic, red_validity: valid-red, meaning: "an executed test terminated in a panic or unhandled error"}
  - {kind: error-exit, red_validity: valid-red, meaning: "an executed test returned an error result"}
  - {kind: timeout, red_validity: valid-red, meaning: "an executed test exceeded its per-test time limit"}
  - {kind: abort, red_validity: valid-red, meaning: "an executed test's process aborted (signal, OOM kill, hard abort)"}
  - {kind: compile-failure, red_validity: conditional, meaning: "the test failed to build; red only under a validator-approved compile-defect declaration, otherwise no demonstration"}
  - {kind: not-collected, red_validity: neither, meaning: "the named test is absent from the runner report; satisfies neither the red nor the green half"}
  - {kind: skipped, red_validity: neither, meaning: "the test was collected and reported skipped at runtime; neither half"}
  - {kind: ignored, red_validity: neither, meaning: "the test carries an ignore attribute; reported but not executed; neither half"}
  - {kind: filtered, red_validity: neither, meaning: "the test was excluded by runner filtering; neither half"}
flake_policy:
  runs_per_gate_execution: 3
  per_test_aggregation: "red only if the named test fails in all runs with the same declared failure kind; green only if it passes in all runs; any mixed outcome is the flaky recorded state, which satisfies neither half and is reported with its per-run kinds"
  scope: "covers the pin runs and the standing-suite delta alike; the suite delta compares per-test aggregated outcomes at baseline and HEAD"
cannot_run_predicate:
  command_binding: "the repo's declared test command at workspace scope — the same command the chassis's gate resolves (crosslink hook-config agent_test_commands at this estate; the adopter's declared equivalent elsewhere)"
  cannot_run_when: "the command produces no runner report at all, or exits nonzero with zero tests collected in the report — the suite failed before executing any test"
  is_not_cannot_run: "a suite that runs and fails tests, however many; a slow suite that completes; a nonzero exit with a populated report"
  timeout_seconds: 600
  timeout_semantics: "a run exceeding the timeout with no report produced is cannot-run; the timeout is recorded on the gate record when it fires"
mapping_schema:
  report_set_default: confirmed
  entry_fields: "{from: <test path present in the baseline report and absent from the HEAD report>, to: <test path present in the HEAD report and absent from the baseline report>, reason: <text>, approval: <validator approval record handle>}"
  rules:
    - "mappings are one-to-one; a path appears in at most one mapping"
    - "targets are drawn from the HEAD runner report (the adopted report-set default, confirmed): a target the runner cannot report is not a mapping target, and that relocation degrades to a source-path disablement with the relocation linkage a validator-trust item"
    - "the approval record enumerates every pair; the gate checks the approved set equals the declared set"
    - "a mapped target arriving reported-but-non-executing carries the composed disablement declaration on the same record"
    - "mappings are declared in the commit's structured evidence section"
---

# Gate data set

The fix-scale and layer-scale gate machinery's data (contract: Phase exit
by gate; items 4-7 of the phase-1c data authoring). Every number and the
report-set confirmation are operator-owned proposals until adoption is
recorded (vsdd-cli #668).

`failure_kinds`: red and green are recorded with test identity and failure
kind, making wrong-reason a mechanical comparison. The `red_validity`
column carries the pin doctrine: five executed-failure kinds are valid
reds; compile-failure is conditional on the validator-approved
compile-defect declaration; the four non-execution kinds are the
neither-red-nor-green recorded states — a skipped, ignored, filtered, or
not-collected pin satisfies neither half.

`flake_policy`: red must be reliable to demonstrate the defect, so a red
that wavers is flaky, not red — and a green that wavers is flaky, not
green. Same-kind agreement across runs keeps wrong-reason detection
honest under repetition.

`cannot_run_predicate`: the mechanical discriminator backing the
broken-verification-surface form — the gate's own run attempt rejects a
cannot-run claim when the suite in fact runs. Validator approval remains
the primary discriminator; this predicate is the additional mechanical
check the contract keeps on the two suite-cannot-run forms.

`mapping_schema`: relocation-as-fix has a lawful lane; the report-set
default is CONFIRMED (targets are the report set), retaining the
cfg-vanished degradation and the fixture list as authored.

Authored under phase-1c data authoring (vsdd-cli #598, set issue #668).
Draft vocabulary under the maturity lifecycle until first publish.
