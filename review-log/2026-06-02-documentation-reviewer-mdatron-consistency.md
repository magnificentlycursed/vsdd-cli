---
schema_class: review-entry
schema_version: 1.0.0
review_number: 3
date: 2026-06-02
phase: phase-1c
scope: >-
  mdatron's operator-facing surface for absorbability as crosslink-substrate
  baseline. Cold-read of mdatron-cli/src/main.rs (verb + flag vocabulary;
  rustc-shaped diagnostic printers); mdatron-core/src/diagnostic.rs
  (Finding/Severity/Location + format_tty contract); mdatron-core/src/verify.rs
  (VerifyError Display strings); mdatron-core/src/dsl/expr.rs:177-203
  (EvalError variants); mdatron-core/src/error.rs (Error enum);
  DESIGN-MDATRON.md (positioning + CLI surface + onboarding sections);
  CHANGELOG.md. Crosslink baseline: crosslink/src/main.rs:80-148 (Init verb +
  flag set); crosslink/src/commands/init/mod.rs:97-214 (InitUI banner /
  step_start / step_ok / step_skip / warn / success prose surface);
  crosslink CLAUDE.md (CLI vocabulary); crosslink README.md (onboarding shape).
lens: >-
  DR cold-read with sanity-check baseline. DR dims weighted: cold-context
  discoverability (1), three-audience effectiveness (4), naming-discipline
  cold-read (7). Sanity-check rubber-ducks "does an adopter who already has
  crosslink installed know which `init` they ran when they see the output?"
  DR-lens bias toward fewer-load-bearing-terms acknowledged in the
  sycophancy_compensation block + pressure-tested against the under-documentation
  risk operator named.
source: operator-directive
session_note: >-
  Cold single-domain single-session. Read the four required priming docs +
  the subject + baseline files end-to-end; no prior-cycle memory of mdatron
  or crosslink. Opinion-only — no code or doc edits proposed for direct merge.
  Composes with the sibling 2026-06-02 DR init-drift entry on the substrate
  side (crosslink/vsdd init flag-name hallucination): same defect-class
  surfaces here as an absence — mdatron has no README, has no `init`, and is
  about to import crosslink's flag-vocabulary noise unless DR flags now.
model: claude-opus-4-7
execution_method: >-
  inline main session (single-agent single-domain DR with sanity-check baseline;
  no worktree isolation; under-800-word opinion shape)
sycophancy_compensation: >-
  DR-lens bias: "mdatron's leanness is the win — leave it." Easy. But the
  operator's question is whether under-documentation is creating absorbability
  friction NOW, before `init` ships + before the first non-VSDD adopter arrives.
  The pressure-test below names the three load-bearing gaps that cannot defer
  past v0.1's first external-facing surface.
filename_note: >-
  Suffixed `-mdatron-consistency` to disambiguate from the sibling
  2026-06-02 init-drift DR entry. Single-topic; single domain.
---

# Documentation Reviewer — mdatron operator surface consistency

**Subject:** mdatron's verb + flag + error + onboarding surface, read cold, against crosslink as substrate baseline, with eye on "absorbability" — what an adopter (especially one who has crosslink installed already) can reconstruct from the surface alone.

## What a cold installer sees today

Cold operator runs `cargo install mdatron`, then `mdatron --help`. They get: `verify`, `explain`, plus the after_help line "Descended from Schematron (ISO/IEC 19757-3). Not related to the TRON blockchain." The TRON disambiguation lands — TW-F3 from the 2026-06-01 review is honored at the binary surface. `mdatron verify --help` reads cleanly: `--project-root`, `--schemas`, `--patterns`, `--files`. Vocabulary count: 2 verbs + 4 flags. Compared to crosslink's 30+ verbs in CLAUDE.md, mdatron is dramatically discoverable on first contact. **This is a real win the design should defend.**

But there is no README. `cargo install mdatron` ships them to a `--help` that names "Schematron" and "TRON" but does not answer "what is this for? how do I get my first finding?" DESIGN-MDATRON.md is 1071 lines aimed at suite-developer + co-evolution audiences, not adopter onboarding. The CHANGELOG line "mdatron verify ... reports per-file count + rustc-style diagnostics on failure" is the closest thing to a getting-started sentence. Cold-read: an adopter at v0.1 has no quickstart surface. **DR finding 1 (DR-F1): no README is a Resolved-only-via-write deficit; the surface is otherwise tight enough that a 60-line README closes most of the absorbability gap.**

