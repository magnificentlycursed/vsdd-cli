---
title: "Slice 2 — Composition, generated context, and static price"
tags: ["design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-01
updated: 2026-08-01
---


## Design Specification

### Summary

Slice 2 builds the toolkit's composition leg: a pure **composition function**
that maps a project's declared surfaces (or, at fix scale, a finding issue's
labels), the composition scope, and the review config to the active domain set
and dispatch shape; a **review-config loader** for `.vsdd/config.yaml` with
layered loading and a vsdd **config SHAPE/schema validation-at-read** leg; a
**context generator** that emits each dispatch's domain/supplement/phase sections
with byte-matched emission and a stamped token count; a **token-budget gate**
(warn at session start, block in CI); and a pure **pricing function** that emits
a priced bill of materials from the composition and the stamped counts. Its
guardrail is the composition act — the validation-at-read leg plus the crosslink
session-start hook plus the token-budget gate — not a git event. It closes
**Composition purity** and feeds Slice 7's **Cost queries** as a dependency.

This is a phase-1a design authored the vsdd way (a `.design/` doc via the design
affordance) for crosslink issue #839, grounded fresh in the ratified contract
(`.design/agent-first-vsdd-toolkit.md`) and the current code — no captured seed
exists. **Cross-repo boundary (stated up front):** the config-integrity RULES
(pair co-activation, pair separation, validator-differs-from-owner) that *reject
an integrity-violating config* are **mdatron's** deliverables; Slice 2 depends on
and integrates them (flagged like Slice 5's mdatron dependency), and they gate on
that family's release. The **vsdd** side of the guardrail is the bootstrap
config-SHAPE validation-at-read leg — shape and schema, not the integrity rules —
which stays live at landing meanwhile.

### Requirements

- REQ-1: **The composition function is pure and deterministic over its declared inputs.** A new pure function in `vsdd-core` maps `(surfaces | fix-scale finding labels, composition scope, review config) → (active domain set, dispatch shape)`. Its second input, the composition scope, is the versioned data already present in `templates/registry/composition-scope-and-actions.md` (`scope_members`: the ten whitepaper phases plus the `fix-lane` member, loaded as `registry::sets::CompositionScopeAndActions`). One function serves both scales (contract, Deterministic composition): at spec scale the input is DESIGN.md surfaces under a phase member; at fix scale the finding issue's labels stand in the surfaces position under the `fix-lane` scope value. The function takes **no token counts** and cannot price (phase-1b amendment). Enumerated properties it is property-tested against: **determinism** (identical inputs → identical output) and **domain-narrowing** (removing domains from the config never adds domains to the output — the monotonicity claim covers the active-domain field only). (Contract: The composition function §208; Deterministic composition §89; Slice 2 §273.)
- REQ-2: **The composition function honors the three pair/validator properties in its output.** For a config the mdatron integrity family accepts, the function's computed active set satisfies: **pair co-activation** (activating an author-side domain also activates its cold-reader/validator domain — e.g. activating `software-engineer` co-activates its `validator_pair` `solution-architect`), **pair separation** (a domain and its validator are marked for different agents in the dispatch shape), and **validator-differs-from-owner** (every active domain's validator is a distinct domain). These per-domain properties hold at any active-set size (contract §92: "pair co-activation and validator-differs-from-owner hold at any set size"). The pairing data is read from the domain-prompt frontmatter (`validator_pair`, `tier`, `activation_criteria`) that already ships in the 18 `.claude/commands/vsdd-domain-*.md` files. **Boundary:** these are the function's *output* properties (vsdd property-tests them); the *rejection of a config that violates them* is mdatron's config-integrity check (REQ-9, cross-repo). (Contract §92, §208; Composition purity §194.)
- REQ-3: **The review config loads from `.vsdd/config.yaml` with layered loading.** A new loader reads the review config with crosslink's layered model — shipped presets, then project config, then local overrides — merging into one effective config. The preset layer is seeded from the `presets` block already in `templates/registry/economics-data.md` (`thorough`, `standard`, `minimal`, each carrying `active_domains`, `round_budget`, `stop_sensitivity`, `mutation_floor_declared`); `thorough` is this project's declared preset. The effective config carries the contract's field set: **active domains, round budget, stop sensitivity, model tiers and effort levels, mutation floor, cost bands**. This loader is net-new: `.vsdd/config.yaml` does not exist in this repo today (only `.vsdd/events/` is present), and the sole config.yaml *writer* is `init.rs` (line ~332-334), which writes just a `vsdd_version:` stub on init — no reader and no field beyond the version stub exist. The review config is the composition function's third input (REQ-1). (Contract: "The review config is a file" §91; Review config §23.)
- REQ-4: **The review config is SHAPE/schema-validated at read (vsdd's bootstrap leg).** The config is validated against a schema at read using the existing `schema_check::Schema` engine (draft 2020-12), on the same trust-boundary discipline the registry loader already applies: bounded read (`bounded_read::read_bounded`), value-free diagnostics that name the file and the violated constraint but never echo the failing content, and a diagnostic — never a panic — on any read failure (malformed, absent, permission/IO, oversize). This is "the grant Trust boundaries actually makes": vsdd validates its own config SHAPE at read during bootstrap, catching schema violations at landing, until mdatron's conformance families own the check. It validates **shape, not the integrity rules** (those are REQ-9, mdatron's). The config is a named adopter-edited file in the contract's Trust-boundaries enumeration (`.vsdd/config.yaml`, §230). (Contract: Slice 2 §273; Bootstrap validation §350; Trust boundaries §230.)
- REQ-5: **The context generator emits each dispatch's sections, byte-matched to their sources.** A new generator builds agent context (session skill, domain prompts, and supplement sections) from source data, emitting the sections that match the composed dispatch so each reviewer or builder receives exactly its slice. Sources are the artifacts already bundled in `vsdd-core/src/lib.rs::artifacts` — `DOMAIN_PROMPTS` (18), `PHASE_PRIMERS` (10), and `SUPPLEMENTS` (14), the last authored as per-domain `## <Domain> extensions` sections (verified in `supplements/rust.md` and its siblings). Domain-to- supplement mappings derive from the section structure at generation time (each supplement's `domains_in_scope` frontmatter and its `## <Domain> extensions` headings); hand-declared duplicate mappings are removed; intentionally-absent sections are declared, distinguishing not-applicable from not-yet-authored. Each emitted artifact **byte-matches its source's emission** — a mutated source byte fails the byte-match. (Contract: Generated context §179; Composition purity §194; section definition §331.)
- REQ-6: **Every generated context artifact carries a stamped token count at build time.** The generator stamps each emitted artifact with its token count, which is the pricing function's input (REQ-8) and the token-budget gate's subject (REQ-7). (Contract: Static price §145.)
- REQ-7: **The token-budget gate is a warn/block surface over per-class budgets.** The gate reads the per-artifact-class budgets already declared in `economics-data.md` (`token_budgets`: `session-skill` 5000, `domain-prompt` 3000, `phase-primer` 2500, `supplement-section` 2000, `always-on-core` 1500) and compares each generated artifact's stamped count (REQ-6) against its class budget. An over-budget artifact **fails** the gate. The surface is warn/block: advisory (warn) at the session-start hook, blocking (fail) at the CI gate — the CI *wiring* is Slice 7's, the gate *behavior* is Slice 2's. (Contract: Slice 2 §273; Composition purity §194 "the token budget gate fails an over-budget generated artifact"; Slice 7 §278.)
- REQ-8: **The pricing function is pure and emits a priced bill of materials.** A pure function maps `(generated artifacts with stamped token counts, composition) → bill of materials (total, per artifact, cacheable portion)`, emitted at session start before the spend. Enumerated properties it is property-tested against: **total equals the sum of the parts**, **cacheable portion bounded by the total**, and **determinism**. The function takes no runtime spend — a preset's cost band prices the static context bill only; effort's runtime thinking spend is the ledger's to report (Slice 7). This behavior feeds Slice 7's close of **Cost queries** as a dependency, not a criterion here. (Contract: Static price §145; The pricing function §209; Slice 7 §278.)
- REQ-9: **The config-integrity RULES are a flagged cross-repo dependency, not a vsdd REQ.** Rejecting an integrity-violating config — a config that violates pair co-activation, pair separation, or has a validator the same as its owner — is **mdatron's** config-integrity family, invoked behind the hooks, and it gates on the mdatron release that ships it (cross-repo cadence, §288: mdatron's actual release line, per-family). Slice 2 integrates that check into its guardrail; it does **not** re-implement the rules as vsdd code. Composition purity's seeded integrity-violating-config falsifiers ride this dependency; vsdd's config-SHAPE leg (REQ-4) stays live at landing meanwhile. (Contract: Slice 2 §273; Config integrity §92; cross-repo cadence §288 — flagged the way Slice 5's mdatron conformance families are.)
- REQ-10: **The guardrail is the composition act, wired as three grades.** Not a git event: (a) the vsdd bootstrap config-SHAPE validation-at-read leg (REQ-4), catching schema violations at landing; (b) the crosslink **session-start hook**, a thin wrapper that invokes vsdd to compute the composition (REQ-1/2) and emit the priced bill (REQ-8) at session start; and (c) the token-budget gate (REQ-7) as a warn/block surface. The hook installs per crosslink conventions into `.claude/`/`.crosslink/`; its enforcement logic lives in the tool, not the script. (Contract: Slice 2 §273; Composition function §175; §248 hooks.)
- REQ-11: **The generated-context surface wires the shared terminal-output-safety cleaner as this slice's act.** Any string sourced outside the tool's compiled-in constants that reaches the agent-consumed generated context — the composition inputs (DESIGN.md surfaces, `.vsdd/config.yaml`-derived domain names, fix-scale finding labels) and any adopter-authored registry data folded into the emission — passes through `text::clean_for_terminal` / `text::clean_json_strings` before it reaches the surface. The byte-match of REQ-5 holds against the cleaned source emission; the bundled sources are the tool's own already-authored constants, so the cleaner binds at the external-sourced injection points, consistent with the registry loader's `PostLoad` and the state read's source-boundary cleaning. (Contract: Terminal output safety §174, per-layer inheritance §266 — "Slice 2's generated context".)

### Acceptance Criteria

- [ ] AC-1: A property test over the composition function asserts **determinism** (identical `(surfaces, scope, config)` inputs yield an identical active domain set and dispatch shape across runs) and **domain-narrowing** (for any config `C` and any domain-removal `C' ⊆ C`, the active-domain set of `C'` is a subset of that of `C` — removing a domain never adds one). (REQ-1)
- [ ] AC-2: A property test asserts, for every config the integrity family accepts, that the computed active set honors **pair co-activation** (for each active author-side domain, its `validator_pair` domain is also active), **validator-differs-from-owner** (no active domain's `validator_pair` equals itself), and **pair separation** (the dispatch shape marks a domain and its validator for different agents); and that all three hold at active-set sizes of 1, the `minimal` preset's set, and the full 18-domain set. (REQ-2)
- [ ] AC-3: The **fix-scale composition fixture** — finding-issue labels in — is loaded under the `fix-lane` scope value and yields a deterministic active set out (e.g. a `security` label summons the security domain; a bare fix yields software-engineer plus its cold-reader validator), exercising the composition- scope domain's second member. (REQ-1, REQ-2)
- [ ] AC-4: A `.vsdd/config.yaml` layering test asserts that with no project config the `thorough` preset's field values are in force; a project config overrides named preset fields; a local-override layer overrides the project config; and the merged effective config exposes all six fields (active domains, round budget, stop sensitivity, model tiers/effort levels, mutation floor, cost bands). (REQ-3)
- [ ] AC-5: A shape-invalid `.vsdd/config.yaml` (wrong type, missing required field, unknown field) yields a value-free diagnostic that names the file and the violated constraint and does **not** echo the failing content, and the process does not panic; a malformed/absent/permission-denied/oversize config each yields its own diagnostic kind; a shape-valid config loads. (REQ-4)
- [ ] AC-6: Each generated context artifact **byte-matches** the canonical emission of its source sections; mutating one source byte (`.claude/commands/vsdd-domain-*.md`, `supplements/*.md`, or `.claude/commands/vsdd-phase-*.md`) makes the byte-match fail; the emitted set for a given dispatch contains exactly the sections matching that dispatch (a reviewer receives its domain slice and no other's); a hand-declared duplicate domain↔supplement mapping is absent from the derived mappings; and an intentionally-absent section is reported as declared-absent, not not-yet-authored. (REQ-5)
- [ ] AC-7: Every generated context artifact carries a non-zero stamped token count reproducible across two builds of the same source (the stamp is deterministic). (REQ-6)
- [ ] AC-8: An over-budget generated artifact **fails** the token-budget gate against its `economics-data.md` class budget; an in-budget artifact passes; the same over-budget condition surfaces as a warn at the session-start-hook surface and as a block (non-zero) at the gate surface. (REQ-7)
- [ ] AC-9: A property test over the pricing function asserts **total = sum of per-artifact parts**, **cacheable portion ≤ total**, and **determinism**; and the emitted bill's total matches the sum of the composed artifacts' stamped counts (REQ-6) for a fixture composition. (REQ-8)
- [ ] AC-10: The seeded integrity-violating configs (one per rule: pair co-activation, pair separation, validator-same-as-owner) **fail validation** when the mdatron config-integrity family is present; and, with that family absent, the same configs still pass vsdd's config-SHAPE leg (REQ-4) while the falsifier is recorded as riding the flagged cross-repo dependency — never silently reported as covered by vsdd. (REQ-9)
- [ ] AC-11: A guardrail block/pass test asserts that the session-start hook, run over a valid tree, computes the composition and emits the priced bill at exit 0; a shape-invalid config blocks at the validation-at-read leg; and an over-budget artifact blocks at the token-budget gate — each grade named (validation-at-read, session-start hook, token-budget gate). (REQ-10)
- [ ] AC-12: A hostile code point (a bidi override, a zero-width joiner, a tag- block character) placed in an external-sourced string that reaches the generated context — a DESIGN.md surface token, a config-declared domain name, or a fix-scale finding label — is stripped by `clean_for_terminal` before it reaches the surface, and the REQ-5 byte-match still holds against the cleaned source emission. (REQ-11)

### Architecture

**Where the new code lives.** Three net-new modules in `vsdd-core`, plus one home
decision for pricing (Q1):

- **Composition function** — a new `vsdd-core/src/composition/` module (peer of
  `answer/`, `state/`, `registry/`). Pure `fn` over its inputs, per the contract's
  vsdd-core placement (§248: "vsdd-core: … composition function with calibration
  data, context generator"). It consumes `registry::sets::CompositionScopeAndActions`
  (already loaded by `registry::load_set(repo_root, "composition-scope-and-actions")`,
  wired at `vsdd/src/main.rs` line ~112) for the scope domain, and the domain-
  prompt frontmatter (`domain_slug`, `tier`, `activation_criteria`,
  `validator_pair`) for the per-domain activation and pairing data. The 18 domain
  prompts are already bundled via `vsdd-core/src/lib.rs::artifacts::DOMAIN_PROMPTS`
  and validated by `.mdatron/schemas/domain-prompt.json`; the composition function
  reads their frontmatter as its domain metadata. Activation is three-mode,
  read from `activation_criteria`: `always-on-baseline` (the core tier: software-
  engineer, security, ux, quality-engineer, solution-owner, solution-architect —
  always active), surface-gated extended domains (active iff the matching surface
  is declared — e.g. `ui-surface` → accessibility+ux, `handles-user-data` →
  privacy, `network-exposed` → red-team+security, `ai-runtime-cost-relevant` →
  ai-engineer), and meta (`hook-triggered` sanity-check, `on-demand`
  vsdd-methodology). Pair co-activation follows `validator_pair`
  (software-engineer→solution-architect, security↔red-team, etc.).

- **Review-config loader** — a new `vsdd-core/src/config.rs` reading
  `.vsdd/config.yaml`. Layered loading is modeled on crosslink's config (shipped
  presets → project → local overrides); the shipped-preset layer is seeded from
  the `presets` block in `templates/registry/economics-data.md` (loaded as
  `registry::sets::EconomicsData::presets`). The validation-at-read leg (REQ-4)
  reuses `schema_check::Schema::{compile,validate}` and `bounded_read::read_bounded`
  and mirrors the value-free-diagnostic and no-panic discipline in
  `registry/mod.rs::load_set` (the `artifact_diagnostic` / `schema_pair_diagnostic`
  pattern). Today the only `.vsdd/config.yaml` writer is `init.rs` (line ~333,
  the `vsdd_version:` stub); the only existing config *reader* in the tree is
  `vsdd/src/status/multi.rs::read_repo_set_config`, which reads the unrelated
  statusline **repo-set** config (`~/.config/vsdd/statusline.yaml`) — not the
  review config. So the review-config reader is entirely net-new.

- **Context generator** — a new `vsdd-core/src/context/` module. It emits the
  session skill, the per-dispatch domain prompts, and the matching supplement
  `## <Domain> extensions` sections from the bundled `lib.rs::artifacts` sources,
  deriving domain↔supplement mappings from each supplement's `domains_in_scope`
  frontmatter and heading structure (the "section" unit of §331). Byte-matched
  emission (REQ-5) means the emitted section equals its source slice verbatim; the
  token stamp (REQ-6) is computed at emission. The shared cleaner
  (`vsdd-core/src/text.rs`: `clean_for_terminal`, `clean_json_strings`) wires in
  at the external-sourced injection points (REQ-11), the same primitive already
  used by `registry/sets.rs::PostLoad`, `state/read.rs`, and `snapshot/acquire.rs`.

- **Pricing function** — pure, over `(stamped artifacts, composition)`. The
  contract fixes its home as the cost crate (§209), but the cost crate is Slice
  7's deliverable (§278) and the workspace today is two crates (`vsdd`,
  `vsdd-core`; `Cargo.toml` members). Whether Slice 2 stands up the cost crate now
  or homes pricing provisionally in `vsdd-core` is Q1.

**The composition inputs — verified data provenance.** The composition function's
four data dependencies resolve to concrete, present artifacts: the **scope**
(`composition-scope-and-actions.md`, present, schema-paired), the **domain
metadata** (18 `.claude/commands/vsdd-domain-*.md` frontmatters, present,
schema-paired), the **presets / token budgets / cost bands / mutation floor**
(`economics-data.md`, present, schema-paired), and the **generator source
sections** (14 `supplements/*.md` + 10 `.claude/commands/vsdd-phase-*.md`,
present). The one composition input with **no reader and no closed vocabulary
today** is DESIGN.md **surfaces** (Q2): grep confirms no surfaces reader in
`vsdd-core/src` or `vsdd/src`, and the contract's example surface words (ui, user
data, locales, ai runtime, attack surface, §330) are spelled differently from the
domain prompts' `activation_criteria` tokens (`ui-surface`, `handles-user-data`,
`localized`, `ai-runtime-cost-relevant`, `network-exposed`).

