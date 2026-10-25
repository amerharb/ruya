//! The languages Ruya names things in.
//!
//! Adding one: a variant here, a field in [`Names`], and an arm in each match.
//! Every scene file then fails to parse until it carries the new language, and
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
}

impl Lang {
    /// In the order the picker lists them.
    pub const ALL: [Lang; 7] = [
        Lang::En,
        Lang::Ar,
        Lang::De,
        Lang::Sv,
        Lang::Es,
        Lang::Uk,
        Lang::He,
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
        }
    }

    pub fn rtl(self) -> bool {
        matches!(self, Lang::Ar | Lang::He)
    }

    pub fn dir(self) -> &'static str {
        if self.rtl() { "rtl" } else { "ltr" }
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
}
