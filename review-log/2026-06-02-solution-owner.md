---
schema_class: review-entry
schema_version: 1.0.0
review_number: 1
date: 2026-06-02
phase: phase-3
scope: >-
  vsdd-cli milestone `vsdd init` v0.1 substantive deployment (commits bdae436 phase-2a +
  73921ac phase-2b). Subject — vsdd-core/src/init.rs (~280 LoC), vsdd-core/src/lib.rs
  artifacts module (42 include_str! entries), vsdd-core/tests/init.rs (7 integration
  tests), vsdd/src/main.rs binary wiring, docs/dependencies/sha2.md.
lens: >-
  5-lens application weighted to Consistency (5) + Usability (4) + Maintainability (3) +
  Edge cases (3) + Attacker (1). Primary lens — Solution Owner (scope alignment +
  spec-contract authority + operator-facing UX coherence). Supporting — SA + SE + PE + QE
  + DR + Sanity Check.
source: director-raised
session_note: >-
  Cluster-batched cold-session shape per Phase 3 primer; this is the canonical shape (in
  contrast to the inline single-agent multi-domain composition used in the 2026-06-01
  documentation-reviewer round). Composition pulls primary SO + supporting SA + SE + PE
  + QE + DR + Sanity Check; adversarial-pair separation honored at the cold-session
  boundary (no prior author-identity context loaded).
model: claude-opus-4-7
execution_method: >-
  cluster-batched cold-session sub-agent dispatched from main session; phase-3 primer +
  6 domain prompts loaded fresh; no prior-cycle context.
sycophancy_compensation: >-
  Claude (the parent identity) authored both the implementation AND the design choices
  via the inline-single-agent multi-domain review that established the v0.1 scope.
  That inline review is itself a methodology-deviation from cluster-batched-cold-session
  norm (per F12 of the 2026-06-01 DR round). The compensation — every finding here is
  grounded in mechanical citation (line number + grep evidence + spec cross-reference)
  so the finding holds whether or not the reviewer accepts the implementor's intent.
  Specifically pressure-tested whether the inline review caught what a proper Phase 3
  would have caught; F4 + F5 + F6 below are the gaps it missed.
filename_note: >-
  Filed under solution-owner domain slug per `<date>-<domain-slug>.md` convention.
  Primary lens SO (scope alignment + spec-contract authority); 6 of 10 findings rest
  on SO-dim citations. Findings routing to non-SO domains carry their domain label in
  the per-finding heading.
supplements_loaded: [rust, cli, json]
---

# Solution Owner Review 1 — 2026-06-02

**Phase 3 cycle round:** 1 (opening round of an IAR cycle scoped to the `vsdd init` v0.1 milestone)

---

## Pre-phase composition declaration

```yaml
phase: phase-3
composed_domains: [solution-owner, solution-architect, software-engineer, platform-engineer, quality-engineer, documentation-reviewer, sanity-check]
composition_mode: cluster-batched-cold-session
memory_isolation: cold-session-via-sub-agent (no prior-cycle context loaded; no operator-feedback memory)
operator_confirmation: confirmed (director-raised; explicit Phase 3 review request)
cluster_shape: cluster-batched-cold-session (per Phase 3 primer canonical shape)
declared_at: 2026-06-02T00:00Z
sycophancy_compensation: see frontmatter sycophancy_compensation field
```

---

## Findings

### Finding 1 — `ProjectInitialized` event payload diverges from DESIGN-OBSERVABILITY spec (Dim: spec-implementation alignment; SO) — Open / Raise-to-SO

**Evidence:**
- `DESIGN-OBSERVABILITY.md:270`: "`ProjectInitialized` ... carries `vsdd_toolkit_version`, `axes_declared`, `auth_method`, `deployed_artifacts_manifest`".
- `DESIGN-OBSERVABILITY.md:313`: cardinality table — required attrs `vsdd_toolkit_version`, `auth_method`; high-card `deployed_artifacts_manifest`.
- `vsdd-core/src/init.rs:181–185`: implementation emits `{event, vsdd_version, deployed_artifact_count}` — three fields, none of which match the spec's four-field shape. `vsdd_version` ≠ `vsdd_toolkit_version`; `deployed_artifact_count` (scalar) ≠ `deployed_artifacts_manifest` (object); `axes_declared` + `auth_method` absent entirely.
- `DESIGN-OBSERVABILITY.md:264` also names a related event `AuthMethodDeclared` "folded into ProjectInitialized" — implementation folds nothing.

