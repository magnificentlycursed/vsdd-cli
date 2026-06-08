# Phase 1b — Verification Architecture

**Issue:** crosslink #13.
**Consumes:** [Phase 1a behavioral specification](./phase-1a-behavioral-spec.md).

## Pre-phase composition declaration

```yaml
phase: phase-1b
composed_domains: [solution-owner, solution-architect, quality-engineer, sanity-check]
composition_mode: skill-interactive
supplements_loaded: [rust]
operator_confirmation: confirmed
declared_at: 2026-06-07T00:05:00Z
```

## Pure functions

Phase 5 property-testing surface for the four bundled changes:

| Function | Inputs | Output | Purity grounds |
|---|---|---|---|
| `Finding::format_tty(&self)` | Finding ref | TTY string | Pure formatting — deterministic; no I/O. Already exists at `mdatron-core/src/diagnostic.rs:76-99`. Phase 2 makes it the CLI's single source of truth |
| `explain_catalog::lookup(&code)` | string | `Option<&'static str>` | Pure lookup over a compile-time-embedded table; deterministic |
| `explain_catalog::all_codes()` | () | `&'static [&'static str]` | Pure compile-time-known set; deterministic |
| `readme_round_trip::extract_example(&readme)` | README string | `(schema_text, pattern_text, example_md)` triple | Pure markdown-fence extraction over a string — no I/O once content is loaded |

The first three are the direct Phase 5 candidates. The fourth is a test
helper rather than production code; it's pure but lives in test scope.

## Automatable vs manual

All four behavioral contracts are **fully automatable**.

| Contract | Surface | Tool |
|---|---|---|
| `--json` finalization (TTY explain line) | Integration test against `mdatron verify` on a fixture project | subprocess spawn + stderr substring assert |
| `mdatron explain CODE` | Integration test per baseline code + not-found case | subprocess spawn + stdout/stderr/exit-code assert |
| README presence + structure | Integration test reads file + asserts heading topics + round-trips embedded example | `std::fs::read_to_string` + substring asserts + subprocess spawn for the round-trip |
| `tests/cli_integration.rs` | The test file IS the verification surface | `cargo test --test cli_integration` |

**Zero manual-test surface.** Phase 2's four changes are all
operator-observable through deterministic CLI behavior; no UI flicker,
no accessibility surface, no human-judgment artifact. The
`manual-tests/layer-phase-2.md` stub (if the post-DESIGN.md
auto-scaffolding hook produces it) would carry the rationale "Phase 2
surfaces are deterministic CLI behavior; no manual tests apply at this
layer" matching Phase 0's pattern.

## Phase 5 candidates

The dominant Phase 5 surface here is property-based testing on the
pure functions above:

| Property | Function under test | Falsifying generator |
|---|---|---|
| `format_tty` round-trip stability: re-parsing the rendered string yields the original `(code, severity, message, location)` tuple | `Finding::format_tty` | `proptest` Arbitrary impl over Finding |
| `format_tty` rustc-shape invariant: every output starts with `<label>[<code>]: <message>` and contains `--> <file>:<line>` | `Finding::format_tty` | Arbitrary Finding |
| `explain_catalog::lookup` totality: every code in `all_codes()` returns `Some(non_empty_string)`; codes outside the set return `None` | `explain_catalog::lookup` | Arbitrary code-shaped strings |
| explain-catalog page-shape: every catalog entry contains the four required headings ("What this means", "How to fix", and the front-matter-style "Severity:" / "Introduced in:" lines) | catalog content (compile-time) | static check over `all_codes()` |
| README example round-trip: extracted schema + pattern + example markdown produces the documented diagnostic when fed to `mdatron verify` | end-to-end | the README content itself |

The last entry is property-test-shaped but runs in `cli_integration.rs`
because it needs subprocess spawn — Phase 5 would lift it to a fuzz
target on the markdown-fence-extraction routine.

**Fuzz testing surface:** none specifically for Phase 2 — the
JSON-bytes-into-output-parser fuzz target lives in vsdd-cli's
subprocess client (Phase 3 of the binary-first plan). mdatron-side
fuzz at v0.1.x candidates: the markdown-fence-extraction routine
(small surface; low ROI but cheap to author).

**Out-of-scope for Phase 5:**

- Mutation testing on Phase 2 surfaces: the surfaces are small and
  property-test-covered; mutation testing's ROI is in the DSL
  evaluator (Phase 1 + Phase 5 binary-first-plan candidate), not
  here
- Proof execution: no formal-verification surface at v0.1.0 for
  Phase 2; the contracts are wholly behavioral

## Trust boundaries

| Boundary | Direction | Trust posture | Hardening |
|---|---|---|---|
| `mdatron explain` argv → catalog lookup | Untrusted (operator-supplied code string) | Strict prefix match + exact-code-key lookup; case-sensitive (per Phase 1a edge case) | Existing argv parsing discipline; no expansion needed |
| CLI argv → `verify` | Untrusted (operator-supplied path + flags) | Clap derives + strict unknown-flag rejection | Existing discipline (Phase 0 BC); Phase 2 doesn't add CLI inputs |
| README content on disk → embedded-example extractor (test surface only) | Trusted (operator-controlled at repo-root) | None needed for production; tests assert round-trip | Test-time only |

The cross-process trust boundary (vsdd → mdatron stdout) is Phase 3
territory — unchanged by this phase. Phase 2's new trust surface is
the explain-code argv, which is a thin string lookup with no
parsing of structured content.

## Co-evolution with vsdd-cli

mdatron's explain catalog adds prose surface that vsdd-cli's
subprocess client may eventually surface to its own operators (e.g.,
when vsdd's interactive prompt offers "run `mdatron explain CODE` for
more"). v0.1.0 vsdd-side: no changes; vsdd's wrapper around mdatron
findings preserves `explain_ref` verbatim per Phase 0 namespace
separation. v0.1.x candidate: vsdd's `verify` subcommand teaches its
TTY output to surface "see `mdatron explain CODE`" hints on the
MDATRON-prefixed findings it parses from the output object.

## Phase 1b exit signal

```yaml
event: PhaseExited
phase: phase-1b
exit_status: complete
layer: phase-2-mdatron-json
declared_at: 2026-06-07T00:05:00Z
next_phase: phase-1c
```
