---
title: "The routing-before-fix guardrail (the unrouted-findings gate)"
tags: ["design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-07-29
updated: 2026-07-29
---


## Design Specification

### Summary

A `vsdd gate` command that consumes the now-live unrouted-findings query (Slice 1's finding-query join) and BLOCKS when a closed-by-fix finding in the forward-only universe carries no routing. Wired as a required CI leg on every PR — install crosslink, hydrate the tracker, run the gate — it turns the routing-before-fix discipline from detection (a `vsdd status` report that never changes the exit code) into an enforced CI-backed block. Fail-closed: an unverifiable acquisition blocks, never silently passes.

### Requirements

- REQ-1: A dedicated `vsdd gate` subcommand (the contract's phase-gate command, runnable under `crosslink swarm gate`): it acquires the snapshot, runs the unrouted-findings query, exits 0 (pass) when the query returns empty and non-zero (block) when any finding is unrouted. It is distinct from `vsdd status` (`vsdd/src/main.rs:63`), which reports the same query but always exits 0 ("an integrity finding never degrades the answer").
- REQ-2: The gate consumes a STATE-FREE unrouted-findings query. The predicate today lives inside `snapshot_integrity` (`vsdd-core/src/answer/integrity.rs:17`), which takes `&State` for the phase-pointer check; extract the unrouted-findings predicate into a standalone pure `unrouted_findings(snapshot: &Snapshot) -> Vec<String>` (the offending handles) so the gate runs without requiring `.vsdd/state.yaml` — a repo may gate before a state artifact exists.
- REQ-3: `snapshot_integrity` calls the extracted `unrouted_findings` for its own "unrouted-findings" kind, so the gate and the `vsdd status` report share ONE predicate and can never diverge.
- REQ-4: Fail-closed on a degraded acquisition. When `acquisition_outcome` is `Absent` (tracker offline / no `.crosslink`) or `Unusable` (broken), or the finding leg returned findings-absent while the tracker was present, the gate BLOCKS (non-zero) with a distinct message — an unverifiable gate never passes vacuously (the no-silent-holes discipline; the same principle as the finding-query cap marker and the non-vacuity canary).
- REQ-5: The block output names each unrouted finding (its handle) and the routing it needs — a `plan` comment naming the target phase, or the fix lane — so the block is actionable, not just a non-zero exit.
- REQ-6: A required CI leg on every PR installs crosslink at a pinned ref, fetches the hub branch, runs `crosslink sync` to hydrate the tracker into `.crosslink/issues.db`, builds vsdd, then runs `vsdd gate`. A non-zero gate fails the check and blocks merge — the CI-backed block grade.
- REQ-7: Block/pass/fail-closed tests: unit tests over the state-free query (unrouted finding → non-empty; routed / disposition-closure / out-of-universe / open → empty) and the gate's exit-code mapping (findings → non-zero; clean → zero; Absent/Unusable → non-zero fail-closed).

### Acceptance Criteria

- [ ] AC-1: `vsdd gate` exits non-zero when the acquired snapshot has a closed-by-fix, in-universe, unrouted finding.
- [ ] AC-2: `vsdd gate` exits zero when there are none (all closed-by-fix findings routed, out of the forward-only universe, or disposition closures).
- [ ] AC-3: `vsdd gate` exits non-zero (fail-closed) when `acquisition_outcome` is `Absent` or `Unusable` — never zero on an unverifiable run — with a message distinguishing "can't verify" from "clean".
- [ ] AC-4: the block output names the offending finding handle(s) and the routing each needs.
- [ ] AC-5: `unrouted_findings(snapshot)` non-empty iff `snapshot_integrity` emits the `unrouted-findings` kind for the same snapshot (gate and status never diverge).
- [ ] AC-6: the CI leg fails the PR check on a non-zero `vsdd gate` against the hydrated tracker.
- [ ] AC-7: unit tests cover block / pass / fail-closed; `cargo test --workspace`, clippy, and `mdatron verify` stay green.

### Architecture

`vsdd gate` lands as a new `Command` variant beside `Status`/`Init` in `vsdd/src/main.rs`. It calls `acquire_snapshot(&cwd)` (the same acquisition `status` uses), then the state-free `unrouted_findings`, and maps the result to `ExitCode`. No `.vsdd/state.yaml` read (unlike `cmd_status`, which needs state for the derivation).

The predicate extraction (REQ-2/REQ-3): move the unrouted-findings condition (`integrity.rs:82` — `status == "closed" && disposition.is_none() && !closed_before_ratification && !routing_present`, gated on `finding_fields_acquired.spine`) into `pub fn unrouted_findings(&Snapshot) -> Vec<String>` returning offending handles; `snapshot_integrity` pushes the kind iff that vec is non-empty. One predicate, two consumers (the gate blocks on it; status reports it).

Fail-closed mapping: `match snapshot.acquisition_outcome { Acquired => if unrouted_findings(&s).is_empty() { pass } else { block(handles) }, Absent | Unusable => block_unverifiable() }`.

The CI leg (`.github/workflows/`): checkout vsdd-cli; checkout + `cargo install` crosslink at a pinned ref; fetch the `crosslink/hub-v3-host` branch; `crosslink sync`; build vsdd; `vsdd gate`. Modeled on the existing mdatron-install pattern in `vsdd-test.yml`, but for crosslink + a hydration step. Required-check status blocks merge.

Interaction with #829: the gate acquires the full snapshot (milestone + session + finding legs). The milestone-empty-parse fix (#829) is a prerequisite — without it, a no-milestone repo's acquisition is `Unusable`, so the gate fail-closes (blocks) even with zero unrouted findings. The gate's end-to-end correctness depends on #829 landing on main first.

### Out of Scope

- The round-parity gate (a sibling gate command consuming the round-parity query) — a later increment.
- The contract-commit amendment-discipline gate (Layer 7) — separate.
- The open-survivor branch of the unrouted-findings query ("survives its round open") — needs round-membership data (Slice 6).
- A local pre-push hook — the operator chose the CI-backed block; a hook could be an additive convenience later, but it is friction-grade and out of scope here.

