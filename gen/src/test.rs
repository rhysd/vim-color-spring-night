extern crate regex;

use self::regex::Regex;
use super::*;
use std::collections::{HashMap, HashSet};
use std::str;

const DUMMY_TERM_COLORS: [&'static str; 16] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
];

#[test]
fn test_color_code() {
    assert_eq!(*ColorCode::Normal(10).normal(), 10);
    assert_eq!(*ColorCode::Contrast(10, 20).normal(), 10);
}

#[test]
fn test_write_header() {
    let out = &mut Vec::new();
    {
        let mut w = Writer {
            table: HashMap::new(),
            highlights: &[],
            term_colors: DUMMY_TERM_COLORS,
            out,
        };
        w.write_header("spring-night").unwrap();
    }
    let rendered = str::from_utf8(out).unwrap();
    assert!(rendered.starts_with(r#"" spring-night: Calm-colored dark color scheme"#));
    assert!(rendered.contains("let g:colors_name = 'spring-night'"));
}

#[test]
fn test_write_contrast_color_variables() {
    let out = &mut Vec::new();
    {
        Writer {
            table: HashMap::new(),
            highlights: &[],
            term_colors: DUMMY_TERM_COLORS,
            out,
        }.write_contrast_color_variables()
            .unwrap();
    }
    assert_eq!(str::from_utf8(out).unwrap(), "\n");

    let out = &mut Vec::new();
    {
        let mut m = HashMap::new();
        m.insert(
            "hi",
            Color {
                gui: ColorCode::Normal("#123456"),
                cterm: ColorCode::Contrast(12, 34),
            },
        );
        m.insert(
            "hello",
            Color {
                gui: ColorCode::Contrast("#123456", "#7890ab"),
                cterm: ColorCode::Contrast(123, 234),
            },
        );
        m.insert(
            "hey",
            Color {
                gui: ColorCode::Normal("#123456"),
                cterm: ColorCode::Normal(123),
            },
        );
        m.insert(
            "goodbye",
            Color {
                gui: ColorCode::Contrast("#000000", "#ffffff"),
                cterm: ColorCode::Normal(123),
            },
        );
        Writer {
            table: m,
            highlights: &[],
            term_colors: DUMMY_TERM_COLORS,
            out,
        }.write_contrast_color_variables()
            .unwrap();
    }
    let mut lines = str::from_utf8(out).unwrap().lines().collect::<Vec<_>>();
    lines.sort();
    assert_eq!(
        lines,
        vec![
            "",
            "let s:goodbye_gui = g:spring_night_high_contrast ? '#000000' : '#ffffff'",
            "let s:hello_cterm = g:spring_night_high_contrast ? 123 : 234",
            "let s:hello_gui = g:spring_night_high_contrast ? '#123456' : '#7890ab'",
            "let s:hi_cterm = g:spring_night_high_contrast ? 12 : 34",
        ]
    );
}

#[test]
fn test_write_highlight() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let testcases = vec![
        ((None, None, None, HighlightAttr::Nothing),                           0, "hi HL term=NONE"),
        ((Some("normal"), None, None, HighlightAttr::Nothing),                 0, "hi HL term=NONE guifg=#123456 ctermfg=123"),
        ((None, Some("normal"), None, HighlightAttr::Nothing),                 0, "hi HL term=NONE guibg=#123456 ctermbg=123"),
        ((Some("normal"), Some("normal"), None, HighlightAttr::Nothing),       0, "hi HL term=NONE guifg=#123456 ctermfg=123 guibg=#123456 ctermbg=123"),
        ((None, None, None, HighlightAttr::Bold),                              0, "exe 'hi' 'HL' 'term=NONE' !g:spring_night_kill_bold ? 'gui=bold cterm=bold' : ''"),
        ((None, None, None, HighlightAttr::Italic),                            0, "exe 'hi' 'HL' 'term=NONE' !g:spring_night_kill_italic ? 'gui=italic' : ''"),
        ((None, None, None, HighlightAttr::Underline),                         0, "hi HL term=NONE gui=underline cterm=underline"),
        ((None, None, None, HighlightAttr::CommentItalic),                     0, "exe 'hi' 'HL' 'term=NONE' g:spring_night_italic_comments && !g:spring_night_kill_italic ? 'gui=italic' : ''"),
        ((None, None, None, HighlightAttr::Undercurl),                         0, "exe 'hi' 'HL' 'term=NONE' 'gui='.s:undercurl 'cterm='.s:undercurl"),
        ((Some("contrast"), None, None, HighlightAttr::Nothing),               0, "exe 'hi' 'HL' 'term=NONE' 'guifg='.s:contrast_gui 'ctermfg='.s:contrast_cterm"),
        ((None, Some("contrast"), None, HighlightAttr::Nothing),               0, "exe 'hi' 'HL' 'term=NONE' 'guibg='.s:contrast_gui 'ctermbg='.s:contrast_cterm"),
        ((Some("contrast"), Some("contrast"), None, HighlightAttr::Underline), 0, "exe 'hi' 'HL' 'term=NONE' 'guifg='.s:contrast_gui 'ctermfg='.s:contrast_cterm 'guibg='.s:contrast_gui 'ctermbg='.s:contrast_cterm 'gui=underline cterm=underline'"),
        ((None, None, None, HighlightAttr::Nothing),                           1, "    hi HL term=NONE"),
        ((None, None, None, HighlightAttr::Undercurl),                         1, "    exe 'hi' 'HL' 'term=NONE' 'gui='.s:undercurl 'cterm='.s:undercurl"),
    ];

    for ((fg, bg, sp, attr), indent, expected) in testcases {
        let out = &mut Vec::new();
        let hl = Highlight {
            name: "HL",
            fg,
            bg,
            sp,
            attr,
        };
        let mut m = HashMap::new();
        m.insert(
            "normal",
            Color {
                gui: ColorCode::Normal("#123456"),
                cterm: ColorCode::Normal(123),
            },
        );
        m.insert(
            "contrast",
            Color {
                gui: ColorCode::Contrast("#123456", "#7890ab"),
                cterm: ColorCode::Contrast(123, 234),
            },
        );
        Writer {
            table: m,
            highlights: &[],
            term_colors: DUMMY_TERM_COLORS,
            out,
        }.write_highlight(&hl, indent as u32)
            .unwrap();
        assert_eq!(str::from_utf8(out).unwrap(), format!("{}\n", expected));
    }

    // Edge case
    let out = &mut Vec::new();
    {
        Writer {
            table: HashMap::new(),
            highlights: &[],
            term_colors: DUMMY_TERM_COLORS,
            out,
        }.write_highlights()
            .unwrap();
    }
    assert_eq!(str::from_utf8(out).unwrap(), "\n");
}

#[test]
fn test_write_highlights() {
    let out = &mut Vec::new();
    fn hl() -> Highlight {
        Highlight {
            name: "HL",
            fg: None,
            bg: None,
            sp: None,
            attr: HighlightAttr::Nothing,
        }
    }
    Writer {
        table: HashMap::new(),
        highlights: &[Always(hl())],
        term_colors: DUMMY_TERM_COLORS,
        out,
    }.write_highlights()
        .unwrap();
    assert_eq!(str::from_utf8(out).unwrap(), "hi HL term=NONE\n\n");

    let out = &mut Vec::new();
    Writer {
        table: HashMap::new(),
        highlights: &[Switch(hl(), hl())],
        term_colors: DUMMY_TERM_COLORS,
        out,
    }.write_highlights()
        .unwrap();
    assert_eq!(
        str::from_utf8(out).unwrap().lines().collect::<Vec<_>>(),
        vec![
            "if s:gui_running",
            "    hi HL term=NONE",
            "else",
            "    hi HL term=NONE",
            "endif",
            "",
        ],
    );
}

#[test]
fn test_write_term_colors() {
    let out = &mut Vec::new();
    let mut m = HashMap::new();
    m.insert(
        "normal",
        Color {
            gui: ColorCode::Normal("#123456"),
            cterm: ColorCode::Normal(123),
        },
    );
    m.insert(
        "contrast",
        Color {
            gui: ColorCode::Contrast("#000000", "#ffffff"),
            cterm: ColorCode::Contrast(1, 2),
        },
    );
    Writer {
        table: m,
        highlights: &[],
        term_colors: [
            "normal", "contrast", "normal", "contrast", "normal", "contrast", "normal", "contrast",
            "normal", "contrast", "normal", "contrast", "normal", "contrast", "normal", "contrast",
        ],
        out,
    }.write_term_colors()
        .unwrap();
    let rendered = str::from_utf8(out).unwrap();
    assert!(rendered.contains("let g:terminal_color_0 = '#123456'"));
    assert!(rendered.contains("let g:terminal_color_1 = '#000000'"));
    assert!(rendered.contains("let g:terminal_color_0 = 123"));
    assert!(rendered.contains("let g:terminal_color_1 = 1"));
    assert!(rendered.contains("let g:terminal_ansi_colors = ["));
    assert!(rendered.contains("\\       '#123456'"));
    assert!(rendered.contains("\\       '#000000'"));
}

#[test]
fn test_spring_night_writer() {
    // Check duplicate highlights
    let out = &mut Vec::new();
    let w = spring_night_writer(out);
    let mut unique_check = HashSet::new();
    for hl in w.highlights {
        let name = match hl {
            Always(h) => h.name,
            Switch(g, t) => {
                assert_eq!(g.name, t.name);
                g.name
            }
        };
        assert!(unique_check.insert(name), "Duplicate highlight '{}'", name);
    }

    // Check terminal colors are correct
    for tc in &w.term_colors {
        assert!(
            w.table.contains_key(tc),
            "Terminal color '{}' is not present in color names",
            tc
        );
    }

    // Check color code is correct
    let re = Regex::new(r"^#[[:xdigit:]]{6}$").unwrap();
    for (name, c) in w.table {
        match c.gui {
            ColorCode::Normal(c) => assert!(
                re.is_match(c),
                "'{}' is invalid color code at '{}'",
                c,
                name
            ),
            ColorCode::Contrast(c1, c2) => {
                assert!(
                    re.is_match(c1),
                    "'{}' is invalid color code at '{}'",
                    c1,
                    name
                );
                assert!(
                    re.is_match(c2),
                    "'{}' is invalid color code at '{}'",
                    c2,
                    name
                );
            }
        }
    }
}
