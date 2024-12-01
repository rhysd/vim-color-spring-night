use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct Color<'a> {
    pub gui: ColorCode<&'a str>,
    pub cterm: ColorCode<u8>,
}

#[derive(Debug, PartialEq)]
pub enum ColorCode<T: Display> {
    Normal(T),
    Contrast(T, T),
}

impl<T: Display> ColorCode<T> {
    pub fn normal(&self) -> &T {
        match self {
            ColorCode::Normal(c) => c,
            ColorCode::Contrast(h, _) => h,
        }
    }
}

type Colors<'a> = HashMap<&'a str, Color<'a>>;

#[derive(Debug)]
pub struct Palette<'a>(Colors<'a>);

impl<'a> Deref for Palette<'a> {
    type Target = Colors<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<Colors<'a>> for Palette<'a> {
    fn from(m: Colors<'a>) -> Self {
        Self(m)
    }
}

impl Default for Palette<'_> {
    #[rustfmt::skip]
    fn default() -> Self {
        use ColorCode::{Normal, Contrast};

        let mut table = HashMap::new();
        let mut color = |name, gui, cterm| {
            assert_eq!(table.insert(name, Color { gui, cterm }), None);
        };

        color("bg",         Contrast("#132132", "#334152"), Normal(233));
        color("bgweaker",   Contrast("#213243", "#3a4b5c"), Normal(235));
        color("bgemphasis", Normal("#3a4b5c"),              Normal(235));
        color("bglight",    Normal("#435060"),              Normal(236));
        color("bgstrong",   Normal("#536273"),              Normal(238));
        color("light",      Normal("#646f7c"),              Normal(60));
        color("fg",         Normal("#fffeeb"),              Contrast(231, 230));
        color("hiddenfg",   Normal("#607080"),              Normal(60));
        color("weakfg",     Normal("#8d9eb2"),              Normal(103));
        color("weakerfg",   Normal("#788898"),              Normal(102));
        color("black",      Normal("#111e25"),              Normal(233));
        color("gray",       Normal("#545f6e"),              Normal(59));
        color("white",      Normal("#ffffff"),              Normal(231));
        color("nasu",       Normal("#605779"),              Normal(61));
        color("fuchsia",    Normal("#b9a5cf"),              Normal(183));
        color("purple",     Normal("#e7d5ff"),              Normal(189));
        color("yaezakura",  Normal("#70495d"),              Normal(95));
        color("sakura",     Normal("#a9667a"),              Normal(132));
        color("kakezakura", Normal("#e996aa"),              Normal(175));
        color("palepink",   Normal("#e7c6b7"),              Normal(181));
        color("mikan",      Normal("#fb8965"),              Normal(209));
        color("orange",     Normal("#f0aa8a"),              Normal(216));
        color("darkgreen",  Normal("#5f8770"),              Normal(65));
        color("green",      Normal("#a9dd9d"),              Normal(150));
        color("lime",       Normal("#c9fd88"),              Normal(149));
        color("blue",       Normal("#7098e6"),              Normal(69));
        color("paleblue",   Normal("#98b8e6"),              Normal(111));
        color("cloudy",     Normal("#90aecb"),              Normal(75));
        color("skyblue",    Normal("#a8d2eb"),              Normal(153));
        color("sunny",      Normal("#b8e2fb"),              Normal(195));
        color("yellow",     Normal("#f0eaaa"),              Normal(229));
        color("gold",       Normal("#fedf81"),              Normal(222));
        color("dullgold",   Normal("#b6955b"),              Normal(221));
        color("darkgold",   Contrast("#484000", "#685800"), Normal(58));
        color("mildred",    Normal("#ab6560"),              Normal(167));
        color("red",        Normal("#fd8489"),              Normal(210));
        color("crimson",    Normal("#ff6a6f"),              Normal(203));
        color("darkblue",   Normal("#00091e"),              Normal(235));
        color("whitepink",  Normal("#ebeadb"),              Normal(224));
        color("whitegreen", Normal("#eaf0aa"),              Normal(194));
        color("whiteblue",  Normal("#d8e2f0"),              Normal(195));
        color("whitered",   Normal("#ffbfaf"),              Normal(217));
        color("inu",        Normal("#ddbc96"),              Normal(180));

        Self(table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_color_code() {
        assert_eq!(*ColorCode::Normal(10).normal(), 10);
        assert_eq!(*ColorCode::Contrast(10, 20).normal(), 10);
    }

    #[test]
    fn test_hex_color_format() {
        let palette = Palette::default();
        let re = Regex::new(r"^#[[:xdigit:]]{6}$").unwrap();
        for (name, c) in palette.iter() {
            match c.gui {
                ColorCode::Normal(c) => {
                    assert!(re.is_match(c), "'{c}' is invalid color code at '{name}'");
                }
                ColorCode::Contrast(c1, c2) => {
                    assert!(re.is_match(c1), "'{c1}' is invalid color code at '{name}'");
                    assert!(re.is_match(c2), "'{c2}' is invalid color code at '{name}'");
                }
            }
        }
    }
}
