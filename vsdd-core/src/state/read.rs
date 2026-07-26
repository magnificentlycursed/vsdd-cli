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
    let bounded = match crate::bounded_read::read_bounded(path) {
        Ok(bounded) => bounded,
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
    if bounded.oversize {
        return Err(malformed(
            path,
            format!(
                "the state file exceeds the reader's {} byte limit and was not parsed",
                crate::bounded_read::MAX_ARTIFACT_BYTES
            ),
            None,
            vocabulary,
        ));
    }
    validate_state_bytes(&bounded.bytes, path, vocabulary)
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
    // further reading of the contents, and its message distinguishes
    // missing, non-string, and unsupported (vsdd-cli #727).
    match value.get("schema_version") {
        None => {
            return Err(malformed(
                path,
                format!(
                    "the state file has no schema_version key; this reader supports `{SUPPORTED_STATE_SCHEMA_VERSION}`"
                ),
                None,
                vocabulary,
            ));
        }
        Some(v) => match v.as_str() {
            None => {
                return Err(malformed(
                    path,
                    format!(
                        "schema_version is not a string (quote it, e.g. \"0.1.0\"); this reader supports `{SUPPORTED_STATE_SCHEMA_VERSION}`"
                    ),
                    None,
                    vocabulary,
                ));
            }
            Some(version) if version != SUPPORTED_STATE_SCHEMA_VERSION => {
                return Err(malformed(
                    path,
                    format!(
                        "state schema_version `{version}` is not supported; this reader supports `{SUPPORTED_STATE_SCHEMA_VERSION}`"
                    ),
                    None,
                    vocabulary,
                ));
            }
            Some(_) => {}
        },
    }

    let mut state = serde_yaml_ng::from_str::<State>(text).map_err(|e| {
        malformed(
            path,
            format!("state content failure: {e}"),
            yaml_location(&e, 0),
            vocabulary,
        )
    })?;
    // State display strings are cleaned at THIS source boundary (vsdd-cli
    // #799), the peer of the acquisition's snapshot cleaning and the
    // registry loader's PostLoad: so every downstream render form — the
    // segment, the human form, AND the machine form — receives
    // terminal-safe data without a per-sink clean, and the answer's
    // verbatim echo of the composition (vsdd-cli #749) stays an echo of
    // the already-clean state.
    if let Some(phase) = state.current_phase.as_mut() {
        *phase = crate::text::clean_for_terminal(phase);
    }
    state.active_composition.scope =
        crate::text::clean_for_terminal(&state.active_composition.scope);
    for domain in &mut state.active_composition.domains {
        *domain = crate::text::clean_for_terminal(domain);
    }
    Ok(state)
}
