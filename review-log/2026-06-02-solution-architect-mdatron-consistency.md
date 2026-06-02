---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-06-02
phase: phase-1c
scope: >-
  Architectural opinion on mdatron's CLI surface + config-discovery convention +
  error-handling paradigm, evaluated against crosslink as substrate baseline in
  pursuit of the operator-stated "absorbability" goal. Subject —
  mdatron/mdatron-cli/src/main.rs (verify + explain verbs),
  mdatron/mdatron-core/src/{error,diagnostic,verify,schema}.rs (typed-variant
  thiserror discipline), mdatron/mdatron-core/src/dsl/{expr,index}.rs
  (EvalError, IndexError). Compared against crosslink/crosslink/src/main.rs
  + crosslink/crosslink/src/commands/init/mod.rs (anyhow + .with_context).
  No code modification — opinion only, routed to SO for disposition ahead of
  Phase 1a authoring of `mdatron init`.
lens: >-
  Solution Architect primary (decomposition coherence + abstraction altitude +
  hard-to-undo decisions + cross-cutting concerns + trust boundary). Sanity
  Check baseline rubber-ducking the five operator-named questions against the
  mdatron <-> crosslink substrate gap and the standalone-tool-consumed-by-vsdd
  identity.
source: operator-directive
session_note: >-
  Cold-session single-domain composition (SA-primary + Sanity-Check baseline).
  No prior cycle context loaded except the five named referent files plus
  ambient mdatron-core source. Filed as Phase 1c because the operator framing
  is "what shape should mdatron's surface take before Phase 1a authors
  `mdatron init`?" — pre-implementation decomposition opinion. Sibling entry
  2026-06-02-solution-architect-init-drift.md treats the same date's vsdd init
  drift question; this entry treats the upstream mdatron consistency question.
model: claude-opus-4-7
execution_method: >-
  cold-session sub-agent dispatched from main session; SA prompt + Sanity Check
  prompt + review-entry schema + sibling format example loaded fresh; no
  prior-cycle context.
sycophancy_compensation: >-
  The cheaper path is to bless mdatron's current sparseness as "principled
  minimalism" because the codebase reads cleanly and the typed-variant
  discipline is technically defensible in isolation. The prompt explicitly
  pressures this. I apply pressure below — sparseness is principled at the
  library boundary (mdatron-core) but accidental at the CLI boundary, and the
  absent top-level config convention is a hard-to-undo decision being deferred
  by default rather than by design.
filename_note: >-
  Filed under `solution-architect-mdatron-consistency` slug to disambiguate
  from same-day `solution-architect-init-drift.md` (vsdd init scope) and
  `solution-architect.md` (vsdd Phase 3 shipped-code scope).
supplements_loaded: [rust, cli]
---

# Solution Architect Opinion — mdatron CLI + config + error-handling consistency

**Phase 1c opinion round. No code modification. SO disposes.**

---

## Headline recommendation

**Keep mdatron-core's thiserror typed-variant discipline (library-correct); adopt anyhow + .with_context at mdatron-cli's main.rs boundary (binary-correct, mirrors crosslink). Formalize `.mdatron/config.yaml` discovery now as part of Phase 1a `mdatron init` authoring. Stay flat for v0.1 verbs but reserve namespace shape by NOT burning collision-prone root verbs (`check`, `test`, `list`, `generate`). Stay library-first: mdatron-core is the contract; semver lives there.**

## The five questions

### 1. Error-handling — split paradigm at the crate boundary

mdatron-core's thiserror is correct and load-bearing. Library crates publish typed errors so callers (vsdd, future adopters, mdatron-cli itself) match on variants. `VerifyError::SchemaLoad`, `EvalError::TypeMismatch`, `IndexError::PathTraversal` (carries MDATRON-E0011 per `dsl/index.rs:98-100`) are the shape downstream embedders need.

mdatron-cli's pattern is accidental. `main.rs:90-96` matches `VerifyError` only to print + exit 2; `:71-77` hand-rolls `MDATRON-E0070`. No anyhow dep. Fine for a 140-LoC binary, inadequate for Phase 1a `mdatron init` — first multi-step orchestration, exactly crosslink's shape where `.with_context` does real work at `crosslink/src/commands/init/mod.rs:586,601,629`.

**Verdict**: thiserror at core, anyhow at CLI. They compose via `From`. **Sub-rec**: move `MDATRON-E0070` / `MDATRON-E0080` from `main.rs` into mdatron-core; single-source the catalog.

### 2. Diagnostic surface — rustc-shape is right; TTY layer for orchestration only

Findings must remain TTY-invariant — vsdd and editors parse them. Crosslink's TUI is right for crosslink's role; disjoint. What IS unclear: orchestration output (summary at `main.rs:113-116`, progress, anyhow chains). Add TTY-detection there only, mirroring crosslink's `InitUI`. Findings stay rustc-shape. Name the seam in DESIGN-MDATRON.md.

### 3. Subcommand surface — flat now, reserve namespaces

