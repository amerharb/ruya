//! The languages Ruya names things in.
//!
//! Adding one: a variant here, a field in [`Names`] — and in [`Plurals`] or
//! [`Duals`] if it has those forms — and an arm in each match. Every scene
//! file then fails to parse until it carries the new language, and
//! `cargo test` names each file and field still missing.

use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Lang {
    En,
    Ar,
    De,
    Sv,
    Es,
    Uk,
    He,
    Ja,
}

impl Lang {
    /// In the order the picker lists them.
    pub const ALL: [Lang; 8] = [
        Lang::En,
        Lang::Ar,
        Lang::De,
        Lang::Sv,
        Lang::Es,
        Lang::Uk,
        Lang::He,
        Lang::Ja,
    ];

    /// The ISO 639-1 code: the `?l=` value and `<html lang>`.
    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Ar => "ar",
            Lang::De => "de",
            Lang::Sv => "sv",
            Lang::Es => "es",
            Lang::Uk => "uk",
            Lang::He => "he",
            Lang::Ja => "ja",
        }
    }

    /// The language's name for itself, which is how the picker shows it: a
    /// reader finds their own language without reading anyone else's.
    pub fn endonym(self) -> &'static str {
        match self {
            Lang::En => "English",
            Lang::Ar => "العربية",
            Lang::De => "Deutsch",
            Lang::Sv => "Svenska",
            Lang::Es => "Español",
            Lang::Uk => "Українська",
            Lang::He => "עברית",
            Lang::Ja => "日本語",
        }
    }

    /// The word for "language", the picker's accessible name — the one piece
    /// of interface text Ruya has.
    pub fn language_word(self) -> &'static str {
        match self {
            Lang::En => "Language",
            Lang::Ar => "اللغة",
            Lang::De => "Sprache",
            Lang::Sv => "Språk",
            Lang::Es => "Idioma",
            Lang::Uk => "Мова",
            Lang::He => "שפה",
            Lang::Ja => "言語",
        }
    }

    pub fn rtl(self) -> bool {
        matches!(self, Lang::Ar | Lang::He)
    }

    pub fn dir(self) -> &'static str {
        if self.rtl() { "rtl" } else { "ltr" }
    }

    /// The numbers a noun has a form for: one, then two, then many. Hebrew
    /// keeps its dual for a few words, a pair of eyes or of days, and counts
    /// two of anything else with the plural, so here it has none. Japanese
    /// has no plural: 猫 is one cat or many.
    pub fn numbers(self) -> &'static [Number] {
        match self {
            Lang::Ar => &[Number::Singular, Number::Dual, Number::Plural],
            Lang::En | Lang::De | Lang::Sv | Lang::Es | Lang::Uk | Lang::He => {
                &[Number::Singular, Number::Plural]
            }
            Lang::Ja => &[Number::Singular],
        }
    }

    /// What goes between a name's forms when a screen reader says them.
    pub fn comma(self) -> &'static str {
        match self {
            Lang::Ar => "، ",
            Lang::Ja => "、",
            Lang::En | Lang::De | Lang::Sv | Lang::Es | Lang::Uk | Lang::He => ", ",
        }
    }

    /// What the language calls the readings over its names, in one that has
    /// them: Japanese's furigana, over its kanji. None where the letters
    /// already say the sound.
    pub fn readings_name(self) -> Option<&'static str> {
        match self {
            Lang::Ja => Some("ふりがな"),
            Lang::En | Lang::Ar | Lang::De | Lang::Sv | Lang::Es | Lang::Uk | Lang::He => None,
        }
    }

    /// The speaker button's name, in a language Ruya has a voice for: one
    /// that says a thing's name when it is tapped, from recordings made by
    /// `tools/voice.py`. None where it has none yet; English is the first.
    pub fn voice_name(self) -> Option<&'static str> {
        match self {
            Lang::En => Some("Say the names"),
            Lang::Ar | Lang::De | Lang::Sv | Lang::Es | Lang::Uk | Lang::He | Lang::Ja => None,
        }
    }

    /// A language from a BCP 47 tag, ignoring case and region (`sv-SE` is
    /// Swedish), or None when Ruya does not have it. `iw` is the code Hebrew
    /// had before 1989, which some systems still report.
    pub fn from_tag(tag: &str) -> Option<Lang> {
        let primary = tag
            .trim()
            .split(['-', '_'])
            .next()
            .unwrap_or("")
            .to_ascii_lowercase();
        match primary.as_str() {
            "en" => Some(Lang::En),
            "ar" => Some(Lang::Ar),
            "de" => Some(Lang::De),
            "sv" => Some(Lang::Sv),
            "es" => Some(Lang::Es),
            "uk" => Some(Lang::Uk),
            "he" | "iw" => Some(Lang::He),
            "ja" => Some(Lang::Ja),
            _ => None,
        }
    }
}