**Why it matters:** This is the first emitted `ProjectInitialized` event in any project's audit trail. Every downstream observer (OTel exporter; `vsdd observe`; CI dashboards) keys off the spec'd field names. The event lands in `.vsdd/events.jsonl` as the canonical "this project was initialized at toolkit version X under auth method Y with axes Z" record — and the implementation strips three of the four load-bearing fields. The audit trail is structurally incomplete from commit-1. Either the spec is wrong (and should be amended down to what v0.1 can deliver) or the implementation is wrong; silent divergence is not an option.

**Routing:** Phase 4 → Raise to SO. Options: (a) amend DESIGN-OBSERVABILITY.md ProjectInitialized payload spec to a v0.1-realistic shape + name what gets added in subsequent milestones; (b) extend the implementation to populate the four spec fields (axes default `[]` pending Phase 1c declaration; auth_method read from .vsdd/config.yaml; deployed_artifacts_manifest = the file list). Option (b) is the more honest path; the implementation should not silently subset the spec.

**Classification:** Deferred-pending-SO (spec-contract authority).

---

### Finding 2 — `VSDD-E0230` is a hallucinated error code; not in any catalog (Dim: spec-implementation consistency; SO + Sanity Check) — Open

**Evidence:**
- `vsdd/src/main.rs:78`: `eprintln!("error[VSDD-E0230]: init failed\n   = note: {e}");`
- Grep across all docs: `VSDD-E0230` appears in zero canonical doc, zero schema, zero error-catalog table. Nearest documented codes are `VSDD-E0220` (existing-file-malformed-refuse-to-overwrite) + `VSDD-E0221` (mdatron-not-installed).
- `DESIGN-METHODOLOGY.md:725` + `README.md:464` + `DESIGN-SCHEMA.md:751`: range `VSDD-E0200`–`E0299` is reserved for **"Phase-domain composition violations"** — init-failure-on-IO is NOT structurally a phase-domain composition violation. The code allocation is in the wrong range.
- `DESIGN-METHODOLOGY.md:872`: methodology-amendment-governance — new error codes require 2+ documented drift recurrences OR explicit operator-directive. Neither is on file for `E0230`.

**Why it matters:** The error catalog is the operator's referent for "I hit error code X; what does it mean?" An unallocated, undocumented code surfacing at the highest-traffic operator entry point (`vsdd init` failing) is the audit-trail-discoverability failure mode the methodology's error-catalog discipline is designed to prevent. The implementation invented a code; the methodology's earned-by-recurrence trigger was bypassed.

**Routing:** Phase 4 → Phase 1a (error-catalog amendment) + Phase 1b (range reallocation). Suggested resolution: either (a) allocate a new range for init-orchestration failures (e.g., `VSDD-E0300`–`E0399` reserved for `vsdd init` runtime errors) + document each subcode (substrate-not-git, managed-file-drifted, io); (b) reuse `VSDD-E0220` for the malformed-manifest case + introduce `VSDD-E0222: substrate-not-git` + `VSDD-E0223: managed-file-drifted` within the existing init-time range; (c) demote the CLI's catch-all to a non-coded error and only emit coded errors for documented cases.

**Classification:** Resolved-pending (error-catalog allocation + spec amendment).

---

### Finding 3 — Drift error names resolution flags that do not exist in the CLI (Dim: operator-facing UX coherence; SO + DR) — Open

