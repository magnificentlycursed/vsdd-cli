//! vsdd CLI binary.
//!
//! `vsdd init --check` runs the pre-flight environment probe. Substantive deployment
//! (file emission, ProjectInitialized event) lands in subsequent Phase 2b iterations.

use std::io::IsTerminal;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

mod preflight;

#[derive(Parser, Debug)]
#[command(name = "vsdd", about, version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize the VSDD methodology toolkit in the current project.
    Init(InitArgs),
    /// Answer the phase question: human form by default, machine form
    /// for agents, one-line segment for statusline surfaces.
    Status(StatusArgs),
    /// The routing-before-fix guardrail: block when a finding closed by fix
    /// carries no routing. Exit 0 pass, 1 blocked, 2 unverifiable (fail-closed).
    Gate(GateArgs),
}

#[derive(clap::Args, Debug)]
struct StatusArgs {
    /// Render the one-line statusline segment.
    #[arg(long)]
    statusline: bool,

    /// Render the machine form (JSON) instead of the human form.
    #[arg(long, conflicts_with = "statusline")]
    machine: bool,

    /// Compose the multi-repo display over a repo-set config
    /// (statusline only).
    #[arg(long = "repo-set", requires = "statusline")]
    repo_set: Option<std::path::PathBuf>,
}

#[derive(clap::Args, Debug)]
struct InitArgs {
    /// Pre-flight only: report the environment probe without deploying.
    #[arg(long)]
    check: bool,

    /// Non-interactive CI mode: skip operator prompts; use defaults. Implies
    /// --no-prompt.
    #[arg(long = "ci-mode")]
    ci_mode: bool,

    /// Overwrite Conflict (operator-edited) managed files with the template.
    #[arg(long)]
    force: bool,

    /// Apply toolkit upgrades (unedited files whose template changed) only.
    #[arg(long)]
    update: bool,

    /// Non-interactive: skip Conflict files (never overwrite) unless --force.
    #[arg(long = "no-prompt")]
    no_prompt: bool,

    /// Print the per-file classification and planned action; write nothing.
    #[arg(long = "dry-run")]
    dry_run: bool,
}

#[derive(clap::Args, Debug)]
struct GateArgs {
    /// Render the verdict as JSON instead of human text.
    #[arg(long)]
    machine: bool,

    /// CI strictness (remediation REQ-6's mode seam): undecidable
    /// deviation retest triggers are inconclusive (exit 2) instead of
    /// warn-pass, and issue-state triggers resolve via the GitHub API
    /// when GH_TOKEN is provisioned.
    #[arg(long)]
    ci: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::Init(args) => cmd_init(args),
        Command::Status(args) => cmd_status(args),
        Command::Gate(args) => cmd_gate(args),
    }
}

fn cmd_status(args: StatusArgs) -> ExitCode {
    use vsdd_core::registry::{
        self,
        sets::{CompositionScopeAndActions, StatuslineData},
    };

    let cwd = match std::env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("vsdd status: cannot read current directory: {e}");
            return ExitCode::from(2);
        }
    };
    let data: StatuslineData = match registry::load_set(&cwd, "statusline-data") {
        Ok(d) => d,
        Err(diagnostic) => {
            eprintln!("{}", vsdd::status::clean_for_terminal(&diagnostic.message));
            return ExitCode::from(2);
        }
    };
    let actions: CompositionScopeAndActions =
        match registry::load_set(&cwd, "composition-scope-and-actions") {
            Ok(a) => a,
            Err(diagnostic) => {
                eprintln!("{}", vsdd::status::clean_for_terminal(&diagnostic.message));
                return ExitCode::from(2);
            }
        };

    if args.statusline {
        // The composed multi-repo display when a repo set is given:
        // one line per configured repo, current repo first.
        if let Some(config_path) = &args.repo_set {
            let config = match vsdd::status::multi::read_repo_set_config(config_path) {
                Ok(c) => c,
                Err(diagnostic) => {
                    eprintln!("{}", vsdd::status::clean_for_terminal(&diagnostic.message));
                    return ExitCode::from(2);
                }
            };
            let current = vsdd::status::segment_for_repo(&cwd, &data, &actions);
            // Dedup on canonical spellings: a symlinked config entry
            // (/tmp vs /private/tmp) must not render the current repo
            // twice (vsdd-cli #781).
            let canonical_cwd = cwd.canonicalize().unwrap_or_else(|_| cwd.clone());
            // Member repos render under the configured per-repo budget
            // (vsdd-cli #778); the current repo — the one the operator
            // is in — renders unbounded, since its answer is the line
            // the display exists for.
            let budget = std::time::Duration::from_millis(config.per_repo_budget_ms);
            let shared_data = std::sync::Arc::new(data);
            let shared_actions = std::sync::Arc::new(actions);
            let others: Vec<String> = config
                .repos
                .iter()
                .filter(|r| r.canonicalize().unwrap_or_else(|_| r.to_path_buf()) != canonical_cwd)
                .map(|r| {
                    let root = r.clone();
                    let d = shared_data.clone();
                    let a = shared_actions.clone();
                    vsdd::status::bounded_line(r, budget, move || {
                        vsdd::status::segment_for_repo(&root, &d, &a)
                    })
                })
                .collect();
            println!("{}", vsdd::status::multi::render_multi(&current, &others));
            return ExitCode::SUCCESS;
        }
        // Stdin is passed to the counting seam and never read.
        let run = vsdd::status::run_statusline(
            &cwd,
            std::io::stdin().lock(),
            &data,
            &actions,
            vsdd_core::snapshot::acquire::acquire_snapshot,
        );
        println!("{}", run.segment);
        return ExitCode::SUCCESS;
    }

    match vsdd_core::state::read_state(&cwd.join(".vsdd/state.yaml"), &data) {
        Ok(state) => {
            let snapshot = vsdd_core::snapshot::acquire::acquire_snapshot(&cwd);
            let answer =
                vsdd_core::answer::derive::derive_phase_answer(&state, &snapshot, &actions);
            if args.machine {
                println!(
                    "{}",
                    vsdd::status::machine::render_machine(&answer, &snapshot, &data)
                );
            } else {
                print!(
                    "{}",
                    vsdd::status::human::render_human(&answer, &snapshot, &data)
                );
            }
            ExitCode::SUCCESS
        }
        Err(diagnostic) => {
            // The broken-state branch: every surface still speaks.
            let last = vsdd_core::snapshot::acquire::last_boundary_subject(&cwd);
            let surfaces =
                vsdd::status::broken::compose_broken_state(&diagnostic, &data, last.as_deref());
            if args.machine {
                println!("{}", surfaces.machine);
            } else {
                eprint!("{}", surfaces.human);
            }
            ExitCode::from(1)
        }
    }
}

