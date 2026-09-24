---
title: "Ontological review of the vsdd design and code base (2026-09-24)"
tags: ["design-input", "review", "observability"]
sources: []
contributors: ["xqjG"]
created: 2026-09-24
updated: 2026-09-24
---

# Ontological review of the vsdd design and code base (2026-09-24)

What was reviewed: the contract `.design/agent-first-vsdd-toolkit.md` (compacted, PR #34), `.design/build-plan.md`, the thirteen data sets under `templates/registry/`, the mdatron configuration, the code base (`vsdd-core` 32 files, `vsdd` 11 files, 14,003 lines at ad07b484), the tests, hooks and CI, and the vsdd-cli PR #37 rename diff. Method: the ontology discipline of *Observability Engineering* 2e (see [[o11y-ontology-invariants-and-the-dispatch-plan]]): extract the nouns, invariants and relationships each side states, then find the **semantic failures** — the same concept under different rules in different places, entities bound to nothing, and closure claims without their oracle.

## Shape and cost

Five agents, one round, no per-finding verifiers, effort medium, Workflow fallback under `manual-dispatch-fallback`: Solution Architect (contract), Data Engineer (registry data), Software Engineer (modules and types), Quality Engineer (tests and CI against the build-plan's claims), Documentation Reviewer (cold read of the rename diff). Declared worst case 350k tokens; actual 967k (2.76×), 145 tool uses, 10 minutes wall-clock. The pricing error is recorded in the operator-legibility memory: a whole-corpus reviewer costs ~200k regardless of effort dial; a diff-only cold read ~130k. Entities extracted: SA 31, DE 23, SE 33, QE 27. Findings: 17 major, 33 minor, 15 notes, 26 could-not-check items.

## Where the design disagrees with itself (semantic failures)

Convergent findings, seen independently by two or more reviewers, listed first.

| Concept | Homes that disagree | Reviewers | Routed to |
|---|---|---|---|
| Mutation floor number and home | economics-data `kill_ratio_percent: 80`; build-plan says 65 ratified into gate-data; gate-data has no field; no cargo-mutants anywhere | DE, QE | vsdd-cli#879 item 6, #836 |
| Domain roster partition | contract: quartet / process-governing / product-reviewing; six `tier: core` prompts (UX and Security included); presets `standard`/`minimal` omit process-governing domains the verifier will block | SA, DE, SE | #879 item 2, #839 |
| Install payload count | contract and build-plan Slice 3: 15/62; build-plan Slice 1 and AC-11 assertion: 16/63 | SE, QE | #881 item 2 |
| The governed repo's phase | contract header says build; no `.vsdd/state.yaml`, so `vsdd status` here always reports state absent | SA, SE, QE, DE | #881 item 4, #855 |
| Retired-term prohibition | registry `deprecated_aliases` (mdatron cannot read it; deployed to adopters); `.mdatron/vocabulary.yaml` anti_patterns (the one that fires); membership differs | DE, QE, DR | #882 item 2 |
| Composition mode enumeration | state-schema `skill-interactive|cold-dispatch`; primers `operator-orchestrated`, `reviewer-cold-session`; the superseded attended/autonomous split still names the mode in four places | SA, DE | #879 items 2–3 |
| VSDD- code roster | catalog `comprehensive: true` with 11 codes; patterns and binary emit E0201–E0212, E0230 | DE, QE | #882 item 3 |

Single-reviewer majors: the Phase-3 stop signal has three rules and none covers a zero-finding round (SA → #879 item 1, #836); the agent-count ceiling is block-grade at ratification but has no field or gate anywhere (SA → #879 item 4); the build-plan omits the Verifiable-conformance legs for Slices 4/6/7 and counts twelve criteria of thirteen while the pin stays satisfied (SA → #879 item 5); adopters receive the data sets without schema pairs, routes or config (DE → #882 item 1).

## Where the code disagrees with the record

- **Status process-integrity checks** are built in `vsdd-core/src/integrity_shell` and called only by tests; `cmd_status` never joins them. A hollow install is a quiet no-op, the exact falsifier Conformance at action time names. Recorded CLOSED. (SE → #880 a)
- **Round-parity and unresolvable-handles** iterate inputs that acquire.rs never populates and report checked-clean with no could-not-check marker: the #818 shape recurring. (SE → #880 b)
- **Install-offer conduct** is CLOSED with "fixtures in every enumerated direction"; no such fixture exists, only a data-count assertion. (QE → #881 item 1)
- **Convergence corpus** counts two fixtures whose expected answers say "redline for operator adoption": a closed criterion carrying agent-only oracles. (QE → #881 item 3)
- **Signal parity**: pre-commit and CI run different check sets. (QE → #881, Slice 4's pre-commit wrapper)
- Minors: three three-valued verdict enums; `vsdd gate` legs have no `GateKind` and record nothing; two date and version conventions; invariants the code could assert at load time (owner ≠ validator, standing entries need a retest trigger) and does not.

## Names still carrying retired meaning

`baseline` vs `calibration band`; `launch failure` / `dead classification` / `dispatch-failed`; `unknown` vs `inconclusive` (preflight); `expiry` vs "review date"; `draft-proposal|adopted|established` vs unstable/stable; Layer N in four data sets and the paved-path map; `phase` used for swarm segments in the build-plan; `hook trace log` bound to nothing; `static price` / `efficiency insight engine` headings vs the contract's member names. All on #879 as minors. Identifier-level survivors in code (`run_record`, `affordance`, `layer`) are out of #874's scope and tracked as the schema-pair rename follow-on.

## Rename cold read (PR #37)

Verdict minor-fixes: four broken sentences, `first_introduced_in` wrong on the five new terms, three anti-patterns narrower than their aliases, hyphenated and compound survivors. All fixed at 32e1cb67. Both manual-test proposals are clearly marked and every command they cite exists.

## Rulings requested of the Solution Owner

1. Mutation floor: one number, one home (#836).
2. Presets: re-seed `standard`/`minimal` to include the process-governing set, or restate the contract so presets only add to the floor (#839).
3. Install-offer closure: re-open the members or build the fixtures (#881).
4. Convergence redlines: adopt the two oracles or move them out of the counted corpus (#881).
5. `.vsdd/state.yaml` for this repo: write it or register a bootstrap exemption with a retest trigger (#881, #855).
6. Name the surviving split (operator attended slots vs dispatched agent-work) and retire the attended/autonomous phrase (#879).

## Could not check (selected)

Whether a recorded ruling exempts the estate from carrying a state artifact; whether GitHub's ruleset requires the three status checks; whether crosslink's swarm parser accepts any heading form other than `### Phase N:`; the internals of `init.rs` classification logic, `text.rs`, `subprocess.rs`; whether the hub currently holds a fix-closed finding without a plan comment (gate not run live). The full per-reviewer lists are on the four home issues.

Sources: workflow run wf_a7af0f17-323 (journal in the session's subagents directory); issues vsdd-cli#879–#882; related [[deletion-test-sweep-2026-09-18]], [[contract-compaction-change-map-2026-09-18]], [[regression-corpus]].
