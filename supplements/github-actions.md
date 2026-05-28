---
supplement_slug: github-actions
languages_or_interfaces: [GitHub Actions, GitHub workflows, GitHub CI]
domains_in_scope: [platform-engineer, security, ai-engineer]
extensions: []
---

# GitHub Actions Supplement

Per-domain extensions for GitHub Actions CI/CD workflows. Per the methodology's v1 GitHub-only platform requirement, this supplement is canonical for v1 adoption.

## Platform Engineer extensions

- **Per-workflow file discipline.** One workflow per concern (`vsdd-verify.yml`, `vsdd-observe-pr-body.yml`, `release.yml`). Per-workflow trigger declarations explicit.
- **Action pinning.** Third-party actions pinned to commit SHA (not version tag) + Renovate-updated. Tag-pinning is supply-chain-attack-surface (tag can be moved).
- **Runner version pinning.** `runs-on: ubuntu-22.04` (not `ubuntu-latest`). Pin so build is reproducible across CI runs.
- **Concurrency control.** `concurrency:` block per workflow to cancel in-progress runs on push; reduces wasted CI cost.
- **Matrix builds.** Per-platform / per-version matrices for cross-target verification. Matrix fail-fast vs continue-on-error per workflow intent.
- **Caching discipline.** `actions/cache@v4` for dependency caches (cargo / npm / pip); per-lockfile cache key.

## Security extensions

- **Permissions discipline.** `permissions:` block at workflow + job level; least-privilege. `permissions: read-all` baseline; explicit `write` grants per-job.
- **GitHub Secrets.** Credentials via `${{ secrets.<NAME> }}`; never inline + never echoed. Anonymization hook detects accidental echoing.
- **OIDC for cloud auth.** Use OpenID Connect to authenticate against AWS / GCP / Azure rather than long-lived secrets where possible.
- **PR from fork discipline.** `pull_request` event doesn't have write access to secrets; `pull_request_target` event does — used only for trusted post-merge actions. Bypass mode for the methodology's bypass-approval label gate.
- **Bypass-approval label gate.** Per the methodology's bypass-marker enforcement: PRs with `hook-bypass` marker require `bypass-approved` label by maintainer ≠ PR-author. CI workflow enforces structurally.
- **SARIF emission to Code Scanning.** `github/codeql-action/upload-sarif@v3` uploads SARIF; findings appear in PR Conversation tab.

## AI Engineer extensions

- **CI auth method.** API key (Anthropic's recommended for automation); Plan auth structurally rejected by the methodology's cross-field validation. `ANTHROPIC_API_KEY` via GitHub Secrets.
- **Cost-band per workflow.** Per workflow's typical token consumption tracked; budget breach alerts via `PushNotification` events.
- **Scheduled cron sweeps.** `schedule:` triggers for methodology drift sweeps + dependency audit + CHANGELOG discipline check; per-cron cost tracked.
- **CI vs operator-local cost split.** Per the methodology's auth_method.operator_local + auth_method.ci separation; CI cost has its own ledger.
