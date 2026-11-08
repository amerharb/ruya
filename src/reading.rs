//! Readings over a name: furigana, the small kana that say how Japanese
//! kanji are read. A scene file writes a name's readings after each run of
//! kanji, in brackets — 冷蔵庫[れいぞうこ], 肘[ひじ]掛[か]け椅子[いす] — and the label
//! sets each over its kanji as HTML ruby.

/// Whether a character is a kanji, one that needs a reading over it. 々
/// repeats the kanji before it, so it is one too.
pub fn is_kanji(c: char) -> bool {
    matches!(c, '\u{4E00}'..='\u{9FFF}' | '々')
}

/// A name with readings, in pieces: each run of kanji with its reading, and
/// what lies between them alone. 肘[ひじ]掛[か]け椅子[いす] is 肘 under ひじ, 掛
/// under か, け alone and 椅子 under いす.
pub fn pieces(annotated: &str) -> Vec<(&str, Option<&str>)> {
    let mut out = Vec::new();
    let mut rest = annotated;
    while let Some(open) = rest.find('[') {
        let Some(close) = rest[open..].find(']').map(|c| open + c) else {
            break;
        };
        let before = &rest[..open];
        // the reading belongs to the run of kanji just before it
        let start = before
            .char_indices()
            .rev()
            .take_while(|&(_, c)| is_kanji(c))
            .last()
            .map_or(open, |(i, _)| i);
        if start > 0 {
            out.push((&before[..start], None));
        }
        out.push((&before[start..], Some(&rest[open + 1..close])));
        rest = &rest[close + 1..];
    }
    if !rest.is_empty() {
        out.push((rest, None));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_reading_goes_over_the_kanji_before_it() {
        assert_eq!(
            pieces("冷蔵庫[れいぞうこ]"),
            [("冷蔵庫", Some("れいぞうこ"))]
        );
        assert_eq!(
            pieces("肘[ひじ]掛[か]け椅子[いす]"),
            [
                ("肘", Some("ひじ")),
                ("掛", Some("か")),
                ("け", None),
                ("椅子", Some("いす"))
            ]
        );
    }

    #[test]
    fn katakana_beside_kanji_stays_bare() {
        assert_eq!(
            pieces("電気[でんき]ケトル"),
            [("電気", Some("でんき")), ("ケトル", None)]
        );
        assert_eq!(
            pieces("テレビ台[だい]"),
            [("テレビ", None), ("台", Some("だい"))]
        );
        assert_eq!(pieces("テレビ"), [("テレビ", None)]);
    }
}
