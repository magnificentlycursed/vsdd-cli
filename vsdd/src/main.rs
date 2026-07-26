//! vsdd CLI binary.
//!
//! `vsdd init --check` runs the pre-flight environment probe. Substantive deployment
//! (file emission, ProjectInitialized event) lands in subsequent Phase 2b iterations.

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
    /// Dry-run: report the deployment plan without writing files.
    #[arg(long)]
    check: bool,

    /// Non-interactive CI mode: skip operator prompts; use defaults.
    #[arg(long = "ci-mode")]
    ci_mode: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::Init(args) => cmd_init(args),
        Command::Status(args) => cmd_status(args),
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
        // Dry-run: pre-flight passed; no deployment.
        return ExitCode::SUCCESS;
    }

    let options = vsdd_core::init::InitOptions {
        ci_mode: args.ci_mode,
    };
    match vsdd_core::init::init(&cwd, &options) {
        Ok(report) => {
            println!(
                "vsdd init: deployed {} file(s); skipped {} unchanged file(s); manifest at {}",
                report.deployed.len(),
                report.skipped.len(),
                report.manifest_path.display()
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error[VSDD-E0230]: init failed\n   = note: {e}");
            ExitCode::from(2)
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
