# icu_properties

**Status:** approved (runtime dependency, vsdd-core) — retrofit record
**Approved:** 2026-09-24 (retrofit); the addition landed 2026-07-28 (commit 24e76e2e, vsdd-cli #813) with no record, no three-lens review, and no trailers — the `VSDD-E0100` condition the contract's Dependency approval member records verbatim. This record closes the verification debt; the process breach stands as recorded on vsdd-cli #872.
**Approved by:** retrofit under vsdd-cli #872 (Platform Engineer owner, Security validator), the three lenses recorded below

## What it is

[`icu_properties`](https://crates.io/crates/icu_properties) — Unicode character property lookups (General_Category, Default_Ignorable_Code_Point, scripts, and the rest of the Unicode Character Database's binary and enumerated properties) from the ICU4X project (Unicode Consortium), with the property data compiled into the binary under the `compiled_data` default feature — no runtime data load.

## Why we need it

The Terminal output safety requirement (vsdd-cli #807) strips every code point that can hide, reorder, or forge terminal output before a tracker- or state-sourced string reaches the operator's terminal or the agent's machine form. The round-6 implementation kept a hand-maintained list of reserved ranges and a Hangul enumeration; #813 replaced both with the Unicode `Default_Ignorable_Code_Point` property, which subsumes them and is maintained by the Unicode Consortium instead of by us. `vsdd_core::text::is_terminal_unsafe` is the single consumer (`vsdd-core/src/text.rs`).

## Why this crate

- The property is defined by Unicode; the only correct source is the Unicode Character Database, and ICU4X is the Unicode Consortium's own Rust implementation of it.
- `compiled_data` puts the tables in the binary: no file, no network, no environment lookup at run time — the same closed-world posture as the rest of the toolkit.
- No `unsafe` in `icu_properties` itself (grep over 2.2.0's `src/`: zero hits); the `zerovec`/`yoke` layer beneath it carries audited, documented `unsafe` for zero-copy deserialization, maintained by the same project.

## Scope

- Runtime dependency of `vsdd-core`, version requirement `2`, locked at 2.2.0.
- One consumer: `vsdd_core::text` (`DefaultIgnorableCodePoint` via `CodePointSetData`). Any second consumer widens the surface this record covers and re-enters review.

## Three-lens review (retrofit)

**Solution Owner — scope.** In scope: the requirement (#807) is ratified and the crate is its narrowest correct implementation. Overlap noted: `unicode-general-category` (its own record) provides General_Category, which `icu_properties` also provides; consolidating onto one Unicode source is a candidate follow-up, not a condition of this approval.

**Platform Engineer — supply chain.** The crate was already in `Cargo.lock` before #813, pulled transitively through `jsonschema → url → idna → idna_adapter → icu_properties`; #813's lockfile change was one line (the direct edge), and the lockfile held 142 entries before and after. The direct dependency therefore added no crate to the build — it made an existing transitive crate a first-class API surface. Subtree: 13 ICU4X crates (`icu_collections`, `icu_locale_core`, `icu_properties_data`, `icu_provider`, `litemap`, `potential_utf`, `tinystr`, `writeable`, `yoke`, `zerofrom`, `zerotrie`, `zerovec`) plus `displaydoc`, all from `unicode-org/icu4x` except `displaydoc` (MIT OR Apache-2.0). Build: no build script network access; no proc-macro codegen in the property path. Maintenance: active, tagged semver releases, 2.x line.

**Security — CVE, licence, threat.** `cargo audit` on 2026-09-24: no advisory against `icu_properties` or its subtree (the workspace's one allowed warning is RUSTSEC-2026-0190 on the transitive `anyhow`, unrelated). Licence: `Unicode-3.0` (the Unicode License v3, OSI-approved 2024) across the ICU4X subtree — permissive, attribution-only; compatible with this repository's licence. Threat: the crate reads no input from outside the process and holds no I/O; the risk it governs is the opposite one — a property table that lags Unicode would let a newly assigned ignorable code point through the terminal cleaner. Mitigation: track the 2.x line with the workspace's dependency updates; the property version follows the ICU4X release.

## Alternatives considered

- Keep the hand-maintained range list — retired by #813; it lagged Unicode by construction and was already wrong once (the Hangul enumeration).
- `unicode-properties` / `unic-ucd` — smaller crates, but `Default_Ignorable_Code_Point` is not exposed by the first and the second is unmaintained.
- Vendoring a generated table — moves maintenance in-house; the same lag risk with no upstream.
