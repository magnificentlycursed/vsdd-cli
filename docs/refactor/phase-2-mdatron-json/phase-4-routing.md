# Phase 4 — Feedback Integration Routing

**Issue:** crosslink #13 (Phase 2 of binary-first plan).
**Consumes:** 18 cluster-batched Phase 3 cold-session reviews at
`mdatron/review-log/2026-06-07-<domain>-phase-2-bundle.md`.

## Aggregate stats

Counts updated 2026-06-08 after Phase 4-take-2 closure pass + crosslink-
issue filing.

| Metric | Count |
|---|---|
| Reviews | 18 |
| Total findings | 129 (one issue per finding, filed as subissues of `#13`) |
| Resolved (closed in code) | 11 in initial pass + 24 in Phase 4-take-2 = 35 |
| Accepted (open orphans) | 35 → 13 after Phase 4-take-2 |
| Deferred (with block-by routing) | 8 → blocked-by future phases / umbrellas |
| Dismissed | 75 |
| Hallucinated | 0 |

The 129-finding total (not 137; prior counting error) reflects more
per-domain lensing opportunities than Phase 1's pure-Rust internals.

## Cross-cutting convergences

Findings that 2+ domains independently caught — the load-bearing
defects most worth fixing.

### Convergence 1 — Phase 2 closing-commit work pending (5-domain convergence)

**Caught by:** SA F8, SO F1, SO F2, SO F6, VM F3.

**Defect:** Three closing-commit items remain at the end of Phase 2:
1. `binary-first-plan.md` row 14 amendment (Phase 1c acceptance criterion)
2. `OperatorDirectiveApplied` event capture for the explain-disposition
   honored
3. CHANGELOG update (if applicable to mdatron)

**Disposition:** **Resolved-pending Phase 2 closing-commit work.** These
land in the Phase 2 closing commit (this routing pass). Disposition
records below.

### Convergence 2 — Argv reflection / control-char injection in cmd_explain (3-domain convergence)

**Caught by:** Security F1, Red Team F2, SE F6 (partial via format_tty
multi-line concern; same root: untrusted-string-interpolated-into-output).

**Defect:** `cmd_explain` reflects argv `code` into stderr in the
not-found + foreign-namespace paths. Argv may contain ANSI escapes or
control chars; rendered to terminal, the escapes execute.

