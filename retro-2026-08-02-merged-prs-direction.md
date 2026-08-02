---
title: "retro-2026-08-02-merged-prs-direction"
tags: ["reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-02
updated: 2026-08-02
---


## Design Specification

### design specification

### per-pr notes

| PR | Merged | What/why | Debt / pattern visible in the merge |
|---|---|---|---|
| #32 | 07-21, develop | Hooks resolved bare `crosslink` from PATH, bypassing `find_crosslink_binary()`; a 0.8.0 binary's clap exit 2 read as "AGENT PAUSED" and wedged every session. Fail-open on non-protocol exit 2. | The only review comment across all seven merges: dollspace's "lint the code :p" — origin of the CI-lint-parity mandate (now MEMORY). File list includes lint-only touches to `migrate_hub_v3.rs`/`knowledge/core.rs`/`seam.rs` from that pass. First instance of a local-gate/CI mismatch. |
| #35 | 07-21, develop | v3 state-layer correctness bundle (gh#7 cold lock reads, gh#7 integrity false-FAIL + destructive `--repair`, gh#12 comment dup). One commit per issue. | Documented 2 pre-existing failing tests (`test_bootstrap_*`) rather than fixing — rode along until #51 (07-29). Predicted textual overlap with #6's rewrite; materialized as the #39 conflict. |
| #38 | 07-21, **main** | Completed the v3 promotion path (gh#11 SQLITE 1555 on post-migrate sync, gh#4 `to-shared` half). Based on main because it needed #6's machinery, absent from develop. | Cross-base split (main vs develop) forced dollspace's reconciliation PR #39. Body pre-specified the exact conflict resolution for `restore_sqlite_only_issues` — good practice; #39 applied it as written. "Completes #4" (non-closing verb) used deliberately for a partial fix. |
| #46 | 07-28, develop | v3 completeness pair: gh#45 finalize seeded-genesis guard (metadata-recorded genesis + ancestry checks), gh#4 dashboard v3-awareness (7 sites). | Scope notes explicitly named the intentional gaps that became #48/#49. Field-tested against the 07-22 production migrations. Largest merged diff (+681). |
| #50 | 07-29, develop | gh#34 half 1: dashboard advisory git calls get `GIT_TERMINAL_PROMPT=0` / neutralized askpass / empty credential.helper so headless polls fail fast instead of hanging. | "Addresses #34 ... this PR does not close #34" — explicit non-close statement; the partial-fix keyword discipline at its best. Named but did **not file** the follow-up for discovery/onboarding clones (`github_api.rs`, `api.rs` same treatment) — still unfiled, visible debt. |
| #51 | 07-29, develop | gh#34 half 2: `ensure_cache_git_identity` trusted `git config` exit code, so an **empty-string** identity passed as configured; now requires non-empty email AND name. | Closed the loop on the #35/#38 "documented failing tests" debt. Pattern: exit-code-trust vs value-trust — same genus as #32's exit-2 misread. |
| #54 | 08-01, develop | Dashboard v3 monitoring completeness: gh#48 (`status='error'` finally set), gh#49 agent-requests half (`read_all_agent_requests` over agent refs), gh#53 (`FETCH_HEAD` mtime for `last_fetch_at`). | "Addresses #49 (the agent-requests half)" — ci_status half deliberately excluded because **no v3 CI-status writer exists** (a design decision, not a port). #49 is now half-fixed but its title still claims both surfaces empty — needs a re-scope comment. #48's consecutive-failure threshold named as possible follow-up, unfiled. |
| #44 | 08-01, main | (rock-solid-sites) Landed `agent.kickoff_template` — the injection seam gh#43. | One-line body ("Fixes #43"), full-prompt **replacement** semantics. Immediately generated follow-up gh#62 and vsdd's R1-R5: replacement forces consumers to fork crosslink's 15-step protocol. The seam's first shape landed without consumer-requirements review. |
| #39 | 07-21, develop | (dollspace-gay) Sync main→develop reconciling the #32/#35 (develop) vs #6/#38 (main) split. | Cost of the cross-base episode made concrete: a 3.9k-line reconciliation merge. Also documents dollspace's own local gate: `PROPTEST_CASES=10 cargo test --bin crosslink` **plus `cargo test --test cli_integration`** — the exact lane the fork's gate was missing at #65. |

### recurring themes & process changes

**1. Local test gate lags CI — twice now, same shape.**
- #32: local gate had no clippy parity → "lint the code :p" → CI-exact clippy+fmt became mandatory (MEMORY: ci-lint-parity).
- #65: local gate was bin-only (`cargo test --bin crosslink`, the justfile:215 fast lane) and skipped `tests/cli_integration.rs`; CI caught it, forcing fixup `b3052d7b` ("test(kickoff): align dry-run integration test with the gh#19 contract").
- **Process change:** extend the mandatory pre-push gate to CI-exact *tests*, not just lint: `cargo test --bin crosslink` AND `cargo test --test cli_integration` (matching dollspace's own #39 validation lane) whenever the diff touches CLI-visible behavior. Update the ci-lint-parity memory to ci-*parity* (lint + test lanes).

**2. Fixes claimed complete before live-fire on the original repro.**
- gh#55 was filed literally about `kickoff plan` stalling. #32 fixed one contributor (false AGENT PAUSED); #63/#64 fix the container-side contributors; but only the live-fire retest at `6b4f736f` (gh#66) found the actual cause of the filed repro — the workspace-trust dialog on the tmux plan path, which no in-flight change touches.
- Same genus at #34: the credential-hang fix (#50) looked complete until the bootstrap-identity mechanism surfaced as a second half (#51).
- **Process change:** for hang/stall-class issues, "fixed" requires re-running the *originally filed reproduction*, not a plausible mechanism being closed. Prefer filing split issues per mechanism (as #66 and the #50/#51 halves did) over stretching one issue.

**3. Headless-vs-interactive gates are the dominant defect class.** Credential prompt (#34), permission prompt (gh#59), workspace-trust dialog (gh#66), entrypoint `source` of auth env (gh#10) — four distinct interactive/ambient-environment gates, each found separately. **Process change:** when touching any agent-launch or remote-touching path, run a one-time checklist: enumerate every prompt/dialog/credential vector on the path and prove each is either disabled, answered, or fails fast. The gh#34 env-scoping idiom (`dashboard/projects.rs:208-219`) is the template.

**4. Partial-fix keyword discipline is mature — keep it, and audit prose too.** The "Fixes = fully closes; Addresses/Completes/half-language = stays open" convention held across #38, #50, #54 and the pending PRs, and commit messages use the non-closing `gh#NN` form, so develop→main promotion cannot fire commit-message auto-close. The residual risk is *prose leading a maintainer to close manually* (see direction changes below).

**5. Follow-up filing is good but not airtight.** #46/#50 filed #48/#49 from scope notes (good). Still unfiled: the #50 discovery/onboarding-clone hardening, and #54's consecutive-failure threshold for stale-data flap. **Process change:** every "left as a follow-up" sentence in a merged PR body gets an issue number before the PR merges, or gets deleted.

**6. Bundle mechanics work.** Thematic bundles with one-commit-per-issue (#35, #46, #54, #64, #65), merge-order-independent CHANGELOG placement verified by 3-way merge simulation (#64/#65) — no reviewer friction, no inter-PR conflicts. Keep.

### direction changes for pending prs

**Keyword audit of the actual bodies (question 2):**
- **PR #63**: "Fixes #59. Likely fixes the container-path half of #55." — the only closing-keyword-adjacent reference is #59. "fixes the container-path half of #55" does **not** match GitHub's `keyword #N` adjacency rule, so no closing link to #55 is created. Safe.
- **PR #64**: "Fixes #9. Fixes #10." #55 appears only as "the #55 stall" with no keyword. Safe.
- **PR #65**: "Fixes #19. Fixes #56. Fixes #57. Fixes #58. Fixes #60." No reference to #55 or #49 anywhere in the body. Safe.
- Structural backstop: all three PRs base `develop`, and the repo default is `main` — GitHub only auto-closes on merge into the default branch. Observed confirmation: #4, #45, #48, #53 are still open despite merged "Fixes" PRs #46/#54 on develop. So there is **no mechanical auto-close risk to #55 or #49 at all** from these merges.

**The real risk is prose-driven manual closure, and it is live.** #63 ("a direct cause of the #55 signature") and #64 ("together with #59, this addresses **every tracked container-side contributor** to the #55 stall") were written before gh#66 existed. A maintainer merging both could reasonably conclude #55 is done and close it by hand — while the plan/tmux half (the scenario #55 was literally filed about; #55's repro is `kickoff plan`, which never touches the container path) remains unfixed. **Action: edit the #63 and #64 bodies (or add a comment) to cross-reference gh#66 as the remaining plan/tmux half and state explicitly: "#55 stays open until #66 lands and the live-fire repro passes."** Note #63's "The other tracked #55 contributor…" parenthetical is now stale for the same reason.

**#49 re-scope:** #54 merged the agent-requests half; #49's title still claims both surfaces are empty. Add a comment retitling/re-scoping #49 to the ci_status remainder (v3 home + writer — a design decision per #54's body) so a post-promotion issue sweep doesn't close it as fully done, and so a future ci_status PR can legitimately say "Fixes #49".

**No content changes needed to #63/#64/#65 diffs.** All CI checks green on all three; #65's integration-test miss was already fixed in `b3052d7b`.

### gh#66 / gh#61 / gh#62 sequencing

**gh#66 — the flag pivot is confirmed codebase-consistent; the issue body's proposal is not.**
- The upstream issue #66 body (authored from the fork) proposes writing `hasTrustDialogAccepted` into `$HOME/.claude.json`. Per kb-config-conventions section 1, there is **no precedent anywhere in `src/` for writing `~/.claude.json`** — `~/.claude/` is read-only (container auth mount, `commands/container.rs:325-334`), and the only user-global writes are `~/.crosslink/` dashboard state and opt-in shell-rc lines. The classifier block on that write was the conventions doc enforcing itself. **Action: comment on upstream #66 revising the proposed fix to the flag approach before dollspace implements the write-scope-violating shape as filed.**
- The pivot — add `--skip-permissions`/`--permission-mode` to `kickoff plan` — is a pure extension of existing surface: `KickoffCommands::Run` (main.rs:1684-1708) and `KickoffCommands::Launch` (main.rs:1826-1847) already expose the exact pair (`conflicts_with` guard included), and PR #63 just extracted the shared `permission_flag()` helper so local/container/plan cannot drift. The fix replaces the hardcoded literals at `plan.rs:242-244` ("plan mode never skips permissions" / "never overrides permission_mode (#603)").
- Subtleties to carry into the PR:
  1. **It relaxes upstream decision #603.** Frame as opt-in only: default stays `skip_permissions: false` (attended plan keeps the trust dialog; read-only plan mode argues for exactly this narrow default — do NOT default plan to skip).
  2. **`--permission-mode` alone does not clear the trust dialog** (per #66's research, only `--dangerously-skip-permissions` skips it). The help text/docs must say this plainly, or headless users will pass `--permission-mode plan` and still stall.
  3. **`kickoff launch --plan` already accepts both flags and silently drops them** on the plan path (main.rs:3204-3205 → hardcoded plan.rs literals) — an accepted-but-ignored-flag bug; thread Launch's values through in the same PR.
  4. **Swarm does not share the plan path** (`lifecycle.rs:746` calls `kickoff::run`), but `lifecycle.rs:741-742` hardcodes `skip_permissions: false, permission_mode: None` — swarm tmux agents are exposed to the same trust dialog. Name this in the PR; the plumbing belongs to the #61 design pass (below), not the #66 fix.
  5. Rebase the gh#66 branch on PR #63 to reuse `permission_flag()` rather than duplicating it.
- **Land gh#66 first among upcoming work**: #55's closure predicate is (#59 merged) AND (#66 fixed) AND (live-fire retest passes), and the retest also gates the vsdd deviation-registry entries and the develop→main re-pin.

**gh#61 + gh#62 — one design cycle, two implementation slices.**
- vsdd's R4 is confirmed in code: the dial (#61) and the seam refinements (#62/R3) both land on `KickoffOpts` (`types.rs:81-99`) and the prompt-assembly step in `run.rs`, and `lifecycle.rs:729-746` is the single choke point that today hardcodes `model: "opus"`, `timeout: 3600`, and the permission posture — every dial and the per-agent template must flow through it. Two independent PRs would each rewrite that struct-literal and the `run.rs` template branch; sequential rebases guaranteed.
- **Recommended order:**
  1. **R1+R2 now, as its own small PR** (per vsdd's suggested split): `{{built_prompt}}`-style interpolation/append mode in `run.rs` step 5 + a per-dispatch `--template` flag into `KickoffOpts`. Same shape/size as PR #44; unblocks vsdd's consumer half of gh#62 without waiting on the design cycle, and stops consumers building against full-replacement semantics.
  2. **One design cycle for gh#61 + R3 (+the swarm permission plumbing from #66 subtlety 4)**: decide the final `KickoffOpts` shape once — effort/thinking-budget field recorded alongside `model` in run records (#61's ask), per-agent composition for swarm (per-phase template files vs the exec hook, R3's end-state), and de-hardcode `lifecycle.rs`. Implementation can then split into two reviewable PRs (dial first — smaller, and the manifest R4 wants to record dials needs the fields to exist before the injection hook can serialize them).
  3. **R5 is a design constraint on both:** the fully-assembled prompt must keep landing in the per-agent `KICKOFF.md` — it is vsdd's verification record. Any template/hook shape that bypasses that write breaks the consumer's conformance loop.

