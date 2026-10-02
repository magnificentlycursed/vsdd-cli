---
title: "Content delivery and audit on crosslink and Claude Code: investigation, proposal and four-domain review (2026-10-02)"
tags: ["design-input", "review", "dispatch"]
sources: []
contributors: ["xqjG"]
created: 2026-10-02
updated: 2026-10-02
---

# Content delivery and audit on crosslink and Claude Code: investigation, proposal and four-domain review (2026-10-02)

## Status

**Design input. Nothing here is ratified and nothing has been amended into the contract.** This page records an investigation, a proposal written by the orchestrating session, and an independent review of that proposal by four domains. None of the four supported the proposal as written. The questions at the end are the operator's to answer; the answers then enter the owning slices' designs through the normal amendment route.

Recorded under vsdd-cli#888. The review dispatch is recorded on vsdd-cli#839.

## Basis and provenance

- **Crosslink:** the fork's working tree at `ddc0cbe57`; installed binary `0.9.0-beta.1+973e395dc`. Read from source and docs by read-only research agents on 2026-10-02. **Nothing was executed.** Paths are relative to the crate (`crosslink/` in the crosslink repo). The detailed kickoff and swarm findings are on `kickoff-swarm-dispatch-pipeline`; the rules and hook findings are on `crosslink-integration-surfaces`.
- **Claude Code:** from the knowledge page `runtime-harness-surface`, verified 2026-08-02 at Claude Code v2.1.212 and **not re-verified** for this work. Reviewer statements from their own knowledge of Claude Code are marked as such.
- **The contract:** `.design/agent-first-vsdd-toolkit.md` at vsdd-cli main `b99829af`. Members are cited by their heading names.

## The operator's intent and the decisions so far

