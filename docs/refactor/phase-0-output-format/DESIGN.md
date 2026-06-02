# Phase 0 — Output-Format Contract DESIGN

**Status:** Phase 1a design draft (skill-interactive composition).
**Issue:** crosslink #11.
**Phase:** 1a — Behavioral Specification.

## Pre-phase composition declarations

```yaml
phase: phase-1a
composed_domains: [solution-owner, solution-architect, software-engineer, quality-engineer, platform-engineer, security, sanity-check]
composition_mode: skill-interactive
supplements_loaded: [json, rust]
operator_confirmation: confirmed
declared_at: 2026-06-02T19:16:40Z
context: output-format contract for mdatron verify --json cross-process IPC

phase: phase-1b
composed_domains: [solution-owner, solution-architect, quality-engineer, sanity-check]
composition_mode: skill-interactive
supplements_loaded: [json, rust]
operator_confirmation: confirmed
declared_at: 2026-06-02T19:35:00Z

phase: phase-1c
composed_domains: [solution-architect, solution-owner, documentation-reviewer, sanity-check]
composition_mode: skill-interactive
supplements_loaded: [json, rust]
operator_confirmation: confirmed
declared_at: 2026-06-02T19:55:00Z

phase: phase-2a
composed_domains: [quality-engineer, sanity-check]
composition_mode: skill-test-author
supplements_loaded: [json, rust]
operator_confirmation: confirmed
declared_at: 2026-06-02T20:20:00Z

phase: phase-2b
composed_domains: [software-engineer, quality-engineer, technical-writer, documentation-reviewer, platform-engineer, sanity-check]
composition_mode: skill-implementer
supplements_loaded: [json, rust]
operator_confirmation: confirmed
declared_at: 2026-06-02T20:30:00Z

phase: phase-2c
composed_domains: [software-engineer, solution-architect, documentation-reviewer, sanity-check]
composition_mode: skill-interactive
supplements_loaded: [json, rust]
operator_confirmation: confirmed
declared_at: 2026-06-02T20:50:00Z
exit_status: complete
rationale: |
  Initially declared skipped-no-refactor-surface. Operator finding 2026-06-02
  surfaced a real vocabulary polish: "BC-N" anchor codes had leaked from the
  DESIGN doc (where they're defined) into operator-facing test assertion
  text where they read as opaque jargon. DR-F4 vocabulary discipline applied
  in mdatron-cli/tests/output_format.rs across ~12 assertion strings. Tests
  remain green (10/10). Phase 2c is now genuinely complete (the polish
  surface existed; was just initially under-noticed).
```

## Project intent

**Phase 5 strategy:** property-based testing on output round-trip invariants + fuzz testing on the consumer-side output parser. Mutation testing not applicable to a output-format contract whose enforcement is structural (strict JSON Schema rejection at construction); the mutation surface lives in the implementation (Phase 5 of mdatron-implementation, not of this contract spec).

## Scope

This document specifies the **observable output-format contract** for cross-process
communication between `vsdd` (consumer) and `mdatron` (provider) under the
binary-first refactor (per `docs/refactor/binary-first-plan.md` Phase 0).

The contract covers:

- Output stream shape: `mdatron verify --json` output object on stdout
- Exit code semantics for the three operational states
- Global flag vocabulary (`--quiet` / `--json` / `--log-level` / `--log-format` / `--dry-run`)
- Error-code namespace separation between `MDATRON-Exxxx` and `VSDD-Exxxx`
- Output-version compatibility discipline

Out of scope for Phase 1a: implementation details (Phase 2b), test surface
(Phase 2a), prose/documentation (Phase 2c).

## Behavioral contracts

### Output schema and version field

`mdatron verify --json` writes exactly one JSON object to stdout per
invocation. The object MUST carry a top-level `mdatron_output_version` string
field.

**Observable assertions:**

- Field present on every successful invocation.
- Value is a string matching `^\d+\.\d+\.\d+$` (semver per SO disposition 2026-06-02).
- Version bumps follow semver-for-output-formats: additive optional fields → minor
  bump (e.g., 1.0.0 → 1.1.0); required field added, field-shape changed, or
  field semantics changed → major bump (1.x → 2.0.0); fix-without-shape-change
  → patch (1.0.0 → 1.0.1). Consumer compatibility matches on major; minor +
  patch are transparent to a consumer pinned at major M.

