---
schema_class: review-entry
schema_version: 1.0.0
review_number: 4
date: 2026-06-02
phase: phase-1a
subject_project: vsdd-cli + mdatron (paired)
scope: >-
  Solution Owner disposition on the binary-first crosslink-cued refactor proposed
  under operator-directive: both mdatron and vsdd consumed AS BINARIES; vsdd shells
  out to `mdatron verify` (no library API call); structure follows crosslink cues
  (binary-first, anyhow, src/commands/, include_str!); each tool ships its own
  init. Re-clusters the 20+ in-flight dispositions accumulated this session
  (per the SA brief at review-log/2026-06-02-solution-architect-mdatron-consistency.md
  plus prior-cycle dispositions) against the new refactor surface. Scopes the
  refactor itself for adopter realism + LoC churn + sequencing risk. Composes
  with the same-date `solution-owner-mdatron-consistency` disposition (review #3,
  the cuts named there bind here) and the `solution-owner-init-drift` disposition
  (vsdd init scope).
lens: >-
  Solution Owner primary (scope-discipline + spec-contract authority +
  adopter-shape reasoning + Raise-to-SO routing integrity). Sanity Check baseline
  (cross-finding coherence with the same-day SO dispositions + the SA brief +
  the prior-cycle init-drift cluster; rubber-duck: does the binary-first payoff
  justify the churn for an adopter base that may not yet exist?). Phase 1a
  framing — re-clustering + sequencing ahead of any structural change.
source: operator-directive
session_note: >-
  Cold-session single-domain (SO + Sanity Check baseline). The operator-directive
  settled the binary-first shape before this session; the SO task here is NOT
  to revisit that directive but to scope-discipline its execution. Sycophancy
  guard: SO-lens biases toward "do nothing major, ship v0.1 with current
  structure" — pressure-tested below by naming what specifically breaks if the
  refactor is deferred (the answer: less than the refactor proponents believe,
  but not nothing).
