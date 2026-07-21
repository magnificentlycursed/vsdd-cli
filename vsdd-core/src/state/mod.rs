//! The `.vsdd/state.yaml` artifact: schema, read, and write
//! (contract: Deterministic phase answer; Layer 1 of the decomposition).

pub mod read;
pub mod schema;
pub mod write;

pub use read::{read_state, validate_state_bytes};
pub use schema::{
    ActiveComposition, CompositionMode, GateKind, GateOutcome, GateResult, OpenFindingsPointer,
    Published, State, SUPPORTED_STATE_SCHEMA_VERSION,
};
pub use write::{published_unchanged, write_state, BoundaryEvidence};