**Falsification path:** consumer can construct an output object missing
`mdatron_output_version` and assert mdatron's parser rejects it; or construct an
output object with `mdatron_output_version: "999"` and assert vsdd produces an
`"output version unsupported"` error rather than parsing.

### Output top-level shape

The output object's top-level object is exactly:

```json
{
  "mdatron_output_version": "<integer-string>",
  "mdatron_version": "<semver-string>",
  "pipeline_status": "ok" | "failed",
  "summary": {
    "error_count": <non-negative integer>,
    "warning_count": <non-negative integer>,
    "lint_count": <non-negative integer>,
    "files_checked": <non-negative integer>
  },
  "findings": [<Finding>, ...]
}
```

**Observable assertions:**

- All listed fields present on every invocation; no additional top-level
  fields at output-version 1 (`additionalProperties: false`).
- `pipeline_status: "ok"` ↔ findings may be empty or non-empty; pipeline ran.
- `pipeline_status: "failed"` ↔ pipeline did not run to completion;
  `findings` MAY be partial or empty; `summary` counts MAY be zero.

**Edge cases:**

- Empty project (zero `.md` files): `pipeline_status: "ok"`,
  `summary.files_checked == 0`, `findings == []`.
- Missing `.mdatron/schemas/` directory: `pipeline_status: "failed"`,
  findings contains a single MDATRON-Exxxx entry naming the missing directory.
- Single malformed pattern file: `pipeline_status: "failed"` (pattern files
  are load-bearing; malformed file blocks the run).
- A single malformed markdown frontmatter file: `pipeline_status: "ok"`,
  finding emitted for that file's `MDATRON-E0002` (now reserved per Phase 1).

**Falsification path:** for any output state, consumer asserts the
documented invariant; a regression that emits an output object with a missing
field, an extra field, or violates the `pipeline_status` ↔ `findings`
relationship fires the test.

### Finding shape

Each `Finding` in the `findings` array is exactly:

```json
{
  "code": "MDATRON-Exxxx",
  "severity": "error" | "warning" | "lint",
  "summary": "<short label>",
  "message": "<full message text>",
  "help": "<optional remediation hint>",
  "location": {
    "file": "<path relative to project root>",
    "line": <positive integer>,
    "column": <non-negative integer>
  },
  "explain_ref": "<optional code reference for mdatron explain>"
}
```

**Observable assertions:**

- `code` matches `^MDATRON-[EWL][0-9]{4}$` (E=error, W=warning, L=lint).
- `severity` corresponds to the code letter prefix (E→error, W→warning,
  L→lint). Mismatch is itself a contract violation.
- `summary` is a short label (≤ 80 characters); `message` may be longer.
- `location.file` is forward-slash separated regardless of host OS.
- `location.line` ≥ 1; `location.column` ≥ 0.

**Edge cases:**

- Finding with no precise location (whole-file finding): `location.line == 1`,
  `location.column == 0`.
- Finding with a multi-line span: `location` represents the START of the span;
  end-of-span is not part of the v1 output-format contract (Phase 6 candidate).
- Empty `message`: rejected at construction (vacuous diagnostic is itself a
  contract violation).

**Falsification path:** consumer can construct a Finding with mismatched
code prefix and severity (e.g., `code: "MDATRON-E0001"`, `severity: "warning"`)
and assert validation rejects it.

### Exit code semantics

`mdatron verify` exits per the three-state model:

| Exit code | Meaning | Output object behavior |
|---|---|---|
| `0` | Pipeline ran to completion; no error-severity findings (warnings or lints may exist) | Output present, `pipeline_status: "ok"`, `summary.error_count == 0` |
| `1` | Pipeline ran to completion; at least one error-severity finding | Output present, `pipeline_status: "ok"`, `summary.error_count >= 1` |
| `2` | Pipeline did not run to completion (configuration error, IO failure, malformed pattern file, etc.) | Output MAY be present with `pipeline_status: "failed"`; MAY be absent if failure occurred before output construction (e.g., CLI parse error) |
| `101+` | Unanticipated failure (Rust panic = `101`; SIGTERM-style = signal number plus `128`) | Output absent |

