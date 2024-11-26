use super::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str;
use toml_edit::DocumentMut;

#[test]
fn test_color_code() {
    assert_eq!(*ColorCode::Normal(10).normal(), 10);
    assert_eq!(*ColorCode::Contrast(10, 20).normal(), 10);
}

#[test]
fn test_write_header() {
    let palette = Palette(HashMap::new());
    let w = Colorscheme::new(&palette);
    let mut out = vec![];
    w.write_header(&mut out).unwrap();
    let rendered = str::from_utf8(&out).unwrap();
    assert!(rendered.starts_with(r#"" spring-night: Calm-colored dark color scheme"#));
    assert!(rendered.contains("let g:colors_name = 'spring-night'"));
}

#[test]
fn test_write_contrast_color_variables() {
    let palette = Palette(HashMap::new());
    let w = Colorscheme::new(&palette);
    let mut out = vec![];
    w.write_contrast_color_variables(&mut out).unwrap();
    assert_eq!(str::from_utf8(&out).unwrap(), "\n");

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
    let palette = Palette(m);
    let w = Colorscheme::new(&palette);
    let mut out = vec![];
    w.write_contrast_color_variables(&mut out).unwrap();
    for (actual, expected) in [
        "let s:goodbye_gui = g:spring_night_high_contrast ? '#000000' : '#ffffff'",
        "let s:hello_gui = g:spring_night_high_contrast ? '#123456' : '#7890ab'",
        "let s:hello_cterm = g:spring_night_high_contrast ? 123 : 234",
        "let s:hi_cterm = g:spring_night_high_contrast ? 12 : 34",
        "",
    ]
    .iter()
    .zip(str::from_utf8(&out).unwrap().lines())
    {
        assert_eq!(*actual, expected);
    }
}

#[test]
fn test_write_highlight() {
    type A = HighlightAttr;

    #[rustfmt::skip]
    let testcases = vec![
        ((None, None, None, A::Nothing),             0, "hi HL term=NONE"),
        ((Some("n"), None, None, A::Nothing),        0, "hi HL term=NONE guifg=#123456 ctermfg=123"),
        ((None, Some("n"), None, A::Nothing),        0, "hi HL term=NONE guibg=#123456 ctermbg=123"),
        ((Some("n"), Some("n"), None, A::Nothing),   0, "hi HL term=NONE guifg=#123456 ctermfg=123 guibg=#123456 ctermbg=123"),
        ((None, None, None, A::Bold),                0, "exe 'hi' 'HL term=NONE' s:bold_attr"),
        ((None, None, None, A::Italic),              0, "exe 'hi' 'HL term=NONE' s:italic_attr"),
        ((None, None, None, A::Underline),           0, "hi HL term=NONE gui=underline cterm=underline"),
        ((None, None, None, A::CommentItalic),       0, "exe 'hi' 'HL term=NONE' g:spring_night_italic_comments ? s:italic_attr : ''"),
        ((None, None, None, A::Undercurl),           0, "exe 'hi' 'HL term=NONE' s:undercurl_attr"),
        ((Some("c"), None, None, A::Nothing),        0, "exe 'hi' 'HL term=NONE' 'guifg='.s:c_gui 'ctermfg='.s:c_cterm"),
        ((None, Some("c"), None, A::Nothing),        0, "exe 'hi' 'HL term=NONE' 'guibg='.s:c_gui 'ctermbg='.s:c_cterm"),
        ((Some("c"), Some("c"), None, A::Underline), 0, "exe 'hi' 'HL term=NONE' 'guifg='.s:c_gui 'ctermfg='.s:c_cterm 'guibg='.s:c_gui 'ctermbg='.s:c_cterm 'gui=underline cterm=underline'"),
        ((None, None, Some("n"), A::Nothing),        0, "hi HL term=NONE guisp=#123456"),
        ((None, None, Some("n"), A::Undercurl),      0, "exe 'hi' 'HL term=NONE' 'guisp=#123456' s:undercurl_attr"),
        ((None, None, None, A::Nothing),             1, "    hi HL term=NONE"),
        ((None, None, None, A::Undercurl),           1, "    exe 'hi' 'HL term=NONE' s:undercurl_attr"),
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
            "n",
            Color {
                gui: ColorCode::Normal("#123456"),
                cterm: ColorCode::Normal(123),
            },
        );
        m.insert(
            "c",
            Color {
                gui: ColorCode::Contrast("#123456", "#7890ab"),
                cterm: ColorCode::Contrast(123, 234),
            },
        );
        let palette = Palette(m);
        let w = Colorscheme::new(&palette);
        let mut out = vec![];
        w.write_highlight(&mut out, &hl, indent).unwrap();
        assert_eq!(str::from_utf8(&out).unwrap(), format!("{}\n", expected));
    }

    // Edge case
    let palette = Palette(HashMap::new());
    let mut w = Colorscheme::new(&palette);
    w.highlightings = &[];
    let mut out = vec![];
    w.write_highlightings(&mut out).unwrap();
    assert_eq!(str::from_utf8(&out).unwrap(), "\n");
}

#[test]
fn test_write_highlights() {
    const fn hl() -> Highlight {
        Highlight {
            name: "HL",
            fg: None,
            bg: None,
            sp: None,
            attr: HighlightAttr::Nothing,
        }
    }

    let palette = Palette(HashMap::new());
    let mut w = Colorscheme::new(&palette);
    let fixed = &[Fixed(hl())];
    w.highlightings = fixed;
    let mut out = vec![];
    w.write_highlightings(&mut out).unwrap();
    assert_eq!(str::from_utf8(&out).unwrap(), "hi HL term=NONE\n\n");

    let dynamic = &[Dynamic {
        gui: hl(),
        term: hl(),
    }];
    let palette = Palette(HashMap::new());
    let mut w = Colorscheme::new(&palette);
    w.highlightings = dynamic;
    let mut out = vec![];
    w.write_highlightings(&mut out).unwrap();
    assert_eq!(
        str::from_utf8(&out).unwrap().lines().collect::<Vec<_>>(),
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
    let palette = Palette(m);
    let mut w = Colorscheme::new(&palette);
    w.term_colors = [
        "normal", "contrast", "normal", "contrast", "normal", "contrast", "normal", "contrast",
        "normal", "contrast", "normal", "contrast", "normal", "contrast", "normal", "contrast",
    ];
    let mut out = vec![];
    w.write_term_colors(&mut out).unwrap();
    let rendered = str::from_utf8(&out).unwrap();
    assert!(rendered.contains("let g:terminal_color_0 = '#123456'"));
    assert!(rendered.contains("let g:terminal_color_1 = '#000000'"));
    assert!(rendered.contains("let g:terminal_color_0 = 123"));
    assert!(rendered.contains("let g:terminal_color_1 = 1"));
    assert!(rendered.contains("let g:terminal_ansi_colors = ['#123456','#000000','#123456','#000000','#123456','#000000','#123456','#000000','#123456','#000000','#123456','#000000','#123456','#000000','#123456','#000000']"));
}

#[test]
fn test_colorscheme_writer() {
    let palette = Palette::default();
    let w = Colorscheme::new(&palette);

    // Check duplicate highlights
    let mut unique_check = HashSet::new();
    for hl in w.highlightings {
        let name = match hl {
            Fixed(h) => h.name,
            Dynamic { gui, term } => {
                assert_eq!(gui.name, term.name);
                gui.name
            }
        };
        assert!(unique_check.insert(name), "Duplicate highlight '{}'", name);
    }

    // Check terminal colors are correct
    for tc in &w.term_colors {
        assert!(
            w.palette.contains_key(tc),
            "Terminal color '{tc}' is not present in color names",
        );
    }

    // Check color code is correct
    let re = Regex::new(r"^#[[:xdigit:]]{6}$").unwrap();
    for (name, c) in w.palette.iter() {
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

#[test]
fn test_write_airline_theme() {
    let mut m = HashMap::new();
    m.insert(
        "color1",
        Color {
            gui: ColorCode::Normal("#123456"),
            cterm: ColorCode::Normal(123),
        },
    );
    m.insert(
        "color2",
        Color {
            gui: ColorCode::Contrast("#000000", "#ffffff"),
            cterm: ColorCode::Contrast(1, 2),
        },
    );
    // Userd for accents
    m.insert(
        "red",
        Color {
            gui: ColorCode::Normal("#ff0000"),
            cterm: ColorCode::Normal(123),
        },
    );

    let palette = Palette(m);
    let w = AirlineTheme {
        palette: &palette,
        modes: {
            let mut m = HashMap::new();
            m.insert(
                "normal",
                AirlineModeColors {
                    label: ("color1", "color2"),
                    info: ("color2", "color1"),
                    main: ("color2", "color2"),
                    modified: Some("color1"),
                    modified_main: Some("color1"),
                },
            );
            m.insert(
                "insert",
                AirlineModeColors {
                    label: ("color2", "color1"),
                    info: ("color1", "color2"),
                    main: ("color1", "color1"),
                    modified: Some("color1"),
                    modified_main: None,
                },
            );
            m.insert(
                "visual",
                AirlineModeColors {
                    label: ("color1", "color1"),
                    info: ("color2", "color2"),
                    main: ("color1", "color1"),
                    modified: None,
                    modified_main: None,
                },
            );
            m.insert(
                "replace",
                AirlineModeColors {
                    label: ("color1", "color1"),
                    info: ("color2", "color2"),
                    main: ("color1", "color1"),
                    modified: Some("color1"),
                    modified_main: None,
                },
            );
            m.insert(
                "inactive",
                AirlineModeColors {
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

    let mut out = vec![];
    w.write_to(&mut out).unwrap();
    let rendered = str::from_utf8(&out).unwrap();

    let re_var = Regex::new(r"^let g:airline#themes#spring_night#palette\.(\w+) =").unwrap();
    let re_palette =
        Regex::new(r"^\\\s+'(red|airline_(a|b|c|x|y|z|error|warning))': \[('(#[[:xdigit:]]{6})?',\s*){2}((\d{1,3}|''),\s*){2}''\]").unwrap();
    for line in rendered.lines() {
        if line.starts_with("let g:") {
            match re_var.captures(line) {
                Some(found) => {
                    let mode = &found[1];
                    assert!(
                        w.modes.keys().any(|m| *m == mode
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

#[test]
fn test_write_alacritty_theme() {
    let mut m = HashMap::new();
    m.insert(
        "color1",
        Color {
            gui: ColorCode::Normal("#ff0000"),
            cterm: ColorCode::Normal(123),
        },
    );
    m.insert(
        "color2",
        Color {
            gui: ColorCode::Normal("#00ff00"),
            cterm: ColorCode::Normal(123),
        },
    );
    m.insert(
        "color3",
        Color {
            gui: ColorCode::Normal("#0000ff"),
            cterm: ColorCode::Normal(123),
        },
    );
    m.insert(
        "color4",
        Color {
            gui: ColorCode::Normal("#ffff00"),
            cterm: ColorCode::Normal(123),
        },
    );
    let palette = Palette(m);

    let w = AlacrittyTheme {
        palette: &palette,
        background: "color1",
        dim: AlacrittyFgColors {
            name: "dim",
            foreground: "color4",
            black: "color4",
            red: "color4",
            green: "color4",
            yellow: "color4",
            blue: "color4",
            magenta: "color4",
            cyan: "color4",
            white: "color4",
        },
        normal: AlacrittyFgColors {
            name: "normal",
            foreground: "color2",
            black: "color2",
            red: "color2",
            green: "color2",
            yellow: "color2",
            blue: "color2",
            magenta: "color2",
            cyan: "color2",
            white: "color2",
        },
        bright: AlacrittyFgColors {
            name: "bright",
            foreground: "color3",
            black: "color3",
            red: "color3",
            green: "color3",
            yellow: "color3",
            blue: "color3",
            magenta: "color3",
            cyan: "color3",
            white: "color3",
        },
    };

    let mut out = vec![];
    w.write_to(&mut out).unwrap();
    let rendered = str::from_utf8(&out).unwrap();
    rendered.parse::<DocumentMut>().unwrap();
}
