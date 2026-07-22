//! Bounded, timed subprocess execution for the shell tier.
//!
//! The subprocess analogue of the bounded file reader (#732): child
//! stdout materializes at most the cap, a wall-clock deadline bounds a
//! wedged child, and every breach maps to a loud outcome — the
//! consistency the Layer 2 round-1 security lens demanded (vsdd-cli #751), with
//! spawn errors classified honestly (vsdd-cli #747: only a missing
//! binary is the offline shape; a present-but-broken binary is a
//! failure, never absence).
//!
//! PATH-resolution posture, stated (vsdd-cli #754): the named binaries
//! resolve through the operator's PATH — a hostile binary shadowing
//! PATH is the operator's own environment and outside this model, the
//! same trust the operator's shell extends.

use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Shares the file-read cap: one documented bound for materialization.
pub(crate) const SUBPROCESS_OUTPUT_CAP: u64 = crate::bounded_read::MAX_ARTIFACT_BYTES;
pub(crate) const SUBPROCESS_DEADLINE: Duration = Duration::from_secs(10);

pub(crate) enum Subprocess {
    /// Exit zero within every bound.
    Completed { stdout: String },
    /// The binary is not on PATH — the offline shape, nothing more.
    NotFound,
    /// Spawn failed with the binary present (permissions, resources):
    /// broken, never offline.
    SpawnBroken(String),
    /// Ran past the deadline; killed.
    TimedOut,
    /// Ran and exited nonzero; stderr truncated for diagnostics.
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
    // pipe before try_wait sees the exit.
    let mut stdout_pipe = child.stdout.take().expect("stdout piped");
    let mut stderr_pipe = child.stderr.take().expect("stderr piped");
    let stdout_reader = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let read = stdout_pipe
            .by_ref()
            .take(SUBPROCESS_OUTPUT_CAP + 1)
            .read_to_end(&mut buf)
            .unwrap_or(0) as u64;
        let _ = std::io::copy(&mut stdout_pipe, &mut std::io::sink());
        (buf, read > SUBPROCESS_OUTPUT_CAP)
    });
    let stderr_reader = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stderr_pipe.by_ref().take(4096).read_to_end(&mut buf);
        let _ = std::io::copy(&mut stderr_pipe, &mut std::io::sink());
        buf
    });

    let deadline = Instant::now() + SUBPROCESS_DEADLINE;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                // The reader threads finish as the pipes close; nothing
                // waits on them past the kill.
                return Subprocess::TimedOut;
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(25)),
            Err(e) => return Subprocess::SpawnBroken(e.to_string()),
        }
    };

    let (stdout, oversize) = stdout_reader.join().unwrap_or_default();
    let stderr = stderr_reader.join().unwrap_or_default();
    if oversize {
        return Subprocess::Oversize;
    }
    if !status.success() {
        return Subprocess::Refused {
            stderr: String::from_utf8_lossy(&stderr).chars().take(500).collect(),
        };
    }
    Subprocess::Completed {
        stdout: String::from_utf8_lossy(&stdout).into_owned(),
    }
}
