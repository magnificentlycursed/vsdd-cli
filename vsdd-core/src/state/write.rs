//! State write: atomic, boundary-evidenced, forward-only on `published`.
//!
//! The state advances only at phase boundaries, in the same commit as
//! the boundary evidence — the write takes that evidence as a required
//! input. The write is unique-temp-plus-rename atomic against
//! IN-PROCESS failure orderings: no partial file survives at the target
//! path or as temp residue. Crash durability (power loss reordering the
//! rename ahead of data blocks) is explicitly out of scope — the state
//! artifact is boundary-committed, and the read-failure discipline's
//! restore-from-boundary recovery is the designed crash path (operator
//! ruling 2026-07-21, vsdd-cli #726).
//!
//! Concurrency posture, stated honestly: the prior-state read and the
//! final rename are separate acts with no lock between them, so a
//! concurrent second writer can produce a lost update. The unique temp
//! name removes temp-file collisions (vsdd-cli #721, #726); the
//! remaining read-to-rename window stands documented rather than
//! claimed away.

use std::io::Write as _;
use std::path::{Path, PathBuf};

use crate::diagnostics::Diagnostic;
use crate::registry::sets::{CompositionScopeAndActions, StatuslineData};

use super::read::validate_state_bytes;
use super::schema::{GateKind, GateOutcome, State};

/// The boundary evidence a state advance records with.
#[derive(Debug, Clone)]
pub struct BoundaryEvidence {
    /// The boundary commit sha or tracker handle; never empty.
    pub commit: String,
}

/// A write refusal; its recovery members load from the action
/// vocabulary's registered `correct-the-write` member, never from a
/// constant (operator registration 2026-07-21, vsdd-cli #724).
fn refusal(
    file: PathBuf,
    message: String,
    actions: &CompositionScopeAndActions,
) -> Box<Diagnostic> {
    let member = actions
        .action_vocabulary
        .iter()
        .find(|a| a.family == "recovery" && a.id == "correct-the-write");
    let (action, text) = match member {
        Some(m) => (m.id.clone(), m.human.clone()),
        // Total rather than panicking at a trust boundary; the E0212-class
        // pattern and the fidelity test make this arm unreachable against
        // a validated vocabulary.
        None => (String::new(), String::new()),
    };
    Box::new(Diagnostic {
        file,
        kind: "refused".to_string(),
        machine_token: "refused".to_string(),
        location: None,
        message,
        recovery_action: action,
        recovery_text: text,
    })
}

/// Write the state artifact atomically (in-process scope; see the
/// module doc for the crash and concurrency posture).
///
/// Refused with a diagnostic when: the evidence is empty; a prior state
/// exists and the new state's `published` block touches a written one
/// (forward-only immutability); or the self-contained phase-gate
/// consistency constraint fails — entering phase-2b requires the
/// layer's red-gate fail record in `last_gate_result`. Scope-member
/// validity joins when the composition consumer lands (Layer 5), per
/// the declared-constraints enumeration.
///
/// The prior state is taken by a single read — `NotFound` is the
/// no-prior case and every other IO failure is a write-shaped refusal,
/// never a read-vocabulary diagnostic leaking from a write API
/// (vsdd-cli #721).
pub fn write_state(
    path: &Path,
    state: &State,
    evidence: &BoundaryEvidence,
    vocabulary: &StatuslineData,
    actions: &CompositionScopeAndActions,
) -> Result<(), Box<Diagnostic>> {
    if evidence.commit.trim().is_empty() {
        return Err(refusal(
            path.to_path_buf(),
            "the state advances only with boundary evidence; the evidence reference is empty"
                .to_string(),
            actions,
        ));
    }

    let prior_bytes = match std::fs::read(path) {
        Ok(bytes) => Some(bytes),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => {
            return Err(refusal(
                path.to_path_buf(),
                format!("cannot read the prior state for the forward-only check: {e}"),
                actions,
            ));
        }
    };
    if let Some(bytes) = prior_bytes {
        // A malformed prior is honestly a read failure of the state
        // artifact; its read-vocabulary diagnostic applies.
        let prior = validate_state_bytes(&bytes, path, vocabulary)?;
        if !published_unchanged(&prior, state) {
            return Err(refusal(
                path.to_path_buf(),
                "the published marker is forward-only: a write touching a written published block is refused"
                    .to_string(),
                actions,
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
                actions,
            ));
        }
    }

    let yaml = serde_yaml_ng::to_string(state).map_err(|e| {
        refusal(
            path.to_path_buf(),
            format!("the state could not be serialized: {e}"),
            actions,
        )
    })?;

    let dir = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    // Unique temp name plus RAII cleanup: a failed write drops the temp
    // file on every failure ordering, and concurrent writers cannot
    // collide on the temp path (vsdd-cli #721, #726).
    let mut tmp = tempfile::NamedTempFile::new_in(dir).map_err(|e| {
        refusal(
            path.to_path_buf(),
            format!("write failure creating the temp file: {e}"),
            actions,
        )
    })?;
    tmp.write_all(yaml.as_bytes()).map_err(|e| {
        refusal(
            path.to_path_buf(),
            format!("write failure into the temp file; the temp is removed: {e}"),
            actions,
        )
    })?;
    tmp.persist(path).map_err(|e| {
        refusal(
            path.to_path_buf(),
            format!(
                "write failure at the atomic rename; the temp is removed: {}",
                e.error
            ),
            actions,
        )
    })?;
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