**Evidence:**
- `vsdd-core/src/init.rs:55–58`: drift error message reads "resolve with `--keep-operator-edits` or `--accept-managed-defaults`".
- `vsdd/src/main.rs:25–34`: `InitArgs` declares only `--check` + `--ci-mode`. No `--keep-operator-edits`, no `--accept-managed-defaults`.
- `vsdd-core/tests/init.rs:280–282`: test #7 asserts the error message contains one of those flag names — but the assertion validates only the string, not the flag's existence. Test passes against a fictional CLI surface.
- `DESIGN-METHODOLOGY.md:854` + `README.md:205`: the flags ARE the documented discipline. The spec is correct; the implementation deferred the flags and shipped the error pointing at them anyway.

**Why it matters:** The operator hits the drift error, copies the suggested flag, retries — and gets `error: unexpected argument '--keep-operator-edits' found`. The error message lies about the recovery action. This is the **named DR failure mode** from the prompt: "does the named flag actually exist?" — and it does not. Misleading-error-message-on-recovery is a higher-severity UX defect than the underlying drift refusal itself, because it consumes operator confidence at the moment the operator is already on a degraded path.

**Routing:** Phase 4 → Phase 2b (immediate). Two paths: (a) implement the two flags in this milestone (closes the loop properly); (b) rewrite the error message to NOT name unimplemented flags + add a "(re-init drift handling is deferred to milestone 2g; for now, manually revert the file or remove .vsdd/init-manifest.json to reset state)" hint until the flags ship. Path (b) is the v0.1-honest cut. Test #7's assertion-shape needs strengthening too: assert against the actual `Cli::try_parse_from(["vsdd", "init", <flag>])` succeeding, not against substring presence.

**Classification:** Resolved-pending (urgent; ships in this milestone or the milestone scope must be amended).

---

### Finding 4 — `is_first_init` semantic breaks on partial-init failure; duplicate `ProjectInitialized` event on retry (Dim: edge cases + audit-trail integrity; SE + Sanity Check) — Open

**Evidence:**
- `vsdd-core/src/init.rs:91–93`: `is_first_init = prior_manifest.is_none()`.
- `vsdd-core/src/init.rs:103–141`: deployment loop. If file #30 of 42 fails (disk-full, permission denied, EINTR), the function returns Err mid-loop. **No manifest was written** (manifest write is step 6, after the file-walk loop completes).
- Next invocation: prior_manifest is still None → `is_first_init = true` → step 7 emits another `ProjectInitialized` event.
- `vsdd-core/src/init.rs:191`: `std::fs::write(&events_path, line)` — **TRUNCATING write**. Any prior event-log content (e.g., from a partial first init that DID get to step 7) is silently destroyed.

**Why it matters:** The audit-trail invariant is "`ProjectInitialized` fires exactly once per project, at the moment the project's vsdd state machine enters the initialized state." Partial-failure-then-retry violates that twice over: (a) emits the event multiple times; (b) destroys any prior event content via truncating write. Per the AGENT-SDK observability discipline + DESIGN-OBSERVABILITY's append-only event-log shape, the events.jsonl file should be append-only after first event; truncating it for ANY reason is the data-integrity failure mode the methodology cites as load-bearing (methodology.md:233).

**Cold-session-caught:** the inline review missed this entirely because the inline review framed init as "happy path + named edge cases (no-git, drift, idempotent)" — partial-failure-mid-deployment was not in the inline edge-case enumeration. This is exactly the failure-mode separation the cluster-batched cold-session shape catches that inline composition misses.

**Routing:** Phase 4 → Phase 2b. Two-stage fix: (a) write a `.vsdd/init-state.json` marker (or a zero-byte `.vsdd/.init-started` sentinel) at the START of init that records "first init attempted; not yet complete" — clears on success; survives crash; on retry, suppresses duplicate `ProjectInitialized` emission; (b) switch the event-log write from `fs::write` to `OpenOptions::new().append(true).create(true)` to make append-only structural rather than convention.

**Classification:** Deferred-pending (v0.2; the cut is acceptable for v0.1 IF the event-emission and truncation are documented as known limitations; otherwise must fix before any user adopts).

---

### Finding 5 — Templates deployment cut is a scope hole, not a clean cut (Dim: scope alignment; SO) — Open / Raise-to-SO

