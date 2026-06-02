---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-06-02
phase: phase-3
scope: >-
  mdatron milestone — top-level verify pipeline. Subject = mdatron-core/src/verify.rs (~750
  LoC incl. tests) at commit e0c7ffb. Surface — pub fn verify(VerifyConfig) -> Result<Vec<Finding>,
  VerifyError>; types VerifyConfig + VerifyError + RuleContext. Walks project, loads schemas
  + patterns, builds IndexRegistry, dispatches by frontmatter schema_class, runs Layer 1
  (JSON Schema) + Layer 2 (DSL) per markdown file, emits rustc-shaped findings.
lens: >-
  SA primary (architecture, purity boundaries, surface design, dispatch coherence). SE
  supporting (implementation defects, error-path coverage, idiomatic Rust). QE supporting
  (test falsifiability against the 11 in-module tests; mutation-survivor enumeration).
  Sanity Check supporting baseline (coinage drift; scope drift vs. committed work).
  Weighted Consistency (5) + Maintainability (4) + Edge cases (3) + Usability (2) +
  Attacker (1).
source: director-raised
session_note: >-
  Cold-session cluster-batched per Phase 3 primer canonical shape. Composition = primary
  Solution Architect with supporting Software Engineer + Quality Engineer + Sanity Check
  (always-on baseline). No prior session memory; subject loaded fresh. Memory-isolation
  mode = worktree-no-memory (no operator-feedback poisoning). Findings ground in
  file:line citations + falsifiability checks per primer discipline.
model: claude-opus-4-7
execution_method: >-
  cold-session cluster-batched (Architecture cluster — SA primary; SE + QE pulled in as
  supporting voices for the Layer-1/Layer-2 dispatch surface where the seam crosses
  domains; Sanity Check always-on baseline).
sycophancy_compensation: >-
  Code authored by Claude (the reviewer-identity overlap). Bias compensation — every
  finding grounded in a verify.rs file:line citation + an explicit falsifiability check
  ("what would have to change in verify.rs for this finding to no longer apply?"). No
  finding rests on aesthetic preference. Where DESIGN-MDATRON.md asserts a contract that
  verify.rs violates, the spec cite is the load-bearing evidence.
filename_note: >-
  Filed under solution-architect domain slug per canonical convention. SA is the primary
  lens for the dispatch-architecture + Layer-1/Layer-2 seam concerns that dominate the
  finding set; SE-flavored implementation defects + QE-flavored test-falsifiability gaps
  carry their domain label in the per-finding heading.
supplements_loaded: []
---

# Solution Architect Review 1 — 2026-06-02

**Phase 3 cycle round:** 1 (opening round; mdatron verify-pipeline milestone)

---

## Pre-phase composition declaration

```yaml
phase: phase-3
composed_domains: [solution-architect, software-engineer, quality-engineer, sanity-check]
composition_mode: cluster-batched-cold-session
memory_isolation: worktree-no-memory
operator_confirmation: confirmed
cluster_shape: 4-cluster-default (Architecture cluster primary)
declared_at: 2026-06-02T00:00Z
```

---

## Scope under review

Primary file: `mdatron-core/src/verify.rs` (~750 LoC incl. 11 in-module tests).
Supporting context read: `lib.rs`, `diagnostic.rs`, `frontmatter.rs`, `schema.rs`,
`dsl/{mod, types, parser, index, expr}.rs`, and `DESIGN-MDATRON.md` § Layer 1 + Layer 2
(lines 110-143, 506-510).

---

## Findings

### F1 — Diagnostic-code swap: verify.rs emits MDATRON-E0001 for schema-violation; DESIGN binds E0001 to parse-failure (SA — spec-implementation consistency) — Open

