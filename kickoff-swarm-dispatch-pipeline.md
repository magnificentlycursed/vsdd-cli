---
title: "kickoff-swarm-dispatch-pipeline"
tags: ["reference", "design-doc", "dispatch"]
sources: []
contributors: ["xqjG"]
created: 2026-08-02
updated: 2026-10-02
---

# Kickoff and swarm: intended use, actual capability, and fit for vsdd

Rewritten 2026-10-02 under vsdd-cli#888. This replaces the 2026-08-02 text of this page, most of which described work that has since merged. The last section lists what changed.

## Basis and provenance

- **Tree read:** the crosslink fork's working tree at `ddc0cbe57` (fork develop `cc756de92` plus one daemon commit). The fork is 12 commits ahead of upstream develop as known locally (`29d018525`, 2026-09-12). None of the 12 touch swarm; the kickoff differences are the container-readiness fixes listed under "This estate's contributions". Nothing later than 2026-09-12 was fetched from upstream.
- **Installed binary:** `0.9.0-beta.1+973e395dc`, which is fork develop before crosslink-fork PR #8.
- **Method:** two read-only research agents read the source, crosslink's own docs (`docs_src/`), the git history and both issue trackers on 2026-10-02. **Nothing was executed** except `--help`. Every behaviour below is read from code, not observed, except the two items marked "observed" or "verified", which come from the live tests run later the same day (vsdd-cli#890).
- **Re-checked by the orchestrating session the same day:** the swarm launch options (`src/commands/swarm/lifecycle.rs:686-720`), the per-phase template lookup (`:632-637`), the gate command (`:809-810`), the review and fix plan writers and their printed messages (`src/commands/swarm/review.rs:118, 278, 547, 567, 574`), the pipeline stub (`src/pipeline.rs:293-339`), template resolution (`src/utils.rs:13-41`), template use in `run` (`src/commands/kickoff/run.rs:107-128`), the base tool list (`src/commands/kickoff/prompt.rs:417-444`), the worktree init call (`src/commands/kickoff/launch.rs:596-599`) and the missing cache-creation class (`src/token_usage.rs:219`).
- **Paths** are relative to the Rust crate (`crosslink/` inside the crosslink repo) unless they start with `docs_src/`. Line numbers are for the tree above and will drift.
- **Handles:** `crosslink#N` and `crosslink PR #N` are the upstream tracker (Corvidae-Coding-Projects/crosslink); `crosslink-fork PR #N` is the fork (magnificentlycursed/crosslink).

## Summary

- **Kickoff is crosslink's only agent launcher.** It starts one coding agent, headless, in a fresh git worktree on a new branch, in tmux or a container, with a generated prompt. It is built for "implement this feature from a description or design doc". It is not built for review or read-only work. It is the maintained path: 108 commits over the period in which swarm had 56.
- **Swarm's build half works, and is driven differently than vsdd assumed.** `init`, `launch`, `gate`, `checkpoint` and `resume` form a plan-and-state layer on the hub branch. A driver agent runs each command on the human's say-so and does the merging itself. `swarm launch` calls kickoff once per agent.
- **Swarm's review half was never finished.** `swarm review` and `swarm fix` write plan files that nothing reads. `swarm pipeline` prints what it would do. The original source comment says real implementations "will be wired in from other modules in subsequent PRs"; they were not. No doc or skill explains how to use the review commands.
- **Crosslink has three planning mechanisms and one launcher:** swarm's heuristic doc splitter, the dashboard orchestrator's LLM decomposition, and `kickoff plan`'s gap analysis. Only kickoff launches agents.
- **For vsdd:** kickoff can carry a computed composition through its prompt template and can set model, effort and a dollar cap per dispatch. It cannot stop a reviewer from editing, it keeps every run record where the agent can rewrite it, and it replaces a project's own hook entries in every worktree. Swarm cannot run review rounds, cannot run vsdd's gates, and cannot use a container.

## Kickoff: intended workflow

From `docs_src/guides/kickoff.qmd` and `docs_src/guides/design-workflow.qmd`.

1. **A human with an attended agent runs `/design`.** It writes `.design/<slug>.md` interactively. "The output feeds directly into `/kickoff` and `crosslink swarm`" (`design-workflow.qmd:10`).
2. **The driver runs `crosslink kickoff plan <doc>`:** "a read-only gap analysis" (`design-workflow.qmd:220`). The agent is asked to write `.kickoff-plan.json` and copy it to `<doc>.plan.json` (`src/commands/kickoff/plan.rs:74, 118-124`).
3. **The driver runs `crosslink kickoff run "<description>" --doc <doc>`.** "The agent receives the full design document as context" (`design-workflow.qmd:242`). `--doc` does five things:
   - inlines the parsed doc sections into the prompt (`src/commands/kickoff/prompt.rs:279-288`);
   - adds the earlier plan's subtasks, assumptions and advisory gaps (`prompt.rs:290-295`);
   - writes `.kickoff-criteria.json` (`run.rs:151-160`);
   - hashes the doc, sets it read-only, and mounts it read-only in a container (`run.rs:305-331`, `launch.rs:916-927`);
   - appends a run row to the pipeline file beside the doc (`run.rs:182-192`).
