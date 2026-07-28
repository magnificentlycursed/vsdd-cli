//! The one terminal-cleaning policy, shared across both crates (contract:
//! Terminal output safety, vsdd-cli #807; the shared helper first landed
//! at #788). Every string vsdd emits to a terminal or agent-consumed
//! surface that is sourced from outside the tool's own compiled-in
//! constants passes through here, so the policy has one definition to
//! audit.
//!
//! The threat is display-spoofing and invisible smuggling into agent
//! context — the bidirectional-reordering Trojan Source class
//! (CVE-2021-42574) — and the rule is the outbound peer of the inbound
//! hidden-Unicode rule crosslink carries for web content
//! (`.crosslink/rules/web.md`). The class is defined BY UNICODE PROPERTY,
//! never a hand-enumerated list (six phase-3 rounds proved an enumeration
//! is always one adversarial probe short of the class): the union of the
//! control category (`Cc`), the format category (`Cf` — bidi overrides,
//! isolates, zero-width, the word joiner, the tag block that smuggles
//! ASCII), the line and paragraph separators (`Zl`/`Zp`), and every code
//! point carrying `Default_Ignorable_Code_Point` from the Unicode
//! Character Database (via `icu_properties`, compiled_data). The property
//! subsumes what the old classifier enumerated by range (the reserved
//! ignorables) and by list (the Hangul fillers), and it WINS over
//! combining-mark preservation: the variation selectors and the combining
//! grapheme joiner carry the property and are stripped (the ratified
//! ruling reversing #798 — the selector-run payload channel outweighs
//! rendering fidelity on these surfaces; the base character survives).
//!
//! Preserved: combining marks that do NOT carry the property (`Mn`/`Mc`/
//! `Me` outside it — the accents and marks of the world's scripts) and
//! normal spaces (`Zs`). Accepted residuals (contract): blank-but-cell-
//! occupying code points outside the class, combining-mark stacking
//! (bounded in practice by the field budgets), and the visible-character
//! threats (confusables, implicit reordering) whose defense is detection,
//! not stripping.
//!
//! Two sinks live here: [`clean_for_terminal`] cleans a string at a
//! source boundary or a composition point, and [`clean_json_strings`]
//! sanitizes a whole serialized machine-form value — every string value
//! AND every object key — so no field is missed whatever the struct's
//! shape (contract: the whole-of-output machine-form pass). Scope binds
//! now on the Status surfaces; the `vsdd init` surface (Layer 4) and the
//! other agent-consumed surfaces inherit the property at their owning
//! layers, wiring this same helper.
//!
//! Degenerate edge, accepted (vsdd-cli #801): a schema-legal display
//! string made ENTIRELY of stripped code points cleans to empty here,
//! after the schema's minLength-1 check has passed. This is safe — the
//! render layer words an empty display value as its registered absence —
//! so the post-clean empty is an accepted degenerate rendering, not a
//! schema breach to re-diagnose.

use icu_properties::props::DefaultIgnorableCodePoint;
use icu_properties::CodePointSetData;
use unicode_general_category::{get_general_category, GeneralCategory};

/// True for a code point that must never reach a terminal or
/// agent-consumed surface (contract: Terminal output safety, vsdd-cli
/// #807): the display-unsafe class is the union of the control category
/// (`Cc`), the format category (`Cf`), the line and paragraph separators
/// (`Zl`/`Zp`), and every code point carrying the Unicode Character
/// Database property `Default_Ignorable_Code_Point` — its reserved
/// unassigned portion (invisible on conformant terminals) and its
/// assigned members alike. The property subsumes what the hand-rolled
/// classifier had to enumerate by range and by the Hangul-filler list,
/// and it wins over combining-mark preservation: the variation selectors
/// and the combining grapheme joiner carry the property and are stripped
/// (the invisible-payload channel outweighs rendering fidelity on these
/// surfaces; the ratified ruling reversing vsdd-cli #798). Combining
/// marks that do NOT carry the property, and normal spaces (`Zs`), are
/// preserved as legitimate text.
pub fn is_terminal_unsafe(c: char) -> bool {
    matches!(
        get_general_category(c),
        GeneralCategory::Control
            | GeneralCategory::Format
            | GeneralCategory::LineSeparator
            | GeneralCategory::ParagraphSeparator
    ) || CodePointSetData::new::<DefaultIgnorableCodePoint>().contains(c)
}

