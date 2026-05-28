---
supplement_slug: javascript-typescript
languages_or_interfaces: [JavaScript, TypeScript]
domains_in_scope: [software-engineer, quality-engineer, platform-engineer, performance-engineer, security, solution-architect]
extensions: []
---

# JavaScript / TypeScript Supplement

Per-domain extensions for JS / TS projects. Loads alongside the domain prompts in scope when the project declares JavaScript or TypeScript.

## Software Engineer extensions

- **TypeScript strict mode.** `tsconfig.json` `strict: true` baseline. `any` reserved for migration boundaries; every `any` is an outstanding type-debt finding.
- **`null` vs `undefined` discipline.** Pick one for absence-of-value; document the choice in DESIGN.md. Mixing both without rationale is the failure mode.
- **Promise + async/await over callbacks.** Modern JS / TS. Callback-style reserved for Node.js stream APIs and event emitters.
- **ESM over CommonJS for new code.** Per-project decision; mixing requires explicit interop layer.

## Quality Engineer extensions

- **`vitest` / `jest` for unit testing.** Project picks one; vitest faster for TS via esbuild; jest broader ecosystem.
- **`fast-check` for property-based testing.** Phase 5 surface A. Properties express DESIGN.md invariants; works for both JS and TS; default 100-1000-case budget.
- **`Stryker` for Mutation Testing.** Phase 5 surface B. Per-mutant disposition required.
- **`fast-check` for Fuzz Testing too.** Phase 5 surface C. Different property shape (parser-input generators); registers separately from property-based testing in the per-domain log.

## Platform Engineer extensions

- **`package.json` discipline.** Engines field pinned (Node version). Scripts named consistently across projects (`build`, `test`, `lint`, `typecheck`).
- **Lockfile committed.** `package-lock.json` (npm) / `yarn.lock` (yarn) / `pnpm-lock.yaml` (pnpm). Tool choice is per-project; mixing is the anti-pattern.
- **`npm audit` / `pnpm audit`.** Supply-chain CVE detection. Per the dependency-approval discipline.
- **Build tool choice.** `tsc` / `esbuild` / `swc` / `webpack` / `rollup` / `vite` — pick per project shape. Build-tool choice is part of architecture.

## Performance Engineer extensions

- **Bundle size discipline.** Per-route / per-entry-point bundle-size budgets declared. Tools: `bundlephobia`, `size-limit`, `webpack-bundle-analyzer`.
- **Tree-shaking enforcement.** ES module imports + `sideEffects: false` in `package.json` enable dead-code elimination at build time.
- **Lazy-load discipline.** Code-splitting at route boundaries; dynamic `import()` for off-critical-path code.
- **Profiling.** Node: `--prof` + Chrome DevTools; Browser: Performance panel + Lighthouse for runtime + Coverage panel for bundle.

## Security extensions

- **`npm audit` integration.** CI-side CVE detection at every PR.
- **CSP for browser apps.** Content-Security-Policy headers; `unsafe-inline` and `unsafe-eval` rejected.
- **XSS discipline.** Frameworks (React, Vue, Svelte) escape by default; `dangerouslySetInnerHTML` / `v-html` requires explicit justification.
- **Input validation at boundaries.** Schema validation libraries (`zod`, `valibot`, `yup`) for runtime input validation matching TypeScript static types.

## Solution Architect extensions

- **Module + package boundaries.** `package.json` `exports` field declares the package's public API. Deep imports across package boundaries are the maintainability gap.
- **Type system + runtime boundary.** TypeScript types vanish at runtime; runtime validation (zod / valibot) at trust boundaries makes the types load-bearing for security.
- **Monorepo discipline.** When applicable: workspace per `package.json` + tool (turbo / nx / pnpm workspaces). Cross-package dependencies are explicit; circular dependencies are the failure mode.