/// One name per language, as a struct rather than a map: a scene missing a
/// language fails to parse, naming the field, instead of showing a blank.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Names {
    pub en: String,
    pub ar: String,
    pub de: String,
    pub sv: String,
    pub es: String,
    pub uk: String,
    pub he: String,
    pub ja: String,
}

impl Names {
    pub fn get(&self, lang: Lang) -> &str {
        match lang {
            Lang::En => &self.en,
            Lang::Ar => &self.ar,
            Lang::De => &self.de,
            Lang::Sv => &self.sv,
            Lang::Es => &self.es,
            Lang::Uk => &self.uk,
            Lang::He => &self.he,
            Lang::Ja => &self.ja,
        }
    }
}

/// How many a name is for — what grammar calls number. Every language has a
/// singular and most a plural; Arabic also has a dual, for two. Japanese has
/// no plural, so the singular alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Number {
    Singular,
    Dual,
    Plural,
}

impl Number {
    /// One, two, many: the order of the buttons, and of a name's lines.
    pub const ALL: [Number; 3] = [Number::Singular, Number::Dual, Number::Plural];

    /// Its part of the `?f=` value.
    pub fn code(self) -> &'static str {
        match self {
            Number::Singular => "one",
            Number::Dual => "two",
            Number::Plural => "many",
        }
    }

    /// Its button shows dots, which need no words; this is what a screen
    /// reader says for it instead, and a mouse's tooltip.
    pub fn name(self, lang: Lang) -> &'static str {
        match (self, lang) {
            (Number::Singular, Lang::En) => "singular",
            (Number::Singular, Lang::Ar) => "مفرد",
            (Number::Singular, Lang::De) => "Einzahl",
            (Number::Singular, Lang::Sv) => "singular",
            (Number::Singular, Lang::Es) => "singular",
            (Number::Singular, Lang::Uk) => "однина",
            (Number::Singular, Lang::He) => "יחיד",
            (Number::Singular, Lang::Ja) => "単数",
            (Number::Dual, Lang::En) => "dual",
            (Number::Dual, Lang::Ar) => "مثنى",
            (Number::Dual, Lang::De) => "Zweizahl",
            (Number::Dual, Lang::Sv) => "dualis",
            (Number::Dual, Lang::Es) => "dual",
            (Number::Dual, Lang::Uk) => "двоїна",
            (Number::Dual, Lang::He) => "זוגי",
            (Number::Dual, Lang::Ja) => "双数",
            (Number::Plural, Lang::En) => "plural",
            (Number::Plural, Lang::Ar) => "جمع",
            (Number::Plural, Lang::De) => "Mehrzahl",
            (Number::Plural, Lang::Sv) => "plural",
            (Number::Plural, Lang::Es) => "plural",
            (Number::Plural, Lang::Uk) => "множина",
            (Number::Plural, Lang::He) => "רבים",
            (Number::Plural, Lang::Ja) => "複数",
        }
    }
}

/// Which forms of a name are shown: any of singular, dual and plural at
/// once, the singular alone to begin with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Forms([bool; 3]);

impl Forms {
    pub const SINGULAR: Forms = Forms([true, false, false]);

    fn has(self, n: Number) -> bool {
        self.0[n as usize]
    }

    fn with(mut self, n: Number, on: bool) -> Forms {
        self.0[n as usize] = on;
        self
    }

    /// Whether any form chosen is one `lang` has.
    fn any_in(self, lang: Lang) -> bool {
        lang.numbers().iter().any(|&n| self.has(n))
    }

    /// As it stands in `lang`: when none of the forms chosen is one `lang`
    /// has — the dual alone, in English — with its singular.
    fn in_lang(self, lang: Lang) -> Forms {
        if self.any_in(lang) {
            self
        } else {
            self.with(Number::Singular, true)
        }
    }

    /// The forms shown in `lang`, one before two before many.
    pub fn shown(self, lang: Lang) -> impl Iterator<Item = Number> {
        let forms = self.in_lang(lang);
        lang.numbers()
            .iter()
            .copied()
            .filter(move |&n| forms.has(n))
    }