**Disposition:** **Deferred to v0.1.x** — argv validation at clap layer
(reject codes that don't match `^[A-Z]+-[A-Z][0-9]{4}$`). Closes
Security F1, Red Team F2, mitigates the broader untrusted-display
concern. ~5 LOC clap macro. Not v0.1.0-blocking; the attack surface is
operator-shell-bounded.

### Convergence 3 — `CARGO_BIN_EXE_mdatron` migration (2-domain convergence)

**Caught by:** PE F5, AIE F4.

**Defect:** `cli_integration.rs` (and the existing `output_format.rs`)
hard-code `target/debug/mdatron`. Release builds + sandboxed agent-loop
runs both break on the hard-coded path.

**Disposition:** **Deferred to v0.1.x drive-by** — replace with
`env!("CARGO_BIN_EXE_mdatron")` in both test files. Trivial; not
v0.1.0-blocking.

### Convergence 4 — README install instruction precision (2-domain convergence)

**Caught by:** PE F1, Red Team F3.

**Defect:** README's `git clone github.com/magnificentlycursed/mdatron`
path assumes a standalone mdatron repo at that URL, but the project lives
as a subdir of the magnificentlycursed monorepo. Pre-Phase-6 the
standalone repo doesn't exist; the install instruction is aspirational.

**Disposition:** **Deferred to Phase 6** — Phase 6 of the binary-first
plan handles repo-split + crates.io publish; the install instruction
finalizes then. v0.1.0 README documents the operator-local form (via
relative path under the magnificentlycursed checkout) as a fallback;
this could be added now as belt-and-suspenders but is not v0.1.0-blocking.

### Convergence 5 — Path control-char escaping in format_tty (2-domain convergence)

**Caught by:** Security F2, SE F6.

**Defect:** `format_tty` interpolates `self.location.file.display()`
verbatim. Filenames containing control chars / newlines produce escape
sequences in stderr; visually broken rustc-shape + potential ANSI
injection.

**Disposition:** **Deferred to v0.1.x** — a `safe_display()` on
`Location` that escapes control chars. Closes both findings. Belt-and-
suspenders for an attacker model that requires write access to the
project tree.

### Convergence 6 — Explain-page-not-found message lacks actionable pointer (4-domain convergence)

**Caught by:** SC F1, UX F4, DR F4 (related), AIE F6.

**Defect:** `mdatron explain MDATRON-E0010` (reserved-but-not-cataloged)
returns "not in v0.1.0 baseline catalog" without telling the operator
where to go next.

**Disposition:** **Resolved in closing commit** — extend the not-found
message with a help line pointing to `DESIGN-MDATRON.md § Reserved
mdatron codes`. ~3 LOC. Belt-and-suspenders for the catalog-growth
asymmetry.

### Convergence 7 — README "First run" gotcha + sample output (4-domain convergence)

**Caught by:** SC F2, TW F7, UX F3, DR F1.

**Defect:** README's "First run" describes the run but doesn't show
sample TTY output, and doesn't warn that running `mdatron verify`
without `.mdatron/` exits 2.

**Disposition:** **Deferred to v0.1.x** — README polish: add a 6-line
sample-output block + a note about the empty-project case. Not
v0.1.0-blocking; the README's sequential prose ("create the dir THEN
run verify") prevents the gotcha for adopters who follow the sequence.

## Other notable findings

Routed by single domain, scope-confirmed at this Phase 4 pass:

| Finding | Source | Disposition |
|---|---|---|
| SE F4 — `format_tty` skip `= note:` when summary == message | SE | Deferred v0.1.x; 5 LOC |
| SE F5 — `print_pipeline_error` still open-coded | SE | Deferred v0.1.x; consolidation |
| SE F7 — `extract_fence` nested-fence edge case | SE | Dismissed; test scope only |
| QE F1 — README heading test substring-match permissive | QE | Deferred v0.1.x; tighten |
| QE F2 — README round-trip too permissive (asserts not-crash) | QE | Deferred v0.1.x; tighten |
| QE F8 — README fence ordering load-bearing | QE | Deferred v0.1.x; HTML markers |
| SA F1 — Cross-crate emission/catalog asymmetry | SA | Deferred Phase 4 binary-first; compile-time lint when collapsed |
| SA F3 — Phase 2c happened during 2b: spec wording slip | SA | Resolved in closing commit (amend Phase 1c wording) |
| SO F3 — Phase 0 disposition listed 4 codes; impl ships 5 | SO | Resolved in closing commit (amend Phase 0 DESIGN) |
| SO F5 — Catalog-emission lint | SO | Deferred v0.1.x |
| TW F1 — README opener differentiation polish | TW | Deferred v0.1.x |
| TW F2 — Explain page voice consistency (E0080) | TW | Deferred v0.1.x |
| TW F3 — `markdown` vs `md` fence portability | TW | Deferred v0.1.x |
| TW F4 — README relative-link rot post-Phase-6 | TW | Deferred Phase 6 |
| TW F8 — README "Relationship to vsdd" reorder | TW | Deferred v0.1.x |
| DR F2 — E0080 page leaks "Phase 5 of binary-first" lingo | DR | Resolved in closing commit (small wording fix) |
| DR F4 — E0070 page cites uncataloged E0010/E0011 | DR | Resolved in closing commit (mark as reserved-for-future) |
| DR F5 — README VSDD-vocabulary density | DR | Deferred v0.1.x |
| DR F6 — Explain pages link DESIGN-MDATRON.md per section | DR | Deferred v0.1.x |
| DR F8 — README forward-references mdatron-examples | DR | Resolved in closing commit (mark as v1.0 roadmap explicit) |
| PE F4 — CI subprocess-spawn wall-time | PE | Dismissed; 18 tests fine |
| RT F1 — `include_str!` PR-modifiable | RT | Dismissed; PR review mitigates |
| RT F4 — `extract_fence` unsafe if migrated to production | RT | Dismissed; test scope only |
| Sec F3 — External URL audit | Sec | Dismissed; pass |
| AIE F2 — Compact-form catalog for agent-loop | AIE | Deferred v0.1.x |
| AIE F3 — Domain-routing hints in vsdd-side overlay | AIE | Deferred v0.1.x (vsdd-cli territory) |
| AIE F7 — `mdatron explain --json` | AIE | Deferred v0.1.x |
| UX F1 — Markdown pretty-print | UX | Deferred v0.1.x (adds deps) |
| PerfE F2 — `format_tty` allocation | PerfE | Deferred v0.1.x; not Phase 2 hot path |
| PerfE F3 — eprintln batching | PerfE | Deferred v0.1.x |
| DE F1 — Explain-page format schema | DE | Deferred v0.1.x |
| VM F7 — events.jsonl emission | VM | Deferred v0.1.x; writer is Phase 4 binary-first-plan territory |

## Domain-wide honest dismissals

Per the operator-directive memory + 2026-06-02 cluster-batched discipline,
domains with no substantive Phase 2 surface produce honest domain-wide
dismissals with rationale. Phase 2 dismissals:

- **Accessibility** — CLI-only; no UI surface; rustc-shape ASCII safe
- **Localization** — English-only by mdatron spec; matches rustc/cargo
- **Privacy** — Local-only; no telemetry; argv-reflection bounded to operator
- **PerfE (mostly)** — Phase 2 has no hot path; verify pipeline is
  Phase 1 territory

These four mirror crosslink #12's four domain-wide dismissals
(Accessibility, Localization, Privacy, PerfE). Methodology shape
consistent across cycles.

## Closing-commit work (this routing pass)

1. **Amend `binary-first-plan.md` row 14** — "Strip" → "Implement
   explain catalog + retain line"; footnote citing the 2026-06-02 SO
   disposition recorded at `phase-0-output-format/DESIGN.md:566-575`
2. **Amend Phase 0 DESIGN.md open question #2** — list 5 codes (add
   E0050) not 4; cite SO F3 catch