## Error coherence — codes vs context

mdatron's diagnostic surface (`Finding.format_tty`) reads `error[MDATRON-E0001]: <message>\n  --> <file>:<line>\n   = help: ...\n   = explain: mdatron explain MDATRON-E0001`. Cold-read: this is rustc convention executed correctly. The `--explain CODE` parallel is recognizable from `rustc --explain E0277` muscle memory. Code namespace `MDATRON-E0001 / E0002 / E0070 / E0080` is sparse + reads as a catalog stub, not a finished catalog. **DR finding 2 (DR-F2): `mdatron explain` is a stub returning "extended docs not yet implemented at v0.1.0" — the `= explain:` line every finding emits points to a dead surface.** The Finding type promises a discoverable explain page; the binary's response is to say "not yet." For a CI investigator the loop is: `error[MDATRON-E0001]` → `mdatron explain MDATRON-E0001` → "not yet implemented" → grep the binary's source. Three-audience failure: adopter sees a dead-end, CI investigator gives up.

Crosslink's `anyhow::Context::with_context` produces "Failed to write repo-id: ..." style chains. Side-by-side: crosslink's error reads as "what went wrong, immediately, in operator-language"; mdatron's reads as "what classification, with a promised explain page." mdatron wins for (b) CI triage when codes are stable + greppable; crosslink wins for (a) developer-in-terminal who just needs to fix it. mdatron's current state is half-wins-by-design — it commits to the codes-with-explain shape but ships the codes-without-explain-content surface. **DR-F2 routes to TW pair: either ship even minimal explain stubs (one paragraph per code) or remove the `= explain:` line from format_tty until they exist.**

## Verb collision with crosslink + the upcoming `init`

`mdatron init` is coming-soon per operator directive. Crosslink has `init`. Adopter with both installed: `crosslink init` vs `mdatron init` is unambiguous at the shell. But the cold-read failure surface is shared prose — review-logs, design docs, CHANGELOG entries that say "run init" without binary qualifier. Crosslink CHANGELOG already shows the hazard: "`crosslink init --update`" is qualified; "Init: respect `--dry-run`" is not. **DR finding 3 (DR-F3): mdatron's docs should establish the prose convention NOW (always `mdatron init`, never bare `init`) before any review-log entry blurs it.** Cheap fix; defended at v0.1 instead of retroactively.

## Naming load-bearing audit

Three competing terms surface for the same concept: `Finding` (diagnostic.rs), `Diagnostic` (doc references), `Error` (error.rs internal). Cold-read of mdatron-cli/src/main.rs:125-137: the printer function is `print_finding`; the type is `Finding`; the rendered output starts with `error[...]`. The label "error" comes from `Severity::Error.label()`, but the *thing emitted* is a Finding. A CI investigator grepping logs for "error" finds the rendered output; grepping the source for "Diagnostic" finds the module name; grepping for "Finding" finds the type. **DR-F4: pick one load-bearing term in operator-facing prose. Recommendation: `Finding` for the type/concept (it's already the source-of-truth name); reserve `diagnostic` for the module + the convention name ("rustc-shaped diagnostic format"); never use `Diagnostic` as a noun in operator surfaces.**

`schema_class` dispatch: cold-read of verify.rs:253-278 — the frontmatter field name appears literally in the operator's YAML, so it is load-bearing. For a non-VSDD adopter this term arrives unexplained; DESIGN-MDATRON.md uses it as if it were obvious. **DR-F5: the future README + first explain page must one-sentence the term; the cost of leaving it implicit is each new adopter reinventing the mental model.**

## Headline recommendation

mdatron's lean v0.1 surface is the asset. **The three absorbability cracks that cannot defer past v0.1 are (a) ship a minimal README, (b) make `mdatron explain` non-trivial OR strip the `= explain:` promise from format_tty until it is, and (c) lock the `Finding`/`mdatron init` prose-naming conventions before `init` lands and review-logs ossify them as ambiguous.**

**Classification:** Deferred-pending-TW + SO. DR-F1 + DR-F2 route TW (prose authoring); DR-F3 + DR-F4 + DR-F5 are convention declarations that route SO. None require code changes; all five compose with the sibling 2026-06-02 init-drift entry's spec-vs-impl-coherence theme.

**MVR signal:** N/A — opinion only.
