# Feature: Terminal output safety — a standalone requirement in the contract

## Summary

A spec amendment (phase 1a/1b re-entry) to the ratified contract
`.design/agent-first-vsdd-toolkit.md`. It adds the requirement the
six-round Layer-3 terminal-cleaning work was built *without*: that every
string vsdd emits to a terminal or agent-consumed surface, sourced from
outside the tool's own compiled-in constants, is stripped of the
display-unsafe Unicode class before it reaches the surface. The class is
defined **by Unicode property** — the control, format, and
line/paragraph-separator categories together with `Default_Ignorable_Code_Point`
— never a hand-enumerated list. It is the outbound peer of crosslink's
inbound hidden-Unicode rule (`.crosslink/rules/web.md`): crosslink strips
what an agent *reads* from the web; this strips what the tool *emits*.

This is the v2 draft, revised after a five-lens cold review (solution
owner, security, solution architect, documentation reviewer, technical
writer; all five returned revise-before-ratify) and operator triage on
four spec-intent questions. The review's convergent findings reshaped the
amendment: it is now a **standalone requirement** with a leading name (not
a clause buried in Status); the class **strips variation selectors** (the
property wins over combining-mark preservation — the operator ruling
reversing vsdd-cli #798, closing the variation-selector smuggling
channel); its scope **binds at each owning layer** (Status now, generated
context and gate outputs and the init surface at theirs); and
confusable/homoglyph substitution is a **declared out-of-scope residual**,
routed to the shared-primitive raise. The motivating record is the
Layer-3 phase-3 loop, which hardened a hand-rolled classifier over six
rounds that was "always one adversarial probe short of the class"
(vsdd-cli #788/#793/#798/#803/#804) — the enumeration failure the
by-property specification exists to end.

## Requirements

- REQ-1: The **terminal-output-safety property** is stated: every string
  vsdd emits to a terminal or agent-consumed surface that is sourced from
  **outside the tool's own compiled-in constants** is stripped of the
  display-unsafe Unicode class before it reaches the surface. The cleaned
  set is the **complement of the tool's own authored constants** — every
  string read across a trust boundary the contract already names (adopter-
  edited state and configuration, tracker and git query output, adopter-
  authored registry data, the settings probe, the installed-artifact
  manifest, the repo-set configuration) is external and cleaned; the named
  sources are **instances, not the whole set**.
- REQ-2: The class is defined **by Unicode property, never a
  hand-enumerated list**: the control category (`Cc`), the format category
  (`Cf`), the line and paragraph separators (`Zl`/`Zp`), and every code
  point carrying the Unicode Character Database property
  `Default_Ignorable_Code_Point` — including the property's reserved
  (unassigned) portion, which renders invisibly on conformant terminals.
  The class is exactly this union. **Precedence** (operator ruling 2026-07-28,
  reversing vsdd-cli #798): where a code point is both default-ignorable
  and a combining mark — the variation selectors, the combining grapheme
  joiner, the Mongolian variation selectors — **the property wins and the
  code point is stripped**; the invisible-payload channel (arbitrary bytes
  encoded as selector runs into agent context) outweighs rendering fidelity
  on these surfaces, and the base character survives. **Preserved**:
  combining marks that do *not* carry the property (`Mn`/`Mc`/`Me` outside
  `Default_Ignorable_Code_Point` — the accents and marks of the world's
  scripts) and normal spaces (`Zs`). **Accepted residuals**, stated so the
  class is honest about its edge: blank-but-cell-occupying code points
  outside the class (the braille pattern blank `U+2800`, the wider `Zs`
  spaces) survive — they cannot carry zero-width payload, and legitimate-
  text preservation wins; unbounded combining-mark stacking survives,
  bounded in practice by the per-field display budgets; and the invisible
  joiners (`Cf`) are stripped despite legitimate uses in some scripts,
  because on these surfaces the smuggling channel outweighs the fidelity
  cost. Stripping is silent on these surfaces (rendering safety is the
  purpose); forensic surfacing of a strip event is a later detection item,
  not this requirement.
- REQ-3: The **cleaning happens at three points**, stated because each
  alone has a demonstrated gap:
  - the **source boundaries** — the snapshot acquisition, the state read,
    the registry load, and every other read that crosses a trust boundary
    the contract names (the settings probe, the installed-artifact manifest
    read, the repo-set configuration load, and any tracker- or git-sourced
    string joined in the effectful shell) — so every downstream rendering
    inherits safe data;
  - the **machine form**, sanitized whole at output — every string value
    **and every object key**, so no field can be missed regardless of the
    serialized struct's shape (a key sourced from external data, such as a
    map keyed by adopter repo name, is cleaned like any value);
  - the **broken-state composition** — the one branch where the source
    read failed, so no source boundary fired and the machine form's
    whole-of-output pass does not reach the human diagnostic — where the
    composed diagnostic payload, including any quoted external file content,
    is cleaned at composition, on all three forms.
  The whole-of-output pass is a **pure filter applied once by the effectful
  shell at output** — a rendering-stage filter over the serialized value,
  not a second derivation; it preserves the one-acquisition-one-derivation-
  one-rendering discipline.
- REQ-4: The **threat is named**: display-spoofing and invisible smuggling
  into agent context — the bidirectional-reordering half of the Trojan
  Source class (the vulnerability record CVE-2021-42574), whose controls
  are format-category and so category-covered. Confusable/homoglyph
  substitution (the vulnerability record CVE-2021-42694), and visual
  reordering built from legitimate right-to-left characters with no stripped
  control, are **distinct visible-character threats outside the stripped
  class** and are declared out-of-scope residuals (REQ-7). The requirement
  is stated as the **outbound peer** of the inbound hidden-Unicode rule
  crosslink carries for web content (mechanized through its safe-fetch
  server, agent conduct on the built-in path), so the two directions read
  as one policy split across a trust boundary.
- REQ-5: **Scope binds at each owning layer**: the property is bound now on
  the Status renderings (their three forms and the broken-state branch);
  **generated agent context** (the session skill and domain prompts built
  from routed sources including the adopter-authored registry) and **gate
  command outputs** inherit the property at their owning layers, and the
  `vsdd init` / preflight surface (Layer 4) likewise — each an inheritance
  the owning layer wires, named here so no agent-consumed surface is left
  silently unbound. The cleaner is one shared implementation; the wiring at
  each layer is that layer's act.
- REQ-6: The property is homed as a **standalone requirement** with a
  leading name (**Terminal output safety**) in the Requirements list, so
  every layer that must inherit it (REQ-5) can cite it by name — the
  contract's own referencing rule. The **Status requirement gains a
  one-line pointer** beside its rendering clauses (its three forms and
  their broken-state branch carry the property); **Trust boundaries** gains
  a cross-reference stating the inbound/outbound peer framing and carrying
  the shared-primitive raise (REQ-7).
- REQ-7: A **shared-primitive raise** is recorded in Trust boundaries as a
  **plain cross-tool proposal filed on the crosslink tracker** — *not*
  under the vsdd-cli #739 three-question boundary procedure, which governs
  mdatron check-needs and, applied to a runtime string-cleaning primitive,
  answers "vsdd implements it locally, never raised." The raise is
  **non-blocking**: vsdd satisfies the requirement locally now via the
  Unicode property; whether crosslink adopts a shared implementation —
  serving both the inbound (strip what you read) and outbound (strip what
  you emit) directions — is crosslink's decision. Confusable-skeleton
  detection (REQ-4's declared residual) rides the same raise as a companion
  cross-tool question, since it too is a hidden-content discipline both
  tools would otherwise re-derive.
- REQ-8: The **mechanism** is stated in the Verification architecture
  (phase 1b): the property comes from a **maintained crate that sources the
  Unicode Character Database's `DerivedCoreProperties` and states its
  Unicode version** — the contract names the crate *class*, not the crate:
  the candidate (`icu_properties` is verified-capable; the incumbent
  `unicode-general-category` and the earlier-named `unicode-properties`
  expose general-category data only and cannot carry a derived-core
  property) is recorded in the amendment's decision record, not the
  contract. **Hand-deriving the property from category data
  is forbidden** — it is the rejected enumeration approach under the
  property's name. The enumeration approach is explicitly rejected (the
  six rounds demonstrated it is always one adversarial probe short of the
  class). The existing terminal-safety tests re-point as this requirement's
  verification, with the variation-selector test **inverted** per the
  strip ruling (REQ-2); the whole-of-output machine-form sanitizer
  (vsdd-cli #803) is retained as the REQ-3 backstop. The keep/discard
  disposition of specific working-tree edits lives in the amendment's
  decision record, not in the contract.

## Acceptance Criteria

*(The spec-review loop has no red-gate stage; falsification paths and
seeded criteria play that role for prose — per the contract's Methodology
requirement. These criteria are the cold review's falsification targets,
and they run against the verbatim clause text in "Proposed contract text"
below, not against these descriptions.)*

- [ ] AC-1: The contract states the property with the trusted set as the
  complement of the tool's compiled-in constants, anchored to the existing
  Trust-boundaries enumeration, with the named sources marked as instances,
  not an exhaustive list. (REQ-1)
- [ ] AC-2: The contract defines the class as the union `Cc` ∪ `Cf` ∪ `Zl`
  ∪ `Zp` ∪ `Default_Ignorable_Code_Point` (reserved portion included),
  states the property-wins precedence over combining marks (variation
  selectors stripped), names the preserved set (combining marks outside the
  property, `Zs`), and names the accepted residuals; no hand-enumerated
  code-point list appears in the normative text. (REQ-2)
- [ ] AC-3: The contract names all three cleaning points — source
  boundaries (with the trust-boundary reads named), whole-of-output
  machine-form sanitization of values and keys, and broken-state
  composition cleaning — and classifies the whole-of-output pass as a pure
  shell-applied filter, not a second derivation. The closed list of
  surfaces escaping every point is empty. (REQ-3)
- [ ] AC-4: The contract names the Trojan Source bidirectional class
  (CVE-2021-42574) as covered, names confusable substitution
  (CVE-2021-42694) as the declared out-of-scope residual, and frames the
  rule as the outbound peer of crosslink's inbound web rule. (REQ-4)
- [ ] AC-5: The contract binds the property on the Status renderings now
  and names generated context, gate outputs, and the init surface as
  inheriting it at their owning layers. (REQ-5)
- [ ] AC-6: The property is a standalone leading-named requirement; the
  Status requirement carries a one-line pointer; Trust boundaries carries
  the peer framing and the shared-primitive raise. (REQ-6)
- [ ] AC-7: The Trust-boundaries raise is recorded as a plain crosslink-
  tracker proposal, explicitly not under the #739 procedure, marked
  non-blocking, with vsdd's local satisfaction stated and confusable
  detection named as a companion. (REQ-7)
- [ ] AC-8: The Verification-architecture note states the mechanism (a
  maintained crate sourcing the Unicode Character Database's
  `DerivedCoreProperties` and stating its Unicode version; the candidate
  crate named in the decision record, not the contract), forbids
  hand-derivation, rejects enumeration, re-points the existing tests with
  the variation-selector test inverted, and retains the whole-of-output
  sanitizer; the working-
  tree edit disposition and any function-level identifier stay out of the
  contract. (REQ-8)
- [ ] AC-9: The Revision line records the amendment in the established
  chronicle shape — the loop named; the re-entry issue (#807) and the
  same-day ratification decision; the five-lens cold-review composition and
  the terminal verify round; a motivated-by clause naming the Layer-3
  finding class (#788/#793/#798/#803/#804) and the out-of-process-edit
  deviation #808 whose reverted commit carried this amendment's first
  unreviewed draft; then the clause summaries — and the register is clean
  (no coined labels, no unexpanded acronyms, plain domain names, the
  disclosure's name written "Trojan Source"), verified by the Documentation
  Reviewer and Technical Writer lenses against the verbatim text. (all)

## Architecture

The amendment edits the ratified contract
`.design/agent-first-vsdd-toolkit.md` in four regions plus the Revision
line; the verbatim insertions are in "Proposed contract text" below.

1. **Requirements list** (`## Requirements`, after the Status
   requirement): a new leading-named requirement **Terminal output
   safety** carrying REQ-1 through REQ-5 (REQ-6).
2. **Status requirement** (line 173): a one-line pointer to the new
   requirement, phrased as "the three forms and their broken-state branch"
   (matching the contract's established three-forms framing, not "four
   surfaces"). (REQ-6)
3. **Trust boundaries** (the Verification-architecture subsection, ~lines
   226-236): the inbound/outbound peer framing and the shared-primitive
   raise (REQ-6, REQ-7).
4. **Verification architecture (phase 1b)** (line 202): the mechanism note
   (REQ-8).
5. **Revision line** (line 6): the chronicle entry (AC-9).

The implementation footprint the mechanism note anchors (for the rebuild
that follows ratification, not this amendment): `vsdd-core/src/text.rs`
swaps `is_terminal_unsafe`'s category+range+Hangul body for a predicate
that is the union of the four categories and the
`Default_Ignorable_Code_Point` property from the maintained crate; the
variation-selector-survives test inverts (they now strip); the whole-of-
output sanitizer over the machine form (values and keys) is retained; the
round-6 Hangul arm is dropped (subsumed by the property). This footprint,
the keep/discard of the specific uncommitted edits, and the crate
selection are decision-record and rebuild-plan content, deliberately kept
out of the contract text (cold-review finding: transient content does not
belong in a ratified contract).

## Proposed contract text

*The verbatim insertions, so the register is reviewed against the words
that ship (cold-review finding: a review of text composed later at the
commit is a review that never ran against the shipping words — the gap the
#808 deviation was about).*

**(1) New requirement, added to `## Requirements` after the Status
requirement:**

> - **Terminal output safety** — every string vsdd emits to a terminal or
>   agent-consumed surface that is sourced from outside the tool's own
>   compiled-in constants is stripped of the display-unsafe Unicode class
>   before it reaches the surface; the cleaned set is the complement of the
>   tool's authored constants — every string read across a trust boundary
>   this contract names (adopter-edited state and configuration, tracker
>   and git query output, adopter-authored registry data, the runtime-harness
>   settings file, the installed-artifact manifest, the repo-set
>   configuration) is external and cleaned, the named sources instances
>   rather than the whole set. The class is defined by Unicode property, never a hand-enumerated
>   list: the union of the control category (`Cc`), the format category
>   (`Cf`), the line and paragraph separators (`Zl` and `Zp`), and every
>   code point carrying the Unicode Character Database property
>   `Default_Ignorable_Code_Point`, its reserved unassigned portion
>   included — invisible on conformant terminals. Where a code point is both
>   default-ignorable and a combining mark — the variation selectors, the
>   combining grapheme joiner — the property wins and it is stripped: the
>   invisible-payload channel, arbitrary bytes encoded as selector runs
>   into agent context, outweighs rendering fidelity on these surfaces and
>   the base character survives. Combining marks that do not carry the
>   property and normal spaces (`Zs`) are preserved as the legitimate text
>   of the world's scripts. Edges accepted rather than chased:
>   blank-but-cell-occupying code points outside the class (the braille
>   pattern blank, the wider spaces) survive, carrying no zero-width
>   payload; combining-mark stacking survives, bounded in practice by the
>   per-field display budgets; and the invisible joiners are stripped
>   despite legitimate script uses, the smuggling channel outweighing the
>   fidelity cost — stripping is silent on these surfaces, forensic
>   surfacing of a strip a later detection concern. Cleaning happens at three points, each
>   alone having a demonstrated gap: at the source boundaries — the snapshot
>   acquisition, the state read, the registry load, and every other read
>   crossing a named trust boundary — so downstream renderings inherit safe
>   data; on the machine form, sanitized whole at output, every value and
>   every object key, so no field is missed whatever the serialized shape;
>   and at the broken-state composition, the one branch where the source
>   read failed and no boundary fired, where the composed diagnostic
>   payload including any quoted external content is cleaned at composition,
>   on all three forms. The whole-of-output pass is a pure filter the
>   effectful shell applies once at output, a rendering-stage filter and not
>   a second derivation. The threat is display-spoofing and invisible
>   smuggling into agent context — the bidirectional-reordering Trojan
>   Source class (the vulnerability record CVE-2021-42574), its controls
>   format-category and so covered — and the requirement is the outbound
>   peer of the inbound hidden-Unicode rule crosslink carries for web
>   content (mechanized through its safe-fetch server, agent conduct on the
>   built-in path): crosslink governs what an agent reads, this strips what
>   the tool emits. Confusable and homoglyph substitution (the vulnerability
>   record CVE-2021-42694), and visual reordering built from legitimate
>   right-to-left characters with no stripped control, are distinct
>   visible-character threats outside the stripped class — declared
>   residuals carried to the shared-primitive raise (Trust boundaries),
>   never silently implied as covered. The
>   property binds now on the Status renderings — their three forms and the
>   broken-state branch; generated context, gate command outputs, the
>   waiver enumeration, the cost-query and efficiency-advisory renderings,
>   and the `vsdd init` and preflight surfaces inherit it at their owning
>   layers, one shared cleaner wired per layer as that layer's act — the
>   named surfaces instances of the property's universal scope, every
>   agent-consumed surface a later layer builds inheriting it the same way,
>   never left unbound by omission from this list.

**(2) Status requirement — one-line pointer, appended to the sentence
naming the three forms and the broken-state outputs:**

> The three forms and their broken-state branch carry the Terminal output
> safety property; every rendered string sourced outside the tool's own
> constants is cleaned of the display-unsafe class before it reaches the
> surface.

**(3) Trust boundaries — new member:**

> Terminal output safety is the outbound peer of the inbound hidden-Unicode
> rule crosslink carries for web content (the Terminal output safety
> requirement carries the property; this member records the boundary
> framing and the raise). It is a candidate shared primitive: one
> implementation could serve both directions rather than each tool
> re-deriving it. vsdd satisfies the requirement locally now via the
> Unicode property; the shared-primitive question is raised to crosslink as
> a plain proposal on the crosslink tracker — not under the three-question
> boundary procedure, which governs mdatron check needs and would answer
> that vsdd implements it locally — non-blocking, with confusable-skeleton
> detection named as a companion cross-tool question, and mdatron's
> action-time diagnostic stream — adopter-authored markdown quoted into
> agent-visible context through vsdd-installed hooks — named as a second
> companion, its cleaning to be carried by mdatron's own contract or the
> shared primitive.

**(4) Verification architecture (phase 1b) — mechanism note, appended to
the phase-answer/rendering purity discussion:**

> Terminal output safety's cleaner draws the `Default_Ignorable_Code_Point`
> property from a maintained crate that sources the Unicode Character
> Database's `DerivedCoreProperties` data and states its Unicode version
> (the class then the union of that property with the control and format
> categories and the line and paragraph separators, per the requirement);
> deriving the property from category data by hand is forbidden, being the
> rejected enumeration approach under the property's name. The whole-of-output machine-form
> sanitizer is a pure filter the shell applies once at output. The existing
> terminal-safety tests are this requirement's verification, the
> variation-selector case inverted to assert stripping.

**(5) Revision line — chronicle entry, appended to line 6:**

> Amended 2026-07-28 under the phase-1a spec-amendment loop (vsdd-cli #807,
> ratified same day by operator decision on that issue after a five-lens
> cold review — solution owner primary, security co-primary, solution
> architect, documentation reviewer, technical writer — and a terminal
> verify round; motivated by the terminal-safety finding class #788, #793,
> #798, #803, and #804, and the out-of-process-edit deviation #808 whose
> reverted commit carried this amendment's first unreviewed draft): the
> Terminal output safety requirement — the display-unsafe class defined by
> the Unicode property `Default_Ignorable_Code_Point` with the control and
> format categories and the line and paragraph separators, stripped from
> every terminal- and agent-consumed surface sourced outside the tool's own
> constants, at the
> source boundaries, whole at machine-form output, and at broken-state
> composition; variation selectors and the other default-ignorable marks
> stripped, the property winning over combining-mark preservation
> (reversing #798); generated context and gate outputs bound at their
> owning layers; confusable substitution a declared out-of-scope residual;
> the cleaner's property drawn from a maintained crate sourcing the Unicode
> Character Database and stating its Unicode version, hand-derivation
> forbidden; and the inbound/outbound peer framing and the shared-primitive
> raise recorded in Trust boundaries.

## Open Questions

*(All resolved — the two first-draft questions by the cold review and
operator triage, the four cold-review-surfaced questions by operator
triage 2026-07-28; recorded here.)*

- **Q1 (clause placement) — resolved: standalone requirement.** The
  property is cross-cutting (it binds Status, generated context, gate
  outputs, and the init surface across three layers, per REQ-5), so it
  needs a referenceable leading name those layers can cite — which a
  requirement gives and a Trust-boundaries member does not. The peer
  framing and the raise stay in Trust boundaries as a cross-reference. The
  solution-architect and solution-owner lenses split (standalone
  requirement versus Trust-boundaries member); the operator ruled the
  standalone requirement, the D2 binding decision tipping it.
- **Q2 (variation selectors) — resolved: strip.** The property and the
  combining-mark preservation overlapped (variation selectors, the
  combining grapheme joiner are both default-ignorable and `Mn`); the
  operator ruled the property wins, stripping them, reversing vsdd-cli #798
  — the variation-selector smuggling channel outweighs the emoji-
  presentation fidelity cost, and the ruling keeps the by-property thesis
  (no hand-carved exceptions). The existing variation-selector-survives
  test inverts (REQ-8).
- **Q3 (scope) — resolved: bind at owning layer.** The requirement's
  generality reaches generated agent context — the threat's prime target —
  so the operator ruled it bound at each owning layer rather than narrowed
  to Status (REQ-5).
- **Q4 (confusables) — resolved: declared out-of-scope residual.**
  Confusables are visible characters no stripping class catches; their
  defense is detection, a different mechanism. The operator ruled them out
  of scope as a declared residual, routed to the shared-primitive raise
  (REQ-4, REQ-7), with the CVE citation split.
- **Q5 (crate/property carriage) — resolved: DerivedCoreProperties crate.**
  Neither first-named candidate carries the property; the mechanism note
  names the crate *class* (a maintained crate sourcing the Unicode
  Character Database's `DerivedCoreProperties`, stating its Unicode version)
  and forbids hand-derivation, with the candidate crate (`icu_properties`)
  recorded in the decision record rather than the contract (REQ-8).
- **Q6 (sink completeness) — resolved: three points.** The broken-state
  composition and the machine-form object keys are named as cleaning points
  beside the source boundaries and the whole-of-output value pass (REQ-3).

## Out of Scope

- **Confusable/homoglyph substitution** (CVE-2021-42694) and **implicit
  bidirectional reordering** built from legitimate right-to-left characters
  — declared residuals (REQ-4): visible-character threats whose defense is
  detection, not stripping; routed to the shared-primitive raise (REQ-7),
  not built here.
- The **Layer-3 rebuild** (the classifier swap to the property crate, the
  variation-selector test inversion, the whole-of-output wiring) — parked
  until this amendment ratifies; its construction is Layer 3's resumed
  phase-2 work.
- **Building the shared crosslink primitive** and the confusable detector —
  this amendment raises the boundary questions; adoption is crosslink's
  decision.
- The **generated-context, gate-output, and Layer-4 init wiring** — REQ-5
  binds the property at those layers; each layer wires the shared cleaner
  as its own act when it hardens.
- **Forensic strip-event surfacing** — stripping is silent on these
  surfaces (REQ-2); a strip-count or detection signal is a later item.
- Any edit to source code or to the contract in this cycle — this document
  is the review draft; the contract changes only on ratification, through
  the owning-domain composition and cold review. The ratification commit
  that applies this amendment cites this review's ratified decision record
  in its structured evidence section, per the contract's amendment-
  discipline format-carry.