fn cmd_gate(args: GateArgs) -> ExitCode {
    use vsdd_core::answer::deviations::{self, GateMode};
    use vsdd_core::answer::integrity::{gate_verdict, GateVerdict};

    let cwd = match std::env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("vsdd gate: cannot read current directory: {e}");
            return ExitCode::from(2);
        }
    };
    let snapshot = vsdd_core::snapshot::acquire::acquire_snapshot(&cwd);
    let routing_exit: u8 = match gate_verdict(&snapshot) {
        GateVerdict::Pass => {
            if args.machine {
                println!(
                    "{}",
                    serde_json::json!({"gate": "unrouted-findings", "verdict": "pass"})
                );
            } else {
                println!(
                    "vsdd gate: pass — no unrouted findings in the forward-only universe \
                     (routing-before-fix satisfied)"
                );
            }
            0
        }
        GateVerdict::Block(handles) => {
            if args.machine {
                println!(
                    "{}",
                    serde_json::json!({
                        "gate": "unrouted-findings",
                        "verdict": "block",
                        "unrouted": handles,
                    })
                );
            } else {
                eprintln!(
                    "vsdd gate: BLOCKED — {} finding(s) closed by fix without filed routing:",
                    handles.len()
                );
                for handle in &handles {
                    eprintln!(
                        "  {handle} — file a routing `plan` comment naming the target phase, \
                         or the fix lane"
                    );
                }
            }
            1
        }
        GateVerdict::Unverifiable(reason) => {
            // Fail-closed: an unverifiable acquisition blocks, distinct from a
            // clean pass and from a findings block (exit 2, mirroring mdatron's
            // pipeline-failure code).
            if args.machine {
                println!(
                    "{}",
                    serde_json::json!({
                        "gate": "unrouted-findings",
                        "verdict": "unverifiable",
                        "reason": reason,
                    })
                );
            } else {
                eprintln!("vsdd gate: UNVERIFIABLE (fail-closed) — {reason}");
            }
            2
        }
    };

    // The deviations gate leg (build-plan Phase 1; remediation REQ-6), a
    // sibling verdict source beside the routing check: both legs run, and
    // the worst verdict (highest exit class, 0/1/2) wins the exit code.
    // The clock and the issue-state oracle live HERE at the CLI layer —
    // the core stays pure (caller-supplied today, injected oracle).
    let mode = if args.ci { GateMode::Ci } else { GateMode::Local };
    let today = {
        let days = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| (d.as_secs() / 86_400) as i64)
            .unwrap_or(0);
        deviations::iso_date_from_unix_days(days)
    };
    // Local mode never queries: every issue-state trigger is undecidable
    // (warn-pass, resolved Q3). CI mode queries via `gh api` when GH_TOKEN
    // is provisioned; without it the oracle stays undecidable and the CI
    // leg reports inconclusive (fail-closed), never a silent pass.
    let use_gh = args.ci && std::env::var("GH_TOKEN").is_ok_and(|v| !v.is_empty());
    let oracle = move |issue_ref: &str| {
        if use_gh {
            gh_issue_state(issue_ref)
        } else {
            None
        }
    };
    let outcome = deviations::deviations_gate(
        &cwd.join(".vsdd/registry/deviation-registry.yaml"),
        &today,
        mode,
        &oracle,
    );
    for warning in &outcome.warnings {
        eprintln!("deviations: {warning}");
    }
    let deviations_exit: u8 = match &outcome.verdict {
        GateVerdict::Pass => {
            if args.machine {
                println!(
                    "{}",
                    serde_json::json!({
                        "gate": "deviations",
                        "verdict": "pass",
                        "warnings": outcome.warnings,
                    })
                );
            } else {
                println!(
                    "vsdd gate: deviations pass — no standing entry lapsed or fired unre-armed"
                );
            }
            0
        }
        GateVerdict::Block(entries) => {
            if args.machine {
                println!(
                    "{}",
                    serde_json::json!({
                        "gate": "deviations",
                        "verdict": "block",
                        "blocked": entries,
                        "warnings": outcome.warnings,
                    })
                );
            } else {
                eprintln!(
                    "vsdd gate: deviations BLOCKED — {} registry entries owe a retest or re-arm:",
                    entries.len()
                );
                for entry in entries {
                    eprintln!("  {entry}");
                }
            }
            1
        }
        GateVerdict::Unverifiable(reason) => {
            if args.machine {
                println!(
                    "{}",
                    serde_json::json!({
                        "gate": "deviations",
                        "verdict": "unverifiable",
                        "reason": reason,
                        "warnings": outcome.warnings,
                    })
                );
            } else {
                eprintln!("vsdd gate: deviations UNVERIFIABLE (fail-closed) — {reason}");
            }
            2
        }
    };

    ExitCode::from(routing_exit.max(deviations_exit))
}

