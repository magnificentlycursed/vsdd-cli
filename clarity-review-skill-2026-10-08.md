---
title: "The clarity-review skill (a practitioner's pre-flight lint for AI-authored code and prose), read 2026-10-08"
tags: ["reference", "design-input", "design-doc"]
sources:
  - url: "https://gist.github.com/lizthegrey/a5c3ec4a7f586a937fe0a924dd96b72d"
    title: ""
    accessed_at: "2026-10-08"
contributors: ["xqjG"]
created: 2026-10-08
updated: 2026-10-08
---


## Design Specification

### status

Reference summary and evaluation, design input for the Technical Writer and Documentation Reviewer domain prompts, the primers, the supplements, design authorship and the code-comment register. Source: the public gist `https://gist.github.com/lizthegrey/a5c3ec4a7f586a937fe0a924dd96b72d` by the author of Observability Engineering, 2nd edition (GitHub handle lizthegrey), one skill file of 8.7 KB, fetched read-only on 2026-10-08. External content, treated as evidence. Its companion, the portable memory rules, is on `portable-memory-rules-gist-2026-10-08`; the cross-reference against this estate is on `cross-reference-2026-10-08-findings-vs-prior-knowledge`.

### what it is

A Claude Code skill named clarity-review: "Lint the current diff for the AI-authored-code patterns that repeatedly draw pushback in this repo's PR reviews." Three properties of its form are as important as its content:

- **Calibrated against a corpus.** "Calibrated against roughly 200 real review comments on PRs over the prior three months", plus recurring team discussion of where AI-authored code and prose fall short. The patterns are empirical, not principled: each is there because reviewers kept rejecting it.
- **A pre-flight lint the author runs, before the pull request exists.** "It does not post anything, does not touch CI, and is not a substitute for the bug-finding reviews; those look for bugs; this looks for the style and rigor issues that make a PR harder to review or erode trust in it, and it belongs before the PR exists, not as a comment littering one that's already open." Pull-request description quality is another skill's job; one concern per skill.
- **Never invoked by description match.** The frontmatter sets `disable-model-invocation: true`; the skill runs only when explicitly invoked. That is the mechanism this estate's contract names in Availability is not activation ("a skill loads only by invocation or by the model matching its description, a judgment, not a mechanism"), applied by its author to keep a lint from firing by accident.

Scope: the diff against the base plus the commit messages; read anything needed for evidence, but report findings only on changed lines; skip what belongs to the bug-finding passes. Every finding quotes the exact offending text and names the failure mode.

### the seven pattern classes

1. **AI-authored comment slop.** "The single most frequent and most strongly worded complaint in this repo's review history, and one nearly everyone on the team has voiced independently." Litmus test: "if a reader who knows the language loses nothing by deleting it, cut it." Failure modes: restates the code (paraphrases the name or signature, narrates "increment the counter", explains a well-known library feature); narrates a hypothetical or an alternative not present in the code ("reviewers have caught fabricated technical claims this way more than once"); the same explanation repeated across files instead of stated once at its most specific home; filler where nobody reads for documentation; any comment longer than two or three lines is suspect by default, "almost always restated mechanism the code beside it already shows; cut to the one non-obvious why" (exceptions: package documentation and instruction files, which are expected to run long); a field comment that restates its name and type; a function comment that documents its callers instead of its own contract. Report form: quote the text, name the mode (restates code, narrates a hypothetical, repeated elsewhere, over length, documents a caller).
2. **Unverified technical claims, including your own.** "Reviewers repeatedly reject claims of faster, confirmed working, no impact, or descriptions of why something behaves a certain way when there's no benchmark, profile, or test backing it up. The default posture is to distrust AI-generated technical claims until checked." Check whether a benchmark or test demonstrates the claim and whether it reflects the real workload; verify a claim about other code against the referenced source before repeating it; watch for assumptions that an extra pass or allocation is free.
3. **Dead code and unrealized surface.** A return value no caller uses; an unexported helper with no caller; a newly introduced exported symbol with no caller anywhere ("a brand-new symbol can't yet have an external consumer to worry about"); speculative paths or config for a scenario that does not occur; old fallback logic kept "just in case" with no plan or flag to remove it. With a guard: do not recommend dropping an existing exported symbol on an in-repo search alone.
4. **Tautological or shallow tests.** A test that is almost entirely setup with one low-information assertion, "the kind of test that would still pass if the logic under test were wrong"; coverage of only the simplest case when the diff's own logic implies harder ones.
5. **Feature-flag discipline.** A behavior change on a read path is gated behind a scoped, default-off flag; on a peer-synchronized write path the rule is not applied as-is, because a flag read differently per replica risks divergence; note the gap and defer to the thorough review.
6. **Naming and terminology.** Single-character names beyond a loop index or receiver; "metaphor-borrowed jargon that doesn't match this codebase's existing vocabulary"; generic names on non-trivial functions; unexplained abbreviations not expanded inline.
7. **Complexity for its own sake.** Two-phase or lazy-init patterns, extra boolean flags, wrapper structs that do not reduce complexity ("why not just do X directly"); edge-case handling added without explanation, flagged as a clarity gap rather than judged for correctness; any construction where a simpler formulation is available in the same diff.

