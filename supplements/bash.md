---
supplement_slug: bash
languages_or_interfaces: [Bash, sh, POSIX shell]
domains_in_scope: [software-engineer, quality-engineer, security, platform-engineer]
extensions: []
---

# Bash Supplement

Per-domain extensions for shell-script-bearing projects (build scripts, CI scripts, operator-runbook scripts).

## Software Engineer extensions

- **`set -euo pipefail` at top of every script.** Fail-fast on error + unset-variable + pipeline-failure. The single most load-bearing bash discipline.
- **Quoting discipline.** Every variable expansion quoted: `"$var"` not `$var`. Unquoted expansion in word-splitting contexts is the load-bearing footgun.
- **`shellcheck` linting.** CI-side static analysis catches quoting bugs, undefined variables, common anti-patterns. `shellcheck` exit code 0 is the gate.
- **Function vs command discipline.** Reusable logic in functions; commands invoke programs. Don't fork a subshell when a function suffices.

## Quality Engineer extensions

- **`bats` for shell-script testing.** Bash Automated Testing System; per-function unit tests in `tests/*.bats`.
- **Per-script execution test.** Every operator-facing script has an "invoke + verify expected output" test, even if just a smoke check.
- **`shellcheck` as the falsifiability check.** Every commit touching shell scripts runs shellcheck; failures block merge.

## Security extensions

- **Command injection discipline.** Never `eval` user input. Never construct commands by string concatenation with untrusted input. Use arrays for argument lists: `cmd=(prog arg1 "$user_input"); "${cmd[@]}"`.
- **Path traversal awareness.** When script processes file paths from input, validate the path stays within expected directories before operations.
- **Temporary file discipline.** `mktemp` for temp files; `trap 'rm -rf "$tmpdir"' EXIT` for cleanup. Hardcoded `/tmp/myfile` is a TOCTOU race.
- **`PATH` discipline.** Set `PATH` explicitly at script top OR use absolute paths for security-sensitive commands. Untrusted `PATH` can lead to command-substitution attacks.

## Platform Engineer extensions

- **Shebang discipline.** `#!/usr/bin/env bash` for portability; `#!/bin/sh` for strict POSIX compatibility. Pick per script intent.
- **Cross-platform discipline.** macOS uses BSD utilities (different flags than GNU); scripts run on both require either: (a) feature-detection, (b) strict POSIX-only, (c) named per-platform branches.
- **CI invocation discipline.** Scripts invoked from CI workflows have explicit `bash -e script.sh` or shebang-execution; relying on implicit shell choice is the failure mode.
- **Idempotency.** Scripts that mutate state are idempotent: re-running the script produces the same end-state. Per the methodology's idempotent re-init discipline applied to operator scripts.