/// Resolve an issue reference (`owner/repo#N`) to its state by shelling
/// to `gh api` — the provisioned-GH_TOKEN seam (REQ-6). Chosen over a new
/// HTTP dependency: `gh` ships on GitHub runners and reads GH_TOKEN
/// itself, and arguments pass directly to the process (no shell
/// interpolation). Any failure — missing gh, network, unknown ref — is
/// None: undecidable, which the CI mode treats as inconclusive
/// (fail-closed), never a silent pass.
fn gh_issue_state(issue_ref: &str) -> Option<vsdd_core::answer::deviations::IssueState> {
    use vsdd_core::answer::deviations::IssueState;
    fn plain(s: &str) -> bool {
        !s.is_empty()
            && s.bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'.' || b == b'_')
    }
    let (repo, number) = issue_ref.rsplit_once('#')?;
    let (owner, name) = repo.split_once('/')?;
    if !plain(owner)
        || !plain(name)
        || number.is_empty()
        || !number.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    let out = std::process::Command::new("gh")
        .args([
            "api",
            &format!("repos/{owner}/{name}/issues/{number}"),
            "--jq",
            ".state",
        ])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    match String::from_utf8_lossy(&out.stdout).trim() {
        "open" => Some(IssueState::Open),
        "closed" => Some(IssueState::Closed),
        _ => None,
    }
}

fn cmd_init(args: InitArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("vsdd init: cannot read current directory: {e}");
            return ExitCode::from(2);
        }
    };

    let report = preflight::check_environment(&cwd);
    print!("{}", report.render());

    if !report.all_pass() {
        return ExitCode::from(1);
    }

    if args.check {
        // Pre-flight only: pre-flight passed; no deployment.
        return ExitCode::SUCCESS;
    }

    // Interactive Conflict prompt (REQ-6): on a TTY, without --no-prompt /
    // --ci-mode / --force, discover the conflicts and prompt per file, then run
    // for real with the operator's choices. The core `init` never reads stdin.
    let mut resolved = std::collections::BTreeMap::new();
    let interactive = !args.dry_run
        && !args.no_prompt
        && !args.ci_mode
        && !args.force
        && std::io::stdin().is_terminal()
        && std::io::stdout().is_terminal();
    if interactive {
        let probe = vsdd_core::init::InitOptions {
            ci_mode: args.ci_mode,
            force: args.force,
            update: args.update,
            no_prompt: args.no_prompt,
            dry_run: false,
            resolved_conflicts: std::collections::BTreeMap::new(),
        };
        if let Ok(conflicts) = vsdd_core::init::plan_conflicts(&cwd, &probe) {
            for conflict in conflicts {
                let choice = prompt_conflict(&conflict);
                resolved.insert(conflict.rel_path, choice);
            }
        }
    }

    let options = vsdd_core::init::InitOptions {
        ci_mode: args.ci_mode,
        force: args.force,
        update: args.update,
        // After prompting we've captured every choice, so suppress the core's
        // fail-closed drift refusal for any file we did not explicitly resolve.
        no_prompt: args.no_prompt || interactive,
        dry_run: args.dry_run,
        resolved_conflicts: resolved,
    };
    match vsdd_core::init::init(&cwd, &options) {
        Ok(report) => {
            if args.dry_run {
                println!("vsdd init: dry-run complete; no files written");
            } else {
                println!(
                    "vsdd init: deployed {} file(s); skipped {} unchanged file(s); manifest at {}",
                    report.deployed.len(),
                    report.skipped.len(),
                    report.manifest_path.display()
                );
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error[VSDD-E0230]: init failed\n   = note: {e}");
            ExitCode::from(2)
        }
    }
}

