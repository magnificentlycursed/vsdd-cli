# Phase 0 — Wire-Format Contract DESIGN

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
context: wire-format contract for mdatron verify --json cross-process IPC

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
```

## Project intent

**Phase 5 strategy:** property-based testing on envelope round-trip invariants + fuzz testing on the consumer-side envelope parser. Mutation testing not applicable to a wire contract whose enforcement is structural (strict JSON Schema rejection at construction); the mutation surface lives in the implementation (Phase 5 of mdatron-implementation, not of this contract spec).

## Scope

This document specifies the **observable wire contract** for cross-process
communication between `vsdd` (consumer) and `mdatron` (provider) under the
binary-first refactor (per `docs/refactor/binary-first-plan.md` Phase 0).

The contract covers:

- Output stream shape: `mdatron verify --json` envelope on stdout
- Exit code semantics for the three operational states
- Global flag vocabulary (`--quiet` / `--json` / `--log-level` / `--log-format` / `--dry-run`)
- Error-code namespace separation between `MDATRON-Exxxx` and `VSDD-Exxxx`
- Wire-version compatibility discipline

Out of scope for Phase 1a: implementation details (Phase 2b), test surface
(Phase 2a), prose/documentation (Phase 2c).

## Behavioral contracts

### BC-1 — Envelope schema and version field

`mdatron verify --json` writes exactly one JSON object to stdout per
invocation. The object MUST carry a top-level `mdatron_wire_version` string
field.

**Observable assertions:**

- Field present on every successful invocation.
- Value is a string matching `^[1-9][0-9]*$` (positive integer; not semver).
- Version bumps follow: additive optional fields → no version bump; required
  field added or field-shape changed → version bump.

**Falsification path:** consumer can construct an envelope missing
`mdatron_wire_version` and assert mdatron's parser rejects it; or construct an
envelope with `mdatron_wire_version: "999"` and assert vsdd produces an
`"wire version unsupported"` error rather than parsing.

### BC-2 — Envelope top-level shape

The envelope's top-level object is exactly:

```json
{
  "mdatron_wire_version": "<integer-string>",
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
  fields at wire-version 1 (`additionalProperties: false`).
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

**Falsification path:** for any envelope state, consumer asserts the
documented invariant; a regression that emits an envelope with a missing
field, an extra field, or violates the `pipeline_status` ↔ `findings`
relationship fires the test.

### BC-3 — Finding shape

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
  end-of-span is not part of the v1 wire contract (Phase 6 candidate).
- Empty `message`: rejected at construction (vacuous diagnostic is itself a
  contract violation).

**Falsification path:** consumer can construct a Finding with mismatched
code prefix and severity (e.g., `code: "MDATRON-E0001"`, `severity: "warning"`)
and assert validation rejects it.

### BC-4 — Exit code semantics

`mdatron verify` exits per the three-state model:

| Exit code | Meaning | Wire envelope behavior |
|---|---|---|
| `0` | Pipeline ran to completion; no error-severity findings (warnings or lints may exist) | Envelope present, `pipeline_status: "ok"`, `summary.error_count == 0` |
| `1` | Pipeline ran to completion; at least one error-severity finding | Envelope present, `pipeline_status: "ok"`, `summary.error_count >= 1` |
| `2` | Pipeline did not run to completion (configuration error, IO failure, malformed pattern file, etc.) | Envelope MAY be present with `pipeline_status: "failed"`; MAY be absent if failure occurred before envelope construction (e.g., CLI parse error) |
| `>2` | Reserved for unanticipated failure modes (panic = 134 from SIGABRT, etc.) | Envelope absent |

**Observable assertions:**

- Successful clean run: exit `0`, `summary.error_count == 0`.
- Pipeline run with findings: exit `1`, `summary.error_count >= 1`.
- Pipeline failure: exit `2` or higher.
- Exit code is the consumer's primary signal; envelope is secondary
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

### BC-5 — Output stream contract

| Mode | stdout | stderr | Use case |
|---|---|---|---|
| Default (no `--json`) | silent | rustc-shaped diagnostics + final summary line | Interactive operator |
| `--json` | one JSON envelope (compact, single line) | rustc-shaped diagnostics + final summary line | Machine consumer + operator visibility |
| `--json --quiet` | one JSON envelope | silent | Pure machine consumer (CI) |
| `--quiet` (no `--json`) | silent | silent except for non-zero exit cause | Hooks/scripts where exit code is the only signal needed |

**Observable assertions:**

- The JSON envelope is emitted as a single compact line on stdout when
  `--json` is set; readers can parse with line-oriented tools.
- stdout never contains anything other than the JSON envelope when `--json`
  is set (no diagnostic text on stdout under any flag combination).
- stderr is for human-readable text; it is never machine-parsed by consumers.

**Edge cases:**

- Empty findings + `--json`: the envelope still emits (one line) with
  `findings: []`.
- Pipeline failure + `--json`: if the envelope is constructed (failure detected
  after pipeline init), it emits with `pipeline_status: "failed"`. If the
  failure prevents envelope construction (CLI parse error), no envelope.

**Falsification path:** consumer pipes stdout to `jq` and asserts parse
success on every invocation where exit ≤ 2.

### BC-6 — Global flag vocabulary

`mdatron` accepts the following global flags (applicable to all subcommands
unless noted):

| Flag | Effect |
|---|---|
| `--quiet` / `-q` | Suppress stderr human-readable output; exit code is the only signal |
| `--json` | Emit JSON wire envelope on stdout (verify only); other subcommands MAY emit JSON if explicitly opted-in |
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

### BC-7 — Error-code namespace separation

`MDATRON-Exxxx` and `VSDD-Exxxx` namespaces are **strictly separate** across
the cross-process boundary.

**Observable assertions:**

- mdatron NEVER emits a `VSDD-Exxxx` code in any envelope or stderr text.
- vsdd, when wrapping or summarizing mdatron's envelope, MUST preserve
  `MDATRON-Exxxx` codes verbatim. vsdd MUST NOT re-tag, alias, or wrap them
  under a different `VSDD-Exxxx` code.
- vsdd's own findings (against VSDD-content layered on top of mdatron's
  substrate) carry `VSDD-Exxxx` codes only.
