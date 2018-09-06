use super::*;
use std::collections::HashMap;
use std::io;
use std::str;

struct StrBuf {
    buf: String,
}

impl StrBuf {
    fn new() -> Self {
        StrBuf { buf: String::new() }
    }
}

impl io::Write for StrBuf {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.push_str(str::from_utf8(buf).unwrap());
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

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
    let out = &mut StrBuf::new();
    {
        let mut w = Writer {
            table: HashMap::new(),
            highlights: &[],
            term_colors: DUMMY_TERM_COLORS,
            out,
        };
        w.write_header("spring-night").unwrap();
    }
    assert!(
        out.buf
            .starts_with(r#"" spring-night: Calm-colored dark color scheme"#)
    );
    assert!(out.buf.contains("let g:colors_name = 'spring-night'"));
}

#[test]
fn test_write_contrast_color_variables() {
    let out = &mut StrBuf::new();
    {
        Writer {
            table: HashMap::new(),
            highlights: &[],
            term_colors: DUMMY_TERM_COLORS,
            out,
        }.write_contrast_color_variables()
            .unwrap();
    }
    assert_eq!(out.buf, "\n".to_string());

    let out = &mut StrBuf::new();
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
    let mut lines = out.buf.lines().collect::<Vec<_>>();
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
        ((None, None, None, HighlightAttr::Nothing), 0, "hi HL term=NONE"),
        ((Some("normal"), None, None, HighlightAttr::Nothing), 0, "hi HL term=NONE guifg=#123456 ctermfg=123"),
        ((None, Some("normal"), None, HighlightAttr::Nothing), 0, "hi HL term=NONE guibg=#123456 ctermbg=123"),
        ((Some("normal"), Some("normal"), None, HighlightAttr::Nothing), 0, "hi HL term=NONE guifg=#123456 ctermfg=123 guibg=#123456 ctermbg=123"),
        ((None, None, None, HighlightAttr::Bold), 0, "exe 'hi' 'HL' 'term=NONE' !g:spring_night_kill_bold ? 'gui=bold cterm=bold' : ''"),
        ((None, None, None, HighlightAttr::Italic), 0, "exe 'hi' 'HL' 'term=NONE' !g:spring_night_kill_italic ? 'gui=italic' : ''"),
        ((None, None, None, HighlightAttr::Underline), 0, "hi HL term=NONE gui=underline cterm=underline"),
        ((None, None, None, HighlightAttr::CommentItalic), 0, "exe 'hi' 'HL' 'term=NONE' g:spring_night_italic_comments && !g:spring_night_kill_italic ? 'gui=italic' : ''"),
        ((None, None, None, HighlightAttr::Undercurl), 0, "exe 'hi' 'HL' 'term=NONE' 'gui='.s:undercurl 'cterm='.s:undercurl"),
        ((Some("contrast"), None, None, HighlightAttr::Nothing), 0, "exe 'hi' 'HL' 'term=NONE' 'guifg='.s:contrast_gui 'ctermfg='.s:contrast_cterm"),
        ((None, Some("contrast"), None, HighlightAttr::Nothing), 0, "exe 'hi' 'HL' 'term=NONE' 'guibg='.s:contrast_gui 'ctermbg='.s:contrast_cterm"),
        ((Some("contrast"), Some("contrast"), None, HighlightAttr::Underline), 0, "exe 'hi' 'HL' 'term=NONE' 'guifg='.s:contrast_gui 'ctermfg='.s:contrast_cterm 'guibg='.s:contrast_gui 'ctermbg='.s:contrast_cterm 'gui=underline cterm=underline'"),
        ((None, None, None, HighlightAttr::Nothing), 1, "    hi HL term=NONE"),
    ];

    for ((fg, bg, sp, attr), indent, expected) in testcases {
        let out = &mut StrBuf::new();
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
        assert_eq!(out.buf, format!("{}\n", expected));
    }

    // Edge case
    let out = &mut StrBuf::new();
    {
        Writer {
            table: HashMap::new(),
            highlights: &[],
            term_colors: DUMMY_TERM_COLORS,
            out,
        }.write_highlights()
            .unwrap();
    }
    assert_eq!(out.buf, "\n".to_string());
}

#[test]
fn test_write_highlights() {
    let out = &mut StrBuf::new();
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
    assert_eq!(out.buf, "hi HL term=NONE\n\n");

    let out = &mut StrBuf::new();
    Writer {
        table: HashMap::new(),
        highlights: &[Switch(hl(), hl())],
        term_colors: DUMMY_TERM_COLORS,
        out,
    }.write_highlights()
        .unwrap();
    assert_eq!(
        out.buf.lines().collect::<Vec<_>>(),
        [
            "if s:gui_running",
            "    hi HL term=NONE",
            "else",
            "    hi HL term=NONE",
            "endif",
            "",
        ].iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
    );
}

#[test]
fn test_write_term_colors() {
    let out = &mut StrBuf::new();
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
    assert!(out.buf.contains("let g:terminal_color_0 = '#123456'"));
    assert!(out.buf.contains("let g:terminal_color_1 = '#000000'"));
    assert!(out.buf.contains("let g:terminal_color_0 = 123"));
    assert!(out.buf.contains("let g:terminal_color_1 = 1"));
    assert!(out.buf.contains("let g:terminal_ansi_colors = ["));
    assert!(out.buf.contains("\\       '#123456'"));
    assert!(out.buf.contains("\\       '#000000'"));
}
