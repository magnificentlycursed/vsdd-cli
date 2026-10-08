# Feature: the one owned contract amendment of October 2026

Status: proposed, 2026-10-08; the interview's choices and the open questions were resolved by the operator the same day and are folded into the text below. Record: the tracker issue "Amend the contract in one owned cycle, the last before the build window" (vsdd-cli#897). Decisions it carries: decisions 1, 3, 4, 6 and 9, and the vocabulary ruling of the same day, on the knowledge page `big-picture-and-decisions-2026-10-08`, all accepted by the operator on 2026-10-08 and recorded on vsdd-cli#839. Composition: Solution Owner primary with the core always-on quartet, skill-interactive, in-session and hand-audited under the contract's bootstrap interim; the review shape is one cold reviewer plus CI, per decision 5. The exit gate is the operator's approval comment on vsdd-cli#897 before the pull request merges.

## Summary

One amendment cycle changes the contract (`.design/agent-first-vsdd-toolkit.md`), the build-plan (`.design/build-plan.md`), the mdatron configuration under `.mdatron/`, the versioned data under `templates/registry/`, the exception register, and the governed prose corpus, in one pull request, so that:

1. The design folder holds the contract as the frozen umbrella, the build-plan as the program projection, one live design document per open milestone, and proposal records under `.design/proposals/`. The first live milestone document is the composition milestone's, restored from its retired knowledge page and the decisions recorded since.
2. Decisions about a milestone are written at the question inside that milestone's document, with the tracker comment as the log entry that points at the section. The contract's Revision history keeps its role for contract changes.
3. Every binding of a methodology act to a crosslink or Claude Code specific (the session-start hook's home, kickoff's container mode, the trace channel, the swarm residue) leaves the contract's normative text for the paved-path map and the runtime rules file.
4. The member "Verifiable conformance and efficiency" keeps its text as the statement of intent; its build legs that have no mechanical check today move to a second program named in the Decomposition; this program keeps the legs that have one.
5. The evidence wording follows the quality-engineering sense of "oracle": the operator authors the oracle; what the verifier reads is the synced trace, named as evidence; what a review or CI produces is a verdict record in the shape Peritus implements.
6. Four vocabulary families are renamed across the governed corpus in one change: phase primer to phase skill, supplement to rules file, domain prompt to reviewer role, and the numbered slices to milestones named by feature. The old words become deprecated aliases that the vocabulary check rejects in prose.
7. The workspace sentence schedules the crate collapse as the composition milestone's first scaffold commit.
8. Design pull requests merge with a merge commit, so each design file's history is its version record.
9. The 39 items on the reconciliation ledger are routed: the composition milestone's rows as inline resolutions in its document, the rest as one routed comment per owning issue.

The cycle is the last amendment before the composition milestone's build, under the work-in-progress limit of decision 2.

## User-visible behavior

What a maintainer or a dispatched agent sees after the merge:

- `.design/` lists four kinds of file: the contract; the build-plan; `composition-milestone.md`, a live document amended as the composition milestone's code lands; and `proposals/`, holding this document and any later proposal, each with a status line (proposed, accepted, superseded) and never amended after acceptance except for that line.
- The contract no longer names `kickoff run --container`, the `.crosslink/` folder, the session-start hook's installation path, or the trace sync channel as the vehicle of any act. It names the act and says "its vehicle is the paved-path map's binding". The map (`templates/registry/act-to-affordance-map.md`) and the runtime rules file (`supplements/claude-code-cli.md` at its current path) carry the specifics, including the statement that the session-start hook is this repository's own Claude Code entry, absent in kickoff worktrees by design.
- A reader who asks "what does the composition milestone build, and what was decided about it" opens one file and finds the design, the two increments, the decisions of 2026-09-28, 2026-10-01, 2026-10-02 and 2026-10-06 at the questions they answered, and the nineteen reconciliation rows resolved inline, each naming its record.
- `mdatron verify` rejects "primer", "supplement", "domain prompt" and "Slice N" in governed prose, and accepts them inside code spans (quoted titles, file names, historical identifiers).
- Tracker milestone titles keep "Slice N —" because the tracker has no rename command; the contract's References section maps each milestone name to its tracker milestone.
- `vsdd gate --ci` passes with the register entry hand-authored-build-plan resolved (the build-plan is hand-authored by design from this amendment on) and the two conformance-leg entries standing under the second program's owning issue.

