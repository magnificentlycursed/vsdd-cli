//! The schema-pair validation shim (vsdd-cli #764): the one place vsdd
//! compiles and executes a JSON Schema, in-crate since the library seam
//! died (the #739 workspace marker; mdatron #81 collapsed its workspace
//! to a single binary-first crate upstream). Same engine, version, and
//! draft as mdatron runs, so the two tools' verdicts stay aligned; the
//! mdatron BINARY remains the walk's executor — this shim serves only
//! vsdd's own read-time validation of its registry artifacts.
//!
//! Messages are VALUE-FREE: they name where the violation sits (the
//! instance path) and which constraint it broke (the schema path),
//! never the failing content itself — validated text must not ride
//! unmarked into agent-consumed diagnostics. The full rustc-shaped
//! human rendering with marked quotation is mdatron's own surface and
//! joins vsdd's diagnostic-format alignment at Layer 3.

use serde_json::Value as JsonValue;
use serde_yaml_ng::Value as YamlValue;

/// A compiled JSON Schema (draft 2020-12), compiled once per pair and
/// validated against many instances.
pub struct Schema {
    compiled: jsonschema::JSONSchema,
}

/// A single validation failure, value-free.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaViolation {
    /// JSON Pointer into the instance (empty at root).
    pub instance_path: String,
    /// Schema path of the violated constraint
    /// (e.g. `/properties/phase/enum`).
    pub schema_path: String,
    /// Engine-independent, value-free description.
    pub message: String,
}

impl Schema {
    /// Compile a schema from its parsed JSON representation; an invalid
    /// schema is an error string for the caller's diagnostic.
    pub fn compile(schema_json: &JsonValue) -> Result<Self, String> {
        let compiled = jsonschema::JSONSchema::options()
            .with_draft(jsonschema::Draft::Draft202012)
            .compile(schema_json)
            .map_err(|e| format!("schema compile failed at `{}`", e.schema_path))?;
        Ok(Self { compiled })
    }

    /// Validate a YAML value; empty on conformance.
    pub fn validate(&self, value: &YamlValue) -> Vec<SchemaViolation> {
        let json = match serde_json::to_value(value) {
            Ok(j) => j,
            // A YAML shape JSON cannot carry (a non-string map key) is
            // itself a violation, reported not swallowed.
            Err(_) => {
                return vec![SchemaViolation {
                    instance_path: String::new(),
                    schema_path: String::new(),
                    message: "the value cannot be represented as JSON for validation".to_string(),
                }]
            }
        };
        let mut violations = Vec::new();
        if let Err(errors) = self.compiled.validate(&json) {
            for e in errors {
                violations.push(SchemaViolation {
                    instance_path: e.instance_path.to_string(),
                    schema_path: e.schema_path.to_string(),
                    // Value-free by construction: the constraint's own
                    // name is the last schema-path segment.
                    message: format!(
                        "violates the `{}` constraint",
                        e.schema_path
                            .to_string()
                            .rsplit('/')
                            .next()
                            .filter(|s| !s.is_empty())
                            .unwrap_or("schema")
                    ),
                });
            }
        }
        violations
    }
}