4. **The agent follows the generated protocol:** session, plan comment, implement, test and lint, commit, report, then write `DONE` to `.kickoff-status` (`kickoff.qmd:177-188`).
5. **The driver monitors** with `/check` and `kickoff status|logs|list` (`kickoff.qmd:287-299`).
6. **The driver reads `kickoff report`, reviews and merges by hand,** then runs `kickoff cleanup`. Kickoff has no merge step.

**Verify levels** are prompt sections only (`kickoff.qmd:198-220`): `local` adds tests and lint; `ci` adds push, a draft PR, CI polling and up to five fix cycles, then `CI_FAILED` (`prompt.rs:45-66`); `thorough` adds a self-review checklist (`prompt.rs:68-86`).

**The pipeline file `<doc>.pipeline.json`** holds `schema_version`, `design_doc`, `doc_hash`, `stage`, `plans[]` and `runs[]` (`src/commands/kickoff/pipeline.rs:6-44`). `mark_planned` is dead code (`pipeline.rs:135`), so plan rows never reach "done" through the CLI. Runs are reconciled only by the `kickoff status` overview and by `cleanup` (`src/commands/kickoff/mod.rs:465-467`, `cleanup.rs:242`).

Doc drift: `kickoff.qmd:34` shows `kickoff run --doc …` without the description, which is a required positional argument (`src/main.rs:1414`).

## Kickoff: subcommands and flags

From `src/main.rs:1412-1622`. The installed binary's `--help` prints no descriptions.

| Subcommand | Flags and defaults |
|---|---|
| `run <description>` | `--issue`, `--container none\|docker\|podman`, `--verify local`, `--model standard`, `--image`, `--timeout 1h`, `--dry-run`, `--branch`, `--doc`, `--skip-permissions`, `--permission-mode`, `--effort`, `--budget-usd`, `--template` |
| `plan <doc>` | `--issue`, `--model`, `--timeout 30m`, `--dry-run`, permission flags, `--effort`, `--budget-usd`, `--template`; tmux only |
| `launch` / `go [doc]` | wizard, or `--plan` / `--run`; no template, effort or budget flags (`mod.rs:257-258, 271, 326`) |
| `status [agent]` | none; with no agent it prints the pipeline overview |
| `logs <agent>` | `-l 20` |
| `stop <agent>` | `--force` |
| `show-plan <agent>` | none |
| `report [agent]` | `--json`, `--markdown`, `--all` |
| `list` | `--status` |
| `cleanup` | `--dry-run`, `--force`, `--keep`, `--json` |
| `graph` | `--all` |

## Kickoff: how the prompt is built and how a project can shape it

- **The built prompt** (`prompt.rs:202-316`) is a fixed header and 11 instruction steps, then the design-doc sections, the open-questions escalation, the canonical-doc stanza, plan context, test and lint steps, the CI and self-review sections, a reporting section (only when the doc has acceptance criteria) and final steps. Project files are not read into it; one step tells the agent to read AGENTS.md, CLAUDE.md and `.crosslink/rules/` (`prompt.rs:252`).
- **Conventions detection** picks test and lint commands and extra tools from the manifests it finds: Cargo.toml, package.json, pyproject, go.mod, justfile, Makefile, shell scripts, mix.exs (`src/commands/kickoff/helpers.rs:184-319`). The commands are hard-coded per manifest, not configurable.
- **Allowed tools** start from a fixed base that always includes `Read`, `Write`, `Edit`, `Skill`, `Task`, `WebSearch`, `WebFetch`, `Bash(git *)` and `Bash(crosslink *)` (`prompt.rs:417-444`). `ci` and `thorough` add `gh` and `sleep`. Conventions and the `kickoff.allowed_tools` config key add more. **Config can only add tools, never remove them** (`helpers.rs:164-182`). `plan` uses a separate fixed read-only list with no Write (`plan.rs:13-33`).
- **The prompt template.** `--template <path>` wins; otherwise `agent.kickoff_template` in `hook-config.json`, a path relative to `.crosslink/` (`src/utils.rs:13-41`).
  - Interpolation is plain string replacement of exactly eight placeholders: `{{built_prompt}}`, `{{issue_id}}`, `{{branch}}`, `{{description}}`, `{{model}}`, `{{effort}}`, `{{doc_path}}`, `{{allowed_tools}}`, with `{{built_prompt}}` replaced last (`prompt.rs:190-200`).
  - There are no custom variables. An unknown `{{x}}` stays in the text verbatim.
  - **A missing, unreadable or empty template file silently falls back to the built prompt.** When the flag is given, the config key is not consulted.
  - **A template without `{{built_prompt}}` replaces crosslink's protocol wholesale, with no warning.**
  - `agent.no_template: true` makes `run` write an empty `KICKOFF.md` (`run.rs:107-108`). `plan` ignores that key.
  - `{{allowed_tools}}` holds only the project-added tools, not the full list (`run.rs:113`), and `kickoff.allowed_tools` appears in it twice (`run.rs:48-50, 103-105`).
  - In `plan`, branch, description and allowed_tools are empty and issue_id is 0 when absent (`plan.rs:181-190`).
