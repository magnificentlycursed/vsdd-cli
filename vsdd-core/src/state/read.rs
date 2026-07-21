//! State read: an effectful wrapper over a pure validate-bytes core.
//!
//! Trust boundary: `.vsdd/state.yaml` is adopter-edited — every read
//! failure yields a [`Diagnostic`], never a panic. Bootstrap
//! self-validation gates on `schema_version` before reading further.

use std::path::Path;

use crate::diagnostics::Diagnostic;
use crate::registry::sets::StatuslineData;

use super::schema::State;

/// Read and validate the state artifact.
///
/// Failure kinds map exactly onto the statusline data set's
/// `read_failure_kinds`: absent, malformed (with the parser's location),
/// permission-or-io. A schema_version this crate does not support is
/// refused as malformed content naming the version seen and supported.
pub fn read_state(path: &Path, vocabulary: &StatuslineData) -> Result<State, Box<Diagnostic>> {
    let _ = (path, vocabulary);
    todo!("2b: effectful read over the pure validator")
}

/// The pure core: validate raw bytes into a `State`.
///
/// Deterministic over its inputs; the property-test and mutation target.
pub fn validate_state_bytes(
    bytes: &[u8],
    path: &Path,
    vocabulary: &StatuslineData,
) -> Result<State, Box<Diagnostic>> {
    let _ = (bytes, path, vocabulary);
    todo!("2b: pure validation with location capture")
}