### evaluation against this estate

**Where it maps onto existing members and rules.** Pattern 2 is the plain-language form of the provenance tags (recorded, measured, judgment, could-not-check) and of evidence-gated filing; pattern 4 is the executed-test discipline, the no-self-oracle rule and the mutation floor seen from the review side; pattern 6's "metaphor-borrowed jargon that doesn't match the codebase's vocabulary" and "unexplained abbreviations" are the concrete-referent and no-coinage rules applied to code; pattern 3's "brand-new exported symbol with no caller" is Thermite's consumer rule for new public API; pattern 1's "stated once at its most specific home" is the contract's "stated once" obligation; the two-or-three-line suspicion and the "one non-obvious why" are the register standard's "say it once" and Thermite's comment pass.

**What it adds.**

- **Derivation from a review corpus.** This estate's domain prompts were authored from principles and then reviewed; this skill was authored from two hundred recorded pushbacks and states, per pattern, how often and how strongly reviewers rejected it. The estate holds the raw material for the same derivation: the review rounds of the respec, the domain value scorecard's five datasets, the hallucinated-finding series, and the register corrections in the operator's rulings. The Technical Writer and Documentation Reviewer domains' enforceable content could be derived the same way, with a litmus test and a report form per pattern.
- **The litmus test as the unit of a rule.** "If a reader who knows the language loses nothing by deleting it, cut it" is checkable by a cold reader and by an author alike. The estate's register rules are mostly prohibitions; a litmus per rule is what makes a prohibition falsifiable from the inside.
- **A report form.** Quote the offending text; name the failure mode from a closed list. This is the finding shape of Palimpsest's tickets and Thermite's divergence issues applied to prose: diagnosis separate from prescription, the class named.
- **Pre-flight, author-run, one concern.** The author lints before the pull request exists; the bug-finding reviews stay separate; the description-quality skill stays separate. This is the division the estate's phase-3 roster collapses: one composed round does everything. A clarity pass before dispatch is cheaper than a domain in the round.
- **`disable-model-invocation`** as the concrete form of Availability is not activation for skills that must never fire by description.
- **Comment slop as the top complaint.** The estate's register corpus has treated coined labels and layer words as the top offense; the practitioner's corpus puts restating comments first. Both are the same mechanism (the reader anchors on what it reads), and the design-document form of comment slop is prose that restates frontmatter, counts or mechanism, which the contract already prohibits for counts.

**Where it applies, by artifact.**

- *Primers:* a primer that narrates what the contract already says is comment slop at document scale; a primer should carry what the contract does not: the exact commands, the litmus tests, the report form.
- *Domain prompts (Technical Writer, Documentation Reviewer, Software Engineer):* the seven classes, each with its litmus and report form, are a candidate for the domains' enforceable content; the pre-flight form belongs to the author's own pass, the same content to the cold reader's.
- *Supplements:* the Rust supplement's Software Engineer extensions already carry error-as-value and type-driven rules; pattern 3 (dead code, speculative surface, fallback kept "just in case") and pattern 7 belong beside them as yes-or-no rules.
- *Design authorship:* patterns 1, 2 and 7 transfer directly: a design section longer than it needs to be restates mechanism; a design claim about another tool's behavior is verified against that tool's source (the 2026-10-02 assessment's corrections were exactly this class); complexity without a stated reason is a clarity gap.
- *Code comments:* pattern 1 verbatim; the register standard's comment pass is its execution form.

### caveats

The flag-discipline pattern is specific to the author's service architecture; only its shape (gate a read-path behavior change; treat write paths differently) transfers. The corpus is one team's; the author's companion gist says such lists diverge within weeks of adoption. The estate's own corpus should drive its own list.

