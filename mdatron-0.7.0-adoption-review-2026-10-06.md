---
title: "mdatron 0.7.0 adoption and four-domain impact review (2026-10-06)"
tags: ["review", "dispatch", "design-input", "mdatron", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-06
updated: 2026-10-06
---

# mdatron 0.7.0 adoption and four-domain impact review (2026-10-06)

## Status

**Adoption merged; review fixes in pull request #58; feedback posted to mdatron; eight Solution Owner rulings owed.** Tracker: vsdd-cli#893. Pull request #57 (the adoption) merged 2026-10-06 as d7fd4eee with all three CI checks green. The ten review fixes are committed as 675ad95d on `chore/893-review-fixes` and open as pull request #58. The feedback to mdatron was posted on the operator's instruction as mdatron GitHub issue #73. The upgrade guide is vsdd-cli GitHub issue #55, written from the mdatron side's dry run; this page records what the adoption changed, what four domains found, where each finding went, and what was sent back to mdatron.

## What changed

The corpus needed no change: 53 files, verify clean, identical findings and families under 0.6.0 and 0.7.0, plus `rule_dsl=active`. The consumer side moved:

- The envelope pin: `.mdatron/envelope-3.1.0.schema.json` replaces 3.0.0 (printed by `mdatron envelope-schema`). The CI assertion moves to 3.1.0 on both `mdatron_output_version` and `envelope_schema`, and adds `families.rule_dsl.state == "active"` and, after review, `pipeline_status == "ok"` first. The exact-equality pin fired on this MINOR as implemented and is kept deliberately (see Decisions).
- Install pins: version, grep and cache key to 0.7.0 in `mdatron-verify.yml` and `vsdd-test.yml` (the guide named only the first); after review both fallbacks take `--force` and the grep is anchored. The adopter workflow template moved 0.5.0 to 0.7.0 (it had lagged since the 0.6.0 adoption). The README install line is pinned.
- Pre-commit: minimum version and window to 0.7.x; after review the absence-branch hint names the crates.io pinned form instead of the retired sibling checkout.
- `mdatron init` re-run: four inert `*.example` templates committed with their hashes in `manifest.yaml`. After review a CI step re-runs `init` against the pinned binary and fails on any diff under `.mdatron/`, so the templates are exercised rather than inert (only `init` detects template drift; verify reads the manifest for lineage).
- Alias migrations: `disjoint` operands `id_from` to `element` (`h3`, `list-item-bold-name`); `anti_patterns[].register` to `guidance`; the W0045 headline comment.
- An `every` rule beside the build-plan's count rule. As adopted: every H3 under `## Requirements` has the phase shape. After review: every heading of any level, and each must carry `Slice N —`. One E0123 per malformed heading at its own line.

Verification under 0.7.0, local and on CI: `verify --deny-warnings` clean; the jq assertion passes; `cargo test --workspace --locked` and clippy exit 0; mutants on a scratch copy fire E0120, E0121, E0122 and E0123 as expected (stray H4, heading without slice id, missing paren, completed bullet claiming an open slice, section renamed).

## The review

Four domains, one batch, read-only, no verifier fan-out, each reading its own domain prompt file in full: Platform Engineer, Quality Engineer, Solution Architect, VSDD Methodology meta-domain. Hand-run through the interactive Agent tool as a bootstrap interim, model Fable 5.1 at default effort. Stated budget 200k tokens each; actual 253k, 304k, 367k, 333k. Coverage declared: the release changes no corpus verdict, so the impact is confined to tooling, oracles, estate architecture, and the methodology's boundary, deferrals and record. Absent with reasons: AI Engineer (Slice 2's delivery decisions are #839's own round), Red Team (#855 residual 1 routed, not re-assessed), Documentation Reviewer (the guide's prose is mdatron's). The dispatch record is on #893.

**Verdicts.** All four: adequate and correctly scoped as a version re-pin; no contract amendment forced by 0.7.0. Three of four: the record overclaimed what `rule_dsl=active` proves.

## Findings and where they went

**Fixed in the follow-up pull request.** The `rule_dsl` wording (lane-level evidence, reproduced: every context in one pattern file pointed at a class no file declares still reports active with zero findings); `pipeline_status` asserted first; `--force` and anchored greps; the `init` drift step; the `every` rule widened and tightened; the pre-commit hint; the `config.yaml` code-catalog scope comment (false since `code_catalog_globs`); the `vocabulary.yaml` carve-out comment (the mdatron#28 raise shipped as the 0.6.0 default exemption, mdatron#159); the README pin; the CHANGELOG handle and adopter-template caveat.

**Routed to #855.** Validate the real envelope against the committed 3.1.0 schema in a vsdd-core test (the pinned schema is provenance only; the jsonschema crate is already a dependency). A seeded-defect proof through the real pattern files and a pin on `inputs.patterns`. Strengthen the jurisdiction canary test with the `inputs` digests (its docstring is stale at 0.4.0 and 79 files; `families.schema` is inert in its scenario). A two-glob narrowing of `file_globs` (53 to 30 files) still passes every family assertion; W0051 and W0054 under `--deny-warnings` catch a sloppy narrowing, a consistent one is silent by construction. The fail-open version guard is residual 2, unchanged, ruling requested.

**Routed to the cross-references test file.** One fire-and-sentinel pair per pattern code E0201 to E0206 and E0210 to E0212; today only E0207 and E0208 are exercised, and E0206 (`review-entry`) has no subject on this tree since the archive retired.

**Routed to #839 (Slice 2).** Route-bound schemas answer the skill-frontmatter question: a route binding `.claude/skills/vsdd-*/SKILL.md` to mdatron's standards-pack skill profile with `name_equals_dir: name` verifies clean over the estate plus one skill-shaped file and catches a name/directory mismatch (E0035); binding the vsdd class `domain-prompt` instead aborts the whole run (pipeline failed, zero files, E0080 kind eval: a `key()` lookup with a null index). So vsdd metadata and the cross-reference keys stay on a source corpus distinct from the skill output. The installed-artifact manifest and `init` have no skill class. `imports: true` is an existence check, not the rules-file binding; the generator's byte-check is the one home. W0054 fails `--deny-warnings` the moment `.claude/commands/vsdd-*.md` empties, so the mdatron configuration must move with the files.

**Routed to #882 / Slice 3.** The adopter workflow template cannot run verify (no jurisdiction is deployed; E0080) and calls a nonexistent `vsdd verify check`; its pin is current, its workflow is not runnable. Two managers over `.mdatron/` (vsdd's init manifest, mdatron's managed partition) with no declared partition.

**Routed to #844.** The body-link gap has been supported since mdatron 0.6.0: `links: true` on all five routes is clean on today's tree. The raise is retired; only the activation decision stands.

**Routed to #879.** A marker rule on the build-plan's Provenance lines against the contract's Decomposition section fires on exactly the two name drifts that review found by hand.

**Other follow-ups.** A mirror-pin test across every pin site (ten literals in four files plus the README). `coinage_globs` makes arming new-coinage detection possible (Slice 5). A summary-first `order` rule for `.design/` documents has no three-question decision yet.

## Decisions recorded on #893

- Exact-equality envelope pin retained deliberately: with an exact binary pin the envelope version is a function of the binary; a MINOR can add assertable members (3.1.0 did); the committed schema is top-level closed. Fire pair recorded as operating-effectiveness evidence (negative: the 3.0.0 assertion fails under 0.7.0; positive: the 3.1.0 assertion passes on CI), a candidate control-effectiveness registry entry.
- The `every` rule: three-question decision (in scope, supported, no raise) with the review addendum (widen to any heading; require the slice id). The `order` rule not adopted; the recorded reason corrected: the falsifier exists (E0124 fires under a section swap), what is absent is any consumer of H2 order.
- Per-family record under 0.7.0: schema, route, section, code_catalog, vocabulary, rule_dsl active; pin, link, marker, citation inactive with their homes named. Since 3.1.0 a family opted in but claiming nothing reports `inert`, so each active assertion also proves the family claimed a file.
- Raise loop closed: 0.7.0 ships vsdd-cli's own raises from the 0.6.0 adoption (route-table loudness W0053/W0054, `code_catalog_globs`, the `*.example` templates, `docs inputs`).

## Solution Owner rulings owed

(a) Fail-closed pre-commit version guard (#855 residual 2). (b) Arm `links: true` now or at Phase 4 (#844). (c) Land the pin family early for the build-plan Decomposition hash, which a one-entry section pin reproduces byte for byte. (d) Land the marker rule early with the #879 fix. (e) A closed H2 set rule and a no-H3-under-Completed-phases rule on the build-plan. (f) A handle-grammar form for GitHub-side issues (third recurrence). (g) Whether Dependency approval covers CI-installed binaries (mdatron, crosslink). (h) The build-plan Phase 4 family-list amendment.

## Feedback for mdatron

Posted 2026-10-06 as mdatron GitHub issue #73 ("vsdd-cli adoption notes and feedback on 0.7.0: guide accuracy, eight defects with reproductions, seven raises"), compiled from the four reports and the orchestrating session's own log; closes the reciprocal loop with vsdd-cli GitHub issue #55. Headlines: the guide omitted the test workflow's pin; `rule_dsl=active` is lane-level and unconditionally active under `--changed`; a `key()` lookup with a null index aborts the whole run instead of yielding a per-file finding; `verify` never checks managed-template drift; template refresh is bidirectional; the 3.1.0 schema still says "via `mdatron schema`"; the E0060 explain page claims a per-file version the manifest does not record; one E0122 per rule on a renamed section; the host-path leak's Fixed entry. Raises: stem-derived `requires_sibling`, standards-pack schemas as managed deployables, a Claude Code rules-file recipe, a shared pattern for count plus every, release attestation, a jurisdiction-shrinking FAQ entry, a shared GitHub-issue handle form. Positives: the section-pin span reproduced a hand-stated hash byte for byte; the marker family found exactly the drift a human review recorded; `docs inputs` was accurate for every key exercised; `inert` made every existing active assertion stronger for free.

## Process notes

Two agent commits were refused by the work-check gate although the session had an active issue: the hook's status call has a three-second budget and routine hub latency here is four to five seconds (upstream crosslink#104 item 2; the commit gate also ignores the `.active-issue` sentinel that `session work` writes, which the strict gate honours). The operator committed by hand. Readiness blocked several read-only commands intermittently for the reviewers and once for the session; nobody drove readiness and it cleared on its own. The reviewers wrote their reports through the shell because the Write tool was refused by the same hook.

## Sources

vsdd-cli#893 (dispatch record, routing, decisions, interventions); vsdd-cli GitHub issue #55 (the guide); pull request #57; mdatron at tag v0.7.0 (CHANGELOG, DESIGN, docs/cookbook, src/verify.rs); the four reports in the session scratchpad (platform-engineer, quality-engineer, solution-architect, vsdd-methodology).