## Requirements

Each requirement is testable; its acceptance evidence is named in Acceptance criteria.

- **REQ-1 Design-folder rule.** The contract's Solution Owner change authority member states the design folder's composition: the contract (umbrella, frozen except through an owned amendment), the build-plan (projection), one live document per open milestone, and proposal records under `.design/proposals/`. The route table claims each kind; an unrouted file in the folder blocks, as today.
- **REQ-2 Milestone document form.** A milestone document has, in order: a title naming the milestone by feature; a status line (live or retired, the tracker milestone handle, the design issue handle); Summary; Requirements; Acceptance criteria; Design; Increments (each with its scaffold commit contents); Decisions (dated entries, each with the question it answers, the ruling, and the record handle); Open questions; Out of scope; and a final `Provenance:` line naming the Decomposition bullet it projects, in the build-plan's marker form. The form is a section rule and a marker rule in the route table, so a malformed document blocks.
- **REQ-3 The composition milestone document.** `.design/composition-milestone.md` exists, restored from the knowledge page `composition-slice` (its requirements, criteria and architecture), carrying every decision recorded on vsdd-cli#839 and vsdd-cli#881 since ratification as a dated entry, the two increments of decision 2 (first the composition function with its config loader and the state artifact, starting from the scaffold commit that performs the crate collapse; second the generator with its three outputs and the install members), and the nineteen composition rows of the reconciliation ledger as inline resolutions, each naming the ledger page and the row's title.
- **REQ-4 Decisions at the question.** The contract states that a decision about a milestone is written in that milestone's document at the question it answers, with the tracker comment pointing at the section; a contract change keeps the Revision history row. The Revision history preamble changes from "Provenance lives here, not in the sentences" to a sentence that names both homes. The handle grammar sentence permits handles in milestone documents' Decisions entries and status lines, in addition to Evidence lines and the Revision history.
- **REQ-5 Vehicle bindings in data.** No member, requirement, criterion or Decomposition bullet of the contract names a crosslink command, flag, folder or hook installation path as the vehicle of an act. Each such passage names the act and defers to the paved-path map. The map carries the vehicle for each act, including: autonomous execution on container kickoff; phase-3 rounds as one dispatch per reviewer; the session-start hook as this repository's own Claude Code `SessionStart` entry, installed by `vsdd init` into the Claude Code settings, absent in kickoff worktrees by design; the conformance evidence channel as the verdict record of REQ-7. The runtime rules file states what the hook injects. Evidence lines and the Revision history may keep their historical specifics.
- **REQ-6 The conformance member's build scope.** The member keeps its text as intent. Its legs are partitioned in the Decomposition: built in this program (the installed-artifact integrity check; the negative-case fixture pairs of the control-effectiveness registry; the dispatch manifest's recorded phase, model and effort dials; the intended-tools check over the map; the anonymization pre-commit hook) and deferred to a second program (the trace-based verifier and the expected-versus-observed check; the skill-invocation audit; the operator-session audit; the spend-shape report and its runtime leg; the persona and rules-file conformance checks that read a trace). The second program is a named boundary in the Decomposition with no design; its umbrella is written when the composition, install, gate-execution and finding-lifecycle milestones have shipped. The criterion "Verifiable conformance" is re-scoped to the legs built in this program, and the gate-execution, recorded-dispatch and cost bullets lose the deferred legs. The register entries synced-trace-oracle-unbuilt and native-spawn-interceptor-unbuilt are re-scoped to the second program and re-owned to its standing issue, which is opened at this amendment's ratification with no design attached (decided 2026-10-08): it owns the two entries and collects the deferred legs' records until the umbrella is written.
- **REQ-7 Evidence wording.** The bullet "The oracle is the trace, synced and tamper-evident" is replaced. The new bullet states: the operator authors the oracle (the expected results); the verifier's evidence is the trace as recorded by the dispatcher on the operator's side, never the agent-writable local copy; the verdict a review or CI produces is a verdict record, content-addressed, that must already exist unchanged on the protected branch before a transition counts, with the authorizing record and the applying change as two pull requests so a source edit cannot approve its own review; no transcript sync is required. "Oracle" appears in the contract only in that first sense; the register and the pages are not touched for history.
- **REQ-8 Vocabulary renames in one change.** The mapping in Data and compatibility is applied across every file in vocabulary scope, the schema class values, the schema files, the data sets, the route table's structure rules, the register's trigger pattern, and the code constants that read the data values, in one pull request. The old words are added as deprecated aliases in the vocabulary registry and as anti-patterns in `.mdatron/vocabulary.yaml`. Code spans are exempt. Tracker titles are not renamed.
- **REQ-9 Workspace sentence.** The Architecture section states: one crate is the target; the collapse of `vsdd-core` into the `vsdd` binary crate is the composition milestone's first scaffold commit; mdatron's core-library seam is mdatron's own decision.
- **REQ-10 Merge method.** The Per-milestone PR discipline member records that design pull requests merge with a merge commit, never squashed or rebased, so each design file's history keeps the author's commits, the reviewer's commits and the ratification merge; the operator sets the repository setting before this cycle merges (decided 2026-10-08). Build pull requests keep their current method.
- **REQ-11 Ledger routing.** Every ledger row outside the composition milestone receives one routed comment on its owning issue, citing the ledger page and the row's title and stating its disposition (resolved by this amendment, routed to a milestone document when that milestone opens, or standing with an owner). The mapping is in the appendix.
- **REQ-12 Records.** The knowledge page `composition-slice` gains a superseded-by line naming the live document; the conventions page's section 4 is cited by the milestone document form; `vsdd-cli#897` carries the operator's approval comment.