- "My intent is to leverage crosslink so it makes sense for my supplements and skills to hook into crosslink's mechanisms."
- Supplements should be in force "when you write rust interactively".
- "I still kind of feel that we're selling some of this short for interactive sessions."
- **Decided 2026-10-02 (recorded on vsdd-cli#839):** the 28 vsdd prompt files — 18 domain prompts and 10 phase primers, today `.claude/commands/vsdd-*.md` — become Claude Code skills. This confirms the contract ("delivered and audited as skill invocations"; "generated skills and domain prompts") rather than changing it.

## State today

- **Nothing delivers a supplement to any session.** No hook, settings entry or rules file names `supplements/`. What an interactive session receives for Rust is crosslink's own `.crosslink/rules/rust.md`, not `supplements/rust.md`.
- **The supplement schema has no activation-trigger field,** although the contract's Deterministic composition member requires every supplement to declare one.
- **`.claude/agents/` and `.claude/rules/` do not exist** in vsdd-cli.
- **`.claude/skills/` is ignored** by the repo's `.gitignore` and by crosslink's managed block, with no tracked files in it. A tracked `vsdd-*` skill cannot exist there today without a carve-out.
- **Rust guidance already has three hand-kept homes:** `supplements/rust.md`, `.crosslink/rules/rust.md`, and crosslink's `rust-quality` and `rust-fix-discipline` skills.
- **No composition function exists** in `vsdd-core`; `init.rs` deploys the 28 prompts as static command files. Slice 2 owns the generator; Slice 6 owns the dispatcher.

## What crosslink offers: the delivery slots

| Slot | What the project controls | When it fires | Mechanical? | Limits |
|---|---|---|---|---|
| Fixed-name rule files in `.crosslink/rules/` (`global`, `project`, `knowledge`, `quality`, `external-content`, `tracking-<mode>`, and the file for a detected language) | The whole file content | Crosslink's prompt hook: the full block on the first prompt or after 4 h, then every third prompt here; every call in an agent worktree | Yes | Fixed filenames only; an extra file is read and never emitted. Language detection is per project, not per file. Upstream ships every rule file empty and its preflight skill says to confirm they stay empty. `crosslink init --force` blanks them. About 23 KB (roughly 5.8k tokens) per emission today. |
| A `design-doc:<slug>` label on the active issue | Which knowledge pages | Session start, including after compaction | Yes | At most 3 pages of 8,000 characters. Nothing applies the label automatically. |
| The kickoff prompt template (`--template`, or `agent.kickoff_template`) | A file that becomes the dispatched agent's prompt, with eight placeholders | Per dispatch, or repo-wide | Yes | File path only, no custom variables. A missing file silently falls back to the default prompt. |
| Per-phase swarm templates | One template per swarm phase | `swarm launch` | Yes | Per phase, not per agent |
| `--model`, `--effort`, `--budget-usd` on kickoff | The dials per dispatch | Per dispatch | Yes | Dollar cap only, Claude only. Recorded in an agent-writable file. |
| Session handoff notes and the last action | One string each | Re-injected at session start | Re-injection yes; writing depends on the agent | The last-action slot is overwritten by the next action |
| Hook entries in the tracked `.claude/settings.json` | The project's own hooks on any Claude Code event | Interactive sessions in the main checkout | Yes | A plain `crosslink init` replaces the whole `hooks` object unless it skips because every managed file is present; a fresh kickoff worktree is never complete, so these entries are always absent in dispatched agents |
| Knowledge pages | Content and tags | When the agent searches | No: by the agent's judgment | Substring search only |
| A project's own folders in `.claude/skills/` | Everything in them | Claude Code's own skill discovery | By judgment or explicit invocation | Crosslink neither registers nor delivers them; the folder is ignored |

## What crosslink does not offer

- **No way to register a downstream skill.** Its 18 skills are compiled in.
- **No per-file trigger and no read-before-edit gate** in any hook.
- **No generic "run this command as a gate" slot.** `swarm gate` runs only the detected test command.
- **No review vehicle.** `swarm review` launches nothing; kickoff always allows Write and Edit.
- **No record an agent cannot rewrite, other than its own signed hub events.** The prompt, the dial record and the full transcript sit in the worktree.
- **No parsing of skill invocations,** no token cap, no agent-count cap outside sentinel, and no cache-creation token class in its usage harvest.
- **Hook scripts cannot be shipped by a project.** They are machine-local and overwritten on update. A project ships data: rule files, `hook-config.json`, `.claude/settings.json`.

## What Claude Code offers natively

From `runtime-harness-surface` (2026-08-02), not re-verified. Crosslink manages none of these folders.

- **`.claude/rules/*.md`:** Claude Code's own rules folder, with optional `paths:` scoping. Unscoped rules are re-injected after compaction; path-scoped ones are lost until a matching file is read again.
- **Skill frontmatter:** `paths:` ("glob-triggered automatic activation"), skill-scoped `hooks:`, `model`, `effort`, and embedding a shell command's output in the skill body. **Disputed:** the AI Engineer reviewer's own understanding is that `paths:` only makes a skill eligible, with loading still left to the model's judgment. Nobody has tested it.
- **Agent types in `.claude/agents/*.md`:** a tool allow-list or deny-list, `model`, `effort`, `skills:` (the full skill content preloaded at start), and agent-scoped hooks. Per-agent transcripts record the agent type, the attributed skill, effort, and usage by cache class.
- **Hooks:** a pre-tool hook can deny with a reason; session start has a `compact` matcher; a pre-compaction hook is blockable; subagent start and stop match on agent type; a config-change hook is blockable; an `InstructionsLoaded` event records which rule files loaded and why.

## Where the contract assumes more than crosslink does

| Contract member | What it says | What was found |
|---|---|---|
| Phase exit by gate; Requirements: Gates | "Gates are commands; `crosslink swarm gate` runs them" | `swarm gate` runs only the detected test command |
| Recorded review dispatch | "Phase 3 dispatch runs on crosslink swarm" | `swarm review` writes a plan and launches no agents |
| Conformance at action time (the paved path) | Phase-3 rounds ride the swarm review; `swarm init --doc .design/build-plan.md` is the entry gate | The review command is unbuilt; `swarm init` is a heuristic splitter, and swarm-launched agents get only their one bullet and cannot use a container |
| Acceptance criteria: Swarm live fire; Decomposition: Slice 6 | One live `crosslink swarm review` run with findings filed and a manifest recorded is Slice 6's exit act | Cannot pass as written |
| Recorded review dispatch | "Reviewer roles are tool-restricted; a critic role cannot edit" | Kickoff's tool list always includes Write and Edit |
| Conformance at action time | "Edits to governed files are blocked until the session has read the governing docs" | No crosslink mechanism; it needs vsdd's own hook |
| Architecture | Hooks are "installed by init into `.claude/` and `.crosslink/` per crosslink conventions" | A project cannot ship hook scripts into crosslink's payload, and its settings entries are replaced in every worktree |
| Open questions: Swarm fallback | The fallback is "swarm primitives (worktree launch plus gate) with vsdd supplying the review stage" | The evidence supports taking the fallback now. The gate primitive is still the test command only. |

## The proposal as reviewed

Written by the orchestrating session after the investigation. Its first assessment had already been corrected once by the operator, and the reviewers were told so.

- **P1.** Kickoff becomes the contract's dispatch vehicle and the swarm bindings come out: phase-3 dispatch, `swarm gate`, the Swarm live fire criterion re-scoped, the Swarm fallback question closed in favour of kickoff plus vsdd supplying the review stage.
- **P2.** Delivery is stated per session kind. Dispatched agents: `vsdd dispatch` computes the composition and renders a kickoff prompt template with the primer, domains and supplements inlined, then calls kickoff with the dials and the container option. Interactive sessions: vsdd's own session-start hook entry in `.claude/settings.json` for always-on content and the phase pointer; crosslink's prompt hook keeps carrying the existing short rules; each piece of content has exactly one home.
- **P3.** Supplements become skills with `paths:`, reversing the contract's "supplements emit no invocation event". The supplement schema gains the activation-trigger field.
- **P4.** Activation evidence is split: skill invocation records for interactive sessions, a hash of the rendered prompt for dispatched agents.
- **P5.** Before launch the dispatcher posts a tracker comment, signed with the operator's driver identity, carrying the prompt hash, the composition hash and the dials. Proof of what an agent loaded stays could-not-check.
- **P6.** An open question, not recommended either way: make the 18 domain prompts agent types (tool-restricted, the domain skill preloaded, dials set) so that review rounds can run on the interactive path now. A pre-tool hook on the Agent tool might enforce an agent-count ceiling (untested).
- **P7.** Leave alone: riding crosslink for the tracker, sessions, signing and knowledge; the could-not-check grade; the customised rule files, added to the install manifest so that a blanking is detected; the exception-register entries.

## Review: verdicts

| Item | Solution Owner | Solution Architect | AI Engineer | Red Team |
|---|---|---|---|---|
| P1 kickoff as the vehicle, swarm out | Amend | Support | Amend | Amend |
| P2 delivery per session kind | Amend | Amend | Amend | Amend |
| P3 supplements as skills with `paths:` | Amend | Amend | Oppose | Oppose |
| P4 split activation evidence | Amend | Amend | Amend | Amend |
| P5 signed pre-launch record | Amend | Support, regraded | Support | Amend |
| P6 domain reviewers as agent types | Oppose bundling | Oppose as framed | Support defining them | Oppose |
| P7 leave the rest alone | Support | Support | Support | Amend |

## Review: where the reviewers agree

- **Supplements belong in Claude Code's `.claude/rules/`, not in skills** (Solution Architect, AI Engineer; the Solution Owner and Red Team reject P3 on the unverified mechanism). Path-scoped rules are injected when a matching file is read, unscoped ones survive compaction, and the contract's "delivered by injection" rule stays intact. Caveat: a rule triggers on read, so a brand-new file may not fire it.
- **The signed pre-launch comment does not prove an operator act.** The interactive agent signs with the same driver key, and `crosslink ` is an allowed command prefix. It shows only that the declaration was not altered after posting. The contract's Trust boundaries already say the driver-key anchor cannot tell an operator decision from an agent-authored comment.
- **Every delivery form must be generator output from one source,** checked byte for byte in CI and listed in the install manifest. The contract's Generated context requirement and its byte-match criterion are that mechanism; the proposal did not invoke them.
- **This is a contract amendment and needs the owned route:** owned composition, cold review, and a recorded Solution Owner decision. P1, P3 and P4 change at least nine places in the contract.
- **P1 must name what runs the gates** once `swarm gate` is out, and say what happens to the build-plan's swarm entry.
- **P6 should not be bundled.** It reopens recorded decisions and needs a spike first.

## Review: corrections to the proposal's facts

- **A tracked vsdd skill cannot exist today.** The proposal said a tracked skill would reach dispatched worktrees; `.claude/skills/` is ignored.
- **A plain `crosslink init`, not only a forced one, replaces the `hooks` object** of the settings file whenever it runs rather than skips.
- **The install manifest cannot detect blanked rule files.** Its resolution kinds only check that a file exists. It would need a content-hash kind.
- **The proposal misstated the contract's fallback.** The Swarm fallback question names swarm primitives plus a vsdd review stage, not kickoff alone. Removing `swarm gate` as well is new scope.
- **Two recorded decisions bear on P6 and were missed:** the exception register resolved `manual-dispatch-fallback` on 2026-09-27 (the Agent-tool fallback "no longer rides"), and the act-to-vehicle map carries `attended-review-round-fan-out`, operator-adopted 2026-07-21.
- **"Each piece of content has exactly one home" was asserted, not shown.** See the three homes for Rust guidance above.
- **Crosslink's rule block is not short.** The "condensed" reminder carries every rule in full.

## Review: findings by reviewer

**Solution Owner**
- Blocker: the proposal is a contract amendment with no amendment route.
- Major: "vsdd supplying the review stage" is unpriced scope growth: vsdd would own round fan-out, stop rules and reconciliation, against the contract's division of labour, which puts orchestration in crosslink. It needs a stated size and a slice home.
- Major: P3 may breach "Availability is not activation"; it also drops the required hook backstop and delivers whole supplements where the contract requires per-domain sections priced separately.
- Minor: harness detail (`paths:`, `.claude/agents/`) belongs in the runtime-harness supplement and the act-to-vehicle map, not in the contract.
- Missed by the proposal: the dispatcher is Slice 6 and depends on Slices 2 and 5, so the proposal re-sequences the decomposition implicitly; nothing says how `vsdd init` ships any of this to adopters; a review round needs eleven or more domains, the same shape as the two overspends; P3 and P6 deepen the Claude Code binding against the agnosticism goal.
- Suggested order: re-verify the Claude Code facts and whether hooks fire headless; add the supplement trigger field; take P1 through the owned flow; fold P2 to P5 into the Slice 2 and Slice 6 designs; decide P6 last, after a spike.

**Solution Architect**
- Blocker: tracked vsdd skills cannot exist today (above). Generate into a folder crosslink does not manage, or carry a keep-line outside the managed block and list it in the install manifest.
- Major: vsdd's session-start entry is one plain init away from deletion. Add an install-integrity member that fails loudly when it is absent, or deliver always-on content through unscoped `.claude/rules/`.
- Major: the prompt hash is not recomputable. The rendered prompt includes crosslink's built prompt, which varies by crosslink version, and the contract requires the verifier to recompute the expected set. Render vsdd's block as a pure function of the composition and the source content hashes, delimit it, and hash it separately.
- Major: nothing in P2, P4 or P5 can be built yet; the proposal does not place itself against Slices 2, 3 and 6.
- Minor: the pure-core and effectful-shell split survives if rendering stays pure and file-writing, kickoff and comment posting stay in the shell.
- Missed: commands and skills are one surface in Claude Code, so "become skills" may be a frontmatter change on the tracked command files rather than a move into an ignored folder (to be checked); no control-effectiveness registry entries for the new controls; which surfaces are documented interfaces (`--template`, `--budget-usd`, frontmatter) and which are internals read from source (the prompt hook's filename set, the init merge) is not recorded with retest triggers.

**AI Engineer**
- Major: inlined dispatch content does not survive compaction. Kickoff passes the composition as the first user message; a long run keeps only a summary, which the contract counts as a paraphrase. Carry it in the system prompt (an agent type, or a system-prompt file flag; crosslink passes neither) or re-inject on the compact matcher.
- Major: the proposal's account of skill `paths:` is probably overstated (reviewer's own knowledge, moderate confidence). One live test before P3 is decided.
- Major: whole-roster inlining is the wrong shape. Twelve domains, the phase-3 primer and three always-on supplements come to about 101 KB, roughly 25k tokens; one reviewer's slice is about 33 KB, roughly 8k. At slice size, inlining beats on-demand loading. P2 should say "per-agent slice".
- Major: the prompt hash has a gap between hashing and launch. Crosslink writes `KICKOFF.md` into the agent-writable worktree and the launch reads that file. Hash the dry-run output or the written file before launch, and record per-member hashes, because the whole-prompt hash varies with issue and branch.
- Minor: `rigor.md` is loaded by crosslink's prompt hook and never emitted. `supplements/claude-code-cli.md` at 9.7 KB is over the 2,000-token supplement-section budget if inlined whole; per-section emission is unbuilt.
- Missed: the `InstructionsLoaded` hook as the audit signal for rules-delivered supplements; passing an agent type to kickoff so one definition serves both paths (needs an upstream flag); hooks declared in tracked skill and agent files, which are not in the settings file and so escape the replacement; a Claude Code plugin as the package; cross-agent cache reuse on kickoff is probably nil because each worktree has a different working directory.

**Red Team**
- Blocker: the "operator's driver identity" is the key the interactive agent already signs with, and every dispatched agent's private key sits under the same OS user. State P5 as tamper-evidence of the declaration only, or sign with a key the agent cannot read.
- Blocker: P6 has no preventive leg. The exception register says a count interceptor needs "an outer-harness hook the checked agent cannot reach"; a project-settings hook is reachable. Both overspends are still marked advisory in the regression corpus.
- Major: template failure is silent. `vsdd dispatch` must fail closed unless the rendered prompt contains every composed member.
- Major: the worktree init leaves the tracked settings file dirty with the hooks stripped (inferred); a broad `git add` could carry that into a PR. Nothing checks that vsdd's entries are present.
- Major: a prompt hash proves what was sent. The paraphrase escape and vsdd-cli#821 were about what was read and done.

| Control | How it is bypassed | Prevents or detects | Where its record sits | Honest grade |
|---|---|---|---|---|
| Session-start injection | Edit `.claude/settings.json`, or disable hooks in the gitignored `settings.local.json` | Neither; delivery only | Local transcript | Friction |
| Skills with `paths:` | Write through Bash, or edit the skill's frontmatter | Neither | Local transcript | Convention |
| Pre-tool edit gate | Bash writes if the matcher is Write and Edit only; a settings edit; `hook-config.local.json` | Prevents until skipped | Local | Friction |
| Tool-restricted agent types | Spawn the general-purpose type instead; edit the agent-type file | Prevents only inside the chosen type | Tracked file, agent-writable | Friction |
| Agent-count hook | The counter file is agent-writable; nested spawns; other spawn primitives; a settings edit | Prevents until skipped | Local | Friction (untested) |
| Inlined kickoff prompt | Call kickoff directly; `agent.no_template`; a missing template | Delivers; no proof of adherence | `KICKOFF.md` in the worktree | Friction |
| Signed pre-launch comment | The agent posts it itself | Detects edits after posting | The hub, out of reach once synced | Convention on authorship |
| Worktree transcript | Rewrite the file | Neither | Worktree | Could-not-check |
| The main-branch ruleset | None found | Prevents | Server | Server-side block |

## The simpler structure the reviewers put forward

The Solution Architect and the AI Engineer converge on this, and it needs no contract reversal for supplements: **one generator, emitting (a) a delimited block for the kickoff prompt of each dispatched agent and (b) files for the interactive session.**

| Content | Interactive session | Dispatched agent | Why |
|---|---|---|---|
| Always-on supplements (about 3.3k tokens) | Unscoped `.claude/rules/` | Inlined, or in an agent-type body | Unscoped rules are re-injected after compaction; no hook needed |
| File-triggered supplements (the Rust supplement, 5.5 KB) | `.claude/rules/` with `paths:` | Inlined at dispatch | Mechanical on file read; tracked, so it also reaches worktrees |
| Phase primer (2 to 8 KB) | A skill the operator invokes | Inlined, or preloaded through an agent type's `skills:` | Invoked skills are re-attached after compaction (5k tokens each, 25k in total) |
| Domain prompt (3 to 9 KB) | Not loaded; reviews are dispatched | An agent type with `skills:` preload and a tool allow-list | By construction, not judgment; the tool restriction is mechanical |
| Phase pointer (under 200 tokens) | Session-start hook, including the compact matcher | In the rendered prompt | Small, and must re-arrive after compaction |

Token figures are estimates from file sizes at four bytes per token, not trace-recorded.

## Hardening options (Red Team)

The three changes that would most raise the cost of bypass:

1. Move the hooks, the agent ceiling and the hook-disabling switch into Claude Code managed settings, outside the agent's write reach.
2. A required CI check that fails when the settings file, skill, agent-type or rule files differ from a hashed manifest, and when a kickoff has no matching pre-launch record.
3. Sign dispatch approval with a key the agent's OS user cannot read (a hardware token or a passphrase), so that the dispatcher cannot be the agent.

## A finding about the current hook configuration

Raised by the Red Team and confirmed by the orchestrating session against the deployed hooks and the tracked config on 2026-10-02. Recorded on vsdd-cli#855. Not exercised.

- **Agent context is decided by `agent.json` carrying the role "agent", or by the working directory containing `/.claude/worktrees/` or `/.codex/worktrees/`** (`crosslink_config.py`, the agent-context check).
- **In agent context the hook takes `agent_overrides` from `hook-config.json`.** vsdd-cli's tracked `agent_overrides` block only forced pushes, hard resets, cleans and wholesale checkouts or restores, gate nothing, and set tracking to relaxed. So in agent context the hook neither blocks `gh pr merge`, `git merge` or `git rebase` nor gates `git commit`.
- **Upstream's shipped default is stricter.** Since crosslink PR #65 (crosslink#58, this estate's own fix) the default agent list also blocks merge, rebase, cherry-pick, stash, tag and branch deletion, and gates commit. vsdd-cli's tracked config did not pick that up. Neither list blocks `gh pr merge` for agents.
- **A gitignored `hook-config.local.json` is merged over the tracked config,** so it can change the tracking mode and the block lists invisibly. None exists in this checkout today.

