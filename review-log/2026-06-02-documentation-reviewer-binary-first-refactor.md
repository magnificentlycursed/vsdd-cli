---
schema_class: review-entry
schema_version: 1.0.0
review_number: 4
date: 2026-06-02
phase: phase-1a
scope: >-
  Proposal-shape DR cold-read of the binary-first / crosslink-cued refactor
  seam between vsdd and mdatron. Subject is the operator-facing prose surface
  the refactor will reshape: per-repo READMEs, the two `init` verbs and their
  managed-config files, error-code namespaces across the process boundary,
  `mdatron explain` flow, and the four DESIGN docs whose mdatron-consumption
  passages currently assume library-link integration (DESIGN-MDATRON.md,
  DESIGN-METHODOLOGY.md, DESIGN-SCHEMA.md, DESIGN-VERIFICATION.md,
  DESIGN-OBSERVABILITY.md). Composes with the two prior 2026-06-02 DR entries
  (init-drift and mdatron-consistency) as the prose-side refactor scaffolding.
lens: >-
  DR cold-read with sanity-check baseline. DR dims weighted: cold-context
  discoverability (1), cross-reference resolution (2), three-audience
  effectiveness (4), naming-discipline cold-read (7). Sanity-check
  rubber-ducks "does the binary-first refactor cut vocabulary or pile on?"
  — the DR-lens bias toward fewer-load-bearing-terms is pressure-tested
  against the real risk that process-boundary contracts introduce new
  terms (JSON envelope, exit-code mapping) the operator must learn.
source: operator-directive
session_note: >-
  Cold single-domain single-session draft. Read the priming docs +
  the two sibling DR cold-sessions; no code or doc edits proposed for
  direct merge. Design-drafting only. Output is a Documentation Reviewer
  proposal for the prose-surface implications of the binary-first refactor
  the operator has settled — not a critique of whether to do the refactor,
  but a cold-read of what the refactor's prose contract owes the
  three audiences.
model: claude-opus-4-7
execution_method: >-
  inline main session (single-agent single-domain DR with sanity-check
  baseline; no worktree isolation; under-1500-word opinion shape)
sycophancy_compensation: >-
  DR-lens bias: "binary-first cuts a dependency, so it cuts vocabulary —
  win." Easy. But process boundaries introduce their own vocabulary
  (envelope, exit-code, propagation, shell-out). Pressure-test below names
  the load-bearing additions the refactor cannot avoid + the ones it can.
  Second bias to guard: "two READMEs is more prose than one" — pressure-tested
  against whether splitting actually reduces the surface each audience reads.
filename_note: >-
  Suffixed `-binary-first-refactor` to disambiguate from the two prior
  2026-06-02 DR entries (init-drift, mdatron-consistency). This entry
  composes both: init-drift's flag-vocabulary discipline + mdatron-consistency's
  absorbability findings both inform the binary-first prose contract.
---

# Documentation Reviewer — binary-first refactor prose surface

**Subject:** the prose contract the binary-first / crosslink-cued refactor owes day-one adopters across three audiences. DR-lens cold-read of the seam between vsdd and mdatron under the settled directive (both binaries, vsdd shells out to `mdatron verify`, both have their own `init`).

## 1. Operator-facing surface — the day-one cold read

**Install-order story.** Refactor must pick one of three: (i) mdatron-only is supported; vsdd auto-installs mdatron as a side-effect dependency; (ii) operator installs both; (iii) install order is symmetric — `vsdd init` detects missing mdatron and tells you what to run. **DR-F1: the README must name the chosen story in its first 20 lines.** Cold operator who installs vsdd first + runs `vsdd init` + sees "mdatron not found on PATH" without onboarding prose has no path forward. Recommendation: option (iii) is the only one that serves the (b) non-VSDD-mdatron-only adopter — the other two implicitly make mdatron a vsdd-subordinate.