Evidence: the reconciliation on 2026-10-08 (`reconciliation-ledger-2026-10-08`); the retrospective (`append-accumulation-retrospective-2026-10-08`); the live tests of 2026-10-02 (vsdd-cli#890: a project cannot ship hook scripts into crosslink's payload; crosslink's init replaces a project's hook entries); the reference practice (`reference-practice-design-documents-and-estate-divergences`); the verification papers and Peritus's review ledger (`cross-reference-verification-papers-2026-10-08`).

## Acceptance criteria

- **AC-1 Mechanical checks green.** On the pull request: `mdatron verify` clean; `vsdd gate --ci` pass; `cargo test` green; the routing-gate, verify and test checks green.
- **AC-2 No old words in prose.** This search returns nothing in vocabulary scope outside code spans and the contract's Revision history and Evidence lines: `grep -r -n -i -E "\b(phase )?primers?\b|\bsupplements?\b|\bdomain[ -]prompts?\b|\bSlice [1-7]\b" .design .claude/commands supplements templates/registry`, read against code spans by the vocabulary check itself (the check is the oracle; the search is the author's pre-flight).
- **AC-3 No vehicle specifics in normative text.** This search over the contract returns only Evidence lines and Revision history rows: `grep -n -E "kickoff run --container|--container|\.crosslink/|session-start hook|swarm" .design/agent-first-vsdd-toolkit.md`.
- **AC-4 The milestone document passes its form.** The route table's section and marker rules for `.design/composition-milestone.md` pass; each of the nineteen composition row titles from the ledger appears in the document; each dated decision from vsdd-cli#839 and vsdd-cli#881 since ratification appears with its handle.
- **AC-5 The pin is re-pinned in the same change.** `.mdatron/pins.yaml` carries the new Decomposition hash and its restored comments.
- **AC-6 Register consistent.** hand-authored-build-plan has status resolved with this cycle's decision reference; synced-trace-oracle-unbuilt and native-spawn-interceptor-unbuilt are standing, owned by the second program's issue, with their deviation text re-scoped.
- **AC-7 Every ledger row routed.** For each of the twenty rows outside the composition milestone, the owning issue carries a comment citing `reconciliation-ledger-2026-10-08` and the row's title.
- **AC-8 Fresh-reader check.** A cold session given the amended Decomposition and the composition document answers "what is the first increment and what is in its scaffold commit" by exact match to the document; the answer is recorded on vsdd-cli#897.
- **AC-9 Review and approval recorded.** One cold reviewer's verdict comment (typed findings, pass or changes-required) and the operator's approval comment exist on vsdd-cli#897 before the merge; the merge is a merge commit.
- **AC-10 Adopter payload consistent.** The install-slice red-gate test reflects the renamed schema files, and the installed-artifact manifest's version is bumped with the change.

