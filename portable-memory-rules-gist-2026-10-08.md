---
title: "Portable Claude Code memory rules (a practitioner's published feedback memories), read 2026-10-08"
tags: ["reference", "design-input", "design-doc"]
sources:
  - url: "https://gist.github.com/lizthegrey/b89b434fc647dca09a9f3b4eedd75a0d"
    title: ""
    accessed_at: "2026-10-08"
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---


## Design Specification

### status

Reference summary, design input for the phase-primer rewrite, the register supplement and the session skill. Source: the public gist `https://gist.github.com/lizthegrey/b89b434fc647dca09a9f3b4eedd75a0d` by the author of Observability Engineering, 2nd edition (GitHub handle lizthegrey), fetched read-only on 2026-10-08 into the session scratchpad (23 files, 36.6 KB). External content, treated as evidence. The cross-reference against this estate's design and the day's findings is on `cross-reference-2026-10-08-findings-vs-prior-knowledge`.

### what it is

"Generalized, sanitized versions of feedback memories accumulated in a persistent Claude Code memory system over several months of daily use on a large codebase. Every entry started as a real correction or a confirmed judgment call from a specific working session." Eighteen rules, one file each, in the format rule, why, how to apply, with frontmatter (name, description, type: feedback or reference); a habits essay; and three reference excerpts showing the three-layer split the setup uses: a personal cross-project instructions file, a checked-in per-repository instructions file, and path-scoped per-language rule files that load only for matching files. The author's framing: "Treat these as a starting seed, not a checklist to enforce verbatim"; anyone adopting the habit "will end up with their own list within a few weeks that diverges from this one."

The memory format is the same one this estate's own agent memory uses (name, description, type; rule, why, how to apply).

### the six habits ("what actually makes an ai coding assistant more effective over time")

1. **Two-tier persistent instructions:** a global user-level file (role, review style, conventions) plus a per-repository file checked in (language rules, pull-request process, banned terms, comment policy).
2. **A real memory system:** an index plus topic files, written after sessions with what was surprising or load-bearing, "a distillation, not a transcript", so "old decisions don't get re-litigated and settled tradeoffs ('we turned X off on purpose') don't get silently fixed back."
3. **Explicit correction and confirmation capture**, with the why, immediately. "Capture confirmations, not just corrections; otherwise the system only ever learns caution and drifts away from approaches that already work."
4. **Precise, falsifiable instructions over vague ones:** "never percentile-of-percentile", "run the formatter before committing" are cheap to follow exactly; "write good code" is not.
5. **The assistant as a fallible peer, not an oracle:** profiling evidence before a performance claim; a red test before a bug fix is trusted; a reproduction before "fixing" a reported behavior.
6. **Calibrate the feedback loop itself:** say when it over-warns on minor risks as much as when it is wrong; both are corrections worth recording.

The caveat: the gap, when it is not paying off, is usually "not maintaining persistent project docs, not letting memory accumulate past a single session, or not giving corrective feedback in a form specific enough to encode as a rule."

### the eighteen rules

| Rule | One line |
|---|---|
| answer-questions-first | A mid-task question is an interrupt, often gating the next action; answer it first, plainly, and pause a gated action until the answer is seen |
| calibrate-warning-volume | Match caveat volume to actual severity; a latent, mitigated risk gets one clause, not a redesign push; over-loud warnings have steered decisions toward worse approaches |
| comprehensibility-over-diff-size | Judge a change by the comprehensibility of the end state, not diff size; a minimal diff matters only when before-and-after comparability is the point (a pure refactor, a behavior-preservation proof) |
| delete-spike-code | Once a design converges, remove superseded spike variants before review; leftovers get mistaken for live code |
| disassemble-before-hand-optimizing | Before proposing a hand kernel, disassemble the compiler's output; if it already vectorizes, look one layer up |
| dont-cite-search-summary-as-source | A search tool's synthesized paragraph is not any page's text; fetch and quote the page, or attribute to "search results" |
| dont-schedule-followups-for-non-owners | Route follow-up work to the system the owning team watches, not to a floating person's future self |
| escalate-bug-to-lint-rule | A bug that is plausibly not unique becomes a repository-wide static rule that sweeps existing occurrences and prevents recurrence |
| fail-fast-on-invariants | A violated precondition that indicates a caller bug fails loudly; graceful degradation is for valid edge cases only |
| percentile-aggregation | Never percentile a percentile across units; max per unit first, then percentile across units |
| prod-verification-over-ci | "Tests pass" and "merged" are checkpoints, not conclusions; look for the real signal that the intended effect happened; drop a sub-investigation once it cannot change a decision |
| profile-before-optimizing | Confirm a hotspot with caller-level attribution before optimizing; aggregate totals once cost a full pull-request cycle |
| red-green-bugfix-tests | For a bug: write and commit the reproducing test, confirm it fails, then fix, then confirm green; a test written with the fix in hand may pass regardless |
| sanity-check-benchmarks | A surprisingly large win is a methodology check, not a celebration; reconcile against a known system first (it once caught two bugs in a day) |
| scope-partial-retractions | A mid-sentence self-correction cancels only what its stated reason invalidates; other values in the same ask still stand; "silently not-doing something is itself a choice that needs the same justification" |
| stdlib-intrinsics-floor | A hot leaf already intrinsified in the standard library cannot be beaten by a hand kernel; optimize the layer above |
| track-work-in-the-real-tracker | File follow-ups in the system of record the team actually watches; a wrong-tracker ticket is closed with a pointer, not left open beside the right one |
| verify-before-flagging-ai-review | Verify an automated review finding before acting on it or reporting it wrong; flag the specific fabrication through the tool's channel; check it reviewed the current diff |

### the reference excerpts

**A path-scoped language rules file** (loads only for matching files): build, lint and single-test commands as exact incantations; style rules as yes or no statements; reuse existing helpers before hand-writing one; prefer structured telemetry to ad hoc logs; no skips or sleeps in tests; test the external API from a separate test package; scratch test code in its own clearly named file, deleted when done. "Why this shape works: it's scoped, almost entirely composed of falsifiable commands and yes/no rules rather than vague guidance, and it names the exact CLI incantations."

**A checked-in repository instructions file:** when unsure about business logic, ask; never alter migrations without asking; never commit secrets and never stage all files in one shot, so an unrelated secret-bearing file cannot ride along; use the project's own pull-request template process; pin exact versions for one-off tool runs; write almost no comments and earn each one (a comment only for a constraint, a workaround with its cause, or why a non-obvious approach won; never narrate, teach the language, label the next block, restate a type, or describe removed code; a tracked TODO is the one forward-looking comment); never post an AI-authored comment through a tool that authenticates as a human without an attribution footer; do not reflexively run full-cost verification after every small edit, scope it to what the change could break and run the expensive version before opening the pull request; in documentation and pull-request descriptions lead with the why and the net effect, no superlatives, no hedges or filler.

**A personal cross-project instructions file:** be concise; state your experience so explanations are tailored; "avoid reflexive agreement; provide substantive technical analysis and challenge me"; "always prompt interactively for design, specification and API decisions, never guess or infer these"; "work in small increments, each task a few minutes max; don't proceed until the current one is confirmed"; "document the design before implementing; have me review the documented plan first"; "the plan of what to write must be human-defined; code can be AI-generated, architecture and specifications cannot be guessed"; never commit to the default branch; ticket numbers belong in history, not code comments; comments describe current state only; the formatter before committing; table tests over bare asserts; each test meaningful, none to pad coverage.

