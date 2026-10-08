---
title: "Reference practice: design documents, flow control, and this estate's divergences"
tags: ["design-input", "process", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---


## Design Specification

### status

Design input for the Slice 2 design and the phase-primer rewrite. How the reference repositories keep design documents, how design flows into implementation, and where this estate's practice diverges; from the orchestrating session's own reads of crosslink's design workflow and documents, Thermite, Peritus, OpenClaudia, ferrotorch and the VSDD whitepaper, plus the four `practice-report-*` pages, on 2026-10-08 (index: `vsdd-in-practice-reference-repositories-2026-10-08`).

### the reference shape

**One stable umbrella plus many per-slice or per-component documents.**

- The umbrella is a thesis or program document: Thermite's 27 KB thesis is cited by section number from every component contract ("thesis-refs"); Peritus's 119 KB umbrella carries the staging order, the requirement groups, the traceability table and the merge gates, not the slices' requirements; OpenClaudia's remediation design carries principles, the definition of operational and the workstreams, with the slices as an execution backlog that cites it as canonical.
- The umbrella freezes within days. Thermite's program document: four commits, all in its first week. Peritus's: six commits in nine days, then none. OpenClaudia's audit: one commit, never amended; its design: one amendment.
- The component or slice document is where change lands. Thermite's stage-3 document: nine commits across its build. Peritus's slice documents: one or two commits each, the second being the delivery record. OpenClaudia's slices: a delivery section appended at implementation.
- Each document governs one thing and is cited by its own name. Size is not the variable: Peritus's authority document is 70 KB and Thermite's mutation-scoring document 64 KB. ferrotorch keeps 465 documents, one per translation unit, under per-crate folders, plus ten phase documents and a swarm work breakdown of thirty independent units.

**Decisions are written in the document at the question.** "(resolved) Decision:" paragraphs and a Q-register table with decide-by milestones (Thermite); a Decisions section with numbered entries (crosslink); Non-negotiable contracts and an Architecture verdict (Peritus); an Architecture decision section (OpenClaudia). crucible, the newest repository with visible process, keeps architecture decision records as numbered files with status, owners, context and evidence, decision, preserved invariants, alternatives, impact, verification and consequences; accepted records are immutable and superseded by new ones.

**Versions are derived, not declared.** The file's history is the amendment history; Thermite's RFC process says a version is cited as the commit count on the file plus the commit, and that an RFC pull request should not be squash-merged or its review history collapses.

**Drift is pinned to code.** Thermite's routed documents carry a content digest over their governed files; CI fails when the code moves under the document. Peritus does not hash design Markdown; it keeps documents honest by the freeze commit and review, and governs code by the architecture policy file.

### flow control: write one, build one

The observed pattern is neither one document for everything nor all slice documents up front.

- **Thermite** committed the program umbrella with stage 1 marked final and stages 2 and 3 marked provisional, in those words. Stage 2's questions were resolved against the spike results that had just merged; stage 3's "resolved design doc" landed and its first requirement landed in code the next day; stage 4's document was first committed the day its build began. Design to first implementation: zero to ten days.
- **Peritus** committed the umbrella on day one and stopped touching it after nine days; slice documents followed one at a time in dependency order, each with one or two commits, each immediately built; one slice went from freeze to merged implementation in two hours; the whole catalogue in seven days.
- **OpenClaudia** went audit, remediation design, 108 numbered slices with dependency headers, dispatched in waves named by slice and wave.
- **crosslink** is the simplest form: one feature document, one gap analysis, one kickoff; median design-to-implementation the same day.
- The whitepaper says "a formal specification document for each unit of work."

The kickoff plan is the instrument between the stage document and the increments (Thermite): it does not restate requirements; it sequences them into committable increments with primary files and self-verify commands, states the per-increment gauntlet, lists what is out of scope, dates its groundings, and ends with a done-when checklist and a "context already landed, do not redo" section.

### idea cycles without rfcs

Peritus keeps thirteen topic-named design documents beside its twenty-five lettered slice documents (benchmark failure remediation, external benchmark qualification, proactive bug discovery, local working memory, plain-folder workspaces, and others). Each is its own document under the same template, references the umbrella rather than amending it, and hands its requirements to the owning slice. OpenClaudia does the same through findings in the audit that become workstreams and then slices. This is what crosslink's design flow is built for: one idea, one document, one kickoff. Thermite's RFC files are the language-project form of the same thing.

### this estate's divergences

Recorded as findings for the Slice 2 design and the primer rewrite, not as decisions.

1. **The constitution absorbs proposals.** Each idea cycle amended the single contract in place through the owned amendment route. There is no proposal or idea document, so rejected reasoning lives in tracker comments.
2. **The issue became the document.** The contract's rule, "provenance lives here, not in the sentences; the record carries the narrative", chose the tracker comment as the home of decisions. Thermite's RFC-5 diagnoses the failure exactly: an issue is a report, "amendments become comments, so the document a reader sees first is the stalest version of it."
3. **The integration layer was removed.** The 2026-08-02 unification retired the slice designs to knowledge pages, leaving rulings with nowhere to land but the tracker. The 39-item reconciliation ledger of 2026-10-08 is the measured result.
4. **Three unresolvable reference systems** (heading-name prose, tracker handles, knowledge-page names) instead of stable identifiers backed by a registry that CI checks. The rule against invented labels was satisfied by prose citations, which is the same accumulation in another form.
5. **Closure without evidence binding.** Criteria are prose; status is a sentence ("CLOSED (boundary sha)"); no typed evidence per requirement; "closure claims without their oracles" (vsdd-cli#881) is the predictable failure.
6. **Designs retired at ratification** instead of amended during the build and pinned to the code. Both retired slice designs predate every October decision; the build-plan calls each phase "its projection".

Measured context: since Slice 1 merged on 2026-08-02, 30 commits landed on main, none of type feature; 452 production lines changed against 1,326 governed-text lines; five contract amendments reached the revision history while dozens of rulings waited as comments. Of those weeks, six were a break; the three active weeks were re-entry.

### what adopting the practice looks like here (candidates)

- The contract becomes the thesis and umbrella, cited by section, frozen except through an owned amendment that a slice or idea document forces.
- Each open slice keeps one live design document in `.design/`, written at build time against a named revision, with decisions at the question, amended during its build, pinned to the code it governs.
- An idea cycle becomes its own design document through the design flow, never a contract amendment; its requirements land in a slice document or a requirements record with typed evidence.
- A requirements record with typed evidence replaces prose closure and generates the status view CI checks.
- Design pull requests are not squash-merged.
- The reconciliation ledger's items become inline resolutions in slice documents, not tracker comments.

