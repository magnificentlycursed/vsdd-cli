//! State write: atomic, boundary-evidenced, forward-only on `published`.
//!
//! The state advances only at phase boundaries, in the same commit as
//! the boundary evidence — the write takes that evidence as a required
//! input. The write is temp-plus-rename atomic: no partial file survives
//! a failure (the Install requirement's no-partial-write discipline,
//! applied to the state artifact).

use std::path::{Path, PathBuf};

use crate::diagnostics::Diagnostic;
use crate::registry::sets::StatuslineData;

use super::read::read_state;
use super::schema::{GateKind, GateOutcome, State};

/// The boundary evidence a state advance records with.
#[derive(Debug, Clone)]
pub struct BoundaryEvidence {
    /// The boundary commit sha or tracker handle; never empty.
    pub commit: String,
}

fn refusal(file: PathBuf, message: String) -> Box<Diagnostic> {
    Box::new(Diagnostic {
        file,
        kind: "refused".to_string(),
        machine_token: "refused".to_string(),
        location: None,
        message,
        recovery_action: "correct-the-write".to_string(),
        recovery_text: "correct the refused write's input and retry; the prior file is untouched"
            .to_string(),
    })
}

/// Write the state artifact atomically.
///
/// Refused with a diagnostic when: the evidence is empty; a prior state
/// exists and the new state's `published` block touches a written one
/// (forward-only immutability); or the self-contained phase-gate
/// consistency constraint fails — entering phase-2b requires the
/// layer's red-gate fail record in `last_gate_result`. Scope-member
/// validity joins when the composition consumer lands (Layer 5), per
/// the declared-constraints enumeration. A write failure leaves the
/// prior file untouched — no partial write survives.
pub fn write_state(
    path: &Path,
    state: &State,
    evidence: &BoundaryEvidence,
    vocabulary: &StatuslineData,
) -> Result<(), Box<Diagnostic>> {
    if evidence.commit.trim().is_empty() {
        return Err(refusal(
            path.to_path_buf(),
            "the state advances only with boundary evidence; the evidence reference is empty"
                .to_string(),
        ));
    }

    if path.exists() {
        let prior = read_state(path, vocabulary)?;
        if !published_unchanged(&prior, state) {
            return Err(refusal(
                path.to_path_buf(),
                "the published marker is forward-only: a write touching a written published block is refused"
                    .to_string(),
            ));
        }
    }

    if state.current_phase.as_deref() == Some("phase-2b") {
        let red_recorded = state.last_gate_result.as_ref().is_some_and(|gate| {
            gate.gate == GateKind::RedGate
                && gate.result == GateOutcome::Fail
                && state.current_layer == Some(gate.layer)
        });
        if !red_recorded {
            return Err(refusal(
                path.to_path_buf(),
                "phase-gate consistency: a state entering phase-2b requires the layer's red-gate fail record in last_gate_result"
                    .to_string(),
            ));
        }
    }

    let yaml = serde_yaml_ng::to_string(state).map_err(|e| {
        refusal(
            path.to_path_buf(),
            format!("the state could not be serialized: {e}"),
        )
    })?;

    let dir = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "state.yaml".to_string());
    let tmp = dir.join(format!(".{file_name}.tmp"));

    std::fs::write(&tmp, yaml).map_err(|e| {
        refusal(
            path.to_path_buf(),
            format!("write failure before any content landed: {e}"),
        )
    })?;
    if let Err(e) = std::fs::rename(&tmp, path) {
        let _ = std::fs::remove_file(&tmp);
        return Err(refusal(
            path.to_path_buf(),
            format!("write failure at the atomic rename; the temp file was removed: {e}"),
        ));
    }
    Ok(())
}

/// The pure immutability check: true exactly when a written `published`
/// block survives unchanged into the next state. Property-test target.
pub fn published_unchanged(prior: &State, next: &State) -> bool {
    match (&prior.published, &next.published) {
        (None, _) => true,
        (Some(p), Some(n)) => p == n,
        (Some(_), None) => false,
    }
}
