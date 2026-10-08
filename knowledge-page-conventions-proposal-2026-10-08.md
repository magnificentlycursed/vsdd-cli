---
title: "Knowledge-page conventions: a proposal for splitting, surfacing and provenance (2026-10-08)"
tags: ["design-input", "process", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---


## Design Specification

### status

**Proposal for the operator's ruling; nothing here is in force.** Written after the 2026-10-08 research arc published 25 pages and read about 20 more, in answer to three questions: which pages should be broken up and expanded, how pages get surfaced when they are needed, and how source, revision and retrieval date are carried so that pages do not reproduce the append problem (`append-accumulation-retrospective-2026-10-08`). Grounded in what the tool does today: `crosslink knowledge add` writes frontmatter with title, tags, a sources list (url, title, accessed_at, populated by the source flag), contributors, created and updated; pages live on a git branch, so each page's history is its version history; a page attached to an issue by a design-doc label is injected at session start, at most three pages of 8,000 characters each; search is substring over the pages.

### 1. the split rule, and which pages it applies to

**The rule.** One page per question a session will ask, under 8,000 characters when the page is meant to be injected. Three kinds of page, stated in the first line: a *reference* (how something works, at a named revision), an *evidence* page (a report or a record, allowed to be long, never the injected one), and an *input* page (what a specific design or rewrite needs, short, attachable). A cluster of pages gets an index page listing each with a one-line "read when". This is the shape Thermite's knowledge branch and Palimpsest's architecture folder already have.

**Pages reviewed this session that the rule says to split or re-kind:**

| Page | Size | What to do |
|---|---|---|
| `content-delivery-assessment-2026-10-02` | 35 KB | Split: the delivery-slot table and limits (reference); the three live tests (evidence); the proposal and four-domain review (record). Keep the original as the record and link the parts |
| `crosslink-integration-surfaces` | 34 KB, five sections marked not re-verified | Split by surface, each re-verified at a named revision; retire the sections nobody re-verified instead of carrying them under a warning |
| `kickoff-swarm-dispatch-pipeline` | 39 KB | Split kickoff (current) from swarm (retired by the 2026-10-02 amendment); the swarm half becomes a record |
| `verifiable-conformance-and-efficiency` | 77 KB | Re-kind as retired rationale, the contract member governs; extract per-slice input pages only when a slice opens (the oracle, skill invocation, operator session and spend-shape sections for Slices 4 and 6) |
| `gate-execution-slice`, `composition-slice`, `install-slice`, `finding-query-join`, `routing-before-fix-guardrail` | 24 to 36 KB | Re-kind as retired designs with a superseded-by line once the live slice documents exist; no split |
| `oe2e-impact-analysis-2026-09-18` | 12 KB | Keep as a record; its routed items B1 to B9 are promises and belong in the promise record |
| `regression-corpus` | 10 KB | Migrate to versioned data as the contract already says; the page becomes a pointer |
| `reconciliation-ledger-2026-10-08` | 24 KB | Split per slice so a slice's section can be attached to its issue; keep the whole as the record |
| `cross-reference-2026-10-08-findings-vs-prior-knowledge`, `cross-reference-recursion-wtf-2026-10-08` | 18 and 15 KB | Keep as records; extract the two adoption tables into one short input page that can be attached to the primer-rewrite issue |
| `reference-practice-phases-build-and-review` | 13 KB | Split 2a to 2c from 3 to 6 |
| `reference-practice-prose-control` | 12 KB | Split Thermite's register from the Palimpsest mapping |
| the four `practice-report-*` pages | 40 to 48 KB | Evidence; keep |
| `thermite-assessment`, `thermite-state-architecture`, `domain-value-scorecard` | 3 to 4 KB | Keep; add a superseded-in-part line pointing at the 2026-10-08 pages |

### 2. surfacing, from cheapest to strongest

1. **Attach input pages to the issue that needs them** with design-doc labels, so they arrive at session start. Example: the Slice 2 issue carries `reference-practice-design-documents-and-estate-divergences`, the ledger's Slice 2 section and `reference-practice-phases-specification`. This is why the split matters: three pages, 8,000 characters each.
2. **Path-scoped rules for file-triggered knowledge.** Editing a primer or domain prompt triggers a rule naming the register, clarity and prompt-authoring pages; editing a registry data set triggers the data-authoring pages. This is the 2026-10-02 delivery decision applied to knowledge.
3. **A maintained cluster index** with one line per page and its "read when", replacing the July stub. Maintained by convention at each add until a check exists.
4. **Primers cite pages by slug at the step that needs them.** The primer rewrite is the place.
5. **Slug and tag discipline for search:** a prefix per kind (`reference-practice-`, `cross-reference-`, `practice-report-`), a date suffix on records, tags for kind; search is substring, so the slug carries the search.
6. **The strong form, later:** a route table from governed files to governing pages with a read-before-edit gate, which the contract's Conformance at action time already names and Thermite runs today.

### 3. provenance, so pages do not append

**Every page's first paragraph states, in this order:** its kind; its source with revision (a repository at a commit, a URL with the date fetched, a tracker record by handle); the date read or verified; its status (current, superseded by a named page, or retired); and what it supersedes. External sources also go through the source flag so the accessed date is machine-readable in frontmatter.

**Three rules that stop the append pattern:**

- A re-verification is a new dated page that supersedes the old one, never a correction paragraph stacked at the top. The old page gets one line: superseded by. Thermite's RFC process, crucible's decision records and the write-plans skill all state this form: records are immutable and later records supersede with cross-links.
- Versions are derived, not declared: the knowledge branch's history per page is the amendment history; no version field.
- A page that cites code or a tool states the revision in its first line, so a reader knows when it stopped being true without a currency section.

**Grade.** Convention, because the knowledge branch is outside the working tree the conformance engine walks. A small check over the page list and first lines, run in CI against a checkout of the branch, would raise it to a block; that is a candidate, not part of this proposal.

