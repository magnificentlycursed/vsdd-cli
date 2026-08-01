---
title: "Slice 4 — Gate execution and the mutation floor"
tags: ["design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-01
updated: 2026-08-01
---


## Design Specification

### Summary

Slice 4 builds vsdd's own gate machinery: the red-gate command at both the
layer scale and the fix scale (checkout-and-run, agent assertion plays no
part), the standing-suite delta with its three removal-shaped lanes and
approval machinery, the invocation stamp, graded retrofit, the broken-surface
form, the phase-exit gate that mechanizes the engine's unrouted-findings query
at command scale over fixtures, the terminal-round stop-signal check, and the
mutation floor. It EXTENDS the routing-before-fix guardrail Slice 1 already
shipped (`vsdd gate`, the state-free `unrouted_findings` / `gate_verdict`
queries) rather than duplicating it, and it EXTENDS the existing CI workflows
with gate steps plus a thin pre-commit wrapper — not a new enforcement surface.
It is NOT the defunct mdatron subprocess client (closed as vsdd-cli #835); it
is vsdd's own gate execution engine.

### Requirements

- REQ-1: **Red gate at layer scale (checkout-and-run).** A gate command performs the checkout and run for the layer red gate defined in *Phase exit by gate*: phase 2a closes only when the layer's declared test suite fails against the pre-implementation commit and the failing run is recorded; phase 2b closes only when the same suite passes at HEAD. The gate performs the checkout and run; agent assertion plays no part. Runs are recorded as `GateResult` values (`vsdd-core/src/state/schema.rs:62`) with the existing `GateKind::RedGate` / `GateKind::GreenGate` variants written to `.vsdd/state.yaml` and to the tracker.
- REQ-2: **Red gate at fix scale (checkout-and-run + test application).** The gate reads the finding issue's recorded baseline commit (for a single-commit fix, the fix commit's parent), the named pin test(s), and the declared expected failure kinds (a non-empty set drawn from the valid-red members of the failure-kind enumeration in `templates/registry/gate-data.md`). It checks out the baseline, APPLIES the defect's pin test to the baseline — test application is the defined fix-scale mechanism, since a fix and its test may share one commit — runs the named test(s) in isolation for defect attribution, and requires red at baseline and green at HEAD. The wrong-reason comparison's second operand (the declared expected-kind set) is a required field, never inferred. `GateKind::FixScaleGate`. Agent assertion plays no part.
- REQ-3: **The executed-pin discipline.** A valid red is a run-time failure of an executed pin: the named test built, was collected, ran, and failed with a recorded kind that is a member of the finding's declared set (the six `valid-red` kinds in `gate-data.md`, doctest-compile-failure among them). A valid green is the symmetric executed pass — built, collected, ran, passed. A skipped, ignored, filtered, or not-collected pin is neither red nor green but its own recorded state satisfying neither half (the `neither` kinds in `gate-data.md`). Wrong-reason is a mechanical comparison: a recorded test-identity mismatch against the finding's declaration, or a recorded kind outside the declared set — membership, not equality, so the check stays mechanical under platform/run-order kind instability.
- REQ-4: **The standing-suite delta with three removal-shaped lanes and their approval machinery.** The gate runs the repo's declared test command at workspace scope (`gate-data.md` `cannot_run_predicate.command_binding` — the same command `crosslink swarm gate` resolves) at baseline and HEAD and compares by runner-reported test path. No test that passed at baseline may fail at HEAD. The removal-shaped set (a baseline-passing test gone, moved, or dormant at HEAD) is partitioned by intent into exactly three lanes, each carrying validator approval: **deletion** (an obsolete passing test removed — path + reason, approved); **mapping** (relocation — set-valued from/to per `gate-data.md` `mapping_schema`, splits and merges lawful, a path in at most one from-set and one to-set, targets drawn from the HEAD runner report, the approval record enumerating every from-to pair and the gate checking the approved pair set equals the declared pair set); **disablement** (declared dormancy of a still-valid test — path + reason + approval, covering both a runtime skip whose path survives and a cfg-gated flip whose path vanishes). Each affected path takes exactly one primary lane plus one stated mechanical exception (a mapped target the runner cannot report degrades to a source-path disablement; a mapped target arriving non-executing carries the composed disablement declaration on the same record). Approval for all three lanes clears one bar — a single declared, git-tracked approval record (Architecture, *The removal-lane approval record*), read from the HEAD tree and checked by set-equality against the gate's computed removal set: every computed removal must be declared and every declared entry must correspond to a computed removal. An undeclared baseline-passing path absent at HEAD fails the delta — the join fails closed, never dropping the vanished test.
- REQ-5: **The invocation stamp.** The gate runs baseline and HEAD in one invocation on one runner; each run's `GateResult` (`vsdd-core/src/state/schema.rs:62`) carries a new `invocation` stamp — a per-invocation identity (a nonce generated once per gate invocation and written to both the baseline and HEAD runs) plus a monotonic/time basis (an ISO-8601 start instant and a monotonic clock reading captured at invocation start) and the runner identity. The stamp check requires the paired runs to share one invocation identity and one runner and to form the complete ordered pair; a split record (two invocations — differing identities or runners) and a post-Slice-4 gate record missing its stamp both fail the check, fail-closed. The field is a forward-compatible, optional-with-default schema addition (Architecture, *The invocation stamp*), so pre-stamp records still deserialize. This mirrors the one-acquisition-per-invocation count-conduct pattern already established in `vsdd-core/src/snapshot/acquire.rs:4`.
- REQ-6: **Graded retrofit.** A fix that landed ungated is remediated by the retrofit form — the same gate run performed after the fact against the recorded baseline, marked `retrofit` on the finding issue: a full record where the pin is expressible, the validator-approved surface-limitation where it is not, partial evidence declared as partial, never a fabricated equivalence. The retrofit closes verification debt, not the process breach — the deviation record stands (the fix still trips the lifecycle falsifier Slice 5 owns). The same clause governs forward and retrofit so neither path fabricates a red the topology cannot produce.
- REQ-7: **The broken-surface form and its two no-executed-pin siblings.** Three no-executed-pin forms meet one bar — validator approval, the primary discriminator: **surface-limitation** (a pin inexpressible against the baseline surface), the **compile-defect** declaration (validator-approved; its compile-red validly satisfies the gate — the acceptance branch, not only rejection), and **broken-verification-surface** (a defect that breaks the verification surface itself — a build or suite that cannot run — whose recorded failing state is the red evidence, reviews following landing). The gate's own baseline run attempt is the additional mechanical check on the two suite-cannot-run forms (never surface-limitation, whose suite runs by definition), rejecting the claim outright when the suite in fact runs — the `cannot_run_predicate` in `gate-data.md` (its command binding, report binding, three cannot-run arms, and 600s timeout semantics).
- REQ-8: **The flake-resistance repetition policy.** The gate repeats the pin runs and the suite delta per `gate-data.md` `flake_policy` (`runs_per_gate_execution: 3`), aggregating at the declared-set grain: red only if the pin fails in every run with each observed kind in the declared set; green only if it passes in every run; a mixed outcome is the flaky recorded state satisfying neither half. The delta fails closed — a baseline-passing test that aggregates flaky at HEAD fails the delta.
- REQ-9: **The phase-4-exit gate over fixtures (reuse of the engine query).** The `vsdd gate phase-exit` subcommand (Architecture, *Gate command surface*) mechanizes, at phase-4-exit command scale over the fixture corpus, the same unrouted-findings query Slice 1 wired at the commit/CI guardrail over live data: it consumes the engine's `unrouted_findings` (`vsdd-core/src/answer/integrity.rs:105`) and `gate_verdict` (`integrity.rs:138`) — Phase 4 closes only when the query returns empty, and a fixture with an unrouted finding fails phase-4 exit. The result is recorded as `GateKind::PhaseExitGate`. The gate reuses the query, adding the phase-exit command surface and its fixture-based criterion closure; it does not re-implement the predicate.
- REQ-10: **The terminal-round stop-signal check.** Phase 3 closes only at the stop signal — a terminal round producing only hallucinated (or otherwise disposition-closed) findings. Phase-3 exit fails while the terminal round holds a real finding. The check reads the round children and their dispositions the engine's acquisition already carries (Slice 1 wired round manifests and children into `acquire_snapshot`); the stop signal made mechanical.
- REQ-11: **The mutation floor.** When the review config declares the criterion in force (the thorough preset does — Summary *Review config* line; *Deterministic composition* — the config field is Slice 2's), the mutation kill-ratio on changed code is a standing gate criterion, and a fixture below the declared floor fails the gate. Mutation testing runs `cargo-mutants` (the decided tool) CI-enforced on the pure core — `vsdd-core` (and the cost crate when Slice 7 ships it) — per the *Phase 5 strategy* line. "Changed code" is the set of mutants `cargo-mutants` generates within the PR diff against the merge base (`cargo mutants --in-diff`), bounding CI cost to the change under review; the kill-ratio is caught / (caught + missed), excluding unviable and timeout mutants (cargo-mutants' own categorization). The floor NUMBER is a phase-1c data-authoring item authored into `templates/registry/gate-data.md` (not yet present there); the config declares only whether the criterion is active, and the gate reads the declared floor from the data rather than hardcoding it. **Starting floor: 65% (ratified 2026-07-30, #836)** — a conservative starting calibration, set below typical mature-Rust-crate targets (~80%+) so it does not block on day one before a baseline exists but well above chance, to be ratcheted upward once a full-tree `cargo mutants` baseline run establishes vsdd-core's actual survivor set.
- REQ-12: **The guardrail — extend CI + a thin pre-commit wrapper.** The gate steps extend the existing CI workflows (`.github/workflows/routing-gate.yml`, `.github/workflows/vsdd-test.yml`) rather than adding a new surface, and a thin pre-commit wrapper (peer to `.githooks/pre-commit`) runs the fix-scale gate (`vsdd gate red --scope fix`, Architecture, *Gate command surface*) at commit time. Per the guardrail single-source rule, the block logic lives in the vsdd binary and its data; the git-hook wrapper and the `vsdd init` payload wrapper (Slice 3) differ only in wiring. A per-commit guardrail wall-clock budget is authored at this slice's phase-2a entry, the standing obligation of every slice binding a per-commit git-hook guardrail (Slices 1 and 4).
- REQ-13: **Terminal-output safety on gate outputs.** The gate's diagnostics surface tracker-sourced handles and runner-report content (external strings); this slice wires the shared cleaner `clean_for_terminal` (`vsdd-core/src/text.rs:120`) into its gate outputs, per the per-slice terminal-output-safety inheritance.
- REQ-14: **The state-consistency deep-history half of Status detection.** This slice closes the concurrent-boundary clean-merge-inconsistent fixture of *Status detection* — re-homed to vsdd (vsdd-cli #739) and riding the gates — so a deep-history clean-merge that produces an inconsistent state is reported, not silently accepted.
- REQ-15: **The fresh-container install-and-smoke gate (manual-recurring).** As a payload-extending slice, Slice 4 re-runs the fresh-container install-and-smoke check over its own payload delta at its phase-2a entry — the bootstrap form of the *Gates* requirement's `crosslink kickoff --container` mechanization, carried as a manual-recurring check until that command ships. Its smoke oracle is: every installed-artifact-manifest entry resolves and `vsdd status` renders a working segment at exit 0. This is not a fixture-based Gate-coverage member (promoting it is a routed phase-1a act, not taken here).

### Acceptance Criteria

- [ ] AC-1 (REQ-1): The red-gate-cheat fixture reproducing the layer-7 incident (a suite that passes green against the pre-implementation base) is blocked by the gate, and a fixture attempting phase-2b entry with no recorded red-gate failure is blocked (*Red-gate cheat blocked*, layer-scale conduct).
- [ ] AC-2 (REQ-2, REQ-3): The `compliant-fix` fixture (finding filed before implementation, gate-performed executed red at baseline and executed green at HEAD via test application) passes the gate falsifier on its merits; the `green-at-baseline` fixture (a named test that never goes red against the baseline) is rejected — the canonical cheat exercised at fix scale.
- [ ] AC-3 (REQ-3): The `skipped-pin-at-HEAD` fixture satisfies neither half and fails the gate; the `wrong-reason-failure` fixture (recorded kind outside the declared set, or identity mismatch) does not satisfy the fix-scale gate.
- [ ] AC-4 (REQ-3, REQ-7): The `spurious-compile-red` fixture (a pin unbuildable at the baseline) is rejected as no demonstration, while the `declared-compile-defect` fixture's validator-approved compile-red validly satisfies — both branches of the fork exercised.
- [ ] AC-5 (REQ-4): The `pin-green-suite-red` fixture fails the gate on the standing-suite delta; the `rename-in-fix` fixture fails the delta on its undeclared vanished path and passes through its declared, validator-approved mapping.
- [ ] AC-6 (REQ-4): The `declared-deletion` fixture passes as the lawful removal form; the `unapproved-declared-deletion`, `unapproved-mapping`, `dishonest-mapping`, and `decoy-mapping` (a regression mapped onto a fresh passing target) fixtures are all rejected at the shared validator-approval bar; the `bulk-mapping-mismatch` seed fails set-equality (a declared row absent from the approval record in a large table).
- [ ] AC-7 (REQ-4): The `declared-disablement` fixture passes as the lawful dormancy form; the `in-place-quarantine` seed (a baseline-passing test ignored at HEAD, undeclared) is flagged as removal-shaped; the `unapproved-disablement` seed is rejected; the `cfg-vanished-disablement` fixture joins as explained; the `cfg-vanished mapping-target` fixture is rejected and degrades to a source-path disablement; the `relocated-dormancy` pair passes with its composed disablement declaration and is flagged without it; the `genuinely-new-ignored-test` fixture blocks nothing.
- [ ] AC-8 (REQ-5): The `split-invocation` seed (a gate record whose baseline and HEAD runs carry differing `invocation` identities or runners) fails the stamp check, and a post-Slice-4 gate record missing its `invocation` stamp fails closed; a single-invocation record with one shared stamp across the ordered baseline+HEAD pair passes.
- [ ] AC-9 (REQ-6): Both retrofit fixtures still trip the lifecycle falsifier (the deviation stands) — the `separable-retrofit` yields a gate record marked `retrofit`, the `inseparable-retrofit` is recorded as the validator-approved surface-limitation, never a full record.
- [ ] AC-10 (REQ-7): The `broken-verification-surface` fixtures in both directions resolve correctly — the genuine, validator-approved failing state is accepted as red evidence, and the false claim against a suite that runs is rejected by the gate's own baseline run attempt; the `surface-limitation` fixture passes the gate falsifier while the `unapproved-surface-limitation` and `unapproved-broken-surface` fixtures are rejected at the approval conjunct (the discriminator the contract names primary).
- [ ] AC-11 (REQ-8): The mutation/flake behavior repeats each pin run and suite delta the declared number of times (3), and a baseline-passing test that aggregates flaky at HEAD fails the delta closed.
- [ ] AC-12 (REQ-9): A fixture with an unrouted finding fails phase-4 exit, and a fixture with none passes; the phase-exit gate's verdict is non-empty iff the engine's `unrouted_findings` is non-empty for the same fixture snapshot (gate and the `vsdd status` report never diverge — one predicate, *Gate coverage*).
- [ ] AC-13 (REQ-10): A fixture whose terminal round contains a real finding fails phase-3 exit; a terminal round of only hallucinated/disposition-closed findings passes the stop-signal check (both firing directions, *Gate coverage*).
- [ ] AC-14 (REQ-11): A `mutation-floor` fixture with a computable kill ratio (caught / (caught + missed), cargo-mutants' categorization) below the declared floor fails the gate; one at or above the floor passes.
- [ ] AC-15 (REQ-2, REQ-11): The consolidation fixture gates on ALL the survivor's pins (the gate side of the consolidation path), and the fix-scale gate runs under the finding-label composition (the security-labeled fix summons the security domain and the mutation floor when the config declares one).
- [ ] AC-16 (REQ-11, REQ-12): The gate steps run as legs of `routing-gate.yml` / `vsdd-test.yml` on PR, the `cargo-mutants` mutation leg runs on `vsdd-core` (`cargo mutants --in-diff` against the merge base) and fails below the declared floor, and the pre-commit wrapper runs the fix-scale gate (`vsdd gate red --scope fix`) at commit time and exits non-zero on a fix-scoped commit whose gate record is absent or red; `cargo test --workspace`, clippy, and `mdatron verify` stay green (`vsdd-test.yml`).
- [ ] AC-17 (REQ-13): Gate output containing a crafted control/format-character handle or runner string is emitted cleaned (`clean_for_terminal` applied), matching the existing terminal-safety assertions in `vsdd-core/src/text.rs`.
- [ ] AC-18 (REQ-14): The concurrent-boundary clean-merge-inconsistent fixture is reported by the state-consistency check, not silently merged.
- [ ] AC-19 (REQ-15): Running the smoke oracle over Slice 4's payload delta, every installed-artifact-manifest entry resolves and `vsdd status` exits 0 with a working segment; a hollow-shell tree (a manifest entry with an absent payload) is reported.
- [ ] AC-20 (REQ-1, REQ-2, REQ-5): The runnable-mini-repo fixture form (Architecture, *Fixtures*) is exercised by a real git checkout-and-run — the `compliant-fix` mini-repo is checked out to its baseline commit, its pin test applied and run in isolation (executed red), and re-run at HEAD (executed green), with the baseline+HEAD run pair carrying one shared `invocation` stamp; the snapshot-based corpus (state/snapshot/expected trios) continues to back the pure-query ACs (phase-exit, terminal-round, state-consistency) with no checkout.

### Architecture

Slice 4's code lands as vsdd-core's gate module plus the `vsdd` binary's gate
command surface, mapping onto the contract's Architecture-sketch component
"Slice 4: vsdd-core's gate and the CI workflows." The phase-1a behavioral
contracts here are followed by a phase-2a crate-level module map (a separate
act); the CLI-contract half — the subcommand surface — is fixed below (*Gate
command surface*), and the reuse and data-flow are fixed throughout.

**Gate command surface.** Slice 4 EXTENDS Slice 1's existing `vsdd gate`
(`vsdd/src/main.rs:187`, `cmd_gate`, dispatched from the `Command::Gate` arm at
`main.rs:28`, `GateArgs` at `main.rs:59`) with a subcommand layer rather than
adding top-level commands. The bare `vsdd gate` (no subcommand) is preserved as
Slice 1's routing-before-fix guardrail over live data — its 0/1/2 exit contract
and its `routing-gate.yml` invocation keep working unchanged. Slice 4 adds:
`vsdd gate red` (the red gate, `--scope layer|fix`, default `fix` — the
checkout-and-run of REQ-1/REQ-2, recording `GateKind::RedGate` /
`GateKind::GreenGate` / `GateKind::FixScaleGate`); `vsdd gate phase-exit`
(`--phase 3|4` — the terminal-round stop-signal check of REQ-10 and the
unrouted-findings-over-fixtures check of REQ-9, recording
`GateKind::PhaseExitGate`); and `vsdd gate mutation` (the mutation floor of
REQ-11). The pre-commit wrapper calls `vsdd gate red --scope fix`; the CI legs
call the subcommand each workflow needs (*The guardrail*). The exact flag
spelling is refined at the phase-2a module-map act; the subcommand set and its
dispatch relationship to Slice 1's bare `vsdd gate` are fixed here.

**What Slice 1 already provides (reuse, do not duplicate).** The
routing-before-fix guardrail (`.design/routing-before-fix-guardrail.md`) shipped
the first gate: the `vsdd gate` command (`vsdd/src/main.rs:187`, `cmd_gate`); the
state-free `unrouted_findings(&Snapshot) -> Vec<String>`
(`vsdd-core/src/answer/integrity.rs:105`) shared by `snapshot_integrity` and the
gate so report and block never diverge; the fail-closed `gate_verdict(&Snapshot)
-> GateVerdict` (`integrity.rs:138`) with `Pass` / `Block(handles)` /
`Unverifiable(reason)` and exit codes 0/1/2; and the CI leg
`.github/workflows/routing-gate.yml` (checkout, `cargo install` crosslink @
develop, `crosslink sync` to hydrate the tracker, run the gate). The live
acquisition (`vsdd-core/src/snapshot/acquire.rs`) already reads findings, their
routing presence, round manifests, and round children. Slice 4 consumes all of
this unchanged.

**What Slice 4 adds.** The red-gate engine — layer-scale and fix-scale
checkout-and-run — is new: it drives git checkout of a recorded baseline, test
application at fix scale, isolated pin runs, and the standing-suite delta with
the three removal-shaped lanes and set-equality approval checks. The invocation
stamp binds one baseline+HEAD run pair per gate record (schema at *The
invocation stamp* below), following the one-acquisition-per-invocation pattern
documented at `vsdd-core/src/snapshot/acquire.rs:4`. Gate runs are recorded
through the state
artifact's already-modeled types — `GateResult`, `GateKind` (`RedGate`,
`GreenGate`, `FixScaleGate`, `PhaseExitGate`), and `GateOutcome`
(`vsdd-core/src/state/schema.rs:39-71`) — via the existing `write_state` path,
and mirrored to the tracker (*Phase exit by gate*: gate results are recorded in
the state artifact and the tracker). The phase-exit gate (REQ-9/10) reuses
`unrouted_findings` / `gate_verdict` for the phase-4 empty-query condition and
reads the acquired round children + dispositions for the phase-3 terminal-round
check.

**The invocation stamp.** `GateResult` (`vsdd-core/src/state/schema.rs:62`)
gains one field, `invocation: Option<InvocationStamp>`, a forward-compatible
addition marked `#[serde(default, skip_serializing_if = "Option::is_none")]` —
the same optional-with-default posture the schema already uses for
`last_gate_result` and `published` (`schema.rs:23,28`) — so pre-stamp state
files still deserialize under the struct's `#[serde(deny_unknown_fields)]`. The
new `InvocationStamp` carries: `invocation_id: String` (a nonce generated once
per gate invocation and written to both the baseline and HEAD run records — the
per-invocation identity); `started_at: String` (ISO 8601, the wall-clock time
basis); `monotonic_ns: u64` (a monotonic clock reading captured at invocation
start — the monotonic basis, also ordering the two runs); `run_index: u32`
(0 = baseline, 1 = HEAD — the ordered pair); and `runner: String` (the runner
host, for the one-runner clause). The split-invocation check (REQ-5) passes only
when the baseline and HEAD records share one `invocation_id` and one `runner`
and form the complete ordered pair `{0, 1}`; differing identity/runner (a split)
or a missing stamp on a post-Slice-4 gate record fails closed. This mirrors the
one-acquisition-per-invocation count-conduct documented at
`vsdd-core/src/snapshot/acquire.rs:4`.

**The removal-lane approval record.** The three removal lanes (REQ-4) clear one
approval bar via a single declared, git-tracked approval record — not a live
crosslink comment — so the check is deterministic at commit/CI time and
replayable over the fixture corpus (consistent with the fixture seam below). The
record is a YAML document read from the checked-out HEAD tree at a fixed path
(proposed `.vsdd/removal-approvals.yaml`; fixtures carry their own copy under
the fixture dir), and the finding issue references the record handle so the
tracker linkage survives while the git-tracked file is the authoritative approved
set. Proposed schema `(ratified 2026-07-30, #836)`:

```yaml
schema_class: removal-approval
schema_version: 0.1.0
records:
  - change: <HEAD commit sha or the fix/finding handle the removal rides>
    baseline: <the baseline commit sha the delta is computed against>
    entries:
      - kind: deletion          # an obsolete passing test removed
        targets: [<baseline-report test path, absent at HEAD>]
        reason: <why this passing test is obsolete>
        approver: <validator identity/handle>
        rationale: <the validator's stated basis for approval>
      - kind: mapping           # relocation; set-valued (splits + merges lawful)
        from: [<baseline-report paths absent at HEAD>]
        to:   [<HEAD-report paths absent at baseline>]
        reason: <text>
        approver: <validator identity/handle>
        rationale: <text>
      - kind: disablement       # declared dormancy of a still-valid test
        targets: [<baseline-passing path: a surviving runtime skip, or a
                   cfg-gated flip whose path vanishes>]
        reason: <text>
        approver: <validator identity/handle>
        rationale: <text>
```

The gate computes the removal set R (baseline-passing test paths gone, moved, or
dormant at HEAD, partitioned into the three lanes by comparing the baseline and
HEAD runner reports), expands each declared entry into lane-tagged path claims —
deletion targets as removed, mapping into the declared from→to PAIR set, and
disablement targets as dormant/vanished — and checks SET EQUALITY: every
computed removal must be declared and every declared claim must correspond to a
computed removal (a declared row with no matching removal fails as a stale or
dishonest declaration; an undeclared removal fails the fail-closed join). The
mapping half is checked as pair-equality (approved pair set == declared pair
set, `gate-data.md` `mapping_schema`), with a path in at most one from-set and
one to-set. Every entry across all three lanes MUST carry a non-empty `approver`
and `rationale` — the one-bar-on-all-three discriminator (REQ-4, REQ-7). The
eventual home for this schema is the phase-1c data package alongside
`templates/registry/gate-data.md`'s `mapping_schema`; it is drafted here as the
concrete phase-1a contract.

**Engine data the gate reads.** `templates/registry/gate-data.md` (schema_class
`gate-data`) supplies the failure-kind enumeration with `red_validity` and
`scope`, `pin_kind_declaration` (declared expected-kind set, wrong-reason by
membership), `flake_policy` (`runs_per_gate_execution: 3`, per-test
aggregation), `cannot_run_predicate` (command binding, report binding, the three
cannot-run arms, `timeout_seconds: 600`), and `mapping_schema` (report-set
default `confirmed`, set-valued entries, the approval-record set-equality rule
the removal-approval schema above operationalizes). The mutation floor NUMBER is
a distinct phase-1c data-authoring item authored into that same `gate-data.md`
set (not yet present there — proposed 65%, see REQ-11). The review config
(`.vsdd/config.yaml`, *Deterministic composition* — Slice 2's field) declares
only whether the mutation-floor criterion is IN FORCE (per preset); the gate
reads the activation from the config and the floor number from the data.

**Where mutation testing runs.** Mutation testing targets the pure core —
`vsdd-core` (and the cost crate at Slice 7) — enforced as a CI leg per the
*Phase 5 strategy*. The decided tool is `cargo-mutants`: no mutation tool is
referenced in the contract, `Cargo.toml`, or the workflows today (confirmed by
grep), so its adoption is new to this slice. The CI leg runs `cargo mutants
--in-diff <merge-base diff>` on `vsdd-core`, scoring caught / (caught + missed)
and failing below the declared floor (REQ-11); a full-tree `cargo mutants`
baseline run establishes the ratchet baseline the floor climbs toward. The CI
wiring extends `vsdd-test.yml` / `routing-gate.yml` (REQ-12) rather than adding a
new workflow file, and the per-commit git-hook wrapper is a peer of
`.githooks/pre-commit`.

**Terminal-output safety.** Gate diagnostics run tracker- and runner-sourced
strings through `clean_for_terminal` (`vsdd-core/src/text.rs:120`), the same
cleaner already applied at `acquire.rs:244`.

**Fixtures.** The fix-lane corpus (*Fixture corpus*) is Slice 4's phase-2a
deliverable. Slice 4 uses TWO fixture forms. (1) The existing snapshot-based
corpus under `vsdd-core/tests/fixtures/` (the `convergence/` fixtures use the
`state.yaml`/`snapshot.yaml`/`expected.yaml` trio; `state/` holds the state
fixtures) continues to back the pure-query ACs — phase-exit (REQ-9),
terminal-round (REQ-10), and state-consistency (REQ-14) — which need no real run.
(2) A NEW runnable-mini-repo form is added for the checkout-and-run cases (REQ-1,
REQ-2, REQ-7's cannot-run/broken-surface arms), because the red gate performs a
real git checkout-and-run and a static snapshot alone cannot supply a baseline to
check out. Each runnable fixture is a self-contained, deliberately tiny tree — a
minimal crate with one or two trivial tests plus a `manifest.yaml` naming the
pin test(s), the declared expected-kinds, and the removal-approval record — that
the test harness materializes into a throwaway git repo in a temp dir (baseline
commit, then the HEAD delta), so the gate really checks out the baseline, applies
the pin, and runs; keeping the mini-repos tiny keeps the ~40-member corpus
hermetic and fast. These land in a new `vsdd-core/tests/fixtures/gate/` subdir
alongside `convergence/` and `state/`. The gate abstracts the run behind a
runner-report boundary so the SAME code path serves live and fixture runs — the
fixture supplies a real mini-repo, not a canned report.

### Out of Scope

- The mdatron subprocess client / any mdatron-integration gate — a defunct line, closed as vsdd-cli #835. This slice is vsdd's own gate machinery.
- Round-parity and never-started/dispatch-failed detection, and their fixtures — the round-parity query (a manifest's narrated finding count reconciled against its round's tracked children) and the never-started signatures need Slice 6's dispatch-manifest data and launch/heartbeat instruments. This is the *named split* with Slice 6 on *Gate coverage*: the "unfiled round findings fails phase-3 exit" and never-started falsifiers home at Slice 6; Slice 4 owns the terminal-round, unrouted-findings phase-4-exit, and mutation-floor falsifiers.
- The swarm live fire (Slice 6's one-time exit act).
- The composition function, config-integrity rules, and the `.vsdd/config.yaml` mutation-floor activation FIELD and presets (Slice 2) — Slice 4 only READS the criterion's activation from the config and the floor number from `gate-data.md`, and consumes fix-scale labels as the composition's surface input.
- `vsdd init` and the managed-file install payload (Slice 3) — Slice 4 depends on install behavior for the fresh-container smoke oracle and packages its gate wrappers into that payload, but does not build install.
- The lifecycle falsifiers Slice 5 owns: evidence-gated filing, closure grading, consolidation machinery, waiver ENUMERATION, and the lifecycle-side flags on the un-owned ungated-hotfix, malformed-disposition-closure, multi-finding-commit, and inline-review fixtures. Slice 4 owns only the GATE side of the shared fixtures (e.g. the consolidation fixture's "gate runs all pins").
- Making the engine's acquisition queries fire on live tracker data (Slice 1, already shipped) — Slice 4 exercises the queries over fixtures.
- Turning the CI gate legs into hard landing-blocks via branch-protection required-status-checks — the operator's to set.
- `crosslink kickoff --container` command mechanization — the fresh-container install-and-smoke gate stays a manual-recurring check here; promoting it to a fixture-based Gate-coverage member is a routed phase-1a act, not taken here.
- The contract-commit amendment-discipline gate and the #815 tamper-evidence corroboration oracle — separate, format-carried / follow-on work.

### resolved decisions

All five open questions are resolved and folded into the REQ / AC / Architecture
above. Two carry a concrete proposal awaiting operator ratification — marked
`(ratified 2026-07-30, #836)` at their home and repeated here.

### Q1 (resolved): Mutation tool and kill-ratio floor
Tool = `cargo-mutants`, CI-enforced on `vsdd-core` (and the cost crate at Slice
7). "Changed code" = mutants generated within the PR diff against the merge base
(`cargo mutants --in-diff`); kill-ratio = caught / (caught + missed), excluding
unviable and timeout mutants. The floor NUMBER is phase-1c data authored into
`templates/registry/gate-data.md`; the config declares only whether the criterion
is in force. See REQ-11 and Architecture *Where mutation testing runs*.
**Starting floor: 65% (ratified 2026-07-30, #836)** — a conservative
starting calibration, below typical mature-Rust-crate targets (~80%+) so it does
not block day one before a baseline run exists but well above chance, to be
ratcheted upward once a full-tree `cargo mutants` baseline establishes
vsdd-core's actual survivor set.

### Q2 (resolved): Gate command surface
Subcommands under the existing `vsdd gate`, not top-level commands: bare `vsdd
gate` stays Slice 1's live routing-before-fix guardrail; Slice 4 adds `vsdd gate
red` (`--scope layer|fix`), `vsdd gate phase-exit` (`--phase 3|4`), and `vsdd
gate mutation`. See Architecture *Gate command surface* (REQ-1, REQ-2, REQ-9,
REQ-11, REQ-12).

### Q3 (resolved): Removal-lane approval-record format
A single declared, git-tracked approval record (not a live crosslink comment),
read from the HEAD tree at a fixed path and checked by SET EQUALITY against the
gate's computed removal set, uniform across deletion / mapping / disablement. The
concrete schema is in Architecture *The removal-lane approval record* and is
marked **`(ratified 2026-07-30, #836)`** there. See REQ-4.

### Q4 (resolved): Invocation-stamp representation
Add `invocation: Option<InvocationStamp>` to `GateResult` — a forward-compatible,
optional-with-default schema addition. The stamp carries a per-invocation
identity (`invocation_id` nonce), a monotonic/time basis (`started_at`,
`monotonic_ns`), the ordered `run_index` (0 baseline / 1 HEAD), and the `runner`.
Split invocations (differing identity/runner) and a missing stamp both fail
closed. See REQ-5 and Architecture *The invocation stamp*.

### Q5 (resolved): Fix-lane fixture structure
Add a runnable-mini-repo fixture form (tiny self-contained trees the harness
materializes into a throwaway git repo and the gate really checks out and runs),
distinct from the snapshot-based corpus, landing in a new
`vsdd-core/tests/fixtures/gate/` subdir. The snapshot corpus continues to back
the pure-query cases. See REQ-1/REQ-2, AC-20, and Architecture *Fixtures*.

