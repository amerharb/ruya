//! The styles a scene can be shown in: the same kitchen as a flat drawing or
//! as a realistic picture. The things and their names are the same in every
//! style; each style has its own picture and its own regions over it.
//!
//! Adding one: a variant here, a field in `scene::Styles`, and an arm in each
//! match. A scene need not have every style; the switch shows those it has.

use crate::lang::Lang;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Style {
    Basic,
    Realistic,
}

impl Style {
    /// In the order the switch shows them.
    pub const ALL: [Style; 2] = [Style::Basic, Style::Realistic];

    /// What a scene opens in when nothing was chosen.
    pub const DEFAULT: Style = Style::Realistic;

    /// The `?s=` value.
    pub fn code(self) -> &'static str {
        match self {
            Style::Basic => "basic",
            Style::Realistic => "realistic",
        }
    }

    pub fn from_code(code: &str) -> Option<Style> {
        Style::ALL.into_iter().find(|s| s.code() == code.trim())
    }

    /// The word for "style", the switch's accessible name.
    pub fn label(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Style",
            Lang::Ar => "النمط",
            Lang::De => "Stil",
            Lang::Sv => "Stil",
            Lang::Es => "Estilo",
            Lang::Uk => "Стиль",
            Lang::He => "סגנון",
        }
    }

    /// The switch shows a little picture of each style, which needs no words;
    /// this is what a screen reader says for it instead, and a mouse's tooltip.
    pub fn name(self, lang: Lang) -> &'static str {
        match (self, lang) {
            (Style::Basic, Lang::En) => "drawing",
            (Style::Basic, Lang::Ar) => "رسم",
            (Style::Basic, Lang::De) => "Zeichnung",
            (Style::Basic, Lang::Sv) => "teckning",
            (Style::Basic, Lang::Es) => "dibujo",
            (Style::Basic, Lang::Uk) => "малюнок",
            (Style::Basic, Lang::He) => "ציור",
            (Style::Realistic, Lang::En) => "realistic",
            (Style::Realistic, Lang::Ar) => "واقعي",
            (Style::Realistic, Lang::De) => "realistisch",
            (Style::Realistic, Lang::Sv) => "realistisk",
            (Style::Realistic, Lang::Es) => "realista",
            (Style::Realistic, Lang::Uk) => "реалістичний",
            (Style::Realistic, Lang::He) => "מציאותי",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_round_trip() {
        for s in Style::ALL {
            assert_eq!(Style::from_code(s.code()), Some(s));
        }
        assert_eq!(Style::from_code("photo"), None);
        assert!(Style::ALL.contains(&Style::DEFAULT));
    }

    #[test]
    fn every_style_has_a_name_in_every_language() {
        for s in Style::ALL {
            for l in Lang::ALL {
                assert!(!s.name(l).trim().is_empty());
            }
        }
    }
}
