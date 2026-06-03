# Phase 1 — Reserved-Code Drift + DSL Fixes

**Issue:** crosslink #12.
**Parent plan:** [`../binary-first-plan.md`](../binary-first-plan.md) Phase 1.

This phase splits across three artifacts per VSDD discipline (one per
sub-phase). The Phase 1a/1b/1c condensation that originally lived in a
single combined DESIGN.md was retroactively split on 2026-06-02 per
operator-directive (Phase 4 disposition of Phase 3 SC + VSDD-methodology
self-authorization findings).

## Sub-phase artifacts

- [Phase 1a — Behavioral Specification](./phase-1a-behavioral-spec.md)
- [Phase 1b — Verification Architecture](./phase-1b-verification-architecture.md)
- [Phase 1c — Decomposition + Acceptance Criteria](./phase-1c-decomposition.md)

## Implementation status

- Phase 2a Red Gate: committed `mdatron@3aae201` (10 tests; 5 failing-by-default)
- Phase 2b implementation: `mdatron@ebf6320` + `vsdd-cli@13429b9` (10/10 green;
  schema-tightening reverted)
- Phase 2c polish: skipped (no refactor surface)
- Phase 3 cluster-batched cold-session: 18 domain reviews filed under
  `review-log/2026-06-02-*-phase-1-bundle.md`
- Phase 4 dispositions: amended DESIGN-MDATRON.md (`mdatron@1d81c4e`);
  rewrote circular probe test + added nested-Field test (`mdatron@8dc0392`);
  extended workspace-walk lint to non-`.rs` carriers (`mdatron@bc83588`);
  Phase 1a/1b/1c sub-phase split (this commit)
