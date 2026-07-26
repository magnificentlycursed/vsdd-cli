# unicode-general-category

**Status:** approved (runtime dependency, vsdd-core)
**Approved:** 2026-07-26
**Approved by:** phase-3 round-4 security finding (vsdd-cli #793); operator awareness flagged at the fix pass

## What it is

[`unicode-general-category`](https://crates.io/crates/unicode-general-category)
— a pure-data lookup of a code point's Unicode General_Category
(`Cc`, `Cf`, `Zl`, `Lu`, `Mn`, …), generated from the Unicode Character
Database.

## Why we need it

The terminal-cleaning policy (`vsdd_core::text`) must strip the whole
class of display-affecting code points — the C0/C1 controls, every
format character (`Cf`: bidi overrides, zero-width, word joiner, the
tag block, interlinear annotation), and the line/paragraph separators
(`Zl`/`Zp`) — from any string bound for a terminal. Two consecutive
review rounds (vsdd-cli #788, #793) demonstrated that a hand-enumerated
list of "the bad code points" is always incomplete against the next
adversarial probe: round 3 enumerated the bidi set and missed the word
joiner and the tag block; the robust fix is to strip by category, which
needs the category of a code point at runtime — a fact the standard
library does not expose.

## Why this crate

- Pure data: a generated lookup table, no `unsafe` on the query path,
  no C dependencies, no build-script network access, no `proc-macro`.
- Single-purpose and minimal: it answers exactly `char ->
  GeneralCategory` and nothing else, so the supply-chain surface is a
  data table and one lookup function.
- Tracks the Unicode Character Database, so the category set stays
  current as Unicode adds format characters — precisely the
  maintenance the hand-list could not carry.

## Scope

Consumed only by `vsdd_core::text::is_terminal_unsafe`. Combining marks
(`Mn`/`Mc`/`Me`) and normal spaces (`Zs`) are deliberately NOT stripped
— they are legitimate text, not on the hidden-Unicode threat list, and
stripping them would corrupt real repo and milestone names in most of
the world's scripts.
