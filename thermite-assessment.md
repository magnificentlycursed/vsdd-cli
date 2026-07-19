---
title: "Thermite assessment: harness patterns and language maturity"
tags: ["respec", "upstream", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-07-19
updated: 2026-07-19
---


## Design Specification

### the harness (actor) — the part that transfers

Thermite's development process is the proven implementation of agent-first methodology enforcement, in the Claude Code + crosslink substrate:

- Four tool-restricted subagent roles in .claude/agents/: doc-author (no Edit on code), builder (pre-declared ~10-file manifest, absolute boundary), critic (no Edit, cannot approve — only "GENERATOR MUST FIX" / "NO DIVERGENCE FOUND"; must pin every divergence as a failing test), fixer (one minimal fix per pinned divergence). Human orchestrator holds all approval and all pushes.
- Action-time hooks: spec-discipline.py blocks edits to routed source until the session has Read goal.md + the governing design doc (route table: tooling/spec-routes.toml — a file with no route BLOCKS); anti-pattern-gate.py blocks todo!/unwrap!/panic! patches.
- doc-drift.py: design docs SHA-pin the files they govern; governed change without re-pin fails CI.
- No-self-oracle rule (R-CHAR-3): expected values come from hand-certified conformance corpus or golden files, never the toolchain's own output.
- Binary honest status: every requirement SHIPPED or NOT-STARTED with quoted evidence and a real non-test consumer; no deferral vocabulary.
- Greppable waivers: #[slag] marks every trusted-by-fiat escape; grep enumerates the faith list.
- Register control: .design/tone-and-voice.md + R-TONE-1 — named AI-prose tics (antithesis pairs, virtue adverbs, em-dash drama, rhetorical bold); rationale: agents anchor on the register they read, so corpus tics reproduce. Systematic per-crate tone passes in the commit history.
- Agent-facing language definition (THERMITE.skill.md) is GENERATED from the compiler's enums under a CI-gated 6,000-token budget — vocabulary cannot drift from source and cannot bloat.
- Crosslink is the tracker: agents file blockers via crosslink quick; milestones are the phases (no phase-as-deferral).
- Deliberate divergences from VSDD: no Red Gate phase (critic tests are post-hoc + mutation floors as anti-Goodhart), binary blocker/wont-fix finding taxonomy (no classification universe), mechanical stopping predicate instead of MVR convergence.

### the language — not usable for our software class yet (as of 2026-06-26)

Real and working: 7-crate workspace (~132K LOC Rust + 13K Lean), full pipeline (parse → spec cage → Verus/Kani/Lean lowering → translation validation with dual encoders), forge CLI complete (check/goal/fill/edit/build/audit/tv), 772-LOC proven text editor example, hand-certified conformance corpus, non-stub lowering_faithful Lean proof, zero TODO/unwrap/panic in source.

Blocking for adoption here: no dependency ecosystem (cannot host mdatron-class tools needing serde/jsonschema), frozen language subset, requires self-installed Verus + Z3 + Lean + Kani, seccomp runtime cage is Linux-only (macOS can verify but not enforce), L4 kernel-grounding mid-flight at HEAD, 196 lines of human-facing docs. Version 0.0.1 is accurate. Revisit on ecosystem + macOS maturity.

### standing conclusions for this project

Generalize the harness, not the language: the respec adopts route-table closed-world discipline, SHA doc-pinning, tool-restricted roles, no-self-oracle, binary status, greppable waivers, generated budget-gated context, and register control — as data-driven mdatron checks + crosslink-seam installation rather than hand-built per-project scripts. Thermite's token-economics thesis ("agents bear the token cost of proving correctness") has no measurement layer; the cost-knowability ledger is a natural upstream absorption proposal.