The scheme matches the **rustc / clippy / cargo-check convention** per SO
disposition 2026-06-02 (Raise-to-SO #3). `3` is not reserved; vsdd's
"couldn't spawn mdatron" state is vsdd-internal and does not appear in
mdatron's contract.

**Observable assertions:**

- Successful clean run: exit `0`, `summary.error_count == 0`.
- Pipeline run with findings: exit `1`, `summary.error_count >= 1`.
- Pipeline failure: exit `2` or higher.
- Exit code is the consumer's primary signal; output object is secondary
  (consumer MAY infer `pipeline_status` from exit code without parsing).

**Edge cases:**

- `--quiet` flag does not change exit code.
- `--json` flag does not change exit code.
- Findings with mixed severity (one error, three warnings): exit `1` (the
  highest severity is error).
- Only warnings, no errors: exit `0` (warnings do not fail the pipeline).
- Empty project, no findings: exit `0`.

**Falsification path:** consumer asserts exit code matches the documented
state for each scenario; a regression that fails the pipeline with warnings
only (exit `1` for warning-only output) fires the test.

### Output stream contract

| Mode | stdout | stderr | Use case |
|---|---|---|---|
| Default (no `--json`) | silent | rustc-shaped diagnostics + final summary line | Interactive operator |
| `--json` | one JSON output object (compact, single line) | rustc-shaped diagnostics + final summary line | Machine consumer + operator visibility |
| `--json --quiet` | one JSON output object | silent | Pure machine consumer (CI) |
| `--quiet` (no `--json`) | silent | silent except for non-zero exit cause | Hooks/scripts where exit code is the only signal needed |

**Observable assertions:**

- The JSON output object is emitted as a single compact line on stdout when
  `--json` is set; readers can parse with line-oriented tools.
- stdout never contains anything other than the JSON output object when `--json`
  is set (no diagnostic text on stdout under any flag combination).
- stderr is for human-readable text; it is never machine-parsed by consumers.

**Edge cases:**

- Empty findings + `--json`: the output object still emits (one line) with
  `findings: []`.
- Pipeline failure + `--json`: if the output object is constructed (failure detected
  after pipeline init), it emits with `pipeline_status: "failed"`. If the
  failure prevents output construction (CLI parse error), no output object.

**Falsification path:** consumer pipes stdout to `jq` and asserts parse
success on every invocation where exit ≤ 2.

### Global flag vocabulary

`mdatron` accepts the following global flags (applicable to all subcommands
unless noted):

| Flag | Effect |
|---|---|
| `--quiet` / `-q` | Suppress stderr human-readable output; exit code is the only signal |
| `--json` | Emit JSON output object on stdout (verify only); other subcommands MAY emit JSON if explicitly opted-in |
| `--log-level <level>` | Set structured-log verbosity: `trace`, `debug`, `info`, `warn`, `error`. Default: `warn`. Logs are separate from diagnostics |
| `--log-format <format>` | `text` or `json`. Default: `text`. Controls structured-log emission shape, not diagnostics |
| `--dry-run` | Subcommand-specific (init reads; no-op for verify) |

**Observable assertions:**

- Flag combinations behave additively (no flag overrides another).
- Unknown flags fail with non-zero exit and stderr error message; consumer can
  assert this for forward-compatibility (when adding flags, consumer should
  pin to known mdatron version).
- `--quiet` and `--json` are orthogonal.

**Falsification path:** consumer invokes `mdatron verify --unrecognized-flag`
and asserts non-zero exit + descriptive error on stderr.

### Error-code namespace separation

`MDATRON-Exxxx` and `VSDD-Exxxx` namespaces are **strictly separate** across
the cross-process boundary.

**Observable assertions:**

- mdatron NEVER emits a `VSDD-Exxxx` code in any output object or stderr text.
- vsdd, when wrapping or summarizing mdatron's output object, MUST preserve
  `MDATRON-Exxxx` codes verbatim. vsdd MUST NOT re-tag, alias, or wrap them
  under a different `VSDD-Exxxx` code.
- vsdd's own findings (against VSDD-content layered on top of mdatron's
  substrate) carry `VSDD-Exxxx` codes only.
- The reverse is also true: vsdd never emits `MDATRON-Exxxx` codes from its
  own code paths; if it surfaces mdatron findings, they keep their codes.

**Falsification path:** static check (lint) over both repos asserts code-prefix
constants are scoped to their owning binary; a regression that introduces a
`VSDD-Exxxx` literal in mdatron's source (or vice versa) fires the lint.

### Output-version compatibility

Compatibility is enforced via the `mdatron_output_version` field per the output-version contract:

**Consumer-side (vsdd) discipline:**

- vsdd pins the highest mdatron output version it knows how to parse
  (compile-time constant).
