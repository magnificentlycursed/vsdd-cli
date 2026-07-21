//! Bounded file reads for adopter-edited artifacts.
//!
//! Every Layer 1 read site materializes at most [`MAX_ARTIFACT_BYTES`]
//! before parsing — the capped reader the #726 ruling kept code-side,
//! landed per the round-2 correction (vsdd-cli #732). The parser's own
//! amplification and nesting limits bound expansion; this cap bounds
//! the flat pre-parse materialization, which is a memory concern of
//! this layer's reads themselves.

use std::io::Read;
use std::path::Path;

/// One mebibyte — generous against the estate's kilobyte-scale
/// artifacts; the documented limit oversize diagnostics name.
pub const MAX_ARTIFACT_BYTES: u64 = 1_048_576;

pub(crate) struct BoundedRead {
    pub bytes: Vec<u8>,
    /// True when the file exceeds [`MAX_ARTIFACT_BYTES`]; `bytes` then
    /// holds a truncated prefix that callers must not parse.
    pub oversize: bool,
}

/// Read at most the cap plus one byte, so oversize is detectable
/// without materializing the whole file.
pub(crate) fn read_bounded(path: &Path) -> std::io::Result<BoundedRead> {
    let file = std::fs::File::open(path)?;
    let mut bytes = Vec::new();
    let read = file.take(MAX_ARTIFACT_BYTES + 1).read_to_end(&mut bytes)? as u64;
    Ok(BoundedRead {
        bytes,
        oversize: read > MAX_ARTIFACT_BYTES,
    })
}