## Current architecture

- `.design/` holds the contract (110 KB, frontmatter-less), the build-plan (26 KB) and the build-plan's pipeline sidecar. The 2026-08-02 unification (vsdd-cli#860) retired the per-slice designs to knowledge pages (`composition-slice`, 24 KB; `gate-execution-slice`, 36 KB; `install-slice`).
- mdatron governs `.design/*.md` through `.mdatron/config.yaml` (file and vocabulary globs), `.mdatron/routes.yaml` (closed-world allowlist; the build-plan's section rules key on `Slice \d+`; its marker rule ties `Provenance:` lines to the Decomposition's bold names), `.mdatron/pins.yaml` (a section pin over the Decomposition, re-pinned with `mdatron pin --update` in the same pull request), and `.mdatron/vocabulary.yaml` (anti-patterns for retired words; identifier schemes REQ-, AC-, Q). Schema classes in use: `phase-primer` (10 files), `domain-prompt` (18), `supplement` (14), the data sets.
- The data sets: `templates/registry/economics-data.md` (schema 0.2.0) carries the token-budget classes `session-skill`, `domain-prompt`, `phase-primer`, `supplement-section`, `always-on-core`; `templates/registry/vocabulary.yaml` (registry 0.4.0) carries the project terms with `deprecated_aliases`; `templates/registry/installed-artifact-manifest.md` (schema 0.3.2); `templates/registry/act-to-affordance-map.md` carries fourteen acts, the vehicle bindings and their conditions.
- Code: `vsdd-core/src/lib.rs` embeds the three schema files as `schemas::PHASE_PRIMER`, `schemas::DOMAIN_PROMPT` and `schemas::SUPPLEMENT`; `vsdd-core/src/init.rs` deploys them and the artifact sets `PHASE_PRIMERS`, `DOMAIN_PROMPTS`, `SUPPLEMENTS` to adopters; the tests under `vsdd-core/tests/` name them (the install-slice red-gate test counts 15 templates and 62 artifacts).
- The exception register has four standing entries, re-armed on 2026-10-08 to 2027-01-31 under vsdd-cli#897; hand-authored-build-plan's trigger greps the build-plan's Completed phases for `Slice 2`.
- Hooks wired today: five Claude Code hooks in `.claude/settings.json` call scripts under `.crosslink/integrations/hooks/`, plus a `SessionStart` entry. The contract places the session-start hook in crosslink's payload in three passages (the operator-session audit bullet, the Architecture sketch, the composition bullet of the Decomposition); the live tests of 2026-10-02 showed that placement cannot hold.
- The contract's commits since July are squash merges (one parent each), so the pull requests' review commits are not in the file's history.
- The contract's Revision history preamble: "Provenance lives here, not in the sentences." The handle grammar: "handles appear only in Evidence lines and the Revision history."

## Proposed design

### The design folder

The Solution Owner change authority member gains a bullet:

