---
title: "cli-output-rendering-conventions"
tags: ["reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-02
updated: 2026-08-02
---


## Design Specification

### design specification

### 1. the three output modes and how they are plumbed

Both flags are **global clap flags** on the `Cli` struct:

- `--quiet` / `-q` — `src/main.rs:91-93`: "Quiet mode: only output essential data (IDs, counts)".
- `--json` — `src/main.rs:95-97`: "Output as JSON (supported by list, show, search, session status)".
  The help text is the honest contract: every other subcommand *accepts* `--json` (it is global)
  and silently ignores it.

There is **no output context object and no shared formatting/color layer**. The flags are plumbed
as two bare `bool`s from `main()` (`src/main.rs:2699`, `Cli::parse` at `src/main.rs:2700`) into
per-command-group dispatch, and each command module hand-builds its own renderer:

- Issue commands: `dispatch_issue(action, cli.quiet, cli.json)` — `src/main.rs:2745`, fn at
  `src/main.rs:2332`. Inside, dispatch branches per-variant: e.g. `List` does
  `if json { list::run_json(..) } else { list::run(..) }` (`src/main.rs:2461-2470`), `Close` does
  `if quiet { lifecycle::close_quiet(..) } else { lifecycle::close(..) }` (`src/main.rs:2552-2562`).
- Session: `session::run(action, &db, &crosslink_dir, cli.json)` — `src/main.rs:3063-3067`.
  **`cli.quiet` is not passed at all.**
- Milestone: `milestone::run(action, &db, &crosslink_dir)` — `src/main.rs:3057-3061`.
  **Neither `cli.quiet` nor `cli.json` is passed** — milestones have no quiet and no JSON mode.
- Tree: `tree::run(&db, Some(&status), json)` — `src/main.rs:2689`.
- Locks: `locks_cmd::run(action, .., cli.json)` — `src/main.rs:3101-3105` (json only).
- Cpitd: `cpitd::run(action, &db, cli.quiet)` — `src/main.rs:3086-3089` (quiet only).

Mode-selection idioms in use (three coexisting styles — pick the one matching the module you edit):

1. **Sibling functions**: `run()` (human) vs `run_json()` — `src/commands/list.rs:7-45`,
   `src/commands/search.rs:7-57`, `src/commands/show.rs:21-56`, `src/commands/import.rs:15`.
2. **Bool parameter, branch inside**: `session::status(db, dir, json)` —
   `src/commands/session.rs:141-234`; `tree::run(.., json)` — `src/commands/tree.rs:84-99`;
   `external_issues::list(.., json, quiet)` — `src/commands/external_issues.rs:38-86`.
3. **`OutputMode` enum**: `src/commands/lifecycle.rs:10-17` (`Normal`/`Quiet`), consumed by
   `close_inner` at `src/commands/lifecycle.rs:53-111` with public wrappers `close`/`close_quiet`
   (`src/commands/lifecycle.rs:19-51`). This is the most deliberate pattern but is used only here.

Shared helpers (the entire "formatting layer"): `format_issue_id` (`src/utils.rs:65-71`, negative
offline IDs render as `L1`, positive as `#42`) and `truncate` (`src/utils.rs:76-84`,
char-counted, appends `...`). There are **no color/ANSI crates** in `Cargo.toml:31-61`
(`crossterm`/`ratatui` are TUI-only); command output is plain text by design.
`hint()` (`src/main.rs:2306-2311`) emits advisory text via `tracing::info!` — with the default
`--log-level warn` (`src/main.rs:99-101`) hints are invisible unless `CROSSLINK_LOG=info`.

### 2. human renderers vs json serializers — where they can drift

Every command with a JSON mode has **two independent hand-built code paths**; nothing derives one
from the other. Field-by-field:

- **`issue list`** — human: `src/commands/list.rs:18-45`, fixed-width row
  `{:<5} {:8} {:<40} {:8} {}` with `truncate(&issue.title, 40)` at `src/commands/list.rs:34-41`
  (shows id, status, title, priority, created date only). JSON: `src/commands/list.rs:7-16`,
  `serde_json::to_string_pretty(&issues)` — the full `Issue` model, no labels in either path.
  Independent, but JSON is a serde dump of the model, so model-field drift is low; presentation
  drift (what the human row shows) is unchecked.
- **`issue show`** — JSON: purpose-built `IssueDetail` struct (`src/commands/show.rs:8-19`,
  `#[serde(flatten)]` on the issue plus labels/milestone/comments/deps/subissues/related),
  serialized at `src/commands/show.rs:21-39`. Human: seven separate `print_*` helpers
  (`src/commands/show.rs:41-56`, header at 58-76, comments at 104-129, deps at 131-150).
  Any new *derived* section (labels, milestone, relations) must be added in both places by hand.
- **`session status`** — **this is gh#16.** JSON branch `src/commands/session.rs:160-183` builds a
  `serde_json::json!` object by hand (`active`, `session_id`, `started_at`, `duration_minutes`,
  `agent_id`, optional `working_on`, `last_action`); human branch `src/commands/session.rs:185-231`
  prints its own set. Already drifted both directions: JSON has `agent_id`
  (`src/commands/session.rs:169`) which the human path never prints; the human path has the
  activity summary ("N issues created, M comments recorded", `src/commands/session.rs:210-231`)
  which JSON omits. The no-session case is also duplicated (`src/commands/session.rs:144-154`).
- **`milestone`** — human-only, no JSON serializer exists anywhere
  (`src/commands/milestone.rs:9-23` takes no `json`/`quiet`). Same smell, worse: mode flags never
  reach the module (`src/main.rs:3057-3061`).
- **`issue tree`** — JSON: `TreeNode` struct + `build_tree_node` recursion
  (`src/commands/tree.rs:52-82`); human: `print_issue` + `print_tree_recursive`
  (`src/commands/tree.rs:18-50`). The status-filter predicate is **duplicated** in both recursions
  (`src/commands/tree.rs:39-45` vs `src/commands/tree.rs:67-70`) — a logic-drift hazard, not just a
  field-drift one.
- **`issue search`** — same sibling pattern, JSON dump at `src/commands/search.rs:7-11`, decorated
  human renderer with a 60-char description snippet at `src/commands/search.rs:13-57`
  (`truncate(&flat, 60)` at `src/commands/search.rs:52`).

Verdict: list/show/tree share the gh#16 smell structurally (two hand-built paths), but session
status is the only one with *observed* field drift; tree is the only one with duplicated *logic*.

### 3. defect anchors: gh#14, #21, #23, #25, #22

### gh#14 — quiet `issue list` routes through the human renderer (titles truncated to 40 chars)

Root cause: the `List` dispatch arm consults only `json` — `src/main.rs:2459-2471`. With `-q` and
no `--json`, control falls to `commands::list::run` (`src/main.rs:2469`), the decorated human
renderer, whose row format truncates via `truncate(&issue.title, 40)` at
`src/commands/list.rs:38`. Quiet is supposed to mean "essential data (IDs, counts)"
(`src/main.rs:91`), like `create`'s bare-`{id}` behavior (`src/commands/create.rs:462-463`).
Fix locus: add a quiet branch at `src/main.rs:2461` (mirroring the `Close` pattern at
`src/main.rs:2552-2562`) calling a new `list::run_quiet` beside `src/commands/list.rs:18` that
prints one untruncated id per line. Note the external-repo path has the same smell in softer form:
`external_issues::list` accepts `quiet` but only suppresses banners
(`src/commands/external_issues.rs:56-58`) — rows still go through `truncate(.., 40)` at
`src/commands/external_issues.rs:75`.

### gh#21 — `milestone create --quiet` prints the full creation line

Root cause: `cli.quiet` is dropped at the dispatch boundary — `src/main.rs:3057-3061` calls
`milestone::run(action, &db, &crosslink_dir)` with no mode flags; `create` unconditionally prints
`"Created milestone #{id}: {name}"` on both writer paths (`src/commands/milestone.rs:33` and
`src/commands/milestone.rs:36`). Fix locus: thread `quiet` through `milestone::run`
(`src/commands/milestone.rs:9`) from `src/main.rs:3060`, and in `create`
(`src/commands/milestone.rs:25-39`) print bare `{id}` when quiet — exactly the
`src/commands/create.rs:462-463` convention.

### gh#23 — `issue list -l` takes a single label only

Root cause (two layers): the clap arg is `label: Option<String>` (`src/main.rs:646-648`) — with
clap 4's default `ArgAction::Set` a repeated `-l` is a **hard parse error**, verified against the
built binary: `error: the argument '--label <LABEL>' cannot be used multiple times` (exit 2). So
"silently keeps the last" is not what HEAD does — repetition is rejected outright; the real
limitation is single-label filtering. Underneath, `Database::list_issues` accepts
`label_filter: Option<&str>` and emits one `JOIN labels` + one `l.label = ?`
(`src/db/issues.rs:163-190`). Fix locus: change the arg to `Vec<String>`
(`src/main.rs:647-648`, matching `Create`/`Quick` at `src/main.rs:589-590` and
`src/main.rs:625-626`), widen `list_issues` to a slice with `l.label IN (...)` or AND-of-EXISTS
semantics (`src/db/issues.rs:163-190`), and update the callers `list::run`/`run_json`
(`src/commands/list.rs:7-24`) plus dispatch (`src/main.rs:2462-2469`). Decide OR vs AND semantics
in the issue before coding — the SQL shape differs.

### gh#25 — `import` is JSON-only, no YAML

Root cause: the command is defined as "Import issues from JSON file" (`src/main.rs:178-182`), the
dispatch hard-calls `commands::import::run_json` (`src/main.rs:3044-3050`), and both accepted
formats are parsed with `serde_json::from_str` — `Vec<IssueFile>` first, then legacy `ExportData`
(`src/commands/import.rs:27` and `src/commands/import.rs:39`). No YAML crate exists in
`Cargo.toml:31-61`. Fix locus: add a YAML serde dependency, then branch on extension (or content
sniff) in a new `import::run` entry above `src/commands/import.rs:15`; the parse step is already
cleanly separated from the import machinery via `ImportedIssueSpec` lowering
(`src/commands/import.rs:27-44`, `spec_from_issue_file` at 49, `specs_from_legacy` at 76), so only
the front-end deserializer changes.

### gh#22 — milestone subcommands reject names with "invalid digit found in string"

Root cause: `MilestoneCommands::Show/Add/Remove/Close/Delete` all declare bare `id: i64`
positionals with no custom value parser (`src/main.rs:1145`, `1150`, `1157`, `1159`, `1164`,
`1169`), so clap's stock `i64` parser rejects a name before crosslink code runs. Verified:
`crosslink milestone show v1.0` → `error: invalid value 'v1.0' for '<ID>': invalid digit found in
string` (exit 2). Fix locus: change those fields to `String` and resolve name-or-id in
`milestone::run` (`src/commands/milestone.rs:12-22`); there is no `get_milestone_by_name` in
`src/db/milestones.rs` (only `get_milestone` by id at `src/db/milestones.rs:31` and
`list_milestones` at `src/db/milestones.rs:74`), so add a by-name lookup there or match against
`list_milestones`. Compare `parse_issue_id_clap` (`src/main.rs:2282-2304`), the existing precedent
for a custom ID value parser.

### small-issue cluster: root cause + fix locus (gh#14/#16/#21/#22/#23/#25)

| Issue | One-line root cause | Minimal fix locus |
|---|---|---|
| gh#14 | `List` dispatch ignores `quiet`; falls into human renderer with 40-char `truncate` | `src/main.rs:2461`; `src/commands/list.rs:18,38` (+`external_issues.rs:75`) |
| gh#16 | `session status` human and JSON branches are independent hand-built renderers (agent_id/activity already drifted) | `src/commands/session.rs:160-231` — one struct, two views |
| gh#21 | `cli.quiet` never reaches `milestone::run`; `create` prints decorated line unconditionally | `src/main.rs:3060`; `src/commands/milestone.rs:9,25-39` |
| gh#22 | Bare `id: i64` clap positionals — stock parser rejects names pre-dispatch | `src/main.rs:1145-1169`; resolver in `src/commands/milestone.rs:12-22` + `src/db/milestones.rs` |
| gh#23 | `label: Option<String>` (repeat = clap error, not last-wins) + single-label SQL filter | `src/main.rs:647`; `src/db/issues.rs:163-190`; `src/commands/list.rs:7-24` |
| gh#25 | Dispatch hard-wires `import::run_json`; both parse paths are `serde_json` | `src/main.rs:3049`; `src/commands/import.rs:15-45`; `Cargo.toml` dep |

### 4. conventions for adding output to a command

- **Quiet = machine-terse, stdout is the value.** Print the bare id (or count) and nothing else:
  `println!("{id}")` — `src/commands/create.rs:462-463`, `src/commands/create.rs:559-560`. Quiet is
  not silent: essential data always prints; only decoration, banners, and hints are suppressed
  (`src/commands/external_issues.rs:56-58`, `hint` at `src/main.rs:2306-2311`). Anything advisory
  goes to stderr so it cannot contaminate quiet stdout — see the AC-13 comment at
  `src/commands/create.rs:377`.
- **JSON = structured and complete.** Serialize the full model (`src/commands/list.rs:14`) or a
  purpose-built `Serialize` struct for composed views (`IssueDetail` `src/commands/show.rs:8-19`,
  `TreeNode` `src/commands/tree.rs:52-61`). Always `to_string_pretty`. Never truncate, never
  decorate. Prefer `#[serde(flatten)]` on the model so scalar fields cannot drift
  (`src/commands/show.rs:10-11`); hand-built `json!` blobs like `src/commands/session.rs:164-181`
  are the anti-pattern that produced gh#16. Update the `--json` help list at `src/main.rs:95` when
  adding coverage.
- **Human = decorated, and the only place truncation lives.** Fixed-width columns
  (`src/commands/list.rs:34-41`), `truncate` from `src/utils.rs:76`, always `format_issue_id`
  (`src/utils.rs:65-71`) for ids so offline `L`-ids render. No ANSI colors — there is no color
  dependency and none should be introduced ad hoc.
- **Branch in the dispatch layer, not deep in the command.** `main.rs` owns the
  quiet/json decision (`src/main.rs:2461-2470`, `2552-2562`); command modules expose
  `run`/`run_json`/`run_quiet` siblings or take an `OutputMode`
  (`src/commands/lifecycle.rs:10-17`) — prefer `OutputMode` over a growing tuple of bools.
- **Errors**: `bail!`/`anyhow` propagate out of `fn main() -> Result<()>` (`src/main.rs:2699`) and
  print as `Error: ...` on stderr with exit 1; clap parse errors exit 2. Never `println!` an error
  to stdout with `Ok(())` — the existing `"Milestone #{id} not found"` sites at
  `src/commands/milestone.rs:218` and `src/commands/milestone.rs:231` (stdout, exit 0) are warts,
  not precedent; the `bail!` in `show` (`src/commands/milestone.rs:75`) is the convention.

### sources

- Crate root: `/Users/claire.celesterra/Documents/Source/magnificentlycursed/crosslink/crosslink`
  (develop @ `6b4f736f`); all citations relative to it.
- `src/main.rs` — Cli struct 86-114; issue dispatch 2332-2694; command dispatch 2699-3105;
  `parse_issue_id` 2282-2304; `hint` 2306-2311.
- `src/commands/`: `list.rs`, `show.rs`, `search.rs`, `session.rs`, `milestone.rs`, `tree.rs`,
  `import.rs`, `create.rs`, `lifecycle.rs`, `external_issues.rs`.
- `src/db/issues.rs:163-213`; `src/db/milestones.rs:31-141`; `src/utils.rs:63-84`;
  `Cargo.toml:31-61`.
- Empirical checks against `target/release/crosslink`: `milestone show v1.0` (exit 2,
  "invalid digit found in string"); `issue list -l a -l b` (exit 2, "cannot be used multiple
  times").