**Init outputs as substrate-signal.** `vsdd init` touches `.vsdd/`. `mdatron init` touches `.mdatron/`. Both should print the substrate path they touched in their success line (crosslink's InitUI banner discipline). Cold operator who runs both in sequence must be able to read backwards from the terminal which directory came from which binary. **DR-F2: success-line prose must lead with the literal directory name + the binary name, e.g., `mdatron init: wrote .mdatron/config.yaml` / `vsdd init: wrote .vsdd/config.yaml`.** Symmetry is the discoverability win.

**Shell-out error surface.** When `vsdd verify` shells out to `mdatron verify` and mdatron exits nonzero, the operator sees the mdatron-rendered diagnostics (rustc-shaped, MDATRON-Exxxx codes) + a vsdd wrapper line. **DR-F3: vsdd's wrapper line must name the boundary explicitly — `vsdd: mdatron verify exited 1 with N findings (see above)` — never paraphrase or swallow mdatron's output.** The investigator's path is `mdatron explain MDATRON-Exxxx` directly; vsdd does not intercept the code.

**Cross-cutting `--explain` discipline.** Both binaries should ship `--explain CODE` with parallel semantics. **DR-F4: `mdatron explain MDATRON-Exxxx` and `vsdd explain VSDD-Exxxx` are the two valid forms.** `vsdd explain MDATRON-Exxxx` should print a one-liner pointer: "MDATRON-Exxxx is an mdatron diagnostic; run `mdatron explain MDATRON-Exxxx`." Don't proxy — the proxy is the discoverability anti-pattern (operator no longer knows which binary owns the catalog).

## 2. Vocabulary discipline across two binaries

**`init` collision.** Bare `init` in any prose surface (review-logs, design docs, CHANGELOG) is ambiguous. Prior DR-F3 (2026-06-02 mdatron-consistency) flagged this preemptively; the binary-first refactor makes it acute. **DR-F5: prose convention is "always qualify the binary." Adopt a CONTRIBUTING.md-level rule: bare `init` is a doc bug.** Cheap to defend now; expensive after review-logs ossify ambiguous references.

**Error-code namespaces.** MDATRON-Exxxx + VSDD-Exxxx stay strictly separate — no nesting, no overlap, no rewriting. vsdd surfaces mdatron's codes verbatim. **DR-F6: docs must name "vsdd does not own MDATRON-Exxxx codes" as a contract.** The investigator who greps `MDATRON-E0001` in vsdd's source should find no catalog hits — only the shell-out plumbing.

**Glossary ownership.** `Finding`, `Diagnostic`, `Schema`, `Pattern`, `schema_class` — mdatron owns the canonical definitions; vsdd cites. **DR-F7: vsdd's DESIGN docs must use `mdatron:Finding` notation (or equivalent) when referring to mdatron-domain terms, OR explicitly link to mdatron's glossary.** Currently DESIGN-METHODOLOGY.md uses `Finding` as if vsdd owned it. The library-link assumption made this invisible; the binary-first refactor exposes it.

**Crosslink parallel.** Crosslink's load-bearing terms (session / issue / kickoff / swarm) are methodology-substrate vocabulary; the mdatron+vsdd parallel is **verification-substrate vocabulary**: mdatron owns `Finding / Severity / Schema / Pattern / schema_class`; vsdd owns `phase / domain / review-entry / validator-pair`. These are non-overlapping by design — the refactor should make the non-overlap discoverable rather than implicit.

## 3. README + onboarding

**mdatron README** (closes DR-F1 from the 2026-06-02 mdatron-consistency review): 60-80 lines covering (i) what it is — one sentence; (ii) "Descended from Schematron, not the blockchain" disambiguation; (iii) `cargo install` + `mdatron init` + a 3-line `.mdatron/schemas/example.yaml` + `mdatron verify` walkthrough; (iv) link to DESIGN-MDATRON.md for spec; (v) one-line link to vsdd ("VSDD is a methodology layer that uses mdatron for verification — see vsdd-cli"). **The vsdd link is a sidebar, not the lede.** Audience (b) must not feel they're reading vsdd's README-by-proxy.

**vsdd README** (currently 78KB). Under binary-first, the mdatron-internals sections should shrink to a "Requires mdatron ≥ X.Y; vsdd shells out to `mdatron verify`. See mdatron-cli for the verification substrate." paragraph. **DR-F8: estimate 78KB → ~50KB; the deletion is the schema/pattern/verify-internals prose that now lives in mdatron's docs.** Reorganization alone misses the win — actual cuts are the value.

**First-five-minutes story.**
- (a) Adopter through vsdd: `cargo install vsdd mdatron` → `vsdd init` → run a domain swarm → see findings. mdatron is named once in the install line; its role is named once in vsdd's README; the operator never opens mdatron's docs unless verify fails.
- (b) mdatron-only adopter: `cargo install mdatron` → `mdatron init` → write a schema → `mdatron verify`. vsdd is never named in the happy path; mdatron is standalone.

## 4. `mdatron explain` across the boundary

Prior DR-F2 (2026-06-02 mdatron-consistency) flagged the dead `= explain:` line. Binary-first refactor sharpens the contract: **the explain-flow stays inside mdatron's process — vsdd never reads, paraphrases, or intercepts.** Format_tty emits `= explain: mdatron explain MDATRON-E0001`; the operator runs that command directly; mdatron renders the catalog page. **DR-F9: this is a cross-process contract that needs naming in DESIGN-OBSERVABILITY.md — "diagnostic catalogs are binary-local; the producing binary owns the explain page."** Closes a class of future drift where vsdd would otherwise be tempted to cache or proxy mdatron's catalog.

## 5. Init + config prose

**`.mdatron/config.yaml` audience** is the mdatron-only adopter + the vsdd adopter peeking under the hood. Prose must read standalone — no "see vsdd docs for context." **`.vsdd/config.yaml` audience** is the vsdd adopter; may reference mdatron config by relative path.

**Managed-section markers.** Crosslink's gitignore discipline uses `# BEGIN crosslink-managed` / `# END crosslink-managed`. **DR-F10: both `.mdatron/config.yaml` and `.vsdd/config.yaml` need parallel markers — `# BEGIN mdatron-managed` / `# BEGIN vsdd-managed` — so each `init` can re-deploy its section without trampling operator edits or the other binary's section.** The drift-detection error from the 2026-06-02 init-drift review applies symmetrically to both binaries.

**"What `vsdd init` does vs what `mdatron init` does"** belongs in a side-by-side table in each README's onboarding section. Cold-reader catches the symmetry; library-link prose hid it.

## 6. Cross-doc consistency

- **DESIGN-MDATRON.md**: needs a new "Standalone binary contract" section + remove any library-API exposure prose. Audience: mdatron-only adopter + tool author. ~1071 lines → modest growth (~100 lines), not shrink.
- **DESIGN-METHODOLOGY.md**: the mdatron-consumption sections (currently library-link assumed) need rewrite as shell-out contract. Specifically: section on verification invocation must name `mdatron verify` as a subprocess + name the exit-code mapping. **Estimated rewrite: 200-400 lines.**
- **DESIGN-SCHEMA.md**: vsdd's references to schema mechanics should redirect to mdatron's DESIGN-SCHEMA equivalent (if mdatron has one) or to DESIGN-MDATRON.md. **DR-F11: avoid duplicating schema-mechanics prose in two repos — pick one canonical home (mdatron) and cite from vsdd.**
- **DESIGN-VERIFICATION.md**: invocation contract rewrite; exit-code semantics; envelope format if any structured output is consumed by vsdd.
- **DESIGN-OBSERVABILITY.md**: cross-process diagnostic contract (per DR-F9); how vsdd surfaces mdatron's findings in its own observability surfaces.

## 7. Three-audience effectiveness

- **(a) VSDD user, never touches mdatron directly:** layered surface is acceptable IF `vsdd verify` failures name `mdatron verify` in the wrapper line (DR-F3). Otherwise mdatron's role is obscured — operator can't grow into it when ready.
- **(b) mdatron-only adopter:** README + DESIGN-MDATRON.md must read standalone. The link to vsdd is sidebar, not lede. Currently fails at v0.1 — README absent.
- **(c) Tool author composing:** the binary-first contract is the integration discoverability win. `mdatron verify` as a stable subprocess with documented exit codes + diagnostic format is far more composable than a Rust library API. **DR-F12: DESIGN-MDATRON.md should add a "Composing mdatron" section naming the subprocess contract — exit codes, output format on stdout vs stderr, MDATRON-Exxxx code stability guarantee.**

## 8. Disposition re-clustering

Prior items that this refactor's prose work absorbs: prior DR-F1 (no README) → resolved by DR-F1 here; prior DR-F2 (dead explain) → resolved by DR-F4 + DR-F9 here; prior DR-F3 (init prose convention) → resolved by DR-F5 here; prior DR-F4 (Finding/Diagnostic term-pick) → resolved by DR-F7 here; prior DR-F5 (`schema_class` first-use prose) → routes to mdatron README onboarding (DR-F1).

Pure-code items not touched by this prose proposal: mdatron's verify-error variant set, DSL eval semantics, init-manifest hashing — all code-side concerns the refactor inherits unchanged.

## Bias pressure-test result

DR-lens prediction: binary-first cuts vocabulary. **Actual:** binary-first *adds* one term (subprocess contract / shell-out boundary) but *cuts* multiple (no shared Rust types in vsdd docs, no library-API prose in DESIGN-MDATRON.md, no vsdd-internal references to mdatron-domain glossary). **Net: vocabulary shrinks for the (a) and (b) audiences; grows by one term for (c).** Acceptable trade — (c) is the audience for whom the subprocess contract is the value.

**Classification:** Deferred-pending-SO + TW. 12 findings; all prose-side; all compose with the two sibling 2026-06-02 DR entries. None block the refactor; all should land alongside it to avoid post-refactor prose-drift.

**MVR signal:** N/A — design-drafting only.
