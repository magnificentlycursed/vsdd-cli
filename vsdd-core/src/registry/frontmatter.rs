//! Pure markdown-with-frontmatter extraction with parse-location capture.
//!
//! The registry artifacts are markdown files opening with a `---`-fenced
//! YAML frontmatter block (the phase-1c format ruling, vsdd-cli #660).
//! This split is pure over the input text; failures carry the location.

/// A frontmatter extraction failure with its location in the input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontmatterError {
    /// 1-indexed line of the failure.
    pub line: usize,
    /// 1-indexed column of the failure.
    pub column: usize,
    pub message: String,
}

/// Split an artifact into `(frontmatter_yaml, body)`.
///
/// Pure; deterministic; the property-test target for the registry's
/// parse boundary. An input with no opening fence, an unterminated
/// fence, or an empty frontmatter block is an error with its location.
pub fn split_frontmatter(input: &str) -> Result<(&str, &str), FrontmatterError> {
    let _ = input;
    todo!("2b: fence scan with location capture")
}
