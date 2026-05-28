<!-- hook-bypass[check-no-letter-clusters]: documentation of antipattern names per file scope -->
<!-- hook-bypass[check-document-staleness]: planning document; references in-flight + to-be-authored state by design -->

# vsdd-cli cleanroom restart — proposal

**Date:** 2026-05-28
**Author:** main session (Claude Opus 4.7)
**Status:** proposal — awaiting operator decision
**Slug status:** `CLEANROOM-RESTART-PROPOSAL.md` improvised; no approved slug exists in vsdd-suite for "planning report" artifact class — operator-flagged 2026-05-28
**Prior context:** [`review-log/2026-05-28-vsdd-methodology.md`](review-log/2026-05-28-vsdd-methodology.md) (12-defect retrospective) + [`review-log/2026-05-28-security.md`](review-log/2026-05-28-security.md) (identity-leak incident) + [`review-log/2026-05-28-upstream-audit.md`](review-log/2026-05-28-upstream-audit.md) (42-surface upstream conformance audit)

## Summary

Rename current `vsdd-cli` → `vsdd-cli-wip` (preserves all session work + git history + retrospective + 100+ crosslink issues + the 2 hooks just authored). Reset main to a pre-session-defect baseline + start `vsdd-cli` fresh from that baseline. Apply lessons from the retrospective + upstream audit from day 1; dogfood vsdd-suite hooks + crosslink discipline from first commit.

The current state has accumulated 12 distinct methodology-spirit defects, 22 Class (c) downstream inventions (some legitimate, some now-known-wrong), one identity-leak incident with remediation, and a paused state with pending decisions on cluster file splitting + intent removal documentation + FINDINGS-INDEX migration + hook deployment wiring. The cleanup work is substantial; the redo work from a clean baseline is comparable; the clean baseline avoids fighting legacy structures throughout.

## Baseline SHA proposal

**Recommended baseline: `fdb10d1`** — "track .crosslink/.gitignore (per crosslink convention; inner gitignore for agent files)" — committed 2026-05-27 17:21:05 -0700.

State at `fdb10d1`:
- `.claude/settings.json` (Claude Code config)
- `.crosslink/` substrate (full crosslink rules directory; driver-key; hook-config)
- `.crosslink/.gitignore` (inner gitignore for agent files — the commit that ratified `fdb10d1` itself)
- **Nothing else** — no DESIGN docs, no methodology.md, no README, no domain prompts, no supplements, no templates, no hooks, no review-log, no FINDINGS-INDEX

Rationale:
- Crosslink substrate is present + properly tracked (enables crosslink-mode from day 1)
- No vsdd-cli authored content exists yet — every methodology decision is fresh
- All 12 retrospective defects entered AFTER `fdb10d1` (earliest defect entry was at `b75345b` — Phase 1a methodology.md spec authoring with Defect 6 over-target)
- The pre-design-positioning content from the initial commit (`5ccf740`) is included via `fdb10d1`'s history
- No risk of carrying legacy methodology decisions forward

**Alternative considered: `b3d6e50`** — one commit earlier (crosslink init: deploy substrate). Materially identical state; `fdb10d1`'s only addition is the inner `.gitignore` for agent files. Either works; `fdb10d1` slightly cleaner.

**Alternative considered: `5ccf740`** — initial commit (pre-design positioning, before crosslink init). Forces re-deploying crosslink; loses substrate-deployment commit; no benefit.

## What carries forward in `vsdd-cli-wip`

The renamed directory preserves all current state + git history:

- All 25 commits from `5ccf740` → `5c7543a` (initial → identity-leak remediation)
- Plus uncommitted work this session:
  - 3 review-log entries (security, vsdd-methodology, upstream-audit)
  - DESIGN.md / DESIGN-METHODOLOGY.md / DESIGN-OBSERVABILITY.md / DESIGN-VERIFICATION.md edits (intent removal; Phase 5/6 mandatory; SO-prompt fix)
  - methodology.md / README.md edits (same)
  - Domain prompts (SO, SA, AI Engineer, Localization, VSDD-Methodology — intent removed; spirit substitutions)
  - supplements/rust.md edit (kani axis-gating removed)
  - templates/DESIGN.md.vsdd-template edits
  - hooks/check-no-letter-clusters.py + hooks/check-document-staleness.py (cleanroom Python implementations)
