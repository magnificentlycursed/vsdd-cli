---
schema_class: review-entry
schema_version: 1.0.0
review_number: 2
date: 2026-06-02
phase: phase-3
scope: >-
  Crosslink #12 bundled changes per docs/refactor/phase-1-codes-and-dsl/DESIGN.md
  — reserved-code drift fix (E0001/E0002 swap + E0050/E0070/E0080 reservation),
  DSL Field-on-Object-missing-key symmetry, defined() empty-string carve-out drop.
  AI Engineer lens applied at operator direction: Claude Code is the direct
  consumer of mdatron verify stderr (rustc-shaped diagnostics carry MDATRON-Exxxx
  codes that an agent reads), the all_emitted_codes_are_reserved lint runs on
  every `cargo test`, and pattern-authoring agents previously relied on the
  pre-change DSL behavior.
lens: >-
  AIE dim 2 (cost-band cataloging per operation), dim 3 (prompt-cache discipline),
  dim 5 (sub-agent scope-down), dim 7 (rate-limit headroom). Cross-cutting:
  Claude Code as direct consumer of mdatron output — semantics-bearing literals
  in prompt-context and agent muscle memory.
source: operator-directive
session_note: >-
  Cold-session Phase 3 reviewer per phase-3 primer; no prior-session memory of
  crosslink #12 design. Operator directive named the AIE lens explicitly
  ("codes affect Claude Code's consumption of mdatron output"). Five questions
  framed by operator; one finding per question, plus a synthesis headline.
model: claude-opus-4-7
execution_method: >-
  Inline single-domain AI Engineer cold reviewer; read DESIGN.md, phase-3 primer,
  ai-engineer + sanity-check domain prompts, claude-code-cli supplement, the
  cross-references.yaml pattern files (vsdd-core/ + .mdatron/ copies), and prior
  bundle reviews (QE F2/F4, PE F1, DR F1) for cross-finding coherence. Verified
  literal-search claims with grep across .md/.py/.json/.yaml in .claude/,
  supplements/, vsdd-core/, .mdatron/.
sycophancy_compensation: >-
  AIE bias is "more observability, more cost-bands, more cache discipline."
  Counter-bias for v0.1: a code rename inside mdatron-core is not an AI-runtime
  surface unless Claude Code reads the literal. I am grading the operator's five
  questions against actual artifact evidence — when the grep returns empty, the
  finding is Hallucinated, not "let's add a hook anyway."
supplements_loaded: [claude-code-cli]
---

# AI Engineer Review — Phase 1 codes-and-dsl bundle (crosslink #12)

## Headline

The rename is AI-runtime-inert in this checkout: zero MDATRON-Exxxx literals
in `.claude/`, `supplements/`, `vsdd-core/`, or `.mdatron/` outside the spec
+ review-log. Claude Code consumes mdatron stderr per-invocation (no
persisted memory of code numerics), so the rename has no cache-poisoning or
context-staleness surface. The genuine AIE concerns are (1) DSL Field-symmetry
removes a crash that taught agents to add `defined()` guards — agent-confidence
shifts from "implicit fail-loud" to "explicit guard-or-silently-null"; (2) the
`defined("")` semantic flip silently invalidates the muscle-memory pattern
"`defined(s)` = non-empty string"; (3) the lint's per-test-run cost is
negligible at human-keystroke cadence but enters the hot loop if an
agent-driven verify hook runs per file-save.

## Per-question findings

### F1 — Hardcoded MDATRON-E0001/E0002 literals outside spec/source: NONE FOUND
**domain:** ai-engineer **dim:** 2 (cost-band cataloging) **classification:** hallucinated **routing:** none
**source:** director-raised

Grep across `*.md` / `*.py` / `*.json` / `*.yaml` for `MDATRON-E0001` and
`MDATRON-E0002` returns hits only in (a) `docs/refactor/**/DESIGN.md` files,
(b) `review-log/` history, (c) `docs/refactor/binary-first-plan.md`. NO
hits in `.claude/commands/`, `.claude/hooks/` (directory absent),
`supplements/`, `vsdd-core/patterns/`, `.mdatron/patterns/`, `methodology.md`.
The codes are not embedded in any prompt template, hook script, slash command,
or context-injection payload. Prompt-cache staleness surface: zero.

Disposition: hallucinated — the rename has no AI-runtime context-staleness cost.
The concern would be live if any `.claude/commands/vsdd-*.md` quoted a specific
code as a working example; none do. Confidence boundary: I did not scan the
sibling mdatron repo's `.claude/` (not on disk in this checkout); if mdatron
maintains its own slash commands referencing `MDATRON-E0001`-as-example, that
is a sibling-repo finding routed there.

### F2 — Claude Code as direct stderr consumer: rename is per-invocation-fresh; saved-context surface is zero
**domain:** ai-engineer **dim:** 3 (prompt-cache discipline) **classification:** hallucinated **routing:** none
**source:** director-raised

Claude Code reads `mdatron verify` stderr via the Bash tool — the output is
a fresh subprocess capture per invocation. No prompt-cache entry persists
the rustc-shaped diagnostic across invocations (the diagnostic is a tool
result, not a system-prompt fragment; tool results are cached by the
calling-turn boundary, not by code-numeric identity). Within a single
session, an agent may have read the OLD code in stderr earlier and the NEW
code now — but the diagnostic line is self-describing (`error[MDATRON-E0050]:
frontmatter-schema-violation`), so the agent has the semantic next to the
code, not just the number. No `.claude/CLAUDE.md` in this checkout pins
example codes; no slash command in `.claude/commands/` references a specific
code as canonical.