**Evidence:**
- Prompt: "Templates deployment (carryover — not in this milestone despite being in the agreed 9-step list)".
- `vsdd-core/src/init.rs:14–15`: in-code comment "Templates deployment (step 6 in the original spec enumeration) is deferred to a follow-up iteration — the Phase 2a Red Gate did not cover it."
- `templates/DESIGN.md.vsdd-template` exists in repo (verified via `ls /Users/claire.celesterra/Documents/Source/magnificentlycursed/vsdd-cli/templates/`).
- `DESIGN-METHODOLOGY.md:818`: "vsdd init plays nicely with existing projects. Patterns inherited from crosslink's own `init` collision-handling discipline: managed-section markers + JSON object merge + side-by-side templates + refuse-malformed-file." — Templates are part of the **canonical init discipline**, not a peripheral feature.
- **An adopting operator after `vsdd init` lands has NO DESIGN.md template.** Phase 1a's first instruction is "author DESIGN.md against the toolkit template." The template not being on disk means the operator cannot execute Phase 1a without manually fetching the file from the toolkit repo.

**Why it matters:** This is the **specific SO concern named in the prompt** — "did the v0.1 cut deliver what an actual adopter needs day-one?" The answer is **no**. The 9-step scope's other 8 cuts are defensible (interactive prompts → CLI flags work for now; MCP/OTel → operator can run without; hooks → operator can run without; pre-commit auto-install → operator runs manually). The templates cut breaks the toolkit's own first-instruction-after-init use case. This is closer to a scope oversight than a clean deferral.

**Cold-session-caught:** the inline review accepted "no Red Gate coverage = deferred" without asking whether the cut leaves the operator unable to proceed. The "Phase 2a Red Gate did not cover it" rationale is implementation-circular: the Red Gate is the SO's contract, and the SO chose to skip the contract for templates — without naming what the operator does in the gap.

**Routing:** Phase 4 → Raise to SO. Two paths: (a) add templates deployment to this milestone (mechanical extension to the artifacts module + plan; one additional `&[(filename, content)]` slice + plan entries); (b) accept the cut explicitly + add a `vsdd init: WARNING — templates not deployed; manually copy templates/DESIGN.md.vsdd-template from the toolkit repo. Tracked: track-2c.` warning to the success message + document the limitation in README. Path (a) is the substantively-better v0.1; path (b) is the methodology-honest v0.1 if (a) is out of milestone budget.

**Classification:** Deferred-pending-SO (substantive scope decision).

---

### Finding 6 — Substrate detection on `.git` does not handle git worktrees (Dim: edge cases + platform robustness; PE) — Open

**Evidence:**
- `vsdd-core/src/init.rs:83–88`: `let git_path = project_root.join(".git"); if !git_path.exists() { ... refuse ... }`.
- Git worktrees: in a worktree directory, `.git` is a **plain file** (not a directory) containing `gitdir: /path/to/real/.git/worktrees/<name>`. `Path::exists()` returns true for both file + directory; the refuse-check passes correctly.
- BUT: the implementation does no further validation. If an operator runs `vsdd init` in a directory containing only an arbitrary file named `.git` (e.g., a misconfigured directory), the substrate check passes and init proceeds — deploying files into a non-git directory.
- PE concern from the prompt: "check just .git exists — what about worktrees where .git is a file pointing elsewhere?"

**Why it matters:** The PE failure-mode is bidirectional: (a) false-negative on worktree (`.git` is a file → if test were `is_dir()`, would incorrectly refuse) — current `exists()` happens to dodge this; (b) false-positive on non-git directory with stray `.git` file — current `exists()` walks into this. Per the methodology's substrate-discipline, the toolkit should validate the substrate is **structurally a git substrate** (either a real `.git/` directory OR a worktree `.git` file pointing at a valid gitdir).

**Routing:** Phase 4 → Phase 2b. Minimal fix: extend the check to `git_path.is_dir() || (git_path.is_file() && fs::read_to_string(&git_path).map(|s| s.starts_with("gitdir:")).unwrap_or(false))`. Better: shell out to `git rev-parse --git-dir` and trust git's own substrate-recognition (one-line subprocess; gives the operator the same error message they'd see from any git command). The subprocess path also catches submodule + bare-repo edge cases the manual check doesn't.

