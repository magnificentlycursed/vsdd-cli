//! State read: an effectful wrapper over a pure validate-bytes core.
//!
//! Trust boundary: `.vsdd/state.yaml` is adopter-edited — every read
//! failure yields a [`Diagnostic`], never a panic. Bootstrap
//! self-validation gates on `schema_version` before reading further.

use std::path::Path;

use crate::diagnostics::{yaml_location, Diagnostic, StateReadKind};
use crate::registry::sets::StatuslineData;

use super::schema::{State, SUPPORTED_STATE_SCHEMA_VERSION};

/// Read and validate the state artifact.
///
/// Failure kinds map exactly onto the statusline data set's
/// `read_failure_kinds`: absent, malformed (with the parser's location),
/// permission-or-io. A schema_version this crate does not support is
/// refused as malformed content naming the version seen and supported.
pub fn read_state(path: &Path, vocabulary: &StatuslineData) -> Result<State, Box<Diagnostic>> {
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) => {
            let kind = if e.kind() == std::io::ErrorKind::NotFound {
                StateReadKind::Absent
            } else {
                StateReadKind::PermissionOrIo
            };
            return Err(Box::new(Diagnostic::state_read_failure(
                kind,
                path.to_path_buf(),
                format!("cannot read the state file: {e}"),
                None,
                vocabulary,
            )));
        }
    };
    validate_state_bytes(&bytes, path, vocabulary)
}

fn malformed(
    path: &Path,
    detail: String,
    location: Option<(usize, usize)>,
    vocabulary: &StatuslineData,
) -> Box<Diagnostic> {
    Box::new(Diagnostic::state_read_failure(
        StateReadKind::Malformed,
        path.to_path_buf(),
        detail,
        location,
        vocabulary,
    ))
}

/// The pure core: validate raw bytes into a `State`.
///
/// Deterministic over its inputs; the property-test and mutation target.
pub fn validate_state_bytes(
    bytes: &[u8],
    path: &Path,
    vocabulary: &StatuslineData,
) -> Result<State, Box<Diagnostic>> {
    let text = std::str::from_utf8(bytes).map_err(|e| {
        malformed(
            path,
            format!("the state file is not valid UTF-8: {e}"),
            None,
            vocabulary,
        )
    })?;

    let value: serde_yaml_ng::Value = serde_yaml_ng::from_str(text).map_err(|e| {
        malformed(
            path,
            format!("parse failure: {e}"),
            yaml_location(&e, 0),
            vocabulary,
        )
    })?;
    if !value.is_mapping() {
        return Err(malformed(
            path,
            "the state file is empty or not a mapping".to_string(),
            None,
            vocabulary,
        ));
    }

    // Bootstrap self-validation: the version gate fires before any
    // further reading of the contents.
    let version = value
        .get("schema_version")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if version != SUPPORTED_STATE_SCHEMA_VERSION {
        return Err(malformed(
            path,
            format!(
                "state schema_version `{version}` is not supported; this reader supports `{SUPPORTED_STATE_SCHEMA_VERSION}`"
            ),
            None,
            vocabulary,
        ));
    }

    serde_yaml_ng::from_str::<State>(text).map_err(|e| {
        malformed(
            path,
            format!("state content failure: {e}"),
            yaml_location(&e, 0),
            vocabulary,
        )
    })
}
