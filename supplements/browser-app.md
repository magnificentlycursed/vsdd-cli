---
supplement_slug: browser-app
languages_or_interfaces: [browser-frontend, SPA, PWA, MPA]
domains_in_scope: [ux, accessibility, security, performance-engineer, software-engineer]
extensions: []
---

# Browser App Supplement

Per-domain extensions for projects that ship a browser-side UI (single-page applications, multi-page apps, progressive web apps).

## UX extensions

- **Routing discipline.** Per-route layout + per-route lazy loading; URL is the source-of-truth for app state where applicable.
- **Loading + skeleton states.** Per-route loading indicators; skeleton screens for known-shape content; no blank-page-while-fetching.
- **Error boundary discipline.** Per-component error boundaries (React / equivalent) prevent whole-app crash on per-component failure. Error UI names the recovery action.
- **Offline + low-bandwidth tolerance.** Service worker for offline-capable surfaces; per-feature graceful degradation when offline.

## Accessibility extensions

- **Route-change announcements.** SPA route changes don't fire native page-load events; explicit ARIA-live announcements + focus management on route change.
- **Modal + dialog focus management.** Focus moves to dialog on open; trapped within dialog; returns to triggering element on close.
- **Toast + notification accessibility.** `role="status"` for non-critical; `role="alert"` for critical. Auto-dismiss timing respects `prefers-reduced-motion` extended-duration mode.

## Security extensions

- **Authentication flow.** Token storage in HttpOnly cookies (not localStorage); refresh-token rotation; logout invalidates server-side session.
- **Authorization at every API call.** Frontend authorization is UX hint, not security boundary; every API call re-authorizes server-side.
- **Dependency surface.** Per the dependency-approval discipline applied to npm packages: SO + PE + Security investigation for new direct deps; transitive-dep audit via `npm audit` / `pnpm audit`.
- **CSP + SRI.** Content-Security-Policy header + Subresource Integrity attribute on `<script src>` + `<link rel="stylesheet">`. Defends against compromised CDN.

## Performance Engineer extensions

- **Core Web Vitals.** Largest Contentful Paint (LCP) < 2.5s; First Input Delay (FID) / Interaction to Next Paint (INP) < 200ms; Cumulative Layout Shift (CLS) < 0.1. Per-metric budgets in CI.
- **Bundle splitting.** Per-route bundle; per-vendor bundle; dynamic imports for off-critical-path code.
- **Image optimization.** Modern formats (AVIF / WebP) with fallbacks; responsive `srcset` + `sizes`; lazy loading below-the-fold.
- **Service worker caching strategy.** Per-resource-type caching (stale-while-revalidate for HTML; cache-first for hashed assets); cache-invalidation on deploy.

## Software Engineer extensions

- **Framework choice.** React / Vue / Svelte / Solid / vanilla — declared in DESIGN.md as part of architecture; trade-offs named.
- **State management.** Per-component state for local; lifted state for shared; context / store libraries for cross-tree state. Per-pattern boundary explicit.
- **API client.** Generated from OpenAPI / GraphQL schema where applicable; hand-rolled clients are an architectural seam requiring discipline.
