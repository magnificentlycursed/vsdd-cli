---
title: "Regression corpus — recorded escapes, named evasions, deferred fixtures (interim home until Slice 4's versioned data)"
tags: ["reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-09-19
updated: 2026-09-19
---

# Regression corpus (interim home)

The enumerated escape paths agents have taken, or reviewers have named, against the implemented system — each with its control, its non-forgeable trace, and an enforcement-versus-advisory mark (contract: Verifiable conformance and efficiency). A probe-found escape is added as found; a fixture is authored when the escape's occasion first arises, audited at the owning slice's phase-2a corpus audit. **Interim home:** this page, until Slice 4 materializes the corpus as versioned data with its schema pair beside the control-effectiveness registry. Opened by the 2026-09-18 compaction (vsdd-cli#873; the Deletion-Test sweep's REGISTER bin, SO decision 1).

Legend — trace: the harness-produced record that proves the escape or the control firing · mark: **enforced** (a built control blocks it) / **detective** (a built check reports it) / **advisory** (no built control; named only) / **unbuilt**.

## A. Recorded escapes (occurred)
| id | escape | control | trace | mark |
|---|---|---|---|---|
| spend-shape-verifier-fanout-mdatron | Phase-3 roast sprawled to 39 agents / 2.27M tokens: the gate ratified the lenses, not the run's shape (mdatron#11) | spend-shape bound legs 1–3 | harness usage totals (local; could-not-check grade) | advisory |
| spend-shape-verifier-fanout-vsdd | Phase-3 roast of mdatron ran 32 agents / 1.8M tokens against a declared token band, refutation as a per-finding verifier fan-out, after the control was ratified but before it reached the primer (vsdd-cli#867, vsdd-cli#869) | spend-shape bound leg 1 (now in the phase-3 primer v0.2.0); leg 2 per-agent caps on the vehicle; leg 3 post-hoc flag | harness usage totals (local; could-not-check grade) | advisory |
| paraphrase-composition-bypass | The #840 cycle's opening dispatched a general-purpose agent with a paraphrase of the composition (vsdd-cli#840) | injection + skill-invocation audit | trace `attributionSkill` events (see the runtime-harness supplement) | unbuilt (hand-audited) |
| personas-unread-#821 | Cold-review lenses read no domain persona; the lens came from the dispatch prompt (vsdd-cli#821) | skill-invocation audit | trace Read / invocation events | unbuilt |
| in-session-phase-work | The orchestrator does phase work in-session, unaudited (the #840 design's own authoring; the 2026-09-18 compaction, marked hand-audited) | phases-dispatched rule; AC-17 marking | tracker marking comment | advisory (marking is convention) |
| ungated-hotfix-mdatron | Security-labeled fix landed direct-to-main through none of the gates (mdatron#51) | fix-lane red gate + lifecycle falsifier | gate record; commit evidence section | detective (Slice 1 routing) → enforced (Slice 4) |
| inline-review-mdatron | Retro-reviews run inline in the orchestrating session under hand-coined review-log names (mdatron#52) | manifest falsifier; underivable-name check | manifest presence; route table | unbuilt |
| hollow-shell-install | Tracked hook wiring survived a re-clone while payloads were lost; guard shims degraded to silent no-ops in both repos | install manifest + installed-artifact-integrity check | Status integrity finding | enforced (engine) |
| dependency-approval-lapse | icu_properties landed with no record, review, or trailers (vsdd-cli#872) | Dependency approval (simplified) | docs/dependencies record presence | advisory (convention) |
| stale-decomposition-pin | The build-plan's Decomposition pin drifted after #860/#862 and nobody heard the "loud drift signal" (found 2026-09-18) | design-doc pin check | pin hash vs section bytes | advisory (convention) → enforced (doc-drift check, Slice 5) |

## B. Named evasions of the fix-lane gate (never occurred; mechanics held here, built when Slice 4 needs them or on first occurrence)
| id | evasion | control sketch | mark |
|---|---|---|---|
| mapping-set-valued | A relocated test split or merged across paths to slip a regression past a one-to-one mapping | set-valued from→to mappings; every target inherits dormancy scrutiny; approval record enumerates every pair | unbuilt |
| decoy-mapping-target | A regression mapped onto a fresh passing target | validator asks what each pair relocates; a mapping onto a pre-existing or failing target rejected | unbuilt |
| bulk-mapping-mismatch | A declared row buried in a large table absent from the approval record | set-equality between declared and approved sets | unbuilt |
| cfg-vanished-target | A mapping target vanished from collection (cfg-gated) | report set is the join's universe; such a relocation degrades to a source-path disablement | unbuilt |
| relocated-dormancy | A mapped target arriving reported-but-non-executing at HEAD | dormancy scrutiny composes with the mapping; the disablement declaration rides the same record | unbuilt |
| split-invocation | Baseline and HEAD runs from different gate invocations stitched into one record | one invocation stamp spanning both runs; a split record fails | unbuilt |
| in-place-quarantine | A baseline-passing test ignored at HEAD, path surviving | removal-shaped regardless of path; takes a declared lane or fails the delta | unbuilt |
| spurious-compile-red | A fix-introduced test unbuildable at the baseline claimed as red | compile failure is red only under a validator-approved compile-defect declaration | unbuilt |
| false-broken-surface | A cannot-run claim against a suite that runs | the gate's own baseline run attempt rejects it | unbuilt |
| gutted-body-untouched-sidecar | a `.design/` document's body gutted while its pipeline sidecar's doc_hash stays untouched — letter-compliant today because nothing compares the hash to the bytes | a CI check comparing each sidecar's doc_hash to its document's bytes, with a decision-carrying re-baseline sanctioned | unbuilt |
| hand-authored-design-doc-and-sidecar | a brand-new design doc and sidecar hand-authored outside the design flow — a creation, so the sanctioned-operations rule never fires | the same CI check plus a flag on `.design/` body changes with no recorded Solution Owner decision; upstream raise for tool-mediated sidecar creation | unbuilt |

## C. Deferred fixture classes (from the former Fixture corpus enumeration; one-sided until their occasion arises)
directive-reconciliation violation seeds; the delta's tolerance case (a pre-existing failure neither blocking nor excusing); the dangling survivor reference; the unapproved compile-defect declaration; the missing declared-kind seed; the free-slug-only and zero-finding inline-review variants; the preflight fail and unknown directions; the stalled-agent fixture; the execution-on-attended-vehicle seed; the unresolved-question pair; the unmanifested-artifact fixture; the hand-rolled-paved-path seed; the agent-invoked-dispatch seed; the late-writer seed; the fail-closed-wiring seed; the post-compaction stale-read seed; the label-narrowing consolidation seed; the genuinely-new-ignored-test fixture; the two-repo composed-display fixture; the tracker-corroboration fixtures; the oracle-provenance seeds; the completed cost cycle with baselines.

## D. Unexercised legs of Per-milestone PR discipline (carried here, not claimed as controls)
| id | leg | status |
|---|---|---|
| manual-tests-merge-gate | merge validates every manual-test item checked or deferred (VSDD-E0090) | unbuilt; checklists scoped to operator-facing slices (SO 2026-09-18) |
| bypass-marker-gate | a PR carrying a bypass marker requires the approval label from a non-author | authored in the adopter payload, never wired here |
| co-authorship-trailers | prose commits carry verifiable co-authorship | self-reported; the phantom-trailer condition is vsdd-cli#657 |

## E. Advisory controls registered from the conformance design (ruled REGISTER, 2026-09-18; none has an occurrence)
| id | control | trace | mark |
|---|---|---|---|
| limit-less-read-of-governed-file | a full-file Read of a large governed file where a scoped slice was available is flagged; read cost modeled as compounding re-inclusion net of the cache-read discount, not a per-read boolean | trace `tool_use` Read inputs (offset/limit) + usage by cache class | advisory (cost-and-efficiency report, Slice 7) |
| cross-agent-redundant-load | sibling agents under one parent posting high cache-creation over overlapping Read ranges instead of cache-read — the fresh-load proxy for "same base context loaded N times" | trace usage by cache class + Read ranges | advisory (Slice 7); the warm-handoff dispatch primitive is the by-construction half |
| issue-created-without-prior-search | a new tracker issue opened with no recorded search / find-or-create before it (the consult-before-create duty) | tracker: search event preceding the create | unbuilt (a find-or-create enforcement point; the pre-code guard forces create, never search) |
| mid-cycle-without-milestone | a mid-cycle slice with no active crosslink milestone; the designed detector exists only in Status's absence report | Status integrity finding | detective (Status) → an active-milestone enforcement point, unbuilt |
| named-mechanism-left-unbuilt | an available mechanism that is the correct home for a control left unused: sentinel (scheduled sweeps), intervene (mid-flow records), cron / push / remote triggers (budget and rate-limit alerts), the viewers (the surfacing home), the paved-path map's own detection | the paved-path map vs session records | advisory (an underutilization report, Slice 7) |
| could-not-check-shown-as-recorded | a figure whose source record is unavailable presented as recorded or measured | provenance tag per figure | unbuilt (the report's provenance audit, Slice 7); falsifier restored in the contract |
