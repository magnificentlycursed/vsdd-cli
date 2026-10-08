---
title: "Reference practice: the other organization repositories (ferrotorch, crucible, Palimpsest, LNP, rookery-nest)"
tags: ["evidence", "reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---


## Design Specification

### status

Evidence supplement to the 2026-10-08 reference-practice reconstruction (index: `vsdd-in-practice-reference-repositories-2026-10-08`), from the orchestrating session's own read-only survey of the organization's remaining repositories. The four main repositories have their own `practice-report-*` pages. Survey measure per repository: last push, design-folder file count, agent definitions, published hub refs, skills, and the standing documents present.

### survey

| Repository | Last push | Design files | Agent definitions | Hub refs | Standing documents |
|---|---|---|---|---|---|
| ferrotorch | 2026-08-31 | 504 | 4 | 1 | goal statement, tooling |
| ferray | 2026-06-25 | 29 | 4 | 0 | goal statement, tooling, 18 deployed skills |
| Palimpsest | 2026-09-13 | 0 | 0 | 30 | one root design document, architecture folder, agents file |
| crucible | 2026-09-28 | 0 | 0 | 0 | spec chapters, decision records, roadmap, governance, pull-request template |
| LNP | 2026-09-29 | 0 | 0 | 0 | agents file, product-design document |
| rookery-nest | 2026-09-28 | 0 | 0 | 0 | agents file |
| fluid-matrix-multiply, prism-stack, hdn-linux, Thermite-Microkernel, ripsed | Jun–Oct 2026 | 0 | 0 | 0 | none visible |

### ferrotorch: the actor origin at scale

The translation-fork form of the method ("the vibe-fork ACToR machinery, proven on ferrotorch, ferrolearn and ferray", per Thermite's goal statement). 465 design documents under per-crate folders, one per translation unit, each with a header comment (tier, status, baseline upstream commit, upstream paths), a Summary, and requirements that cite the upstream file and line; ten phase documents at the top level (autograd engine, modules, optimizers, data loading, vision, GPU backend, distributed, JIT, parity) each in the feature-document shape with Resolved Questions; two gap analyses; and a swarm work breakdown of thirty independent units, each touching exactly one crate, with files, deliverables, tests and the upstream reference, launched with one kickoff per unit. The four agent definitions are the same doc-author, builder, critic and fixer as Thermite's. Tooling: an anti-pattern gate, a translate-discipline hook over a 123 KB route table, a cite-drift fixer, and a requirement-status injector. Cadence: more than a hundred commits a month in May and June 2026, then nine in July and August.

The locked goal statement is the rule book, with mechanically verifiable completion (three counts that must agree: routed translation units, parity operations verified, files carrying a requirement-status table) and eight speed disciplines, among them: batch by upstream file, not per operation; dispatch two to four builders with disjoint manifests in one message, critics in parallel, fixers serialized per blocker; symbol anchors in design citations, never line numbers, because line numbers "spawn cite-drift fixer dispatches every commit"; a critic only after substantive builds; new public API must have a consumer. Every routed file carries a requirement-status table in its top-of-file comment with two states only, shipped with quoted-code evidence or not started with a concrete blocker.

### crucible: decision records and spec chapters

The newest repository with visible process (last push 2026-09-28; a hundred commits in August, all by the maintainer, subjects such as "Verify YAML alias cycle rejection" and "Add verified Linux run pipeline"). Its documents: twelve numbered spec chapters (mission and principles; architecture and domain model; targets, execution and isolation; configuration; campaigns, oracles and bug model; engines; findings, replay and minimization; repair, verification and agents; scheduling, storage, CLI and reporting; phases, MVP and acceptance; runtime operational contracts; expansion and completion standard), 6 to 80 KB each; six architecture decision records from a template (status, date, decision owners, related issue, supersedes; context and evidence, decision, preserved invariants, alternatives considered, Verus and trusted-boundary impact, security and privacy impact, compatibility and migration, verification and acceptance, consequences and follow-up), with the rule that accepted records are immutable and superseded by new records that cross-link both; a roadmap organized by delivery groups with "how priorities are chosen"; governance, contributing, security and support documents; and a pull-request template with Summary, Motivation and evidence, Scope and compatibility, Verification (documentation checks, tests and known-defect fixtures, proofs reproduce, limitations recorded), trusted-computing-base impact, AI assistance ("identify material AI tools and what was independently checked"), security and privacy, and a checklist ("the change does not silently remove or downgrade a committed capability; original evidence remains reachable from derived artifacts; public behavior and documentation are updated together"). Contributing: "Do not present an unimplemented capability as available, and do not reduce the declared end-state scope merely to simplify an early phase."

### palimpsest: one root design plus an architecture folder

A book-writing harness (Rust core, TypeScript desktop). One 142 KB consolidated design document at the root, committed once with the core delivery on 2026-08-20 and not amended since; fifty architecture documents of 2 to 11 KB under `docs/architecture/` recording "dependency rules that Cargo alone cannot state", one concept each; golden corpora per phase with recorded inputs and expected compilations; the required quality gate stated in the contributing document as exact commands. Hub refs show a single agent identity and the v3 reconciliation layout; the hub event subjects are opaque (agent events, checkpoints, heartbeats), so the process is not reconstructable from subjects alone. Its prose-control content is on `reference-practice-prose-control`.

### lnp and rookery-nest: the agents-file form

The two newest repositories carry no design folder and no hub. Their process is one agents file each: mission in plain language; repository rules and boundaries ("Nest owns orchestration, VM lifecycle, persisted run state; rookpkg owns parsing, package construction, signatures"); required behavior ("Treat guest output as untrusted: validate protocol framing, identities, paths, sizes, archive structure"); the baseline validation commands; state and disk rules; and a closing honesty rule: "Never claim a package set, image, boot path, desktop, or hardware workflow is complete without the corresponding real build or runtime evidence. Report the last verified boundary and the remaining gap directly." LNP's points at a product-design document for product and safety decisions.

### what the survey adds to the main findings

- The method's heavier forms (agent types, route tables, registries, gates) appear where the project is large and agent-built; the newest small projects run on an agents file and CI alone. The form scales with the project, not with the date.
- Decisions as files exist outside the language project: crucible's architecture decision records are the general-software form of Thermite's RFCs.
- The per-unit design document and the symbol-anchor rule come from the translation-fork era and carried into Thermite unchanged.
- "Report the last verified boundary and the remaining gap directly" is the plain-language form of the three-evidence-class discipline seen in OpenClaudia and Peritus.

