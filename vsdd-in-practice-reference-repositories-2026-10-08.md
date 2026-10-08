---
title: "VSDD in practice across the methodology author's repositories (2026-10-08)"
tags: ["design-input", "review", "dispatch", "process", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---


## Design Specification

### Summary

Ten facts hold across all four repositories.

1. **The whitepaper's phase vocabulary is not used.** Zero occurrences of phase labels, "red gate", "roast", "Sarcasmotron" or "bead" in any maintainer commit, pull request, design document or hub comment. The native vocabulary is slice, increment, freeze, gate, pin, divergence, receipt, critic, fixer, root integrator, worker, independent final review, architecture verdict.
2. **No red gate for feature increments.** Tests, proofs and implementation land in one commit per increment in every repository. Failing-first is practised for bug fixes (a regression test that fails before the fix) and for critic pins (Thermite's failing divergence tests), and mutation testing is the mechanical substitute for "tests that would pass anyway".
3. **No multi-domain specification review.** Review of a design is self-review by the root agent with the human, a freeze commit, a kickoff-time gap analysis, or a pre-flight comment on the tracker. The adversarial energy is spent on the implementation tree, not the specification.
4. **The adversary is one reviewer, read-only, in a separate dispatch**, plus CI as the second reviewer. Peritus enrolls a distinct fresh reviewer identity per formally governed change, from a different model family than the workers. Thermite's critic may not fix, approve or give prose verdicts. OpenClaudia's reviewer is the orchestrator in a fresh context. crosslink's is a second agent session labelled architect or independent audit.
5. **Termination is mechanical, never judgment-based.** Zero blocking findings plus every gate green; registry counts; stage-gate checklists; a verified release policy. Public headlines flip only at gate time.
6. **Formal hardening is first, not fifth.** Peritus verifies proofs from the first slice under a no-cheating flag; Thermite gates CI on an axiom probe from the start; fuzzing, mutation and chaos arrive as later campaigns.
7. **No refactor phase.** Thermite's fixer forbids adjacent cleanup; Peritus enforces architecture as policy (line limits, forbidden module names, ownership, registered exceptions) on every commit; register-only passes run as gated slices.
8. **The human does not review on GitHub.** Zero reviews on 96, 91 and 12 sampled pull requests; one review on 193 in crosslink's former home. The human owns approvals, the push, the merge, secrets and redirections. The tracker, not GitHub, is the work ledger; GitHub is intake and CI.
9. **Three evidence classes are kept distinct in status wording** and never upgraded in prose: deterministic gates passed; independent review passed; verifier or formal receipt recorded. "No such receipt is fabricated."
10. **The design document is written just before its build, in the same window, and amended during it.** Design-to-first-implementation latency ran from two hours to ten days in every repository. Umbrellas freeze within days; slice documents absorb the amendments as dated paragraphs or appended delivery sections.

### status

**Design input for the phase-primer rewrite.** Read-only reconstruction of how the method is actually practised in the repositories of the organization that publishes the VSDD whitepaper (Corvidae-Coding-Projects), compiled from four per-repository reports (Thermite, Peritus, OpenClaudia, crosslink) produced by read-only research agents on 2026-10-08, plus the orchestrating session's own reads of ferrotorch, Palimpsest, crucible, LNP and rookery-nest. Nothing was executed in any checkout; hub refs were fetched read-only into scratch clones. The four reports (about 5,000 words each, every claim cited to a path, commit, pull request or hub issue) live in the session scratchpad under `agent-thermite/`, `agent-peritus/`, `agent-openclaudia/` and `agent-crosslink/`; this page is the cross-repository synthesis and carries the citations only at the level of repository and artifact.

**Exclusions.** mdatron was excluded as a reference (operator direction: it was designed under this estate's own contract). Everything this estate contributed to crosslink (upstream pull requests, the dispatch-dials design document, the hub identity and issues of 2026-10-08) was excluded from the crosslink evidence. Thermite's RFC process (RFCs as files) is noted but not generalized: it is a language-project convention and no other repository uses it.

**Caveat.** These are a solo maintainer's own projects, built mostly by agents, under time pressure; the more recent repositories are more likely to reflect current practice, but the operator's guidance is that recency is not proof of currency. The operator's own experience with the predecessor library (vsdd-suite, which worked when run by hand) is a fifth data point that the repositories do not record.

### the four repositories at a glance

| | Thermite (June–Aug 2026) | Peritus (Aug–Oct 2026) | OpenClaudia (Aug 2026) | crosslink (Dec 2025–Sep 2026) |
|---|---|---|---|---|
| Unit of specification | Umbrella program doc plus stage docs, each with a kickoff plan; 81 per-component contracts governing named file sets | Umbrella with 52 stable-ID requirements and 25 criteria; lettered slice docs frozen by their own commits | One audit, one remediation design, 108 numbered slice docs with Outcome, Implementation boundary, Acceptance, Handoff | One feature doc per feature; committed with or after the code |
| Requirements record | TOML registry, 524 requirements, typed evidence (file, symbol, test), generated status views, CI check | Traceability table, obligations file (157 proof obligations with owner and evidence), architecture policy file | Backlog index with integrity invariants; status lines per slice | Requirement and criterion IDs in docs, cited from commits; one criterion made executable |
| Roles | doc-author, builder, critic, fixer (Claude Code agent types with tool allowlists and file manifests) | root integrator agent, path-scoped workers (three concurrent), read-only reviewers of a different model family, human | one driver agent, waves of three code-only workers, orchestrator-as-reviewer, human | user or driver, kickoff implementer, architect or independent-audit session, human |
| Adversary form | failing divergence test plus blocker issue; verdict only "generator must fix" or "no divergence found" | typed findings with severity, blocking flag, disposition; detached content-addressed verdict with mandatory report | fresh-context pass with written changes-required or pass verdict, one re-review | architect redirect rounds and an independent audit comment, in-session |
| Convergence | registry 518 of 524 shipped; gauntlet; gates G1–G4 with checklists | Gate A locally, hosted, and again on fresh main; H4 release policy (25 criteria, 44 evidence requirements) never yet reached | administrative: one pull request with 191 commits; zero slices reached Verified | closing result comment plus merge |
| Design-to-build latency | 0–10 days | 0 hours to 1 day for slices; days for topic plans | all 102 slices authored in one day, implemented over 14 | median same day |
| GitHub reviews | 0 of 96 | 0 of 91 | 0 of 3 sampled, 191-commit merge | 0 of 12; 1 of 193 at the former home |

### per phase, across the repositories

**1a Behavioral specification.** The artifact is a per-slice document with stable-ID requirements, observable acceptance criteria, a verification section naming exact commands, and in Peritus a "Non-negotiable contracts" list, a "Parallel ownership" split, and an "Architecture verdict". Thermite's template adds a header comment (tier, status, governs, thesis references, audited content digest) and a requirements-status table. Edge cases appear as acceptance criteria, failure-handling sections and adversarial test lists written into the spec before implementation (Peritus B2: "unknown dependencies, cycles, duplicates, wrong-spec tuple binding, one field of tuple drift at a time, stale observations"). Decisions are written at the question: "(resolved) Decision:" paragraphs, a Q-register table with decide-by milestones, or a Decisions section. The producer is a doc-author agent (Thermite, with no edit tool, writing only under `.design/`) or the root agent; design-only issues say so explicitly ("Design only: do not implement, commit, push, install or launch agents").

**1b Verification architecture.** Not a separate document. It is the slice's verification section (exact commands), the authority each requirement is checked against (Thermite: the conformance corpus and golden files, with the rule that expected values are never copied from the system's own output), a per-package verification class (Peritus: verified, hybrid, trusted, ordinary, with allowed dependency directions enforced by a policy file), and a registry entry per requirement with typed evidence. The whitepaper's provable-properties catalogue exists in Thermite as eleven documents under `.design/verified/`.

**1c Specification review gate.** Not a gate anywhere. What exists: Peritus freezes a slice document by its own commit and ends each design with an architecture verdict; Thermite runs a kickoff gap analysis and a design re-pass before each stage opens, and its critic later pins divergences against the documents after implementation starts; OpenClaudia enforces backlog integrity invariants (every finding owned by exactly one slice, acyclic dependencies, every slice small or medium) and audits parallel lanes before each wave; crosslink's recent practice gates on a pre-flight comment that the human may waive. Component documents never leave draft status in Thermite.

**2a Test suite generation.** Not practised as a phase. Three substitutes carry the intent: regression tests that fail before a fix ("Every bug fix begins with a failing regression test at the narrowest meaningful boundary"); critic pins, which are failing tests committed with an ignore marker and a blocker issue, and which close only when the fix lands and the marker is removed; and mutation testing with a kill-ratio floor to catch tests that would pass anyway. Peritus also requires at least one negative executable test per proof invariant "that would fail if the guard disappeared".

**2b Implementation.** Complete, not minimal: "No stage is an MVP"; placeholders, stubs and unwraps outside tests are blocked by a pre-edit gate; a new public API needs a non-test consumer in the same commit (Thermite). The unit is one increment per tracker issue whose body is the dispatch prompt: the authoritative spec by requirement and criterion, the sequencing document, the merged predecessors to build on, the loop discipline ("read, write, verify, commit; commit per coherent sub-unit; revert anything that fails"), the self-verify commands, and a budget stop rule. Builders carry a pre-declared manifest of about ten files and must stop and ask when the work needs a file outside it. Workers in Peritus and OpenClaudia are path-scoped and, after an incident, code-only: the root agent runs every build and gate serially and owns the commit.

**2c Refactor.** Absent as a phase. Continuous architecture gates replace it (Peritus: soft 400 and hard 700 source lines, root module 80, forbidden names such as utils and helpers, owned exceptions with rationale). Register-only passes (Thermite's tone pass) run as gated base slices that everything else rebases onto.

**3 Adversarial refinement.** One reviewer per round, read-only, separately dispatched; CI is the second reviewer (23 of 25 follow-up commits on crosslink's three large pull requests were CI fixes, each announced with the run id). Findings are typed: identifier, severity, blocking flag, disposition (fixed, invalid, superseded), evidence, reproduction, and in Peritus a content-addressed verdict record with a mandatory review report ("an empty findings array is never the only retained evidence for a no-findings verdict"). Each finding becomes a tracker issue fixed in its own commit by the owning worker, then a bounded re-check by the same reviewer. Thermite's critic is forbidden to propose fixes; the whitepaper's adversary is required to. No persona theatrics, no negative prompting, no hallucination-based exit.

**4 Feedback integration.** Findings to issues to commits to closure, with a machine-generated changelog line committed unchanged. Specification-level feedback returns as a dated amendment paragraph appended to the slice document (Thermite, clustering on re-audit days: twelve on one day) or an amendment commit tied to the slice issue (Peritus, five in nine days, then frozen). The OpenClaudia audit was never reopened; feedback became new slices.

**5 Formal hardening.** Continuous and early. Verus proofs per slice from the first, verified under a no-cheating flag, with a trusted computing base that starts empty; a Lean spine with an axiom allowlist probed in CI; proof-coverage closure tracked as gap issues with evidence files. Fuzz, mutation and chaos as later campaigns with their own design document.

**6 Convergence.** Slice level: gate green locally, hosted on the pull request, and again on the merged commit; independent review with no blocking findings; signed merge; changelog; issue closed with a result comment. Program level: stage gates with checklists and pinned gate comments (Thermite), a verified release policy that has not yet been reached while releases ship labelled pre-qualification (Peritus), or an administrative merge with the backlog's own vocabulary saying nothing is verified (OpenClaudia). Headlines flip at gate time only.

### roles and dispatch shape

The common shape is a root session that writes or freezes the design, creates one tracker issue per increment with the dispatch prompt in its body, dispatches a worker into an isolated worktree, runs the gates itself, commits, and hands the push to the human. The variants:

- **Thermite:** four Claude Code agent types with tool allowlists, dispatched from the orchestrator session; a kickoff plan per stage sequences increments, and increments parallelize once the foundation lands; agents cannot push; the orchestrator arms a monitor on the pull-request checks and merges on green.
- **Peritus:** a root integrator agent (84 percent of 3,300 hub comments) with two or three path-scoped workers per slice chosen by a written ownership table, concurrency bounded to three, builds never concurrent, workers stopped before the first build; reviewers are read-only subagents of a different model family, one fresh identity per formally governed record.
- **OpenClaudia:** one driver agent implemented most slices alone; nine waves of three workers chosen by a pre-wave overlap audit; after a plugin hook launched a repository-wide lint inside a worker and interrupted all of them, workers became code-only.
- **crosslink:** one implementer per issue; a root architect session writes the pre-flight and runs the independent audit; the kickoff criteria and report loop that the tool ships was barely used on the tool itself; swarm was never used on itself.

Ceilings are explicit and recorded: three workers, serialized builds with job and thread limits, swap resets before dispatch, manifests of about ten files. Fan-in is one integration commit per wave or one pull request per increment.

### review conduct

What "adversarial" means in each repository differs in strength but agrees in shape: a separate dispatch, read-only, over the exact tree, returning typed findings, followed by scoped repair and a bounded re-check. Stop rules are written: "until clean" (Thermite), "no blocking findings and every gate passed" (Peritus), "review, corrections, one re-review, pass" (OpenClaudia), "independent audit PASS" (crosslink). Review dimensions are enumerated in the receipt. The reviewer is forbidden to edit. Genuinely cold reviews on record are rare and external: a trust audit of Thermite at a named revision, two outside-filed issues, an ontological review of Peritus after months of code.

Findings filing is uniform: a tracker issue per finding (Thermite titles them "Divergence: <symbol> <claim>"), a label (blocker, remediation), a result comment on closure, and a dedicated fix commit. Waiver policies exist in Peritus and OpenClaudia; no granted waiver was observed.

### records discipline

- **The hub is the work ledger; GitHub is intake and CI.** Comment kinds in use, by volume: result dominates everywhere (738 of 1,503 in crosslink; 956 of 3,300 in Peritus; 392 in OpenClaudia), then plan or note, then decision (767 in Peritus, 80 in crosslink, 28 in OpenClaudia, 6 in Thermite), handoff, observation, intervention. Interventions record blocked commands faithfully, including hook false positives repeated across agents until allow-listed.
- **Commit subjects are conventional** (type, scope, imperative) with the tracker id in parentheses; bodies carry a verification paragraph with integer counts or, in Thermite's June, a template of design sources, requirement status and verification. Commits are signed and the signature verification is itself logged in the hub. Verification detail migrated over time from commit bodies to pull-request bodies and result comments.
- **Pull-request bodies** carry Summary and Verification (exact commands, counts, run ids); Thermite's kickoff pull requests add "Delivered", "Adversarial verification" and "Gauntlet (local)"; crucible's template adds motivation and evidence, scope and compatibility, trusted-boundary impact, AI assistance, and a checklist.
- **Status lines are the honesty mechanism.** OpenClaudia's 108 slices use a census of distinct wordings that never upgrade deterministic verification into independent review or a verifier receipt. Peritus's obligations file says "Nothing in this file claims discharge until a registered independent reviewer approves."
- **Receipts are digests.** SHA-256 over sorted file manifests or diff ranges, invalidated by any later mutation, bound to the CI run on the exact head; failed attempts kept in the receipt.
- **Changelog lines are generated on issue close and committed unchanged**, or curated only at gate time with agents closing issues without changelog entries.
- **Knowledge pages** hold process lore (Thermite's kickoff-orchestration operations page: worktrees, merge-on-green, re-pin rules, registry union conflicts) and mirrors of program documents; the "every validated design becomes a knowledge page" step ran for two of five crosslink designs.

### gates and tooling

Recurring mechanical gates, by repository:

- **Issue-before-edit** (all): the pre-tool hook blocks edits and commands without an active tracker issue; mutating version-control commands are denied to agents.
- **Read-before-edit** (Thermite, ferrotorch): a route table maps every governed source file to its design document and reference; an edit blocks until the document exists and was read this session.
- **Anti-pattern gate** (Thermite, ferrotorch): stubs, unwraps, panics, root-level lint suppressions and shared-mutable wrappers blocked before the edit, with a per-item override that must carry a reason and a tracker comment.
- **Document drift** (Thermite): every routed document pins a content digest over its governed files; CI fails when the code moves under the document; clearing the gate is a conscious re-pin or an amendment.
- **Requirements registry check** (Thermite): stale generated views fail CI; shipped requires file, symbol or test evidence; blocked requires an open blocker.
- **Architecture as policy** (Peritus): layer dependency rules, verification classes, owner slices, line limits, forbidden names, owned exceptions, enforced by a workspace task before every signed commit and again in the hosted gate.
- **Reproducibility policy** (Peritus): canonical workflow copies, the ruleset template, the task runner file, dependency policy, pins and timeouts compared byte for byte in CI.
- **Review ledger** (Peritus): an actor registry with provenance, change records with raw-byte fingerprints, detached signed verdicts, and a two-step authorize-then-apply rule so a source edit cannot approve its own review record.
- **Control plane** (Thermite): a gate that checks the hook wiring itself, built after both agent-facing gates were dormant for five weeks following a tool re-initialization that rewrote the settings file, "while README, goal.md and all four agent files kept asserting they fire." The rule drawn from it: anything load-bearing for a trust claim must be CI-enforced and harness-agnostic; authoring-time tooling may never be cited as the reason a property holds.
- **Generated agent-facing spec with a token budget** (Thermite): the language definition is generated from the same registries the implementation consumes and held to 6,000 tokens in CI.

### cadence

Design and build happen in the same window. Thermite: 677 of 698 commits in June 2026, half of them touching the design folder; stage docs committed zero to ten days before their first implementation pull request and amended during the build. Peritus: umbrella on day one, frozen after nine days; slice documents one to two days each in dependency order, each immediately built, the whole catalogue in seven days; afterwards fixes outnumber features three to one. OpenClaudia: audit, design and all slices in one day, implementation in fourteen. crosslink: design documents committed the same day as the code, amended as status notes.

### how design documents are used, and this estate's divergences

The reference shape is one stable umbrella (thesis or program document, frozen within days, cited by section) plus many per-slice or per-component documents that govern a named file set, carry their own decisions, are written just before their build, and are amended during it. Umbrellas carry the staging order and a traceability table, not the slices' requirements. The whitepaper itself says "a formal specification document for each unit of work."

Measured against that, this estate's practice diverges in six ways (recorded here as findings for the primer rewrite and the Slice 2 design, not as decisions):

1. The constitution absorbs proposals: idea cycles amend the single contract in place; there is no proposal or idea document, so rejected reasoning lives in tracker comments.
2. The issue became the document: rulings are recorded as tracker comments by rule ("provenance lives here, not in the sentences"), so the document a reader sees first is the stalest version. Thermite's own RFC-5 diagnoses exactly this failure and chose files.
3. The integration layer was removed: the 2026-08-02 unification retired the slice designs to knowledge pages, leaving rulings with nowhere to land but the tracker; the 39-item reconciliation ledger of 2026-10-08 is the result.
4. Three unresolvable reference systems: heading-name prose, tracker handles and knowledge-page names, instead of stable IDs backed by a registry that CI checks.
5. Closure without evidence binding: criteria are prose and status is a sentence; no typed evidence per requirement.
6. Designs retired at ratification instead of amended during the build and pinned to the code.

Size is not the divergence: Peritus's authority document is 70 KB and Thermite's mutation-scoring document 64 KB. What differs is that each governs one thing.

### prose control in the reference repositories

**Thermite** binds a 3.5 KB register standard by one rule in its goal statement and carries it into the doc-author and builder agent definitions: affirmative not defensive; plain not emphatic (no capitals for emphasis, no intensifiers; "exactly" judged claim by claim and kept where it states an if-and-only-if); narrative only in introductions and conclusions; the named tics are the antithesis pair, the virtue adverb, dash drama, rhetorical bold and the cute aside; "a register change, not a content change." The mechanism statement: "residual emphasis is what a downstream agent anchors on and drifts toward." The comment pass ran as a gated base slice, one agent per crate paired with an adversarial verifier whose only job was to confirm the diff is comments-only, scoped by tic counts treated as upper bounds.

**Palimpsest** (a book-writing harness, September 2026) encodes the same problem at corpus scale. Its philosophy: separate creation from judgment; read freshly with isolated reader agents that know only what the text has taught them; revise large before small; preserve disagreement rather than a single score; protect intentional weirdness; treat revision like refactoring. Its mechanisms: intent tags with stated effects that reviewers test rather than normalize; tickets that keep diagnosis separate from prescription, with a required outcome and a rewrite radius, resolved only by an independent verifier who repeats the required outcome "so changed words cannot masquerade as a solved reader problem"; voice as language decisions with no catchphrase field; a failure catalogue naming accumulated default voice; a lint platform with versioned inheriting profiles, four policies (forbidden, report, watch as a density per thousand words, intentional as a protected observation with a required reason that stays visible), phrase families bound to a content hash, findings keyed to stable component IDs, and analyzer failure as an error, never a clean bill; a texture auditor that flags low syntactic variance, semantic reiteration and paragraph-length regularity "without pretending the metric itself knows good prose."

**Cross-domain mapping to this estate's governed corpus** (primers, skills, designs, documentation). Already present: the no-coinage and concrete-referent rules, the vocabulary registry, action-time activation (Palimpsest's "exposition through need"), cold review, the operator authoring the oracle, "stated once" redundancy. Candidates the references add: a reader contract per primer and skill (who reads it, at what phase, what it must leave them able to do); intent tags with intended effects on deliberate choices, so review tests the effect instead of sanding it off; a canon level on every governed document and section, so a reader knows which layer governs; a promise record for every "pending", "owed" and "lands in Slice N" sentence, checked for a payoff; a check that every reference in a primer resolves inside the dispatched slice; fresh-reader calibration as the oracle for primers (fresh sessions, one fixture, did the required acts happen); required outcome and rewrite radius on prose findings, closed by re-testing the outcome; impact reports before an amendment; watch-density and intentional policies as raises to the conformance engine's register family; a corpus-level density view of the tics. What does not transfer: the fiction machinery. What transfers only partly: confidence as a field separate from severity; and the goodharted-prose warning, which already applies to the way the label ban was satisfied by prose citations.

### divergences from the whitepaper, consistent across repositories

1. No red gate for feature increments; failing-first for fixes and pins only; mutation testing as the substitute.
2. No specification review by an adversary before tests; review targets the implementation tree.
3. The adversary is typed and read-only; Thermite's may not propose fixes; no persona, no negative prompting, no judgment-based exit.
4. Convergence is mechanical, and the headline flips at gate time.
5. Formal hardening is continuous and early.
6. No refactor phase; architecture policy and register slices instead.
7. One tracker issue per increment or campaign, with proofs tracked in a registry file, not sub-issues per spec item.
8. The human approves, pushes, merges and redirects; the human does not review on GitHub.
9. Design documents are committed with or just before the code and amended during the build; the whitepaper's return-to-phase-1 loop shows up as amendments and new slices, not re-review.
10. Multi-domain rosters, Solution Owner and domain-reviewer roles exist in none of the repositories; roles are doc-author, builder, critic, fixer, root integrator, worker, reviewer, human.

### implications for the phase-primer rewrite

Recorded as candidates for the rewrite's own design, not as decisions.

- **Vocabulary.** Ground the primers in the native vocabulary where it is attested (slice, increment, freeze, gate, pin, divergence, receipt, critic, fixer, root integrator, worker, independent review, architecture verdict) and keep the whitepaper's phase names as the index, not the register. Note the three senses of "pin" and that "gauntlet" means the mechanical check set.
- **1a** produces one slice document in the attested shape, written at build time against a named revision, with decisions inline and an architecture verdict, frozen by its own commit; not a program-wide contract.
- **1b** is the slice's verification section plus the authority per requirement and a registry entry with typed evidence; the rule that expected values are never copied from the system's own output.
- **1c** is light: freeze, verdict, gap analysis against the tree, backlog integrity; the heavy adversarial budget goes to phase 3. A multi-domain cold review of a specification is this estate's own addition and should be labelled as such in the primer, with its cost stated.
- **2a** splits: red-first is mandatory and enforceable for fixes and pins; for new increments the attested substitute is negative tests per invariant plus a mutation floor, with tests and code landing together.
- **2b** is complete, not minimal, under anti-stub gates, a manifest, a consumer rule for new public API, and an issue body that is the dispatch prompt with predecessors, loop discipline, self-verify commands and a stop rule.
- **2c** becomes continuous architecture policy plus explicit register slices, or is dropped as a phase.
- **3** is one read-only reviewer per round with typed findings, a blocking flag and dispositions, findings filed as issues and fixed in their own commits, a bounded re-check, CI as the second reviewer, and a written stop rule; a different model family where available; review dimensions enumerated in the receipt.
- **4** routes findings to issues and amendments to the slice document as dated paragraphs; the umbrella is not reopened.
- **5** is continuous from the first slice, not a late phase.
- **6** is a mechanical checklist with the headline flipping at gate time, three evidence classes kept distinct in every status line, and "no implicit success."
- **Every primer** carries a reader contract, intent tags on its deliberate choices, a canon level, and conforms to a register standard bound by one rule and carried into every dispatched role; a primer is tested by fresh-reader calibration.

### open questions the record does not settle

- Whether the critic and reviewer dispatches ran in fresh contexts for ordinary (non-ledgered) reviews; only Peritus's formal records retain a report.
- Which models ran the roles over time; Thermite's agent files and front matter disagree.
- Thermite's hub after 2026-06-18 and OpenClaudia's pre-migration hub are unpublished.
- Whether any retrospective verifier run was ever applied to an OpenClaudia slice.
- Whether the Peritus review-shaped design documents constitute a specification review by a second party; they were not read.
- Who the "Director" and "root architect" sessions are in process terms.

### sources

The four agent reports in the session scratchpad (practice-thermite.md, practice-peritus.md, practice-openclaudia.md, practice-crosslink.md) with their bare hub clones; the orchestrating session's reads of Thermite (`goal.md`, the program and stage documents, the kickoff plan, the tone documents, the agent definitions, the gate tooling, the RFC branch), crosslink's design workflow guide and skill, Peritus's architecture file and slice documents, ferrotorch's goal statement and design tree, Palimpsest's design document and literary-lint architecture, crucible's decision records and spec chapters, and the VSDD whitepaper. Repository states as of 2026-10-08.