- On output object parse, vsdd reads `mdatron_output_version` first:
  - If known and supported: parse normally.
  - If newer than known: error `VSDD-Exxxx output-version-too-new` instructing
    operator to upgrade vsdd.
  - If older than known but still in supported window: parse with the older
    schema; emit a warning that mdatron is behind.
  - If older than the supported window: error
    `VSDD-Exxxx output-version-too-old` instructing operator to upgrade mdatron.

**Provider-side (mdatron) discipline:**

- mdatron emits the highest output version it supports.
- mdatron MAY support emitting older output versions via a `--output-version=N`
  override (deferred to v0.2; not in v0.1.0 scope).

**Versioning rule:**

- Add an optional field: no version bump (consumers ignore unknown fields).
- Remove a field, add a required field, change a field's type or semantics:
  bump `mdatron_output_version`.
- SemVer-major bump on mdatron may carry a output version bump but isn't
  required to (a major rewrite of internals can keep output version stable).

**Falsification path:** consumer can construct output objects at boundary versions
and assert vsdd's handling matches the documented matrix.

## Invariants (audit checks across contracts)

| Invariant | Check |
|---|---|
| Every emitted code is in mdatron's reserved-code table | Static lint over mdatron source asserts every code literal matches the table |
| Exit code corresponds to `pipeline_status` and `summary.error_count` | Property test on Finding emission: `(pipeline_status, error_count) → exit_code` is a pure function |
| Output's `summary` counts match `findings` array | Property test: derive counts from array; assert equality |
| `findings` array entries' `code` prefixes match `severity` field | Per-Finding validation |
| `mdatron_output_version` is monotonically non-decreasing across releases | CHANGELOG audit |

## Verification architecture

### Pure functions

Functions on this contract that are deterministic + side-effect-free are
candidates for Phase 5 property-based testing:

| Function | Inputs | Output | Purity grounds |
|---|---|---|---|
| `build_output(findings, files_checked, mdatron_version)` | array of Finding + non-negative integer + semver string | output object object | Pure construction from inputs; no I/O |
| `compute_summary(findings)` | array of Finding | summary object with the four counts | Pure reduction; deterministic counts |
| `derive_exit_code(pipeline_status, error_count)` | enum + non-negative integer | exit code integer | Pure function of two inputs per the exit-code table |
| `parse_output(json_bytes)` | bytes | Result<Output, ParseError> | Pure given strict JSON parser (no environment lookup) |
| `validate_finding(finding)` | Finding | Result<(), ValidationError> | Pure; checks code-prefix-vs-severity alignment + location bounds |
| `code_in_namespace(code, expected_prefix)` | string + string | bool | Pure prefix match |
| `compare_output_versions(emitted, supported_window)` | integer + range | enum (Supported / Too-New / Too-Old / In-Window-Old) | Pure comparison |

These seven functions form the purity boundary for the output-format contract. Phase 5
property tests target this list.

### Automatable vs manual classification

All all 8 behavioral contracts are **automatable**. The
output-format contract is operator-invisible (no UI; no human-readable artifact
under inspection); there is no `manual-tests/output-format.md` checklist.

| BC | Test surface | Automation tool |
|---|---|---|
| Output schema & version | JSON Schema validation + Rust struct deserialization round-trip | `jsonschema` crate + `serde_json` |
| Output top-level shape | Integration test against `mdatron verify --json` output | subprocess spawn + JSON parse + struct assert |
| Finding shape | Per-Finding property test | `proptest`: generate Finding tuples; assert validate_finding succeeds for well-formed; fails for code-severity mismatch |
| Exit codes | Integration matrix: 6 fixture scenarios × asserted exit code | subprocess spawn + exit-code assertion |
| Stream contract | Integration test capturing stdout + stderr separately | subprocess capture |
| Global flags | Integration test per flag combination | clap + subprocess; assert unknown-flag rejection |
| Namespace separation | Static lint (compile-time) over both repos | custom proc-macro or grep-based lint in CI |
| Output-version compatibility | Property test on `compare_output_versions` + integration test on boundary output objects | `proptest` + fixtures with synthetic output versions |

### Phase 5 candidates

Per the **Phase 5 strategy** declared above:

**Property-based testing (the dominant Phase 5 surface here):**