- The reverse is also true: vsdd never emits `MDATRON-Exxxx` codes from its
  own code paths; if it surfaces mdatron findings, they keep their codes.

**Falsification path:** static check (lint) over both repos asserts code-prefix
constants are scoped to their owning binary; a regression that introduces a
`VSDD-Exxxx` literal in mdatron's source (or vice versa) fires the lint.

### BC-8 — Wire-version compatibility

Compatibility is enforced via the `mdatron_wire_version` field per BC-1:

**Consumer-side (vsdd) discipline:**

- vsdd pins the highest mdatron wire version it knows how to parse
  (compile-time constant).
- On envelope parse, vsdd reads `mdatron_wire_version` first:
  - If known and supported: parse normally.
  - If newer than known: error `VSDD-Exxxx wire-version-too-new` instructing
    operator to upgrade vsdd.
  - If older than known but still in supported window: parse with the older
    schema; emit a warning that mdatron is behind.
  - If older than the supported window: error
    `VSDD-Exxxx wire-version-too-old` instructing operator to upgrade mdatron.

**Provider-side (mdatron) discipline:**

- mdatron emits the highest wire version it supports.
- mdatron MAY support emitting older wire versions via a `--wire-version=N`
  override (deferred to v0.2; not in v0.1.0 scope).

**Versioning rule:**

- Add an optional field: no version bump (consumers ignore unknown fields).
- Remove a field, add a required field, change a field's type or semantics:
  bump `mdatron_wire_version`.
- SemVer-major bump on mdatron may carry a wire version bump but isn't
  required to (a major rewrite of internals can keep wire version stable).

**Falsification path:** consumer can construct envelopes at boundary versions
and assert vsdd's handling matches the documented matrix.

## Invariants (audit checks across contracts)

| Invariant | Check |
|---|---|
| Every emitted code is in mdatron's reserved-code table | Static lint over mdatron source asserts every code literal matches the table |
| Exit code corresponds to `pipeline_status` and `summary.error_count` | Property test on Finding emission: `(pipeline_status, error_count) → exit_code` is a pure function |
| Envelope's `summary` counts match `findings` array | Property test: derive counts from array; assert equality |
| `findings` array entries' `code` prefixes match `severity` field | Per-Finding validation |
| `mdatron_wire_version` is monotonically non-decreasing across releases | CHANGELOG audit |

## Verification architecture

### Pure functions

Functions on this contract that are deterministic + side-effect-free are
candidates for Phase 5 property-based testing:

| Function | Inputs | Output | Purity grounds |
|---|---|---|---|
| `build_envelope(findings, files_checked, mdatron_version)` | array of Finding + non-negative integer + semver string | envelope object | Pure construction from inputs; no I/O |
| `compute_summary(findings)` | array of Finding | summary object with the four counts | Pure reduction; deterministic counts |
| `derive_exit_code(pipeline_status, error_count)` | enum + non-negative integer | exit code integer | Pure function of two inputs per BC-4 table |
| `parse_envelope(json_bytes)` | bytes | Result<Envelope, ParseError> | Pure given strict JSON parser (no environment lookup) |
| `validate_finding(finding)` | Finding | Result<(), ValidationError> | Pure; checks code-prefix-vs-severity alignment + location bounds |
| `code_in_namespace(code, expected_prefix)` | string + string | bool | Pure prefix match |
| `compare_wire_versions(emitted, supported_window)` | integer + range | enum (Supported / Too-New / Too-Old / In-Window-Old) | Pure comparison |

These seven functions form the purity boundary for the wire contract. Phase 5
property tests target this list.

### Automatable vs manual classification

All 8 behavioral contracts (BC-1 through BC-8) are **automatable**. The
wire contract is operator-invisible (no UI; no human-readable artifact
under inspection); there is no `manual-tests/wire-format.md` checklist.

| BC | Test surface | Automation tool |
|---|---|---|
| BC-1 (envelope schema/version field) | JSON Schema validation + Rust struct deserialization round-trip | `jsonschema` crate + `serde_json` |
| BC-2 (envelope top-level shape) | Integration test against `mdatron verify --json` output | subprocess spawn + JSON parse + struct assert |
| BC-3 (Finding shape) | Per-Finding property test | `proptest`: generate Finding tuples; assert validate_finding succeeds for well-formed; fails for code-severity mismatch |
| BC-4 (exit codes) | Integration matrix: 6 fixture scenarios × asserted exit code | subprocess spawn + exit-code assertion |
| BC-5 (stream contract) | Integration test capturing stdout + stderr separately | subprocess capture |
| BC-6 (global flags) | Integration test per flag combination | clap + subprocess; assert unknown-flag rejection |
| BC-7 (namespace separation) | Static lint (compile-time) over both repos | custom proc-macro or grep-based lint in CI |
| BC-8 (wire-version compatibility) | Property test on `compare_wire_versions` + integration test on boundary envelopes | `proptest` + fixtures with synthetic wire versions |

### Phase 5 candidates

Per the **Phase 5 strategy** declared above:

**Property-based testing (the dominant Phase 5 surface here):**

| Property | Function under test | Falsifying generator |
|---|---|---|
| Envelope round-trip: `parse(serialize(envelope)) == envelope` | `build_envelope` + `parse_envelope` | `proptest` Arbitrary impl over Envelope |
| Summary correctness: `compute_summary(findings).error_count == findings.iter().filter(\|f\| f.severity == "error").count()` | `compute_summary` | Arbitrary Vec<Finding> |
| Exit-code monotone: `error_count == 0 → exit_code in {0}`; `error_count >= 1 → exit_code in {1}`; `pipeline_status == failed → exit_code in {2,…}` | `derive_exit_code` | Arbitrary `(pipeline_status, error_count)` tuples |
| Finding code-severity alignment: validate succeeds iff first letter of code matches severity letter | `validate_finding` | Arbitrary code + severity, biased toward mismatches |
| Wire-version comparison transitivity: standard total-order properties | `compare_wire_versions` | Arbitrary triples |

**Fuzz testing surface:**

| Target | Tool | Corpus seed |
|---|---|---|
| `parse_envelope(arbitrary_bytes)` | `cargo-fuzz` (libFuzzer) | One real-mdatron envelope + 1KB of structured-but-invalid JSON |

Fuzz target asserts: no panic; either Ok(Envelope) or Err(ParseError); never UB.

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
| `mdatron verify --json` stdout (bytes) → vsdd parser | Untrusted (subprocess could be old version, attacker-controlled binary, etc.) | Strict JSON parsing + schema validation + version check before structural use | Fuzz target on `parse_envelope`; wire version asserted first |
| CLI argument parsing | Untrusted (operator may pass malformed flags) | Clap derives + strict unknown-flag rejection | Standard clap discipline |
| Schema/pattern file content on disk → mdatron evaluator | Untrusted (operator-supplied) | Strict JSON Schema parsing; refuse-malformed | Existing mdatron discipline (out of wire-contract scope) |
| Environment variable resolution (RUST_LOG, MDATRON_*) | Semi-trusted (operator-set) | Bounded enum parsing; log-level enum is closed | Strict enum parse; default on invalid |

