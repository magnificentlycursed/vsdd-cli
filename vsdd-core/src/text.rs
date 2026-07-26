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
//! bidi controls. The policy strips by Unicode General_Category rather
//! than a hand-list (vsdd-cli #793): two rounds proved an enumeration
//! is always one adversarial probe short of the class. It drops the
//! control category (`Cc` — C0/C1), the entire format category (`Cf` —
//! bidi overrides, isolates, directional marks, zero-width, word
//! joiner, the deprecated set, interlinear annotation, and the tag
//! block that smuggles ASCII), and the line/paragraph separators
//! (`Zl`/`Zp`) that would break a one-line surface.
//!
//! Deliberately NOT stripped: combining marks (`Mn`/`Mc`/`Me`) and
//! normal spaces (`Zs`) — legitimate text, not on the threat list;
//! over-stripping would corrupt real repo and milestone names in most
//! of the world's scripts. The line is drawn at the category boundary.
//!
//! Scope, declared (vsdd-cli #796): this policy covers the STATUS
//! surfaces (the segment, human, machine, and broken-state renderings)
//! and the registry strings they consume. The `vsdd init` command
//! surface — its preflight report and init-error prints — is Layer 4's
//! (Install) and routes its own operator-local strings (cwd, on-PATH
//! tool versions) through this same helper when Layer 4 hardens; the
//! cleaner is available to it, the wiring is that layer's act.
//!
//! Degenerate edge, accepted (vsdd-cli #801): a schema-legal display
//! string made ENTIRELY of format characters cleans to empty here,
//! after the schema's minLength-1 check has passed. This is safe — the
//! render layer words an empty display value as its registered absence
//! rather than an empty slot — so the post-clean empty is an accepted
//! degenerate rendering, not a schema breach to re-diagnose.

use unicode_general_category::{get_general_category, GeneralCategory};

/// True for a code point that must never reach a terminal surface: a
/// control, a format character, or a line/paragraph separator (by
/// category, so a new Unicode format character is covered without a
/// code change — vsdd-cli #793), OR an UNASSIGNED code point in a
/// default-ignorable reserved range (vsdd-cli #798). The category proxy
/// covers only ASSIGNED invisibles; a reserved-but-default-ignorable
/// code point like U+E0000 renders invisibly on conformant terminals
/// today while categorized `Cn` (Unassigned), so it must be stripped by
/// range. The gate on `Unassigned` is deliberate: the same E0000-E0FFF
/// block holds the variation selectors (U+E0100-E01EF, category `Mn`),
/// which are legitimate text and must survive.
pub fn is_terminal_unsafe(c: char) -> bool {
    let category = get_general_category(c);
    matches!(
        category,
        GeneralCategory::Control
            | GeneralCategory::Format
            | GeneralCategory::LineSeparator
            | GeneralCategory::ParagraphSeparator
    ) || (category == GeneralCategory::Unassigned && in_default_ignorable_reserved(c))
}

/// The default-ignorable reserved ranges (Unicode
/// DerivedCoreProperties, the reserved half of
/// `Default_Ignorable_Code_Point`): invisible today, unassigned today.
fn in_default_ignorable_reserved(c: char) -> bool {
    matches!(c, '\u{2065}' | '\u{FFF0}'..='\u{FFF8}' | '\u{E0000}'..='\u{E0FFF}')
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
    fn the_whole_format_and_separator_class_is_stripped() {
        // The category boundary (vsdd-cli #793), member by member across
        // the class the two rounds probed: bidi, zero-width, the word
        // joiner and invisible operators, the deprecated set, BOM, the
        // interlinear annotations, the tag block (ASCII smuggling), the
        // soft hyphen, and the line/paragraph separators.
        assert_eq!(clean_for_terminal("safe\u{202E}reversed"), "safereversed");
        assert_eq!(clean_for_terminal("zero\u{200B}width"), "zerowidth");
        assert_eq!(clean_for_terminal("iso\u{2066}late\u{2069}"), "isolate");
        assert_eq!(clean_for_terminal("\u{FEFF}bom"), "bom");
        assert_eq!(clean_for_terminal("word\u{2060}joiner"), "wordjoiner");
        assert_eq!(clean_for_terminal("op\u{2061}\u{2064}s"), "ops");
        assert_eq!(clean_for_terminal("soft\u{00AD}hyphen"), "softhyphen");
        assert_eq!(
            clean_for_terminal("inter\u{FFF9}lin\u{FFFB}ear"),
            "interlinear"
        );
        // The tag block smuggles ASCII invisibly into agent-consumed text.
        assert_eq!(clean_for_terminal("tag\u{E0068}\u{E0069}"), "tag");
        // Line and paragraph separators would break a one-line surface.
        assert_eq!(clean_for_terminal("a\u{2028}b\u{2029}c"), "abc");
        // Unassigned default-ignorables render invisibly but are Cn, not
        // Cf — stripped by range (vsdd-cli #798).
        assert_eq!(clean_for_terminal("tag\u{E0000}block"), "tagblock");
        assert_eq!(clean_for_terminal("ign\u{2065}ore"), "ignore");
        assert_eq!(clean_for_terminal("re\u{FFF0}served"), "reserved");
    }

    #[test]
    fn variation_selectors_survive() {
        // The E0100-E01EF block shares the tag block's page but is Mn
        // (legitimate emoji/glyph variation), not Unassigned — kept
        // (vsdd-cli #798).
        let vs = "text\u{E0100}";
        assert_eq!(clean_for_terminal(vs), vs);
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