**Evidence:**
- `DESIGN-MDATRON.md:116`: "Malformed YAML frontmatter → `MDATRON-E0001: frontmatter-parse-failed`".
- `DESIGN-MDATRON.md:117`: "Frontmatter `schema_class:` references unknown class → `MDATRON-E0002: schema-class-unknown`".
- `verify.rs:231`: parse-failure emits `code: "MDATRON-E0002"`, `summary: "frontmatter-parse-failed"`.
- `verify.rs:264`: schema-violation emits `code: "MDATRON-E0001"`, `summary: "frontmatter-schema-violation"`.

The implementation has swapped E0001 and E0002 relative to spec. Worse — `MDATRON-E0002` in DESIGN is `schema-class-unknown` (a wholly different defect from `frontmatter-parse-failed`), so the code reused E0002 for an unrelated condition. Adopters running `mdatron explain MDATRON-E0001` against a third-party explain catalog will receive the parse-failure explanation while staring at a schema-violation finding.

**Falsifiability:** Finding stops applying iff verify.rs:231 emits `MDATRON-E0001` for parse failure AND verify.rs:264 emits something other than `E0001`/`E0002` for schema violation (DESIGN's E0001-E0009 band is for frontmatter parsing; schema-violation belongs in the E0040-E0049 schema-load-failure band per `DESIGN-MDATRON.md:510`).

**Routing:** Phase 4 → SA + SE (raise-to-SO if DESIGN's code allocation is the desired truth). **Classification: Accepted.**

---

### F2 — `schema_class:` referencing an unknown class silently no-ops; DESIGN mandates MDATRON-E0002 (SA + SE — dispatch-model coherence) — Open

**Evidence:**
- `DESIGN-MDATRON.md:117`: unknown `schema_class:` → `MDATRON-E0002: schema-class-unknown`.
- `verify.rs:260-278`: `if let Some(schema_class) = &schema_class_opt { if let Some(schema) = schemas.get(schema_class) { ... } }` — no `else` arm for the unknown-class branch. Layer 2 proceeds against `schema_class_opt` for context-matching, but Layer 1 silently skips with zero diagnostic.

**Architectural impact:** The dispatch model declared in DESIGN (schema_class as the load-bearing dispatch key) is silently softened to "best-effort lookup." A typo in any document's `schema_class:` field passes Layer 1 with no signal — the document goes effectively unvalidated. This is the maintainability failure mode for cross-milestone reasoning: a downstream operator cannot derive from the validator output whether Layer 1 ran or not.

**Falsifiability:** Stops applying iff verify.rs emits an E0002 finding when `schemas.get(schema_class)` returns `None`.

**Routing:** Phase 4 → SA + SE. **Classification: Accepted.**

---

### F3 — `MDATRON-E0001` carries column 0 + "(instance_path)" inline in message; loses JSONPath structure (SA — diagnostic-surface design) — Open

**Evidence:**
- `verify.rs:267`: `message: format!("{} ({})", ve.message, ve.instance_path)`. The instance_path (the JSONPath into the value tree, e.g., `/relevant_domains/0`) is concatenated into the prose message.
- `Finding` struct (`diagnostic.rs:58-66`) has no dedicated field for instance_path; `Location` carries only `file`/`line`/`column`.
- `DESIGN-MDATRON.md` § Layer 1 promises pinpoint location ("the rustc-shaped diagnostic surface that distinguishes which layer caught each finding"). Burying the JSONPath in the message string makes downstream consumers (SARIF, LSP) re-parse a free-form message to recover structure.

**Falsifiability:** Stops applying iff `Finding` grows a structured `instance_path: Option<String>` field (or `Location` grows a JSONPath component) AND verify.rs populates it.

**Routing:** Phase 4 → SA (Finding shape design crosses milestone — touches diagnostic.rs surface). **Classification: Accepted.**

---

### F4 — Let-binding evaluation order is BTreeMap-alphabetical, not declaration-order (SE — dependency-ordering correctness) — Open

**Evidence:**
- `verify.rs:322-337`: iterates `rule.let_bindings` (a `BTreeMap<String, String>` per `dsl/types.rs:47-48`) and stores results in `ctx.bindings`. BTreeMap iterates by key.
- `verify.rs:323` comment acknowledges: "BTreeMap iterates by key — not strictly the declared order, but stable; for v0.1.x this is acceptable."
- `dsl/parser.rs:62-64` example: `let: { expected: key(...), missing: difference($expected.required, ...) }`. The `missing` binding depends on `$expected`. Alphabetically `expected` < `missing`, so it works **by accident.**

**Mutation-survivor risk (QE lens):** Swap any author's let-bindings such that the dependency direction reverses alphabetically (e.g., `actual: key(...)` then `b: count($actual)`) — the rule will fail with `EvalError` ("undefined variable") that the author cannot easily trace to "your let-bindings are evaluated in alphabetical order." None of the 11 in-module tests exercises a let-binding-chain where alphabetical and declared order disagree.

**Falsifiability:** Stops applying iff either (a) `let_bindings` becomes an ordered map (Vec<(String, String)> or IndexMap), OR (b) the evaluator runs a topological sort with cycle detection.

**Routing:** Phase 4 → SE + SA (data-type choice in `dsl/types.rs:48`). **Classification: Accepted.**

---

### F5 — `$project` is hardcoded to `Value::Null`; rules referencing `$project` silently fail (SA — purity-boundary / dispatch completeness) — Open

**Evidence:**
- `verify.rs:285`: `let project_value = Value::Null;`
- `verify.rs:287-293`: `RuleContext { ..., project_value: &project_value, ... }`.
- `dsl/expr.rs:87` declares `Var(VarRef)` with `$self`, `$file`, `$project` as canonical scopes. The DSL evaluator is built to receive a project-level value; verify.rs never provides one.

**Architectural-seam impact:** `$project` is a load-bearing scope in the DSL surface; verify.rs silently dispatches `Null` into it. Any pattern author writing `$project.repo_root` or `$project.config.foo` gets `Null` access errors with no operator-facing explanation that `$project` is simply unwired. This is the "purity boundary that holds in spec but the implementation snuck in I/O (or in this case, snuck in a Null)" failure mode named in the SA primer.

**Falsifiability:** Stops applying iff verify.rs constructs a non-Null `project_value` from project-root metadata (e.g., `{ root: PathBuf, config: Value }`) AND a test asserts a `$project`-referencing rule reaches non-Null binding.

**Routing:** Phase 4 → SA. **Classification: Accepted.**

---

### F6 — `interpolate_message` corrupts non-ASCII bytes in literal message text (SE — Unicode edge case) — Open

**Evidence:**
- `verify.rs:437`: `out.push(bytes[i] as char);` — casts a single `u8` to `char`. Multi-byte UTF-8 sequences (anything U+0080 and above) get split into separate `char` insertions of values 0x80-0xBF, which `String::push` writes as the corresponding code points — corrupting the text.
- Any rule message containing a non-ASCII character outside `{{...}}` markers (e.g., `"phase '–' must equal expected"`, an en-dash) emits mojibake.

**QE lens:** None of the 3 `interpolate_message_*` tests (`verify.rs:707-743`) supplies non-ASCII input. Mutation survivor — a mutation that replaces the body with a no-op for non-ASCII bytes would still pass all 3 tests.

**Falsifiability:** Stops applying iff `interpolate_message` walks `template.chars()` (or uses byte-index-aware slicing through `template[i..]` and char-boundary checks) and a test asserts a U+00A0+ literal survives interpolation.

**Routing:** Phase 4 → SE + QE (test-suite gap). **Classification: Accepted.**

---

### F7 — `load_schemas` + `load_patterns` silently return empty when dir absent; spec wants distinct error (SE + SA — error-path coverage) — Open

**Evidence:**
- `verify.rs:141-143`: `if !dir.is_dir() { return Ok(out); }` (schemas).
- `verify.rs:184-186`: same shape (patterns).
- `DESIGN-MDATRON.md:592`: "mdatron run with no `.mdatron/config.yaml` → `MDATRON-E0060: no-config-file` (project not initialized); exit code 2."

The pipeline returns success with empty schemas/patterns when `.mdatron/schemas/` or `.mdatron/patterns/` is missing, indistinguishable from "directory present, no files." DESIGN signals a project-not-initialized failure should be distinguishable from a clean run.

**Falsifiability:** Stops applying iff (a) absence of `.mdatron/` is upgraded to a `VerifyError::ProjectNotInitialized` (and the CLI maps it to E0060), OR (b) DESIGN-MDATRON.md is amended to declare absent-dirs is silent-skip semantics.

**Routing:** Phase 4 → Raise to SO (which path is canonical) + SE. **Classification: Accepted.**

---

### F8 — 11 in-module tests under-falsify the pipeline contract (QE — falsifiability gap) — Open

**Evidence (per-test falsifiability audit):**
- `clean_project_returns_zero_findings` (`verify.rs:508`): would pass against `fn verify(_) { Ok(vec![]) }`. Liveness only.
- `passing_rule_emits_no_finding` (`verify.rs:570`): same shape — vacuous-pass survivor.
- `file_without_frontmatter_is_skipped` (`verify.rs:641`): vacuous-pass survivor.
- `schema_violation_emits_mdatron_e0001` (`verify.rs:522`): falsifying. Asserts code + message content.
- `rule_violation_emits_rule_code_with_interpolated_message` (`verify.rs:538`): falsifying.
- `cross_file_rule_with_key_lookup_runs_end_to_end` (`verify.rs:596`): falsifying. Strongest test in the suite.
- `file_without_schema_class_skips_layer_one_runs_layer_two` (`verify.rs:652`): falsifying via the `assert: false` rule.
- `findings_are_sorted_by_file_then_code` (`verify.rs:676`): falsifying.
- 3× `interpolate_message_*` (`verify.rs:707-743`): falsifying for ASCII; per F6, miss the Unicode mutant.

**Gap enumeration (zero tests cover):**
- Schema-class on a frontmatter that references an unknown class (F2 surface — no test).
- Malformed-YAML frontmatter → does emission of MDATRON-E0002 (currently the code, swapped per F1) happen? No test.
- Combined ContextSelector (`{ schema_class, path }`) — no test exercises `verify()` end-to-end against the Combined form (parser test exists in `dsl/parser.rs:90` but verify.rs's `context_matches` Combined branch has no integration coverage).
- Let-binding chain across alphabetical-order boundary (F4 surface).
- `$project` reference in a rule (F5 surface).
- Non-ASCII in message template (F6 surface).
- Pattern-file-loading failure path (corrupt YAML in `.mdatron/patterns/`).

**Falsifiability:** Stops applying iff at least 4 of the 7 enumerated gap-coverage tests land AND each is verified to fail against the corresponding stub implementation.

**Routing:** Phase 4 → QE (test additions) + SE (likely surfaces real defects per F2, F4, F5, F6). **Classification: Accepted.**

---

### F9 — `Value::Object` format_value emits Debug representation in user-visible interpolated message (SE — usability) — Open

**Evidence:**
- `verify.rs:453`: `Value::Object(_) => format!("{v:?}")`. A rule whose `{{$self.metadata}}` interpolates an object emits the internal Debug form (`Object({...})`) into the finding message.

**Falsifiability:** Stops applying iff `format_value` either renders objects as compact YAML / JSON OR refuses to interpolate object-typed values with a typed error.

**Routing:** Phase 4 → SE. **Classification: Accepted.**

---

### F10 — `canonicalize()` on non-existent `project_root` produces opaque IO error (SE — error-path UX) — Deferred

**Evidence:**
- `verify.rs:104-107`: `config.project_root.canonicalize().map_err(|e| VerifyError::Io { ... })`. If the user passes a typo'd path, the error is "io error at '/typo/path': No such file or directory (os error 2)" — accurate but misses the opportunity for a `VerifyError::ProjectRootNotFound` variant with help text.

**Routing:** Phase 4 → SE (low-priority polish). **Classification: Deferred.**

---

### F11 — `glob::glob` invoked on `to_string_lossy()` silently drops non-UTF-8 path components (SE — edge case) — Deferred

**Evidence:**
- `verify.rs:118-119`: `let absolute = project_root.join(glob_pattern); let paths = glob::glob(&absolute.to_string_lossy())...`. On macOS/Linux paths legally contain non-UTF-8 bytes; `to_string_lossy` substitutes U+FFFD, so the glob pattern silently mismatches paths under such directories.

**Routing:** Phase 4 → SE. Real but low-incidence; deferred pending operator confirmation that mdatron's adopters do not author markdown under non-UTF-8 paths. **Classification: Deferred.**

---

## Round-close summary

**11 findings raised. Zero Hallucinated. Zero Dismissed.** Phase 3 round-trigger fires; round MUST continue.

| F  | Lens     | Class    | Routing      | Severity |
|----|----------|----------|--------------|----------|
| F1 | SA       | Accepted | Phase 4 → SA+SE+SO | **High** (spec-contract violation) |
| F2 | SA+SE    | Accepted | Phase 4 → SA+SE    | **High** (dispatch silently softened) |
| F3 | SA       | Accepted | Phase 4 → SA       | Medium (downstream-consumer impact) |
| F4 | SE       | Accepted | Phase 4 → SE+SA    | Medium (correctness by accident) |
| F5 | SA       | Accepted | Phase 4 → SA       | Medium (surface unwired) |
| F6 | SE       | Accepted | Phase 4 → SE+QE    | Medium (Unicode corruption) |
| F7 | SE+SA    | Accepted | Phase 4 → SO+SE    | Medium |
| F8 | QE       | Accepted | Phase 4 → QE+SE    | **High** (test-suite vacuity) |
| F9 | SE       | Accepted | Phase 4 → SE       | Low |
| F10| SE       | Deferred | Phase 4 → SE       | Low |
| F11| SE       | Deferred | Phase 4 → SE       | Low |

**Most-severe:** F1 (spec-implementation code swap on the load-bearing MDATRON-E0001 diagnostic) — operator-facing breakage on the canonical first-class error code.

**Cross-finding coherence (sanity-check):** F1 + F2 + F7 form a coherent meta-pattern — the error-code surface declared in DESIGN-MDATRON.md is partially implemented in verify.rs (codes swapped; one branch missing; one absent-state silent). The work is one milestone away from spec-conformant; the dispatch architecture is sound but the diagnostic-emission code-paths drift from DESIGN's catalog. F8 (test vacuity) explains why F1/F2 weren't caught by the suite — the tests assert what the implementation does, not what DESIGN asserts.

**MVR signal:** NOT YET. 9 Accepted + 2 Deferred; zero Hallucinated. Next round requires F1/F2/F7 dispositions (likely Phase 4 → Phase 1a code-catalog alignment) before re-sweep.

---

## Cross-references

- `mdatron-core/src/verify.rs` (subject, ~750 LoC)
- `mdatron-core/src/diagnostic.rs` (Finding/Severity/Location surface; F3 lands here)
- `mdatron-core/src/dsl/types.rs:47-48` (let_bindings BTreeMap; F4 lands here)
- `mdatron-core/src/dsl/expr.rs:87` (VarRef scopes incl. `$project`; F5 surface)
- `DESIGN-MDATRON.md:116-117` (E0001/E0002 code allocation; F1 + F2 binding evidence)
- `DESIGN-MDATRON.md:506-510` (error-code band allocation; F1 routing target)
- `DESIGN-MDATRON.md:592` (no-config-file → E0060; F7 binding evidence)
