---
title: "MDATRON CAPABILITY SURFACE"
tags: ["reference", "design-doc"]
sources: []
contributors: ["xqjG"]
created: 2026-08-02
updated: 2026-08-02
---


## Design Specification

### 1. cli surface

| Command | Purpose | Flags |
|---|---|---|
| `mdatron verify` | validate the governed corpus | `--project-root <DIR>`, `--schemas <DIR>`, `--patterns <DIR>`, `--files <GLOB>...` (ad-hoc jurisdiction), `--json` (envelope 2.1.0), `--compact` (512-byte agent blocks), `-q/--quiet`, `--changed <FILE>` (incremental: file + transitive dependents; a `.mdatron/` change falls back to whole-tree), `--deny-warnings` (alias `--strict`: warnings-only run exits 1) |
| `mdatron explain [CODE]` | per-code prose (rustc `--explain` pattern); accepts short codes (`E0050`); rejects ANSI/shell-meta injection | `--list` (full catalog, `code — summary`), `--json` (structured page), `--compact` (one line: `<code> <severity>: <summary> — <first fix sentence>`) |
| `mdatron pin` | verify the sha256 pin record | `--update` (recompute + rewrite `.mdatron/pins.yaml`), `--dry-run`, `--project-root`, `-q` |
| `mdatron init` | scaffold `.mdatron/` skeleton + managed manifest; idempotent; refuses a hand-modified managed file with E0060 | `--project-root`, `-q` |
| `mdatron schema` | print the published `verify --json` envelope JSON Schema on stdout — binary-only consumers can pin/validate without a repo checkout; lockstep with `mdatron_output_version` | — |

**Exit codes** (`verify`): `0` clean, `1` findings (errors; or warnings under `--deny-warnings`), `2` pipeline failure.

---

### 2. check-family inventory — every code in the 0.5.0 catalog (30 codes)

The envelope reports five named families: `schema`, `route`, `pin`, `vocabulary`, `citation`. Layer-2 pattern rules emit **adopter-namespace** codes (e.g. `VSDD-E0201`); `MDATRON-*` is reserved for the engine. Everything below is verbatim-grounded in `mdatron explain --list` + the explain pages at the tag (`src/explain/MDATRON-*.md`).

### Frontmatter / schema family (Layer 1)
| Code | Name | Fires when |
|---|---|---|
| E0001 | frontmatter-parse-failed | the YAML frontmatter block does not parse; Layer 1+2 stop for the file (cascade suppression) |
| E0002 | schema-class-unknown | frontmatter declares a `schema_class` matching no schema in `.mdatron/schemas/` |
| E0050 | frontmatter-schema-violation | frontmatter parsed and bound to a known class but violates its JSON Schema (message names the field path + failing constraint) |
| W0040 | governed-file-has-no-frontmatter | a file matching `require_frontmatter` globs carries no frontmatter block at all (opt-in loudness; closes the loud-parse-failure / silent-absence asymmetry) |
| W0045 | schema-class-unrouted | a declared `schema_class` matches no schema AND no rule `context` while validation infrastructure exists — since 0.5.0 also fires under a present-but-**empty** `schemas/` dir |
| W0047 | schema-dir-missing | `.mdatron/schemas/` is absent AND a walked file declares a `schema_class` nothing serves (project-level companion to W0045; the two never double-report) |

### Path confinement (applies to every adopter-supplied path: `keys:` sources, routes, pins, citations)
| Code | Name | Fires when |
|---|---|---|
| E0010 | key-source-absolute-path | an adopter path is absolute — rejected on path text, before any filesystem access |
| E0011 | key-source-parent-traversal | a path contains a `..` segment (lexical check; even segments resolving back inside the root) |
| E0012 | symlinked-component-refused | resolving a path hits a symlink at any depth (no-follow `openat`/`O_NOFOLLOW`; swap-proof guarantee is Unix-only) |

### Route family (`.mdatron/routes.yaml` — closed-world governance allowlist)
| Code | Name | Fires when |
|---|---|---|
| E0030 | unrouted-file | routes are supplied and a walked file is claimed by no route |
| E0031 | governing-document-absent | a route's `governed_by` document cannot be opened inside the tree |
| E0032 | route-conflict | two or more routes' `files` globs claim the same file (conflict is an error, never first/last-match) |
| W0041 | name-underivable | a filename is not derivable from its route's `naming` grammar (linear-time regex) |

