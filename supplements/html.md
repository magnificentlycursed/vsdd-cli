---
supplement_slug: html
languages_or_interfaces: [HTML, HTML5]
domains_in_scope: [ux, accessibility, localization, security]
extensions: []
---

# HTML Supplement

Per-domain extensions for HTML-bearing surfaces (when the project ships a browser-side UI, generated report pages, or HTML templates).

## UX extensions

- **Semantic HTML.** Use `<button>` for buttons, `<a>` for navigation, `<form>` for forms. `<div>` and `<span>` reserved for layout-only purposes. Semantic tags carry default behavior + accessibility for free.
- **Form discipline.** Every input has a `<label>`; submit buttons have descriptive text; client-side validation surfaces errors near the field. Form submission with no client-side feedback is the UX failure mode.
- **Progressive enhancement.** Core functionality works without JavaScript; JS adds polish + interactivity. Server-side rendering as the base + hydration as enhancement.

## Accessibility extensions

- **ARIA labels + roles.** Used when semantic HTML doesn't cover the case; `aria-label` for icon-only buttons; `role="dialog"` for modals; `aria-live` for status updates. ARIA-without-testing is the failure mode.
- **Keyboard navigation.** Every interactive element reachable via Tab; focus-order logical; focus-visible (not just :hover); focus-trap on modals released on close.
- **Color-contrast.** WCAG 2.1 AA minimum (4.5:1 for body text, 3:1 for large text); AAA where contrast-sensitive. Tested with browser DevTools' contrast checker.
- **Screen-reader testing.** Tested with at least one screen reader named in `manual-tests/<layer>.md` (NVDA / JAWS / VoiceOver / Orca).
- **Skip-links.** `<a href="#main">Skip to main content</a>` as first interactive element for keyboard users.

## Localization extensions

- **`<html lang="..." dir="...">`.** Language + text-direction declared on `<html>` element. Per-page (or per-section if mixed) language attribution.
- **Text expansion tolerance.** Layouts tolerate +50% string length (German, Russian, Finnish often expand from English source).
- **RTL script support.** `dir="rtl"` on root for Arabic / Hebrew / Persian / Urdu; bidi text (mixed LTR + RTL) renders correctly via Unicode bidi algorithm.

## Security extensions

- **Content Security Policy (CSP).** HTTP response header restricting script-src, style-src, connect-src, etc. `unsafe-inline` + `unsafe-eval` rejected. Nonces or hashes for unavoidable inline scripts.
- **XSS prevention.** Framework escapes by default (React, Vue, Svelte, Angular). `dangerouslySetInnerHTML` / `v-html` / `[innerHTML]` requires explicit justification + sanitization (DOMPurify / equivalent).
- **CSRF prevention.** State-changing requests carry CSRF tokens OR use SameSite cookies (`SameSite=Strict` or `SameSite=Lax`).
- **Iframe sandboxing.** Iframes loading untrusted content use `sandbox` attribute restricting capabilities.
