# Phase 1a — Behavioral Specification (DRAFT)

**Issue:** vsdd-cli #14 (Phase 3 of the binary-first plan).
**Parent plan:** [`../binary-first-plan.md`](../binary-first-plan.md) § Phase 3.
**Consumes:** [Phase 2 mdatron `--json`](../phase-2-mdatron-json/phase-1a-behavioral-spec.md) — the output-envelope + exit-code contract, as shipped by **mdatron v0.3.0**; and the current `vsdd-core` (the `mdatron-core` path dependency is already removed; `tests/cross_references.rs` already spawns the binary).

> **STATUS: DRAFT authored 2026-07-29 for the operator's phase-1a composition + review — NOT ratified and NOT a phase entry.** The composition (which domains) and the confirmation are the operator's; this is a starting point so the design work is teed up rather than blank. It is a design doc under `docs/refactor/` (outside mdatron's jurisdiction), not a contract edit. The Architecture-section workspace-shape contract touch belongs to #15/Phase 4, not here (see Out of scope).

## Pre-phase composition declaration (PROPOSED — operator confirms)

```yaml
phase: phase-1a
# Proposed from #14's surface: a CLI + manifest change (SE/PE), a subprocess
# trust boundary (SA/SE), a breaking manifest-format change (DE), and an
# interactive prompt (UX may be warranted). Operator adjusts.
composed_domains: [solution-owner, solution-architect, software-engineer, platform-engineer, data-engineer, documentation-reviewer]
composition_mode: skill-interactive
operator_confirmation: PENDING
declared_at: <on entry>
```

## Scope

Three capability areas, all v0.1.0 blockers, per the plan's Phase 3:

1. **The mdatron subprocess client** — vsdd invokes `mdatron verify --json` as a subprocess, parses the output envelope, and maps exit codes to typed outcomes.
2. **init drift-handling** — the current two-way classification (manifest vs disk) becomes three-way (manifest vs disk vs new-template), with a `--force` / `--update` / `--no-prompt` / `--dry-run` flag surface, an interactive Conflict prompt, and a `template_version_at_deploy` manifest field.
3. **Template deployment** — `vsdd init` deploys the `templates/*` artifacts alongside the 42 markdown artifacts it already deploys.

Explicitly retained shape: vsdd stays a two-crate workspace this phase (the `vsdd-core` removal is #15/Phase 4).

## Behavioral contracts

### A. The mdatron subprocess client

The client is the single seam through which vsdd calls mdatron. It builds on the existing bounded subprocess runner (`vsdd-core/src/subprocess.rs`). Every contract below is observable from the client's returned value.

- **A1 — invocation shape.** The client spawns `mdatron verify --json --project-root <root>` (no shell; args passed directly). It reads stdout as the sole machine channel.
- **A2 — clean.** Exit `0` + `pipeline_status: "ok"` + empty `findings` → a `Clean` outcome carrying the parsed envelope (`mdatron_version`, `families`, `summary`). No finding is fabricated.
- **A3 — findings.** Exit `1` + `pipeline_status: "ok"` + non-empty `findings` → a `Findings` outcome carrying every finding verbatim (`code`, `severity`, `location{file,line,column}`, `summary`, `message`, `help`, `explain_ref`, `quoted[]`). No finding is dropped or truncated.
- **A4 — pipeline failure is NOT clean.** Exit `2` / `pipeline_status: "failed"` → a `DidNotRun` outcome that is *distinct from Clean* and carries the failure reason. This is the load-bearing contract: a caller must never read a pipeline failure as clean (the exact class behind vsdd-cli #816 / #822 M1). Edge: under `--json` the E0080 reason currently lives on stderr, which `-q` suppresses — see Open question Q1.
- **A5 — unusable checker.** Exit `≥101`, non-JSON stdout, or an unparseable envelope → an `Unusable` error (loud). Never degraded-to-clean.
- **A6 — checker absent.** mdatron not on PATH → a `NotInstalled` error, distinct from every run outcome (fail-closed, matching the pre-commit precedent).
- **A7 — envelope version.** The client honors `mdatron_output_version`: a **major** mismatch is a distinct `IncompatibleEnvelope` error (an incompatible envelope is never parsed as Clean); unknown **minor/additive** fields are tolerated (forward-compatible reading).
- **A8 — bounded.** The subprocess carries a deadline; a hung mdatron yields a `TimedOut` outcome, never a hang.
- **A9 — quoted content is untrusted.** `findings[].quoted[].content` is adopter/external data (the visible-prose injection class, vsdd-cli #818). When the client surfaces it, it is marked/treated as untrusted — never rendered as vsdd's own instruction voice.
- **A10 — coverage is not inferred from `files_checked`.** The client does not treat `summary.files_checked > 0` as proof of coverage (it counts findings-bearing files, vsdd-cli #822 M2). Coverage is not derivable from today's envelope.

### B. init drift-handling (two-way → three-way)

Today (`vsdd-core/src/init.rs`): two-way — Unchanged (disk==manifest) is skipped; a disk hash differing from the manifest is refused as drift; no-drift-but-template-differs silently overwrites (upgrade). No flags; `InitOptions` carries only an unused `ci_mode`; `ManifestEntry` is `sha256`-only.

- **B1 — three-way classification.** Each managed file is classified from (manifest hash, current-disk hash, new-template hash): **Unchanged** (all equal → skip), **ToolkitUpgrade** (disk==manifest ≠ template → update), **OperatorEdited/Conflict** (disk ≠ manifest → do not silently overwrite), **Missing** (absent → deploy). Mirrors crosslink's `classify_update`.
- **B2 — `--dry-run`.** Prints the per-file classification + planned action; writes nothing; exit 0. (No manifest write, no event.)
- **B3 — `--force`.** Overwrites Conflict (OperatorEdited) files with the new template and records the new hash. This replaces today's hard refuse-on-drift as the explicit override.
- **B4 — `--update`.** Applies ToolkitUpgrade updates; leaves Conflict files untouched (unless `--force`).
- **B5 — `--no-prompt`.** Non-interactive: Conflict files are **skipped** (never overwritten) unless `--force`; suitable for CI. `ci_mode` implies `--no-prompt`.
- **B6 — interactive Conflict prompt** (default; TTY; not `--no-prompt`). On a Conflict, the operator is prompted per file: keep the operator edit / accept the new template / show the diff. The choice is applied per file.
- **B7 — `ManifestEntry.template_version_at_deploy`.** Each entry records the toolkit version whose template it deployed, enabling three-way classification across upgrades. This is a **breaking manifest-format change** and must land before v0.1.0 publishes. A manifest lacking the field is treated as pre-Phase-3 and re-classified conservatively (see Q4).
- **B8 — idempotence preserved.** All-Unchanged → no writes, no `ProjectInitialized` event, exit 0 (unchanged from today).
- **B9 — errors.** Non-git directory → refuse (unchanged); IO error → typed IO error; corrupt manifest → treated as first-init (unchanged), never a false drift.

### C. template deployment

- **C1 — templates deployed.** `vsdd init` deploys the `templates/*` artifacts to their adopter destinations alongside the 42 markdown artifacts (the current plan deploys 4 schemas + 1 pattern + 10 primers + 18 domain prompts + 14 supplements; templates were explicitly deferred, `init.rs:14`).
- **C2 — templates are managed.** Deployed template files enter the manifest (hashed, drift-tracked) and are subject to the same three-way classification (B) as every other managed artifact.
- **C3 — destinations.** Each template deploys to its adopter location (e.g. the two CI workflow templates → the adopter's `.github/workflows/`; the DESIGN template + registry data sets → their documented adopter paths). The exact template set + destination mapping is Q2.

## Edge cases + error conditions to cover

mdatron: absent (A6), old/incompatible envelope (A7), timeout (A8), crash/non-JSON (A5), pipeline-failure-vs-clean (A4), quoted-content injection (A9). init: corrupt/absent/pre-Phase-3 manifest (B7/B9/Q4), a Conflict on a *template* file (C2), interrupted init (partial deployment — atomicity, Q3), `--force --dry-run` together, `--no-prompt` in a TTY, a template whose destination path collides with a non-managed file.

## Out of scope

- The `vsdd-core` removal / workspace re-org and its **Architecture workspace-shape contract touch** — that is #15 / Phase 4 (this phase keeps the two-crate workspace).
- mdatron's own `init` / `config.yaml` (Phase 5 — already shipped in mdatron v0.2/v0.3).
- crates.io publish + CI install-hint migration (Phase 6 / #48).

## Open questions (for the design)

- **Q1 — the pipeline-failure reason channel (A4).** mdatron under `-q` suppresses the E0080 reason to stderr only. Does the client (a) run mdatron **without** `-q` and capture stderr for the reason (brittle — human prose), or (b) rely on the reason reaching the JSON envelope? (b) is the durable fix and is already raised in the diagnostic-efficacy feedback ("carry the failure reason into the envelope"); (a) is a viable interim. Recommend: interim (a) capturing stderr, with a note that the envelope fix retires it.
- **Q2 — the template set + destinations (C1/C3).** Enumerate `templates/*` and each deploy destination from the artifact registry + the adopter layout.
- **Q3 — init atomicity.** Is a partially-completed init (interrupted mid-deploy) required to be recoverable/atomic, or is re-run-to-converge sufficient (today's posture)?
- **Q4 — manifest migration (B7).** How is an existing manifest without `template_version_at_deploy` handled — re-hash and adopt current templates as the baseline, or force a Conflict review? This is the breaking-change migration path.
- **Q5 — where the client lives.** `src/mdatron.rs` vs `src/commands/verify/subprocess.rs` (the plan lists both) — a Phase-4 module-map question that only needs a placeholder here.
