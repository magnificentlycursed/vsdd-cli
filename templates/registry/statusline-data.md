---
schema_class: statusline-data
schema_version: 0.2.0
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
    human_diagnostic: "state file malformed: <the parser's reported location and message>"
    human_recovery: "restore the state file's content to the last boundary commit's version, then re-run vsdd status"
  - kind: permission-or-io
    machine_token: state-unreadable-io
    recovery_action: fix-state-permissions
    human_diagnostic: "state file unreadable: permission or input/output failure"
    human_recovery: "clear the fault the diagnostic names — file permissions on the state artifact, or the disk or mount failure — then re-run vsdd status"
degraded_kinds:
  - kind: tracker-absent
    marker_word: tracker offline
    benign: true
    next_step_text: "tracker offline — corroboration unavailable; the phase answer is computed from the state artifact alone, which is the contracted normal offline behavior; corroboration resumes when the tracker is reachable"
  - kind: tracker-unusable
    marker_word: tracker degraded
    benign: false
    next_step_text: "tracker degraded — tracker data unusable; the phase answer is computed from the state artifact alone; run crosslink integrity to diagnose the tracker store"
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
  - {field: work-item, width_budget_chars: 24, absence_text: "no work item"}
  - {field: milestone-with-count, width_budget_chars: 34, absence_text: "no milestone"}
truncation_mark: "(cut)"
truncation_rule: "the mark is set off from the truncated value by a space, never glued; milestone-with-count truncates the name before the count — the open-finding count always survives truncation (operator ruling 2026-07-21, vsdd-cli #680)"
installed_artifact_findings_visibility: "the glance segment does not carry installed-artifact-integrity findings; the machine and human forms carry them (operator ruling 2026-07-21, vsdd-cli #690)"
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
Round-1 rulings folded in 2026-07-21: vsdd-cli #679, #680, #681, #690.

`read_failure_kinds` maps one-to-one into the action vocabulary's recovery
family (composition-scope-and-actions) — the machine form emits
the kind's machine token, the recovery action id, and the diagnostic
payload; the human form renders the diagnostic and recovery text; the
segment renders the broken-state mark. The never-silent principle covers
absence, degradation, truncation, and unreadable state alike. The
permission-or-io recovery text branches both of the kind's causes; its
action id stays `fix-state-permissions`, the registered vocabulary
member (vsdd-cli #681).

`degraded_kinds`: the tracker-absent kind is the contracted normal offline
mode and its text says so rather than raising a false alarm; the marker
words carry their own scope — `tracker offline`, never a bare word a
glance could read network-wide — and are plain words, never glyphs alone,
so the information survives color stripping.

`display_fields` absences render as words, never empty slots; the
truncation mark is a word, not a glyph, and `truncation_rule` states the
order: the count outlives the name. The session identifier is not a
glance segment — the operator is in the session; it renders in the human
form, one `crosslink session status` away (operator ruling 2026-07-21,
vsdd-cli #679), and its freed width moved to milestone-with-count.
repo-name's empty absence text is deliberate: the renderer runs
repo-rooted and the repo-set config enumerates repos by path, so the
field is structurally present (vsdd-cli #697). The wiring-outcome
members are the Install requirement's fixed six — success affirmative,
deliberate skips and failures distinct. The interactive offer's prompt
copy is implementation text (Layer 4), deliberately outside this set:
outcomes, not prompts, are the versioned enumeration (vsdd-cli #697).

Correspondence to the snapshot schema's display fields (vsdd-cli #695):
repo-name is display_repo_name, work-item is display_work_item,
milestone-with-count is display_active_milestone (count precomputed
there); phase-answer is derived by the pure derivation, never
materialized; display_session serves the human form now that the glance
segment omits the session.

`repo_set_config` resolves the routed items of "phase-1c work item:
statusline wiring script shape" (#362): explicit adopter-owned repo set
(never discovered, so the display cannot grow unbounded), aggregate cost
denominated as repo count times the per-repo wall-clock budget, and the
composition instruction covering wrapper and opaque-command shapes.

Authored under phase-1c data authoring (vsdd-cli #598, set issue #667).
Draft vocabulary under the maturity lifecycle until first publish.

Member adoptions recorded on the set issue do not advance this
artifact's status: the status field advances by the phase-exit
adoption act, then first publish (vsdd-cli #715, executing the #697
item-5 standing disposition at the cold pass's finding).