    pub fn shows(self, n: Number, lang: Lang) -> bool {
        self.shown(lang).any(|m| m == n)
    }

    /// `n`'s button pressed in `lang`: its form shown, or hidden unless it is
    /// the last one showing, when nothing changes. A form `lang` does not
    /// have keeps its choice for a language that has it — the dual, through
    /// a visit to English.
    pub fn toggled(self, n: Number, lang: Lang) -> Forms {
        let forms = self.in_lang(lang);
        let next = forms.with(n, !forms.has(n));
        if next.any_in(lang) { next } else { self }
    }

    /// The `?f=` value: the forms chosen, joined by `-`, as `one-many`.
    pub fn code(self) -> String {
        Number::ALL
            .into_iter()
            .filter(|&n| self.has(n))
            .map(Number::code)
            .collect::<Vec<_>>()
            .join("-")
    }

    /// From a `?f=` value, a part it does not know ignored; None when no part
    /// is a form.
    pub fn from_code(code: &str) -> Option<Forms> {
        let none = Forms([false; 3]);
        let forms = code
            .split('-')
            .filter_map(|part| Number::ALL.into_iter().find(|n| n.code() == part.trim()))
            .fold(none, |forms, n| forms.with(n, true));
        (forms != none).then_some(forms)
    }

    /// The buttons' accessible name, after the forms the language has. A
    /// language with one form, as Japanese, shows no buttons.
    pub fn label(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Singular and plural",
            Lang::Ar => "المفرد والمثنى والجمع",
            Lang::De => "Einzahl und Mehrzahl",
            Lang::Sv => "Singular och plural",
            Lang::Es => "Singular y plural",
            Lang::Uk => "Однина і множина",
            Lang::He => "יחיד ורבים",
            Lang::Ja => "単数",
        }
    }
}

/// A name for many, one per language that has a plural: all of Ruya's but
/// Japanese. A field missing fails to parse, as in [`Names`].
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plurals {
    pub en: String,
    pub ar: String,
    pub de: String,
    pub sv: String,
    pub es: String,
    pub uk: String,
    pub he: String,
}

impl Plurals {
    pub fn get(&self, lang: Lang) -> Option<&str> {
        match lang {
            Lang::En => Some(&self.en),
            Lang::Ar => Some(&self.ar),
            Lang::De => Some(&self.de),
            Lang::Sv => Some(&self.sv),
            Lang::Es => Some(&self.es),
            Lang::Uk => Some(&self.uk),
            Lang::He => Some(&self.he),
            Lang::Ja => None,
        }
    }
}

/// A name for two, one per language that has a dual: Arabic. A field for any
/// other fails to parse.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Duals {
    pub ar: String,
}

impl Duals {
    pub fn get(&self, lang: Lang) -> Option<&str> {
        match lang {
            Lang::Ar => Some(&self.ar),
            Lang::En | Lang::De | Lang::Sv | Lang::Es | Lang::Uk | Lang::He | Lang::Ja => None,
        }
    }
}

/// A name with its readings — see [`crate::reading`] — in a language that
/// has them: Japanese, wherever its name has kanji. A field for any other
/// language fails to parse.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Readings {
    #[serde(default)]
    pub ja: Option<String>,
}