Disposition: hallucinated — agent-consumption is robust to the rename because
the rustc-shaped diagnostic carries summary text adjacent to the code. The
adversarial probe ("an agent that learned `E0001 = schema-violation` will
misread the new `E0001 = parse-failed`") fails because the diagnostic text
disambiguates per emission.

### F3 — Field-symmetry: agent-authoring confidence shifts from fail-loud to silent-Null; explicit defined() guards become mandatory
**domain:** ai-engineer **dim:** 5 (sub-agent scope-down) **classification:** accepted **routing:** documentation-reviewer
**source:** director-raised

Pre-change, an agent helping an adopter write a DSL pattern could rely on
the implicit "missing-key crashes loudly" failure mode as a development
signal — the rule would refuse to evaluate, surfacing the bug at the
pattern's first invocation. Post-change, the same agent must explicitly
add `defined($self.field) and …` guards or the rule flows silently with
`Null`, producing a false-pass. Vsdd-core's own patterns already encode this
discipline (`vsdd-core/patterns/cross-references.yaml:32,40,48,56,64` all
wrap with `every(… defined(…))` or top-level `defined(key(…))`), so the
existing corpus is robust. The cost lands on future pattern authoring:
an agent that writes `assert: $self.field == "x"` (without the `defined()`
guard) now produces a rule that silently passes when the field is absent.

Routing to DR: the pattern-authoring example in any future tutorial must
lead with the guard, not the bare comparison. The `every(s in $self.…, …)`
form is naturally safe (empty iteration is vacuously true); the dangerous
form is direct field-equality on optional fields. AIE flag to DR: surface
this as a documented authoring-discipline note before the next adopter
on-ramps.

### F4 — defined() carve-out drop: corpus audit confirms no pattern relied on the old "non-empty string" semantic
**domain:** ai-engineer **dim:** 5 (sub-agent scope-down) **classification:** resolved-pending **routing:** quality-engineer
**source:** director-raised

Grep of `vsdd-core/patterns/cross-references.yaml` + `.mdatron/patterns/cross-references.yaml`
for `defined(` returns 7 + 7 hits, all of the shape `defined(key(…))` or
`defined($self.field)`. None passes a `Value::Str` argument that could be
empty; all arguments are either dictionary-lookups (key()) or whole-field
references that resolve to non-Str types (slugs are Str, but the patterns
test `defined(key("domains", slug))` — the lookup result, not the slug
itself). Corpus is invariant under the carve-out drop.

The AIE-relevant residual: an agent helping a future adopter write a pattern
of the shape `defined($self.title)` to mean "title is set and non-empty"
will silently break post-change. This is the symmetric concern to F3 —
authoring-discipline note: `defined()` now means "not Null"; for
"non-empty string" use `$self.field != ""`. Route to QE to add a property
test fixture demonstrating both semantics (already named in QE's F4 as a
deferred Phase 5 surface — cross-finding coherence holds).

### F5 — all_emitted_codes_are_reserved lint cost: negligible per cargo test; enters hot loop only if a per-keystroke verify-hook fires the test suite
**domain:** ai-engineer **dim:** 7 (rate-limit headroom) **classification:** dismissed **routing:** none
**source:** director-raised

The lint runs at `cargo test` time, not at `mdatron verify` time. Claude
Code consuming mdatron output in a hot loop (per-keystroke validation) does
NOT trigger the lint — the verify subcommand calls the validator, not the
test suite. The lint's I/O cost (a directory walk + regex over Rust source)
lands on test invocation, which an agent runs at commit-prep cadence, not
per-keystroke. At the Phase 3 reviewer cluster batching shape (4 clusters
~6-10k tokens each), one extra `cargo test` invocation per Phase 3 round
is well within the cost band.

The concern would be live if (a) a hypothetical IDE-integrated agent ran
`cargo test phase_1_contracts::all_emitted_codes_are_reserved` per file-save
(unlikely; the per-keystroke surface is `mdatron verify`, not `cargo test`),
or (b) a CI hook ran the full test suite per push for cost-budget-relevant
agents. Neither is the actual workflow per `binary-first-plan.md`.

Disposition: dismissed — cost characteristics are correct for the actual
agent-invocation pattern. The lint is a build-time gate, not an
agent-hot-loop tax.

## Summary

- **2 hallucinated** (F1 + F2 — rename has no AI-runtime context-staleness
  surface; literals confined to spec + history; stderr consumption is
  per-invocation-fresh)
- **1 accepted** (F3 — Field-symmetry shifts agent-authoring confidence;
  DR routing for authoring-discipline note)
- **1 resolved-pending** (F4 — corpus audit confirms safety; QE routing
  for property test cross-coherent with QE F4)
- **1 dismissed** (F5 — lint cost lands at cargo-test cadence, not
  agent-hot-loop)

Phase-3 swarm-invocation signal: 2/5 hallucinated, 1 dismissed, 0 deferred,
2 routable. AIE does NOT raise implementation-MVR-blocking concerns for the
bundle. The two routable findings (F3 + F4) are documentation /
property-test surface, both already on QE's deferred Phase 5 list per
cross-finding coherence with `2026-06-02-quality-engineer-phase-1-bundle.md`
F4. Net AIE-add: one DR-side authoring-discipline note before the next
adopter on-ramp; no v0.1 ship-blocker.
