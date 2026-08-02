---
title: "kickoff-swarm-dispatch-pipeline"
tags: ["reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-02
updated: 2026-08-02
---


## Design Specification

### design specification

### 1. prompt assembly

### `build_prompt` (`commands/kickoff/prompt.rs:186-303`)

Entry: `run()` calls it at `commands/kickoff/run.rs:114` ("step 5"), then writes the result verbatim
to the worktree's `KICKOFF.md` at `run.rs:117-118`. The agent receives the file content as its prompt
argument — `claude ... -- "$(cat KICKOFF.md)"` (`launch.rs:212-214` local, `launch.rs:787-789`
container). Sections, in emission order:

| Section | Source | Condition |
|---|---|---|
| Header: title, `## Context` (issue `#{issue_id}`, branch, verify level), `## Feature Description`, `## Environment` (worktree/hub explainer), `## Blocked Actions` (git policy list), `## Instructions` steps 1-11 (agent init, session start/work, CLAUDE.md, explore, knowledge search, plan/decision/observation comments, sync, intervene) | format string `prompt.rs:194-258` | always |
| `## Design Specification` (Summary, Requirements, Acceptance Criteria as `- [ ]`, Architecture, Out of Scope, unknown sections) | `build_design_doc_section`, `commands/design_doc.rs:361-407` | `opts.design_doc.is_some()` (`prompt.rs:261-262`) |
| `## Open Questions — Escalation Required` | `build_open_questions_escalation`, `design_doc.rs:412-434` | doc has open questions (`prompt.rs:263-265`) |
| `## Design Document — Canonical Input` (read-only stanza, GH#580) | `build_canonical_doc_stanza`, `prompt.rs:311-332` | `opts.doc_path.is_some()` (`prompt.rs:270-272`) |
| `## Plan Context` (subtasks/assumptions/advisory gaps from `.design/<slug>.plan.json`) | `build_plan_context_section`, `prompt.rs:338-416`; path via `plan_path_for_doc` (`pipeline.rs:83-89`) | plan JSON exists (`prompt.rs:276-281`) |
| Steps 10-15: test cmd, lint cmds, result comment, `/commit` loop | `build_test_lint_instructions`, `prompt.rs:8-45`; commands from `detect_conventions` (`helpers.rs:221`) | always (`prompt.rs:283`) |
| `### CI Verification` steps 16-17 (push, draft PR, poll `gh run list`, max 5 retry cycles, `CI_FAILED` sentinel) | `build_ci_verification_section`, `prompt.rs:48-69` | verify ∈ {Ci, Thorough} (`prompt.rs:285-287`) |
| `### Adversarial Self-Review` step 18 | `build_adversarial_review_section`, `prompt.rs:72-90` | verify == Thorough (`prompt.rs:289-291`) |
| `### Spec Validation & Reporting` (validate `.kickoff-criteria.json`, write `.kickoff-report.json` schema) | `build_reporting_section`, `prompt.rs:96-162` | doc has acceptance criteria (`prompt.rs:293-298`) |
| `### Final Steps` (checklist, `crosslink sync`, session end, write `DONE` to `.kickoff-status`) | `build_final_steps_section`, `prompt.rs:165-183` | always (`prompt.rs:300`) |

User/design-doc data enters at exactly three points: `opts.description` (interpolated into title +
Feature Description, `prompt.rs:195/205/254`), the parsed `DesignDoc` (struct `design_doc.rs:17-29`,
parsed by `parse_design_doc` `design_doc.rs:47` from the `--doc` file read in `mod.rs:68-78`), and
the prior plan JSON. Everything else is tool-authored text.

Criteria extraction: `extract_criteria` (`helpers.rs:75-120`) converts `doc.acceptance_criteria` into
`CriteriaFile` (`types.rs:44-50`), honoring explicit `AC-N:` prefixes via `parse_criterion_id` and
auto-numbering the rest; `run()` serializes it to `.kickoff-criteria.json` (`run.rs:121-130`).

Allowed tools: `build_allowed_tools` (`prompt.rs:419-463`) = static base + `Bash(gh *)`/`Bash(sleep *)`
for CI/Thorough + conventions (`detect_conventions`, `helpers.rs:221`: Rust→cargo, Node→npm/npx,
Python→uv/pytest, Go) + explicit `kickoff.allowed_tools` from `hook-config.json`
(`read_kickoff_allowed_tools`, `helpers.rs:200-218`, wired at `run.rs:108-111`, GH#584).

### `build_plan_prompt` (`commands/kickoff/plan.rs:37-134`)

Emits: `# KICKOFF PLAN: Gap Analysis — <title>`, `## Context` (optional issue line + read-only mode
notice, `plan.rs:42-49`), the same `build_design_doc_section` + open-questions escalation
(`plan.rs:51-55`), `## Analysis Instructions` (6-step gap analysis + `.kickoff-plan.json` JSON schema,
`plan.rs:57-118`), and Final Steps with an optional copy-to-`.design/<slug>.plan.json` instruction
(`plan.rs:121-131`). Written to `PLAN_KICKOFF.md` (`plan.rs:187-188`); launched read-only via
`build_allowed_tools_plan` (`plan.rs:14-34`) and `build_agent_command(..., "PLAN_KICKOFF.md", ...)`
(`plan.rs:234-245`) — plan mode never skips permissions (`plan.rs:242-244`). **Plan mode never
consults the template seam**, on either branch (PR #44 touched `plan.rs` only to add `agent_binary`).

### 2. the `agent.kickoff_template` seam (main-only; decisive for gh#62)

**Branch status**: commit `26ee1885` (merge of PR #44, `3db234dc`) is reachable from `main` and
`upstream/main` only — `git branch -a --contains 26ee1885` lists no `develop`. On `develop` @
`6b4f736f`, `run.rs:113-114` is the plain `let prompt = build_prompt(...)` with no template check.

On `main`, step 5 becomes (`run.rs:113-119` @ `26ee1885`):

```rust
let prompt = if crate::utils::read_no_template(crosslink_dir) {
    String::new()
} else if let Some(custom) = crate::utils::read_kickoff_template(crosslink_dir) {
    custom
} else {
    build_prompt(opts, issue_id, &branch_name, &conventions)
};
```

`read_kickoff_template` (`utils.rs:26` @ main) reads the `agent.kickoff_template` path from
`hook-config.json`, resolves relative paths against `.crosslink/`, and returns the **file content
verbatim**. `read_no_template` (`utils.rs:50` @ main) reads `agent.no_template: bool` and yields an
empty prompt. `read_agent_binary` (`utils.rs:7` @ main) is the third key (`agent.binary`, default
`"claude"`), threaded via the new `KickoffOpts.agent_binary` / `PlanOpts.agent_binary` fields.

**Verdict: the template FULLY REPLACES the built prompt.** No interpolation, no placeholder
substitution, no append — issue id, branch, description, conventions, verify steps, and the design-doc
sections are all discarded when a template is configured. Downstream artifacts survive unchanged
(`.kickoff-criteria.json`, `.kickoff-metadata.json`, `.kickoff-doc.json`, `mark_running` — `run.rs`
steps 6b-8b run regardless), and the template still lands in `KICKOFF.md`, so R5's audit record holds.
But the prompt half is all-or-nothing — exactly the R1 complaint.

### 3. `kickoffopts` / `planopts` and the flag plumbing

`KickoffOpts<'a>` (`types.rs:81-100`), develop:

| Field | Type | CLI default (main.rs) |
|---|---|---|
| `description` | `&str` | positional (`main.rs:1655-1656`) |
| `issue` | `Option<i64>` | `--issue` (`main.rs:1658-1659`) |
| `container` | `ContainerMode` (`types.rs:14-22`) | `--container "none"` (`main.rs:1661-1662`) |
| `verify` | `VerifyLevel` (`types.rs:25-33`) | `--verify "local"` (`main.rs:1664-1665`) |
| `model` | `&str` | `--model "opus"` (`main.rs:1667-1668`) |
| `image` | `&str` | `DEFAULT_AGENT_IMAGE` (`types.rs:11`, `main.rs:1670-1671`) |
| `timeout` | `Duration` via `parse_duration` (`types.rs:336-365`) | `--timeout "1h"` (`main.rs:1673-1674`) |
| `dry_run` | `bool` | `--dry-run` (`main.rs:1676-1677`) |
| `branch` | `Option<&str>` | `--branch` (`main.rs:1679-1680`) |
| `quiet` | `bool` | global `-q` |
| `design_doc` | `Option<&DesignDoc>` | parsed from `--doc` (`mod.rs:68-78`) |
| `doc_path` | `Option<&str>` | `--doc` (`main.rs:1682-1683`) |
| `skip_permissions` | `bool` | `--skip-permissions` (`main.rs:1690-1691`) |
| `permission_mode` | `Option<&str>` | `--permission-mode`, 6 values, `conflicts_with skip_permissions` (`main.rs:1700-1708`, GH#603) |

**There is no `effort` field** (gh#61 confirmed). Main adds only `agent_binary: String`
(`types.rs:97-100` @ `26ee1885`).

Threading: clap (`main.rs:1652-1708`) → `commands::kickoff::dispatch` builds `KickoffOpts`
(`mod.rs:79-94`; also `dispatch_launch --run` at `mod.rs:265-280` and wizard Run at `mod.rs:347-362`)
→ `run()` → `launch_local(worktree, session, opts.model, allowed_tools, opts.timeout,
preflight.timeout_cmd, sandbox, crosslink_dir, opts.skip_permissions, opts.permission_mode)`
(`run.rs:203-214`) → `build_agent_command` (`launch.rs:167-223`) which emits
`timeout <secs>s [sandbox] env -u CLAUDECODE [CLAUDE_CONFIG_DIR=..] claude [--permission-mode X |
--dangerously-skip-permissions] --model <model> --allowedTools <list> -- "$(cat KICKOFF.md)"`
(permission resolution `launch.rs:193-199`, command `launch.rs:212-214`). Container path:
`run.rs:240-251` → `launch_container` (`launch.rs:666-803`), model/timeout in the `bash -c` at
`launch.rs:787-789`; permission flags are **not** forwarded into containers.

`PlanOpts<'a>` (`types.rs:212-221`): `doc`, `doc_path`, `model` (default `"opus"`, `main.rs:1739-1740`),
`timeout` (default `"30m"`, `main.rs:1742-1743`), `dry_run`, `issue`, `quiet`. Built at `mod.rs:120-128`
(plan subcommand), `mod.rs:230-238` (`--plan`), `mod.rs:315-323` (wizard).

### 4. swarm dispatch (`commands/swarm/lifecycle.rs`)

`launch` (`lifecycle.rs:677-792`) loads the phase from the hub, filters `AgentStatus::Planned`
(`lifecycle.rs:697-703`), and per agent builds `KickoffOpts` (`lifecycle.rs:729-744`) with:
`description` = the agent's per-bullet `phase.agents[idx].description` (`lifecycle.rs:725/730`),
`container: None` (732), `verify: Local` (733), **`model: "opus"` hardcoded (`lifecycle.rs:734`)**,
`timeout: 3600s` hardcoded (736), `skip_permissions: false` / `permission_mode: None` (742-743) — then
calls `kickoff::run(crosslink_dir, db, writer, &opts)` (`lifecycle.rs:746`). Success flips the agent
to Running and records `feature/<compact_name>` (`lifecycle.rs:748-751`); results are committed back
to the hub (`lifecycle.rs:762-767`). `launch_retry_failed` (`lifecycle.rs:271`) re-enters the same path.

**R3 hazard — verified.** Swarm rides the exact same `run()`; on main the template check
(`run.rs:113-119` @ `26ee1885`) sits upstream of `build_prompt` and reads a single repo-global key
from `crosslink_dir/hook-config.json`. Every agent in a wave shares that `crosslink_dir`, so one
configured template replaces every agent's prompt with identical content — the per-bullet
`description` (the only thing differentiating wave agents, injected at `prompt.rs:195/205` only via
`build_prompt`) never reaches any prompt. Issue binding and branch differ per agent but are likewise
absent from a template prompt. The hazard is structural, not hypothetical.

### 5. `pipeline.json` lifecycle and `.kickoff-*` sidecars

Sidecar per design doc: `.design/<slug>.pipeline.json` (`pipeline_path_for_doc`, `pipeline.rs:72-78`).
Shapes: `PipelineState { schema_version, design_doc, doc_hash, stage, plans, runs }`
(`pipeline.rs:10-20`); `PlanRecord { agent_id, worktree, started_at, completed_at?, status,
blocking_gaps, advisory_gaps, plan_file? }` (`pipeline.rs:24-37`); `RunRecord { agent_id, worktree,
issue_id?, started_at, completed_at?, status }` (`pipeline.rs:41-50`).

- `ensure_pipeline_state` (`pipeline.rs:127-129`) → `create_initial_pipeline` (`107-123`) writes
  `stage:"designed"` with a `compute_doc_hash` SHA-256 (`53-58`; staleness via `is_plan_stale` `61-67`).
- `mark_planning` (`132-154`): called from `plan()` at `plan.rs:194-200` (before the dry-run guard on
  develop — the gh#19 defect); pushes a `status:"running"` PlanRecord, stage `"planning"`.
- `mark_planned` (`161-187`) is `#[allow(dead_code)]` — not yet wired (`pipeline.rs:158-160`).
- `mark_running` (`199-218`): called from `run()` at `run.rs:175-185` **only when `opts.doc_path` is
  Some**, after worktree + agent identity exist (GH#614; no more `"pending"` placeholders).
- Reconciliation: `probe_run_worktree` (`245-292`) classifies a row via worktree existence +
  `.kickoff-status` content + live-agent set into `RunProbe` (`226-239`); `reconcile_runs` (`312-350`)
  mutates rows (SentinelDone→completed, SentinelFailed→failed, Gone→aborted) and settles `stage` via
  `stage_after_runs_settle` (`361-368`). Display seam: `reconcile_runs_for_display` (`377-389`),
  invoked from the status overview (`mod.rs:391-404`). Positive-completion hook:
  `reconcile_completion_by_worktree` (`471-486`) → `mark_run_finished` (`400-425`), fired from
  `monitor::status` when it sees a terminal sentinel (`monitor.rs:74-91`).

Worktree sidecars (all git-excluded via `exclude_kickoff_files`, `launch.rs:536-567`):

| File | Written | Read |
|---|---|---|
| `.kickoff-slug` | `run.rs:101-102`, `plan.rs:179-180` | worktree identification |
| `KICKOFF.md` / `PLAN_KICKOFF.md` | `run.rs:117-118` / `plan.rs:187-188` | `$(cat ...)` in `launch.rs:212-214/787-789`, `plan.rs:239` — **the R5 audit artifact** |
| `.kickoff-criteria.json` | `run.rs:121-130` | agent (reporting section), `read_agent_issue` |
| `.kickoff-metadata.json` | `run.rs:133-142` (`KickoffMetadata`, `types.rs:57-62`) | `is_timed_out` (`types.rs:371-386`), `read_timeout_metadata` (`helpers.rs:797-801`) |
| `.kickoff-doc.json` | `protect_design_doc`, `run.rs:314-343` (+chmod 0444; GH#580) | `verify_protected_doc` via `print_doc_integrity` (`monitor.rs:149-168`) |
| `.kickoff-status` | `launch_local`: LAUNCHING `launch.rs:623-624`, FAILED `634`, RUNNING `640`; DONE/`CI_FAILED` written by the agent per prompt (`prompt.rs:67/181`) | `monitor.rs:59-72/193-202`, `pipeline.rs:270-284`, `swarm/status.rs:262-271`, `sentinel/collect.rs:65-74` |
| `.kickoff-session` | `run.rs:217`, `plan.rs:275` | `monitor.rs:213-217` (#507) |
| `.kickoff-plan.json` / `.kickoff-report.json` | agent-authored | `show_plan` (`plan.rs:328`), `report` (`monitor.rs:804`), `prompt.rs:338-416` |

**gh#19/gh#60 status**: fixed on branch `fix/56-57-58-60-19-kickoff-flow-integrity` (PR #65, in
flight), *not* in `develop` @ `6b4f736f`. gh#19 (`2cf4fbd4`): dry-run guards moved before worktree
creation — on develop, `run.rs:158-165` and `plan.rs:203-213` still leak worktrees, `.kickoff-slug`,
metadata, and (plan) a permanent `"planning"` PlanRecord on every dry run. gh#60 (`d6ee70c3`): both
launch commands gain an exit-124 trailer writing `TIMEOUT` to `.kickoff-status`, and
`normalize_status` learns `timed-out` — on develop a timeout kill leaves the sentinel at `RUNNING`
forever, caught only by the wall-clock check.

### 6. status / harvest classification

- `normalize_status` (`helpers.rs:783-794`): `done` | contains `fail`/`error` → `failed` | contains
  `running`/empty → `running` | else passthrough.
- `discover_agents` (`monitor.rs:173-314`): scans `.worktrees/*` (sentinel + `read_agent_id`
  `helpers.rs:804-817` + `.kickoff-session`), reconciles timeout (`monitor.rs:221-222`) and dead tmux
  → `stopped` (223-225), then overlays `docker ps --filter label=crosslink-agent=true` (246-311),
  classifying container-only agents Up→running / Exited(0)→done / else failed (287-293). Feeds `list`
  (`monitor.rs:319`) and `cleanup`.
- `monitor::status` (`monitor.rs:11-142`): per-agent sentinel + timeout + tmux + hub heartbeat + doc
  integrity, plus the GH#614 positive-completion hook (74-91).
- Swarm: `probe_agent_status` (`swarm/status.rs:247-304`) — missing worktree → merged/removed/planned;
  sentinel verbatim; tmux exact then substring match; else `"failed (session died)"`. `sync_status`
  (`lifecycle.rs:381+`) maps that onto hub `AgentStatus` before gates.
- Sentinel harvest: `collect_completed` (`sentinel/collect.rs:41-128`) polls pending dispatches;
  `classify_status` (`collect.rs:329-343`) — `DONE*`→success, `FAILED*`/`TIMEOUT*`→failure (attempt
  ≥2 → exhausted), anything else → keep pending (GH#561). Artifacts from `.kickoff-report.json` etc.
  (`collect.rs:131-149`).
- **gh#18 (doc-less visibility)**: `mark_running` is gated on `opts.doc_path` (`run.rs:175`), and both
  wizard QuickDescription (`mod.rs:344`, `doc_path: None` at 359) and plain `kickoff run "desc"`
  without `--doc` skip it. The pipeline overview (`pipeline_status_overview`, `mod.rs:386-492`) reads
  only `.design/*.pipeline.json` (`scan_pipeline_states`, `pipeline.rs:540-558`) — so doc-less
  kickoffs are invisible there and appear only in the worktree-scan surfaces (`kickoff list`/`status
  <agent>`/`cleanup`).

### vsdd seam requirements → code map (r1-r5, gh#61/#62)

| Req | Locus | Change shape | Difficulty |
|---|---|---|---|
| **R1** interpolation/append, not replacement | `run.rs:113-119` (main): compute `build_prompt` first, then substitute `{{built_prompt}}`, `{{issue_id}}`, `{{branch}}`, `{{description}}` into the template string (or honor `agent.kickoff_template_mode: "append"` read next to `utils.rs:26`). One substitution pass; all inputs (`opts`, `issue_id`, `branch_name`, `conventions`) already in scope at that line. | small | **small** |
| **R2** per-dispatch `--template <path>` | Add `template: Option<&str>` to `KickoffOpts` (`types.rs:81-100`) and `PlanOpts` (`types.rs:212-221`); clap arg beside `--doc` (`main.rs:1682-1683`, plan `main.rs:1732-1747`); thread through the five `KickoffOpts` construction sites (`mod.rs:79/265/347`, `lifecycle.rs:729`, plus PlanOpts sites); in `run.rs` step 5 prefer `opts.template` over the config key. Mirrors PR #44's exact footprint (types+mod+run+lifecycle+main). | small | **small** |
| **R3** per-agent composition for swarm | The uniform-template hazard is confirmed (section 4). Per-agent injection needs either per-phase template keys resolved in `lifecycle.rs:723-744` (passed via R2's `KickoffOpts.template`), or the exec-hook shape: invoke a configured command per agent with phase/slug/issue metadata, inject stdout into the prompt at the `run.rs` step-5 seam. Touches config schema + `lifecycle.rs` + `run.rs` + prompt assembly ordering. | design-sized (gh#61 cycle) | **design** |
| **R4** dials coherent with the seam | `lifecycle.rs:734` (hardcoded `"opus"`), `lifecycle.rs:736` (hardcoded 3600s), missing `effort` in `KickoffOpts` — same struct + same construction sites as R2/R3; design together to avoid two passes. | design-sized with R3 | **design** |
| **R5** keep the prompt auditable | Already satisfied by construction: whatever step 5 produces is written to `KICKOFF.md` (`run.rs:117-118`) before launch and is the literal `$(cat KICKOFF.md)` payload (`launch.rs:212-214/787-789`). Constraint on R1-R3: perform all composition **before** the `run.rs:117` write; never inject post-write. | invariant to preserve | **free** |

**gh#61 (effort dial) threading**: `effort: Option<&str>` in `KickoffOpts` (`types.rs:81-100`) →
clap `--effort` (`main.rs:~1668`) → construction sites (`mod.rs:79-94/265-280/347-362`,
`lifecycle.rs:729-744` replacing the hardcodes) → `launch_local` params (`run.rs:203-214`,
`launch.rs:571-582`) → `build_agent_command` (`launch.rs:167-223`) emitting the agent-CLI flag next
to `--model` (`launch.rs:212-214`), plus the container command at `launch.rs:787-789`, plus
`plan.rs:234-245`. Swarm additionally needs per-agent dial storage in the phase JSON
(`swarm/types.rs` `AgentEntry`) if dials vary within a wave.

**gh#62 verdict**: yes — `agent.kickoff_template` fully replaces the built prompt on main
(`run.rs:113-119` @ `26ee1885`; `read_kickoff_template` returns raw file content, `utils.rs:26-48`).
R1+R2 together are a PR-shaped change of the same scale as PR #44: one substitution pass at the
`run.rs` step-5 seam + one optional CLI arg threaded through `KickoffOpts`/`PlanOpts`, tests in
`commands/kickoff/tests.rs`. Note the seam is main-only today; a fork PR should target upstream
`develop`-equivalent and account for the `agent_binary` field drift between branches.

### sources

- `crosslink/src/commands/kickoff/run.rs` — run entry, step 5, sidecar writes, doc protection
- `crosslink/src/commands/kickoff/prompt.rs` — build_prompt + all sections, allowed tools
- `crosslink/src/commands/kickoff/plan.rs` — build_plan_prompt, plan launch
- `crosslink/src/commands/kickoff/types.rs` — KickoffOpts, PlanOpts, sidecar structs, parsers
- `crosslink/src/commands/kickoff/mod.rs` — dispatch, dispatch_launch, wizard wiring, status overview
- `crosslink/src/commands/kickoff/launch.rs` — build_agent_command, launch_local/container, init_worktree_agent, exclude_kickoff_files, preflight
- `crosslink/src/commands/kickoff/helpers.rs` — extract_criteria, detect_conventions, read_kickoff_allowed_tools, normalize_status
- `crosslink/src/commands/kickoff/pipeline.rs` — PipelineState/PlanRecord/RunRecord, mark_*, reconcile_*
- `crosslink/src/commands/kickoff/monitor.rs` — status, discover_agents, list, report
- `crosslink/src/commands/design_doc.rs` — DesignDoc, build_design_doc_section, open-questions escalation
- `crosslink/src/commands/swarm/lifecycle.rs` — swarm launch → KickoffOpts → kickoff::run, sync_status
- `crosslink/src/commands/swarm/status.rs` — probe_agent_status
- `crosslink/src/commands/sentinel/collect.rs` — collect_completed, classify_status
- `crosslink/src/main.rs` — clap KickoffCommands (defaults for model/verify/timeout/permission flags)
- `crosslink/src/utils.rs` @ main `26ee1885` — read_kickoff_template, read_no_template, read_agent_binary
- git: `26ee1885` (PR #44, main-only), `2cf4fbd4`/`d6ee70c3` (gh#19/gh#60, branch `fix/56-57-58-60-19-kickoff-flow-integrity`, PR #65 in flight)

