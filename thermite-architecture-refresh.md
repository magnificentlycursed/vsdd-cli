---
title: "Thermite architecture refresh (exhaustive)"
tags: ["reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-02
updated: 2026-08-02
---


## Design Specification

### 1. what thermite now is

Thermite is a verification-mandatory programming language for AI-authored code: every function states `req` (preconditions), `ens` (postconditions), `fx` (effect row); `forge check` proves the contracts or returns concrete counterexamples, and records everything in an assurance certificate. Premise (docs/overview.md): agents pay for proof in compute, not human attention — "spend the cheap resource (tokens) to buy the expensive one (trust)." Humans decide what the software should do and read certificates; the agent writes the proofs.

### Workspace layout (Cargo.toml, 8 member crates)

- `thermite-syntax` — lexer, recovering parser, AST, stable semantic addressing.
- `thermite-spec` — SpecTherm combinator registry (frozen SMT triggers + Verus definitions + executable forms) and the Rust mirror of the stratified-cage admission classifier.
- `thermite-lower` — AST → Verus-annotated Rust lowering; L1 runtime-check compilation; effect subsumption.
- `forge` — the CLI/driver (~50 modules incl. `verdict.rs`, `covenant_engine.rs`, `battery.rs`, `seven_verdicts.rs`, `slag.rs`, `review.rs`, `lean_smt_export.rs`, `epr_reconstruct.rs`, `kernel_image.rs`, `verified_build.rs`, `sandbox.rs`, `mutation.rs`, `vacuity.rs`, `metrics.rs`).
- `thermite-skill` — the generated `THERMITE.skill.md` language reference, CI-gated below 6,000 estimated tokens; `forge skill --claude` emits the Claude Code skill from the same content (no hand-maintained copy to drift).
- `thermite-verified` — Verus-verified soundness-critical pure core (effect-subsumption decision as a proved 9-atom u16 bitset; shrinks the TCB by moving a soundness-critical decision out of unverified Rust).
- `thermite-tv` — the contract-faithfulness translation-validation engine: independent reference encoder + per-clause Z3 equivalence obligation `P_production <==> P_reference`; build-system-enforced independence from the production lowerer.
- `thermite-kernel` — NEW: the verified freestanding kernel library (capability, frame, memory, smp, irq, scheduler, dma, atomic, sync, boundary, policy, registry, services… modules) with model tests.

Non-crate directories: `lean/` (the Lean proof spine incl. `Thermite/Strat/` and reconstruction), `conformance/` (~60 entries: hand-authored `.th` programs + golden certificate oracles, now incl. `bootable_kernel.th`, `kernel_primitives.th`), `tests/golden/` (hand-authored Verus lowerings + SMT), `platform/x86_64-pc-uefi-smp-v1/` (NEW: frozen target platform layer — boot adapter, AP trampoline assembly, QEMU acceptance harness, `registry.toml`, `build-image.sh`, `test-qemu.py`), `tooling/` (the discipline gates + REQ registry), `.design/` (per-component contracts + audit pins + the stage design docs), `scripts/` (audit + g3/g4 gates), `docs/` (overview/language/verification/trust), `.claude/agents/` (the four ACToR roles), `examples/` (editor, calculator, formatter, parser).

### The stage/gate system — current state

The RFC-1 "Thermite 2" program is organized by stage gates; CHANGELOG.md is organized by gates, per R-GATE-1 (headline claims flip at gate declaration, not merge time):

- **Gate G1 — Stage 1 "forge tier" (2026-06-18)**: five-rung ladder L0–L4; seven certificate verdicts; the Lean discharge engine; covenant engine; anti-Goodhart at L3 (re-elaboration mutation scoring, definition-tower budget); frozen tactic/simp battery; relax routing → L4 (nlsat, QF_NRA, RealWitness); `thermite2-semantics.md` as the normative semantics home.
- **Gate G2 — Stage 2 "stratified-FOL cage" (2026-06-22)**: surface `forall`/`exists`; the `Strat` Lean spine; sort-typed admission classifier with a Rust mirror + SplitMix64 differential battery; MBQI encoder + kernel-proved soundness theorems; restratification with the `Side(φ',φ)` obligation (R-SIDE-1); the trust flip mechanically withheld by `forge g2-gate` unless four audit checks are green; 8-pin kernel-`decide` regression battery.
- **Gate G3 — Stage 3 "fixed-width clauses + checked reconstruction" (declared 2026-07-29)**: `ens@bvN`/`inv@bvN`/`@bvN(nowrap)` for N ∈ {8,16,32,64}; per-clause `bv_shadow` records; width-aware mutation; fail-closed nowrap; and **default Lean replay of solver-proved QF_LIA/QF_BV clauses as the actual `req → clause` theorem** (REQ-7/REQ-8 below). Gate command: `scripts/g3-gate.sh`.
- **Stage 4 "checked EPR reconstruction" (shipped post-G3, `.design/stage4-epr-reconstruction.md`, Gate G4 = `scripts/g4-gate.sh`)**: closes the remaining S₂.0 trust gap G2 left model-relative. Every formula admitted by the stratified classifier must either produce a genuine countermodel or a kernel-checked proof: Lean rebuilds the finite grounding and CNF itself, checks a CaDiCaL-produced LRAT certificate (pinned CaDiCaL 2.1.3 @ f13d7443…, drat-trim @ effa1dcc…; the gate accepts only those binary identities), and derives the clause theorem. False clauses: CaDiCaL supplies the Boolean assignment, Forge realizes QF_LIA/QF_BV leaves, Z3 only *finds* the integer witness and Lean's `omega` *checks* it; unrealizable masks are blocked and re-asked. "Missing tools, exhausted budgets, and unrealized models are named failures, never proofs." G4 runs under a 6 GiB address-space ceiling (prlimit) for CI/small machines.
- **Post-Stage-4 arc (the current frontier)**: correspondence-backed L3 *builds* — `VerifiedCompositionReceiptV1` exact-source composition closures (#102/#106/#107), receipts binding the exact Verus codegen toolchain (#105), verified kernel byte-slice reads (#109), fail-closed Verus error accounting (#112), struct-invariant binding through unary operators (#113), and the bootable multicore kernel image (#114).

### What the bootable-multicore-kernel milestone means (`.design/build/bootable-multicore-kernel.md`, status: shipped; REQ-MKERNEL-1..16 shipped in the registry)

Thermite's kernel product is no longer just a verified freestanding rlib: a new `forge build --target kernel-image --platform x86_64-pc-uefi-smp-v1` closes the verified library over one **frozen target platform layer (TPL)**, boot adapter, link policy, compiler runtime, and image packager, and publishes a bootable UEFI disk image **only with a receipt (`ThermiteBootableKernelReceiptV1`) that binds every member of that closure** (sources, contracts, effects, ABI symbols, toolchain digests, PE/COFF+PDB inventories, boot transcripts).

Key structure:
- **Four assurance layers**: verified kernel core (L3 where exact-source rules admit) / verified concurrency runtime (L3 against frozen atomic+interrupt models) / TPL (registered L1 boundary contracts + review/external evidence) / image closure (digest-bound build evidence + runtime acceptance).
- **Honest scope labeling**: the receipt reports artifact scope `to_platform_boundary`; it "cannot describe the whole image as end-to-end L3 while registered platform calls remain boundary assumptions." Hardware, firmware, compiler backend, linker, frozen TPL bodies stay in the stated TCB.
- **Frozen platform registry**: privileged operations are `#[boundary("kernel::<domain>::<operation>@v1")]` functions; authority values are `#[sealed]` capability types (BootInfo, CpuCap, FrameCap, MmioCap, IrqCap, DmaCap, …) with a generation-tracked capability ledger. A final image accepts only boundary declarations exactly matching its platform registry (104 operations in the shipped profile); unknown names, signature drift, weaker contracts, extra effects, ABI mismatch, or unbound assembly reject the build. A new closed `platform(...)` effect family (boot/memory/mmio/pio/irq/cpu/atomic/smp/dma/clock/entropy/power) participates in caller subsumption; hosted effects are invalid for kernel targets.
- **Paired closures**: the proof closure (Verus proves each caller against the boundary's exact contract) and the implementation closure (the declaration resolves to exactly one frozen registry entry with the bound Rust/assembly body) must correspond one-to-one; slag is excluded from image closures.
- **Release gate**: boots the same image at 1, 2, 4, and 8 logical CPUs under QEMU/OVMF (4-CPU is the normative SMP acceptance case); real post-firmware paging, INIT/SIPI AP startup, IPIs, TLB shootdown epochs, virtio-blk DMA, RDRAND, ring-3 entry with architectural SYSCALL/SYSRET, user page fault, reboot/poweroff. A large adversarial negative matrix (receipt-field mutations, transcript-marker mutations, boundary drift, stale capabilities, …) is permanent tests; "a failed proof, link, receipt validation, replay, boot, SMP test… leaves the publication path absent." Two clean builds must be byte-identical; replay reproduces them before acceptance. CI job `kernel-image` runs the release-shaped gate (build+boot+bind, then `forge verify-build dist/ci-kernel.img --json`).

---

### 2. the enforcement architecture (current)

### 2.1 The rules register — every R-* family at HEAD

Primary source: `goal.md` (the binding /goal contract); supplements: `.claude/agents/*.md`, `thermite2-semantics.md`, code enforcement sites named below.

**Citation (goal.md)**
- **R-CITE-1** — never cite a design source in a commit without Reading it this iteration.
- **R-CITE-2** — `thermite-design.md` cites carry `§<section>`; golden/corpus cites carry the file path.
- **R-CITE-2b** — cite symbols with symbol anchors, never line numbers, in `.design/`.
- **R-CITE-3** — prefer citing the design REQ/AC or thesis pillar over an internal helper.

**Honesty (goal.md)**
- **R-HONEST-1** — never reframe integration work as "vocabulary-only" when the design doesn't defer it.
- **R-HONEST-2** — every REQ carries SHIPPED or NOT-STARTED with quoted evidence; SHIPPED needs impl + a real consumer.
- **R-HONEST-3** — honest underclaim beats unverified overclaim (also cited by the generated-TV workflow: Unverifiable/Skipped never fail the exit).
- **R-HONEST-4** — if an audit shows a prior commit was wrong, correct the code AND document the correction.

**Code quality (goal.md)**
- **R-CODE-1** — no `unsafe` outside leaf primitives; every `unsafe` needs `// SAFETY:`.
- **R-CODE-2** — no `unwrap()`/`expect()`/`panic!()` in production; `Result<T, ThermiteError>` with context-bearing variants.
- **R-CODE-3** — no module/crate-root `#![allow]`; per-item `#[allow(<lint>, reason="…")]` only.
- **R-CODE-4** — no swallowing solver/subprocess failures; timeouts degrade the ladder and are reported, never silent successes.
- **R-CODE-5** — determinism is a contract: no wall-clock/un-seeded randomness in build/format/codegen/check paths; solver seeds pinned.

**Prose (goal.md)**
- **R-TONE-1** — prose follows `.design/tone-and-voice.md`: affirmative not defensive, no antithesis pairs, no virtue adverbs, no rhetorical bold/ALL-CAPS; a register rule that never changes a claim.

**Spec-mirror (goal.md)**
- **R-SPEC-1** — surface grammar, mandatory `req`/`ens`/`fx` + `inv`/`dec`, the combinator set + frozen triggers, ladder semantics match the design exactly; "one way to do everything."
- **R-SPEC-2** — certificate/manifest fields, assurance levels, vacuity-battery outputs, slag metadata match the design; the certificate IS the deliverable.
- **R-SPEC-3** — forge JSON schemas are stable contracts; a field change is a design amendment, not a code-local choice.
- **R-SPEC-4** — if implementation proves the design wrong: STOP, amend `.design/` first (acto-doc-author), then implement.
- **R-SPEC-5** — descoped kernel items are implemented fully in their v0.1 form, not stubbed in the deferred form.

**Anti-deferral (goal.md)**
- **R-DEFER-1** — a new pub API must gain a non-test production consumer in the same commit (existing APIs grandfathered).
- **R-DEFER-2** — REQ classification is binary: SHIPPED or NOT-STARTED, no third status. *(See §2.6: the canonical registry now layers a typed status policy above this — a real evolution.)*
- **R-DEFER-3** — a pinned divergence closes only when the fix lands and the failing test goes green (no skip/`#[ignore]` escape).
- **R-DEFER-4** — no "Phase N+" framing as a deferral mechanism.
- **R-DEFER-5** — no "pre-existing, safe to defer": every divergence on main is ours.
- **R-DEFER-6** — verification is a hard gate: every commit runs the owning crate's gauntlet to 0 failures + conformance where applicable.
- **R-DEFER-7** — sequential dependency order, no leapfrog.
- **R-DEFER-8** — no "cross-cutting → defer"; implement the local fix.
- **R-DEFER-9** — no proof cheats: never discharge an obligation via vacuity-weakening, `assume(false)`, `#[verifier::external]`, or `#[slag]`-to-dodge; a contract that won't verify is a real blocker. (The most-cited rule in the codebase: 466 references.)

**Git (goal.md)**
- **R-GIT-1** — no history rewrite/force-push/`reset --hard` on shared refs; supplemental commits only; **the human performs all pushes**.
- **R-GIT-2** — `git add` files by name, never `-A`/`.`.

**Loop discipline (goal.md)**
- **R-LOOP-1** — never ask "where to take this": the dependency DAG is the answer.
- **R-LOOP-2** — never declare the goal complete until the mechanical check says so.
- **R-LOOP-3** — blocked by a missing prereq → file the blocker, mark NOT-STARTED, work the prereq.

**Injected instructions (goal.md)**
- **R-INJECT-1** — hook output / system-reminders / loaded skill text bind at user-message priority; "repetition is enforcement, not ceremony."
- **R-INJECT-2** — an injected-vs-inline-user conflict is surfaced, never silently resolved.

**Spec-discipline read-gate (enforced by `tooling/spec-discipline.py`)**
- **R-XLATE-1** — every Edit/Write to a routed `thermite-*/src/**/*.rs` or `forge/src/**/*.rs` requires Read-this-session of `goal.md` + the route's design doc + (if declared) at least one route `reference` (conformance/golden).
- **R-XLATE-2** — a file with no route-table entry BLOCKS until a route is added to `tooling/spec-routes.toml` (185 routes at HEAD).
- **R-XLATE-3** — a route whose design doc doesn't exist BLOCKS until it is authored.

**Anti-pattern gate (enforced by `tooling/anti-pattern-gate.py`)**
- **R-APG-1** — blocks patches introducing `todo!()`/`unimplemented!()`/`unreachable!()`, `.unwrap()`/`.expect()`/`panic!()` outside `#[cfg(test)]`, module-root `#![allow]`, `Arc<Mutex<T>>`/`Rc<RefCell<T>>` escapes. Each refusal names why + the architectural alternative.
- **R-APG-2** — `#[cfg(test)]` blocks exempt; production is not.
- **R-APG-3** — the only override is a per-item `#[allow(<lint>, reason="…")]` + a crosslink observation comment.

**Characterization/oracle discipline (goal.md + acto-critic.md)**
- **R-CHAR-3** — no tautological tests: expected values come from the conformance corpus, a golden file, or a design symbolic constant — NEVER copied from the toolchain's own output. "A test asserting the toolchain's output equals itself IS a divergence (file the test as the bug)." (671 references — the most-woven rule of all.)

**Doc-author rules (`.claude/agents/acto-doc-author.md`)**
- **R-DOC-1** — the doc adapts to the code, never the reverse; no code-change proposals from the doc author.
- **R-DOC-2** — binary classification only (= R-HONEST-2 restated for docs).
- **R-DOC-3** — every SHIPPED REQ cites the impl symbol AND the non-test consumer symbol with quoted evidence.

**Builder rule (`.claude/agents/acto-builder.md`)**
- **R-BUILD** — the pre-declared ≤~10-file manifest is "an absolute boundary": off-manifest need → STOP and report "manifest needs expansion: <file> because <reason>"; the orchestrator re-authorizes. Tests + production in the same commit; critic re-audits every manifest file.

**Thermite-2 program rules (goal.md §"Thermite 2 program rules" + `thermite2-semantics.md`)**
- **R-VERDICT-1** (never-converts-silently, **enforced** in `forge/src/verdict.rs`) — a certificate carries exactly one of the seven `CertVerdict` outcomes (`Proved`, `Counterexample`, `RealWitness`, `CovenantRefuted`, `Stuck`, `KernelBudget`, `Timeout`); `Proved` is constructed only from an engine `Proven`; the engine→cert map is total with no wildcard arm; Lean kernel-budget/residual-goal outcomes are classed upstream, never remapped.
- **R-COV-1** (covenant-before-burn, **enforced** in `forge/src/covenant_engine.rs`) — the L3 proof search is entered only on a validated covenant carrying at least one author-stated `inhabit` witness; a `falsify` refutation or malformed covenant returns named, without burning.
- **R-GATE-1** (headline-at-gate-time, applied as a review rule at each gate) — a stage's product-facing headline claim changes when its gate is declared, not when an increment merges. The CHANGELOG itself is organized by this rule.
- **R-SIDE-1** (restratify Side obligation) — `forge edit --restratify` emits an in-cage `Side(φ',φ)` obligation; certification of φ' counts for φ only when Side is discharged. **Shipped with Stage 2 REQ-7** (goal.md's "candidate, not yet enforced" text is stale — see Deltas).
- **R-BV-1** (`@bv` shadow-flag parse gate) — a build without the bv plumbing rejects `@bvN` at parse time with a structured syntax error. **Shipped with Stage 3 REQ-1/AC-1 and CI-gated** ("bv build-flag gate" step + G3 checks 1/5 and 2/5 run both release parser configurations).
- **R-BAT-1** (frozen tactic/simp battery) — citing a tactic or simp lemma outside the frozen auditable allowlist (`forge/src/battery.rs` REGISTRY, pinned against the hand-derived `conformance/battery/registry.json`) is refused at elaboration as a named hard error, never a warning.

### 2.2 The ACToR loop, agents, and tool restriction

Four tracked subagent roles in `.claude/agents/` drive the read → write → verify → commit loop under a human orchestrator (authority chain: `thermite-design.md` → `.design/<area>/<doc>.md` → impl → verification, never reverse):

- **acto-doc-author** — authors/amends `.design/` docs; NO toolchain-code edits (R-DOC-1).
- **acto-builder** — ships a missing abstraction inside a pre-declared ≤~10-file manifest (R-BUILD); no `--no-verify`, no commenting-out tests; gauntlet-fails-you-can't-fix → revert and report. Explicit "fix the cause's whole class" discipline (enumerate all instances of a structural cause, no symptom patches).
- **acto-critic** — the discriminator: tools `Read, Write, Bash, Grep, Glob` — **no Edit** ("if you find yourself wanting to Edit, you have drifted from discriminator into generator — STOP"). Pins every divergence as a runnable FAILING test whose expected value traces to the authority (R-CHAR-3); files a blocker; verdict vocabulary is only "GENERATOR MUST FIX" / "NO DIVERGENCE FOUND" — it cannot APPROVE, and there is no "ACCEPTABLE DRIFT." One narrow Write exception: overwriting its OWN prior critic test with a self-acknowledged authoring bug.
- **acto-fixer** — one minimal root-cause fix per pinned divergence, serially, followed by a critic re-audit.

All four agent files now carry `model: fable` frontmatter (prose in the critic body still says "Opus — always"; the frontmatter is the operative pin). All four carry an "Operational discipline" section: never switch branches (orchestrator's job), clean scratch files, `--no-changelog` on issue closes.

Speed disciplines S1–S8 (goal.md) bound the loop: batch by component, parallel dispatch of disjoint manifests, symbol anchors never line numbers, critic only after substantive builds, aggressive won't-fix on noise.

### 2.3 Hooks and read-gates (the control plane)

Tracked `.claude/settings.json` wires:
- **PostToolUse / Read → `tooling/spec-discipline.py`** — records reads in `.crosslink/.spec-reads.json`.
- **PreToolUse / Write|Edit → `tooling/spec-discipline.py`** — blocks routed edits until goal.md + the governing design doc + a declared reference were Read this session (exit-2 with exact remediation).
- **PreToolUse / Write|Edit → `tooling/anti-pattern-gate.py`** — the R-APG regex gate; Write scans full content excluding `#[cfg(test)]`, Edit scans replacement text.
- Additional crosslink-generated hooks are referenced (`.claude/hooks/post-edit-check.py`, `heartbeat.py`, `pre-web-check.py`, `work-check.py`, `session-start.py`, `prompt-guard.py`) but **not tracked in the repo** — every entry is wrapped in `if [ -f "$HOOK" ]` so an absent script degrades to a silent no-op. Only the two tooling gates + the Read recorder are asserted-live (next item).

### 2.4 The control-plane meta-gate — NEW (`tooling/control-plane-check.py`, `.design/tooling/control-plane.md`, `make control-plane`, CI step "control-plane gate (hook wiring)")

Born from the de-wired-gates incident (crosslink #93; fix commit `904f4bc6`): commit `5581b65f` ran `crosslink init`, which regenerates `.claude/settings.json` from a generic template and **silently dropped the project-specific hook entries — the spec-discipline and anti-pattern gates were dormant for the entire Stage-3 arc.** The new check reads the tracked settings file and asserts each REQUIRED_HOOK (the three above) has a covering matcher and an existing script; each requirement names the doc line whose claim goes false if unwired ("a finding points at the prose that would go false"). Defect classes MISSING-WIRING / MISSING-SCRIPT / UNPARSEABLE; a missing wiring's report includes the ready-to-paste JSON restore snippet. Exits 0/1/3. Explicitly NOT part of `make audit` (Makefile: "hook wiring is a development-discipline invariant, not a link in the proof-trust chain — the doc-drift decision-5 precedent").

### 2.5 Doc-drift pins (`tooling/doc-drift.py`)

Every routed design doc must carry an `audited-content-sha256:` pin — a deterministic aggregate SHA-256 over the doc's governed file set — making drift a data-consistency check independent of merge topology. Legacy `audited-sha:` 40-hex commit pins remain as fallback (checked against later commits touching governed files) but were migrated in bulk (commit `d0196595` "migrate 6 legacy commit-pins to content-sha"). Exits 0/1/3. `make doc-drift` mirrors PR CI by evaluating a **synthetic base+head merge commit in a temporary worktree** (so the gate sees what CI's merge ref would see). The gate has its own hand-authored oracle fixture suite (`tooling/tests/`, R-CHAR-3 applied to the gate itself).

### 2.6 The canonical REQ registry — NEW LAYER (`tooling/req-registry.py`, `tooling/reqs`, `.design/reqs/registry.toml`, `.design/tooling/req-registry.md`)

Above the legacy `req-status.py` comment-table lint sits a machine-readable registry: **stable REQ IDs, one owner (a design doc), registry-declared status policy, typed evidence (file/symbol/test/command), tracker-neutral references, and named generated views** (125 views rendered into `.design/reqs/status.md` and `//!` regions of source files; `tooling/reqs check` fails if generated views are stale). At HEAD: **524 validated requirements** (519 shipped / 3 not_started / 2 partial / 1 retired). Status schema v1: `shipped` and `retired` are final and require evidence; `partial`, `blocked`, `deferred`, `not_started` are non-final and require remaining-scope (and for `blocked`, a named blocker). Growth is chronicled in the design doc header (462 → 472 (G4) → 496 (L3COMPOSE) → 502 (KERNELBYTES) → 505 (VERUSERR) → 524 (MKERNEL)). `make gauntlet` and CI run both `req-status.py` (legacy bridge) and `tooling/reqs check`.

### 2.7 Waiver/slag machinery

`#[slag]` (L0, trusted-by-fiat) is the sole escape hatch: `forge/src/slag.rs` `validate()` requires three non-empty justification fields `reason` / `owner` / `review` (the human-review slot). A valid slag item is **L3-exempt but L1-enforced** — its contract still compiles to runtime checks; it certifies `Level::L1` with `slag: true` and metadata in the certificate. Deliberately greppable: "`grep slag` over a codebase is the complete inventory of fiat-trusted code"; `forge audit`'s TCB section enumerates every slag block. Slag is subject to vacuity triage (it exempts a body from proving, never from stating and checking) and is **excluded from kernel image closures** entirely. (There is no separate "waiver" vocabulary at HEAD — the estate's "slag-waiver review" = the slag justification-fields + human `review` owner.)

### 2.8 Oracle discipline and manifests

Unchanged in principle, extended in surface: golden certificates (`conformance/<name>.cert.json`), golden lowerings (`tests/golden/lower/*.verus.rs`), the battery registry oracle (`conformance/battery/registry.json`), and the gate tools' own fixture oracles are hand-authored from the design, never regenerated from the toolchain (R-CHAR-3). Builder manifests stay the absolute dispatch boundary (§2.2).

### 2.9 Three-valued exits

The closed vocabulary **0 pass / 1 fail / 3 inconclusive** governs doc-drift, control-plane-check, and req-registry (env-failure → 3); `make audit` prints an INCONCLUSIVE verdict and exits nonzero when a guarantee-bearing check SKIPs ("a SKIP of a guarantee-bearing check is a degraded verdict, not a pass"); missing tools in proof-bearing gates (g3/g4) are hard exit-2 failures, never successful skips. The Makefile documents that GNU make collapses nonzero exits to 2, so scripts branching on 1-vs-3 must run the Python tools directly.

### 2.10 Spec-intent review slot

`.design/forge/spec-review.md` (status: draft, blocker #19): `forge review` extracts the pre-screened declarative spec layer (contracts only, no bodies) for battery-passing items and pairs each with an "is this what you meant?" prompt plus a pluggable structured verdict slot (`aligned: bool` + note) for an external reviewer — human or critic model; forge itself never calls an LLM (R-CODE-5 determinism). Battery-failing contracts are answered mechanically first, never surfaced for intent review.

---

### 3. the verification architecture (current)

### 3.1 The assurance ladder as now drawn (README/overview at HEAD)

| Rung | Meaning |
|---|---|
| **L4** | An admitted decidable route with **checked reconstruction** and concrete failures: nonlinear relaxation (nlsat), fixed-width BV, or finite EPR relation/array clauses |
| **L3** | All-input machine proof through Verus/Z3 **or the Lean engine** |
| **L2** | Bounded Kani/CBMC result with the bound recorded |
| **L1** | Always-active runtime contract check |
| **L0** | `#[slag]` fiat trust |

A function's level is the minimum over clauses; a counterexample is a hard failure, never a downgrade; plain `forge check` auto-routes eligible BV and EPR clauses through L4 reconstruction (Stage-3 REQ-8 made checked replay the default, Stage 4 extended it to EPR). Two proof engines discharge L3 (Verus default, Lean via `--engine lean|auto`); if the engines disagree on the same obligation, forge **halts with a soundness alarm** rather than resolving by preference.

### 3.2 The Rust→Lean obligation exporter (Stage-3 REQ-7) and trust migration (REQ-8)

- **REQ-7 (`forge/src/lean_smt_export.rs`)**: Forge renders QF_LIA and the full shipped QF_BV term surface as Lean propositions — the actual validity theorem `req → clause` (for QF_BV, `result` replaced by the body expression; for QF_LIA, `result` stays a quantified solver variable with unsigned-domain guards).
- **REQ-8 (default checked replay — the per-clause trust migration)**: a solver-proved QF_LIA/QF_BV clause is replayed by default as its Lean theorem. Trust changes ONLY when: Lean accepts the theorem; the anchored `#print axioms` report ⊆ `{propext, Classical.choice, Quot.sound}` (no `sorryAx`, no custom axioms); the certificate records theorem name, checker, generated-source SHA-256, fragment, axiom list; and, when the route exposes its solver input, that input's SHA-256. QF_LIA replays via Lean's verified `omega`; QF_BV via an axiom-clean portfolio (proof-producing LRAT checker + Lean automation + proved library lemmas — `bv_decide` deliberately not used because native evaluation adds an axiom). Unsupported or failed replay **remains solver-trusted and is listed by the audit** — trust migration is per-clause and evidence-carrying, never a blanket flip.
- **Stage 4** extends the same discipline to the stratified relation/array fragment S₂.0 (see §1): kernel-checked LRAT reconstruction for true clauses, Lean-checked concrete countermodels for false ones, canonical deterministic wire format consumed by classification, SMT emission, replay, hashing, and drift checks alike.

### 3.3 Translation validation and the correspondence spine

Two mechanisms, both machine-checked (docs/verification.md): per-run N-version TV (an independent reference encoder, build-enforced no-code-sharing, with Z3 proving production≡reference per clause) and the ∀-programs Lean faithfulness theorem (`Thermite.lowering_faithful` — the verified-validator/CompCert-lineage architecture; "Thermite's meaning is defined by the Lean semantics; Verus is the first proof engine against it, proven faithful"). The falsification battery ("teeth" suites) injects production-side infidelity classes and asserts Z3 CATCHES them. The Rust⇄Lean correspondence is pinned by content-SHA and drift-checked (audit check [4]/[4′]). NEW since the estate capture: the **rotating-seed generated-TV watchdog** (`.github/workflows/generated-tv.yml`) — daily scheduled `forge tv --generated` over a deterministically generated off-corpus clause space with seed = run number, so seed-dependent lowering divergences eventually surface as a red build (main CI runs the same space at a pinned seed for reproducibility).

### 3.4 The audit (`make audit` / `scripts/audit.sh`) — the skeptic's re-derivation

Checks at HEAD: [1] the universal theorem re-verified by YOUR Lean kernel (lake build from source + parsed axiom probe over the twelve gated theorems); [2] full-corpus TV (`forge tv`/`exec-tv`/`body-tv`, zero Divergent); [3] the multi-class falsification battery + one visible end-to-end mutant; [4] the correspondence drift tripwire; [5] third-party Verus re-check of the emitted proof with forge excluded; [G2] the four stage-2 checks combined by `forge g2-gate` (mechanically withholding the stratified trust flip unless all green in THIS run); [6] the verdict + the honest residual-trust statement. `make audit-fast` is the 60-second existence demo (faithful program certifies L3; same program with an injected bug is REFUSED; independent re-verification).

### 3.5 What's proven vs tested vs reviewed — the boundary as they draw it now

- **Proven (kernel-grounded)**: the lowering-faithfulness theorem and stratified soundness theorems (axiom-clean in the 3 standard axioms); L4 reconstruction theorems per clause (QF_LIA/QF_BV since G3, finite EPR since Stage 4); the `thermite-verified` effect-subsumption core (Verus bit-vector proofs); kernel-core transition predicates ("executable mirrors of the critical transition predicates are proved by `verus --no-cheating`").
- **Solver-trusted (proven modulo Z3/Verus)**: L3 clauses whose replay is unsupported/failed — enumerated by the audit, per-clause.
- **Tested**: the toolchain crates themselves (cargo test + conformance corpus + golden files + differential batteries + teeth suites + the rotating generated-TV space) — the development-discipline tier, deliberately outside the proof-trust chain.
- **Reviewed (human)**: slag justifications (reason/owner/review), spec-intent alignment (`forge review`'s verdict slot — "the one irreducible judgment the deterministic battery cannot make"), TPL bodies and platform-registry evidence, design amendments.
- **Residual TCB after a clean audit (docs/trust.md)**: (1) the Lean kernel + its 3 axioms; (2) Z3/Verus soundness (kernel-replay already covers the scalar-linear fragment); (3) the spec-vs-intent gap; (4) the pinned Rust↔Lean correspondence inspection; (5) rustc + LLVM. For kernel images additionally: hardware, firmware, compiler backend, linker, frozen TPL bodies — stated in the receipt.
- **Runtime cage**: hosted binaries are confined by an `fx`-derived seccomp-BPF filter — the same declaration drives static checking and the OS-level kill.

---

### 4. deltas — corrections/extensions to the estate's two pages (captured ~2026-07-20/22)

### Against `thermite-state-architecture`

Still accurate: the two-tier architecture with a defended boundary (proof-trust tier spent on code + code-spec correspondence; a deliberately weaker development-discipline tier of markdown/Python/TOML/JSON governing authoring); decision 5 (doc freshness ≠ proof-trust link); the closed 0/1/3 exit vocabulary; truth-in-evidence-bindings over schema-validated markdown.

Extend with:
1. **The control-plane meta-gate is a NEW third element of the discipline tier**: `tooling/control-plane-check.py` asserts the agent-facing hooks are actually wired in tracked `.claude/settings.json`, with defect classes and paste-back restore snippets, in CI and `make control-plane` — same decision-5 placement (explicitly not in `make audit`). It exists because the discipline tier's controls demonstrably CAN die silently (incident below).
2. **The de-wired-gates incident (crosslink #93)**: `crosslink init` at commit `5581b65f` regenerated settings.json from a generic template and silently dropped the spec-discipline and anti-pattern hooks **for the entire Stage-3 arc**. Enforcement-grade lesson: a hook-based friction control has a liveness problem the control itself can't see; the fix is a CI-backed check *of the wiring*, plus file-existence guards that make absence explicit rather than an error.
3. **The canonical REQ registry** (`.design/reqs/registry.toml`, 524 REQs, 125 generated views, `tooling/reqs check` in gauntlet+CI) now sits above the comment-table lint as the source of truth — stable IDs, one owner, typed evidence, policy-bearing statuses, staleness-checked generated views.
4. **Content-SHA pins are now the norm**: the legacy commit-pin fleet was migrated (`d0196595`); doc-drift additionally evaluates a synthetic CI-style merge commit in a temp worktree.

### Against `attended-design-autonomous-execution` (THERMITE'S BOUNDARY section)

Still accurate: the authority chain design → impl → verification; autonomy activated by explicit /goal + bounded by the mechanical stopping condition; the human slots — pushes (R-GIT-1), design amendments/thesis escalations, oracle authorship (R-CHAR-3), stage gates (R-GATE-1), slag review fields, spec-intent review, injected-instruction conflicts (R-INJECT-2); frozen-decision registers; critic-lacks-Edit; the spec-discipline read-gate; the anti-pattern regex gate; builder manifests as absolute boundaries; three-valued exits; doc-drift content-SHA pins.

Corrections and extensions:
1. **R-SIDE-1 and R-BV-1 are no longer forward-looking candidates** — both shipped and are enforced (Stage 2 REQ-7 `Strat/Restratify.lean` + `forge edit --restratify`; Stage 3 REQ-1/AC-1 with a dedicated CI "bv build-flag gate" step and both-parser-configuration G3 checks). Note: goal.md's own §"Thermite 2 program rules" prose still labels them "not yet enforced" — the repo's contract text lags its code here; cite the code, not that paragraph.
2. **The binary-status doctrine has a successor layer**: R-DEFER-2's SHIPPED/NOT-STARTED binary still binds legacy source-comment tables, but the canonical registry's schema v1 legitimizes `partial`, `blocked`, `deferred`, `retired` as *policy-bearing* statuses — non-final statuses must carry remaining-scope (and blockers where applicable), final statuses must carry typed evidence. The discipline evolved from "two states only" to "no state without its obligations."
3. **Stage-gate machinery got scripted and extended**: G3 declared 2026-07-29 (`scripts/g3-gate.sh`, 5 checks), Stage 4/G4 shipped (`scripts/g4-gate.sh`: pinned CaDiCaL/drat-trim binary identities, prlimit 6 GiB memory envelope, hard tool-missing failures) — beyond the estate's G1/G2 snapshot. The CHANGELOG is organized by gates per R-GATE-1.
4. **New rule in the register**: R-BAT-1 (frozen tactic/simp battery: unlisted citation refused at elaboration as a named hard error). The estate pages don't name it.
5. **The agent model pins changed**: all four acto-* agent files now carry `model: fable` frontmatter (S6/"Opus — always" prose survives inside the bodies unrevised).
6. **New verification surfaces since capture**: the per-clause trust-migration discipline (REQ-8) is now default-on and extends to EPR (Stage 4); L3 *builds* are receipt-bound (`VerifiedCompositionReceiptV1`, exact-source composition closures, Verus-codegen-toolchain binding); the rotating-seed generated-TV daily watchdog; honest Verus error accounting (fail-closed on unknown counts, #112).
7. **The kernel milestone adds a new enforcement genus**: the frozen platform registry + sealed capabilities + paired proof/implementation closures + `ThermiteBootableKernelReceiptV1` + replay-to-byte-identity + a permanent adversarial negative matrix, with honest scope labeling (`to_platform_boundary`) that structurally refuses to overclaim end-to-end L3. Publication is absent on any failure — the "no artifact without its evidence" pattern at image scale.
8. Minor: crosslink-generated hooks (heartbeat/work-check/prompt-guard/session-start) referenced by settings.json are **untracked** in this repo; every hook entry is `[ -f ]`-guarded, so only the three control-plane-checked wirings are asserted-live. The estate's "watchdog nudges" description reflects the crosslink kickoff substrate, not tracked Thermite files.