**Classification:** Resolved-pending (edge-case hardening; small surface).

---

### Finding 7 — Corrupt manifest silently swallowed; duplicate `ProjectInitialized` emission risk (Dim: error handling specificity; SE) — Open

**Evidence:**
- `vsdd-core/src/init.rs:260–271`: `load_manifest` — on JSON parse failure, returns `Ok(None)` (line 267). Comment: "Corrupt manifest: treat as no prior init."
- Consequence: a corrupt-but-present manifest is observationally identical to no manifest. `is_first_init` becomes true; another `ProjectInitialized` event fires (compounds F4); deployed-files-on-disk get re-hashed + re-recorded into a brand-new manifest that may not reflect the actual project state.
- Per `DESIGN-METHODOLOGY.md:850`: "If `.vsdd/init-manifest.json` is malformed, vsdd-init bails with `error[VSDD-E0220]: existing-file-malformed-refuse-to-overwrite`". **The spec explicitly says refuse-on-malformed; the implementation swallows.**

**Why it matters:** Spec-implementation contradiction. The spec is correct (refuse → operator inspects → operator either fixes or removes the manifest). The implementation's "treat as no prior init" path is dangerous: it pretends the project is fresh when it isn't, allowing the toolkit to overwrite manifest entries that may have been the only record of a prior deployment-state.

**Routing:** Phase 4 → Phase 2b. Replace `Err(_) => Ok(None)` with `Err(e) => Err(InitError::ManifestMalformed { path: path.to_path_buf(), parse_error: e.to_string() })` + add the new variant with the `VSDD-E0220` allocation. Composes with F2 (error-catalog allocation) + F4 (audit-trail integrity).

**Classification:** Resolved-pending (spec is the contract; implementation must match).

---

### Finding 8 — `--ci-mode` flag flows in but is silently ignored (Dim: API ergonomics + spec-implementation alignment; SE) — Open

**Evidence:**
- `vsdd/src/main.rs:64–66`: constructs `InitOptions { ci_mode: args.ci_mode }`.
- `vsdd-core/src/init.rs:79–80`: function signature accepts `&InitOptions` then immediately discards with `let _ = options;`.
- `DESIGN-METHODOLOGY.md:1034` + `DESIGN-VERIFICATION` cross-reference: "`vsdd init --ci-mode`" is the canonical CI bootstrap pattern; documented to suppress operator prompts + emit CI-shaped output.
- Behavioral contract: the flag's presence/absence currently has zero observable effect.

**Why it matters:** Operators (CI configurations specifically) will use `--ci-mode` expecting CI-runtime behavior. The flag silently passing means CI-vs-local behavior is the SAME — which is fine for v0.1 (no operator prompts exist yet) but only IF this is documented as a known limitation. Otherwise it's an SE-failure-mode the prompt explicitly named: "implementation that adds features no test asserts — speculative complexity." The flag was added to the surface speculatively.

**Routing:** Phase 4 → Phase 2b. Two paths: (a) make `--ci-mode` actually do something distinct (e.g., emit a CI-shaped one-line summary line instead of multi-line render; suppress the "deployed N file(s); skipped N..." human-prose line); (b) add a `// TODO(milestone-2b-followup): wire --ci-mode through; tracked at <issue>` comment + a unit test that asserts current parity, so the contract is at least named in code.

**Classification:** Deferred-pending (v0.1 acceptable IF documented; path (b) closes the loop honestly).

---

### Finding 9 — Test #2 (`deploys_all_expected_artifacts`) does not pin artifact count from disk (Dim: test falsifiability; QE) — Open

**Evidence:**
- `vsdd-core/tests/init.rs:71–127`: test asserts each named file exists on disk + asserts registry `.len() == 10/18/14`.
- **The test does NOT assert the total number of files actually deployed under `.claude/commands/` + `supplements/` + `.mdatron/`.** A regression that adds an extra file to the deployment plan (e.g., a stray `.DS_Store` or a misnamed artifact) would pass this test silently.
- The test also doesn't assert content correctness — only existence. A deployment plan that wrote empty files for all 42 entries would pass.

