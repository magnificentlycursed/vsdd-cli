---
title: "Knowledge-page conventions: a proposal for splitting, surfacing and provenance (2026-10-08)"
tags: ["design-input", "process", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---

# Knowledge-page conventions: a proposal for splitting, surfacing and provenance (2026-10-08)

## Status

**Proposal for the operator's ruling; nothing here is in force.** Written after the 2026-10-08 research arc published 25 pages and read about 20 more, in answer to three questions: which pages should be broken up and expanded, how pages get surfaced when they are needed, and how source, revision and retrieval date are carried so that pages do not reproduce the append problem (`append-accumulation-retrospective-2026-10-08`). Grounded in what the tool does today: `crosslink knowledge add` writes frontmatter with title, tags, a sources list (url, title, accessed_at, populated by the source flag), contributors, created and updated; pages live on a git branch, so each page's history is its version history; a page attached to an issue by a design-doc label is injected at session start, at most three pages of 8,000 characters each; search is substring over the pages.

Revised 2026-10-08, the day it was published, under the operator's vocabulary and citation decisions of that day: "phase skill" replaces "primer", "rules file" replaces "supplement", "reviewer role" replaces "domain prompt", milestones are named by feature instead of "Slice N", "increment" is the unit of work dispatched as one issue, and "oracle" is kept only for the expected results the operator authors (what the verifier reads is the synced trace; what a review produces is a verdict record). Words quoted from a source keep the source's words. The decisions are recorded on `vsdd-cli#839` and on `terminology-grounding-2026-10-08`. Handles cited on this page are listed with their titles at the end.

Phase names on this page are the contract's: 1a behavioral specification, 1b verification architecture, 1c the spec review gate, 2a test-suite generation (the red gate), 2b minimal implementation, 2c refactor, 3 adversarial refinement, 4 the feedback integration loop, 5 formal hardening, 6 convergence. The whitepaper has six phases; the a, b and c splits are this repository's.

Milestones are named by feature. On the tracker (`crosslink milestone list`) they are: the live self-governance milestone, #8 "Slice 1 — Live self-governance"; the composition milestone, #9 "Slice 2 — Composition, generated context, and static price"; the install milestone, #10 "Slice 3 — Install"; the gate-execution milestone, #11 "Slice 4 — Gate execution and the mutation floor"; the finding-lifecycle milestone, #12 "Slice 5 — Finding lifecycle and conformance"; the recorded-dispatch milestone, #13 "Slice 6 — Recorded dispatch and directive flow"; the cost milestone, #14 "Slice 7 — The cost crate".

"This estate" means vsdd-cli together with crosslink and mdatron, the two sibling tools it runs on.

## 1. The split rule, and which pages it applies to

**The rule.** One page per question a session will ask, under 8,000 characters when the page is meant to be injected. Three kinds of page, stated in the first line: a *reference* (how something works, at a named revision), an *evidence* page (a report or a record, allowed to be long, never the injected one), and an *input* page (what a specific design or rewrite needs, short, attachable). A cluster of pages gets an index page listing each with a one-line "read when". This is the shape Thermite's knowledge branch and Palimpsest's architecture folder already have.

**Pages reviewed this session that the rule says to split or re-kind:**

| Page | Size | What to do |
|---|---|---|
| `content-delivery-assessment-2026-10-02` | 35 KB | Split: the delivery-slot table and limits (reference); the three live tests (evidence); the proposal and four-domain review (record). Keep the original as the record and link the parts |
| `crosslink-integration-surfaces` | 34 KB, five sections marked not re-verified | Split by surface, each re-verified at a named revision; retire the sections nobody re-verified instead of carrying them under a warning |
| `kickoff-swarm-dispatch-pipeline` | 39 KB | Split kickoff (current) from swarm (retired by the 2026-10-02 amendment); the swarm half becomes a record |
| `verifiable-conformance-and-efficiency` | 77 KB | Re-kind as retired rationale, the contract member governs; extract per-milestone input pages only when a milestone opens (the conformance evidence, skill invocation, operator session and spend-shape sections for the gate-execution and recorded-dispatch milestones) |
| `gate-execution-slice`, `composition-slice`, `install-slice`, `finding-query-join`, `routing-before-fix-guardrail` | 24 to 36 KB | Re-kind as retired designs with a superseded-by line once the live milestone documents exist; no split |
| `oe2e-impact-analysis-2026-09-18` | 12 KB | Keep as a record; its routed items, which that page labels B1 to B9, are promises and belong in the promise record |
| `regression-corpus` | 10 KB | Migrate to versioned data as the contract already says; the page becomes a pointer |
| `reconciliation-ledger-2026-10-08` | 24 KB | Split per milestone so a milestone's section can be attached to its issue; keep the whole as the record |
| `cross-reference-2026-10-08-findings-vs-prior-knowledge`, `cross-reference-recursion-wtf-2026-10-08` | 18 and 15 KB | Keep as records; extract the two adoption tables into one short input page that can be attached to the phase-skill-rewrite issue |
| `reference-practice-phases-build-and-review` | 13 KB | Split 2a to 2c from 3 to 6 |
| `reference-practice-prose-control` | 12 KB | Split Thermite's register from the Palimpsest mapping |
| the four `practice-report-*` pages | 40 to 48 KB | Evidence; keep |
| `thermite-assessment`, `thermite-state-architecture`, `domain-value-scorecard` | 3 to 4 KB | Keep; add a superseded-in-part line pointing at the 2026-10-08 pages |

## 2. Surfacing, from cheapest to strongest

1. **Attach input pages to the issue that needs them** with design-doc labels, so they arrive at session start. Example: the composition milestone's issue, `vsdd-cli#839`, carries `reference-practice-design-documents-and-estate-divergences`, the ledger's composition section and `reference-practice-phases-specification`. This is why the split matters: three pages, 8,000 characters each.
2. **Path-scoped rules for file-triggered knowledge.** Editing a phase skill or reviewer role triggers a rule naming the register, clarity and prompt-authoring pages; editing a registry data set triggers the data-authoring pages. This is the 2026-10-02 delivery decision applied to knowledge.
3. **A maintained cluster index** with one line per page and its "read when", replacing the July stub. Maintained by convention at each add until a check exists.
4. **Phase skills cite pages by slug at the step that needs them.** The phase-skill rewrite is the place.
5. **Slug and tag discipline for search:** a prefix per kind (`reference-practice-`, `cross-reference-`, `practice-report-`), a date suffix on records, tags for kind; search is substring, so the slug carries the search.
6. **The strong form, later:** a route table from governed files to governing pages with a read-before-edit gate, which the contract's Conformance at action time already names and Thermite runs today.

## 3. Provenance, so pages do not append

**Every page's first paragraph states, in this order:** its kind; its source with revision (a repository at a commit, a URL with the date fetched, a tracker record by handle); the date read or verified; its status (current, superseded by a named page, or retired); and what it supersedes. External sources also go through the source flag so the accessed date is machine-readable in frontmatter.

**Three rules that stop the append pattern:**

- A re-verification is a new dated page that supersedes the old one, never a correction paragraph stacked at the top. The old page gets one line: superseded by. Thermite's RFC process, crucible's decision records and the write-plans skill all state this form: records are immutable and later records supersede with cross-links.
- Versions are derived, not declared: the knowledge branch's history per page is the amendment history; no version field.
- A page that cites code or a tool states the revision in its first line, so a reader knows when it stopped being true without a currency section.

## 4. Citation form, ruled 2026-10-08 for the pages published that day and proposed as the standing form

- A tracker handle carries its title at first mention or in a key at the end of the page (`vsdd-cli#839`, "Slice 2 (Composition) phase-1a design", opened with `crosslink issue show 839`); GitHub-side records are spelled out ("mdatron GitHub issue #79"; "PR #59"); a knowledge page is cited by title and slug.
- A milestone is named by its feature ("the composition milestone"), with the tracker's own title in the key; the unit of work dispatched as one issue is an increment.
- Phases are named at first mention on a page ("phase 2a, test-suite generation, the red gate"), with the whitepaper's six phases and this repository's a, b and c splits stated once.
- No labels of this estate's own making: no letter-number ids, acronyms or all-caps status tokens; a row is cited by its plain title. A source record's own labels (Thermite's RFC-5, the impact analysis page's B1 to B9) are cited as that record's, with the page or file that holds them named.
- Dates carry the year; "this estate" is expanded at first mention as vsdd-cli with crosslink and mdatron; a file is cited by path; an external source by URL with the date read.
- Vocabulary follows `terminology-grounding-2026-10-08`: phase skill, rules file, reviewer role, milestone by feature name, increment, attestation; "oracle" only for the expected results the operator authors. Words quoted from a source keep the source's words.

**Grade.** Convention, because the knowledge branch is outside the working tree the conformance engine walks. A small check over the page list and first lines, run in CI against a checkout of the branch, would raise it to a block; that is a candidate, not part of this proposal.

## Handles cited on this page

Open any `vsdd-cli#N` with `crosslink issue show N`; the title and state are as of 2026-10-08.

- `vsdd-cli#839`: Slice 2 (Composition) phase-1a design — vsdd way: composition function + config-integrity ... [open]
- `PR #N` is a pull request on this repository's GitHub side, opened with `gh pr view N`.
- mdatron GitHub issue #79 is the roadmap feedback filed from this repository on 2026-10-08.
