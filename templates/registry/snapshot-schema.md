---
schema_class: snapshot-schema
schema_version: 0.1.0
status: draft-proposal
snapshot_fields:
  - {field: acquisition_outcome, contents: "acquired, absent, or unusable — the corroboration condition the degraded-kind derivation branches on"}
  - {field: milestones, contents: "per milestone: exact name, state, is-active"}
  - {field: findings, contents: "per finding under the active scope: handle, status, owner domain or absent, validator domain or absent, evidence-reference presence for closed records, disposition where closed"}
  - {field: round_manifests, contents: "per round issue: handle, the manifest's declared finding count"}
  - {field: round_children, contents: "per round issue: the tracked child count"}
  - {field: comment_handles, contents: "per handle cited in result comments: the handle text and whether it resolves"}
  - {field: display_repo_name, contents: "the worded repo name"}
  - {field: display_session, contents: "session identifier or worded absence"}
  - {field: display_work_item, contents: "work item handle and title or worded absence"}
  - {field: display_active_milestone, contents: "active milestone name with its open-finding count precomputed, or worded absence"}
audit:
  snapshot_scoped_checks:
    - {check: round-parity, consumes: "round_manifests + round_children", materialized: true}
    - {check: unresolvable-handles-in-result-comments, consumes: comment_handles, materialized: true}
    - {check: findings-missing-owner-or-validator, consumes: findings, materialized: true}
    - {check: closed-findings-missing-evidence, consumes: findings, materialized: true}
    - {check: phase-pointer-against-milestone-state, consumes: "milestones + the state artifact (the derivation's first input)", materialized: true}
    - {check: degraded-kind-derivation, consumes: acquisition_outcome, materialized: true}
  shell_side_checks:
    - {check: off-grammar-branch-names, home: "the effectful shell's refs query over this clone's own branches; joins the report there — git references cannot materialize into the snapshot"}
    - {check: session-substrate-check, home: "the effectful shell over the installed-artifact manifest and the filesystem, including the project-root-equals-repo-root member; joins the report there"}
    - {check: unsigned-event-count, home: "the chassis's own compaction detection, consumed by the shell; joins the report there"}
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

The statusline renderer's display fields ride the same snapshot: one
acquisition, one derivation, one rendering, never a second computation.
The open-finding count is precomputed into display_active_milestone and
scoped to the active milestone — a bounded gauge, not an estate alarm.

Proposals until operator adoption is recorded (vsdd-cli #672). Authored
under phase-1c data authoring (vsdd-cli #598) — the package's closing
item. Draft vocabulary under the maturity lifecycle until first publish.