- crosslink local DB with 100+ session issues + #127 (SO prompt staleness)
- `.git/hooks/check-anonymization` (legacy hook shim)
- Auto-memory artifacts (in `~/.claude/projects/.../memory/`) — all forward-applicable to fresh vsdd-cli; not moved

## What carries forward into fresh vsdd-cli as lessons-applied

Not as files copied — as **decisions encoded as direction** for the fresh authoring:

**Methodology decisions** (from retrospective + upstream audit + this session):
- Project intent removed; per-feature axes are the sole calibration mechanism
- Phase 5 + Phase 6 are mandatory for VSDD (per upstream gist Core Principle 7) — no `not applicable — <rationale>` opt-out option in the template
- Phase 5 includes all 5 upstream surfaces: Proof Execution + Fuzz Testing + Security Hardening + Mutation Testing + Purity Boundary Audit
- Property-based testing belongs in Phase 2a, not Phase 5
- `safety-critical` + `formal-verification-candidates` axes do not exist
- FINDINGS-INDEX.md is an antipattern; crosslink-issue-board is the canonical index when crosslink-substrate is present
- README implementation-order roadmap is an antipattern; track/phase work flows through crosslink-tracked issues + milestones
- vsdd-cli is a cleanroom implementation of vsdd-suite (Layer 0 gist → Layer 1 vsdd-suite → Layer 2 vsdd-cli); measure conformance against Layer 1 + acknowledge specializations
- crosslink-mode is REQUIRED when crosslink-substrate is present; manual mode requires explicit operator-directive
- Composition discipline applies to ad-hoc in-session tooling (bash scripts, helper functions, Python utilities), not just to authored artifacts
- Adversary cognitive-diversity recommendation from upstream Section V is acknowledged (Claude-as-Adversary against Claude-as-Builder violates the diversity principle; defensible on tooling-availability grounds; flag explicitly)

