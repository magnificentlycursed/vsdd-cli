---
supplement_slug: yaml
languages_or_interfaces: [YAML]
domains_in_scope: [software-engineer, platform-engineer, security]
extensions: []
---

# YAML Supplement

Per-domain extensions for YAML-bearing surfaces (CI workflows, OTel collector config, methodology frontmatter, structural rule files, `.vsdd/config.yaml`).

## Software Engineer extensions

- **Schema-validated.** Every YAML file the toolkit owns has a JSON Schema (or YAML schema) that validates it at commit-time. Unvalidated YAML is the load-bearing failure mode.
- **Anchor + alias discipline.** YAML anchors (`&anchor`) + aliases (`*anchor`) reserved for cases where the readability + maintenance gain justifies the indirection. Deep aliasing makes the spec hard to read.
- **String-vs-scalar gotchas.** `version: 1.0` parses as float (1.0), not string ("1.0"). `version: "1.0"` to force string. Per-file schema declares expected types.

## Platform Engineer extensions

- **CI workflow YAML.** GitHub Actions workflows live at `.github/workflows/*.yml`. Schema validated against [GitHub's published schema](https://json.schemastore.org/github-workflow.json). Per-step env discipline; per-job runner pinning.
- **OTel collector config YAML.** `.vsdd/otel-collector.yaml` follows OpenTelemetry collector schema. Processor ordering invariant (redaction before exporters) is structural.
- **Per-project config YAML.** `.vsdd/config.yaml` validated against the vsdd-config artifact class schema. Cross-field validation rejects invalid combinations (e.g., `auth_method.ci: plan`).

## Security extensions

- **Credential-shaped value rejection.** Schema validators reject credential-shaped patterns (regex against `sk-ant-api03-`, `Bearer <token>`, env-var-assignment-with-credential-shaped-value).
- **External-source YAML.** YAML files loaded from external sources (CI artifacts, downloaded configs) require schema validation before any field is read. Unvalidated external YAML is a deserialization attack surface.
- **No code-execution YAML loaders.** Python's `yaml.load(stream)` is unsafe; use `yaml.safe_load`. Other languages' equivalents.
