---
supplement_slug: toml
languages_or_interfaces: [TOML]
domains_in_scope: [software-engineer, platform-engineer]
extensions: []
---

# TOML Supplement

Per-domain extensions for TOML-bearing surfaces (`Cargo.toml`, `pyproject.toml`, `rust-toolchain.toml`, application config).

## Software Engineer extensions

- **Schema-validated where available.** `Cargo.toml` validated by `cargo`; `pyproject.toml` by PEP 517/518-compliant tools. Application TOML files declare their own schema.
- **Per-table organization.** Top-level keys for metadata; per-component sub-tables. Avoid deep nesting beyond 2-3 levels; deep nesting in TOML is a maintainability cost.
- **Array-of-tables for repeated entries.** `[[dependencies]]` style for repeated structured entries. Cleaner than alternative representations.
- **Comment discipline.** TOML supports comments; use them for non-obvious choices + deprecation notes + cross-reference pointers.

## Platform Engineer extensions

- **`Cargo.toml` discipline.** `[workspace.package]` for shared metadata across workspace crates. `[features]` for compile-time feature flags. `[profile.release]` for ship-binary build config.
- **`pyproject.toml` discipline.** PEP 517/518 `[build-system]` declaration. Tool config (`[tool.black]`, `[tool.mypy]`, `[tool.ruff]`) centralized.
- **`rust-toolchain.toml`.** Pin toolchain version + components. CI installs from this file; reproducible builds.
- **Dependency version pinning.** `version = "1.2.3"` for exact pin; `version = "^1.2"` for compatible-update range; `version = "*"` rejected. Per the dependency-approval discipline.
