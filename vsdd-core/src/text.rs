//! The one terminal-cleaning policy, shared across both crates
//! (vsdd-cli #788). Every string that reaches a terminal surface — the
//! acquisition's display fields, the render tier's segment and human
//! forms, the broken-state diagnostics — passes through here, so the
//! policy has a single definition to audit and extend.
//!
//! The threat is display-spoofing, not just cursor forging: the
//! project's own hidden-Unicode rule (`.crosslink/rules/web.md`) names
//! "zero-width chars, RTL overrides — strip and re-evaluate", and the
//! Trojan-Source class (CVE-2021-42574) reorders visible text with the
//! bidi controls. So the cleaner drops the C0/C1 control set AND the
//! bidi-formatting, isolate, directional-mark, zero-width, and
//! deprecated-format code points.
//!
//! Deliberately NOT stripped: combining marks and other legitimate
//! non-Latin text shaping — over-stripping would corrupt real repo and
//! milestone names in most of the world's scripts, and they are not on
//! the threat list. The line is drawn exactly at the named set.

/// True for a code point that must never reach a terminal surface: a
/// control character or a display-affecting format/bidi code point.
pub fn is_terminal_unsafe(c: char) -> bool {
    c.is_control()
        || matches!(c,
            // Bidi embeddings and overrides (Trojan-Source).
            '\u{202A}'..='\u{202E}'
            // Directional isolates.
            | '\u{2066}'..='\u{2069}'
            // Left-to-right / right-to-left marks and the Arabic mark.
            | '\u{200E}' | '\u{200F}' | '\u{061C}'
            // Zero-width space, non-joiner, joiner.
            | '\u{200B}'..='\u{200D}'
            // Zero-width no-break space / BOM.
            | '\u{FEFF}'
            // Deprecated format controls.
            | '\u{206A}'..='\u{206F}'
        )
}

/// Drop every terminal-unsafe code point; the shared cleaner for all
/// terminal-destined text (vsdd-cli #777, #784, #788).
pub fn clean_for_terminal(s: &str) -> String {
    s.chars().filter(|c| !is_terminal_unsafe(*c)).collect()
}

#[cfg(test)]
mod tests {
    use super::clean_for_terminal;

    #[test]
    fn control_bytes_are_stripped() {
        assert_eq!(clean_for_terminal("a\u{1b}[31mb\nc"), "a[31mbc");
    }

    #[test]
    fn bidi_and_zero_width_are_stripped() {
        // The Trojan-Source set and the project's named hidden-Unicode.
        assert_eq!(clean_for_terminal("safe\u{202E}reversed"), "safereversed");
        assert_eq!(clean_for_terminal("zero\u{200B}width"), "zerowidth");
        assert_eq!(clean_for_terminal("iso\u{2066}late\u{2069}"), "isolate");
        assert_eq!(clean_for_terminal("\u{FEFF}bom"), "bom");
    }

    #[test]
    fn legitimate_text_survives() {
        // Accented and non-Latin names (combining and precomposed) are
        // data, not a threat — they render intact.
        assert_eq!(clean_for_terminal("café"), "café");
        assert_eq!(clean_for_terminal("cafe\u{0301}"), "cafe\u{0301}");
        assert_eq!(clean_for_terminal("日本語"), "日本語");
        assert_eq!(clean_for_terminal("proj-2a"), "proj-2a");
    }
}
