---
schema_class: dispatch-data
schema_version: 0.1.0
status: draft-proposal
branch_grammar:
  forms:
    - id: session-form
      pattern: "^feature/[a-z0-9][a-z0-9._-]*$"
      meaning: "the vehicle's session-scoped form — crosslink kickoff creates feature/<slug> (create_worktree, crosslink 0.8.0 launch.rs:345, the installed and held version; vsdd-cli #694); valid before and after hub sync, perpetually"
    - id: display-form
      pattern: "^issue/[0-9]+(-[a-z0-9][a-z0-9-]*)?$"
      meaning: "the display-ID form once hub sync has assigned one — issue/<display-id> with an optional plain slug; provisional local IDs never enter branch names"
  rules:
    - "both registered forms are perpetually valid; the standing refs query checks membership in either, decidable from the ref alone"
    - "the refs query runs over this clone's own branches — local refs and their remote-tracking counterparts; other actors' refs are their own repos' concern"
    - "the pre-push hook checks the same membership and warns toward the currently-preferred form; it never blocks a registered form, never forces a retroactive rename"
    - "preferred form: display-form once the repo's hub sync has assigned display IDs, session-form before that"
    - "exempt refs: main, the chassis's own branches (crosslink/hub, crosslink/knowledge, wip-archive and other operator-created archive refs recorded on the tracker)"
preflight_members:
  - id: container-runtime
    observation_binding: "the configured container runtime's daemon answers a status probe (docker info or podman info) within 10 seconds; command absent is fail; unresponsive or ambiguous output is inconclusive"
    note: "recorded 2026-07-20: even with the runtime passing, released crosslink versions cannot complete Rust fix-lane verification in-container (dollspace-gay/crosslink#9, #10); the working postures are attended tmux kickoff, with gate verification run host-side by the attended session — recorded on vsdd-cli #597"
  - id: consent-state
    observation_binding: "the launch mode's consent grants are recorded in Claude Code's config surfaces for the exact target path — project trust for the vehicle's worktree path (path-scoped, so fresh worktrees re-raise it) and the bypass-permissions acceptance where the mode requires it; unreadable config is inconclusive"
  - id: identity-approval
    observation_binding: "the dispatch identity's key fingerprint is present in the chassis trust store before launch (approve-then-dispatch: the pre-approved pool or the per-dispatch approval act) and its key material is provisioned into the launch environment; either half absent is fail"
  - id: model-credentials
    observation_binding: "model credentials resolve in the launch environment by presence probe — never by echoing values; absent is fail; a probe that cannot determine presence is inconclusive"
  - id: project-server-trust-and-load
    observation_binding: "every server the project config declares resolves and is enabled for the launch mode; headless silent non-load is the named degradation this member exists to catch — a mode that cannot prove load is inconclusive, which blocks"
preflight_semantics:
  result_values: [pass, fail, inconclusive]
  rule: "fail-closed — a failed or inconclusive member blocks dispatch naming the member; inconclusive never passes silently"
  bootstrap: "format-carried until the preflight command ships: an autonomous dispatch records its member checks as a hand-performed checklist on the dispatch record"
fencing_rule: "a dispatch classified dead (never-started or stalled, by the chassis's launch record, session records, and heartbeat staleness) is closed under that classification; a first write landing after the classification joins as a flagged late-writer under the closed dispatch, never a silent revival"
manifest_fields:
  - {field: dispatch_id, meaning: "stable identifier for this dispatch"}
  - {field: invoked_by, meaning: "the operator act that invoked the dispatch — invocation provenance recorded, never assumed"}
  - {field: vehicle, meaning: "the dispatch vehicle and its side of the attended/autonomous split, derived from the work"}
  - {field: reviewer_role, meaning: "the tool-restricted agent identity receiving domain prompts"}
  - {field: domains, meaning: "the domain prompts sent, per the computed composition"}
  - {field: inputs, meaning: "what was sent, by content hash per input artifact"}
  - {field: composition_ref, meaning: "the composition in force, with its config_inputs_hash"}
  - {field: model_tier, meaning: "the model tier — the value chosen explicitly at dispatch and the value observed in telemetry after the run, both recorded, never assumed (the round-1 correction on vsdd-cli #673)"}
  - {field: effort_level, meaning: "the effort level, chosen explicitly at dispatch — the Workflow orchestration surface exposes it per agent (operator-adopted 2026-07-21, vsdd-cli #597) — and confirmed post-hoc where telemetry exposes it; never inherited silently"}
  - {field: expected_cost_band, meaning: "the calibration band declared before the spend"}
  - {field: preflight_record, meaning: "the member-by-member preflight results for autonomous dispatches"}
  - {field: returned, meaning: "what came back — result reference or the dispatch-failed classification"}
  - {field: findings_filed, meaning: "tracker handles of every finding filed under this dispatch"}
  - {field: coverage, meaning: "domains that did not run, each with its reason — no matching surface, declined by config, or deferred with a trigger"}
  - {field: signature, meaning: "the crosslink agent identity signing the manifest; identities enter trust before dispatch"}
---

# Dispatch data set

The recorded-dispatch machinery's data (contract: Recorded review
dispatch; Conformance at action time's branch-grammar seam; part of the
phase-1c data-authoring package, vsdd-cli #598). Proposals until
operator adoption is recorded (vsdd-cli #669).

`branch_grammar`: registered data, not archaeology — the session form is
verified against the vehicle's own source. Both forms perpetually valid;
the query decides membership from the ref alone; the pre-push hook warns
and never blocks a registered form, so no push is rejected and no
retroactive rename is ever forced.

`preflight_members`: the five members with how each check learns its
answer. Three-valued and fail-closed: an environment failure is never
collapsed to a pass. Environment provisioning is a precondition whose
acts are the operator's, performed once before dispatch.

`manifest_fields`: the dispatch record as data, so the round-parity and
provenance queries read manifests mechanically. The falsifiers this
feeds: unsigned manifests, postdated approvals, missing tier or effort,
dispatches tracing to no operator act, coverage gaps.

Authored under phase-1c data authoring (vsdd-cli #598, set issue #669).
Draft vocabulary under the maturity lifecycle until first publish.

Member adoptions recorded on the set issue do not advance this
artifact's status: the status field advances by the phase-exit
adoption act, then first publish (vsdd-cli #715, executing the #697
item-5 standing disposition at the cold pass's finding).