2 verbs is principled minimalism at v0.1 (SA dim 7 rule-of-three). Crosslink's 30+ evolved across milestones; mdatron is at milestone one.

Plausible v0.2-v0.5: `schema check/generate/list`, `pattern test/list`. **Verdict**: stay flat (verify, explain, init). But do NOT take root verbs `check`, `test`, `list`, `generate`, `schema`, `pattern` — they will become subcommand-noun namespaces. CLI surface is hard-to-undo (SA dim 5): reserving costs nothing now, costs a deprecation cycle later.

### 4. Config discovery — the load-bearing finding

mdatron has no top-level config today. `VerifyConfig` at `verify.rs:38-57` is CLI-flags + conventions. `mdatron init` will write the precedent. Within v0.2 operators will want: `file_globs` defaults, per-rule severity overrides, version pin, code-prefix mappings. None belong as CLI flags (per-invocation wrong) or per-file (cross-cutting wrong). Only top-level config fits.

**Verdict**: formalize `.mdatron/config.yaml` in Phase 1a. Schema declared in mdatron-core as a typed struct + JSON Schema (self-validating via Layer 1). Precedence: CLI flag > env > config > defaults. v0.1 minimum keys: `version`, `file_globs`. Everything else accretes.

Hard-to-undo (SA dim 5): "No config in v0.1" IS a decision — it locks operators into CLI-flag-only and reversing later costs migration. Adding now with deliberately minimal keys costs nothing.

### 5. Library-first — correct; name the implications

The operator directive ("vsdd consumes mdatron as a dependency") IS the decision. mdatron-core is the contract; mdatron-cli + vsdd are two consumers. Crosslink is binary-first because it has one consumer; mdatron has two.

**Implications**:

- **Semver lives on mdatron-core**. Breaking `Finding` fields breaks vsdd. Workspace single-version for v0.1; revisit when cadences diverge.
- **Error catalog is a versioned public surface**. Rules reference `MDATRON-E0001`, `MDATRON-E0011`, etc. Renumbering is breaking. Name in DESIGN-MDATRON.md § Public contracts.
- **Diagnostic JSON is a versioned contract**. `Finding` derives `Serialize` (`diagnostic.rs:57`). Ship `mdatron verify --format=json` in v0.1 (one branch in `cmd_verify`) so the JSON shape is exercised + tested from day one.

## Applied bias pressure

Implementor-cheap path: ship mdatron as-is — 2 verbs, no config, typed errors — because v0.1 doesn't require more. Fine for `mdatron verify` alone, inadequate for what's landing: (a) Phase 1a init without anyhow regresses error UX; (b) the first operator wanting a default glob discovers no config exists, and config-in-v0.2 ships under feature-request pressure with shape decided wrong; (c) first `mdatron check` collision reveals namespace discipline wasn't held. None are crises alone. All compound. Pay now.

## Routing

- **Raise to SO** for spec amendment: DESIGN-MDATRON.md names (a) thiserror-at-core / anyhow-at-CLI discipline; (b) `.mdatron/config.yaml` discovery; (c) error-catalog versioning; (d) Finding JSON as versioned contract.
- **Phase 1c -> Phase 1a** for `mdatron init`: `.mdatron/config.yaml` scaffold, anyhow at CLI boundary, `--format=json` on verify.
- **Phase 1c -> Phase 1b** for DESIGN.md § Verification architecture: name rustc-shape Finding as diagnostic seam + typed VerifyError -> anyhow chain as orchestration seam.
- **NOT in v0.1**: subcommand-namespace grouping. Stay flat; don't burn collision-prone verbs.

---

## Cross-references

- `mdatron/mdatron-cli/src/main.rs:71-77` (hand-rolled MDATRON-E0070; move to core)
- `mdatron/mdatron-cli/src/main.rs:90-96` (typed-VerifyError-printed-as-string; anyhow target)
- `mdatron/mdatron-cli/src/main.rs:141` (hand-rolled MDATRON-E0080; move to core)
- `mdatron/mdatron-core/src/error.rs:12-25` (library-correct thiserror)
- `mdatron/mdatron-core/src/diagnostic.rs:56-99` (Finding + format_tty; public contract)
- `mdatron/mdatron-core/src/verify.rs:38-57` (VerifyConfig; no top-level config today)
- `mdatron/mdatron-core/src/verify.rs:62-95` (VerifyError; library-correct typed variants)
- `mdatron/mdatron-core/src/dsl/expr.rs:177-203` (EvalError; same pattern)
- `mdatron/mdatron-core/src/dsl/index.rs:87-111` (IndexError; carries MDATRON-E0011)
- `mdatron/Cargo.toml:13-20` (workspace deps; no anyhow)
- `crosslink/crosslink/src/main.rs:73` (anyhow + Context at binary boundary)
- `crosslink/crosslink/src/commands/init/mod.rs:586,601,629` (.with_context patterns)
- `crosslink/CLAUDE.md` (30+ verb dispatch; what NOT to grow into prematurely)
