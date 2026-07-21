<!-- Project-Specific Rules — vsdd-cli -->
<!-- Injected into every session. Mechanical guards + pointers only; the
     governing contract is .design/agent-first-vsdd-toolkit.md (ratified).
     Substantive process rules live THERE — do not restate them here. -->

## Naming and register
- Never invent labels or tokens: no letter-number clusters (FM1, D2, r5-SA),
  no new acronyms, no all-caps coinages. Use the plain name the source record
  already carries ("the driver-key leak", not "FM2").
- Operator or relay phrasing is NOT registration. A word someone used casually
  does not become a standing term. Term registration is an operator act under
  the contract's maturity lifecycle — propose, never self-serve.
- No variants of existing phrases either: quote a governing document's exact
  phrasing when citing it, or describe plainly — never mutate a sanctioned
  phrase into a new compound (the contract says "hollow shell"; "hollow
  install" was a caught variant coinage). Adopter-facing text (error
  messages, diagnostics, published issues) uses plain description only,
  never estate shorthand.
- Concrete referents over abstraction words (operator ruling 2026-07-20):
  say "crosslink" when you mean crosslink, "Claude Code" when you mean
  Claude Code, "any AI coding assistant" when you mean the class — not
  "substrate" or similar layer-words. Exact names of contract constructs
  (e.g. the session-substrate check) are citations and keep their names
  until an amendment rewords them.

## Crosslink at 0.8.0 — the upgrade hold and known defects
- HARD CONSTRAINT: the host binary stays at crosslink 0.8.0. Never upgrade to
  0.9.x and never run `crosslink migrate hub-v3` on this hub. Retest trigger:
  dollspace-gay/crosslink #4/#5/#7/#8/#11/#12 closing. (Operator ruling
  2026-07-20, decision on vsdd-cli #597.)
- `issue close` and `archive add` can report success without persisting
  (dollspace-gay/crosslink#29, #30). Re-read state after these operations;
  never treat the success message as the record.
- After any `crosslink sync` that promotes issue IDs, re-verify the session
  work binding and locks — promotion strands them (#653's defect, live at 0.8.0).
- Titles from `-q` output are truncated (dollspace-gay/crosslink#14): use
  `--json` whenever titles or exact text matter.
- Never suppress crosslink WARN output (no `2>/dev/null` on crosslink
  commands). WARN dismissal was the proximate cause of the 2026-05-28
  identity leak.

## Identity and privacy
- Nothing machine- or person-identifying enters commits, tracker records, or
  published text: no hostnames, no personal names, no SSH key comments or
  fingerprints, no user-absolute paths. Describe mechanisms generically.
  (The 2026-05-28 incident report is the reference discipline.)

## Commits
- Model attribution trailer (Co-Authored-By: Claude ...) stays, per operator
  ruling 2026-07-20. Domain co-author trailers are never added — domain
  attribution lives in signed tracker records, not git authorship.
- Every governed-set commit names its owning tracker issue in the message.

## Governing documents
- The ratified contract (.design/agent-first-vsdd-toolkit.md) governs; where
  any primer or rule here disagrees, the contract wins.
- Classify every operator directive before executing (contract § Directive
  reconciliation) — including the scope echo.
- Bind session work (`crosslink session work <id>`) before acting; record
  rounds as typed comments with handles cited by exact title or display ID.
