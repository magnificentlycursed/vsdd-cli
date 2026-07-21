//! The off-grammar branch query (contract: Conformance at action time,
//! the branch-grammar seam): a pure membership core over injected ref
//! names, consuming the registered grammar — both forms perpetually
//! valid, the exemption set as data (vsdd-cli #688 addendum), decidable
//! from the ref alone. The git listing is the shell's.

use std::path::Path;

use crate::diagnostics::Diagnostic;
use crate::registry::sets::BranchGrammar;

/// The refs the query runs over: this clone's own branches — local refs
/// and their remote-tracking counterparts (the shell half).
pub fn local_refs(repo_root: &Path) -> Result<Vec<String>, Box<Diagnostic>> {
    let _ = repo_root;
    todo!("2b: git branch listing, never a panic")
}

/// The pure membership core: refs matching neither registered form and
/// not exempt are off-grammar. An invalid pattern in the registered
/// data is a diagnostic naming the grammar — the data is adopter-owned.
pub fn off_grammar_refs(
    refs: &[String],
    grammar: &BranchGrammar,
) -> Result<Vec<String>, Box<Diagnostic>> {
    let _ = (refs, grammar);
    todo!("2b: regex membership over the registered forms plus the exemption set")
}
