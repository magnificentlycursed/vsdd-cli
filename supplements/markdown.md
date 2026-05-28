---
supplement_slug: markdown
languages_or_interfaces: [Markdown, CommonMark, GitHub-Flavored Markdown]
domains_in_scope: [technical-writer, documentation-reviewer]
extensions: []
---

# Markdown Supplement

Per-domain extensions for markdown-bearing surfaces (README, DESIGN docs, methodology spec, primers, domain prompts, supplements, manual-tests, CHANGELOG, review-log entries).

## Technical Writer extensions

- **CommonMark + GFM tables.** [CommonMark](https://commonmark.org/) as base spec; GitHub-Flavored Markdown for tables, task lists, strikethrough. Other dialect features avoided unless target renderer supports them.
- **Heading hierarchy discipline.** H1 once per document (the title); H2 for top-level sections; H3 for subsections; H4+ reserved for tightly-nested context. Skipping heading levels is the accessibility + structural failure mode.
- **Inline-reference navigability.** Cross-references are clickable + scrollable + greppable. "See above" / "the prior section" are vague navigation cues that defeat discoverability (R79 F3 anti-pattern).
- **Code-fence language tags.** Every code fence carries its language (` ```yaml ` / ` ```rust ` / ` ```bash `). Syntax highlighting + machine-parseable language detection.
- **Anchor-ID determinism.** Per the anchor-ID generation conventions, anchors are deterministically derived from frontmatter (`{review_number}-f{finding_number}` etc.). No hand-authored `<a id="..."></a>` tags.

## Documentation Reviewer extensions

- **Cold-context discoverability.** Cold-read each doc as a first-time reader. What requires reconstruction the doc doesn't enable?
- **Dead-link detection.** `check-cite-resolution.py` hook validates every linked anchor + every cited document exists. Fires `VSDD-E0010`.
- **Stale-claim suspicion.** Quantitative claims trace to measurement evidence; citations reference current state. Fires `VSDD-W0030`.
- **Heading-anchor stability.** Renaming a heading breaks every cross-reference pointing at the prior anchor. Heading-rename routes via Phase 4 to update all references.
- **Frontmatter discipline.** Per-document-class frontmatter (Review entry, Finding, Phase primer, Domain prompt, Supplement, etc.) validated against the artifact-class schema.
