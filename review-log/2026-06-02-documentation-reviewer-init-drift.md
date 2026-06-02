---
schema_class: review-entry
schema_version: 1.0.0
review_number: 2
date: 2026-06-02
phase: phase-1c
scope: >-
  vsdd init drift / collision / upgrade surface for v0.1 — operator-facing
  error text in vsdd-core/src/init.rs ManagedFileDrifted; the matching test
  assertion in vsdd-core/tests/init.rs:280-281; and the spec passage
  DESIGN-METHODOLOGY.md:816-862 (Adoption into existing projects — collision
  handling). Substrate reference: crosslink/src/main.rs:117-144 (Init flag set
  —update / --force / --no-prompt / --dry-run).
lens: >-
  DR cold-read with sanity-check baseline. DR dims weighted: cold-context
  discoverability (1), spec-vs-impl drift (2), cross-reference resolution (3),
  three-audience effectiveness (4). Sanity-check rubber-ducks the question
  "what does the operator do when this error fires?"
source: operator-directive
session_note: >-
  Cold single-domain single-session. Read the four required priming docs +
  cited spec/impl/test sites + crosslink Init flag definitions; no prior-cycle
  memory. Opinion-only — no code or doc changes proposed for direct merge;
  Raise-to-SO recommendation framed as DR cold-reader concern. DR-lens bias
  toward fewer-load-bearing-terms acknowledged + pressure-tested per the
  operator's framing.
model: claude-opus-4-7
execution_method: >-
  inline main session (single-agent single-domain DR with sanity-check
  baseline; no worktree isolation; under-800-word opinion shape)
sycophancy_compensation: >-
  DR-lens bias is "delete the made-up flag names; declare the vocabulary cut
  the win; ship." That's the easy answer; it leaves the operator at a drift
  error with no resolution path named. The pressure-test below names the floor
  the error text has to clear regardless of which flag-vocabulary path SO
  picks.
filename_note: >-
  Suffixed `-init-drift` to disambiguate from the 2026-06-01 documentation-
  reviewer entry on the canonical-doc naming surface. Single-topic review;
  single domain.
---

# Documentation Reviewer — vsdd init drift handling (v0.1)

**Subject:** how v0.1 should treat drift / collision / upgrade, with focus on the operator-facing error surface and the spec-vs-impl flag-name asymmetry at DESIGN-METHODOLOGY.md:854.

## What the operator sees today (cold-read walkthrough)

The drift error message reads (init.rs:56-57):

> `managed file drifted at <path>; resolve with --keep-operator-edits or --accept-managed-defaults (expected sha256 <hex>, got <hex>)`

Cold operator at `vsdd init` re-run, post-edit-of-a-managed-file: they get told to "resolve with" two flags. They `vsdd init --help`. The flags are not there. They `vsdd --help`. Still not there. They grep the binary — nothing. The error told them to do a thing they cannot do. Three-audience reads:

- **First-time adopter:** "is this tool broken?" — they hit a dead end with no escape hatch named. Worst-case: they delete `.vsdd/init-manifest.json` to make it stop, silently disarming the drift detection.
- **v0.1 → v0.2 upgrader:** same dead end + the upgrade-context primes them to suspect they're missing a flag added in v0.2 — they read changelog, find nothing, file an issue.
- **CI failure investigator:** sees the error in logs, greps the repo for the flag name, finds the spec at :854 referencing it, concludes the impl is behind spec, files the wrong bug.

All three audiences are mis-served. The error text is **load-bearing-by-default** — operators reach it precisely when they have no other recourse.

## Spec-vs-impl drift: the spec hallucinated

DESIGN-METHODOLOGY.md:818 promises "Patterns inherited from crosslink's own `init` collision-handling discipline." :854 then names `--keep-operator-edits` / `--accept-managed-defaults` as the discipline's flag vocabulary. Neither name exists in crosslink (which ships `--update` / `--force` / `--no-prompt` / `--dry-run` per main.rs:117-144). The inheritance promise is rhetorical, not mechanical. This is exactly the failure mode the 2026-06-01 DR review F11 catches at a different site: the spec follows the letter (cites the substrate, names flags) and violates the spirit (the flags it names do not exist in the substrate it claims inheritance from).

## Vocabulary discipline — pressure-test of the DR-lens easy answer

Crosslink's four verbs (`init` / `--update` / `--force` / `--no-prompt`) carry mental models the cold operator can already hold. `--keep-operator-edits` / `--accept-managed-defaults` add two new verbs whose *function* (resolve a drift conflict) is not what crosslink's verbs do (orchestrate an upgrade). They are not synonyms; they are a separate vocabulary axis. Piling them on is a real learning-load cost.

**But:** the DR easy answer ("delete the flag names from the error string, ship v0.1") cuts the wrong load-bearing term. The flag names are placeholder; the *resolution-path-must-be-named* invariant is not. The cold operator at the drift error needs **some** named path forward.

## Recommendation (Raise to SO)

1. **Spec amendment (DESIGN-METHODOLOGY.md:854):** strike the made-up flag names. Replace with the literal v0.1 contract: "v0.1 refuses on drift; operator either restores the file to the manifest-recorded hash OR removes `.vsdd/init-manifest.json` to opt out of drift detection for the next run. Three-way merge + interactive resolution deferred to v0.2." This honors the inheritance-from-crosslink claim by *naming the divergence* rather than papering over it — crosslink ships `--update` for the upgrade path; vsdd v0.1 does not, and that delta is the doc's job to surface.

2. **Error-text rewrite (init.rs:56-57):** replace the flag-naming form with a path-naming form. Cold operator needs to read: *what happened, where, what to do, what NOT to do.* Concretely: "managed file drifted at <path>: vsdd-deployed content was modified outside `vsdd init`. v0.1 refuses to overwrite. Either restore the file to its manifest-recorded sha256 (<expected>) or delete `.vsdd/init-manifest.json` to opt out of drift detection. Interactive resolution arrives in v0.2."

3. **Test fix (tests/init.rs:280-281):** assert on the resolution-path text (`init-manifest.json`, `restore`, `v0.2`), not on flag names. Current assertion is false-green by construction — it tests that the error string contains the string the spec hallucinated; both spec and impl agree on a name no operator can act on.

4. **Three-audience check post-fix:** the rewrite serves all three — adopter sees the v0.1 boundary, upgrader sees the v0.2 promise, CI investigator sees the operator-action and the toolkit-action separated.

**Classification:** Deferred-pending-SO. Raises to SO because (a) spec amendment + (b) v0.2 commitment ("interactive resolution arrives in v0.2") is a roadmap claim, not a DR-resolvable prose tweak. Composes with the 2026-06-01 DR-review F11 (recursive self-violation pattern).

**MVR signal:** N/A — opinion only; no code or doc changes proposed for direct close.
