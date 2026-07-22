//! Bounded, timed subprocess execution for the shell tier.
//!
//! The subprocess analogue of the bounded file reader (#732): child
//! stdout materializes at most the cap, a wall-clock deadline bounds a
//! wedged child, and every breach maps to a loud outcome — the
//! consistency the Layer 2 round-1 security lens demanded (vsdd-cli
//! #751), with spawn errors classified honestly (vsdd-cli #747: only a
//! missing binary is the offline shape; a present-but-broken binary is
//! a failure, never absence). The deadline covers the WHOLE run
//! (vsdd-cli #756): the wait on the child and the reads of its pipes
//! share one budget, so a descendant process holding the inherited
//! write-ends past the child's own exit is a timeout, never an
//! unbounded join — on breach the detached reader threads are
//! abandoned and the run reports what happened. A failed read with a
//! clean exit is loud for stdout (a silently truncated success is the
//! narrowing this module exists to refuse); stderr capture is
//! best-effort, since it garnishes diagnostics and never carries data.
//!
//! Refused stderr returns WHOLE up to its capture bound: the consumer
//! sanitizes machine-identifying text FIRST and truncates for display
//! after (vsdd-cli #760 — truncating first can bisect an absolute path
//! and defeat the exact-match replace).
//!
//! PATH-resolution posture, stated (vsdd-cli #754): the named binaries
//! resolve through the operator's PATH — a hostile binary shadowing
//! PATH is the operator's own environment and outside this model, the
//! same trust the operator's shell extends.

use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

/// Shares the file-read cap: one documented bound for materialization.
pub(crate) const SUBPROCESS_OUTPUT_CAP: u64 = crate::bounded_read::MAX_ARTIFACT_BYTES;
pub(crate) const SUBPROCESS_DEADLINE: Duration = Duration::from_secs(10);
/// Stderr capture bound; display truncation happens at the diagnostic
/// site, after sanitization (vsdd-cli #760).
pub(crate) const SUBPROCESS_STDERR_CAP: u64 = 4096;

pub(crate) enum Subprocess {
    /// Exit zero within every bound.
    Completed { stdout: String },
    /// The binary is not on PATH — the offline shape, nothing more.
    NotFound,
    /// Spawn, wait, or read failed with the binary present: broken,
    /// never offline.
    SpawnBroken(String),
    /// The run — child or its pipes — outlived the deadline.
    TimedOut,
    /// Ran and exited nonzero; stderr whole up to the capture bound,
    /// for the consumer to sanitize then truncate.
    Refused { stderr: String },
    /// Stdout exceeded the cap.
    Oversize,
}

pub(crate) fn run_bounded(program: &str, args: &[&str], cwd: &Path) -> Subprocess {
    let mut child = match Command::new(program)
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Subprocess::NotFound,
        Err(e) => return Subprocess::SpawnBroken(e.to_string()),
    };

    // Pipe readers on threads so a chatty child cannot deadlock a full
    // pipe before try_wait sees the exit; results come back over
    // channels so the collection below can carry the deadline too.
    let mut stdout_pipe = child.stdout.take().expect("stdout piped");
    let mut stderr_pipe = child.stderr.take().expect("stderr piped");
    let (stdout_tx, stdout_rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut buf = Vec::new();
        let result = stdout_pipe
            .by_ref()
            .take(SUBPROCESS_OUTPUT_CAP + 1)
            .read_to_end(&mut buf)
            .map(|read| (buf, read as u64 > SUBPROCESS_OUTPUT_CAP));
        let _ = std::io::copy(&mut stdout_pipe, &mut std::io::sink());
        let _ = stdout_tx.send(result);
    });
    let (stderr_tx, stderr_rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stderr_pipe
            .by_ref()
            .take(SUBPROCESS_STDERR_CAP)
            .read_to_end(&mut buf);
        let _ = std::io::copy(&mut stderr_pipe, &mut std::io::sink());
        let _ = stderr_tx.send(buf);
    });

    let deadline = Instant::now() + SUBPROCESS_DEADLINE;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                return Subprocess::TimedOut;
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(25)),
            Err(e) => {
                // The wait surface itself broke: stop the child before
                // reporting rather than leaving it running unbounded
                // (vsdd-cli #756), mirroring the timeout arm.
                let _ = child.kill();
                let _ = child.wait();
                return Subprocess::SpawnBroken(format!("wait on the child failed: {e}"));
            }
        }
    };

    // The child exited, but a descendant may hold the inherited pipe
    // write-ends open: the reads get the REMAINING budget, never an
    // unbounded join (vsdd-cli #756).
    let remaining = deadline.saturating_duration_since(Instant::now());
    let (stdout_buf, oversize) = match stdout_rx.recv_timeout(remaining) {
        Ok(Ok(pair)) => pair,
        // A read failure under a clean exit would otherwise ship a
        // silently truncated success — loud instead (vsdd-cli #756).
        Ok(Err(e)) => return Subprocess::SpawnBroken(format!("stdout read failed: {e}")),
        Err(mpsc::RecvTimeoutError::Timeout) => return Subprocess::TimedOut,
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            return Subprocess::SpawnBroken(
                "the stdout reader stopped without reporting".to_string(),
            )
        }
    };
    let remaining = deadline.saturating_duration_since(Instant::now());
    let stderr_buf = match stderr_rx.recv_timeout(remaining) {
        Ok(buf) => buf,
        Err(mpsc::RecvTimeoutError::Timeout) => return Subprocess::TimedOut,
        // Best-effort: lost stderr degrades a diagnostic's garnish,
        // never the run's data.
        Err(mpsc::RecvTimeoutError::Disconnected) => Vec::new(),
    };

    if oversize {
        return Subprocess::Oversize;
    }
    if !status.success() {
        return Subprocess::Refused {
            stderr: String::from_utf8_lossy(&stderr_buf).into_owned(),
        };
    }
    Subprocess::Completed {
        stdout: String::from_utf8_lossy(&stdout_buf).into_owned(),
    }
}