impl Readings {
    pub fn get(&self, lang: Lang) -> Option<&str> {
        match lang {
            Lang::Ja => self.ja.as_deref(),
            Lang::En | Lang::Ar | Lang::De | Lang::Sv | Lang::Es | Lang::Uk | Lang::He => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn tags_ignore_case_and_region() {
        assert_eq!(Lang::from_tag("sv-SE"), Some(Lang::Sv));
        assert_eq!(Lang::from_tag("AR"), Some(Lang::Ar));
        assert_eq!(Lang::from_tag("es_ES"), Some(Lang::Es));
        assert_eq!(Lang::from_tag(" de "), Some(Lang::De));
        assert_eq!(Lang::from_tag("iw"), Some(Lang::He));
        assert_eq!(Lang::from_tag("ja-JP"), Some(Lang::Ja));
        assert_eq!(Lang::from_tag("zz"), None);
        assert_eq!(Lang::from_tag(""), None);
    }

    #[test]
    fn every_code_round_trips_and_is_unique() {
        let codes: HashSet<_> = Lang::ALL.iter().map(|l| l.code()).collect();
        assert_eq!(codes.len(), Lang::ALL.len());
        for l in Lang::ALL {
            assert_eq!(Lang::from_tag(l.code()), Some(l));
            assert!(!l.endonym().trim().is_empty());
            assert!(!l.language_word().trim().is_empty());
        }
    }

    #[test]
    fn only_arabic_and_hebrew_read_right_to_left() {
        let rtl: Vec<_> = Lang::ALL.into_iter().filter(|l| l.rtl()).collect();
        assert_eq!(rtl, [Lang::Ar, Lang::He]);
    }

    #[test]
    fn every_language_has_one_first_and_only_arabic_has_two() {
        for l in Lang::ALL {
            assert_eq!(l.numbers().first(), Some(&Number::Singular));
        }
        let dual: Vec<_> = Lang::ALL
            .into_iter()
            .filter(|l| l.numbers().contains(&Number::Dual))
            .collect();
        assert_eq!(dual, [Lang::Ar]);
    }

    #[test]
    fn every_number_has_a_name_in_every_language() {
        for n in Number::ALL {
            for l in Lang::ALL {
                assert!(!n.name(l).trim().is_empty());
            }
        }
    }

    fn shown(forms: Forms, lang: Lang) -> Vec<Number> {
        forms.shown(lang).collect()
    }

    #[test]
    fn forms_start_singular_and_stack_one_two_many() {
        use Number::{Dual, Plural, Singular};
        assert_eq!(shown(Forms::SINGULAR, Lang::Ar), [Singular]);
        let plural_too = Forms::SINGULAR.toggled(Plural, Lang::En);
        assert_eq!(shown(plural_too, Lang::En), [Singular, Plural]);
        // pressed last, the dual still goes between
        let all = plural_too.toggled(Dual, Lang::Ar);
        assert_eq!(shown(all, Lang::Ar), [Singular, Dual, Plural]);
        assert_eq!(shown(all, Lang::Sv), [Singular, Plural]);
    }

    #[test]
    fn japanese_shows_its_one_form_whatever_is_chosen() {
        use Number::{Dual, Plural, Singular};
        let plural = Forms::SINGULAR
            .toggled(Plural, Lang::En)
            .toggled(Singular, Lang::En);
        let all = Forms::SINGULAR
            .toggled(Dual, Lang::Ar)
            .toggled(Plural, Lang::Ar);
        for forms in [Forms::SINGULAR, plural, all] {
            assert_eq!(shown(forms, Lang::Ja), [Singular]);
        }
    }

    #[test]
    fn the_last_form_showing_stays() {
        use Number::{Plural, Singular};
        assert_eq!(Forms::SINGULAR.toggled(Singular, Lang::En), Forms::SINGULAR);
        let plural = Forms::SINGULAR
            .toggled(Plural, Lang::En)
            .toggled(Singular, Lang::En);
        assert_eq!(shown(plural, Lang::En), [Plural]);
        assert_eq!(plural.toggled(Plural, Lang::En), plural);
    }

    #[test]
    fn a_dual_waits_out_a_language_without_one() {
        use Number::{Dual, Plural, Singular};
        let dual = Forms::SINGULAR
            .toggled(Dual, Lang::Ar)
            .toggled(Singular, Lang::Ar);
        assert_eq!(shown(dual, Lang::Ar), [Dual]);
        // English has no dual: its singular shows instead, and stays, as the
        // last one showing, without its press changing the choice
        assert_eq!(shown(dual, Lang::En), [Singular]);
        assert!(dual.shows(Singular, Lang::En));
        assert_eq!(dual.toggled(Singular, Lang::En), dual);
        // and a choice made there keeps the dual for Arabic
        let back = dual.toggled(Plural, Lang::En);
        assert_eq!(shown(back, Lang::En), [Singular, Plural]);
        assert_eq!(shown(back, Lang::Ar), [Singular, Dual, Plural]);
    }

    #[test]
    fn forms_codes_round_trip() {
        let all = Forms([true; 3]);
        assert_eq!(all.code(), "one-two-many");
        assert_eq!(Forms::SINGULAR.code(), "one");
        for forms in [all, Forms::SINGULAR, Forms([false, true, true])] {
            assert_eq!(Forms::from_code(&forms.code()), Some(forms));
        }
        assert_eq!(
            Forms::from_code("many-lots").map(Forms::code).as_deref(),
            Some("many")
        );
        assert_eq!(Forms::from_code("lots"), None);
        assert_eq!(Forms::from_code(""), None);
    }
}
