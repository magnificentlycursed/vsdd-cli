//! The effectful snapshot acquisition (the shell side of the purity
//! split): builds the [`Snapshot`](super::Snapshot) from the chassis's
//! query surface. One acquisition per invocation; the count-conduct
//! instruments join at Layer 3.
//!
//! Absent and unusable are OUTCOMES carried in the snapshot, never
//! errors: an unreachable tracker is the contracted normal offline
//! mode, and the derivation degrades with the kind rather than this
//! function failing.

use std::path::Path;

use super::Snapshot;

/// Acquire the corroboration snapshot for the repo.
pub fn acquire_snapshot(repo_root: &Path) -> Snapshot {
    let _ = repo_root;
    todo!("2b: chassis query surface; absent and unusable as outcomes, never a panic")
}
