//! The vsdd binary's library surface: Layer 3's status renderings
//! (vsdd-cli #772). The binary target composes these; the lib target
//! exists so the red-gate suite exercises them — no API-stability
//! promise rides on it (the same posture mdatron's collapse declared).

pub mod status;