| Property | Function under test | Falsifying generator |
|---|---|---|
| Output round-trip: `parse(serialize(output object)) == output object` | `build_output` + `parse_output` | `proptest` Arbitrary impl over Output |
| Summary correctness: `compute_summary(findings).error_count == findings.iter().filter(\|f\| f.severity == "error").count()` | `compute_summary` | Arbitrary Vec<Finding> |
| Exit-code monotone: `error_count == 0 → exit_code in {0}`; `error_count >= 1 → exit_code in {1}`; `pipeline_status == failed → exit_code in {2,…}` | `derive_exit_code` | Arbitrary `(pipeline_status, error_count)` tuples |
| Finding code-severity alignment: validate succeeds iff first letter of code matches severity letter | `validate_finding` | Arbitrary code + severity, biased toward mismatches |
| Output-version comparison transitivity: standard total-order properties | `compare_output_versions` | Arbitrary triples |

**Fuzz testing surface:**

| Target | Tool | Corpus seed |
|---|---|---|
| `parse_output(arbitrary_bytes)` | `cargo-fuzz` (libFuzzer) | One real-mdatron output object + 1KB of structured-but-invalid JSON |

Fuzz target asserts: no panic; either Ok(Output) or Err(ParseError); never UB.

**Out-of-scope for Phase 5:**

- Mutation testing on contract enforcement code: the contract is structural;
  mutation kills are captured by the property-test surface above.
- Symbolic execution / Proof Execution: the contract's invariants are
  small-scale and propable; formal verification is excessive for v0.1.0.

### Trust boundaries

Input data crossing process boundaries — these are the fuzz-test targets
and security-review surface:

| Boundary | Direction | Trust posture | Hardening |
|---|---|---|---|
| `mdatron verify --json` stdout (bytes) → vsdd parser | Untrusted (subprocess could be old version, attacker-controlled binary, etc.) | Strict JSON parsing + schema validation + version check before structural use | Fuzz target on `parse_output`; output version asserted first |
| CLI argument parsing | Untrusted (operator may pass malformed flags) | Clap derives + strict unknown-flag rejection | Standard clap discipline |
| Schema/pattern file content on disk → mdatron evaluator | Untrusted (operator-supplied) | Strict JSON Schema parsing; refuse-malformed | Existing mdatron discipline (out of output-format scope) |
| Environment variable resolution (RUST_LOG, MDATRON_*) | Semi-trusted (operator-set) | Bounded enum parsing; log-level enum is closed | Strict enum parse; default on invalid |

The cross-process boundary between vsdd → mdatron stdout is the **load-bearing
trust boundary** for this contract. All others are pre-existing surfaces from
prior phases.

## Falsification surface (Phase 1c will operationalize)

Each contract above contains a "falsification path" naming how it can be
broken. Phase 1c (acceptance criteria) settles which of these become
mandatory tests; Phase 2a writes them.

Concrete should-fire fixtures candidates:

- Output missing `mdatron_output_version` field
- Output with `additionalProperties` at top level
- Finding with mismatched code prefix and severity
- Pipeline-fail run that exits `0` (regression: exits `1` when error finding present, `2` when pipeline broke)
- `--json` invocation emitting non-JSON content on stdout
- Consumer parsing output object with an unsupported `mdatron_output_version`
- vsdd emitting a literal `MDATRON-` code prefix in its own diagnostic surface
- mdatron emitting a literal `VSDD-` code prefix anywhere

## Decomposition (Phase 1c)

The output-format contract decomposes into three milestones with sequential
dependencies. Each is independently buildable + verifiable.

### Milestone M1 — Output shape

**Closes:** Output schema & version, Output top-level shape, Finding shape.

**Acceptance criteria:**

- `mdatron verify --json` emits a single JSON object on stdout per
  invocation, structurally conformant to the top-level shape's schema
- Every emitted Finding conforms to the Finding shape (code-prefix-vs-severity
  alignment enforced)
- `mdatron_output_version` is present and parseable per the output-version contract
- JSON Schema for the output object exists at `<repo>/docs/refactor/phase-0-output-format/output.schema.json` (built in Phase 2b; referenced from this DESIGN)

**Phase 2a Red Gate seed:**
- Output-missing-version-field fixture
- Output-with-extra-top-level-field fixture
- Finding-with-mismatched-code-prefix-and-severity fixture

**Dependencies:** none (foundational).

**Exit Signal pointer:** `ExitSignaled{milestone: m1-output-shape}` emitted
on the closing commit.

### Milestone M2 — Process behavior

**Closes:** Exit codes, Stream contract, Global flag vocabulary.