model: claude-opus-4-7
execution_method: >-
  Cold-session sub-agent; SO + Sanity Check prompts + review-entry schema loaded
  fresh; SA brief read for the 20+ disposition list; same-day SO entries (review
  #3 mdatron-consistency, prior init-drift) read for binding-cuts. LoC anchors:
  mdatron-core ~3000 LoC (incl. DSL ~2900), mdatron-cli 147 LoC, vsdd-core ~700
  LoC src + ~660 LoC tests, vsdd-cli 346 LoC src.
sycophancy_compensation: >-
  Operator pre-named the bias: "the cheap path is do nothing major; pressure-test
  whether the refactor's payoff justifies the churn for an adopter base that may
  not yet exist." Adopted below — the disposition splits: vsdd-core's
  library-to-binary refactor (drop mdatron-core dep, subprocess to `mdatron
  verify`) IS the v0.1-blocking work because it ratifies the binary boundary
  the operator directive declares; mdatron's workspace consolidation is NOT
  v0.1-blocking and may be NEVER (the library is the public contract per review
  #3, so consolidating it away would invert the published-crate-dep coordination
  story). The pressure cuts both ways: rejecting the full mirror-crosslink
  refactor AND rejecting the do-nothing path.
filename_note: >-
  Filed `<date>-solution-owner-binary-first-refactor.md`; fourth same-date SO
  entry, suffix names scope to disambiguate from review #1 (Phase 3 vsdd),
  review #2 (init-drift), review #3 (mdatron-consistency).
supplements_loaded: []
---

# Solution Owner Disposition — Binary-first refactor scope discipline

**Phase 1a re-clustering + sequencing.** No code modification. Composes with same-day reviews #2 (init-drift) and #3 (mdatron-consistency).

## Headline

**The refactor splits in two. The vsdd-side half (drop `mdatron-core` lib-dep, shell out to `mdatron verify`) ratifies the operator-directive boundary and IS v0.1-blocking. The mdatron-side half (workspace consolidation; `src/commands/` mirror) is NOT v0.1-blocking and partly anti-thematic — `mdatron-core` IS the public contract per review #3, and collapsing the workspace would invert the published-crate-dep coordination story. Ship the seam, not the symmetry. Carryover dispositions (#1, #3a, #6, #8, #20) re-cluster into "fix before refactor" (reserved-code spec drift only), "absorb into refactor" (audit-trail emission + templates ownership), and "after refactor" (everything else). v0.1.0 lands AFTER phase 3 below, not after phase 7.**

## 1. Re-cluster of in-flight dispositions

Three buckets keyed to "what does the refactor change about this item":

**v0.1-blocking (must ship at v0.1.0):**
- **vsdd-core → vsdd subprocess seam** (the refactor itself, vsdd-side). Without it, the operator directive is paper-only.
- **#1, #20 — Reserved-code spec drift**. Already a real spec violation per review #3 (engine-range codes leaking from `main.rs:74,141`). Fix BEFORE the refactor — the refactor moves these surfaces and the fix would otherwise drift unattributed.
- **vsdd init refuse-on-drift** (per prior `solution-owner-init-drift`). Already cut to minimum.
- **mdatron v0.1.0 → crates.io publish** (per review #3). The subprocess seam works against `cargo install`'d binary; publishing is the precondition.

**v0.1.x (named-and-deferred, NOT silently dropped):**
- **#6 — Audit-trail tightening** (events.jsonl emission shape). The refactor touches the emission path; tightening rides WITH the refactor but the spec amendments land at v0.1.1.
- **#3a — Should-fire fixtures**. The subprocess seam ENABLES better fixtures (you can fixture `mdatron verify` stdout/exit-code without library-mock plumbing). Land v0.1.1.
- **CI install-command divergence** (M4 F2 per review #3). Already v0.1.1-bound.
- **`--format sarif|json|compact`** (per review #3). v0.1.2.

**v0.2+ (deferred-with-trigger):**
- **#8 — Templates deferral**. Under the new directive, templates are mdatron's job IF they're methodology-neutral (catalog scaffolds), vsdd's job IF they're VSDD-doctrine-shaped (DESIGN.md skeleton). Defer to v0.2 with the trigger: second adopter materializes who wants either flavor without the other.
- **mdatron `src/commands/` layout** (the symmetry half of the refactor). v0.2, IF a third subcommand lands. At 2 verbs the directory split is cargo-cult.
- **mdatron workspace consolidation**. NEVER on current evidence — mdatron-core is the publishable public contract. Re-trigger requires the contract claim itself to dissolve.
- **`.mdatron/config.yaml`** with formal schema (per the SA brief's load-bearing finding). v0.2.

**Adopter-load if all v0.1-blocking ships together vs spread:**
The v0.1-blocking set above is FOUR items, not twenty. Spreading them across v0.1.0 → v0.1.x is the disciplined cut. Adopter (vsdd-cli itself, transitively methodology users) cares about: (a) `mdatron verify` works under subprocess invocation; (b) `vsdd init` doesn't break; (c) `cargo install mdatron` succeeds. Three observable surfaces. The remaining 16 dispositions are tooling-grade polish — adopter-invisible at v0.1.

## 2. Scope realism for the refactor itself

**LoC churn estimate (additions + deletions, not net):**

- **vsdd-core library-to-binary refactor**: ~400 LoC churn. Current vsdd-core is 99 lib.rs + 295 init.rs + 219 preflight.rs ≈ 613 LoC src. The mdatron-core dep is consumed in preflight.rs's verification surface — replacing with `std::process::Command::new("mdatron").arg("verify")` + JSON-stdout parsing is ~100 LoC net add, ~50 LoC delete from the typed-call sites. Collapsing the workspace (vsdd-core → vsdd/src/commands/) is the larger chunk, ~250 LoC of move + module-path rewrites.
- **mdatron workspace consolidation**: **REJECTED** (see headline). 0 LoC.
- **mdatron `src/commands/` layout**: **DEFERRED** (rule of three not met at 2 verbs). 0 LoC at v0.1.
- **vsdd `src/commands/` layout**: ~150 LoC moves (init.rs → commands/init/mod.rs, verify-orchestration → commands/verify/mod.rs). Mostly mechanical.
- **init + config implementations**: mdatron init starter-scaffold-only per review #3 = ~80 LoC new. vsdd init refuse-on-drift already exists. Config files = YAML pass-through for v0.1, ~20 LoC.

**Total v0.1-blocking churn: ~570 LoC churn for ~120 LoC net new functionality.** Plus the test rewrite (vsdd-core/tests/init.rs at 297 LoC will need subprocess-fixture rework if the refactor lands in v0.1.0).

**Time-to-deliverable picture:**
If the refactor lands first, v0.1.0 ships AFTER (i) reserved-code fixes, (ii) vsdd subprocess seam, (iii) mdatron crates.io publish, (iv) vsdd init refuse-on-drift. That's the four-item v0.1-blocking set. The `src/commands/` move can ride with (ii) at low marginal cost. **v0.1.0 lands at the end of phase 3 below**, not phase 7.

## 3. Adopter realism — has the directive shifted the adopter shape?

**Yes, partially.** Review #3 named the v0.1 adopter as (a) "VSDD methodology user reached via `vsdd init`". The binary-first directive doesn't change WHO the adopter is — it changes WHAT the adopter sees: instead of "vsdd is a tool that bundles mdatron's logic", the adopter sees "vsdd is a tool that orchestrates a separate `mdatron` binary they can also use directly".

This is the shift toward the **"tooling integrator who wants validators and methodologies as separable concerns"** shape — but it's a v0.1.x adopter, not a v0.1.0 adopter. v0.1.0 still ships for (a). The directive PREPARES for the integrator-adopter without requiring them to exist yet.

**What the integrator-adopter needs that today's adopter doesn't:**
- Stable `mdatron verify` JSON output (the subprocess contract). v0.1.0 must lock this shape OR explicitly mark it unstable. Recommend: ship `--format=json` with explicit `"schema_version": "0.1"` field, signal instability via x-prefix-or-similar.
- `mdatron` discoverable on `$PATH` independently. `cargo install mdatron` after the crates.io publish satisfies.
- vsdd not reaching into mdatron internals — the subprocess boundary IS the contract. No library types crossing.

**Adopter does NOT need yet:** policy plugins, custom validators, schema authoring tools, registry. Those are integrator-adopter-v0.2 surfaces.

## 4. Init + config minimum surface

**mdatron init: ship v0.1.0 at starter-scaffold-only.** Per review #3 — `.mdatron/{schemas,patterns,catalogs,registries}/` empty dirs + `mdatron.yaml` catalog file + .gitignore section + refuse-on-conflict. No interactive walkthrough. No presets. No `--check` dry-run. NO `.mdatron/config.yaml` at v0.1 (deferred to v0.2 per same review).

**vsdd init under new directive:** refuse-on-drift scope (per `solution-owner-init-drift`) is unchanged. Subprocess-seam-aware addition: vsdd init must NOT call mdatron-core to validate; it shells out (or skips validation at init-time and relies on `vsdd verify` calling `mdatron verify` after). Recommend the latter — init writes, verify validates.

**Config files at v0.1: duck-typed YAML pass-through, NOT formal schemas.** Formal schemas + semver discipline are v0.2 work. Defending this: (a) the only adopter is vsdd-cli itself, which controls its own config production via init; (b) formalizing now locks shape before second-adopter evidence; (c) the SA brief's load-bearing finding ("formalize `.mdatron/config.yaml` in Phase 1a") was disposed at review #3 with v0.2 deferral and that cut binds here. **What ships at v0.1: presence + parseability + version-key**. That's the minimum surface that lets schemas-without-breakage later.

## 5. Carryover prioritization

- **#1, #20 Reserved-code spec drift**: **BEFORE refactor.** Real spec violation. Refactor moves the code surfaces; fix unattributed becomes silent drift (sycophancy_failure_mode). Land at v0.1.0 in a dedicated commit BEFORE the refactor commits.
- **#3a Should-fire fixtures**: **AFTER refactor.** Subprocess seam enables fixtures-by-stdout. Doing it now means rewriting the fixtures twice. v0.1.1.
- **#6 Audit-trail tightening**: **WITH refactor.** Refactor touches events.jsonl emission. Tighten event shape in the same commits that move the emission sites — but the SPEC amendments for tightening land separately at v0.1.1 (Raise-to-SO routing per SO dim 5).
- **#8 Templates deferral**: **AFTER refactor.** Under the new directive, templates split: mdatron owns catalog templates, vsdd owns methodology templates. v0.2 with second-adopter trigger.

## 6. Risk picture

**Biggest scope risk: scope sprawl via "while we're refactoring, let's also...".** The 20+ in-flight dispositions tempt absorption into the refactor's churn-budget. The disposition above explicitly resists: only 4 items v0.1-blocking; 16 deferred-named.

**Most likely failure mode: churn-without-shipping.** ~570 LoC churn for ~120 LoC net new is the kind of ratio that swallows two weeks and produces nothing observable to the adopter. Mitigation: keep mdatron-side untouched at v0.1 (workspace stays split, no `src/commands/` move); confine refactor churn to vsdd-side; the visible deliverable is `mdatron verify` working under subprocess invocation from `vsdd verify`.

**Premature lock-in risk: the subprocess JSON contract.** Once v0.1.0 ships with `mdatron verify --format=json`'s shape, breaking it breaks vsdd. v0.x semver gives cover but the integrator-adopter (v0.1.x) starts depending on it. Mitigation: ship JSON output with an explicit `schema_version` field; v0.x signals instability; promote to v1.0 only when DESIGN-MDATRON.md V1-SHIP-CRITERIA met.

**Pressure-tested against the do-nothing path:** if the refactor is deferred entirely, vsdd stays library-coupled to mdatron-core. The operator directive becomes aspirational. The integrator-adopter shape never materializes because the seam doesn't exist. AND: the path-dep convention review #3 named as fragile stays fragile, because vsdd-cli is still importing mdatron-core types. **Conclusion: the vsdd-side half of the refactor IS load-bearing. The mdatron-side half is not.**

## 7. Recommended sequencing — phases

**Phase A (small) — Reserved-code spec drift fix.** #1, #20 land BEFORE refactor. Accept: `MDATRON-E0070`, `MDATRON-E0080` relocated per review #3's range discipline; no engine-range codes in adopter-surface. Done = `grep` shows no overlap.

**Phase B (small) — mdatron crates.io publish at v0.1.0.** Both `mdatron-core` and `mdatron`. Accept: `cargo install mdatron` succeeds from a clean env; `cargo add mdatron-core@0.1` works. Done = published + tag pushed.

**Phase C (medium) — `mdatron verify --format=json` shape lock.** Add `schema_version` field to Finding-list JSON output. Accept: documented in DESIGN-MDATRON.md § Public contracts; round-trip parse test ships. Done = test passing + spec amendment merged via Raise-to-SO.

**Phase D (medium-large) — vsdd subprocess seam.** Drop `mdatron-core` from vsdd's Cargo.toml; introduce `vsdd/src/verify.rs` that shells out to `mdatron verify --format=json` and parses. Update events.jsonl emission to log the subprocess invocation (composes with #6). Accept: `vsdd verify` produces identical Finding output to current; events.jsonl logs `MdatronInvoked{version, exit_code}`. Done = integration test green; vsdd-core/tests/init.rs migrated to subprocess fixtures (or marked v0.1.1).

**Phase E (small) — vsdd `src/commands/` layout move (OPTIONAL at v0.1).** If Phase D's churn is already in flight, ride along: vsdd-core's init.rs → vsdd/src/commands/init/mod.rs; verify orchestration → commands/verify/mod.rs. Accept: structure mirrors crosslink cue; behavior unchanged. Done = `cargo test` passes post-move. **Cut this phase if Phase D ran long** — the layout is cosmetic at 2-3 verbs.

**Phase F (small) — mdatron init (starter-scaffold-only) ships.** Per review #3 scope. Accept: deploys `.mdatron/{schemas,patterns,catalogs,registries}/` + `mdatron.yaml` + .gitignore section; refuses on conflict with `MDATRON-E0220`. Done = integration test green; no interactive paths.

**Phase G — v0.1.0 RELEASE.** Tag mdatron v0.1.0 + vsdd v0.1.0. Adopter-observable surface: `cargo install mdatron && vsdd init && vsdd verify` works end-to-end. **v0.1.0 lands here.**

**Phase H (v0.1.1) — Carryover absorption.** #3a fixtures-via-stdout; #6 audit-trail spec amendments; M4 CI install-command divergence; reserved-code drift if any escaped Phase A.

**Cut criterion if scope sprawls:** Phases A, B, C, D, F, G are floor. Phase E (layout move) and Phase H (carryover) are deferrable. The refactor's payoff is the seam (Phase D), not the symmetry (Phase E).

## Routing

- **Raise to SO accepted on this disposition itself** (SO is author + decider). Sanity Check rubber-ducked below.
- **Phase 4 → Phase 1b** (DESIGN-MDATRON.md + DESIGN-METHODOLOGY.md amendments): subprocess-seam as public contract; `mdatron verify --format=json schema_version` field added to § Public contracts.
- **Phase 4 → Phase 2a** for Phase A (reserved-code fix) — small, lands first.
- **Phase 4 → Phase 2c** for Phase B (crates.io publish workflow) — composes with same review-#3 disposition.
- **Phase 4 → Phase 2b** for Phases D, F (implementation) — sequenced per above.
- **Composes with** prior `solution-owner-init-drift` (vsdd init scope unchanged) and review #3 `solution-owner-mdatron-consistency` (cuts named there bind here).
- **NOT raised**: mdatron workspace consolidation, mdatron `src/commands/` layout, `.mdatron/config.yaml` formalization — explicitly deferred.

## Sanity Check (validator-pair) baseline

Rubber-ducked the load-bearing question: **"does the binary-first refactor's payoff justify the churn for an adopter base that may not yet exist?"** Answer: partially, and the partial cut is the disposition above. The vsdd-side half (subprocess seam) ratifies the operator directive's boundary AND prepares the integrator-adopter surface without requiring the integrator-adopter to exist yet. The mdatron-side half (consolidation, src/commands/ mirror) is churn-for-symmetry's-sake; mdatron-core IS the publishable public contract per review #3, and dissolving the workspace would invert that.

**Cross-finding coherence:** consistent with review #3 (mdatron-core stays as publishable lib; mdatron init starter-scaffold-only); consistent with `solution-owner-init-drift` (only-adopter-is-vsdd-cli reasoning); consistent with the SA brief's diagnosis (anyhow-at-CLI-boundary already planned). Diverges from the SA brief's `.mdatron/config.yaml` Phase 1a recommendation — review #3 already disposed that to v0.2 and this entry binds to review #3.

**Last-resort discipline check:** I am the author + decider; no abdication. Pressure-tested both directions (do-nothing path AND full-mirror path both rejected with named cuts). The 16 deferred items are NAMED with version-bound dispositions, not silently dropped.

## Cross-references

- review-log/2026-06-02-solution-owner-mdatron-consistency.md (review #3 — binding cuts: mdatron-core stays publishable; init starter-scaffold-only; v0.1.0 crates.io publish; M1/M2/M6 → v0.1.1)
- review-log/2026-06-02-solution-owner-init-drift.md (vsdd init refuse-on-drift; only-adopter-is-vsdd-cli reasoning)
- review-log/2026-06-02-solution-architect-mdatron-consistency.md (SA brief — 20+ disposition source; anyhow-at-CLI seam already named)
- mdatron/mdatron-core/src/{verify.rs:46-57, diagnostic.rs:56-99, error.rs:12-25} (current shape — public contract surface, kept)
- mdatron/mdatron-cli/src/main.rs:24-50 (current 2-command surface — `src/commands/` deferred)
- mdatron/mdatron-cli/src/main.rs:74,141 (engine-range code leak — Phase A fix target)
- vsdd-cli/vsdd-core/src/init.rs (295 LoC — moves to vsdd/src/commands/init/mod.rs in Phase E)
- vsdd-cli/vsdd/src/preflight.rs:219 (mdatron-core typed-call sites — Phase D subprocess seam target)
- vsdd-cli/vsdd-core/tests/init.rs (297 LoC — subprocess-fixture rework target; v0.1.0 if Phase D budget allows else v0.1.1)
- crosslink/crosslink/src/commands/{init,issue,knowledge,...}/mod.rs (cue for src/commands/ layout — Phase E target, deferrable)
- mdatron/DESIGN-MDATRON.md § Public contracts (Phase C spec amendment target: `mdatron verify --format=json schema_version`)
