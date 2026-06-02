---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-06-02
phase: phase-1c
scope: >-
  Architectural opinion on `vsdd init` v0.1 drift / collision / upgrade design.
  Subject — DESIGN-METHODOLOGY.md:818-870 (collision-handling spec) vs
  vsdd-core/src/init.rs (current two-way hard-error impl) vs
  crosslink/src/commands/init/{manifest.rs,merge.rs,mod.rs} (cited substrate).
  No code modification — opinion only, routed to SO for spec disposition.
lens: >-
  Solution Architect primary (decomposition coherence + hard-to-undo decisions +
  cross-milestone seams + abstraction altitude). Sanity Check baseline (always-on)
  rubber-ducking the four named questions in the prompt against the spec ↔
  implementation ↔ substrate triangle.
source: director-raised
session_note: >-
  Cold-session single-domain composition (SA-primary + Sanity-Check baseline).
  No prior cycle context loaded except the four named referent files. Filed as
  Phase 1c because the design question is "what should the milestone scope and
  decomposition be?" — not Phase 3 (no committed-code adversarial pass) and not
  Phase 2b (no implementation in flight). Companion file
  `2026-06-02-solution-architect.md` is a different scope (Phase 3 on the
  shipped v0.1 init); this entry is the upstream decomposition opinion that
  finding F3 + F5 of the SO round routed toward.
model: claude-opus-4-7
execution_method: >-
  cold-session sub-agent dispatched from main session; SA prompt + Sanity Check
  prompt + review-entry schema + recent SO example loaded fresh; no prior-cycle
  context.
sycophancy_compensation: >-
  The cheaper-to-keep path for the implementor identity (Claude) is to keep
  the current two-way hard-error model and rename the flags out of the error
  message. The prompt specifically asks for pressure on that bias. I apply it
  below — the rename-only path is named as one option but rejected as under-
  architecting because the substrate the spec cites (crosslink) already
  established what "playing nicely" means and the cost gap between two-way and
  three-way is shallow at the manifest layer where it matters most.
filename_note: >-
  Filed under `solution-architect-init-drift` slug to disambiguate from the
  same-day `solution-architect.md` Phase 3 entry. Both are SA-lens; this one
  is Phase 1c decomposition opinion, the other is Phase 3 on shipped code.
supplements_loaded: [rust, cli, json]
---

# Solution Architect Opinion — `vsdd init` v0.1 drift design

**Phase 1c opinion round. No code modification. SO disposes.**

---

## Headline recommendation

**Adopt crosslink's three-way classification at the manifest layer + adopt crosslink's actual flag surface (`--update` / `--force` / `--no-prompt` / `--dry-run`) verbatim. Defer per-file-type merge strategies (managed-section markers, JSON-merge) to v0.2; v0.1 ships three-way classify with whole-file overwrite-on-AutoUpdate and refuse-on-Conflict. Delete `--keep-operator-edits` / `--accept-managed-defaults` from the error message (and from DESIGN-METHODOLOGY.md:854) — they are spec hallucination, not crosslink heritage.**

## The four architecture questions

### 1. Two-way vs three-way classification

Two-way is **insufficient** for v0.1. The current impl (`vsdd-core/src/init.rs:114-125`) compares only `prior_entry.sha256` vs `actual_sha` (manifest-vs-current). This conflates two distinct cases under "drift":

- **Case A** (operator edited a managed file, toolkit unchanged): the spec's "drifted managed file" — refuse + escalate is correct.
- **Case B** (operator untouched, toolkit upgraded the template): a routine upgrade path — auto-update is correct.

The two-way model treats both as drift and refuses both. Concretely: when the operator runs `cargo install vsdd@0.2` and then `vsdd init`, every artifact whose source bytes changed in v0.2 will be flagged as drift (because `actual_sha == prior_manifest.sha256 == v0.1_template_sha != v0.2_template_sha`, but the impl never computes the v0.2 template hash to distinguish). The impl will then incorrectly auto-overwrite — see `vsdd-core/src/init.rs:127-133`: it compares `actual_sha == source_sha` and silently overwrites when they differ but the prior-manifest check passed. That branch is a footgun masquerading as upgrade support: if the operator never touched the file AND the prior manifest matches, `actual_sha == prior_entry.sha256` and the source-equality check at :127 falls through to overwrite without any classification step. It works by accident for the (false, true) case but cannot distinguish (true, true) Conflict from (true, false) TemplateUnchanged.