The cross-process boundary between vsdd → mdatron stdout is the **load-bearing
trust boundary** for this contract. All others are pre-existing surfaces from
prior phases.

## Falsification surface (Phase 1c will operationalize)

Each contract above contains a "falsification path" naming how it can be
broken. Phase 1c (acceptance criteria) settles which of these become
mandatory tests; Phase 2a writes them.

Concrete should-fire fixtures candidates:

- Envelope missing `mdatron_wire_version` field
- Envelope with `additionalProperties` at top level
- Finding with mismatched code prefix and severity
- Pipeline-fail run that exits `0` (regression: exits `1` when error finding present, `2` when pipeline broke)
- `--json` invocation emitting non-JSON content on stdout
- Consumer parsing envelope with an unsupported `mdatron_wire_version`
- vsdd emitting a literal `MDATRON-` code prefix in its own diagnostic surface
- mdatron emitting a literal `VSDD-` code prefix anywhere

## Decomposition (Phase 1c)

The wire-format contract decomposes into three milestones with sequential
dependencies. Each is independently buildable + verifiable.

### Milestone M1 — Envelope shape

**Closes:** BC-1 (envelope schema/version field), BC-2 (envelope top-level
shape), BC-3 (Finding shape).

**Acceptance criteria:**

- `mdatron verify --json` emits a single JSON object on stdout per
  invocation, structurally conformant to BC-2's schema
- Every emitted Finding conforms to BC-3 (code-prefix-vs-severity alignment
  enforced)
- `mdatron_wire_version` is present and parseable per BC-1
- JSON Schema for the envelope exists at `<repo>/docs/refactor/phase-0-wire-format/wire-envelope.schema.json` (built in Phase 2b; referenced from this DESIGN)

**Phase 2a Red Gate seed:**
- Envelope-missing-version-field fixture
- Envelope-with-extra-top-level-field fixture
- Finding-with-mismatched-code-prefix-and-severity fixture

**Dependencies:** none (foundational).

**Exit Signal pointer:** `ExitSignaled{milestone: m1-envelope-shape}` emitted
on the closing commit.

### Milestone M2 — Process behavior

**Closes:** BC-4 (exit codes), BC-5 (stream contract), BC-6 (global flag
vocabulary).

**Acceptance criteria:**

- Exit-code matrix: clean run = 0, errors-exist = 1, pipeline-failed = 2;
  warning-only run = 0
- stdout under `--json` contains only the envelope (no diagnostic text)
- stderr always carries the human-readable rustc-shaped output unless
  `--quiet`
- All 5 global flags (`--quiet` / `--json` / `--log-level` / `--log-format`
  / `--dry-run`) parse + apply per BC-6 table
- Unknown-flag rejection with descriptive stderr message

**Phase 2a Red Gate seed:**
- Warning-only-exits-1 regression fixture (must exit 0)
- Pipeline-failure-without-envelope-in-stderr fixture
- Unknown-flag-silent-success fixture (must error)
- `--quiet --json` smoke fixture (asserts envelope on stdout, silence on stderr)

**Dependencies:** M1 (process behavior is observed by inspecting the envelope).

**Exit Signal pointer:** `ExitSignaled{milestone: m2-process-behavior}`.

### Milestone M3 — Contract discipline

**Closes:** BC-7 (namespace separation), BC-8 (wire-version compatibility).

**Acceptance criteria:**

- Static lint over both repos asserts that `MDATRON-Exxxx` string literals
  appear only in mdatron's source; `VSDD-Exxxx` literals only in vsdd's
  source. Lint exit-code-fails the CI on violation.
