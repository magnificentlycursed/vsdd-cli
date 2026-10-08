---
title: "VSDD in practice across the methodology author's repositories (2026-10-08)"
tags: ["design-input", "review", "dispatch", "process", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# VSDD in practice across the methodology author's repositories (2026-10-08)

## Status

**Index and summary. Design input for the phase-primer rewrite and the Slice 2 design.** On 2026-10-08 four read-only research agents reconstructed how the method is actually practised in the repositories of the organization that publishes the VSDD whitepaper (Corvidae-Coding-Projects): Thermite, Peritus, OpenClaudia and crosslink. The orchestrating session read ferrotorch, Palimpsest, crucible, LNP and rookery-nest itself. Nothing was executed in any checkout; hub refs were fetched read-only. This page carries the ten cross-repository facts, a comparison table, and the links. Each topic has its own page so that a question resolves with one search and one page, and so that a page attached to an issue is injected whole at session start (crosslink injects at most three pages of 8,000 characters).

**Exclusions.** mdatron was excluded as a design reference (it was designed under this estate's own contract). Everything this estate contributed to crosslink was excluded from the crosslink evidence. Thermite's RFC-as-file process is recorded but not generalized: it is a language-project convention no other repository uses.

**Caveat.** These are one maintainer's projects, built mostly by agents, under time pressure. More recent repositories are more likely to reflect current practice; recency is not proof of currency. The operator's own experience with the predecessor library (vsdd-suite, which worked when run by hand) is a fifth data point the repositories do not record.

## The ten facts

1. **The whitepaper's phase vocabulary is not used.** Zero occurrences of phase labels, "red gate", "roast" or "bead" in any maintainer commit, pull request, design document or hub comment. The native vocabulary: slice, increment, freeze, gate, pin, divergence, receipt, critic, fixer, root integrator, worker, independent final review, architecture verdict.
2. **No red gate for feature increments.** Tests, proofs and implementation land in one commit per increment everywhere. Failing-first is practised for bug fixes and for critic pins; mutation testing substitutes for "tests that would pass anyway".
3. **No multi-domain specification review.** Designs are self-reviewed by the root agent with the human, frozen by a commit, gap-analysed before kickoff, or gated on a pre-flight comment. The adversarial budget goes to the implementation tree.
4. **One reviewer, read-only, separately dispatched, plus CI as the second reviewer.** Peritus enrolls a fresh reviewer identity of a different model family per formally governed change. Thermite's critic may not fix, approve or give prose verdicts.
5. **Termination is mechanical.** Zero blocking findings plus every gate green; registry counts; stage checklists; a verified release policy. Headlines flip only at gate time.
6. **Formal hardening is first, not fifth.** Proofs from the first slice; axiom probes in CI; fuzzing, mutation and chaos as later campaigns.
7. **No refactor phase.** Architecture as policy on every commit; register-only passes as gated slices; the fixer forbids adjacent cleanup.
8. **The human does not review on GitHub.** Zero reviews on 96, 91 and 12 sampled pull requests. The human owns approvals, the push, the merge, secrets and redirections. The tracker is the work ledger; GitHub is intake and CI.
9. **Three evidence classes stay distinct in status wording** and are never upgraded in prose: deterministic gates passed; independent review passed; verifier or formal receipt recorded.
10. **The design document is written just before its build and amended during it.** Design-to-first-implementation latency: two hours to ten days. Umbrellas freeze within days; slice documents absorb amendments as dated paragraphs or appended delivery sections.

## The four repositories at a glance

| | Thermite (Jun–Aug 2026) | Peritus (Aug–Oct 2026) | OpenClaudia (Aug 2026) | crosslink (Dec 2025–Sep 2026) |
|---|---|---|---|---|
| Unit of specification | Umbrella program doc plus stage docs with kickoff plans; 81 component contracts over named file sets | Umbrella with 52 stable-ID requirements and 25 criteria; lettered slice docs frozen by their own commits | One audit, one remediation design, 108 numbered slice docs | One feature doc per feature, committed with or after the code |
| Requirements record | TOML registry, 524 requirements, typed evidence, generated views, CI-checked | Traceability table; obligations file with 157 proof obligations; architecture policy file | Backlog index with integrity invariants; status lines | Requirement and criterion IDs cited from commits; one criterion executable |
| Roles | doc-author, builder, critic, fixer (agent types with tool allowlists and manifests) | root integrator, path-scoped workers (three concurrent), read-only reviewers of another model family, human | one driver, waves of three code-only workers, orchestrator-as-reviewer, human | driver, kickoff implementer, architect or audit session, human |
| Adversary form | failing divergence test plus blocker issue; verdict only "generator must fix" or "no divergence found" | typed findings with severity, blocking flag, disposition; detached verdict with mandatory report | fresh-context pass with written verdict, one re-review | architect redirect rounds and an independent audit, in-session |
| Convergence | registry 518 of 524 shipped; gauntlet; gates G1–G4 | Gate A locally, hosted, and on fresh main; release policy never yet reached | administrative merge; zero slices reached Verified | closing result comment plus merge |
| Design-to-build latency | 0–10 days | hours to a day for slices | one day for all 102 slices, 14 days to implement | median same day |
| GitHub reviews | 0 of 96 | 0 of 91 | 0 of 3 sampled | 0 of 12; 1 of 193 at the former home |

## Pages

- `reference-practice-phases-specification` — phases 1a, 1b, 1c per repository, with divergences from the whitepaper.
- `reference-practice-phases-build-and-review` — phases 2a through 6 per repository, with divergences.
- `reference-practice-roles-dispatch-review` — the three-party shape, waves, ceilings, what "adversarial" meant, stop rules, finding forms.
- `reference-practice-records-gates-tooling` — the mechanical-control catalogue and the records discipline.
- `reference-practice-design-documents-and-estate-divergences` — umbrella plus slice documents, flow control, and this estate's six divergences.
- `reference-practice-prose-control` — Thermite's register standard, Palimpsest's mechanisms, the cross-domain mapping.
- `reference-practice-other-organization-repositories` — ferrotorch, crucible, Palimpsest, LNP, rookery-nest.
- `standard-comparison-estate-vs-references-2026-10-08` — where this estate's standard exceeds the references and where it falls short.
- `append-accumulation-retrospective-2026-10-08` — why decisions accumulated as appends instead of integrating; the mechanism and the prior fixes.
- `reconciliation-ledger-2026-10-08` — the 39 divergences between recorded decisions and the contract, build-plan, data and register, across Slices 1 to 7.
- Evidence: `practice-report-thermite-2026-10-08`, `practice-report-peritus-2026-10-08`, `practice-report-openclaudia-2026-10-08`, `practice-report-crosslink-2026-10-08` — the four agent reports with citations at commit, pull-request and hub-issue level.
- `portable-memory-rules-gist-2026-10-08` — a practitioner's published feedback memories (the author of Observability Engineering, 2nd edition): the six habits, the eighteen rules, the three instruction-file layers.
- `clarity-review-skill-2026-10-08` — the same author's pre-flight lint for AI-authored code and prose: seven pattern classes with litmus tests, corpus-calibrated, invoked only explicitly.
- `cross-reference-2026-10-08-findings-vs-prior-knowledge` — the day's findings read against the observability engineering pages, the earlier Thermite assessments, the domain value scorecard, the memory rules and the clarity skill, with adoption candidates and their homes.
