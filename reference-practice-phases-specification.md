---
title: "Reference practice: the specification phases (1a, 1b, 1c)"
tags: ["design-input", "process", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---


## Design Specification

### status

Design input for the phase-primer rewrite. How the whitepaper's specification phases are actually practised in Thermite, Peritus, OpenClaudia and crosslink, reconstructed read-only on 2026-10-08 (index: `vsdd-in-practice-reference-repositories-2026-10-08`; evidence: the four `practice-report-*` pages). None of the repositories uses the phase names; the mapping is ours.

### 1a behavioral specification

**The artifact is a per-slice document.** Not a program-wide contract. Its attested shape:

- **Thermite:** `.design/<area>/<doc>.md` with an HTML header comment (tier, status, governs: the file set, thesis references by section, later an audited content digest), then Summary, Requirements (REQ-n), Acceptance criteria (AC-n, "mechanically checkable; tied to a conformance corpus entry or golden file where possible"), Architecture (symbol anchors, never line numbers), Verification, a requirement-status table, Open questions. For program work: an RFC, then an umbrella with a Q-register (defaults adopted, decide-by milestone), then a stage document per stage, then a kickoff plan that sequences requirements into committable increments.
- **Peritus:** the umbrella defines 52 stable-ID requirements in groups, 25 criteria, a traceability table (requirement group, owning slices, acceptance evidence) and a slice catalogue with "Owns / Depends on / Deliverable and completion evidence". Each slice document varies: a 5 KB freeze note (Outcome, Non-negotiable contracts, Crate boundary, Downstream API, Error and compatibility policy, Parallel ownership, Verification target) or a full feature document with numbered Requirements, Acceptance criteria, Verification with exact commands, Rollout, Open questions, and an Architecture verdict.
- **OpenClaudia:** every slice has Status, Effort, Primary findings, Workstreams, Depends on, Canonical sources, then Outcome, Implementation boundary, Acceptance, Handoff. Derived from an audit finding's "Required outcome" and a workstream's bullets.
- **crosslink:** the design skill's thirteen-heading skeleton; the maintainer's own documents use a shorter form with Requirements, Acceptance criteria, Architecture, Decisions or Open questions, Out of scope.

**Decisions are inside the document.** "(resolved) Decision:" under the question (Thermite), a Decisions section (crosslink), Non-negotiable contracts (Peritus), an Architecture decision section (OpenClaudia). One crosslink reversal was recorded only in the tracker, and the report flags it as the exception.

**Edge cases and non-functional requirements** appear as acceptance criteria, failure-handling sections, and adversarial test lists written into the spec before implementation (Peritus B2: "unknown dependencies, cycles, duplicates, wrong-spec tuple binding, one field of tuple drift at a time, stale observations, missing gates/categories/evidence, duplicate reviewers, shared identities/ancestry, blockers, invalid waivers, and required human approval"). Determinism, token budgets and resource budgets are stated as rules.

**Who writes it.** A doc-author agent with no edit tool, writing only under the design folder (Thermite), or the root agent (Peritus, OpenClaudia). Design-only issues say so: "Design only: do not implement, commit, push, install or launch agents" (Peritus). The audit-day practice in OpenClaudia produced the audit, the design and all 102 slice documents in one commit.

**Grounding.** Against the tree at a named revision ("grounded against the tree at c46da3ac"; "Revision: 35255a17"), with a kickoff gap analysis before dispatch.

### 1b verification architecture

**Not a separate document anywhere.** It is:

- the slice's Verification section with exact serialized commands (Peritus D2: the Verus command with its no-cheating flag and resource limit, the workspace task, the gate);
- the authority each requirement is checked against: a conformance corpus and golden files (Thermite, 63 programs, 12 golden certificates, 20 case oracles), with the rule that expected values are never copied from the system's own output;
- a per-package verification class (Peritus: verified, hybrid, trusted, ordinary, with allowed dependency directions in the architecture policy file);
- a registry entry per requirement with typed evidence (Thermite: file, symbol, test; Peritus: the obligations file with statement, owning crate, symbol, status, live issue, owner, evidence rows);
- Thermite's eleven documents under `.design/verified/` (the provable-properties catalogue and tool-selection record).

**Purity boundary.** Not drawn as a map; it is the product's own effect typing and determinism rules (Thermite), the functional-core and effect-shell protocol with verified reducers (Peritus), or absent.

**Amended on first contact.** Thermite amended a verification strategy the same day as the document ("verify emitted output, don't byte-match goldens").

### 1c specification review gate

**Not a gate anywhere.** No status flips; Thermite's component documents never leave draft. What exists instead:

- **The freeze commit and the architecture verdict** (Peritus): `design(d2): freeze production review engine boundary`; each design ends with a verdict ("ready for design review/phase scoping, not blanket implementation"). Self-reviewed by the root agent with the human.
- **The kickoff gap analysis and the design re-pass** (Thermite): stage 1 amended after a gap analysis; stage 2 flipped to kickoff-ready by a re-pass that resolved its open questions against merged spike results; stage 3 marked provisional until its re-pass. The critic later files divergences against documents after implementation starts (twelve blockers against one component document in two days).
- **Backlog integrity invariants** (OpenClaudia): every finding owned by exactly one slice; every workstream has a slice; dependency graph acyclic; every slice small or medium with explicit acceptance; a pre-wave audit of parallel lanes.
- **A pre-flight comment on the tracker** (crosslink, Aug–Sep 2026): the architect session writes the pre-flight; the human may waive it ("without further ceremonial approval").
- **Human RFC review in the issue thread** (Thermite): companion documents, a baseline-drift note, an agreed freeze, the decision to go pull-request based.
- **External cold reviews** are rare: one trust audit of Thermite at a named revision; an ontological review of Peritus after months of code; two outside-filed issues.

**Not observed:** a routine fresh-context adversary over every specification before tests; a multi-domain review; any Solution Owner or domain-reviewer role.

### divergences from the whitepaper in these phases

- 1a: no named edge-case catalogue or non-functional section; the design skill's failure-handling and security headings are the nearest.
- 1b: no provable-properties catalogue as a phase artifact outside Thermite; purity boundaries are product features, not maps.
- 1c: the whitepaper's adversary "can't find legitimate holes" exit is replaced by a freeze, a verdict, a gap analysis and backlog invariants. The heavy review is of code.

### what to take for the primers (candidates)

- 1a produces one slice document in the attested shape, written at build time against a named revision, decisions inline, an architecture verdict, frozen by its own commit.
- 1b is the verification section plus the authority per requirement and a registry entry with typed evidence; expected values never copied from the system under test.
- 1c is light: freeze, verdict, gap analysis against the tree, backlog integrity. A multi-domain cold review of a specification is this estate's own addition; if kept, the primer should say so and state its cost.

