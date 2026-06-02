//! vsdd CLI binary.
//!
//! v0.1.0 ships `init` as a stub. Phase 2b implementation lands in subsequent
//! iterations: pre-flight checks (git repo + crosslink + mdatron + cargo toolchain),
//! file deployment with collision handling, `ProjectInitialized` event emission.

use std::process::ExitCode;

use clap::{Parser, Subcommand};

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

fn cmd_init(_args: InitArgs) -> ExitCode {
    eprintln!(
        "vsdd init: implementation pending; see vsdd-cli crosslink issues for the planned \
         pre-flight + deployment + emission flow."
    );
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