The prompt notes vsdd-cli ships a fixed canonical artifact set, not user templates — so "template changed" only fires on toolkit upgrade. **That's exactly when the discipline matters**: toolkit upgrades are the recurring re-init event, and v0.1 will accumulate operators stuck across version boundaries within weeks of release. Two-way can't disambiguate the four substantive cases (UpToDate / AutoUpdate / TemplateUnchanged / Conflict) that crosslink/src/commands/init/manifest.rs:145-150 already enumerates. The cost of three-way at the manifest layer is roughly +20 LoC (compute `new_template_hash` from `source_bytes`, store enum, branch in run loop) — trivial relative to the cost of shipping an upgrade story that requires operators to manually delete the manifest.

**Verdict: three-way is load-bearing for v0.1.** The substrate already implements it; mirroring it is cheaper than not.

### 2. Per-file-type collision strategy

Crosslink ships **three strategies** (whole-file via `classify_update`; managed-section markers in `merge.rs:48-86` for `.gitignore`; structural JSON merge in `merge.rs:91-228` for `.mcp.json` + `.claude/settings.json`). The vsdd spec at DESIGN-METHODOLOGY.md:822-828 mirrors this matrix verbatim and adds two more (CODEOWNERS, pre-commit-config.yaml).

**For v0.1, the per-file-type strategies are NOT load-bearing yet** — because the v0.1 artifact set (per `vsdd-core/src/init.rs:207-248`) deploys schemas, patterns, phase primers, domain prompts, supplements. **Zero of these are shared-namespace operator-collision files.** The v0.1 deployed set is all under vsdd-owned namespaces (`.mdatron/`, `.claude/commands/vsdd-*`, `supplements/`). Per the spec's own discipline at :844-846, vsdd-prefixed paths have no collision.

The collision-prone files (`.gitignore`, `.mcp.json`, `.claude/settings.json`, `.github/CODEOWNERS`, `.pre-commit-config.yaml`) are **not in the v0.1 deployment plan**. Until they are, per-file-type strategies are dead code.

**Verdict: ship uniform whole-file discipline in v0.1.** Per-file-type strategies enter when the file class enters the deployment plan — milestone-coupled, not pre-built. SA dim 7 (abstraction altitude / rule-of-three) says don't abstract on one repetition.

### 3. Flag surface coherence

`--keep-operator-edits` / `--accept-managed-defaults` appear nowhere in crosslink (verified: grep across `crosslink/src/commands/init/`). The spec's claim at DESIGN-METHODOLOGY.md:854 — "inherited from crosslink" — is **factually wrong**; crosslink's flags are `--force` / `--update` / `--no-prompt` / `--dry-run`. This is spec hallucination, not aspirational naming: the spec text presents the flags as inherited, not proposed.

Architecture cost asymmetry:
- **Persist with vsdd-spec-named flags**: invent semantics that don't exist upstream, diverge from the substrate the spec cites, take on novel-vocabulary maintenance burden (per the methodology's own earned-by-recurrence trigger at :870-872 — single-recurrence terms ship as `status: candidate`, and these have zero recurrence).
- **Mirror crosslink's flag shape**: zero invention cost; the operator-facing semantics are documented upstream; future cross-tool composition (e.g., a meta-command that runs both `crosslink init --update` and `vsdd init --update`) is mechanically coherent.

The asymmetry isn't close. Mirror crosslink. The spec is wrong; amend the spec rather than chase the hallucination into the implementation.

**Sub-recommendation**: `--check` (already in `vsdd/src/main.rs:25-34`) is fine — it's the spec's preflight at :856-858 — but rename to `--dry-run` to match crosslink's naming. `--ci-mode` is a separate concern (SO finding F8 from the same-day round) and is not a drift flag.

### 4. Substrate boundary — "plays nicely with existing projects"

Per DESIGN-METHODOLOGY.md:818 the promise is concrete: collision-handling discipline inherited from crosslink. The minimum architecture to honor that promise in v0.1:

1. **Three-way classify at manifest layer** (covered in Q1).
2. **Refuse-on-Conflict by default** (matches the spec's "refuse rather than overwrite" intent at :848-850).
3. **`--force` to override** (operator says "I know what I'm doing; deploy anyway").
4. **`--dry-run` / `--check` to preview** (no surprises; spec :856).
5. **Refuse-on-malformed-manifest** (spec :850 explicit; current impl swallows at `init.rs:267` — this is the SO-round F7 finding, but it composes here).

That's it. Per-file-type merge can come in v0.2 when the merge-prone files enter the plan. Side-by-side templates (`*.vsdd-template`) compose at the deployment-plan layer, not the drift-handling layer.

### 5. Hard-to-undo dimension (SA dim 5)

Decisions made now that **lock in shape**:

- **Manifest schema** (`vsdd-core/src/init.rs:276-288`): hard-to-undo. The nested `ManifestEntry` shape is fine, but it MUST carry `manifest_schema_version: "1.0.0"` (per the SO round F10). Three-way classify needs to know what fields the manifest carries; future additions (e.g., recording the `vsdd_version_at_deploy` per-file for cross-version diagnostics) need the version field to disambiguate readers. **Add the version field before any operator project ships a v0.1 manifest.**
- **Flag names**: medium-hard-to-undo. CLI surface enters muscle memory + CI scripts + docs. Renaming `--keep-operator-edits` later costs a deprecation cycle; getting the names right at v0.1 costs nothing. Mirror crosslink now.
- **Refuse-vs-prompt-vs-overwrite default on Conflict**: medium-hard-to-undo. Crosslink prompts (`mod.rs:530-540`); refusing is stricter. v0.1 can ship refuse-by-default + `--force` (strict cut); v0.2 adds prompt mode behind `--update` if/when operator feedback says strict-refuse-is-too-painful. Strict-first is reversible; lenient-first is harder to tighten.

Decisions that are **easy to change later**:

- Adding per-file-type merge strategies (gitignore markers, JSON merge) — these compose at the file-class layer; adding them doesn't change the manifest format or flag surface.
- Adding interactive prompt mode (crosslink's `--no-prompt` opt-out shape) — composes on top of the strict refuse-by-default.

## Applied bias pressure

The implementor-cheap path is: leave two-way hard-error, rename the flags in the error message to `--force` (and remove the second one entirely), ship. That **does** close the SO-round F3 finding (misleading error). But it leaves the v0.1 architecture unable to distinguish (false, true) AutoUpdate from (true, true) Conflict — meaning the first toolkit upgrade after v0.1 release puts every operator on the manual-delete-manifest path. The implementor saves a day of work in v0.1 and spends a week of operator-support and a spec-amendment cycle in v0.2.

Three-way classify at the manifest layer is **~20 LoC** plus the enum (mirrorable from `crosslink/src/commands/init/manifest.rs:35-48` and :136-152). Refusing to ship it is the under-architecting failure mode.

## Routing

- **Raise to SO** for spec amendment: DESIGN-METHODOLOGY.md:854 flag-names are wrong; replace with `--force` / `--update` / `--no-prompt` / `--dry-run`. Also amend :818 — "inherited from crosslink's collision-handling" is true for the file-class matrix only; the drift-classification + flag surface should be cited as the same inheritance, with the actual upstream flag names.
- **Phase 1c → Phase 1b** for: (a) `manifest_schema_version` field added to manifest spec; (b) `UpdateAction` enum + three-way classify named in DESIGN.md § Verification architecture.
- **Phase 2b** for implementation: three-way classify + flag surface + refuse-on-Conflict-with-`--force`-override + refuse-on-malformed-manifest (composes with SO-round F7).
- **NOT in v0.1**: per-file-type merge strategies. Re-enter when the first collision-prone file (likely `.gitignore`) enters the deployment plan.

---

## Cross-references

- `DESIGN-METHODOLOGY.md:818-862` (collision-handling spec; hallucinated flag names at :854)
- `vsdd-core/src/init.rs:55-58` (hallucinated flag names in error)
- `vsdd-core/src/init.rs:107-141` (two-way drift loop + silent-overwrite footgun at :127-133)
- `vsdd-core/src/init.rs:260-271` (corrupt-manifest swallow; composes with SO-round F7)
- `vsdd-core/src/init.rs:276-288` (manifest schema; SO-round F10)
- `crosslink/src/commands/init/manifest.rs:33-48` (UpdateAction enum)
- `crosslink/src/commands/init/manifest.rs:136-152` (three-way classify function)
- `crosslink/src/commands/init/merge.rs:8-228` (per-file-type strategies; v0.2 reference)
- `crosslink/src/commands/init/mod.rs:417-540` (run_update orchestration; flag surface reference)
- `review-log/2026-06-02-solution-owner.md` F3 + F5 + F7 + F10 (composes with this opinion)