## Questions for the operator

1. **Review vehicle:** one kickoff per reviewer, interactive agent types, or hold until a spike? Does the 2026-09-27 "fallback no longer rides" decision stand, or do attended review rounds on the Agent tool remain legitimate under the 2026-07-21 adoption?
2. **Swarm:** drop it from the contract entirely, including `swarm gate` and the build-plan's swarm entry, or only for review dispatch?
3. **Supplements:** deliver them through `.claude/rules/` rather than as skills, subject to the live test?
4. **"Become skills":** must the 28 files move into the skills folder, or is skill behaviour from the tracked command files enough?
5. **Rust guidance:** always-on, or loaded when a matching file is first read? Must it be in place before the first edit (which needs a read gate)?
6. **Dispatched agents:** is an inlined prompt plus a hash acceptable evidence in place of skill invocation?
7. **Hardening:** managed settings on this machine, and a separate OS user or hardware key for dispatch approval?
8. **Scope and order:** is vsdd owning the review-round orchestrator in scope for v1, and may the dispatcher be pulled ahead of Slices 4 and 5?
9. **Standing cost:** is roughly 5.8k tokens every third prompt from crosslink's prompt hook accepted?
10. **Upstream:** raise that plain init replaces a project's hooks (crosslink#15 is open) and ask for an agent-type or system-prompt passthrough on kickoff, or accept detection only?

