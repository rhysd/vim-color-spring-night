use super::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str;

const DUMMY_TERM_COLORS: [&'static str; 16] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
];

fn dummy_airline_theme() -> AirlineThemeColors<'static> {
    AirlineThemeColors {
        mode: Default::default(),
        paste: "",
        info_mod: "",
        error: ("", ""),
        warning: ("", ""),
    }
}

#[test]
fn test_color_code() {
    assert_eq!(*ColorCode::Normal(10).normal(), 10);
    assert_eq!(*ColorCode::Contrast(10, 20).normal(), 10);
}

#[test]
fn test_write_header() {
    let mut w = Writer {
        table: HashMap::new(),
        highlights: &[],
        term_colors: DUMMY_TERM_COLORS,
        airline_theme: dummy_airline_theme(),
        out: Vec::new(),
    };
    w.write_header().unwrap();
    let rendered = str::from_utf8(&w.out).unwrap();
    assert!(rendered.starts_with(r#"" spring-night: Calm-colored dark color scheme"#));
    assert!(rendered.contains("let g:colors_name = 'spring-night'"));
}

#[test]
fn test_write_contrast_color_variables() {
    let mut w = Writer {
        table: HashMap::new(),
        highlights: &[],
        term_colors: DUMMY_TERM_COLORS,
        airline_theme: dummy_airline_theme(),
        out: Vec::new(),
    };
    w.write_contrast_color_variables().unwrap();
    assert_eq!(str::from_utf8(&w.out).unwrap(), "\n");

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
    let mut w = Writer {
        table: m,
        highlights: &[],
        term_colors: DUMMY_TERM_COLORS,
        airline_theme: dummy_airline_theme(),
        out: Vec::new(),
    };
    w.write_contrast_color_variables().unwrap();
    for (actual, expected) in [
        "let s:goodbye_gui = g:spring_night_high_contrast ? '#000000' : '#ffffff'",
        "let s:hello_gui = g:spring_night_high_contrast ? '#123456' : '#7890ab'",
        "let s:hello_cterm = g:spring_night_high_contrast ? 123 : 234",
        "let s:hi_cterm = g:spring_night_high_contrast ? 12 : 34",
        "",
    ]
    .iter()
    .zip(str::from_utf8(&w.out).unwrap().lines())
    {
        assert_eq!(*actual, expected);
    }
}

#[test]
fn test_write_highlight() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let testcases = vec![
        ((None, None, None, HighlightAttr::Nothing),                           0, "hi HL term=NONE"),
        ((Some("normal"), None, None, HighlightAttr::Nothing),                 0, "hi HL term=NONE guifg=#123456 ctermfg=123"),
        ((None, Some("normal"), None, HighlightAttr::Nothing),                 0, "hi HL term=NONE guibg=#123456 ctermbg=123"),
        ((Some("normal"), Some("normal"), None, HighlightAttr::Nothing),       0, "hi HL term=NONE guifg=#123456 ctermfg=123 guibg=#123456 ctermbg=123"),
        ((None, None, None, HighlightAttr::Bold),                              0, "exe 'hi' 'HL' 'term=NONE' s:bold_attr"),
        ((None, None, None, HighlightAttr::Italic),                            0, "exe 'hi' 'HL' 'term=NONE' s:italic_attr"),
        ((None, None, None, HighlightAttr::Underline),                         0, "hi HL term=NONE gui=underline cterm=underline"),
        ((None, None, None, HighlightAttr::CommentItalic),                     0, "exe 'hi' 'HL' 'term=NONE' g:spring_night_italic_comments ? s:italic_attr : ''"),
        ((None, None, None, HighlightAttr::Undercurl),                         0, "exe 'hi' 'HL' 'term=NONE' s:undercurl_attr"),
        ((Some("contrast"), None, None, HighlightAttr::Nothing),               0, "exe 'hi' 'HL' 'term=NONE' 'guifg='.s:contrast_gui 'ctermfg='.s:contrast_cterm"),
        ((None, Some("contrast"), None, HighlightAttr::Nothing),               0, "exe 'hi' 'HL' 'term=NONE' 'guibg='.s:contrast_gui 'ctermbg='.s:contrast_cterm"),
        ((Some("contrast"), Some("contrast"), None, HighlightAttr::Underline), 0, "exe 'hi' 'HL' 'term=NONE' 'guifg='.s:contrast_gui 'ctermfg='.s:contrast_cterm 'guibg='.s:contrast_gui 'ctermbg='.s:contrast_cterm 'gui=underline cterm=underline'"),
        ((None, None, Some("normal"), HighlightAttr::Nothing),                 0, "hi HL term=NONE guisp=#123456"),
        ((None, None, Some("normal"), HighlightAttr::Undercurl),               0, "exe 'hi' 'HL' 'term=NONE' 'guisp=#123456' s:undercurl_attr"),
        ((None, None, None, HighlightAttr::Nothing),                           1, "    hi HL term=NONE"),
        ((None, None, None, HighlightAttr::Undercurl),                         1, "    exe 'hi' 'HL' 'term=NONE' s:undercurl_attr"),
    ];

    for ((fg, bg, sp, attr), indent, expected) in testcases {
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
        let mut w = Writer {
            table: m,
            highlights: &[],
            term_colors: DUMMY_TERM_COLORS,
            airline_theme: dummy_airline_theme(),
            out: Vec::new(),
        };
        w.write_highlight(&hl, indent as u32).unwrap();
        assert_eq!(str::from_utf8(&w.out).unwrap(), format!("{}\n", expected));
    }

    // Edge case
    let mut w = Writer {
        table: HashMap::new(),
        highlights: &[],
        term_colors: DUMMY_TERM_COLORS,
        airline_theme: dummy_airline_theme(),
        out: Vec::new(),
    };
    w.write_highlights().unwrap();
    assert_eq!(str::from_utf8(&w.out).unwrap(), "\n");
}

#[test]
fn test_write_highlights() {
    fn hl() -> Highlight {
        Highlight {
            name: "HL",
            fg: None,
            bg: None,
            sp: None,
            attr: HighlightAttr::Nothing,
        }
    }
    let mut w = Writer {
        table: HashMap::new(),
        highlights: &[Always(hl())],
        term_colors: DUMMY_TERM_COLORS,
        airline_theme: dummy_airline_theme(),
        out: Vec::new(),
    };
    w.write_highlights().unwrap();
    assert_eq!(str::from_utf8(&w.out).unwrap(), "hi HL term=NONE\n\n");

    let mut w = Writer {
        table: HashMap::new(),
        highlights: &[Switch(hl(), hl())],
        term_colors: DUMMY_TERM_COLORS,
        airline_theme: dummy_airline_theme(),
        out: Vec::new(),
    };
    w.write_highlights().unwrap();
    assert_eq!(
        str::from_utf8(&w.out).unwrap().lines().collect::<Vec<_>>(),
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
    let mut w = Writer {
        table: m,
        highlights: &[],
        term_colors: [
            "normal", "contrast", "normal", "contrast", "normal", "contrast", "normal", "contrast",
            "normal", "contrast", "normal", "contrast", "normal", "contrast", "normal", "contrast",
        ],
        airline_theme: dummy_airline_theme(),
        out: Vec::new(),
    };
    w.write_term_colors().unwrap();
    let rendered = str::from_utf8(&w.out).unwrap();
    assert!(rendered.contains("let g:terminal_color_0 = '#123456'"));
    assert!(rendered.contains("let g:terminal_color_1 = '#000000'"));
    assert!(rendered.contains("let g:terminal_color_0 = 123"));
    assert!(rendered.contains("let g:terminal_color_1 = 1"));
    assert!(rendered.contains("let g:terminal_ansi_colors = ['#123456', '#000000', '#123456', '#000000', '#123456', '#000000', '#123456', '#000000', '#123456', '#000000', '#123456', '#000000', '#123456', '#000000', '#123456', '#000000']"));
}

#[test]
fn test_spring_night_writer() {
    // Check duplicate highlights
    let w = spring_night_writer(Vec::new());
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

#[test]
fn test_write_airline_theme() {
    let mut table = HashMap::new();
    table.insert(
        "color1",
        Color {
            gui: ColorCode::Normal("#123456"),
            cterm: ColorCode::Normal(123),
        },
    );
    table.insert(
        "color2",
        Color {
            gui: ColorCode::Contrast("#000000", "#ffffff"),
            cterm: ColorCode::Contrast(1, 2),
        },
    );
    // Userd for accents
    table.insert(
        "red",
        Color {
            gui: ColorCode::Normal("#ff0000"),
            cterm: ColorCode::Normal(123),
        },
    );

    let airline_theme = AirlineThemeColors {
        mode: {
            let mut m = HashMap::new();
            m.insert(
                "normal",
                ThemeModeColor {
                    label: ("color1", "color2"),
                    info: ("color2", "color1"),
                    main: ("color2", "color2"),
                    modified: Some("color1"),
                    modified_main: Some("color1"),
                },
            );
            m.insert(
                "insert",
                ThemeModeColor {
                    label: ("color2", "color1"),
                    info: ("color1", "color2"),
                    main: ("color1", "color1"),
                    modified: Some("color1"),
                    modified_main: None,
                },
            );
            m.insert(
                "visual",
                ThemeModeColor {
                    label: ("color1", "color1"),
                    info: ("color2", "color2"),
                    main: ("color1", "color1"),
                    modified: None,
                    modified_main: None,
                },
            );
            m.insert(
                "replace",
                ThemeModeColor {
                    label: ("color1", "color1"),
                    info: ("color2", "color2"),
                    main: ("color1", "color1"),
                    modified: Some("color1"),
                    modified_main: None,
                },
            );
            m.insert(
                "inactive",
                ThemeModeColor {
                    label: ("color1", "color1"),
                    info: ("color2", "color2"),
                    main: ("color1", "color1"),
                    modified: Some("color2"),
                    modified_main: Some("color2"),
                },
            );
            m
        },
        paste: "color1",
        info_mod: "color2",
        error: ("color1", "color2"),
        warning: ("color2", "color1"),
    };

    let mut w = Writer {
        table,
        highlights: &[],
        term_colors: DUMMY_TERM_COLORS,
        airline_theme,
        out: Vec::new(),
    };

    w.write_airline_theme().unwrap();
    let rendered = str::from_utf8(&w.out).unwrap();

    let re_var = Regex::new(r"^let g:airline#themes#spring_night#palette\.(\w+) =").unwrap();
    let re_palette =
        Regex::new(r"^\\\s+'(red|airline_(a|b|c|x|y|z|error|warning))': \[('(#[[:xdigit:]]{6})?',\s*){2}((\d{1,3}|''),\s*){2}''\]").unwrap();
    for line in rendered.lines() {
        if line.starts_with("let g:") {
            match re_var.captures(line) {
                Some(found) => {
                    let mode = &found[1];
                    assert!(
                        w.airline_theme.mode.keys().any(|m| *m == mode
                            || format!("{}_modified", m) == mode
                            || format!("{}_paste", m) == mode
                            || "accents" == mode),
                        "Unknown mode: {}",
                        mode
                    );
                }
                None => assert!(
                    line == "let g:airline#themes#spring_night#palette = {}",
                    "Invalid variable definition: {}",
                    line
                ),
            }
        } else if line.starts_with("\\   ") {
            assert!(re_palette.is_match(line), "Invalid color palette: {}", line);
        }
    }
}