- **Swarm** resolves one template per phase at `.crosslink/swarm-templates/<phase-slug>.md` (`src/commands/swarm/lifecycle.rs:632-637`). There is no per-agent template within a phase.
- **What is not recorded:** the template's path or hash. Only the assembled `KICKOFF.md` survives (`run.rs:148`).
- **Documentation:** the template mechanism is absent from `docs_src/`, from the kickoff skill and from the CHANGELOG. Only the fork's `.design/dispatch-dials-and-injection-seam.md` describes it.

## Kickoff: launch modes

- **Both modes run headless.** Claude gets `-p --output-format stream-json --verbose` (`src/agents/claude.rs:48-56`). Codex gets `codex exec - --json` (`src/agents/codex.rs:24, 74`).
- **Command shape** (`src/agents/invocation.rs:244-282`, `launch.rs:292-294`): `timeout Ns env -u CLAUDECODE -u ANTHROPIC_API_KEY … claude [permission flag] --model M [--effort E] [--max-budget-usd B] --allowedTools … -- "$(cat KICKOFF.md)"`, piped through `tee -a .crosslink/runtime/agent-events.jsonl`.
- **The composition arrives as the first user message.** Crosslink passes no system-prompt flag and no agent-type flag. Nothing re-reads `KICKOFF.md` after a compaction: the session-start hook has no branch for it. In a long run the prompt's content survives compaction only as a summary. (Noted by the AI Engineer review; the compaction behaviour is inferred.)
- **Permissions** (`launch.rs:159-211`): `--skip-permissions` maps to the agent's skip-permissions flag; `--permission-mode plan` forces a read-only posture. The default comes from `agent.providers.<provider>.approval`; for Claude that is "interactive", meaning no flag (`src/agents/config.rs:74-83`). Claude read-only combined with any non-interactive approval is an error (`claude.rs:15-20`).
- **Local:** a tmux session, with an optional `sandbox.command` wrapper (`launch.rs:282-287`). Preflight checks account login (`launch.rs:457-459`).
- **Container** (`launch.rs:788-965`): `docker|podman run -d` with the worktree mounted at `/workspaces/repo`, the host `.git` mounted read-write at its host path, the hub and knowledge caches mounted read-write (fork), a per-provider credential volume, and `GH_TOKEN` plus a git credential helper when the host has a token (fork). API keys are stripped (`invocation.rs:284-296`). There is no sandbox wrapper and no watchdog in a container.
- **Timeout:** the inner `timeout` command is the run limit. Docker's `--stop-timeout` is only a stop grace period; `docs_src/reference/hook-config.qmd:222` calls it timeout enforcement, which is misleading.
- **Stall watchdog, local only** (`launch.rs:78-110`): after a 300 s grace it checks every 120 s, and if the last heartbeat is older than 300 s it types "continue working…" into the tmux session, at most five times. Against a headless process this is probably inert (inferred).

## Kickoff: what is in the worktree at start