### Pin family (`.mdatron/pins.yaml` — sha256 governance edges)
| Code | Name | Fires when |
|---|---|---|
| E0061 | pin-stale | a pinned file's content no longer matches the recorded sha256 (the attention loop: re-read the governing doc, then `pin --update`) |
| E0062 | pin-target-missing | a pin names a file that cannot be opened inside the tree |
| W0042 | governance-weakening-unjustified | an `unpinned:` tombstone lacks `reason`/`owner` |
| L0001 | governance-weakening-standing | a **justified** tombstone stands — fires on every whole-tree run as a standing record, not a defect |

### Vocabulary family (`.mdatron/vocabulary.yaml` — the naming register, scoped by `vocabulary_globs`)
Each registry section activates its own scan independently (`src/vocab.rs`): `terms` → E0090+E0092, `label_schemes.allow` → E0091, `anti_patterns` → E0093, `numeric_claims` → E0094. Absent registry = family inactive.

| Code | Name | Fires when |
|---|---|---|
| E0090 | unregistered-coinage | a **bold-introduced** term in governed prose is absent from the `terms` registry (draft-status terms exempt — registration can lag deliberately, but visibly) |
| E0091 | invented-label-scheme | a letter-plus-number cluster matches no `label_schemes.allow` regex (the letter-cluster incident mechanized) |
| E0092 | reserved-word-misuse | a `reserved`-status term appears at all — reserved means held for a future sense, not usable; every use surfaces for review |
| E0093 | register-anti-pattern | prose matches a listed `anti_patterns` regex (phrasings ruled out of the project register); matched text rides a quoted region |
| E0094 | numeric-claim-drift | a prose numeral (digits or word-numbers) restates a `numeric_claims`-configured frontmatter field's count and disagrees with it — configured field references only, never free inference |
| W0043 | vocabulary-scope-matches-nothing | `vocabulary.yaml` present but `vocabulary_globs` matches no walked file — the whole register scans nothing, loudly (an *empty/absent* list is not this warning: that falls back to all walked files) |
| W0044 | vocabulary-term-status-conflict | a term declared both `registered` and `draft`; resolves to draft (permissive) with a warning |

### Citation family (per-route opt-in: `citations: true` in `routes.yaml`)
`file:line` / `file:start-end` citations verified against the **working-tree snapshot** — uncommitted content counts, no git subprocess. Historical corpora simply don't opt in.

| Code | Name | Fires when |
|---|---|---|
| E0100 | dead-citation | a citation names content that does not exist (motivating evidence: 7 of 8 review findings citing absent code) |
| E0101 | citation-line-out-of-range | target exists but the cited line/range end lies past its last line, or the range is malformed (start>end, line 0; 1-based) |

### Jurisdiction / pipeline / governance infrastructure
| Code | Name | Fires when |
|---|---|---|
| E0060 | managed-manifest-drift | a file `mdatron init` deploys and tracks in `.mdatron/manifest.yaml` (sha256) was hand-modified; `init` refuses rather than overwrite. Guards **only** the engine-owned managed partition — adopter schemas/patterns/config content are outside it |
| E0070 | project-root-unresolved | the project root cannot be resolved before the pipeline opens (bad cwd, no `--project-root`) |
| E0080 | pipeline-orchestration-failure | the verify pipeline itself failed to complete (no per-file findings emitted); under `--json` the `pipeline_error` object disambiguates the sense — see §3 |
| W0046 | jurisdiction-glob-matches-nothing | a `file_globs` entry matches zero files — a dead glob silently shrinks the corpus otherwise; once per pattern, whole-tree runs only |

---

### 3. the `--json` envelope at 2.1.0

Published schema: `$id: https://github.com/magnificentlycursed/mdatron/schema/mdatron-output/2.1.0`; resolvable copy via `mdatron schema` and `schema/mdatron-output.schema.json`. Draft 2020-12, strict (`additionalProperties: false` throughout).

