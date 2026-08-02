---
title: "Claude Code runtime-harness surface — verified current state (vsdd-cli #857 grounding)"
tags: ["reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-02
updated: 2026-08-02
---


## Design Specification

### 1. hooks

### What the surface is

Hooks are configured under the `hooks` key of settings.json (user, project `.claude/settings.json`, local, managed, plugin, and — new — skill/agent frontmatter). Shape: `{"hooks": {"<Event>": [{"matcher": "...", "hooks": [{handler}, ...]}]}}`. Handler types at v2.1.212: `command`, `http`, `mcp_tool`, `prompt` (small-model evaluation), and `agent` (subagent evaluation). Handler fields include `timeout` (default 600 s for command/http/mcp_tool, 30 s for prompt, 60 s for agent; UserPromptSubmit lowered to 30 s), `statusMessage`, `once`, `async` / `asyncRewake` (background with wake-on-exit-2), and `if` (permission-rule filter such as `Bash(git *)` with subcommand-aware matching).

Event inventory (grown well past the classic five):

- Session: `SessionStart` (matchers `startup|resume|clear|compact|fork`), `SessionEnd`, `Setup`.
- Turn: `UserPromptSubmit`, `UserPromptExpansion`, `Stop`, `StopFailure` (matcher = API error class).
- Tool loop: `PreToolUse`, `PermissionRequest`, `PermissionDenied`, `PostToolUse`, `PostToolUseFailure`, `PostToolBatch`.
- Subagents/tasks: `SubagentStart`, `SubagentStop` (matcher = agent type), `TaskCreated`, `TaskCompleted`, `TeammateIdle`.
- Compaction: `PreCompact` (matchers `manual|auto`, blockable), `PostCompact`.
- Environment: `ConfigChange` (blockable; matcher = settings source), `FileChanged` (watched files), `CwdChanged`, `InstructionsLoaded` (which memory/rules files loaded and why), `MessageDisplay`, `Notification`, `WorktreeCreate`/`WorktreeRemove`, MCP `Elicitation`/`ElicitationResult`.

Exit-code semantics: exit 0 = success, stdout JSON parsed on events that support it; exit 2 = blocking error, stderr used, blocks the action on blockable events (PreToolUse blocks the tool call; UserPromptSubmit blocks and erases the prompt; Stop/SubagentStop prevent stopping; PreCompact blocks compaction; PostToolUse cannot un-run the tool but stops before the next model call and surfaces stderr to Claude); other exit codes = non-blocking, logged. JSON decision control: universal fields (`continue`, `stopReason`, `systemMessage`, `suppressOutput`); top-level `{"decision": "block", "reason": ...}` on UserPromptSubmit/PostToolUse/Stop/ConfigChange/PreCompact and siblings; `PreToolUse` `hookSpecificOutput` with `permissionDecision: allow|deny|ask|defer`, `additionalContext`, and `updatedInput` (input rewriting); `PostToolUse` `updatedToolOutput`; `SessionStart` `additionalContext`, `initialUserMessage`, `watchPaths`, `sessionTitle`, `reloadSkills`. `${CLAUDE_PROJECT_DIR}` is available for path references.

### What vsdd binds to it

- **Deployed today (all crosslink-deployed, none vsdd-authored):** six Python payloads in `.claude/hooks/` wired by `.claude/settings.json` through inline `sh` wrappers that exit 2 when the payload file is missing — the fail-closed wiring the Conformance-at-action-time installed-artifact clause cites (vsdd-cli #658; the tracked half's loudness). Wiring: `PreToolUse` `Write|Edit|Bash` → `work-check.py` (blocks code-touching tools unless a crosslink issue is actively worked — the work-check / behavioral guard; also enforces commit/close comment discipline); `PreToolUse` `WebFetch|WebSearch` → `pre-web-check.py` (prompt-injection defense injection); `PostToolUse` `Write|Edit` → `post-edit-check.py` (stub/lint/test reminders); `PostToolUse` unmatched → `heartbeat.py` (throttled `crosslink heartbeat` — the staleness instrument Recorded review dispatch's never-started/stalled detection consumes); `SessionStart` `startup|resume` → `session-start.py` (crosslink context load + session auto-start); `UserPromptSubmit` → `prompt-guard.py` (injects `.crosslink/rules/` reminders per prompt).
- **Contract ambitions that bind here but are unbuilt:** the read-gate ("Availability is not activation": edits to governed files block until the governing documents' Reads are recorded this session — a `PreToolUse` deny with `permissionDecisionReason`); the action-time supplement-activation hook backstop (Deterministic composition); the hook-trace-log-versus-git-history audit (Conformance at action time falsification: "a governed edit absent from the hook trace log"); mdatron invocation "by hooks at action time"; and the post-compaction re-read discipline, for which the harness now provides exact seams: `PreCompact`/`PostCompact` events and the `SessionStart` `compact` matcher (see section 5).
- Enforcement-grade note (per the meta-harness ladder): settings.json hooks are friction/CI-adjacent, not harness-level restriction — `disableAllHooks` and settings edits are agent-reachable in this deployment; only managed settings would move them out of reach.

---

### 2. skills and commands

### What the surface is

**Custom commands have been merged into skills.** A file at `.claude/commands/<name>.md` and a skill at `.claude/skills/<name>/SKILL.md` both create `/<name>` and support the same frontmatter; skills add a directory for supporting files and richer activation control. Locations and precedence: enterprise (managed) > personal `~/.claude/skills/` > project `.claude/skills/` > plugin (namespaced `plugin:skill`); nested `.claude/skills/` below the working directory load lazily when files there are touched (directory-qualified names). Live change detection watches skill directories mid-session.

Frontmatter (all optional; `description` recommended): `name`, `description` + `when_to_use` (combined listing text truncated at 1,536 chars — this listing is what the model matches against), `argument-hint`, `arguments`, `disable-model-invocation` (user-only invocation; description NOT in context), `user-invocable: false` (model-only; hidden from `/` menu), `allowed-tools` (turn-scoped permission grant, cleared on next user message), `disallowed-tools`, `model`, `effort` (`low|medium|high|xhigh|max`), `context: fork` + `agent` + `background` (run the skill in a subagent), `hooks` (skill-scoped lifecycle hooks), `paths` (glob-triggered automatic activation), `shell`. Content supports `$ARGUMENTS`/`$N`/named substitution, `${CLAUDE_SESSION_ID}`, `${CLAUDE_EFFORT}`, `${CLAUDE_SKILL_DIR}`, `${CLAUDE_PROJECT_DIR}`, and dynamic context injection via `` !`command` `` (shell output substituted before the content reaches the model).

Invocation mechanics — the load-bearing distinctions:

- The **listing** (descriptions only) is always in context for model-invocable skills; the **body** loads only on invocation and then persists for the rest of the session (re-invocation deduplicates when content is unchanged).
- Model invocation by description match is a **judgment**; `/name` typing and the Skill tool are **recorded invocations** — the transcript's `attributionSkill` field distinguishes an invocation from a mere `Read` of the skill file (see the run-record-capability-inventory page).
- **Injection paths exist natively:** a subagent's `skills:` frontmatter preloads full skill content at startup (construction, not judgment); `SessionStart` hook `additionalContext` injects at session entry; `paths:` frontmatter activates on matching file work.
- Compaction: invoked skill bodies are re-attached after compaction under a budget — first 5,000 tokens per skill, 25,000 tokens total, oldest dropped — and the skill **listing itself is not re-injected after compaction**; only invoked skills survive.

### What vsdd binds to it

This is the surface the availability-is-not-activation discipline (Conformance at action time) turns on: "a skill in the runtime harness's listing loads only by invocation or by the model matching its description — a judgment, not a mechanism"; required context arrives injected-by-hook or gated-on-recorded-Read, never by description match. The contract's act-to-affordance map clause assigns this supplement the duty to "document the runtime harness's summoning mechanics." Deployed today: 42 command files in `.claude/commands/` — 14 crosslink workflow commands, 10 vsdd phase primers (`vsdd-phase-1a` … `vsdd-phase-6`), 18 vsdd domain prompts (`vsdd-domain-*`) — plus 18 skill directories in `.claude/skills/` (crosslink's workflow set and the operator's Rust-discipline set; none vsdd-generated — the Generated-context requirement is unbuilt). The vsdd primer/domain files carry **mdatron schema frontmatter** (`schema_class`, `primer_id`, `phase`, ...), not harness frontmatter: no harness-recognized `description` field, so the listing falls back to first-paragraph text, and none of the invocation-control fields (`disable-model-invocation`, `user-invocable`, `paths`, `allowed-tools`, `context: fork`) are used anywhere in the vsdd set.

---

### 3. subagent dispatch

### What the surface is

The dispatch primitive is the **Agent tool** (the docs' "subagents"; earlier materials call it the Task tool). Custom agent types are markdown files with YAML frontmatter at `.claude/agents/*.md` (project), `~/.claude/agents/` (user), managed settings, plugin `agents/`, or session-scoped JSON via `--agents`. Frontmatter: `name` and `description` required; `tools` (allowlist; inherits all if omitted), `disallowedTools`, `model` (`sonnet|opus|haiku|fable`, full model id, or `inherit`; per-invocation `model` parameter also exists; resolution: `CLAUDE_CODE_SUBAGENT_MODEL` env > per-invocation parameter > frontmatter > inherit), `effort` (`low|medium|high|xhigh|max`, overrides session), `permissionMode`, `maxTurns`, `skills` (full-content preload at startup), `mcpServers`, `hooks` (agent-scoped), `memory` (own persistent auto-memory scope), `background`, `isolation: worktree`, `color`, `initialPrompt`. The body is the subagent's entire system prompt (it does not receive the main Claude Code system prompt). `--agent <name>` runs the main session as an agent type.

Runtime behavior: subagents run in their own context window; background by default since v2.1.198 (permission prompts surface in the parent session naming the asker); foreground when the result is needed immediately; a completed background agent reports via a completion notification the parent must wait for. Depth-limited nesting; conversation forks inherit the parent context. Subagent final reports are scanned (v2.1.210+) for instruction-shaped patterns (backslash insertion, marker lines) — a taint marker, not a filter. Invocation: automatic delegation by `description` match, explicit naming, or @-mention (guaranteed dispatch).

### What vsdd binds to it

- **Recorded review dispatch** binds directly: "reviewer roles are tool-restricted; a critic role cannot edit" is exactly `tools:`/`disallowedTools` frontmatter (the Thermite critic-lacks-Edit pattern, natively expressible); per-lens model tier and effort dials (Deterministic composition's review config) are exactly the `model` + `effort` frontmatter and the per-invocation `model` parameter. **The contract's "per-lens effort for cold dispatches awaits the dispatch-parameter seam" is satisfied on the Agent-tool path at v2.1.212** (agent `effort` frontmatter; skill `effort`; recorded per-request `effort` in the transcript) — it remains missing only on the crosslink-kickoff path (upstream dollspace-gay/crosslink #61; the dial-less-crosslink-dispatch deviation entry). The manifest-injection seam ("kickoff's prompt assembly is the candidate") likewise has an Agent-tool analog: the dispatch prompt plus `skills:` preload is manifest-carried context by construction.
- **What the transcripts record:** see the crosslink knowledge page `run-record-capability-inventory` (the WAS-side oracle) — per-agent `agent-<id>.jsonl` transcripts with `attributionSkill`, `effort`, usage by cache class, and full tool inputs; `agent-<id>.meta.json` with `agentType`; `journal.jsonl` lifecycle lines; delegation graph derived from `parentUuid`/`sourceToolAssistantUUID`/`subagents/` (no `spawnDepth` field exists); `effort` present on Agent-tool dispatches and absent on kickoff-path records. That page, not this one, is the authority on record fields, token semantics, and the provenance discipline.
- **Deployed today: nothing.** `.claude/agents/` does not exist in this repo; no vsdd domain lens or reviewer role is defined as an agent type. The 18 domain prompts live only as command files (section 2), hand-loaded during the bootstrap interim the phases-dispatched keystone marks.

---

### 4. settings and statusline

### What the surface is

Settings files and precedence: managed policy > CLI flags > `.claude/settings.local.json` > `.claude/settings.json` > `~/.claude/settings.json`; permission rules merge across scopes, other keys override by precedence. Load-bearing keys: `permissions` (`allow`/`ask`/`deny` arrays of `Tool(specifier)` rules — **`allowedTools` and `ignorePatterns` are deprecated legacy keys** migrated to `permissions.allow`/`deny`), `additionalDirectories`-class access grants, `hooks`, `env`, `model` / `effortLevel` / `availableModels` / `fallbackModel`, `enableAllProjectMcpServers` / `enabledMcpjsonServers` / `disabledMcpjsonServers`, `disableAllHooks`, `sandbox`, `autoCompactEnabled`, `autoMemoryEnabled` / `autoMemoryDirectory`, `cleanupPeriodDays` (session-file retention, default 30 days), `fileCheckpointingEnabled`, `apiKeyHelper`, `attribution`, `statusLine`. `/status` and `/doctor` report resolved sources and invalid entries.

statusLine: `{"statusLine": {"type": "command", "command": "...", "padding": 0, "refreshInterval": ...}}`. The command receives **JSON session data on stdin** and whatever it prints is displayed. Refresh: once at session start/resume, then on each new assistant message, `/compact` completion, permission-mode change, vim-mode toggle, and the optional `refreshInterval` timer; 300 ms debounce; an in-flight script is cancelled by a newer trigger; runs locally, consumes no tokens; terminal size via `COLUMNS`/`LINES` env. Stdin payload fields: `model.{id,display_name}`, `cwd`, `workspace.{current_dir,project_dir,added_dirs,git_worktree,repo.{host,owner,name}}`, `cost.{total_cost_usd (client-side estimate),total_duration_ms,total_api_duration_ms,total_lines_added,total_lines_removed}`, `context_window.{total_input_tokens,total_output_tokens,context_window_size,used_percentage (input-only formula),remaining_percentage,current_usage{input_tokens,output_tokens,cache_creation_input_tokens,cache_read_input_tokens}}`, `exceeds_200k_tokens`, `fast_mode`, `effort.level`, `thinking.enabled`, `rate_limits.{five_hour,seven_day}.{used_percentage,resets_at}` (subscription auth only), `session_id`, `session_name`, `prompt_id`, `transcript_path`, `version`, `output_style.name`, `vim.mode`, `agent.name`, `pr.{number,url,review_state}`, `worktree.*`. Multi-line output renders as multiple rows; ANSI colors and OSC 8 links supported.

### What vsdd binds to it

The **Status requirement** binds here: `vsdd status --statusline` is the one-line segment for "persistent display surfaces such as the runtime harness's statusLine"; the segment consumes **no stdin** ("the runtime harness's session JSON stays cataloged but unconsumed" — the deferred-observability seam); refresh is "the runtime harness's own events" (the list above is that event set); the Install requirement's offered-never-imposed statusLine wiring (read-modify-write, wiring-outcome enumeration, drift refusal on an existing entry) writes exactly this settings key. The runtime-harness settings file is a named Trust-boundaries surface (schema-tolerant probe, fuzz scope) and a named reference surface of the installed-artifact manifest ("the statusline command path"). The stdin payload's `cost.total_cost_usd` being a client-side estimate, and `rate_limits` appearing only under subscription auth, corroborate Cost-is-knowable's dollars-are-a-projection ruling. Deployed today: the wiring script template exists at `templates/statusline/vsdd-statusline.sh` (stdin passes through unconsumed; multi-repo set from `~/.config/vsdd/statusline.yaml`), but **this repo's settings carry no `statusLine` entry** — nothing is wired; and the repo's `allowedTools` key (`Bash(tmux *)`, `Bash(git worktree *)`) rides the deprecated legacy key rather than `permissions.allow`.

---

### 5. session mechanics

### What the surface is

**Compaction.** Auto-compaction runs near the context limit (`autoCompactEnabled`); `/compact` (optionally with focus instructions) runs it manually. What survives (docs' own table): system prompt and output style unchanged; project-root CLAUDE.md and unscoped `.claude/rules/` re-injected from disk; auto memory re-injected; `paths:`-scoped rules and nested CLAUDE.md **lost** until a matching file is read again; invoked skill bodies re-attached under the 5,000-per-skill / 25,000-total budget, oldest dropped; the skill listing is not re-injected; hooks unaffected (code, not context). Hook seams: `PreCompact` (blockable), `PostCompact`, `SessionStart` matcher `compact`.

**Memory.** CLAUDE.md hierarchy: managed policy file, `~/.claude/CLAUDE.md`, project `./CLAUDE.md` or `./.claude/CLAUDE.md`, gitignored `CLAUDE.local.md`; loaded by walking up from the working directory, nested files on demand; `@path` imports (4-hop depth; external imports need a one-time approval dialog); `.claude/rules/*.md` with optional `paths:` frontmatter for path-scoped loading; `~/.claude/rules/` for user scope. Auto memory: per-repo directory `~/.claude/projects/<project>/memory/` shared across worktrees, `MEMORY.md` index loaded every session (first 200 lines or 25 KB), topic files read on demand; machine-local; subagents get their own via the `memory` frontmatter field; not loaded into subagents (forks excepted).

**Background tasks.** `Bash` `run_in_background` for detached commands; background subagents by default (section 3); `/tasks` list; `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` kill switch. Session transcripts/checkpoints retained per `cleanupPeriodDays` (default 30).

### What vsdd binds to it

- The **post-compaction stale-read seed** (Fixture corpus: "an act after compaction against a pre-compaction Read record, gated to re-read") binds here: the read-gate's guarantee is entry-time and honestly bounded ("content evicted by compaction ... post-compaction re-entry re-reads" — Availability is not activation). The harness provides the exact mechanization triggers (`PostCompact` hook, `SessionStart` `compact` matcher) and the exact hazard (governed content loaded by invocation or Read is summarized away; only project-root CLAUDE.md, unscoped rules, auto memory, and budget-capped skill bodies come back).
- The **Deterministic phase answer** contract deliberately rides crosslink session breadcrumbs re-injected on resume, not harness memory; this repo has **no project CLAUDE.md and no `.claude/rules/`** — per-prompt context arrives via the crosslink `prompt-guard` hook instead. That is a real divergence from the harness's default context-delivery path, currently un-recorded as such.
- `cleanupPeriodDays` (30-day session-file retention) bounds the run-record substrate the records-based insight engine (Cost is knowable) and the conformance verifier read — records the reader depends on expire by default; the corroboration keystone's server-synced copies (crosslink) are the durable half.
- Background subagents and `run_in_background` are the in-session autonomous vehicles; crosslink kickoff (tmux/container) is a **detached vehicle outside these session mechanics** — the dispatch-primitive dependence (effort recorded vs absent) documented on the run-record-capability-inventory page.

---

### 6. mcp

### What the surface is

Configuration scopes: **project** — `.mcp.json` at the repo root, version-controlled; **local** (default for `claude mcp add`) and **user** — both stored in `~/.claude.json`; managed/enterprise via policy. Precedence local > project > user, whole-entry, no field merge. Transports: stdio (command + args + env), http, sse; `${VAR}` env expansion (with `${CLAUDE_PROJECT_DIR:-.}`-style defaults needed in project entries). Tool names: `mcp__<server>__<tool>` (plugin form `mcp__plugin_<plugin>_<server>__<tool>`), usable in permission rules, hook matchers, skill `allowed-tools`, and subagent `tools`.

Trust and consent: project-scoped servers from `.mcp.json` require explicit approval before first use; approvals persist via `enableAllProjectMcpServers` (blanket), `enabledMcpjsonServers` / `disabledMcpjsonServers` (named), reset by `claude mcp reset-project-choices`. Since v2.1.196, approvals committed to a repo's own `.claude/settings.json` are **ignored in an untrusted folder** (a cloned repo cannot approve its own servers; workspace-trust dialog first); since v2.1.207 an untracked `settings.local.json`'s approvals are also trust-gated. Failure surfacing: since v2.1.205 a failed server connection is reported to Claude (including in ToolSearch results) — but only on the tool-search path, and it is advisory, not fail-closed. Tool schemas are deferred by default (ToolSearch; `ENABLE_TOOL_SEARCH`). Output limits: 25,000-token default cap per tool result (`MAX_MCP_OUTPUT_TOKENS` raises it), fixed 10,000-token warning.

### What vsdd binds to it

Trust boundaries and the Dispatch preflight name this surface directly: "project-server trust and load state (the silent-non-load degradation Trust boundaries names, bound here)" is a preflight member with a fail-closed three-valued check — the harness's own v2.1.205 reporting narrows but does not close that gap (advisory, path-dependent), so the preflight member stands. The workspace-trust gating is also the consent surface the attended/autonomous split cares about: an autonomous vehicle must be launched with trust and approvals pre-granted or its servers silently stay pending. Deployed today: `.mcp.json` declares three stdio servers — `crosslink-agent-prompt`, `crosslink-knowledge`, `crosslink-safe-fetch` (payload scripts at `.claude/mcp/*.py`, run via `uv`); approval is doubled (`enableAllProjectMcpServers: true` in project settings AND the three named in `settings.local.json`'s `enabledMcpjsonServers`). The safe-fetch server is the inbound hidden-Unicode peer the Terminal output safety requirement names ("crosslink governs what an agent reads, this strips what the tool emits").

---

### 7. verified current baseline of this repo

What `.claude/` and adjacent wiring actually deploy in this working tree (verified 2026-08-01):

| Surface | Deployed state |
|---|---|
| `.claude/settings.json` | Crosslink-managed hook wiring for 4 event types (PreToolUse ×2 matchers, PostToolUse ×2, SessionStart `startup\|resume`, UserPromptSubmit) with fail-closed missing-payload wrappers; `enableAllProjectMcpServers: true`; **deprecated** `allowedTools` key (`Bash(tmux *)`, `Bash(git worktree *)`); **no `statusLine` entry**; no permissions.allow/deny; no Stop/SubagentStop/PreCompact/PostCompact/SessionEnd/ConfigChange hooks |
| `.claude/settings.local.json` | `enabledMcpjsonServers`: the three crosslink servers |
| `.claude/hooks/` | 6 crosslink payloads + shared `crosslink_config.py` (work-check, prompt-guard, pre-web-check, post-edit-check, session-start, heartbeat); zero vsdd-authored hooks |
| `.claude/commands/` | 42 files: 14 crosslink workflow commands, 10 vsdd phase primers, 18 vsdd domain prompts; vsdd files carry mdatron schema frontmatter, no harness frontmatter fields |
| `.claude/skills/` | 18 directories (crosslink workflow set + operator Rust-discipline set); none vsdd-generated |
| `.claude/agents/` | **Does not exist** — no custom agent types |
| `.claude/mcp/` + `.mcp.json` | 3 stdio server payloads; project config at repo-root `.mcp.json` (NOT `.claude/mcp.json`) |
| `.githooks/pre-commit` | `mdatron verify` on staged markdown/schema/pattern files; fail-closed on missing mdatron; per-clone `git config core.hooksPath .githooks` |
| Memory | No project CLAUDE.md, no `.claude/rules/`; user auto-memory in use (`~/.claude/projects/<project>/memory/`) |
| vsdd-shipped templates | `templates/statusline/vsdd-statusline.sh` (unwired), `templates/registry/` (installed-artifact-manifest, statusline-data, act-to-affordance-map, deviation-registry, vocabulary, ...), `templates/.github/workflows/` (vsdd-verify, vsdd-observe-pr-body), `templates/DESIGN.md.vsdd-template` |
| `.vsdd/` | `registry/deviation-registry.yaml` (live self-governance instance), `events/` (one legacy file) |

### Supplement staleness — `supplements/claude-code-cli.md` vs v2.1.212

Itemized against current docs and this tree (this is the catalog-update input for the Methodology-rewrite requirement, which already names the statusLine/effort/model-tier catalog as "catalogued by the rewrite, neither claimed as already present"):

1. **Wrong MCP path.** The supplement states "MCP servers at `.claude/mcp.json`". The project MCP config surface is `.mcp.json` at the repo root — which is what this repo itself deploys; `.claude/mcp/` holds server payload scripts, not config. Factually wrong about both the harness and the tree.
2. **Deprecated settings key taught as discipline.** "UNION-merge for `allowedTools`" — `allowedTools` is a deprecated legacy key; the current surface is `permissions.allow/ask/deny` with merge-across-scopes semantics. The repo's own settings still ride the deprecated key.
3. **Commands catalogued without the skills merge.** The supplement documents `.claude/commands/<name>.md` only. Custom commands have been merged into skills; the invocation-control frontmatter (`disable-model-invocation`, `user-invocable`, `paths`, `allowed-tools`, `context: fork`, per-skill `model`/`effort`) — the mechanics the availability-is-not-activation discipline and the act-to-affordance map's summoning-mechanics duty need — is entirely uncatalogued.
4. **statusLine, effort surface, model-tier surface absent** — the exact gap the Methodology-rewrite requirement records; still open. Same for the hook event inventory (the supplement never lists events, exit-code semantics, or JSON decision control).
5. **"Task tool" naming.** The dispatch surface is now presented as the Agent tool/subagents (with `.claude/agents/` frontmatter incl. `effort` — which materially changes the contract's "dispatch-parameter seam" status; see section 3). The transcript facts in the supplement remain correct and aligned with the run-record-capability-inventory page.
6. **Unverifiable capability names.** "`CronCreate`", "`PushNotification`", "`RemoteTrigger`" appear in the Platform Engineer extensions; none are found in current official docs. The current named surfaces are scheduled tasks/routines, the Notification hook event, and Remote Control push notification settings. These entries fail the verify-before-asserting bar as written.
7. **Minor:** the effort level set now includes `max` (supplement's observed set stops at `xhigh`); prompt-cache TTL claims (`ENABLE_PROMPT_CACHING_1H`) were not re-verified this pass.
8. **Still accurate:** the AI-Engineer run-record extensions (usage-by-cache-class, `attributionSkill`, delegation-graph derivation, no `spawnDepth`, effort's dispatch-primitive dependence, provenance tags, cost-is-a-projection), the hook thin-wrapper pattern, plan-vs-API auth split, and the self-noted absence of a runtime-determined-always-on activation field.

**Verdict: stale in load-bearing places** — one factual error (MCP path), one deprecated-key discipline, the contract-named statusLine/effort/model-tier catalog still missing, the skills/commands merge and hook event surface uncatalogued, and three capability names that do not resolve against current docs. The run-record material is current.