**Acceptance criteria:**

- Exit-code matrix: clean run = 0, errors-exist = 1, pipeline-failed = 2;
  warning-only run = 0
- stdout under `--json` contains only the output object (no diagnostic text)
- stderr always carries the human-readable rustc-shaped output unless
  `--quiet`
- All 5 global flags (`--quiet` / `--json` / `--log-level` / `--log-format`
  / `--dry-run`) parse + apply per the global-flag table
- Unknown-flag rejection with descriptive stderr message

**Phase 2a Red Gate seed:**
- Warning-only-exits-1 regression fixture (must exit 0)
- Pipeline-failure-without-output object-in-stderr fixture
- Unknown-flag-silent-success fixture (must error)
- `--quiet --json` smoke fixture (asserts output object on stdout, silence on stderr)

**Dependencies:** M1 (process behavior is observed by inspecting the output object).

**Exit Signal pointer:** `ExitSignaled{milestone: m2-process-behavior}`.

### Milestone M3 — Contract discipline

**Closes:** Namespace separation, Output-version compatibility.

**Acceptance criteria:**

- Static lint over both repos asserts that `MDATRON-Exxxx` string literals
  appear only in mdatron's source; `VSDD-Exxxx` literals only in vsdd's
  source. Lint exit-code-fails the CI on violation.
- Output-version compatibility:
  - vsdd's compile-time constant declares the highest mdatron output version it
    supports
  - vsdd refuses to parse an output object with `mdatron_output_version` higher
    than that constant; emits `VSDD-Exxxx` output-version-too-new error
  - vsdd accepts an output object with `mdatron_output_version` equal to the
    constant
  - Older-than-window handling deferred to v0.2 (no compatibility window in
    v0.1.0; mdatron == vsdd's pinned version)

**Phase 2a Red Gate seed:**
- VSDD-source-emits-MDATRON-prefix fixture (lint fires)
- MDATRON-source-emits-VSDD-prefix fixture (lint fires)
- Output-with-output-version-999 fixture (vsdd refuses to parse)
- Output-with-pinned-output-version fixture (vsdd parses successfully)

**Dependencies:** M1 (output object must exist for the version check; namespace
lint is repo-static).

**Exit Signal pointer:** `ExitSignaled{milestone: m3-contract-discipline}`.

### Decomposition rationale

Three milestones rather than one or eight because:

- **One milestone** would bundle the namespace-separation lint surface (purely static) with
  the output-shape parser surface (dynamic JSON construction), making the Red Gate
  hard to falsify in aggregate (any-one-test-passing masks failures in
  others). Phase-1c primer flags this as the "milestone whose acceptance
  criteria don't cover the behaviors it claims to close" failure mode.
- **Eight milestones (one per BC)** would force artificial sequential
  dependencies (Output-shape contracts standing alone are meaningless without one another; exit codes
  without stream contract is unobservable). Phase-1c primer flags this as the
  "milestone that bundles too many behaviors / not enough cohesion" failure
  mode.
- **Three milestones** (output object shape / process behavior / contract
  discipline) correspond to three distinct verification mechanisms (schema
  validation / subprocess integration / static lint + boundary fixtures),
  each independently runnable in CI.

### manual-tests/layer-phase-0.md note

The output-format contract is operator-invisible; no manual-test checklist applies
to this layer. The Phase 1b verification architecture explicitly declared
zero manual-test surface. `manual-tests/layer-phase-0.md` would be a stub
file with "no manual tests apply at this layer — output-format contract is
machine-only" and no checkbox items.

Per the post-design-md-modification hook design (DESIGN-METHODOLOGY.md
§ Post-DESIGN.md auto-scaffolding), the hook would still emit the stub
file for symmetry; the `falsifiability_check` field carries the
no-manual-tests rationale.

## Open questions (Raise-to-SO)

1. **Should `mdatron_output_version` use semver or simple integer?**
   - Simple integer (proposed): easier reasoning; bumps less ambiguous.
   - Semver: more familiar; allows minor/patch distinctions.
   - SO disposition needed.

2. **Should `mdatron explain CODE` ship in v0.1.0?**
   - SO disposition 2026-06-02: implement explain for v0.1.0.
   - DR-F2 finding ("strip the dead `= explain:` line") is reversed — the
     line is retained because the surface it promises is built.
   - Implementation belongs to crosslink #13 (Phase 2 of binary-first
     plan), not this Phase 0 issue. The output object's `explain_ref`
     field now points to a real catalog rather than a stubbed surface.
   - Catalog scope: one paragraph of explanation prose per emitted code
     (MDATRON-E0001/E0002/E0070/E0080 at v0.1.0 baseline; grows with
     each emitted code thereafter).

