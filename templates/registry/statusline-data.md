---
schema_class: statusline-data
schema_version: 0.1.0
status: draft-proposal
read_failure_kinds:
  - kind: absent
    machine_token: state-absent
    recovery_action: restore-state-file
    human_diagnostic: "state file absent: .vsdd/state.yaml not found"
    human_recovery: "restore .vsdd/state.yaml from the last boundary commit, then re-run vsdd status"
  - kind: malformed
    machine_token: state-malformed
    recovery_action: fix-state-content
    human_diagnostic: "state file malformed: parse failure at the reported location"
    human_recovery: "repair the malformed state file to match the last boundary commit, then re-run vsdd status"
  - kind: permission-or-io
    machine_token: state-unreadable-io
    recovery_action: fix-state-permissions
    human_diagnostic: "state file unreadable: permission or IO failure"
    human_recovery: "fix file permissions on the state artifact, then re-run vsdd status"
degraded_kinds:
  - kind: tracker-absent
    marker_word: offline
    benign: true
    next_step_text: "offline mode — tracker corroboration unavailable; the phase answer is computed from the state artifact alone, which is the contracted normal offline behavior; corroboration resumes when the tracker is reachable"
  - kind: tracker-unusable
    marker_word: degraded
    benign: false
    next_step_text: "tracker data unusable — the phase answer is computed from the state artifact alone; run crosslink integrity to diagnose the tracker store"
wiring_outcomes:
  - {id: installed, meaning: the default wiring was written on confirmation or accept flag}
  - {id: refused-existing-entry, meaning: "a statusLine entry already exists; init refused and printed the composition instruction"}
  - {id: declined, meaning: "the operator declined the interactive offer; settings untouched"}
  - {id: not-offered-non-interactive, meaning: "non-interactive run without the accept flag; no prompt, no write, offer printed"}
  - {id: withdrawn-read-failure, meaning: "the settings probe failed; offer withdrawn, diagnostic printed, installation proceeded"}
  - {id: failed-write, meaning: "the confirmed or accepted write failed; adopter file untouched, diagnostic printed"}
display_fields:
  - {field: repo-name, width_budget_chars: 16, absence_text: ""}
  - {field: phase-answer, width_budget_chars: 20, absence_text: "not entered"}
  - {field: session, width_budget_chars: 10, absence_text: "no session"}
  - {field: work-item, width_budget_chars: 24, absence_text: "no work item"}
  - {field: milestone-with-count, width_budget_chars: 24, absence_text: "no milestone"}
truncation_mark: "(cut)"
broken_state_mark: "state unreadable — vsdd status"
wall_clock_budget_ms: 250
timing_check:
  runs: 7
  rule: median-at-or-under-budget
  ceiling_rule: max-at-or-under-twice-budget
  scope: whole invocation, acquisition through render
repo_set_config:
  location: "~/.config/vsdd/statusline.yaml"
  ownership: adopter-owned
  fields: "{repos: [<absolute repo paths>], per_repo_budget_ms: <number>}"
  aggregate_cost_rule: "total budget = repo count x per_repo_budget_ms; the count is explicit in the same file (the repos list length), never discovered"
composition_instruction_conduct: "the printed instruction carries the exact invocation line and covers both existing-statusLine shapes: an editable wrapper script (append the line; segments compose by concatenation) and an opaque command (create a two-line wrapper calling the existing command, then the invocation line, and point statusLine at the wrapper)"
---

# Statusline data set

The words, numbers, and enumerations the three status renderings consume
(contract: the Status requirement; Verification architecture — the budgets
are versioned data beside the enumerations). Every number and copy string
here is a proposal until operator adoption is recorded (vsdd-cli #667).

`read_failure_kinds` maps one-to-one into the action vocabulary's recovery
family (composition-scope-and-actions, round 2) — the machine form emits
the kind's machine token, the recovery action id, and the diagnostic
payload; the human form renders the diagnostic and recovery text; the
segment renders the broken-state mark. The never-silent principle covers
absence, degradation, truncation, and unreadable state alike.

`degraded_kinds`: the tracker-absent kind is the contracted normal offline
mode and its text says so rather than raising a false alarm; markers are
plain words, never glyphs alone, and the information survives color
stripping.

`display_fields` absences render as words, never empty slots; the
truncation mark is a word, not a glyph, per the same discipline. The
wiring-outcome members are the Install requirement's fixed six —
success affirmative, deliberate skips and failures distinct.

`repo_set_config` resolves the routed items of "phase-1c work item:
statusline wiring script shape" (#362): explicit adopter-owned repo set
(never discovered, so the display cannot grow unbounded), aggregate cost
denominated as repo count times the per-repo wall-clock budget, and the
composition instruction covering wrapper and opaque-command shapes.

Authored under phase-1c data authoring (vsdd-cli #598, set issue #667).
Draft vocabulary under the maturity lifecycle until first publish.