**Relation to the state artifact (already built).** `vsdd-core/src/state/schema.rs`
already carries `ActiveComposition { scope, domains, mode, config_inputs_hash }`,
and `answer/derive.rs::derive_phase_answer` *echoes* it verbatim
(`state.active_composition.clone()`, with `state/read.rs` line ~153 noting the
"verbatim echo of the composition"). That is a **stored** composition with a hash
that makes a stale composition detectable (vsdd-cli #665/#749) — it is **not** the
composition function. Slice 2 supplies the function that *computes* what today is
only stored and echoed; `config_inputs_hash` is the natural join point (the hash
over the composition function's inputs).

**Boundary discipline reused.** The validation-at-read leg is not new machinery —
it is the registry loader's discipline (`schema_check.rs`, `bounded_read.rs`,
value-free diagnostics, `PostLoad` cleaning) applied to a new artifact. The
config's schema pair is net-new (`.mdatron/schemas/` today holds pairs for
composition-scope-and-actions and economics-data but **no** review-config /
config.yaml pair — Q3).

### Out of Scope

- **The config-integrity RULES (pair co-activation, pair separation, validator-differs-from-owner) as vsdd code** — these are mdatron's config- integrity family (REQ-9), a flagged cross-repo dependency integrated behind the hooks and gating on the mdatron release that ships it (§288). Slice 2 depends on and integrates them; it does not re-implement them. vsdd's leg is the config-SHAPE validation-at-read only (REQ-4).
- **The cost ledger, capture adapters, provenance, unit economics, efficiency advisories, and CI budget-gate wiring** — Slice 7 (The cost crate, §278). This slice ships only the **static price**: the pure pricing function and its priced bill of materials, and the token-budget gate *behavior* (Slice 7 wires it into CI). Cost queries is Slice 7's criterion; Slice 2 feeds it as a dependency.
- **`vsdd init` install of the generated members** — Slice 3 (Install, §274). The generated skills and domain prompts join the install payload "when Slice 2's generator has landed" (declared split); depositing them into an adopter is Slice 3's act, not this one.
- **Gate execution, the mutation floor gate, and the standing-suite delta** — Slice 4 (§275). The mutation floor is a *field* the review config carries (REQ-3); *enforcing* the mutation kill-ratio is Slice 4's gate.
- **The state artifact's stored `ActiveComposition` and its consistency/validation families** — already built (the engine); Slice 2 computes what is stored, but does not re-home the state field or own mdatron's state-consistency family.
- **DESIGN.md surfaces data authoring and the surface vocabulary** (Q2) — if the resolution routes surfaces to the phase-1c data-authoring package, that authoring is not this slice's build; Slice 2 consumes the resolved vocabulary.

