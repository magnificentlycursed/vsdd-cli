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
- Concrete referents over abstraction words (operator ruling 2026-07-20;
  reaffirmed 2026-07-22 naming "chassis" and "substrate" explicitly):
  say "crosslink" when you mean crosslink, "Claude Code" when you mean
  Claude Code, "any AI coding assistant" when you mean the class — not
  "substrate", "chassis", or similar layer-words in free prose, code
  comments, or adopter-facing text. Exact names of contract constructs
  (e.g. "the crosslink chassis" title clause, the chassis-affordance
  closure, the session-substrate check) are citations and keep their
  names until an amendment rewords them.

## Crosslink version — the 0.8.0 hold is CLEARED (2026-07-22)
- The hold is lifted: upstream patched the migration blockers and the
  operator migrated this hub to v3; the host binary runs 0.9.0-beta.1+
  (operator ruling 2026-07-22, vsdd-cli #742). The old hard constraint
  and its retest trigger are history, kept on the #597/#742 trails.
- Conduct that SURVIVES the clear until re-verified against the new
  version: re-read state after `issue close` and `archive add` (the
  0.8.0 persistence defects may be fixed — verify before trusting);
  re-verify session work binding and locks after any ID-promoting
  `crosslink sync`; use `--json` whenever titles or exact text matter.
- Never suppress crosslink WARN output (no `2>/dev/null` on crosslink
  commands). WARN dismissal was the proximate cause of the 2026-05-28
  identity leak. (Version-independent; stands.)

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
