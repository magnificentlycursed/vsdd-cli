---
supplement_slug: json
languages_or_interfaces: [JSON, JSON Schema]
domains_in_scope: [software-engineer, platform-engineer, security]
extensions: []
---

# JSON Supplement

Per-domain extensions for JSON-bearing surfaces (`.claude/mcp.json`, JSON Schema files, NDJSON event logs, SARIF emission, API payloads).

## Software Engineer extensions

- **JSON Schema discipline.** Every JSON file the toolkit owns has a JSON Schema (draft 2020-12) that validates it. Schemas live at `vsdd-core/schemas/<class>.json`.
- **Strict mode parsing.** Reject trailing commas, comments, single-quoted strings — these are JSON5 / JSONC extensions, not JSON. The methodology's JSON files are strict JSON.
- **Stable serialization for diffing.** Keys sorted alphabetically; consistent indentation; trailing newline. Stable serialization makes diffs minimal.
- **NDJSON for append-only logs.** Newline-delimited JSON for `.vsdd/events.jsonl` — one JSON object per line, no trailing comma; append-friendly.

## Platform Engineer extensions

- **SARIF 2.1.0 output.** `vsdd verify check --format sarif` emits the [SARIF 2.1.0 schema](https://docs.oasis-open.org/sarif/sarif/v2.1.0/). GitHub Code Scanning consumes natively; findings appear in PR Conversation tab.
- **JSON Schema code generation.** Rust types are source-of-truth; `schemars` generates JSON Schema at build time. One source; multiple consumers (Python hooks + Rust mirror + LSP).
- **`.claude/mcp.json` JSON object merge.** `vsdd init` merges its MCP server entry into the existing file; preserves operator entries; warns on key collision.

## Security extensions

- **Credential-shaped value rejection.** Schema validators reject credential-shaped patterns in JSON payloads. Per the credential-exclusion structural property applied to event payload schemas.
- **External-source JSON.** JSON from external sources (HTTP responses, downloaded artifacts) requires schema validation before deserialization. Unvalidated external JSON is a deserialization attack surface.
- **JSON injection in templated strings.** When constructing JSON via string concatenation, untrusted input can break the JSON structure. Use serialization libraries; never string-concat JSON.
