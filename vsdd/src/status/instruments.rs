//! The conduct instruments (Layer 3): the counting stdin seam, the
//! acquisition counter, and the whole-invocation wall-clock — the
//! criterion's counts, so a regression fails a count rather than
//! passing on output invariance. Model-absence is the offline test
//! harness's property (no credentials, no network), enforced by the
//! run environment rather than asserted in code.

use std::io::Read;

/// Counts every byte the wrapped reader yields; the statusline path
/// must leave it at zero.
pub struct CountingReader<R> {
    inner: R,
    bytes: u64,
}

impl<R: Read> CountingReader<R> {
    pub fn new(inner: R) -> Self {
        Self { inner, bytes: 0 }
    }
    pub fn bytes_read(&self) -> u64 {
        self.bytes
    }
}

impl<R: Read> Read for CountingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.bytes += n as u64;
        Ok(n)
    }
}

/// What one statusline invocation observed at its seams.
pub struct InvocationInstruments {
    pub stdin_bytes_read: u64,
    pub acquisition_count: u64,
    pub wall_clock: std::time::Duration,
}