3. **Reserved exit codes above 2:**
   - SO disposition 2026-06-02: match rustc/clippy convention: `{0, 1, 2}`
     with `101+` for unanticipated failures (panic). No `3` reserved.
   - vsdd's "couldn't spawn mdatron" state is vsdd-internal; does not
     appear in mdatron's contract.

## Cross-references

This DESIGN consumes from:
- `crosslink/src/commands/init/manifest.rs` — three-way classification model
- `docs/refactor/binary-first-plan.md` — 7-phase work plan
- Prior reviews: `review-log/2026-06-02-solution-architect-binary-first-refactor.md`,
  `review-log/2026-06-02-software-engineer-binary-first-refactor.md`,
  `review-log/2026-06-02-platform-engineer-binary-first-refactor.md`

This DESIGN produces for:
- Phase 1b — Verification architecture (lint surface, test surface, schema validation)
- Phase 1c — Acceptance criteria (per-contract must-pass tests)
- Phase 2a — Red Gate (failing tests against each falsification path)
- Phase 2b — Implementation in mdatron
- Phase 3 — Vsdd subprocess client (parsing the output object per the top-level + Finding shape)

## Phase 1a exit signal

Phase 1a closes when:

- ✅ DESIGN.md § Behavioral contracts is non-empty (the 8 behavioral contracts above)
- ✅ Every behavior is specific + testable + has named edge cases
- ⚠️ Per-axes-activated domains have surfaced their lens (no axes activated
  for this design — output format is operator-invisible)
- ⚠️ Cold reviewer (DR) iteration pending — Phase 3 will adversarially
  refine the contracts

Emit on Phase 1a closing commit (deferred to Phase 1b joint close per Phase 1a
primer's "Operators may author Phase 1a and Phase 1b in a single session"
allowance):

```yaml
event: PhaseExited
phase: phase-1a
exit_status: complete
layer: phase-0-output-format
declared_at: 2026-06-02T19:30:00Z
next_phase: phase-1b
```

## Phase 1c exit signal

Phase 1c closes when:

- ✅ DESIGN.md § Decomposition lists all milestones M1..M3 with acceptance criteria per milestone
- ✅ Each milestone's acceptance criteria are a non-empty subset of DESIGN.md § Behavioral contracts (M1 covers output shape; M2 covers process behavior; M3 covers contract discipline)
- ⚠️ DR's cold-reader pass — applied inline as DR lens during authorship;
  formal cold-session DR pass arrives at Phase 3
- ⚠️ SO sign-off on the spec-gate close — the three Raise-to-SO open
  questions above are the outstanding spec-gate items; SO disposition
  required before Phase 2a opens
- ⚠️ `manual-tests/layer-phase-0.md` stub authored — pending Phase 2a or
  Phase 2c; this DESIGN section names the rationale

Emit on Phase 1c closing commit:

```yaml
event: PhaseExited
phase: phase-1c
exit_status: complete
layer: phase-0-output-format
declared_at: 2026-06-02T20:15:00Z
next_phase: phase-2a
so_dispositions:
  - so-q1-output-version: semver
  - so-q2-explain-v010: ship in v0.1.0
  - so-q3-exit-codes: rustc-style {0,1,2}+101+
milestones_opened: [m1-output-shape, m2-process-behavior, m3-contract-discipline]
```

## Phase 1b exit signal

Phase 1b closes when:

- ✅ Pure functions listed (7 candidates)
- ✅ Automatable-vs-manual classification per behavior (8 BCs automatable; 0 manual)
- ✅ Phase 5 strategy committed (property-based testing + fuzz; see § Project intent)
- ✅ Trust boundaries named (4 listed; vsdd→mdatron stdout is the load-bearing one)
- ⚠️ SA + QE concurrence on the purity-boundary list — recorded inline as
  both lenses participated in skill-mode authorship; cold-session cross-check
  arrives at Phase 3

Emit on Phase 1b closing commit:

```yaml
event: PhaseExited
phase: phase-1b
exit_status: complete
layer: phase-0-output-format
declared_at: 2026-06-02T19:50:00Z
next_phase: phase-1c
```
