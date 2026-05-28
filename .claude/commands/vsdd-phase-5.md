---
primer_id: vsdd-phase-5
phase: phase-5
version: 0.1.0
frequency: per layer (optional — declared in DESIGN.md § Project intent `Phase 5 strategy:`)
governing_skill: true
relevant_domains: [quality-engineer, security, solution-architect]
supplements_in_scope: []
---

# Phase 5 Primer: Formal Hardening

## Composition

You are entering Phase 5 (Formal Hardening). Per the phase-domain composition matrix, load:

- **Quality Engineer** — Mutation Testing + Fuzz Testing
- **Security** — Fuzz Testing (when threat model names the parser) + adversarial-hardening review
- **Solution Architect** — Purity Boundary Audit + Proof Execution

Plus the always-on baseline. Skill mode for tool runs + bounded-disposition surfaces; cold-session for first-run baselines or adversarial-framing surfaces (per the cold-session-vs-inline rubric).

## Phase-specific discipline

Phase 5 produces evidence of correctness **qualitatively different** from Phase 3 cold-batch adversarial review. Phase 3 proves the implementation passes the spec's tests; Phase 5 probes whether the spec's tests are themselves strong enough.

**Four surfaces** (each independent — a layer may exercise all, some, or one per project intent):

**Surface A.0 — Purity Boundary Audit (required preamble).** Audit the implementation against every authoritative purity claim. Cross-source verification: DESIGN.md § Verification architecture vs. module-doc claims vs. actual implementation. Mismatches route via Phase 4.

**Surface A — Property-based testing for purity boundary.** Express spec invariants as properties using language-idiomatic tools (Rust: `proptest`; JS/TS: `fast-check`; Python: `hypothesis`; Go: `gopter`). Anti-pattern: property whose only assertion is "doesn't panic." Each property maps to a DESIGN.md invariant.

**Surface B — Mutation Testing.** Mutate the source; re-run tests; surviving mutants indicate test-suite blind spots. Tools: Rust `cargo-mutants`; JS/TS `Stryker`; Python `mutmut`. Per-surviving-mutant disposition: equivalent (with proof) / missing-test-added / spec-gap-routed / unviable. Anti-pattern: aggregate-only kill-rate reporting.

**Surface C — Fuzz Testing.** For parser / input-boundary surfaces, run language-idiomatic fuzzers (Rust `cargo-fuzz`; C/C++ `libFuzzer`; JS/TS `fast-check` with parser-input generators; Python `atheris`; Go stdlib `testing.Fuzz`). Crashes route via Phase 4 (typically Phase 2b for impl defects; Phase 1a+1b for spec gaps). Anti-pattern: "no crashes found" after short fuzz run without naming the budget elapsed.

**Surface D — Proof Execution (strictly optional; capstone+ intent only).** For formal-proof candidates named in DESIGN.md § Verification architecture, write proof harnesses (Rust `kani`; C `CBMC`; system-level `TLA+`; Coq / Lean / Idris / Agda for full proofs). Each harness establishes a non-trivial spec-asserted property. Anti-pattern: tautology-property harness.

## The Exacting Mentor stance per surface

**Primary failure mode:** Treating Phase 5 as "ran the tool" rather than "designed the hardening to target the spec's named verification-architecture surface." A `cargo-mutants` 90% kill rate against a weak test suite is weaker than 70% against a strong one. Phase 5 evidence is judged against DESIGN.md § Verification architecture, not against tool defaults.

**Sycophancy check per surface:** named at each surface above. Re-read each property's assertions, verify mutation-equivalence proofs, name fuzz budgets elapsed + coverage signal, trace each proof harness to a DESIGN.md sentence.

## Pre-phase composition declaration template

```yaml
phase: phase-5
composed_domains: [quality-engineer, security, solution-architect]
composition_mode: skill-interactive  # OR cold-session-cluster-spawn (first-run baselines)
cold_session_shape: <N/A — bounded judgment surface | cold-session cluster spawn — adversarial-framing judgment>
surfaces_active: [A.0, A, B, C, D]  # subset per project Phase 5 strategy
operator_confirmation: confirmed
declared_at: <ISO 8601 timestamp>
```

## Phase-completion criteria

Phase 5 closes for a layer when (per DESIGN.md § Project intent `**Phase 5 strategy:**` declaration):

1. Purity Boundary Audit preamble verified for the layer
2. Each declared-active surface has a recorded round in the appropriate per-domain log (SA log for A/A.0/D; QE log for B/C — with `**Phase 5 surface:**` preamble tag)
3. Every surviving mutant within evaluation scope has a per-mutant disposition (no aggregate-only reporting)
4. Every Phase 5 finding routed to Phase 4 has either been Resolved or Deferred-with-named-trigger
5. The project's `**Phase 5 strategy:**` declaration's named scope is complete

Emit `PhaseExited{phase: phase-5, exit_status: phase-5-mvr, layer: <N>, surfaces_completed: [<list>]}` at the closing commit.

## Cross-references

- [Phase 3 primer](./vsdd-phase-3.md) — Phase 5 sits AFTER Phase 3 implementation-MVR for the layer
- [Phase 6 primer](./vsdd-phase-6.md) — Convergence (consumes Phase 5 MVR signals)
- [Quality Engineer domain](./vsdd-domain-quality-engineer.md) — Mutation + Fuzz Testing
- [Security domain](./vsdd-domain-security.md) — Fuzz Testing + adversarial hardening
- [Solution Architect domain](./vsdd-domain-solution-architect.md) — Purity Boundary + Proof Execution