- Wire-version compatibility:
  - vsdd's compile-time constant declares the highest mdatron wire version it
    supports
  - vsdd refuses to parse an envelope with `mdatron_wire_version` higher
    than that constant; emits `VSDD-Exxxx` wire-too-new error
  - vsdd accepts an envelope with `mdatron_wire_version` equal to the
    constant
  - Older-than-window handling deferred to v0.2 (no compatibility window in
    v0.1.0; mdatron == vsdd's pinned version)

**Phase 2a Red Gate seed:**
- VSDD-source-emits-MDATRON-prefix fixture (lint fires)
- MDATRON-source-emits-VSDD-prefix fixture (lint fires)
- Envelope-with-wire-version-999 fixture (vsdd refuses to parse)
- Envelope-with-pinned-wire-version fixture (vsdd parses successfully)

**Dependencies:** M1 (envelope must exist for the version check; namespace
lint is repo-static).

**Exit Signal pointer:** `ExitSignaled{milestone: m3-contract-discipline}`.

### Decomposition rationale

Three milestones rather than one or eight because:

- **One milestone** would bundle BC-7's lint surface (purely static) with
  BC-1's parser surface (dynamic JSON construction), making the Red Gate
  hard to falsify in aggregate (any-one-test-passing masks failures in
  others). Phase-1c primer flags this as the "milestone whose acceptance
  criteria don't cover the behaviors it claims to close" failure mode.
- **Eight milestones (one per BC)** would force artificial sequential
  dependencies (BC-1 standing alone is meaningless without BC-2; BC-4
  without BC-5 is unobservable). Phase-1c primer flags this as the
  "milestone that bundles too many behaviors / not enough cohesion" failure
  mode.
- **Three milestones** (envelope shape / process behavior / contract
  discipline) correspond to three distinct verification mechanisms (schema
  validation / subprocess integration / static lint + boundary fixtures),
  each independently runnable in CI.

### manual-tests/layer-phase-0.md note

The wire contract is operator-invisible; no manual-test checklist applies
to this layer. The Phase 1b verification architecture explicitly declared
zero manual-test surface. `manual-tests/layer-phase-0.md` would be a stub
file with "no manual tests apply at this layer — wire contract is
machine-only" and no checkbox items.

Per the post-design-md-modification hook design (DESIGN-METHODOLOGY.md
§ Post-DESIGN.md auto-scaffolding), the hook would still emit the stub
file for symmetry; the `falsifiability_check` field carries the
no-manual-tests rationale.

## Open questions (Raise-to-SO)

1. **Should `mdatron_wire_version` use semver or simple integer?**
   - Simple integer (proposed): easier reasoning; bumps less ambiguous.
   - Semver: more familiar; allows minor/patch distinctions.
   - SO disposition needed.

2. **Should `mdatron explain CODE` ship in v0.1.0?**
   - Strip `= explain:` line (DR-F2 lean): defers explain to v0.2.
   - Implement explain (more work): preserves the catalog discipline.
   - SO disposition needed for v0.1.0 ship scope.

3. **Reserved exit codes above 2:**
   - 3 = "binary unavailable / version mismatch" was proposed (vsdd's reading
     when subprocess fails to spawn).
   - But: this exit code is set by *vsdd*, not by mdatron (mdatron can never
     fail-to-spawn from its own perspective).
   - Disposition: drop `3` from mdatron's contract; vsdd uses `3` for its
     own "couldn't invoke mdatron" state internally without it appearing in
     mdatron's envelope/exit.

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
- Phase 3 — Vsdd subprocess client (parsing the envelope per BC-2/BC-3)

## Phase 1a exit signal

Phase 1a closes when:

- ✅ DESIGN.md § Behavioral contracts is non-empty (BC-1 through BC-8 above)
- ✅ Every behavior is specific + testable + has named edge cases
- ⚠️ Per-axes-activated domains have surfaced their lens (no axes activated
  for this design — wire format is operator-invisible)
- ⚠️ Cold reviewer (DR) iteration pending — Phase 3 will adversarially
  refine the contracts

Emit on Phase 1a closing commit (deferred to Phase 1b joint close per Phase 1a
primer's "Operators may author Phase 1a and Phase 1b in a single session"
allowance):

```yaml
event: PhaseExited
phase: phase-1a
exit_status: complete
layer: phase-0-wire-format
declared_at: 2026-06-02T19:30:00Z
next_phase: phase-1b
```

## Phase 1c exit signal

Phase 1c closes when:

- ✅ DESIGN.md § Decomposition lists all milestones M1..M3 with acceptance criteria per milestone
- ✅ Each milestone's acceptance criteria are a non-empty subset of DESIGN.md § Behavioral contracts (M1→BC-1/2/3; M2→BC-4/5/6; M3→BC-7/8)
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
exit_status: complete-pending-so-disposition
layer: phase-0-wire-format
declared_at: 2026-06-02T20:05:00Z
next_phase: phase-2a
milestones_opened: [m1-envelope-shape, m2-process-behavior, m3-contract-discipline]
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
layer: phase-0-wire-format
declared_at: 2026-06-02T19:50:00Z
next_phase: phase-1c
```
