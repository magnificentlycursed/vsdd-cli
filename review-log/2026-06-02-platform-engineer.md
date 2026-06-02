---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-06-02
phase: phase-3
scope: >-
  mdatron-enforcement automation venues — pre-commit hook (.githooks/pre-commit),
  vsdd-cli's own CI workflow (.github/workflows/mdatron-verify.yml), the deployed
  vsdd-init template (templates/.github/workflows/vsdd-verify.yml). Schema-tightening
  side-effect of 8a28512 (phase-primer.supplements_in_scope + domain-prompt.supplements_applied
  promoted to required) reviewed as a Phase-4-reaction-pattern. Two commits in scope —
  8a28512 + 87feb82.
lens: 5-lens application weighted to Edge cases (5) + Maintainability (4) + Consistency (4) + Attacker (3) + Usability (3). Primary domain Platform Engineer; supporting Software Engineer, Solution Architect, Security, Sanity Check.
source: director-raised
session_note: >-
  Cold-session reviewer-mode discipline maintained — no prior-session memory of the
  authoring choices; findings grounded in artifact text + git-blame evidence + a
  cross-repo read of mdatron-core/src/dsl/expr.rs:221-236 for the schema-tightening
  rationale. Composition is cluster-batched (primary PE + supporting SE/SA/Security/Sanity)
  rather than the per-domain spawn; the deviation is below the bar that fires VSDD-W
  meta findings — the surface (3 files, ~80 LOC) is small enough that cluster-batched
  is the canonical shape per Phase 3 primer.
model: claude-opus-4-7
execution_method: >-
  inline single-agent multi-domain composition (PE primary; SE / SA / Security /
  Sanity-Check lenses applied per the operator-directive cluster spec)
sycophancy_compensation: >-
  Reviewer is the same identity that authored the schema-tightening fix. The bias is
  to read "narrowest fix without DSL surgery" as load-bearing-correct rather than as
  the wrong-layer fix the DSL gap actually calls for. Compensation — F1 grounds the
  finding mechanically in expr.rs:230 (Field-on-Null returns Null; Field-on-Object
  raises FieldNotFound) so the load-bearing claim is verifiable cross-source, not
  rhetorical.
supplements_loaded: []
---

# Platform Engineer Review 1 — 2026-06-02

**Phase 3 cycle round:** 1 (opening round scoped to the three mdatron-enforcement venues; not a continuation of any prior round)

## Pre-phase composition declaration

```yaml
phase: phase-3
composed_domains: [platform-engineer, software-engineer, solution-architect, security, sanity-check]
composition_mode: cluster-batched-cold-session (primary + supporting lenses single-agent)
memory_isolation: cold-session (no prior context; cross-repo reads grounded in line citations)
operator_confirmation: confirmed (director-raised review of mdatron-enforcement milestone)
cluster_shape: 4-cluster-default (architecture-cluster-primary spawn)
declared_at: 2026-06-02T00:00Z
```

## Scope

Two commits — 8a28512 (pre-commit hook + schema-tighten) + 87feb82 (CI workflow + template threading). Total surface: ~80 LOC across `.githooks/pre-commit` (18 LOC bash), `.github/workflows/mdatron-verify.yml` (49 LOC), `templates/.github/workflows/vsdd-verify.yml` (added install step + comments), plus 4 lines of schema diff across `domain-prompt.json` + `phase-primer.json` (mirrored in `.mdatron/schemas/`).

## Findings

### F1 — Schema-tightening was the wrong layer; the bug lives in `mdatron-core/src/dsl/expr.rs:221-236` (Dim: spec-vs-implementation alignment + abstraction altitude; SA + SE)