/// Recursively clean every string VALUE and every object KEY in a JSON
/// tree — the machine-form output's whole-of-output pass (contract:
/// Terminal output safety; vsdd-cli #803, key coverage ratified in
/// #807): rather than cleaning hand-picked struct fields (which left one
/// sibling uncovered each round), the whole serialized form passes
/// through here, so no field is missed whatever the struct's shape. Keys
/// are cleaned too — a map keyed by an adopter repo or milestone name
/// carries the class in its keys exactly as a value does; a tool-authored
/// constant key cleans to itself, so the common case rebuilds nothing.
/// Collision disposition: two keys that clean to the same string collapse
/// to one entry, the last iterated winning (never a panic). This is
/// reachable only for hostile keys carrying the class — where collapsing
/// the colliding pair is the safe disposition, not a data loss to
/// preserve — since tool-authored constant keys never collide.
pub fn clean_json_strings(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::String(s) => {
            let cleaned = clean_for_terminal(s);
            if &cleaned != s {
                *s = cleaned;
            }
        }
        serde_json::Value::Array(items) => items.iter_mut().for_each(clean_json_strings),
        serde_json::Value::Object(map) => {
            if map.keys().any(|k| clean_for_terminal(k) != *k) {
                let old = std::mem::take(map);
                for (k, mut v) in old {
                    clean_json_strings(&mut v);
                    map.insert(clean_for_terminal(&k), v);
                }
            } else {
                map.values_mut().for_each(clean_json_strings);
            }
        }
        _ => {}
    }
}

/// Drop every terminal-unsafe code point; the shared cleaner for all
/// terminal-destined text (vsdd-cli #777, #784, #788).
pub fn clean_for_terminal(s: &str) -> String {
    s.chars().filter(|c| !is_terminal_unsafe(*c)).collect()
}

#[cfg(test)]
mod tests {
    use super::{clean_for_terminal, clean_json_strings};

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
    fn variation_selectors_are_stripped() {
        // The ratified reversal of #798 (contract: Terminal output
        // safety): variation selectors carry Default_Ignorable_Code_Point
        // and are a selector-run smuggling channel, so the property wins
        // over their Mn category and they strip — the base character
        // survives. This is the rebuild's red-gate seed: it asserted the
        // opposite before the property swap.
        assert_eq!(clean_for_terminal("text\u{E0100}"), "text");
        assert_eq!(clean_for_terminal("emoji\u{2764}\u{FE0F}"), "emoji\u{2764}");
        // The combining grapheme joiner is default-ignorable Mn too.
        assert_eq!(clean_for_terminal("a\u{034F}b"), "ab");
    }

    #[test]
    fn assigned_default_ignorables_are_stripped_by_property() {
        // The Hangul fillers (category Lo) render as blank cells and are
        // default-ignorable; the property catches them where the
        // category proxy could not (vsdd-cli #804, subsumed by the
        // ratified property predicate — the round-6 hand-list retired).
        assert_eq!(clean_for_terminal("ba\u{115F}d"), "bad");
        assert_eq!(clean_for_terminal("ba\u{1160}d"), "bad");
        assert_eq!(clean_for_terminal("ba\u{3164}d"), "bad");
        assert_eq!(clean_for_terminal("ba\u{FFA0}d"), "bad");
    }

    #[test]
    fn json_sanitizer_cleans_values_and_keys_recursively() {
        // The whole-of-output machine-form pass (contract: Terminal
        // output safety): every string value AND every object key,
        // recursively, so no field is missed whatever the struct's shape
        // — the systematic fix for the field-by-field misses (#803/#805).
        let mut v = serde_json::json!({
            "safe\u{202E}key": ["a\u{200B}b", {"nested\u{2060}": "va\u{E0100}l"}],
            "plain": "ok"
        });
        clean_json_strings(&mut v);
        let obj = v.as_object().unwrap();
        assert!(obj.contains_key("safekey"), "the object key is cleaned");
        assert!(!obj.contains_key("safe\u{202E}key"));
        let arr = obj["safekey"].as_array().unwrap();
        assert_eq!(arr[0], serde_json::Value::String("ab".into()));
        let nested = arr[1].as_object().unwrap();
        assert!(nested.contains_key("nested"), "a nested key is cleaned");
        assert_eq!(nested["nested"], serde_json::Value::String("val".into()));
        assert_eq!(obj["plain"], serde_json::Value::String("ok".into()));
    }

    #[test]
    fn json_sanitizer_collapses_colliding_cleaned_keys_without_panic() {
        // Hostile edge (verify-round finding, both lenses): two keys that
        // clean to the same string collapse to one entry — last iterated
        // wins, never a panic. The documented residual, pinned.
        let mut v = serde_json::json!({"k": 1, "k\u{200B}": 2});
        clean_json_strings(&mut v);
        let obj = v.as_object().unwrap();
        assert_eq!(obj.len(), 1, "colliding cleaned keys collapse to one");
        assert!(obj.contains_key("k"));
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
