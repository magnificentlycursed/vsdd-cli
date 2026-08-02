---
schema_class: snapshot-schema
schema_version: 0.1.0
status: draft-proposal
snapshot_fields:
  - {field: acquisition_outcome, contents: "acquired, absent, or unusable — the corroboration condition the degraded-kind derivation branches on"}
  - {field: milestones, contents: "per milestone: exact name, state, is-active"}
  - {field: findings, contents: "per finding under the active scope: handle, status, owner domain or absent, validator domain or absent, evidence-reference presence for closed records, disposition where closed, filed-routing presence, and whether the record closed before the routing amendment's ratification boundary (the unrouted-findings query's forward-only universe datum)"}
  - {field: round_manifests, contents: "per round issue: handle, the manifest's declared finding count"}
  - {field: round_children, contents: "per round issue: the tracked child count"}
  - {field: comment_handles, contents: "per handle cited in result comments: the handle text and whether it resolves"}
  - {field: display_repo_name, contents: "the worded repo name"}
  - {field: display_session, contents: "session identifier or worded absence"}
  - {field: display_work_item, contents: "work item handle and title or worded absence"}
  - {field: display_active_milestone, contents: "active milestone name with its open-finding count precomputed, or worded absence"}
  - {field: finding_fields_acquired, contents: "which finding-field groups this acquisition populated — spine (status, disposition, filed-routing presence, closed-before-ratification), lifecycle-roles (owner, validator), evidence (evidence-reference presence); the finding-reading checks gate on it so a spine-only live acquisition does not mis-fire the checks that read the groups it did not acquire (vsdd-cli #820); a finding query that FAILED with the tracker present records NO group acquired — never bit-identical to a clean tracker's spine claim — and the routing gate fails closed on the unacquired spine (vsdd-cli #818)"}
  - {field: finding_acquisition_note, contents: "a worded note when the finding query was capped this acquisition (findings past the cap were not examined) or failed with the tracker present (findings could not be acquired, the failed step named), else absent — the no-silent-caps, no-silent-failure marker (vsdd-cli #820, #818)"}
audit:
  snapshot_scoped_checks:
    - {check: round-parity, consumes: "round_manifests + round_children", materialized: true}
    - {check: unresolvable-handles-in-result-comments, consumes: comment_handles, materialized: true}
    - {check: findings-missing-owner-or-validator, consumes: findings, materialized: true}
    - {check: closed-findings-missing-evidence, consumes: findings, materialized: true}
    - {check: unrouted-findings, consumes: findings, materialized: true}
    - {check: phase-pointer-against-milestone-state, consumes: "milestones + the state artifact (the derivation's first input)", materialized: true}
    - {check: degraded-kind-derivation, consumes: acquisition_outcome, materialized: true}
  shell_side_checks:
    - {check: off-grammar-branch-names, home: "the effectful shell's refs query over this clone's own branches; joins the report there — git references cannot materialize into the snapshot"}
    - {check: installed-artifact-integrity-check, home: "the effectful shell over the installed-artifact manifest and the filesystem, including the project-root-equals-repo-root member; joins the report there"}
    - {check: unsigned-event-count, home: "the crosslink's own compaction detection, consumed by the shell; joins the report there"}
  renderer_display_fields:
    - {field: display_repo_name, materialized: true}
    - {field: display_session, materialized: true}
    - {field: display_work_item, materialized: true}
    - {field: display_active_milestone, materialized: true}
---

# Snapshot schema — audited

The corroboration snapshot's field enumeration (contract: Verification
architecture — the snapshot is an explicitly-acquired materialized view;
the phase-answer derivation is pure over it and never acquires). The
`audit` block is the contracted phase-1c audit: every snapshot-scoped
check's inputs are materialized fields rather than illustrative ones —
per-finding owner, validator, and evidence-reference presence included —
and the three checks that cannot materialize run in the effectful shell
and join the report there, each with its home named.

The three finding-reading checks (findings-missing-owner-or-validator,
closed-findings-missing-evidence, unrouted-findings) additionally gate on
`finding_fields_acquired`: a live acquisition that populates only some
finding-field groups — the Slice-1 spine-only join — runs only the checks
whose input groups it acquired, so partial acquisition never mis-fires the
checks reading the groups it deferred (vsdd-cli #820). A gated-off check
never lets its silence read as checked-clean: the status report names it
dormant (a deferred group) or could-not-check (the failed finding query),
and the routing gate fails closed when the spine group itself was not
acquired — the tracker was present but routing could not be read, so an
unverifiable gate never passes vacuously (vsdd-cli #818 Fix 2; the
fail-closed requirement of the routing-before-fix guardrail).

The statusline renderer's display fields ride the same snapshot: one
acquisition, one derivation, one rendering, never a second computation.
The open-finding count is precomputed into display_active_milestone and
scoped to the active milestone — a bounded gauge, not an estate alarm.

Proposals until operator adoption is recorded (vsdd-cli #672). Authored
under phase-1c data authoring (vsdd-cli #598) — the package's closing
item. Draft vocabulary under the maturity lifecycle until first publish.

Member adoptions recorded on the set issue do not advance this
artifact's status: the status field advances by the phase-exit
adoption act, then first publish (vsdd-cli #715, executing the #697
item-5 standing disposition at the cold pass's finding).