- **Tracked files at HEAD,** then `crosslink init --skip-signing --defaults` (`launch.rs:596-599`).
- **The init replaces the `hooks` object of `.claude/settings.json` with crosslink's template.** `write_settings_json_merged` unions `allowedTools`, then inserts the template's hooks over whatever was there (`src/commands/init/merge.rs:201-219`). Other top-level keys survive. Init skips only when every managed file already exists (`src/commands/init/mod.rs:1011-1029`); `.crosslink/integrations/` is gitignored, so a fresh worktree is never complete and the replacement always happens. **No init flag or config key preserves a project's hooks.** crosslink#15 is open on this.
- **Observed 2026-10-02** (kickoff's init command run in an isolated clone of vsdd-cli, vsdd-cli#890): the probe hook entry added to the settings file was gone afterwards and the session-start entries went from two to one; `permissions`, `statusLine` and a probe top-level key survived; `allowedTools` was added. The template's wrapper is fail-open (`else exit 0`) where vsdd-cli's tracked wiring is fail-closed (vsdd-cli#658). The init left `.claude/settings.json`, `.gitignore` and `.crosslink/.gitignore` modified and added `AGENTS.md` and `.codex/`, so a blanket `git add` in the worktree would stage all five. The root `.gitignore` lost vsdd-cli's keep-lines for `.claude/commands/vsdd-*`.
- **The gitignored payload is regenerated from the binary,** not copied from the host: `.crosslink/integrations/`, crosslink's skills and commands (`init/mod.rs:617-692`).
- **`.mcp.json` is merged:** a project's own servers survive; crosslink's two entries are overwritten (`merge.rs:95-153`).
- **Tracked rules and `hook-config.json` survive.** Rules deploy only when `.crosslink/rules/` is absent. `rules.local/` and `hook-config.local.json` are gitignored, so they are absent.
- **A skill tracked in git would be present.** In vsdd-cli today none can be: `.claude/skills/` is ignored by the repo's `.gitignore` and by crosslink's managed block.
- **A container mounts that same worktree,** and its entrypoint runs `crosslink init` again.
- **After init:** readiness `daemon::ensure` (fork; `launch.rs:617-624`), an agent identity with automatic trust approval when no `agent.json` exists (`launch.rs:625-643`), `crosslink sync`, `session start`, `session work <issue>`. The last two are fatal on failure (`launch.rs:656-681`).

## Kickoff: model, effort and budget

- **Model:** `default|standard|advanced` map through `agent.providers.<provider>.*_model`; any other string passes through as a literal model name (`src/agents/config.rs:108-116`).
- **Effort:** `low|medium|high|xhigh|max`. Claude gets `--effort`; Codex gets `model_reasoning_effort` (`codex.rs:67-72`).
- **Budget:** an unvalidated string passed to Claude as `--max-budget-usd` (`claude.rs:42-44`). It is a dollar cap enforced by the Claude CLI. Codex rejects it (`codex.rs:14-18`). There is no token cap and no agent-count cap in kickoff.
- **Recorded** in `.kickoff-metadata.json`: `started_at`, `timeout_secs`, `provider`, the resolved `model`, `effort`, `budget_usd` (`src/commands/kickoff/types.rs:43-60`, `run.rs:162-170`). That file is in the worktree, unsigned.
- **Swarm** sets one model, effort and budget for the whole swarm with `swarm config`. `crosslink design` has no effort dial (`src/commands/design_cmd.rs:68-80`).

## Kickoff: records and who can rewrite them

| Record | Written by | Where | Integrity |
|---|---|---|---|
| `KICKOFF.md`, `.kickoff-slug`, `.kickoff-metadata.json`, `.kickoff-criteria.json`, `.kickoff-session` | launcher | worktree, git-excluded | none; agent-writable |
| `.kickoff-doc.json` (doc path and sha256) | launcher | worktree | checked by `status` and `report` (`helpers.rs:387-425`), but the file itself is agent-writable |
| `.kickoff-status` | launcher (LAUNCHING, RUNNING, FAILED; local only), the shell wrapper (TIMEOUT), the agent (DONE, CI_FAILED) | worktree | none |
| `.crosslink/runtime/agent-events.jsonl` (the full transcript, tool-use events included) | `tee` beside the agent | worktree | none; agent-writable |
| `.kickoff-report.json`, `.kickoff-plan.json` | the agent | worktree | self-reported; only missing-field warnings (`helpers.rs:329-344`) |
| `<doc>.pipeline.json` | host CLI | main checkout, beside the doc | unsigned; outside a container's mounts |
| typed comments, labels, locks, session events | the agent, through the hub | the agent's own hub ref | signed event envelope with a sequence number (`src/events.rs:29-38, 329-358`); the agent can write its own ref |
| `token_usage` rows | the host, at `kickoff stop` or `cleanup` only (`monitor.rs:534`, `cleanup.rs:18-26`) | the driver's local database | derived from the agent-writable transcript |

- **Usage harvest** parses only lines with a top-level `usage`, which for Claude is the final `result` line (`src/agents/events.rs:163-182`). Classes captured: input, output, cache-read, reasoning. **The cache-creation class is hard-coded to none** (`src/token_usage.rs:219`). Cost is a pricing-table estimate. A run killed by timeout probably yields no usage rows (inferred: no `result` line).
- **Reading it back:** the dashboard API `/api/v1/usage`, or the SQLite file directly. No CLI subcommand was found.
- **No crosslink code parses the transcript for skill invocations.** Only usage, status and the last message are extracted.
- **Observed 2026-10-02 in a headless run with kickoff's flags:** the stream-json transcript also carries `hook_started` and `hook_response` events with each hook's output, and the final result line carries `total_cost_usd` and usage including `cache_creation_input_tokens`. Crosslink's harvest drops that class; the transcript has it.
- **Net:** the only records an agent cannot quietly rewrite are its signed hub events, and it authors those itself. The prompt, the transcript and the dial record all sit where the agent can write.

## Kickoff: failure handling

- **Launch:** preflight fails on missing binaries (`launch.rs:383-455`). A failed tmux send writes FAILED (`launch.rs:769-773`). A container launch error exits with a pull hint and writes no status file.
- **Timeout:** exit 124 writes `TIMEOUT` (`launch.rs:293`). crosslink#60 is fixed. `status` also derives timed-out from the metadata clock (`types.rs:344-359`).
- **Status** is the status file overlaid by the transcript: a `result` line gives done or failed (`events.rs:87-96`, `monitor.rs:156-163`).
- **An agent that crashes without timing out writes nothing.** Status stays RUNNING until the wall clock expires. A container login failure behaves the same way.
- **`list`** marks "running without tmux" as stopped and overlays only `docker ps` filtered by the label `crosslink-agent=true` (`monitor.rs:289-321`). The container launch sets no such label, and podman is never queried.
- **`CI_FAILED`** exists only in the prompt; it is mapped to "failed".
- **Resume:** none. `--branch` reuses an existing worktree (`run.rs:139-143`); otherwise an unmerged existing branch is refused (`launch.rs:565-571`).

## Swarm: intended workflow

From `docs_src/guides/swarm.qmd`, which is laid out as "You say / do" beside "Agent executes" (`swarm.qmd:33-53`). The human speaks; a driver agent runs every command.

1. The human supplies a design doc. The driver runs `swarm init --doc`, then `plan-show`; the plan goes to the hub branch (`swarm.qmd:47-52`).
2. Optional budgeting: `swarm plan --budget-window`, `swarm config`, `swarm estimate` (`swarm.qmd:72-78`).
3. The driver runs `swarm launch 1`. "Each agent in the phase gets its own worktree, branch, crosslink issue, and agent identity" (`swarm.qmd:92`). The launched agents do the implementation.
4. The driver monitors with `swarm status`.
5. Merging agent branches is not a documented step. `swarm resume` prints "Merge X: review and merge … to dev" as a next action for the driver (`lifecycle.rs:503-505`). `swarm merge` exists but is absent from the guide.
6. The driver runs `swarm gate 1`: "The gate runs the project's full test suite" (`swarm.qmd:137`).
7. The driver runs `swarm checkpoint 1 --notes`, then `swarm launch 2`; `swarm resume` after an interruption.

**The review commands have no workflow documentation.** `review`, `fix`, `pipeline`, `review-continue`, `review-status`, `merge` and `trust-init` do not appear in the guide or in `docs_src/reference/commands.qmd:323-349`. The CHANGELOG describes `swarm review` as "parallel adversarial codebase exploration" and `swarm fix` as "parallel issue-to-agent fix execution" (`CHANGELOG.md:582-583`), and the original CLI help said "Launch parallel adversarial review agents" and "Launch parallel fix agents". The bundled crosslink-guide skill listed "swarm review # parallel review agents" until the 2026-08-16 refactor; it now lists only init, status, launch, gate and harvest. So the vsdd contract's phrase "documented upstream" overstates it.

## Swarm: subcommands, documented versus implemented

File references are under `src/commands/swarm/` unless fuller.

| Subcommand | What the code does | Launches agents | Gap against the docs |
|---|---|---|---|
| `init --doc` | Parses the doc and writes the plan and phase files to the hub (`init.rs:9-123`). Phases come from `## Phase:` or `## Layer:` sections, or `### Phase N` / `### Layer N` under `## Requirements` (`src/commands/design_doc.rs:89-94, 158-165`). One agent per top-level bullet, sub-bullets folded in. Fallbacks: flat Requirements bullets, then Acceptance Criteria, then the title; 8 agents per phase (`init.rs:132-176`). Adds a scaffold phase on empty repos. Refuses when a swarm is already active. | No | Docs say phases follow "dependency structure"; the code only chains each phase to the one before, and the `(parallel)` hint changes nothing (`init.rs:231-245`). Nothing validates the doc beyond a title. |
| `status`, `resume`, `sync-status`, `list`, `archive`, `reset`, `adopt` | Read hub phase files and probe each worktree's status file and tmux (`status.rs:240-287`). `resume` prints next actions for the driver. | No | README and guide advertise `swarm create` and `swarm switch`; neither exists (`src/main.rs:1657-1846`). One active swarm at a time. |
| `launch <phase>` | Calls `kickoff::run` once per planned agent (`lifecycle.rs:686-720`). Hard-coded: no container (`:706`), verify local (`:707`), 3600 s timeout (`:692`), no design doc and no doc path (`:714-715`). Model, effort and budget come from the swarm config. Optional per-phase template. | **Yes**, tmux only | Agents receive only their one-bullet description, never the design doc. |
| `launch --budget-aware` | Estimates, blocks on a "Block" recommendation, otherwise launches everything (`budget.rs:215-266`). | Yes | "Split" only warns and launches all agents anyway. |
| `config` | Writes window, model, effort and budget (`budget.rs:11-58`). | No | One setting per swarm. The agent entry has no per-agent dial (`types.rs:50-64`). |
| `plan`, `plan-show`, `estimate`, `harvest` | Wall-clock packing and estimates. `harvest` reads each worktree's self-reported report into a cost log (`budget.rs:268-361`). | No | Costs are durations, not tokens or dollars; the model is hard-coded "standard". |
| `gate <phase>` | Refuses while agents are unresolved, then runs the auto-detected test command (default `cargo test`) in the repo root and records pass or fail and test counts in the phase's hub file, unsigned (`lifecycle.rs:809-855`). | No | Matches the docs. No project-configurable command. Runs on whatever is checked out. |
| `checkpoint` | Requires a passed gate or `--force`; marks the phase complete (`lifecycle.rs:907-1026`). | No | Matches. |
| `merge` | Diffs every `.worktrees/*` against the base and applies each onto a new combined branch (`merge.rs:244-522`). | No | Undocumented. Takes all worktrees, not only this swarm's. |
| plan editing (`move`, `merge-phases`, `split-phase`, `remove-agent`, `reorder`, `rename-phase`) | Edit the plan (`edit.rs`). | No | The docs give different names and shapes (`swarm.qmd:239-254`). |
| `review` | Splits the repo's source files across N reviewer slots and writes `swarm/review-plan.json` (`review.rs:85-143`). | **No** | Nothing in the codebase reads that file. With `--file-issues` or `--fix` it prints "Review agents launched" having launched nothing (`review.rs:278`). There are no reviewer roles: partitions are by file, one mandate per run. |
| `review-continue`, `review-status` | Advance or print a local pipeline file. `review-continue` only flips a flag (`review.rs:582-592`). | No | — |
| consolidation (inside `review`) | Reads `swarm/review-findings-*.json` from the hub cache, de-duplicates, filters by trust model, files GitHub issues through `gh` (`review.rs:174-208, 293-311`; `src/findings.rs:117-145`). | No | Nothing in the repo writes those files and no doc defines their format. Issues go to GitHub, not the crosslink tracker. |
| `fix` | Fetches GitHub issues and writes `swarm/fix-plan.json` (`review.rs:489-580`). | **No** | `--max-agents` prints "Some will queue" (`:567`). `--budget-aware` prints "Budget checking not yet integrated" (`:574`). Nothing reads the plan. |
| `pipeline` | Walks a stage machine printing "[review] Would launch…", "[merge] Would merge…" (`src/pipeline.rs:236-350`). | **No** | Pure stub. |
| `trust-init` | Writes `swarm.toml`. | No | The docs name the command differently. |

**Tests:** about 1,300 lines of unit tests over serialisation, parsing and pure functions. The only CLI-level tests are two `swarm status` smoke tests; the init-and-launch test is ignored (`tests/smoke/lifecycle.rs:464-510`).

## Swarm: development activity

- **Introduced** 2026-03-06. The review system landed 2026-03-12, multi-swarm and plan editing 2026-03-18.
- **56 commits** touch the swarm sources; 40 of them are from March 2026.
- **Since April: 16 commits,** all lint fixes, repo-wide refactors or provider changes, with two exceptions, both this estate's: crosslink PR #77 (dials) and crosslink PR #80 (per-phase template), 2026-08-02.
- **Last substantive upstream-authored swarm commit:** 2026-03-30.
- **Kickoff over the same period:** 108 commits, with real activity in August (18) and September (7). All of September's are readiness or CI side-effects.
- **Markers:** the "Would …" pipeline, "not yet integrated", and one `unreachable!()` (`review.rs:291`). There are no TODO markers, so the stubs are silent.
- **Upstream tracker:** four issues mention swarm (crosslink#24 open; #57, #61, #62 closed). All four were filed from this estate, and the fixes were the fork's. No maintainer statement about swarm's status or roadmap was found.

## How kickoff, swarm, the orchestrator and mission control relate

- **Kickoff** is the only launcher. `kickoff::run` is called from the kickoff CLI, from `swarm launch` and from sentinel. Only the kickoff CLI exposes `--doc`, `--container`, `--verify`, `--template`, `--effort` and `--budget-usd` per dispatch.
- **Swarm** is a plan-and-state layer on the hub branch on top of kickoff.
- **The dashboard orchestrator** (`src/orchestrator/`) is a third mechanism with its own plan model: LLM decomposition, a DAG executor, state under `.crosslink/orchestrator/`. It never references swarm and never calls kickoff; stages are marked running or done through API calls. No feature work since April.
- **Mission control** (`crosslink mc`) is a tmux viewer over kickoff agents.
- **Sentinel** is a polling loop over fixed sources (GitHub labels, GitHub CI, internal hygiene). It files issues and dispatches kickoff agents with no dials and no template (`src/commands/sentinel/engine.rs:396-467`). It cannot run scheduled project commands. Its `max_concurrent_agents` is the only agent-count cap in crosslink.

## This estate's contributions and where each has landed

| Change | PR | Upstream develop | Upstream main | Fork develop | Installed binary |
|---|---|---|---|---|---|
| Template interpolation, `--template`, per-phase swarm templates (crosslink#62) | crosslink PR #80 | yes | yes | yes | yes |
| `--effort` and `--budget-usd`, the metadata record, swarm config dials (crosslink#61) | crosslink PR #77 | yes | yes | yes | yes |
| Side-effect-free dry run (#19), the TIMEOUT status (#60), `/design --continue` keeps the pipeline file (#56), agent hook enforcement aligned with the docs (#58) | crosslink PR #65 | yes | yes | yes | yes |
| Permission flag passed into containers (#59) | crosslink PR #63 | yes | yes | yes | yes |
| Permission flags on `plan` (#66) | crosslink PR #72 | yes | yes | yes | yes |
| Agent image toolchain, safe auth environment, build and publish (#9, #10, #75) | crosslink PRs #64, #76 | yes | yes | yes | yes |
| `container auth --image` | crosslink-fork PR #5 | no; in crosslink PR #103 | no | yes | yes |
| Worktree readiness, the 120 s lock bound, cache mounts, `GH_TOKEN` passthrough, frontier seeding | crosslink-fork PR #6 | no; in PR #103 | no | yes | yes |
| Frontier lineage gate | crosslink-fork PR #7 | no; in PR #103 | no | yes | yes |
| Remove the 30-minute readiness bound | crosslink-fork PR #8 (merged 2026-10-02) | no; in PR #103 | no | yes | **no** |

- The `agent.kickoff_template` and `no_template` keys themselves came from a third-party contribution (crosslink PR #44, backported as #78). PR #80 turned replacement into interpolation.
- crosslink PR #103 is open with no review decision; its head is `cc756de92` (checked 2026-10-02).
- **Two contributions were changed by later upstream refactors:** `plan --skip-permissions` now errors for Claude (`claude.rs:15-20`), and the template documentation added to the kickoff command file is no longer in the deployed skill.

## Open upstream issues that affect dispatch

crosslink#15 (init replaces a project's settings hooks), #18 (runs without a doc are invisible in the overview), #24 (`swarm review --doc` is the output path), #55 (plan agents stall), #99 (`core.hooksPath` binding), #101 (the published agent image is private), #102 and PR #103 (container kickoff on the readiness model), #104 (stale hooks and the 3 s work-check timeout), #105 (`init --update` never reconciles the ignore files), #106 (daemon exit). #19, #9 and #10 are still listed open although merged PRs fixed them.

## Fit for vsdd

**What vsdd needs from a dispatch vehicle, against kickoff:**

| Need | Verdict | Basis |
|---|---|---|
| Inject a computed composition | **Supported.** Write it to a file containing `{{built_prompt}}` and pass `--template`. | `run.rs:107-128`. A bad path silently yields the default prompt, so the caller must check the dry-run output or the written `KICKOFF.md`. |
| A different composition for each reviewer in a round | **Supported with a workaround:** one `kickoff run --template` per reviewer, each with its own worktree, branch and identity. Swarm cannot do it. | `lifecycle.rs:632-637`. Whether several agents can work the same issue was not determined. |
| A reviewer that cannot edit | **Not supported per dispatch.** Write, Edit and `Bash(git *)` are always allowed. Possible workaround: `--permission-mode plan`, untested; it is unknown whether such an agent can still write its status or post comments. | `prompt.rs:417-444`, `launch.rs:185-188` |
| Set and record model and effort per dispatch | **Supported.** The record is agent-writable, so the caller should copy it at launch. | `types.rs:43-73` |
| Cap spend | **Supported for Claude only,** as a dollar cap. No token cap, no agent-count cap. | `claude.rs:42-44`, `codex.rs:14-18` |
| Run headless in a container | **Supported on the fork and the installed binary.** Upstream needs PR #103 and a reachable image (crosslink#101). | `launch.rs:788-965` |
| Records an auditor can trust | **Not supported.** Workaround: the caller snapshots and hashes the prompt, metadata and criteria at launch, and hashes the transcript itself. | "Records" above |
| Detect never-started and stalled agents | **Partial.** A send failure and a timeout are recorded; an early crash leaves "running" until the wall clock. The stall watchdog is local only. | `launch.rs:761-775, 293`, `monitor.rs:147-166` |
| Run the project's own hooks or gates inside the agent | **Not supported for `.claude/settings.json` hooks.** What survives: tracked `hook-config.json`, tracked rules, the prompt text, and CI on the PR with `--verify ci`. There is no generic "run this command as a gate" slot anywhere in crosslink. | `merge.rs:216-217`, `init/mod.rs:1024-1029` |

**What the vsdd contract expects of swarm:**

| Contract expectation | Verdict |
|---|---|
| Phase 3 review rounds on `swarm review`, with a recorded manifest | **Not supported.** No reviewers are launched, there are no reviewer roles, no model or effort for review, no manifest; findings would go to GitHub issues. The usable pieces are consolidation and de-duplication, if vsdd launched its own reviewers and had them write `review-findings-*.json` into the hub cache; that file format is undocumented and inferred from `src/findings.rs:51-58, 131`. |
| The "Swarm live fire" acceptance criterion | **Cannot pass as written.** A live `swarm review` run finishes at once with a plan file and no findings. |
| "Gates are commands; `crosslink swarm gate` runs them" | **Not supported.** It runs only the detected test command and records only pass or fail and counts. |
| `swarm init --doc .design/build-plan.md` as the build entry | **Supported if used differently.** It is a heuristic splitter, not a gate. Each agent gets only its bullet, so the spec would have to be injected through a per-phase template. |
| Container execution | **Not supported through swarm.** Supported through `kickoff run --container` directly. |
| Model and effort per dispatch | **Per swarm only.** Different dials per agent need separate kickoff calls. |

The contract's "Swarm fallback" open question names a fallback of "swarm primitives (worktree launch plus gate) with vsdd supplying the review stage". The evidence supports taking the fallback rather than waiting for the live fire. Even then the gate primitive is the test command only, and container dispatch means calling kickoff directly. Whether and how to amend the contract is recorded on the knowledge page `content-delivery-assessment-2026-10-02`; nothing has been amended.

## Not determined or inferred

- **Nothing was run live.** In particular `swarm launch` under the readiness-era daemon is unverified.
- **Hooks in headless `-p` runs: verified 2026-10-02.** With kickoff's exact flags, session-start, prompt-submit, pre-tool and post-tool hooks all fire, their standard output reaches the model, and a pre-tool exit 2 blocks the call; the transcript records `hook_started` and `hook_response` events. Run in a scratch project, not through `crosslink kickoff run` (vsdd-cli#890; details on `content-delivery-assessment-2026-10-02`).
- **Maintainer intent for the review pipeline** is inferred from the March source comment and the old help text.
- **Upstream state after 2026-09-12** is unknown.
- **Agent identity (inferred):** `crosslink init` in the worktree may create an identity with a random id before kickoff's own check, in which case kickoff's named identity and automatic trust approval are skipped.
- **Agent signing keys in a container:** the keys live in the host's `.crosslink/`, which is not mounted; how in-container hub events get signed was not traced.
- **`kickoff plan` (inferred):** its tool list has no Write and Claude runs in plan mode, yet the prompt requires writing a plan file. This is consistent with crosslink#55.
- **Swarm merged-detection (inferred):** the probe looks for branches named `<slug>` or `swarm/<slug>` while agents use `feature/<slug>`, so it probably never fires (`status.rs:289-303`).
- **Not checked:** whether heartbeats are signed; whether an agent's key can push to hub paths other than its own; whether the dashboard front end launches agents for orchestrator stages.

## What changed from the 2026-08-02 version of this page

- **"The template fully replaces the built prompt", "the template is main-only", "plan never consults the template", "there is no effort field":** all false now. Templates interpolate, are on every branch, apply to `plan`, and effort and budget are flags.
- **"Swarm hard-codes the model to opus":** false now; the model comes from the swarm config. The 3600 s timeout is still hard-coded.
- **"One configured template gives every swarm agent an identical prompt":** resolved by interpolation and per-phase templates. Per-agent templates within a phase still do not exist, and neither does per-agent dial storage.
- **The launch command** is no longer an interactive `claude -- "$(cat …)"`; it is headless with a transcript tee.
- **`skip_permissions`, `permission_mode` and `agent_binary`** are no longer option fields; they became one execution-policy value. `--model` defaults to `standard`.
- **"Permission flags are not forwarded into containers"; crosslink#19 and #60 "not in develop":** all fixed and merged.
- **The requirement-to-code table (R1 to R5)** described work that has since merged and is dropped.
- **New on this page:** the review, fix and pipeline stubs; the fixed gate command; the settings-hooks replacement in worktrees; the records table; the contributions table.

## Related pages

`content-delivery-assessment-2026-10-02` (what to build on these mechanisms, and the four-domain review); `crosslink-integration-surfaces` (rules injection, hooks, MCP servers, signing); `container-vehicle-pilot-2026-09` (the eight container runs); `run-record-capability-inventory` (what the transcripts record); `attended-design-autonomous-execution` (the 2026-07-20 reading, with corrections).
