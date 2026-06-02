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

    // Substantive deployment lands in subsequent Phase 2b iterations.
    eprintln!(
        "vsdd init: pre-flight passed; substantive deployment (file emission + \
         ProjectInitialized event) is pending implementation. Re-run with --check to confirm \
         environment readiness."
    );
    let _ = args.ci_mode; // currently unused; routes will diverge in next iteration
    ExitCode::from(2)
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
        }
    }

    #[test]
    fn rejects_unknown_subcommand() {
        let result = Cli::try_parse_from(["vsdd", "frobnicate"]);
        assert!(result.is_err(), "unknown subcommand should fail to parse");
    }
}