/// Prompt the operator to resolve one Conflict file (REQ-6): keep the edit,
/// accept the new template, or show a diff and re-ask.
fn prompt_conflict(conflict: &vsdd_core::init::ConflictInfo) -> vsdd_core::init::ConflictChoice {
    use std::io::Write;
    use vsdd_core::init::ConflictChoice;

    loop {
        eprint!(
            "vsdd init: conflict at {} — [k]eep your edit / [a]ccept new template / [d]iff? ",
            conflict.rel_path
        );
        let _ = std::io::stderr().flush();

        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).unwrap_or(0) == 0 {
            // EOF: preserve the operator's work (the safe default).
            return ConflictChoice::KeepOperatorEdit;
        }
        match line.trim() {
            "a" | "accept" => return ConflictChoice::AcceptNewTemplate,
            "k" | "keep" => return ConflictChoice::KeepOperatorEdit,
            "d" | "diff" => print_conflict_diff(conflict),
            _ => eprintln!("  please answer k, a, or d"),
        }
    }
}

/// Print a line-level diff of the operator's copy against the template.
fn print_conflict_diff(conflict: &vsdd_core::init::ConflictInfo) {
    let disk = std::fs::read(&conflict.dest).unwrap_or_default();
    let yours = String::from_utf8_lossy(&disk);
    let template = String::from_utf8_lossy(&conflict.template);
    let yours_lines: Vec<&str> = yours.lines().collect();
    let template_lines: Vec<&str> = template.lines().collect();
    eprintln!("--- {} (your copy)", conflict.rel_path);
    eprintln!("+++ {} (new template)", conflict.rel_path);
    let max = yours_lines.len().max(template_lines.len());
    for i in 0..max {
        let y = yours_lines.get(i).copied();
        let t = template_lines.get(i).copied();
        if y != t {
            if let Some(y) = y {
                eprintln!("- {y}");
            }
            if let Some(t) = t {
                eprintln!("+ {t}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn parses_init_without_flags() {
        let cli = Cli::parse_from(["vsdd", "init"]);
        match cli.command {
            Command::Init(args) => {
                assert!(!args.check);
                assert!(!args.ci_mode);
            }
            other => panic!("expected the init command, parsed {other:?}"),
        }
    }

    #[test]
    fn parses_init_with_check_flag() {
        let cli = Cli::parse_from(["vsdd", "init", "--check"]);
        match cli.command {
            Command::Init(args) => {
                assert!(args.check);
                assert!(!args.ci_mode);
            }
            other => panic!("expected the init command, parsed {other:?}"),
        }
    }

    #[test]
    fn parses_init_with_ci_mode_flag() {
        let cli = Cli::parse_from(["vsdd", "init", "--ci-mode"]);
        match cli.command {
            Command::Init(args) => {
                assert!(!args.check);
                assert!(args.ci_mode);
            }
            other => panic!("expected the init command, parsed {other:?}"),
        }
    }

    #[test]
    fn parses_status_with_the_three_form_flags() {
        let cli = Cli::parse_from(["vsdd", "status", "--statusline"]);
        match cli.command {
            Command::Status(args) => {
                assert!(args.statusline);
                assert!(!args.machine);
                assert!(args.repo_set.is_none());
            }
            other => panic!("expected the status command, parsed {other:?}"),
        }
        let conflict = Cli::try_parse_from(["vsdd", "status", "--statusline", "--machine"]);
        assert!(
            conflict.is_err(),
            "the segment and machine forms are distinct surfaces"
        );
        let requires = Cli::try_parse_from(["vsdd", "status", "--repo-set", "x.yaml"]);
        assert!(
            requires.is_err(),
            "the repo set composes the statusline surface only"
        );
    }

    #[test]
    fn rejects_unknown_subcommand() {
        let result = Cli::try_parse_from(["vsdd", "frobnicate"]);
        assert!(result.is_err(), "unknown subcommand should fail to parse");
    }
}