**Evidence:**
- `mdatron-core/src/dsl/expr.rs:221-236`: `Expr::Field` matches `Value::Object` → returns `FieldNotFound` on missing key; matches `Value::Null` → returns `Null`; matches everything else → `TypeMismatch`.
- The asymmetry is the load-bearing defect. `Field-on-Null` is total; `Field-on-Object-missing-key` is partial. The DSL's `every($s in $self.supplements_applied, ...)` predicate composes Field with array iteration; the missing-key path explodes the predicate rather than producing an empty traversal.
- 8a28512 commit message names the root cause correctly ("Field access raises FieldNotFound rather than returning Null") and then walks past it ("Narrowest fix without DSL surgery").
- Pattern surface (`vsdd-core/patterns/cross-references.yaml:32, 48, 56`) already encodes the assumption that an absent collection should iterate zero times: `every(s in $self.supplements_applied, defined(...))`. That assumption is the natural semantics for `every` over a missing field.

**Why it matters (SA lens):** the schema was tightened to make a DSL invariant locally hold; the DSL contract that "every over a missing optional yields true vacuously" remains unstated. Every other artifact class that adds an optional collection-typed field will hit the same crash and reach for the same workaround. The fix replicates per-field rather than per-DSL-operator. This is the "decomposition gap dismissed as 'we'll handle it in milestone N+1'" sycophancy failure mode named in the SA prompt — except here it's "we'll handle it in 4 places per pattern."

**Why it matters (SE lens):** the canonical fix is one of (a) `Field-on-Object` on missing key returns `Null` (mirrors the Null branch — JSON-Pointer / jq / CUE semantics); (b) add a distinct `OptField` operator + thread it through the parser; (c) make `every` short-circuit-eval its collection arg via `try-or-empty`. Path (a) is the narrowest DSL change + matches the existing Null-tolerant branch + is one-line in expr.rs. The schema-tightening fix does NOT close the underlying defect — it papers over it for two specific schemas while leaving the trap armed for every future schema author.

**Sanity-check rubber-duck:** "What happens when a future author adds an `optional` collection field to `review-entry.json` and writes a pattern over it?" Answer with the current fix: same crash, same Phase-4-reaction loop. The fix doesn't generalize.

**Routing:** Phase 4 → mdatron-core repo (Raise to SO since it crosses the substrate boundary). Recommended path: `Value::Object(o).get(name).cloned().unwrap_or(Value::Null)` at expr.rs:224; document the contract in the DSL spec; revert (or keep, as defense-in-depth) the schema-required additions in a follow-up.

**Classification:** Accepted (the change shipped + is not regressive; the finding routes a follow-up to the right layer rather than blocking the milestone).

---

### F2 — `cargo install --path ../mdatron/mdatron-cli` is sibling-repo-layout-coupled; adopters without the layout get an unactionable hint (Dim: install-path realism; PE) — Open

**Evidence:**
- `.githooks/pre-commit:8`: install hint `cargo install --path ../mdatron/mdatron-cli`.
- vsdd-cli's own `Cargo.toml:14` declares `mdatron-core = { path = "../mdatron/mdatron-core" }` — the layout is hard-coded for vsdd-cli developers, not for end-users running the hook.
- `templates/.github/workflows/vsdd-verify.yml:39` correctly uses the public-git form: `cargo install --git https://github.com/magnificentlycursed/mdatron-cli mdatron --locked`. Two surfaces; two install commands; only the template form generalizes.

**Why it matters:** a maintainer of an adopter project clones vsdd-cli (or runs `vsdd init` and inherits the methodology), enables `core.hooksPath .githooks`, then sees the install hint and finds it nonsensical — they don't have a `../mdatron/` directory. The hint trains the operator to disable the hook ("it's broken on my box") rather than to install the tool. Fires the PE sycophancy failure mode "Hot-fix landed via direct push to main — bypass of the per-milestone PR discipline" inversely — a discipline that hand-waves on its own activation criteria is one operators learn to bypass.

**Routing:** Phase 4 → Phase 1a (one-line edit). Replace the hint with the git-form variant the template uses, or print both ("for sibling-repo dev: cargo install --path ../mdatron/mdatron-cli; otherwise: cargo install --git https://github.com/magnificentlycursed/mdatron-cli mdatron --locked"). Composes with F3.

**Classification:** Resolved-pending (mechanical fix).

---