**Naming + slug discipline:**
- review-log slugs follow vsdd-suite convention: `YYYY-MM-DD-<domain-slug>.md` where `<domain-slug>` is an official vsdd-suite domain identifier
- No letter labels for thematic groupings (Cluster A/B/C, Surface A/B, Path A/B, Option A/B) — descriptive identifiers carry meaning at point of use
- No Roman numerals for clusters (vsdd-cli specialization on top of vsdd-suite's hook)
- Approved abbreviations: `Dim N`, `Layer N`, `Round N`, `Phase N`, `Finding N`, `R8 F1`, domain slugs
- "Cycle" is not approved naming; use "Round" instead
- "Pillar" is not approved naming; use descriptive names

**Forward-only + stability discipline:**
- Pre-stability commitment (pre-v1.0 if that's the trigger, or operator-declared): malleable history; no migration pointers; no version bumps per amendment; no earned-by-recurrence-as-Accepted-gate
- Spec assertions about validation infrastructure require `spec_state:` markers (spec'd / implemented / deployed)
- No runbooks in spec text — methodology states rules, not actions

**Substrate + tooling:**
- 2 hooks already authored cleanroom (`hooks/check-no-letter-clusters.py` + `hooks/check-document-staleness.py`) — applicable from first commit of fresh vsdd-cli
- Adopt vsdd-suite's remaining 11 hooks (cleanroom Python re-implementations) per the immediate-pain ordering + then-by-need
- `.git/hooks/pre-commit` dispatcher runs all deployed hooks
- crosslink WARN messages must NOT be suppressed via `2>/dev/null` — the operator-attention discipline (per retrospective sideways-point assessment) is the load-bearing intervention
- `tracker_remote` configured to `local-only` from the start (or whatever explicit operator-directive sets)
- ssh-key comment uses `crosslink-agent:<id>@redacted-machine` form from the start
- anonymization-patterns registry seeded with the 4 patterns added this session (machine-hostname, ssh-key-comment-with-host, user-path-absolute, crosslink-tracker-remote-warn-dismissal)

## What starts fresh

- DESIGN.md (authored applying retrospective Defect 10 — Layer 0 prerequisite — from day 1; vsdd-suite-conformant axis declarations; no intent section; Phase 5/6 mandatory)
- methodology.md (no FINDINGS-INDEX prescription; crosslink-required-when-present; pre-stability discipline explicit; all 5 Phase 5 surfaces named)
- README.md (no FINDINGS-INDEX references; no inline implementation-order roadmap; crosslink-first throughout; vsdd-suite cleanroom-implementation framing prominent)
- Domain prompts (fresh authoring against vsdd-suite spec; SO Dim 2 correctly framed from day 1 — axes calibrate composition, not intent)
- Supplements (cleanroom implementations of vsdd-suite supplements per language)
- Templates (cleanroom implementations of vsdd-suite scaffold)
- crosslink substrate state — fresh `crosslink init` (the 100+ wip issues stay in `vsdd-cli-wip`)
- review-log (empty; first entries follow vsdd-suite slug discipline)
- COMPATIBILITY.md authored at the start (capturing intent removal as deliberate vsdd-cli deviation from vsdd-suite + Adversary cognitive-diversity note + any future deviations)

## Execution sequence

```
1. confirm baseline SHA: fdb10d1 (operator approval pending)
2. cd <parent-dir>   # the directory containing vsdd-cli
3. mv vsdd-cli vsdd-cli-wip   # sibling rename
4. cd vsdd-cli-wip && git checkout -b wip-archive
5. git push -u origin wip-archive   # archive branch preserves all current main history
6. git checkout main
7. git reset --hard fdb10d1
8. git push --force-with-lease origin main   # main now at clean baseline at origin
9. cd ..
10. git clone <github-url-redacted>/vsdd-cli.git vsdd-cli   # fresh clone of the now-reset origin/main
11. cd vsdd-cli — start applying lessons:
    a. crosslink init   # fresh init (confirmed; no DB import from vsdd-cli-wip)
    b. Author DESIGN.md applying retrospective + upstream audit decisions
    c. Author methodology.md (no FINDINGS-INDEX; crosslink-required-when-present; all 5 P5 surfaces)
    d. Author README.md (vsdd-suite cleanroom framing; no inline roadmap)
    e. Author COMPATIBILITY.md documenting deliberate deviations from vsdd-suite
    f. Copy hooks/check-no-letter-clusters.py + hooks/check-document-staleness.py from vsdd-cli-wip
    g. .git/hooks/pre-commit dispatcher wired to run hooks + check-anonymization
    h. First crosslink issue: "Layer 0 dogfooding bootstrap" tracking the fresh-start sequence itself
12. Reference vsdd-cli-wip/review-log/2026-05-28-{vsdd-methodology,upstream-audit,security}.md + vsdd-cli-wip/CLEANROOM-RESTART-PROPOSAL.md from fresh vsdd-cli's DESIGN.md + methodology.md as historical context
```

## Confirmed decisions (operator-directive 2026-05-28)

1. **Remote handling — (a) hard-reset + archive.** Hard-reset `origin/main` to baseline + force-push; archive current state at `origin/wip-archive`. Loses public-history audit-trail of current main but preserved on `wip-archive`. Acceptable because vsdd-cli is pre-stability per the discipline established this session.

2. **crosslink — fresh init.** `crosslink init` from scratch in the fresh vsdd-cli. The 100+ wip issues stay in `vsdd-cli-wip`. Note: `fdb10d1` already has `.crosslink/` substrate (rules + driver-key + hook-config + .gitignore) so the fresh init may detect existing substrate — operator may need to `rm -rf .crosslink/` first to get true fresh init, OR keep the rules + re-create the DB.

3. **vsdd-cli-wip directory location — sibling.** `<parent-dir>/vsdd-cli-wip` alongside the fresh `<parent-dir>/vsdd-cli`.

## Still-open questions for operator

1. **Confirm baseline SHA `fdb10d1`.** Or pick alternative (`b3d6e50` / `5ccf740` / other).

2. **The 2 already-authored hooks.** Confirm copy from `vsdd-cli-wip/hooks/` into fresh vsdd-cli's first commit (vs re-author). They're cleanroom-correct Python implementations; re-authoring is redundant.

3. **Auto-memory.** The session memories at `~/.claude/projects/.../memory/` all transfer forward to fresh vsdd-cli (same operator + same project location); no migration needed. Confirm no memory should be deleted.

4. **vsdd-cli-wip's `.git/hooks/check-anonymization` legacy shim.** Stays with vsdd-cli-wip (already there). Fresh vsdd-cli installs its own dispatcher when wiring up hooks.

5. **Approved-name registry.** This report's slug (`CLEANROOM-RESTART-PROPOSAL.md`) is improvised. After restart, fresh vsdd-cli should either author a `docs/` directory for planning artifacts OR find the vsdd-suite-approved location for one-off planning reports. Resolution deferred.

6. **crosslink fresh-init pre-step.** Does the fresh vsdd-cli (starting from `fdb10d1` state which already has `.crosslink/`) need a `rm -rf .crosslink/` before `crosslink init` for a truly fresh DB? Or is the substrate-without-issues OK? Recommend `rm -rf .crosslink/` + `crosslink init` for an actually-fresh substrate.

## Lessons distilled (one-line per defect class from retrospective)

For fresh vsdd-cli authoring reference:

1. **Recursive naming-discipline failure** — no new letter labels, ever; no Roman numerals as cluster IDs; descriptive identifiers at point of use
2. **Premature stability-discipline application** — no forward-only pointers, no version bumps, no earned-by-recurrence gates pre-stability
3. **Phase-skip willingness** — Phase N requires prior Phase N-1 exit signal OR explicit operator-directive override
4. **Improvising at scale → compound bash + crosslink errors** — stop on compound substrate-tool errors; read the substrate-tool documentation; do not improvise corrections
5. **Composition discipline violation** — load primer + composed-domain prompts + relevant supplement BEFORE phase work AND before authoring ad-hoc in-session tooling
6. **Author-normalized inflation against tracked target** — three consecutive tracked-deviation annotations on same artifact = methodology-spirit violation; revise target or trim artifact
7. **Site-specific fix declared closure** — defect-class closure requires both same-author grep-clean AND next-Phase-3-Round adversarial verification
8. **Operator-facing communication discipline** — descriptive finding titles in operator-facing summaries; never finding-ID shorthand without first-use expansion
9. **Manual-mode operation without declaration; crosslink not used when substrate was present** — crosslink-mode REQUIRED when `.crosslink/agent.json` exists
10. **VSDD methodology not adhered to in vsdd-cli's own development (dogfooding gap)** — apply the methodology to vsdd-cli from day 1, not as v1.0 goal
11. **Schema validation infrastructure asserted as available before implementation existed** — `spec_state:` markers (spec'd / implemented / deployed) on every assertion of validation infrastructure
12. **Methodology specified manual finding-aggregation when crosslink should have been substrate throughout** — substrate-as-source-of-truth principle; no manual files duplicating crosslink-tracked entities

## Effort estimate

Authoring fresh DESIGN.md + methodology.md + README + COMPATIBILITY.md + first crosslink-tracked issues + COMPATIBILITY notes: ~1 working session of focused work.

For comparison: cleaning up the current vsdd-cli-wip state would require splitting 5 cluster files into per-domain files (~8 files), authoring file-level bypasses on ~7 review-log + domain-prompt files, fixing the genuine staleness flags, fixing the README `Surface A`, rewriting the FINDINGS-INDEX-dependent surfaces, retroactively re-scoping intent removal as a documented deviation, splitting the upstream audit into a vsdd-suite-measured re-audit, and so on. Probably ~2 working sessions.

The fresh start is the lower-cost path AND produces a cleaner artifact.

## Recommendation

Proceed with `fdb10d1` as baseline using option (2b) above (fresh-start branch preserves public history). Total estimated time including this proposal review + decisions + fresh authoring: ~1.5 working sessions.
