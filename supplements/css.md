---
supplement_slug: css
languages_or_interfaces: [CSS, Sass, SCSS, Tailwind]
domains_in_scope: [ux, accessibility, performance-engineer]
extensions: []
---

# CSS Supplement

Per-domain extensions for CSS-bearing surfaces (browser-side UI styles, generated report styling, design system).

## UX extensions

- **Design tokens.** Colors, spacing, typography, shadows declared as CSS custom properties (`--color-primary: #...`). Single source-of-truth; per-theme override at root.
- **Component-scoped styles.** Per-component CSS modules / styled-components / Tailwind utility composition. Global stylesheets reserved for reset + design tokens.
- **Responsive design.** Mobile-first breakpoints; `min-width` queries that progressively enhance. Per-breakpoint layout verified across device classes.
- **State + variant discipline.** Hover / focus / active / disabled states styled explicitly; `:focus-visible` for keyboard-only focus indication.

## Accessibility extensions

- **`prefers-reduced-motion`.** Animations + transitions respect the operator preference; auto-play motion is the accessibility failure mode.
- **`prefers-color-scheme`.** Light + dark themes provided where applicable; system-preference detection + manual override toggle.
- **`prefers-contrast: high`.** High-contrast theme provided when color-contrast is load-bearing.
- **Focus indicators.** `:focus-visible` style provides visible focus indication for keyboard users; never `outline: none` without alternative focus indication.
- **Text resizing.** Layouts tolerate 200% text zoom without overlap or clipping (WCAG AA criterion).

## Performance Engineer extensions

- **Critical CSS inline.** Above-the-fold styles inlined in `<head>`; non-critical CSS deferred (`media="print"` swap pattern).
- **Animation performance.** Animate `transform` + `opacity` (compositor-only properties); avoid animating `width` / `height` / `top` / `left` (layout-triggering).
- **Bundle size.** Per-route CSS budgets declared; unused-CSS detection via PurgeCSS / Tailwind's content-scanning / equivalent.
- **`will-change` discipline.** Reserved for measured cases; `will-change: transform` everywhere is an anti-pattern that defeats the optimization.