3. **Amend Phase 1c spec wording** — refactor items may land in 2b OR
   2c (closes SA F3)
4. **Resolve DR F2 (E0080 page Phase 5 lingo)** — replace with "v0.1.x
   `mdatron init` will scaffold this for you"
5. **Resolve DR F4 (E0070 page reserved-codes)** — annotate the
   E0010/E0011 reference as "reserved for v0.1.x path-confinement check;
   explain page lands when the check ships"
6. **Resolve DR F8 (README mdatron-examples forward-reference)** — add
   "(v1.0 candidate; not in v0.1.0)" qualifier
7. **Resolve Convergence 6 (explain-page-not-found message)** — extend
   not-found stderr with a "see DESIGN-MDATRON.md § Reserved mdatron
   codes" help line
8. **CHANGELOG entry** — mdatron CHANGELOG.md per Keep-a-Changelog;
   Phase 2 changes under [Unreleased]

## Operator-directive housekeeping

Per the M5 F3+F9 audit-trail discipline (operator-directives recorded
forward, not retrospectively): Phase 2's opening commit declared the
intent to emit `OperatorDirectiveApplied{directive:
phase-2-explain-disposition-honored, ...}`. The directive is now
captured durably in:

- Phase 1a behavioral spec § Operator-directive housekeeping
- This Phase 4 routing doc § Convergence 1
- The Phase 2 closing commit message body (where the durable event-log
  record lives until `events.jsonl` ships in v0.1.x)

## Phase 4-take-2 (2026-06-08 follow-up)