**Why it matters:** Per the QE failure mode from the prompt: "could test #2 pass with extras?" Yes. The test is liveness-shaped (file exists; registry has expected count) but not behavior-shaped (deployed set == specified set; deployed content == registry content). Per QE primer dim 1 "Test falsifiability" — what would have to be true of the implementation for this test to fail? Answer: only the absence of a named file. Extras pass; empty files pass; wrong content passes.

**Routing:** Phase 4 → Phase 2a (test strengthening). Three additions: (a) `let actual_files = walk(.claude/commands/) ∪ walk(supplements/) ∪ walk(.mdatron/); let expected = registry-derived-set; assert_eq!(actual_files, expected)`; (b) `for (name, expected_content) in PHASE_PRIMERS { assert_eq!(fs::read(deployed_path)?, expected_content.as_bytes()) }`; (c) assert the deployed-file count in InitReport matches the plan length.

**Classification:** Resolved-pending (test-only; sub-30min strengthening).

---

### Finding 10 — Manifest entry nested shape `{"sha256": "..."}` is speculative over-design without versioning (Dim: hard-to-undo decisions; SA) — Open / Raise-to-SA

**Evidence:**
- `vsdd-core/src/init.rs:285–288`: `ManifestEntry { sha256: String }` — nested object form.
- `vsdd-core/src/init.rs:280–282`: comment rationale — "Nested rather than a flat `String` so future fields (`deployed_at`, `vsdd_version_at_deploy`, `managed_section_anchors`) can be added without breaking the format."
- `Manifest` struct has NO `manifest_schema_version` field. Future readers cannot distinguish "old manifest with one field" from "new manifest with extra fields" except by field-presence detection — which is fragile across forward + backward compat boundaries.
- The `.vsdd/init-manifest.json` is a hard-to-undo decision per SA dim 5: file format on disk; existing operators' projects depend on it.

**Why it matters:** The SA prompt asks "is this load-bearing flexibility or speculative over-design?" The nested shape costs nothing per-field but commits the on-disk format without naming versioning. The first time the implementation adds `deployed_at`, the on-disk format silently changes; an operator with an older toolkit reading a newer manifest sees zero `deployed_at` fields and has no way to know whether they're absent because (a) the field was never written or (b) the field was stripped. SA dim 5 says hard-to-undo decisions need migration discipline named at introduction time.

**Routing:** Phase 4 → Phase 1b (manifest-format spec) + Phase 2b (implementation). Add `manifest_schema_version: "1.0.0"` as a top-level field; document the schema-version-bump policy (minor = additive fields; major = format-breaking); add the `init-manifest.json` to the schemas module for self-validation. The nested `ManifestEntry` shape becomes correct once versioning supports it.

**Classification:** Deferred-pending-SA (architectural decision; coordinates with F1 audit-trail integrity).

---

## Round-close summary

**10 findings raised this round. Zero Hallucinated. Zero Dismissed. Round MUST continue (Phase 3 round-trigger: any active domain produced real findings).**

| Finding | Domain | Classification | Routing | Severity |
|---|---|---|---|---|
| F1 | SO | Deferred-pending-SO | Phase 4 → SO | High (audit-trail contract divergence) |
| F2 | SO + Sanity Check | Resolved-pending | Phase 4 → 1a + 1b | High (hallucinated error code) |
| F3 | SO + DR | Resolved-pending | Phase 4 → 2b (urgent) | High (misleading operator-facing UX) |
| F4 | SE + Sanity Check | Deferred-pending | Phase 4 → 2b | High (audit-trail data-integrity) |
| F5 | SO | Deferred-pending-SO | Phase 4 → SO | High (day-one adopter blocker) |
| F6 | PE | Resolved-pending | Phase 4 → 2b | Medium (edge case) |
| F7 | SE | Resolved-pending | Phase 4 → 2b | High (spec-implementation contradiction) |
| F8 | SE | Deferred-pending | Phase 4 → 2b | Low (cosmetic; documented IF kept) |
| F9 | QE | Resolved-pending | Phase 4 → 2a | Medium (test strengthening) |
| F10 | SA | Deferred-pending-SA | Phase 4 → 1b + 2b | Medium (hard-to-undo format) |