## Unverified, and the three live tests proposed

- How `paths:` behaves on a skill versus on a file in `.claude/rules/`: mechanical load, or eligibility only.
- Whether Claude Code hooks fire in a headless kickoff run.
- Whether kickoff's init strips a project's hook entries in a worktree, and whether it leaves the tracked settings file dirty.

Also unverified: the Claude Code facts as a whole (two months old); whether a pre-tool hook on the Agent tool can enforce a count ceiling; whether subagents receive crosslink's rules (one research agent found no rules block in its own context); which spawn primitive the two overspends used.

## How the work was run

- **Research:** five read-only agents on the interactive Agent-tool path: three on crosslink's delivery, install and dispatch surfaces (about 564k tokens), then one each on kickoff and swarm (about 473k tokens).
- **Review:** four independent reviewers on the same path, about 486k tokens. Each was told to read its own domain prompt file in full, the contract and the proposal, and to check the proposal's claims against the raw sources. None saw another's review. No verifier agents were added.
- **Model and effort:** the session default for every agent. This was a hand-run dispatch under the contract's bootstrap interim, recorded as such on vsdd-cli#839. It is the same interactive path that question 1 is about.

## Related pages

`kickoff-swarm-dispatch-pipeline`, `crosslink-integration-surfaces`, `runtime-harness-surface`, `run-record-capability-inventory`, `composition-slice`, `regression-corpus`, `verifiable-conformance-and-efficiency`.