> The design folder holds four kinds of document. The contract: the umbrella, frozen except through an owned amendment. The build-plan: the program projection, hand-authored under this authority. One live document per open milestone, written when the milestone's design opens, amended as its code lands under the milestone's own design issue, and retired to a knowledge page under a recorded decision when the milestone closes. Proposal records under `.design/proposals/`: an idea or an amendment enters as a proposal with a status line, is accepted or superseded by a recorded decision, and is never otherwise amended after acceptance. A milestone's decisions are written in its document at the question they answer; the tracker comment is the log entry and points at the section.

The route table gains two routes: `.design/composition-milestone.md` (and later milestone documents, one route per file, each with the milestone form's section rules and the `Provenance:` marker rule) and `.design/proposals/*.md` (links checked, no section rules beyond a status line in the first paragraph). `.mdatron/config.yaml` adds `.design/proposals/*.md` to both globs.

### The milestone document form

Section rule for every milestone document (route table, `section_rules`): h2 headings in order Summary, Requirements, Acceptance criteria, Design, Increments, Decisions, Open questions, Out of scope; a `Provenance:` marker whose target is the Decomposition's bold name for the milestone. The Decisions section holds one h3 per decision, dated, in the form `### 2026-10-02: the generator's three outputs`, with the record handle in the entry's first line. The status line is the first paragraph.

### The composition milestone document

Restored by hand in this cycle from the retired page and the records, in the form above. Its Decisions section carries, in date order: the 2026-09-28 rulings (presets; the install-offer re-open and the six-direction fixtures); the 2026-10-01 ruling on the state artifact (vsdd-cli#881 item 4); the 2026-10-02 decisions on the generator's outputs, rules files, the hook as this repository's own entry, and the install count citations (vsdd-cli#839 comments 1577 and 1578; vsdd-cli#881 item 2); the 2026-10-06 note on the skill class; and the 2026-10-08 decisions on the two increments and the crate collapse. Its Increments section states the scaffold commit for each increment: types, failing tests, stubs and the document itself; the first scaffold performs the crate collapse. Each of the nineteen composition rows of the ledger becomes a sentence in the section it belongs to, in the form "Resolution (the ledger row `the session-start hook's home`): ...".

### Vehicle bindings

Each passage found by the AC-3 search is rewritten to name the act and defer: "Autonomous execution runs on the vehicle the paved-path map binds; the map's condition states the vehicle's posture." The map's entries for autonomous-execution, phase-3-review-round and session-binding carry the specifics now in the contract; a new entry `session-start-injection` binds the operator session's control to this repository's own Claude Code `SessionStart` hook, installed by `vsdd init` into the Claude Code settings, absent in kickoff worktrees by design, injecting the session skill, the runtime always-on rules file and the phase pointer. The runtime rules file's Activation section states the same in one paragraph. The swarm sentence in Conformance at action time ("No crosslink swarm command is ridden ...") moves to the map's condition on phase-3-review-round; the reserved-word note and the retired-term list keep their swarm mentions as vocabulary.

### The conformance member and the second program

The member's bullets stay. The Decomposition gains a paragraph after the cost milestone's bullet:

> **The second program.** The legs of Verifiable conformance and efficiency that read a trace have no mechanical check in this program: the conformance verifier's expected-versus-observed check, the skill-invocation audit, the operator-session audit, the spend-shape report and its runtime leg, and the persona and rules-file checks that read a trace. They are a second program with its own umbrella, written when the composition, install, gate-execution and finding-lifecycle milestones have shipped, and they are not a milestone of this one. Until then every verdict that would rest on them is could-not-check, and the exception register carries the two capability gaps under the second program's standing issue.

The gate-execution bullet keeps the control-effectiveness registry with its negative-case fixtures, the intended-tools check and the anonymization hook; the recorded-dispatch bullet keeps the manifests, signing, the dispatch preflight, the phase and dials, the reviewer roles, the session skill and the review stage, and loses the golden-path dispatcher's audits; the cost bullet keeps the report as the reader over dispatch manifests and the tracker, and loses the trace-sourced provenance. The criterion "Verifiable conformance" is re-scoped to: the negative-case fixture pairs of the registry entries built in this program, the manifest check, and the phase-and-dials preflight; the rest moves to the second program's umbrella.

### Evidence wording

The replaced bullet:

> **The operator authors the oracle; the verifier reads evidence it did not author; a review produces a verdict record.** The oracle is the expected result: acceptance criteria, reference answers, fixture expectations, and the composition function's expected set. The verifier's evidence is the trace as recorded by the dispatcher on the operator's side; the local copy on a worktree is agent-writable and is never evidence. A review's or CI's output is a verdict record: a content-addressed statement bound to the artifact digest it judged, which must already exist unchanged on the protected branch before a transition counts; the authorizing record and the applying change land as two pull requests so a source edit cannot approve its own review. No transcript sync channel is required. Until the dispatcher writes such records, every verdict that would rest on them is could-not-check, never clean.

The three later mentions of the sync channel (the secret-redaction ladder's trace leg; the Decomposition's note on the ladder; the register reference) are reworded to the dispatcher-side record.

### Vocabulary renames

Applied as a scripted rename over vocabulary scope, then read line by line. The mapping is in Data and compatibility. In the contract: the Decomposition's bold names become "**The live self-governance milestone — ...**" through "**The cost milestone — ...**"; "Slice obligations, stated once" becomes "Milestone obligations, stated once"; the walking-skeleton doctrine keeps "vertical slice" as the lexicon term with one sentence noting the milestone names; the References section gains a name map from milestone names to tracker milestones (#8 through #14) and registers the four families. In the build-plan: headings become `### Phase N: the <feature> milestone — ... (sequential)` and the Completed phases' bold names likewise; the structure rules re-key on `the ([a-z-]+) milestone`. The register trigger pattern becomes `the composition milestone`.

### Workspace sentence

> Workspace: one crate, the `vsdd` binary, is the target; `vsdd-core` collapses into it as the composition milestone's first scaffold commit (vsdd-cli#15, folded into vsdd-cli#820 and carried here). mdatron's core-library seam is mdatron's own decision.

### Merge method and ledger routing

The Per-milestone PR discipline member gains one sentence under its grade bullet: design pull requests merge with a merge commit, never squashed. The routing appendix below is executed as tracker comments at merge time, one per owning issue.

## Data and compatibility

Rename mapping, applied in one change:

| Old | New | Where |
|---|---|---|
| phase primer, primer | phase skill | prose in vocabulary scope; schema class `phase-primer` becomes `phase-skill`; `.mdatron/schemas/phase-primer.json` becomes `phase-skill.json`; `schemas::PHASE_PRIMER` becomes `schemas::PHASE_SKILL`; the budget class `phase-primer` becomes `phase-skill`. Frontmatter field names such as `primer_id` stay this cycle (schema-stable keys; renamed by the phase-skill rewrite with the files' move). File paths under `.claude/commands/` stay. |
| supplement | rules file | prose; schema class `supplement` becomes `rules-file`; `supplement.json` becomes `rules-file.json`; `schemas::SUPPLEMENT` and `artifacts::SUPPLEMENTS` follow; the budget class `supplement-section` becomes `rules-file-section`; the registered term "section" is redefined as a rules file's per-domain part. The directory `supplements/` keeps its path this cycle; its move is the composition milestone's install members. |
| domain prompt | reviewer role | prose; schema class `domain-prompt` becomes `reviewer-role`; `domain-prompt.json` becomes `reviewer-role.json`; `schemas::DOMAIN_PROMPT` and `artifacts::DOMAIN_PROMPTS` follow; the budget class `domain-prompt` becomes `reviewer-role`. "Domain" as the review perspective stays. |
| Slice 1 through Slice 7; "slice" as this program's unit | the live self-governance, composition, install, gate-execution, finding-lifecycle, recorded-dispatch and cost milestones; "milestone" | prose; the Decomposition's bold names; the build-plan's headings and structure rules; the register trigger; code comments. "vertical slice" (lexicon) and "slice" in the retrieval sense stay. Tracker milestone titles stay. Every Rust identifier, comment and test name that carries an old word is renamed (operator choice, 2026-10-08), so a search for the old words over the source tree returns nothing. |

Versions: economics-data schema 0.2.0 becomes 0.3.0 (budget class values); installed-artifact-manifest 0.3.2 becomes 0.3.3 (schema file names in the payload); vocabulary registry 0.4.0 becomes 0.5.0 (four new terms: phase skill, rules file, reviewer role, milestone by feature name; four deprecated-alias sets; the term "section" redefined). "Increment" and "attestation" are not registered here; decision 8 registers them when the phase-skill rewrite needs them.

Compatibility: adopters who ran `vsdd init` receive drifted managed files on the next init and the refusal diagnostic names them; the CHANGELOG entry states the rename and the data versions. The tracker's milestone titles and every closed record keep the old names as history. The knowledge pages published on 2026-10-08 already use the new words.

The Decomposition pin is re-pinned in the same pull request with `mdatron pin --update`, and the pin file's comments are restored by hand.

## Failure handling

- An old word surviving in prose fails `mdatron verify` with the deprecated-alias code; the fix is the rename or a code span where the text is a quoted title.
- A new design file without a route blocks as unclaimed; the route entries land in the same change.
- A Decomposition edit without the re-pin blocks; the re-pin lands in the same change.
- A renamed schema class without its schema file, constant or frontmatter in step fails the schema check or the tests; the change is one pull request, all or nothing.
- The install-slice red-gate test fails on the payload change until its counts and names are updated; it is updated in the same change.
- A ledger row with no owning open issue (the recorded-dispatch rows owned by closed issues) routes to the second program's standing issue or to vsdd-cli#897's close comment, never to a closed issue.
- The merge-method setting is the operator's; if the pull request is squashed by mistake, the file history loses this cycle's review commits and the decision is recorded as a documented exception on vsdd-cli#897.

## Security considerations

No secret, credential or environment change. The hook statement moves to data without changing any hook's behavior. The verdict-record wording defends the trust boundary the contract already draws: the agent remains untrusted for its own conformance, and the record it cannot rewrite is on the operator's side. The route table stays closed-world.

## Verification

- `mdatron verify` and `vsdd gate --ci` locally before the pull request opens; CI re-runs them.
- `cargo test` after the constant and data renames.
- The AC-2 and AC-3 searches, run and their output recorded on vsdd-cli#897.
- The milestone document's form checked by the route table's rules in `mdatron verify`.
- One cold reviewer, the Documentation Reviewer, on a child issue of vsdd-cli#897, reading the pull request diff with typed findings and a written stop rule (one round; a second only on a changes-required verdict); CI as the second reviewer. The reviewer's verdict comment is the cycle's verdict record in the convention form until the dispatcher writes records.
- The fresh-reader check of AC-8, recorded as a comment.

## Rollout and rollback

Execution rides the paved path for autonomous execution: after the proposal record is committed on the branch, `crosslink kickoff plan` over this document on vsdd-cli#897 produces the plan, and one `crosslink kickoff run --container` with explicit effort and budget dials performs the remaining commits on the draft pull request; the operator's dispatch act is the approval of that run. One pull request from a branch off main, opened as a draft at the first commit, with commits in this order so each step is reviewable: (1) the proposal record and its route; (2) the contract text changes (design folder, decisions, vehicle bindings, conformance scope, evidence wording, workspace, merge method, name map) and the re-pin; (3) the vocabulary renames across the corpus with the registry, anti-patterns, schema classes, schema files, constants, data versions and tests; (4) the build-plan rewrite and the register changes; (5) the composition milestone document and its route; (6) the CHANGELOG. The cold review runs against the draft; the operator's approval comment on vsdd-cli#897 precedes the merge; the merge is a merge commit. The ledger routing comments post after the merge, one per owning issue, and the knowledge page supersession lines land the same day.

Rollback: revert the merge commit; every rename, route, pin and register change reverts with it, and the vocabulary check returns to its previous anti-patterns.

## Open questions

None open. Resolved on 2026-10-08 by the operator: the merge method is a merge commit (REQ-10); the second program's standing issue opens at ratification (REQ-6); every Rust identifier that carries an old word is renamed (Data and compatibility); the execution rides a container kickoff run (Rollout and rollback).

## Out of scope

- The phase-skill rewrite's content, the frontmatter field renames and the files' move to the skills folder (vsdd-cli#898).
- Moving `supplements/` to a rules folder and the generated install members (the composition milestone's second increment).
- The gate-execution milestone document (created when that design re-enters; its ledger rows route as comments on vsdd-cli#836).
- The second program's umbrella and the trace-reading legs' design.
- The agent-memory ladder, the work-in-progress limit as a project rule, and the legacy knowledge pages' split (vsdd-cli#899).
- Renaming tracker milestones (no command exists; the titles are history).
- Any crosslink or mdatron feature request.

## Appendix: ledger routing

Composition milestone rows (nineteen): inline in `.design/composition-milestone.md`, one routed comment on vsdd-cli#839 citing the ledger page.

Live self-governance rows: the crate collapse resolves in the workspace sentence and the first increment (comment on vsdd-cli#820); the folded-in subprocess client, drift handling and templates resolve as recorded on vsdd-cli#835 and vsdd-cli#838 (comment on vsdd-cli#820); the manual-test checklist's adoption routes to vsdd-cli#874's successor record on vsdd-cli#820; the per-commit wall-clock budget routes to vsdd-cli#855; the hand-authored build-plan entry's expiry resolves here; the bulk-query upstream raise routes to vsdd-cli#855 as a register candidate; the install count literals resolve in the composition document; the disposition label-carry stays known on vsdd-cli#820.

Install rows: the retired observe workflow template stays on vsdd-cli#842 with a sequencing comment; the statusline wiring nit on vsdd-cli#713; the unclaimed crate name on vsdd-cli#664; the generated members' install path in the composition document.

Gate-execution rows (seven): comments on vsdd-cli#836, the evidence home conflict resolved by REQ-7, the verifier verdicts and the stale design routed to the document created when the design re-enters, the missing config fields to the composition document, the security hardening fixes to vsdd-cli#818, the fix-lane corpus known, the commit-msg friction hook to the gate-execution document.

Finding-lifecycle rows: the branch grammar on vsdd-cli#875; the events-store decommission on vsdd-cli#843 with a home named (the finding-lifecycle milestone); the history rewrite on vsdd-cli#656 and vsdd-cli#657; the doc-drift sidecar check on vsdd-cli#855.

Recorded-dispatch rows: the evidence re-home ruling's home resolves here (REQ-7); the operator-session audit's evidence moves to the second program; the null phase to the composition document; the critic's read-only posture to vsdd-cli#875's successor on the recorded-dispatch design; the review stage's size and the shared signing key to the second program's standing issue until the recorded-dispatch design opens; the native-spawn interceptor entry re-owned (REQ-6).

Cost rows: the report's transcript provenance resolves with REQ-6 and REQ-7 (the report reads dispatch manifests and the tracker); the cache-creation token class to vsdd-cli#875; the token-budget gate's CI wiring to the composition document; the rightsizing catalog on vsdd-cli#875; the completed-cycle fixture to the cost milestone's document when it opens.

Cross-cutting rows: the four register expiries resolved by the re-arm of 2026-10-08; the two retired designs resolved by the composition document and the gate-execution document to come; decisions recorded on other issues resolved by REQ-4; the three named artifacts that do not exist resolved in the composition document (the session skill's source and the state artifact) and the project configuration decision there.