**Top level** (all REQUIRED except `pipeline_error`):

```
{
  "mdatron_output_version": "2.1.0",      // const per release
  "mdatron_version": "0.5.0",
  "pipeline_status": "ok" | "failed",
  "pipeline_error": { code, kind, message },   // ONLY when failed; survives --quiet
  "summary": { error_count, warning_count, lint_count, files_checked },
  "families": { schema, route, pin, vocabulary, citation },   // closed object, all five required
  "findings": [ ... ]
}
```

**`pipeline_error.kind`** — closed enum, 10 values: `config` (jurisdiction/config load), `io`, `schema_load`, `pattern_load`, `glob`, `frontmatter`, `index_build`, `expr_parse` (incl. an over-deep expression past MAX_EXPR_DEPTH), `eval`, and **`bound_exceeded`** (new in 2.1.0, #124: a declared input resource bound — per-file bytes, aggregate bytes, or structural nesting — exceeded). Disambiguates the ~10 senses the single E0080 code conflates, so a consumer branches on failure class without parsing prose. In-band delivery exists because `--json --quiet` (the CI mode) previously got `findings: []` with no cause.

**`families.<name>` tri-state** — each of the five families is `{ "state": "active" | "inert" | "inactive", "reason": "<string>" }`:
- `active` — data supplied and the check ran this pass (invoked, not necessarily fired)
- `inert` — configured but did no work (e.g. `vocabulary.yaml` present but `vocabulary_globs` matched no walked file)
- `inactive` — not configured

The `reason` string makes the audit signal falsifiable (the 2.0.0 reshape that fixed "families was unfalsifiable"). **This is the gateable required-family surface:** a CI leg can assert `families.vocabulary.state == "active"` and fail when a required family silently stopped running.

**`summary.files_checked`** — the number of files this run **VALIDATED** (ran per-file schema + rule checks on), not a count of files that produced findings. A clean run over N files reports N; an empty jurisdiction reports 0. This is the non-vacuity caveat: exit 0 with `files_checked: 0` is "checked nothing", not "checked clean" — a gate should assert the count, not just the exit code. (The pre-0.4.0 stub counted finding-referenced files, so clean == 0 == vacuous; fixed in #105; overlapping-glob double-counting fixed in #109.)

**`findings[]`** — closed object, required: `code`, `severity` (`error|warning|lint`), `summary`, `message`, `help` (nullable), `location` (`{file, line, column}`), `explain_ref` (nullable — emitted only when the binary can actually explain the code; `null` for adopter-namespace codes, never a dead pointer). Optional `quoted[]`: adopter-derived text carried out-of-line, each region `{label, content, origin: "adopter" (const), trusted: false (const)}` — the marking is a serialize-time constant of the type, so no code path can emit adopter content marked trusted. Adopter text NEVER rides inline in an engine-authored line (message placeholders render `[see: <label>]` pointing at the block).

**Path discipline** — every envelope path is project-root-relative and forward-slashed: the same repo produces a **byte-identical** envelope on Unix and Windows; `pipeline_error.message` is guaranteed host-layout-free via a chokepoint strip (0.5.0 #134/#140/#142).

**OUTPUT_VERSION discipline** (`src/output.rs`) — the envelope is a versioned contract per SemVer: additive change (new optional field, new enum value) = MINOR (2.0.0→2.1.0 for `bound_exceeded`); breaking (removed/renamed/reshaped field, type change, new REQUIRED field under a closed object) = MAJOR (1.1.0→2.0.0 for the families reshape + required `origin`/`trusted`). CI tripwires: emitted envelope validates against the published schema, schema version tracks `OUTPUT_VERSION` in lockstep, three output forms agree on findings, every production code literal resolves in the explain catalog, and the committed code-semantics golden (`schema/code-catalog.json`) fails on any meaning change without regeneration. Under `--json`, findings are NOT double-rendered to stderr (suppressed since 0.4.0 — a ~1.7x token saving for agents capturing both streams).

---

### 4. resource bounds (0.5.0, #124)

Enforced with a loud diagnostic; each exceedance is a `pipeline_error`, replacing silent degradation / unbounded memory / an uncatchable parser abort:

| Bound | Value | Surfaces as |
|---|---|---|
| `MAX_FILE_BYTES` | 8 MiB (per file) | `pipeline_error.kind: "bound_exceeded"` |
| `MAX_AGGREGATE_BYTES` | 64 MiB (aggregate snapshot) | `bound_exceeded` |
| `MAX_STRUCTURAL_NESTING` | 256 | `bound_exceeded` |
| `MAX_EXPR_DEPTH` | 256 (DSL expression nesting) | `kind: "expr_parse"` (a bounded ParseError, not the stack-overflow it prevents) |

Constants at `src/verify.rs:173-175`, `src/dsl/expr_parser.rs:81` (tag state). Related hardening in the same release: adopter JSON-Schema `pattern`s compile on a **linear-time** regex engine (jsonschema 0.49 + `PatternOptions::regex()`, ReDoS closed; look-around/backreferences now refused at schema compile); route/vocab regexes were already linear (`regex-lite`).

### 5. pin machinery

`.mdatron/pins.yaml` shape (from mdatron's own dogfood record):

```yaml
pins:
- governing: DESIGN.md          # the document that pins
  file: src/codes.rs            # the governed file — any file, not just markdown
  sha256: f1697301…             # content hash at ratification
unpinned:                       # tombstones for deliberate un-pinning
- file: …
  reason: …                     # absent reason/owner → W0042
  owner: …                      # justified tombstone → standing L0001 on every whole-tree run
```

- Check mode: bare `mdatron pin` (and every `verify` run checks the pin family). Stale hash → E0061; unopenable target → E0062.
- Re-pin: `mdatron pin --update` recomputes every pin in one command (`--dry-run` previews); commit the re-pin with the governing-doc amendment — the commit is the review record. Writes are atomic (temp + fsync + rename, 0.5.0).
- The record cannot pin itself; its integrity anchor is commit review. `owner` is unauthenticated by construction — commit review is the operative control.
- **WHOLE-FILE only today.** Pin granularity is the entire file's sha256; a governing doc cannot pin one section of a large living file without going stale on every unrelated edit. Section-scoped (heading-delimited span) pins are mdatron **#146** (raised from vsdd's live need: the build-plan pinning one section's hash by hand).

### 6. `verify --compact` (agent-context form)

One block per finding, blank-line separated, **hard-capped at 512 bytes per finding** (contract limit, DESIGN §Output / #80 D4; measured real-corpus findings 315/246 bytes). Engine-authored head line: `E[CODE] file:line:col summary — message`. Adopter content rides prefix-marked beneath (`=label:` + `> ` quoted lines via the partition renderer) — the same untrusted-marking property as the JSON `trusted: false`. Budget priority: quoted **value** ahead of message prose (a placeholder can never outlive the value it points at); truncation drops whole lines from the tail, closes cut regions with an engine elision marker, never cuts mid-line or mid-escape; since 0.5.0 prose cuts retreat to a word boundary. Pipeline failures render compact too. Companion: `explain --compact` for hook/context-budget hot paths.

### 7. jurisdiction model

- Jurisdiction is **never guessed** (#80 D1): an absent or globless `.mdatron/config.yaml` REFUSES (`no jurisdiction declared`, E0080/`kind: config`, exit 2) — no silent `**/*.md` walk.
- `config.yaml` keys: `file_globs` (the walked corpus), `require_frontmatter` (globs where a frontmatterless file fires W0040), `vocabulary_globs` (scopes the whole vocabulary family; empty/absent = all walked files; zero-match = W0043).
- `--files <GLOB>...` is the explicit escape hatch: an ad-hoc jurisdiction declared on the command line, needing no config.
- Dead `file_globs` entries are loud (W0046); overlapping globs dedupe (no double-count).
- `--changed <FILE>`: incremental verify of the file + transitive dependents (governance, rule-reference, shared-key edges) with full cross-file context; `.mdatron/` changes and unresolvable paths fail safe to whole-tree.
- **E0060 semantics and the self-audit boundary:** `init`'s managed manifest guards only engine-owned deployed files. `config.yaml` is *seeded but adopter-owned* — editing `file_globs` (or invoking `--files`) is sanctioned adopter action that E0060 does not and cannot detect. This is the jurisdiction self-audit bypass vsdd tracks as #855 residual (1): an agent can shrink the corpus it is judged against; the control direction is a vsdd-side diff-scoped check on jurisdiction-narrowing commits, not an mdatron feature.

### 8. dsl: current expressiveness and limits (`docs/dsl-reference.md` at tag)

One lane by contract: **cross-file and registry integrity over frontmatter**. What exists: `let:` chained bindings (declaration order); context selectors (schema_class, path glob, or ANDed object); `.`-chained field access with Null-propagation; `==`/`!=` deep equality; `in`/`not_in`; short-circuiting `and`/`or`/`not`; quantifiers `every`/`some`/**`filter`** (since 0.3.0 — `filter` composes with `count`/`len` for exactly-N / at-least-N arity over frontmatter arrays); functions `count`, `len`, `defined`, `union`, `intersect`, `difference`, `concat`, `join`, `key` (cross-file index lookup; miss = Null); `keys:` declarations building path-confined cross-file indices (md/yaml/json sources, array fan-out, last-wins collisions). Evaluation errors (type/arity/non-boolean assert) are loud pipeline failures, never a silent pass. `message:` interpolation never inlines the value — it renders a `[see: label]` reference to a quoted block.

**Deliberately absent:** body-content access (headings, links, tables), regex/string-extraction functions, arithmetic and ordering comparisons (equality only — no `<`/`>`/`+`), floats, expression-typed array literals. Precision note: "no filter/count predicates" is *closed* for frontmatter arrays as of 0.3.0; the standing gap tracked as mdatron **#149** is the next tier of predicate expressiveness — the examples ratified there are body-scoped ("exactly one open-phase H3 in `## Requirements`") and cross-section disjointness, which today's frontmatter-only inventory cannot encode at all.

### 9. the vsdd raise pipeline: mdatron #145–#149

All five triaged 2026-08-02 from vsdd's post-0.5.0 roadmap feedback (their GH #20); dispositions on mdatron's tracker.

| Issue | Ships | Unlocks for vsdd |
|---|---|---|
| **#145** (high; SCOPE RATIFIED) | A **6th check family: links** — dead relative body-link targets (E0110), dead anchors vs GitHub-slug headings (E0111); inline `[t](target)`, `[t](target#anchor)`, `[t](#anchor)`; per-route `links: true` (mirrors citations); confinement-held. Version ruling: ONE deliberate major — envelope 2.1.0→**3.0.0** (new required families member) + crate 0.6.0, AND `families` becomes forward-extensible so future families (#147/#148) land additive/minor. Reserved range E0110–E0119 | Body cross-references in the contract/design corpus become mechanically verified — today only frontmatter slugs are (VSDD-E0201–E0208); a renamed heading or moved doc silently breaks design prose links. Also means vsdd's envelope consumer must be ready for a 3.0.0 major |
| **#146** | Section-scoped content pins (heading-delimited spans) extending whole-file `pin` | The live vsdd practice — build-plan pinning ONE section's hash — becomes engine-checked instead of hand-rolled; governing docs can pin the clause they ratified without going stale on unrelated edits |
| **#147** | Marker-line reference rules: a line matching a declared pattern (e.g. `Provenance: <name>`) must resolve to an existing heading/member name | vsdd's provenance-line discipline (design members citing their source) gets a name-anchor sibling of `citations: true` — anchors by name, not line number |
| **#148** | Adopter-namespace code-catalog integrity: every cited adopter code (VSDD-XNNNN) resolves to a declared entry | vsdd's own diagnostic namespace gets the every-code-resolves guarantee mdatron gives itself via the golden catalog; a design doc citing a nonexistent VSDD code becomes a finding |
| **#149** | DSL filter/count-with-predicate at body/section grain ("exactly one open-phase H3 in ## Requirements", "phase ids disjoint between two sections") | Structural build-plan invariants (single open phase, disjoint phase ids) move from convention/manual review into Layer 2 |

### 10. vsdd-cli: wired today vs available-unwired

**Live envelope evidence** (run 2026-08-01, this repo): `pipeline_status: ok`, `files_checked: 79`, families — `schema: active`, `vocabulary: active`, `route: inactive` (no routes.yaml), `pin: inactive` (no pins.yaml), `citation: inactive` (no route opts in).

### Wired today
- **Jurisdiction** (`.mdatron/config.yaml`): 4 `file_globs` (vsdd-* commands, supplements, registry data sets, review-log); same 4 under `require_frontmatter` (W0040 armed); `vocabulary_globs` scoped to the 3 enduring-artifact globs (frozen review-log walked but unscanned, mirroring mdatron's own ruling).
- **Schema family**: 13 JSON Schemas in `.mdatron/schemas/` (domain-prompt, phase-primer, supplement, review-entry, the registry data-set classes, state/snapshot/statusline/gate/economics/dispatch, installed-artifact-manifest, composition-scope-and-actions, act-to-affordance-map).
- **Layer-2 patterns**: 11 rules across 2 files — `cross-references.yaml` (VSDD-E0201–E0208: validator-pair/supplement/domain/phase slug resolution via 3 cross-file indices, primer-id agreement, not-self validator) and `registry-integrity.yaml` (VSDD-E0210–E0212: manifest pairs_with/referenced_by resolution, statusline recovery-action vocabulary membership).
- **Vocabulary family, 2 of 4 sections armed**: `label_schemes.allow` (E0091 — 5 schemes: VSDD/MDATRON codes, H1-6, OSC-n, JSONn) and `anti_patterns` (E0093 — deprecated coinages "forcing seam", "chassis", "substrate").
- **Pre-commit** (`.githooks/pre-commit`): fail-closed on missing binary (exit 2, operator ruling #658); 0.5.x version window check (warn-only); whole-tree `mdatron verify --project-root .` when md/schema/pattern files are staged.
- **CI**: `.github/workflows/mdatron-verify.yml` — crates.io install pinned `--version 0.5.0 --locked`, plain `mdatron verify` on PR + main push; `vsdd-test.yml` installs the same pinned binary for tests that invoke it; template `vsdd-verify.yml` deploys the same pattern to adopters.

### Available but UNWIRED (the raise surface inward)
1. **FLAGSHIP — families tri-state gating (#855)**: CI runs plain `verify` and gates only on exit code. The 2.1.0 envelope makes "a required family actually ran" assertable TODAY — `--json` + assert `families.vocabulary.state == "active"` (and `schema`) closes the silent-family-death gap without waiting on anything upstream. Currently no consumer of `--json` exists anywhere in vsdd's wiring.
2. **`files_checked` non-vacuity assertion**: nothing checks the count; exit 0 over a hollowed-out jurisdiction is indistinguishable from clean. Pairs with (1) as the two halves of the checked-N-vs-checked-nothing signal vsdd itself commissioned (0.4.0 review items 1/4).
3. **`--deny-warnings`**: shipped in 0.5.0 at vsdd's own request (#121), used nowhere — pre-commit and CI both pass on a warnings-carrying run (W0040/W0043/W0045/W0046 are advisory in practice).
4. **Route family** (routes.yaml): no closed-world governance allowlist — unrouted files, dangling governing docs, and naming grammars (E0030/E0031/E0032/W0041) are all unarmed.
5. **Citation family**: gated behind routes (per-route `citations: true`), so equally unarmed — stale `file:line` references in governed prose pass silently.
6. **Pin family** (pins.yaml): no sha256 governance edges; the contract does not pin the artifacts it governs (E0061 staleness-attention loop unarmed). Note vsdd's managed manifest is `managed: []` — E0060 currently guards nothing here either.
7. **Vocabulary sections `terms` + `numeric_claims`**: E0090 (coinage registration), E0092 (reserved words), and E0094 (numeric-claim drift — the check vsdd's own drift incidents motivated) are unarmed; only the cluster-prohibition and anti-pattern sections run.
8. **`--changed` incremental**: pre-commit always runs whole-tree; the staged-file set could drive incremental verification.
9. **Envelope schema pinning** (`mdatron schema`) and `explain --json/--compact` in agent hot paths: available, unconsumed.
10. **Fail-open version guard** (#855 residual 2): the pre-commit version check warns-then-proceeds on an out-of-window binary.