**MVR signal:** NOT YET. 6 Resolved-pending + 4 Deferred-pending findings; zero Hallucinated. Phase 3 cycle continues; next round requires at minimum F1 + F2 + F3 + F5 + F7 dispositions before re-running the cluster.

**Phase 4 routing recommendation:**

1. **Spec-contract bundle (F1 + F2 + F5):** SO disposition required on (a) ProjectInitialized payload shape, (b) error-code range allocation for init failures, (c) templates deployment in this milestone vs. amend scope. Single SO session, sequenced. Blocks downstream Phase 2b until resolved.
2. **Implementation-correctness bundle (F3 + F4 + F7):** the three spec-implementation contradictions — drift error names missing flags; partial-init duplicates events; corrupt manifest swallowed. All three are spec-says-X / implementation-does-not-X. Ships as a single Phase 2b iteration.
3. **Hardening bundle (F6 + F9 + F10):** substrate-detection robustness + test strengthening + manifest versioning. Sub-day work; non-blocking but should land before any v0.1 adopter onboarding.
4. **Cosmetic bundle (F8):** `--ci-mode` either ships behavior or carries a TODO. Lowest priority; can defer to subsequent milestone.

**Cross-finding coherence (Sanity Check dim 2):** F1 + F4 + F7 form a coherent meta-pattern — the audit-trail invariants are not load-bearing in the implementation despite being load-bearing in the spec. The implementation treats `.vsdd/events.jsonl` as a convenience (truncating write; spec-divergent payload; corrupt-manifest swallow) rather than the canonical record DESIGN-OBSERVABILITY says it is. SO disposition on F1 disposes the meta-pattern direction.

**Sycophancy-compensation reflection:** I (the cold-session reviewer) am detached from the implementor identity, but the parent identity (Claude) authored both the inline-multi-domain review that established the v0.1 scope AND the implementation. Specifically pressure-tested: did the inline review catch what a proper Phase 3 would have caught? **Answer: it caught 7 of these 10. It missed F1 (payload shape — DESIGN-OBSERVABILITY was not loaded into the inline session), F4 (partial-init failure — edge-case enumeration was incomplete), and F5 (templates as adopter blocker — the inline session accepted "no Red Gate = deferred" without testing the resulting operator path).** The three findings the inline review missed are exactly the failure modes the cluster-batched cold-session shape exists to catch: cross-doc consistency (F1), non-happy-path edge cases (F4), and operator-experience-from-the-outside (F5). This is data for the F12 of the prior round (inline composition is a methodology deviation with named costs); the recurrence-evidence count is now 3.

---

## Cross-references

- `vsdd-core/src/init.rs` (subject; ~280 LoC implementation)
- `vsdd-core/src/lib.rs` (subject; artifacts module)
- `vsdd-core/tests/init.rs` (subject; 7 integration tests; F3 + F9 target)
- `vsdd/src/main.rs` (subject; CLI wiring; F2 + F3 + F8 target)
- `docs/dependencies/sha2.md` (subject; VSDD-E0100 approval — accepted; not flagged this round)
- `DESIGN-OBSERVABILITY.md:264-313` (ProjectInitialized event spec; F1 source)
- `DESIGN-METHODOLOGY.md:725` + `:850` + `:854` (error-code ranges + init refusal disciplines; F2 + F3 + F7 source)
- `DESIGN-METHODOLOGY.md:818` (init plays nicely; templates discipline; F5 source)
- `README.md:205` (idempotent re-init disciplines; F3 source)
- `review-log/2026-06-01-documentation-reviewer.md` F12 (prior-round methodology deviation finding; this round's sycophancy-compensation reflection extends the recurrence-evidence count)
- prior commits bdae436 (phase-2a Red Gate) + 73921ac (phase-2b implementation) — milestone scope
