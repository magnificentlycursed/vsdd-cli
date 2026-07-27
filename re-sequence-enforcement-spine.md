---
title: "Re-sequence the build to front-load the enforcement spine"
tags: ["design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-07-27
updated: 2026-07-27
---


## Design Specification

### Summary

A decomposition amendment (phase 1c, driven by a phase-1a/1b principle) to
the ratified contract `.design/agent-first-vsdd-toolkit.md`. It adopts
invariant-first ordering and re-sequences the build so the tool's
enforcement spine — the mechanisms that keep the methodology's own flow
honest — is present as early as its true dependencies allow, rather than
deferred to the back third (Layers 6–8). It is the response to the
enforcement-gap finding class (vsdd-cli #806 routing, #809
amendment-discipline) surfaced when Layer 3's phase-3 loop ran six rounds
fix-closing findings with no phase-4 routing — reproducing, during the
tool's own construction, the June-cycle orphaning the tool exists to
prevent.

### Requirements

- REQ-1: The decomposition preamble states **invariant-first ordering** — the cheapest enforceable form of a mission-critical process invariant is built, or carried by a bootstrap format-carry where its mechanized gate's true dependencies land downstream, as early as those dependencies allow — so the un-self-enforced trusted base (the layers built before the tool can enforce its own construction) is minimized. Scoped by the phrase "as early as true dependencies allow" so it governs packaging and bootstrap carry, never impossible orderings.
- REQ-2: The **unrouted-findings query** is assigned to Layer 2, alongside its four already-built tracker-integrity siblings in Layer 2's status integrity set (round-parity, unresolvable-handles-in-result-comments, findings-missing-owner-or-validator, closed-findings-missing-evidence), correcting the split that placed round-parity in Layer 2 while its sibling was packaged into the Layer 6 gate milestone by cohesion.
- REQ-3: The **Gates requirement's bootstrap section** carries a routing format-carry and an amendment-discipline format-carry beside the existing parity format-carry. The routing carry: a phase-3 round does not close, and no finding closes by fix, until every finding carries a routing disposition (a `plan` comment naming the target phase, or the fix lane); a `result` fix-close with no prior routing `plan` is malformed. The amendment-discipline carry: a commit changing the governing contract is malformed unless it cites a ratified spec-amendment review — its shape differs from the round carries (it keys on contract commits, not phase-3 rounds) and is flagged for the review to settle (see Open Questions).
- REQ-4: The **mechanized blocking tier** (directive classification, and the gates that block an unrouted fix-close or an unreviewed contract commit) stays at its true-dependency layer — it needs the hook seam and, for directive classification, the session skill and generated context — but the decomposition names it as the hardening of an already-detected-and-carried discipline and cross-references the format-carries and the Layer 2 detection query it mechanizes.
- REQ-5: The re-sequencing is **forward-only**: it mandates no retroactive re-routing of already-closed findings. The terminal-safety finding class is already re-routed via its own amendment (#807) and the reverted #806/#807 clauses get their own review; no further retroactive exercise is manufactured.
- REQ-6: The **criteria audit holds** after re-sequencing — every acceptance criterion still maps to exactly one closing milestone (or a declared split by named slice), and no layer is left unclaimed.

### Acceptance Criteria

- [ ] AC-1: The decomposition preamble contains the invariant-first principle with the "as early as true dependencies allow" balance clause. (REQ-1)
- [ ] AC-2: The Layer 2 decomposition entry names the unrouted-findings query among its integrity queries; the Layer 6 entry no longer lists it as one of that layer's query deliverables (it retains the mechanized blocking gate that consumes it). (REQ-2, REQ-4)
- [ ] AC-3: The Gates requirement's bootstrap paragraph contains three format-carries — parity (existing), routing, and amendment-discipline — each stating its malformed condition. (REQ-3)
- [ ] AC-4: The mechanized blocking tier's decomposition entries cross-reference the Layer 2 detection query and the format-carries they harden, so the split between detection (early) and blocking (downstream) is explicit and traceable. (REQ-4)
- [ ] AC-5: No clause in the amendment mandates retroactive re-routing of a closed finding; the forward-only scope is stated. (REQ-5)
- [ ] AC-6: The criteria-audit paragraph is updated to reflect the moved query and still shows every criterion with exactly one closing milestone and no layer unclaimed. (REQ-6)
- [ ] AC-7: The Revision line records this amendment with its issue reference and ratification date; register is clean (no coined labels, no new acronyms), verified by the Documentation Reviewer / Technical Writer lens.

### Architecture

The amendment edits three regions of `.design/agent-first-vsdd-toolkit.md`,
plus the Revision line:

- **Decomposition preamble** (the paragraph opening "Milestones are
  independently buildable…"): add the invariant-first principle (REQ-1).
- **Layer 2 entry** ("Layer 2 — Snapshot acquisition, phase answer, and
  corroboration"): add the unrouted-findings query to its process-integrity
  queries, joining the set already implemented in
  `vsdd-core/src/answer/integrity.rs` (`snapshot_integrity`, which today
  computes round-parity, unresolvable-handles, findings-missing-owner, and
  closed-findings-missing-evidence and surfaces them as status integrity
  findings that never degrade the answer). The query is detection, not
  blocking — it reads tracker state through crosslink's existing typed
  comments, so it carries no dependency on install (Layer 4) or composition
  (Layer 5). (REQ-2)
- **Gates requirement** (the bootstrap paragraph carrying the parity
  format-carry): add the routing and amendment-discipline format-carries
  (REQ-3), and adjust the Layer 6 entry so the unrouted-findings *query*
  moves to Layer 2 while Layer 6 retains the mechanized *gate* that blocks
  on it (REQ-4).
- **Criteria audit paragraph**: update for the moved query (REQ-6).

The detection tier lands in an already-built layer (Layer 2), so its
implementation is a Layer 2 amendment increment over existing code; the
mechanized blocking tier stays where its hook/session/composition
dependencies resolve. The bootstrap format-carries are contract text and
conventions, active on ratification, needing no code. This is the
pragmatic reading of invariant-first: pull detection and the carries early
(cheap, no over-reach), harden with blocking gates at their true layer.

### Out of Scope

- The terminal-safety Status-requirement amendment (#807) — a separate spec amendment on its own review track; it rides after or beside this one, not within it.
- The Layer-3 rebuild (the UCD-property swap and the machine-form sanitizer) — parked until both this amendment and #807 land.
- Retroactive re-routing of already-closed findings (REQ-5 forbids manufacturing it).
- Building the mechanized blocking tier itself — this amendment *places* it in the decomposition; its construction is the owning layer's own phase-2 work.
- Any edit to source code or to the contract itself in this cycle — this document is the review draft; the contract changes only on ratification, through the owning-domain composition and cold review.

