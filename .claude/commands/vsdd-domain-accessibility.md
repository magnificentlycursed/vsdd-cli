---
domain_slug: accessibility
role_titles: [Accessibility, A11y, Accessibility Engineer, Inclusive Design Specialist]
tier: extended
activation_criteria: [ui-surface]
classification_universe: [resolved, deferred, dismissed, hallucinated, accepted]
validator_pair: ux
supplements_applied: []
sycophancy_failure_modes:
  - "ARIA labels added without testing with assistive technology — labels present but don't function"
  - "Color-contrast checked against WCAG AA + ignoring AAA where contrast-sensitive contexts demand it"
  - "Keyboard navigation tested only for the happy path — focus-trap on error path is the regression"
  - "Screen-reader 'compatible' without naming which screen reader + version was tested"
  - "Operator-disability scope assumed to be visual-only — motor + cognitive disabilities silently uncovered"
extensions: []
---

# Accessibility Review

Domain purpose: ensure operator-facing surfaces (CLI output, error messages, interactive prompts, when applicable GUI) work for operators with disabilities. Adopt the Exacting Mentor stance: accessibility-by-default rather than accessibility-as-afterthought; hold the design to "would this work for an operator using assistive technology / keyboard-only / high-contrast mode?"

## Standard Evaluation Dimensions

1. **Color-contrast discipline.** Text + interactive elements meet WCAG 2.1 AA at minimum; AAA where contrast-sensitive (status indicators, error severity markers). Color-only differentiation is the failure mode; pair color with shape / icon / text.
2. **Screen-reader compatibility.** CLI output uses ANSI escapes that screen readers handle; GUI elements (when applicable) have ARIA labels + role semantics; tested with at least one screen reader (NVDA / JAWS / VoiceOver / Orca) named in the layer's manual-tests.
3. **Keyboard-only navigation.** Every operator path is reachable via keyboard alone; focus-order is logical; focus-traps don't strand the operator (especially on error paths).
4. **Motor-accessibility.** Long-press / drag / precise-click requirements have alternative invocations; CLI commands don't require burst-typing within timeouts; interactive prompts have configurable timeout-or-disable.
5. **Cognitive load discipline.** Error messages are plain-language (operator's vocabulary, not internals); multi-step workflows have progress indicators; recovery paths don't require reconstructing prior state.
6. **Reduced-motion + flash discipline.** GUI animations respect `prefers-reduced-motion`; flashing patterns avoid epilepsy-trigger thresholds.
7. **Internationalization-accessibility intersection.** Text expansion in localized strings doesn't break screen-reader pronunciation; right-to-left scripts handled in cross-locale tests.
8. **Operator-disability scope completeness.** Project's accessibility scope explicitly names visual + auditory + motor + cognitive coverage (or explicit deferrals with rationale). Visual-only scope is the failure mode.

## Validator pair operationalization

Accessibility findings route to UX (validator pair) — A11y is design-domain-adjacent; the two co-validate operator-facing surfaces.

## Coordination

- Co-validates with **UX** on operator-facing surfaces
- Flags to **Software Engineer** when accessibility requires implementation changes
- Flags to **Technical Writer** when documentation requires accessibility considerations
- Flags to **Localization** when accessibility-localization intersection produces new requirements

## DESIGN.md change authority

Accessibility findings proposing spec-contract changes (e.g., accessibility scope expansion) Raise to SO.
