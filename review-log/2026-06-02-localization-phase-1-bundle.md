---
schema_class: review-entry
schema_version: 1.0.0
review_number: 4
date: 2026-06-02
phase: phase-3
scope: >-
  vsdd-cli crosslink #12 Phase 1 bundle (codes-and-dsl). Localization lens over
  docs/refactor/phase-1-codes-and-dsl/DESIGN.md plus the three probes the
  director seeded — error-string i18n posture, is_reserved_mdatron_code()
  char-boundary safety, and DSL string len() char-vs-byte semantics.
lens: >-
  Localization — string-identifier vs prose-text contract; multi-byte safety
  at string-slice sites; locale-aware string-length semantics. Activation is
  axis-declared (none on this project) plus director-raised probe.
source: director-raised
session_note: >-
  Cold-session sub-agent, worktree-no-memory. Localization is not an active
  axis on vsdd-cli (no locale declared in DESIGN.md; CLI is anglophone-only).
  Director's three probes evaluated on merits against DESIGN.md spec; source
  files for is_reserved_mdatron_code() and expr.rs:314 not yet materialized
  in workspace (pre-implementation), so probes are evaluated as
  spec-vs-claim, not code-vs-claim.
model: claude-opus-4-7
execution_method: >-
  Cold-session sub-agent, Phase 3 primer + localization + sanity-check prompts
  loaded fresh; DESIGN.md and review-entry.json schema read; no prior-session
  memory.
sycophancy_compensation: >-
  No locale declared = high prior toward dismissal. Compensation — each of the
  three director-seeded probes named explicitly + evaluated against DESIGN
  contract, not waved off as "no locale therefore N/A."
supplements_loaded: []
---

# Localization Phase 3 Review — Crosslink #12 Bundle

## L10N-F1 — Lens disposition: dismissed (no axis declared; no load-bearing finding under any of the three probes)

**Classification:** dismissed. **Routing:** none.
**Lens:** Localization (all eight dimensions).

vsdd-cli declares no locale axis. The CLI surface is anglophone-only; no
locale catalog, no MessageFormat, no per-locale layout. The Phase 1 bundle
(crosslink #12) touches three internal surfaces: reserved-code-table drift,
DSL Field-access symmetry, and `defined()` carve-out drop. None alters an
operator-visible string contract that would shift the localization posture.

Director-seeded probes, each disposed:

1. **Error codes as stable identifiers vs prose as translatable surface.**
   The design already lands the correct shape — `MDATRON-E0001`-style codes
   are the stable machine identifier; the human summary
   (`"frontmatter-parse-failed"`) is descriptive prose attached to the code,
   not the identity. This is exactly the i18n-ready posture: a future locale
   axis would translate the prose while the codes remain the cross-locale
   anchor. No finding; the design is i18n-shaped without claiming to be.

2. **`is_reserved_mdatron_code()` char-boundary safety.** Reserved codes are
   `MDATRON-[ELWHISD]\d{4}` — pure ASCII by spec
   (`DESIGN-MDATRON.md:506-514` + amended ranges). The function's domain is
   compile-time string literals authored by the project, not
   network-attacker-controlled input; the "maliciously-crafted `MDATRON-Éxxxx`"
   threat model presupposes an injection path that does not exist (these
   strings are baked into Rust source). Defensive `is_char_boundary()` guards
   are belt-and-suspenders without a real threat surface; routes to Security
   if at all, not Localization. Dismissed at this lens.

3. **DSL `len()` char-vs-byte on `Value::Str`.** The director's note asserts
   `expr.rs:314` already uses char-count; source not yet materialized in
   workspace (pre-impl), so verification deferred to Phase 5 property test —
   but if char-count holds, that is the locale-correct choice (byte-count
   would mis-count multi-byte UTF-8 graphemes and silently break any future
   non-ASCII pattern corpus). Recommend Phase 5 property: `len(s)` equals
   `s.chars().count()` for arbitrary UTF-8 strings including CJK + combining
   marks; flag if grapheme-cluster semantics are wanted instead (separate
   contract decision, not a Phase 1 finding).

No locale declared = no string-extraction-completeness finding, no
plural-rule finding, no RTL finding, no text-expansion finding. The bundle
is localization-neutral.

## Disposition summary

| ID | Classification | Routing | Lens |
|---|---|---|---|
| L10N-F1 | dismissed | none | Localization (all dims) |

Cold-session isolation held; no findings hallucinated; no findings raised.
Sanity-check pass — dismissal is grounded in axis-declaration absence +
per-probe disposition, not abdication. If a future milestone declares a
locale axis, the three probes above are the re-activation seeds.