### F3 — `cargo install vsdd --locked` + checkout-of-magnificentlycursed/mdatron-cli will fail until both repos are public; activation discipline silently breaks (Dim: CI workflow correctness; PE) — Open

**Evidence:**
- `.github/workflows/mdatron-verify.yml:27`: `repository: magnificentlycursed/mdatron-cli`. No token; relies on the repo being public + the checkout default GITHUB_TOKEN scope.
- `templates/.github/workflows/vsdd-verify.yml:34`: `cargo install vsdd --locked` — vsdd is not on crates.io per the milestone's own framing (mdatron is "not published to crates.io yet"; vsdd-cli is also pre-release).
- `templates/.github/workflows/vsdd-verify.yml:39`: `cargo install --git https://github.com/magnificentlycursed/mdatron-cli mdatron --locked` — assumes the repo exists at that URL + is public.
- Neither workflow has a publication-status gate or a pre-flight that surfaces the failure before the cache-key + cargo-install steps are wasted.

**Why it matters (PE lens):** the CI surface is asserted to be enforcement, but the failure mode when the upstream isn't yet published is opaque — `cargo install vsdd` fails with "no matching package found", the operator reads the workflow log, traces it back to the install step, and concludes the toolkit is broken. The intentional "non-enforcing fallback otherwise" discipline (pre-commit hook silent-skips when mdatron is absent) doesn't extend to CI — CI will hard-fail with a confusing message. Inverse asymmetry.

**Why it matters (Sanity-Check lens):** the CI's hard-fail-on-unpublished-toolkit is actually the right default in production. But the milestone ships the workflow template BEFORE either binary is on crates.io; adopters following the README will get red CI on day one. The defense is either (a) gate the template behind a published-binary check at `vsdd init` time, (b) ship both binaries to a GitHub Releases-based pre-built path with a fallback to git-install, or (c) document the prerequisite explicitly in the template comments.

**Routing:** Phase 4 → Raise to SO (publication-status gating is a v1-ship-criterion decision). Composes with F2.

**Classification:** Deferred-pending-SO.

---

### F4 — Bash hook is set -euo pipefail-correct but has two edge-case footguns (Dim: shell defensiveness; SE) — Open

