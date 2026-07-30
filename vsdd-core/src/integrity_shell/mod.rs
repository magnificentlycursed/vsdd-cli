//! The shell-side integrity checks that cannot materialize into the
//! snapshot (contract: Verification architecture): the refs query over
//! git references and the installed-artifact-integrity check over the filesystem.
//! Crosslink's unsigned-event count is consumed, not computed, and
//! joins when its surface is wired (Layer 3's Status assembly).

pub mod refs;
pub mod installed_artifact;