The original Phase 4 (above) routed findings via prose only. Per the
operator-directive 2026-06-07 ("the point of VSDD is findings get routed
AND addressed"), Phase 4-take-2 operationalizes the dispositions:

1. **Filed all 129 #13 findings as crosslink subissues of `#13`** with
   `review-finding` + `classification:<status>` + `domain:<slug>` +
   `cycle:crosslink-13` labels. Same backfill applied to the 86 #12
   findings under `#12` (correcting the original Phase 4's prose-only
   record).

2. **Closed 83 of 215 issues across 9 waves** via 12 commits:
   - Wave 1 `mdatron@883eaaf` — 8 convergent fixes (argv validation,
     CARGO_BIN_EXE, format_tty + safe_display, doc-comment hygiene)
   - Wave 2 `mdatron@ba61527` — 15 README + explain-page polish
     (first-run sample output, install precision, opener
     differentiation, anchor links, $schema)
   - Wave 3 `mdatron@422c2dc` — 6 code quality + test tightening
     (codes.rs slice path, FieldNotFound removal, print_pipeline_error
     consolidation, README test heading-line match)
   - `vsdd-cli@91689e5` — routing-doc updates (initial)
   - Wave 4 `mdatron@f45200b` — 3 mdatron explain --json + structured
     ExplainPage (DE/F1 + AIE/F7 + QE/F5)
   - Wave 5 `mdatron@28e9794` + `vsdd-cli@fc98fa4` — 3 lint hardening
     (is_reserved_mdatron_code unstable doc, pre-commit version sanity,
     mdatron-test CI workflow)
   - Wave 6 `mdatron@6a0fe22` + `vsdd-cli@3aad175` — 6 DESIGN.md
     hygiene + planning-doc clarity + migration note for code rename
   - Wave 7 `vsdd-cli@5d02683` — 4 methodology.md amendments (cluster-
     batched default, Phase 2c skip discipline, single-milestone
     decomposition discipline, sycophancy-compensation placement)
   - Wave 8 `mdatron@f59c9f4` — 3 every/some over Null + README
     round-trip markers
   - Wave 9 `mdatron@cea00ab` — 1 compact-form explain catalog (AIE/F2)

   Plus an additional 22 closures (issues addressed by existing
   infrastructure: vsdd-cli's mdatron-verify.yml CI workflow + the
   verify_tty_does_not_render_explain_line_when_explain_ref_is_none
   test that pre-existed Wave 1) + 29 closures of `classification:
   resolved` issues already addressed by the original Phase 2 cycle's
   implementation commits.

3. **Filed 4 umbrella issues** for legitimate cross-phase deferrals:
   - **L1** — Methodology Phase 5 (formal hardening surface)
   - **L2** — events.jsonl writer + OperatorDirective event emission
   - **L3** — vsdd-cli overlay (domain-routing hints + compact catalog)
   - **L4** — Raise-to-SO pending operator dispositions

4. **Set block-by relationships** from 22 deferred orphans to umbrella
   issues + binary-first phase issues (`#15` Phase 4 collapse, `#17`
   Phase 6 publish).

5. **Created relate-links** across 8 cross-cycle convergence clusters
   (audit-trail discipline; schema-revert provenance; DESIGN doc
   hygiene; code rename operator bridge; catalog completeness;
   methodology meta-findings; CI/pre-commit hardening; test discipline).

## Phase 4 exit signal

```yaml
event: PhaseExited
phase: phase-4
exit_status: complete
layer: phase-2-mdatron-json
declared_at: 2026-06-07T00:30:00Z
findings_total: 215  # 86 crosslink #12 + 129 crosslink #13
findings_filed_as_crosslink_issues: 215
findings_closed: 83  # 52 accepted-now-closed + 29 resolved-already + 2 deferred-now-closed
findings_blocked_by_future_phase: 18  # deferred, gated on a named future-phase issue
findings_accepted_pending_operator_disposition: 10  # blocked-by L4 Raise-to-SO mostly
findings_dismissed: 96  # methodology-correct audit records
findings_hallucinated: 8  # methodology-correct audit records
findings_open_address_now: 0
domain_wide_dismissals: [accessibility, localization, privacy, performance-engineer]
umbrella_issues_filed: [L1, L2, L3, L4]
relate_clusters: [audit-trail, schema-revert-provenance, design-doc-hygiene, code-rename-operator-bridge, catalog-completeness, methodology-meta, ci-pre-commit-hardening, test-discipline]
phase_4_take_2_commits:
  mdatron: [883eaaf, ba61527, 422c2dc, f45200b, 28e9794, 6a0fe22, f59c9f4, cea00ab]
  vsdd-cli: [91689e5, fc98fa4, 3aad175, 5d02683]
next_phase: crosslink-13-close  # the #13 issue itself can now close once an operator-disposition pass completes the 10 L4 Raise-to-SO items
```