**Evidence:**
- `.githooks/pre-commit:12`: `staged=$(git diff --cached --name-only --diff-filter=ACMR)`. When nothing is staged (e.g., the hook is invoked during an empty / amend-only / merge-commit scenario), `staged` is an empty string; `printf '%s\n' "" | grep -qE ...` searches a single newline against the regex, fails to match, and the hook exits 0. Behavior is correct; the path isn't documented.
- The regex `'(\.md$|^\.mdatron/|^vsdd-core/(schemas|patterns)/)'` is grep-compatible BUT `grep -qE` against a multi-line printf will match per-line correctly. However, filenames containing newlines (legal in POSIX) would break the match; git represents them as quoted paths by default but `--name-only` does NOT quote them under `core.quotepath false`. The injection-vector concern is small (this is a developer's own staged file) but not zero.
- No `LC_ALL=C` or `LANG=C` discipline on the regex — locale-sensitive grep on UTF-8 filenames is fine in practice but is a portability hazard the bash author should name.

**Why it matters:** the pre-commit hook is operator-local, but adopters with creative branch workflows (e.g., a merge commit that stages no .md files but stages a Cargo.lock change) get silent-skip behavior. The silent-skip is correct per intent; surface it in the hook output so the operator can verify "the hook ran + nothing was in scope" vs "the hook didn't run." Currently indistinguishable.

**Routing:** Phase 4 → Phase 1a (1-line addition: `echo "pre-commit: no markdown/schema/pattern files staged — skipping" >&2` in the no-match branch). Defensive `LC_ALL=C` declaration optional.

**Classification:** Resolved-pending (minor hardening; not load-bearing).

---

### F5 — Cache-key omits Rust toolchain version + does not key on mdatron source SHA; cache-poisoning surface (Dim: cache-key hygiene; PE + Security) — Open

**Evidence:**
- `.github/workflows/mdatron-verify.yml:40`: `key: cargo-${{ runner.os }}-${{ hashFiles('mdatron/Cargo.lock') }}`. Excludes: Rust toolchain version (rustup show fetches per-checkout `rust-toolchain.toml`; vsdd-cli pins 1.88; mdatron pins 1.88 — currently aligned but the cache key doesn't enforce alignment), the runner-image tag (ubuntu-22.04 is pinned in `runs-on` but not folded into the key — a future image bump silently invalidates / poisons the cache), and the mdatron source SHA (cache hits across mdatron PRs that don't touch Cargo.lock — code changes to mdatron-core won't bust the cache for a recompiled mdatron-cli).
- No `restore-keys:` fallback chain — a single-byte change to Cargo.lock means a full from-scratch cache rebuild. The cache is fragile in both directions: too sticky on source changes; too brittle on lockfile updates.

**Why it matters (PE lens):** "Build pinned to 'latest' — reproducibility traded for ergonomics; bites at release time" is the PE sycophancy failure mode in play. The cache key doesn't capture the build environment's actual reproducibility envelope; a green CI run today may be a poisoned-cache run tomorrow when the mdatron source has drifted but Cargo.lock hasn't. PE dim 1 (Reproducible builds) failure.

**Why it matters (Security lens):** cache poisoning across PR boundaries on `pull_request` triggers is a known supply-chain vector. GitHub's pull_request event runs in the base-repo context for caches scoped to the base ref; a malicious PR that mutates Cargo.lock can prime a cache that a subsequent legitimate PR consumes. The current key gives that PR full control over the cached `mdatron/target` directory.

**Routing:** Phase 4 → Phase 1b (cache key extension). Recommended: `key: cargo-${{ runner.os }}-${{ runner.arch }}-1.88-${{ hashFiles('mdatron/Cargo.lock', 'mdatron/rust-toolchain.toml') }}-${{ github.sha }}` with `restore-keys: cargo-${{ runner.os }}-${{ runner.arch }}-1.88-${{ hashFiles('mdatron/Cargo.lock') }}-`. Consider `actions/cache/restore` + `actions/cache/save` split so PR caches don't write to the shared key.

**Classification:** Resolved-pending (cache key extension).

---

### F6 — `rustup show` as toolchain install is fragile on hosted runners; `dtolnay/rust-toolchain` is the load-bearing canonical (Dim: runner-image choice + reproducibility; PE) — Open

**Evidence:**
- `.github/workflows/mdatron-verify.yml:30-31`: `Install Rust toolchain` step runs `rustup show`. On `ubuntu-22.04` GitHub-hosted runners, the default `rustup` is present + reads `rust-toolchain.toml` from the working directory (defaulting to the repo root). But the workflow checks out into `vsdd-cli/` and `mdatron/` subdirectories — there is no `rust-toolchain.toml` at the cwd at the time the step runs. `rustup show` will report the runner's default toolchain (currently 1.x, image-dependent) NOT the pinned 1.88.
- `mdatron/rust-toolchain.toml` pins 1.88; `vsdd-cli/rust-toolchain.toml` pins 1.88. Neither is read because the `rustup show` step's cwd is the workflow checkout root (above both subdirs).
- The `Install mdatron CLI` step runs `working-directory: mdatron` — at THAT point rustup will read mdatron's toolchain pin via auto-install. So the build succeeds, but for the wrong reason — the toolchain step is silently a no-op + the pin is enforced incidentally by the install step. Future developer reading the workflow can't derive that.

**Why it matters:** PE dim 1 (Reproducible builds) again — the workflow LOOKS reproducible (toolchain step is named) but its reproducibility actually rests on a side-effect of the install step. A refactor that hoists `cargo install` out of the `working-directory: mdatron` block (e.g., to enable build artifact reuse across the install + verify steps) would silently break toolchain pinning.

**Routing:** Phase 4 → Phase 1b. Either (a) replace the `Install Rust toolchain` step with `dtolnay/rust-toolchain@1.88` (explicit pin in the workflow), or (b) move the step's working-directory to `mdatron` so the toolchain file is read deterministically. Composes with F5 (cache key extension should include the toolchain version regardless).

**Classification:** Resolved-pending (1-line fix; either path).

---

### F7 — Coverage gap: force-push to main bypasses BOTH pre-commit hook (operator-local) AND CI workflow (PR-gated only); load-bearing-but-undefended (Dim: venue-coverage matrix; SA + Security) — Open

**Evidence:**
- Pre-commit hook activates only when `mdatron` is on PATH (and `core.hooksPath .githooks` is set; non-default — operator opts in per clone).
- `.github/workflows/mdatron-verify.yml:4-6` triggers: `pull_request:` + `push: branches: [main]`. The `push: main` trigger fires AFTER the push lands; for a `push --force`, the verification reports red AFTER the bad commit is canonical on main.
- No branch-protection enforcement declared in the workflow (cannot be — protection rules are a separate GitHub-API surface).
- Combined: a maintainer (a) without local mdatron installed, (b) bypassing the PR discipline via direct push or force-push, gets zero enforcement at commit-time + zero pre-merge enforcement.

**Why it matters (SA lens):** SA dim 5 (Hard-to-undo decisions named). Force-pushing a bad commit to main is reversible BUT the corruption propagates through any pull/fetch by adopters between the push and the revert — adopters following main's HEAD inherit the bad state. The venue matrix asserts three layers (hook + CI + template) but no layer covers the "post-push verification" surface.

**Why it matters (Security lens):** Security dim 6 (Supply-chain integrity). Force-push to a release branch is an attacker's payload-injection vector when the attacker has commit access (compromised maintainer credentials). The defense isn't in the verification venue; it's in branch-protection rules + signed commits + tag-signing. None of those are surfaced in this milestone.

**Routing:** Phase 4 → Raise to SO. Options: (a) document the venue-coverage matrix explicitly + name "branch protection" as the missing fourth venue + ship `vsdd init` with a branch-protection-rule API call, (b) accept the gap + document it in DESIGN-VERIFICATION as a known-limitation, (c) add a post-merge verification venue that opens a revert-PR when red.

**Classification:** Deferred-pending-SO.

---

### F8 — `working-directory: vsdd-cli` on the verify step runs `mdatron verify --project-root .` — but mdatron's own schemas under `mdatron/.mdatron/` are NOT checked by THIS workflow (Dim: workflow scope clarity; PE) — Open

**Evidence:**
- `.github/workflows/mdatron-verify.yml:46-48`: `working-directory: vsdd-cli` + `mdatron verify --project-root .` — verifies vsdd-cli's corpus only.
- The mirrored `.mdatron/schemas/{phase-primer,domain-prompt}.json` (the schema-tighten path-mirrored from `vsdd-core/schemas/`) are at `vsdd-cli/.mdatron/schemas/`, so they ARE covered. But the workflow's NAME (`mdatron verify`) reads as "verify mdatron + its consumers" — the operator who reads the workflow name expects mdatron's own corpus to be verified too.
- Inverse: if mdatron has its own CI that runs `mdatron verify --project-root .` in its own corpus, this is fine. But the milestone scope doesn't include that workflow + this review can't verify it without leaving scope.

**Why it matters:** consistency lens — the workflow name overpromises. Either rename to `vsdd-cli mdatron verify` to scope-narrow OR add a second job that checks out mdatron's corpus + runs verify against it.

**Routing:** Phase 4 → Phase 1a (rename, OR add a second job, OR add a comment block at the workflow top scoping the responsibility).

**Classification:** Resolved-pending (low-cost rename).

---

### F9 — `set -euo pipefail` + `git diff --cached` interaction: hook can be tricked into ignoring staged files via filename injection (Dim: bash defensiveness + attacker mindset; Security + SE) — Dismissed

**Evidence:**
- `.githooks/pre-commit:12`: `staged=$(git diff --cached --name-only --diff-filter=ACMR)` — git's `--name-only` produces newline-separated paths. Under `core.quotepath false`, non-ASCII filenames pass through as raw bytes; under `core.quotepath true` (default) they're C-quoted.
- A filename containing `.md` as a substring but NOT a suffix could fool the regex... no, the regex is anchored: `\.md$`. Multi-line filenames could match the `^\.mdatron/` anchor in a per-line scan. Actual filenames with newlines AND injection-shaped substrings would be needed; the threat model is "a developer staging their own files to trigger their own hook."

**Why it matters:** the attacker is the same identity as the operator. The hook is not a trust boundary; it is an opt-in convenience. Dismissing on threat-model grounds.

**Routing:** none.

**Classification:** Dismissed (no realistic attack surface; the hook runs in operator-local trust context).

**Dismissal_rationale:** Pre-commit hook executes in the developer's own workspace, after the developer has chosen to enable `core.hooksPath .githooks`. There is no untrusted-input path here — the operator authors the input + invokes the hook. Per Security domain prompt dim 2 (Trust boundary placement), validation inside a boundary against attacks from outside the same boundary is the load-bearing failure mode; this would be the inverse anti-pattern.

---

### F10 — CI workflow secret handling: template's `OTEL_EXPORTER_OTLP_HEADERS: Authorization=Bearer ${{ secrets.OTEL_COLLECTOR_TOKEN }}` is correct but adopter-facing footgun if secret unset (Dim: credential discipline; Security) — Accepted

**Evidence:**
- `templates/.github/workflows/vsdd-verify.yml:30`: `OTEL_EXPORTER_OTLP_HEADERS: Authorization=Bearer ${{ secrets.OTEL_COLLECTOR_TOKEN }}`. When `secrets.OTEL_COLLECTOR_TOKEN` is unset, the env var becomes literally `Authorization=Bearer ` (with trailing space) and the OTel exporter sends an unauthenticated request OR (depending on collector config) an empty-token request. Either way, the secret-handling discipline holds (no leakage); the failure mode is silent telemetry-drop.
- Not a load-bearing security defect in this milestone (the template was pre-existing); the install-step addition (`cargo install --git ... mdatron`) doesn't introduce new secret-handling surface.

**Routing:** none for this milestone; logged for the adjacent telemetry-CI hardening track.

**Classification:** Accepted (out of milestone scope; logged for visibility).

---

### F11 — Silent-skip pre-commit when mdatron absent is correct default + matches the "make adoption frictionless" methodology stance; but the install hint is the only discoverability surface — fragile (Dim: usability + adopter ergonomics; Sanity-Check + PE) — Resolved-pending

**Evidence:**
- `.githooks/pre-commit:6-10`: if mdatron is missing, print install hint + `exit 0`. The discipline-justification is in the commit message ("non-enforcing fallback so unconfigured clones aren't blocked").
- The `exit 0` IS the right default for vsdd-cli's own contributors (forcing a Rust toolchain + mdatron install at clone time would gate first-commit on toolkit setup — that's the "Hot-fix landed via direct push" failure mode in inverse). But the discoverability path is bash-only — the operator only sees the install hint if their commit happens to stage a markdown file (and even then it's on stderr, scrollable past). A developer who never stages markdown never learns the hook exists.
- The methodology's broader discipline asserts "shift-left" — surface the enforcement at the earliest place. Silent-skip on pre-commit is the inverse: postpone enforcement to PR-time. That's defensible per the activation-discipline framing BUT it should be named in the methodology spec as a deliberate trade-off, not as a default emergent from "hook isn't installed yet."

**Why it matters (Sanity-Check lens):** "Last-resort default to 'looks fine' when no other validator has authority — abdication dressed as routing" is the meta sycophancy. Silent-skip + opt-in install + opt-in `core.hooksPath` is three opt-ins stacked; the methodology spec is asserting enforcement, but the realized enforcement requires three operator actions. The CI venue is the actual enforcement layer; the hook is a convenience.

**Routing:** Phase 4 → Phase 1a (documentation). Two-paragraph addition to DESIGN-VERIFICATION naming the activation discipline + the explicit trade-off ("pre-commit is opt-in convenience; CI is load-bearing enforcement; force-push to main is the gap"). Composes with F7.

**Classification:** Resolved-pending (documentation-only; activation discipline itself is sound).

---

## Round-close summary

**11 findings raised this round. None Hallucinated. Two Dismissed/Accepted (F9 + F10 out-of-scope). Round MUST continue (Phase 3 round-trigger: active domain produced real findings).**

| Finding | Domain | Classification | Routing | Composes with |
|---|---|---|---|---|
| F1 | SA + SE | Accepted | mdatron-core → Raise to SO | — |
| F2 | PE | Resolved-pending | Phase 4 → 1a | F3 |
| F3 | PE + Sanity | Deferred-pending-SO | Phase 4 → SO | F2 |
| F4 | SE | Resolved-pending | Phase 4 → 1a | — |
| F5 | PE + Security | Resolved-pending | Phase 4 → 1b | F6 |
| F6 | PE | Resolved-pending | Phase 4 → 1b | F5 |
| F7 | SA + Security | Deferred-pending-SO | Phase 4 → SO | F11 |
| F8 | PE | Resolved-pending | Phase 4 → 1a | — |
| F9 | Security + SE | Dismissed | — | — |
| F10 | Security | Accepted | — | — |
| F11 | Sanity + PE | Resolved-pending | Phase 4 → 1a | F7 |

**MVR signal:** NOT YET. 6 Resolved-pending + 2 Deferred-pending-SO + 1 Accepted-with-routing + 2 closed (Dismissed + out-of-scope-Accepted); zero Hallucinated. Phase 3 cycle continues.

**Phase 4 routing recommendation:**
1. **Mechanical-fix bundle (F2 + F4 + F8):** install-hint generalization + bash silent-skip surfacing + workflow-name scoping. Sub-15-min single-commit.
2. **Cache + toolchain bundle (F5 + F6):** cache-key extension + toolchain pin explicitization. Single CI workflow edit; ~30-min.
3. **DSL-layer fix (F1):** routes cross-substrate to mdatron-core. Replace `Field-on-Object-missing-key` to return `Null`; document the contract; consider whether to keep the schema-required tightening as defense-in-depth or revert it.
4. **SO-disposition bundle (F3 + F7 + F11):** publication-status gating, force-push coverage gap, activation-discipline documentation. Single operator session.

**Cross-finding coherence (sanity-check dim 2):** F1 + F7 + F11 form a coherent methodology-spirit pattern — the milestone ships three venues with intentional gaps (silent-skip pre-commit; post-push CI; opt-in operator install) and treats the gaps as features. They ARE features when named in the methodology spec; they are footguns when emergent. The compensation is the documentation route (F11) + the SO disposition on the venue matrix (F7) + the DSL-layer canonicalization (F1).

**Sycophancy-compensation reflection:** the bias I resisted is treating the schema-tightening (F1) as load-bearing-correct because the commit message frames it as such. The DSL-level read (expr.rs:230 has Field-on-Null already returning Null; the Object-missing-key branch should mirror it) is one-liner-decisive. The schema fix shipped; that doesn't make it the right layer. The Accepted classification + routing-to-substrate is the honest disposition.

## Cross-references

- `.githooks/pre-commit` (F2, F4, F9, F11 — surface)
- `.github/workflows/mdatron-verify.yml` (F3, F5, F6, F8 — surface)
- `templates/.github/workflows/vsdd-verify.yml` (F3, F10 — surface)
- `vsdd-core/schemas/{phase-primer,domain-prompt}.json` (F1 — schema-tighten path)
- `/Users/claire.celesterra/Documents/Source/magnificentlycursed/mdatron/mdatron-core/src/dsl/expr.rs:221-236` (F1 — root-cause)
- `vsdd-core/patterns/cross-references.yaml:32, 48, 56` (F1 — pattern surface consuming the tightened schemas)
- `Cargo.toml:14` (F2 — sibling-repo layout assumption surfacing in the install hint)
- 8a28512 + 87feb82 (milestone commits)
