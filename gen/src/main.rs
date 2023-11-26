#[cfg(test)]
mod test;

use anyhow::{Context, Result};
use getopts::Options;
use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
enum ColorCode<T: Display> {
    Normal(T),
    Contrast(T, T),
}

impl<T: Display> ColorCode<T> {
    fn normal(&self) -> &T {
        match self {
            ColorCode::Normal(c) => c,
            ColorCode::Contrast(h, _) => h,
        }
    }
}

const NONE_COLOR: ColorCode<&'static str> = ColorCode::Normal("NONE");

#[derive(Debug, PartialEq)]
struct Color {
    gui: ColorCode<&'static str>,
    cterm: ColorCode<u8>,
}

type ColorTable = HashMap<&'static str, Color>;

type ColorName = Option<&'static str>;

#[derive(Debug, PartialEq)]
enum HighlightAttr {
    Nothing,
    Bold,
    Italic,
    Underline,
    Reverse,
    None,
    CommentItalic,
    Undercurl,
}

#[derive(Debug)]
struct Highlight {
    name: &'static str,
    fg: ColorName,
    bg: ColorName,
    sp: ColorName,
    attr: HighlightAttr,
}

#[derive(Debug)]
enum HowToHighlight {
    Always(Highlight),
    Switch(Highlight, Highlight),
}

use crate::HowToHighlight::{Always, Switch};

macro_rules! highlight {
    ($name:ident, $fg:expr, $bg:expr, $sp:expr, $attr:ident) => {
        Highlight {
            name: stringify!($name),
            fg: $fg,
            bg: $bg,
            sp: $sp,
            attr: HighlightAttr::$attr,
        }
    };
}

macro_rules! fgbg {
    ($name:ident, - , - , $attr:ident) => {
        highlight!($name, None, None, None, $attr)
    };
    ($name:ident, $fg:ident, - , $attr:ident) => {
        highlight!($name, Some(stringify!($fg)), None, None, $attr)
    };
    ($name:ident, - , $bg:ident, $attr:ident) => {
        highlight!($name, None, Some(stringify!($bg)), None, $attr)
    };
    ($name:ident, $fg:ident, $bg:ident, $attr:ident) => {
        highlight!(
            $name,
            Some(stringify!($fg)),
            Some(stringify!($bg)),
            None,
            $attr
        )
    };
}

macro_rules! fgbgsp {
    ($name:ident, $fg:ident, - , $sp:ident, $attr:ident) => {
        highlight!(
            $name,
            Some(stringify!($fg)),
            None,
            Some(stringify!($sp)),
            $attr
        )
    };
    ($name:ident, $fg:ident, $bg:ident, $sp:ident, $attr:ident) => {
        highlight!(
            $name,
            Some(stringify!($fg)),
            Some(stringify!($bg)),
            Some(stringify!($sp)),
            $attr
        )
    };
}

#[derive(Debug, PartialEq)]
struct ThemeModeColor<'a> {
    label: (&'a str, &'a str),
    info: (&'a str, &'a str),
    main: (&'a str, &'a str),
    modified: Option<&'a str>,
    modified_main: Option<&'a str>,
}

#[derive(Debug)]
struct AirlineThemeColors<'a> {
    mode: HashMap<&'a str, ThemeModeColor<'a>>,
    paste: &'a str,
    info_mod: &'a str,
    error: (&'a str, &'a str),
    warning: (&'a str, &'a str),
}

#[derive(Debug)]
struct Writer<'a, W: io::Write> {
    table: ColorTable,
    highlights: &'a [HowToHighlight],
    term_colors: [&'static str; 16],
    airline_theme: AirlineThemeColors<'a>,
    out: W,
}

impl<'a, W: io::Write> Writer<'a, W> {
    fn write_header(&mut self) -> io::Result<()> {
        write!(
            self.out,
            r#"" spring-night: Calm-colored dark color scheme
"
" Author: rhysd <lin90162@yahoo.co.jp>
" License: MIT
"   Copyright (c) 2016 rhysd
"
" PLEASE DO NOT MODIFY THIS FILE DIRECTLY!
" Generated by script vim-color-spring-night/gen/{source}

" Optimization:
" `:set background=dark` has some side effects which takes a time.
" Avoid the side effects when the value is already 'dark'.
if &background !=# 'dark'
    set background=dark
endif

" Optimization:
" `:hi clear` takes a lot of time since it clears all highlights and set default
" highlights. This guard avoids `:hi clear` if spring-night is the first colorscheme.
" applied in vimrc. In almost all cases no additional highlights are set at start
" up since they are set by Vim plugins.
if exists('g:colors_name')
    " Remove all existing user-defined highlights and set the defaults.
    hi clear
endif

if exists('g:syntax_on')
    syntax reset
endif

let g:colors_name = 'spring-night'

let g:spring_night_italic_comments = get(g:, 'spring_night_italic_comments', 0)
let g:spring_night_kill_italic = get(g:, 'spring_night_kill_italic', 0)
let g:spring_night_kill_bold = get(g:, 'spring_night_kill_bold', 0)
let g:spring_night_highlight_terminal = get(g:, 'spring_night_highlight_terminal', 1)
let g:spring_night_cterm_italic = get(g:, 'spring_night_cterm_italic', 0)

let s:gui_running = has('gui_running')
let s:true_colors = has('termguicolors') && &termguicolors
let s:undercurl_attr = s:gui_running ? 'gui=undercurl cterm=undercurl' : 'gui=underline cterm=underline'
let s:italic_attr = g:spring_night_kill_italic ? '' : g:spring_night_cterm_italic ? 'gui=italic cterm=italic' : 'gui=italic'
let s:bold_attr = g:spring_night_kill_bold ? '' : 'gui=bold cterm=bold'

if exists('g:spring_night_high_contrast')
    if type(g:spring_night_high_contrast) != type(0)
        echoerr 'g:spring_night_high_contrast was changed to number value. Please read README.md of vim-color-spring-night repository and set proper value'
        let g:spring_night_high_contrast = !s:gui_running && s:true_colors
    endif
else
    let g:spring_night_high_contrast = !s:gui_running && s:true_colors
endif

"#,
            source = file!(),
        )
    }

    fn write_contrast_color_variables(&mut self) -> io::Result<()> {
        // Sort by key name to avoid random order
        for (name, color) in {
            let mut v = self.table.iter().collect::<Vec<_>>();
            v.sort_by_key(|(&k, _)| k);
            v
        } {
            if let ColorCode::Contrast(high, low) = color.gui {
                writeln!(
                    self.out,
                    "let s:{}_gui = g:spring_night_high_contrast ? '{}' : '{}'",
                    name, high, low
                )?;
            }
            if let ColorCode::Contrast(high, low) = color.cterm {
                writeln!(
                    self.out,
                    "let s:{}_cterm = g:spring_night_high_contrast ? {} : {}",
                    name, high, low
                )?;
            }
        }
        writeln!(self.out)
    }

    fn build_highlight_item<T: Display>(
        &self,
        color_name: &'static str,
        item_name: &'static str,
        color: &ColorCode<T>,
    ) -> String {
        match color {
            ColorCode::Normal(c) => format!("{}={}", item_name, c),
            ColorCode::Contrast(..) => {
                if item_name.starts_with("gui") {
                    format!("'{}='.s:{}_gui", item_name, color_name)
                } else {
                    format!("'{}='.s:{}_cterm", item_name, color_name)
                }
            }
        }
    }

    fn write_highlight(&mut self, highlight: &Highlight, indent: u32) -> io::Result<()> {
        let mut args = vec![highlight.name.to_string(), "term=NONE".to_string()];

        for &(color_name, gui, cterm) in &[
            (&highlight.fg, "guifg", "ctermfg"),
            (&highlight.bg, "guibg", "ctermbg"),
        ] {
            if let Some(ref name) = color_name {
                if name != &"NONE" {
                    let color = &self.table[name];
                    args.push(self.build_highlight_item(name, gui, &color.gui));
                    args.push(self.build_highlight_item(name, cterm, &color.cterm));
                } else {
                    args.push(self.build_highlight_item(name, gui, &NONE_COLOR));
                    args.push(self.build_highlight_item(name, cterm, &NONE_COLOR));
                }
            }
        }

        if let Some(ref name) = highlight.sp {
            // Note: ctermsp does not exist
            args.push(self.build_highlight_item(
                name,
                "guisp",
                &self.table[name].gui, // Currently guisp must not be NONE
            ));
        }

        let attr_item = match highlight.attr {
            HighlightAttr::Nothing => "",
            HighlightAttr::Bold => "s:bold_attr",
            HighlightAttr::Italic => "s:italic_attr",
            HighlightAttr::Underline => "gui=underline cterm=underline",
            HighlightAttr::Reverse => "gui=reverse cterm=reverse",
            HighlightAttr::None => "gui=NONE cterm=NONE",
            HighlightAttr::CommentItalic => "g:spring_night_italic_comments ? s:italic_attr : ''",
            HighlightAttr::Undercurl => "s:undercurl_attr",
        };
        if !attr_item.is_empty() {
            args.push(attr_item.to_string());
        }

        let indent = match indent {
            0u32 => "",
            1u32 => "    ",
            _ => unreachable!(),
        };

        if args
            .iter()
            .any(|a| a.starts_with('\'') || a.ends_with('\'') || a.starts_with("s:"))
        {
            for arg in &mut args {
                if !arg.starts_with('\'') && !arg.ends_with('\'') && !arg.starts_with("s:") {
                    *arg = format!("'{}'", arg);
                }
            }
            writeln!(self.out, "{}exe 'hi' {}", indent, args.join(" "))
        } else {
            writeln!(self.out, "{}hi {}", indent, args.join(" "))
        }
    }

    fn write_highlights(&mut self) -> io::Result<()> {
        for highlight in self.highlights {
            match highlight {
                Always(ref hl) => self.write_highlight(hl, 0u32)?,
                Switch(ref gui, ref term) => {
                    writeln!(self.out, "if s:gui_running")?;
                    self.write_highlight(gui, 1u32)?;
                    writeln!(self.out, "else")?;
                    self.write_highlight(term, 1u32)?;
                    writeln!(self.out, "endif")?;
                }
            }
        }
        writeln!(self.out)
    }

    fn write_term_colors(&mut self) -> io::Result<()> {
        writeln!(self.out, "if g:spring_night_highlight_terminal")?;
        writeln!(self.out, "    if has('nvim')")?;
        writeln!(self.out, "        if s:gui_running || s:true_colors")?;
        for (index, name) in self.term_colors.iter().enumerate() {
            writeln!(
                self.out,
                "            let g:terminal_color_{} = '{}'",
                index,
                self.table[name].gui.normal()
            )?;
        }
        writeln!(self.out, "        else")?;
        for (index, name) in self.term_colors.iter().enumerate() {
            writeln!(
                self.out,
                "            let g:terminal_color_{} = {}",
                index,
                self.table[name].cterm.normal()
            )?;
        }
        writeln!(self.out, "        endif")?;
        writeln!(
            self.out,
            "        let g:terminal_color_background = g:terminal_color_0"
        )?;
        writeln!(
            self.out,
            "        let g:terminal_color_foreground = g:terminal_color_7"
        )?;
        writeln!(
            self.out,
            "    elseif (s:gui_running || s:true_colors) && exists('*term_setansicolors')"
        )?;
        let elems_for_vim = self
            .term_colors
            .iter()
            .map(|name| format!("'{}'", self.table[name].gui.normal()))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            self.out,
            "        let g:terminal_ansi_colors = [{}]",
            elems_for_vim
        )?;
        writeln!(self.out, "    endif")?;
        writeln!(self.out, "endif")
    }

    fn write_color_scheme(&mut self) -> io::Result<()> {
        self.write_header()?;
        self.write_contrast_color_variables()?;
        self.write_highlights()?;
        self.write_term_colors()
    }

    fn write_airline_theme_header(&mut self) -> io::Result<()> {
        let red = &self.table["red"];
        // Header
        write!(
            self.out,
            r#"" vim-airline theme for spring-night colorscheme
"
" Author: rhysd <lin90162@yahoo.co.jp>
" License: MIT
"   Copyright (c) 2016 rhysd
"
" PLEASE DO NOT MODIFY THIS FILE DIRECTLY!
" Generated by script vim-color-spring-night/gen/{source}

" TODO: Terminal mode

let g:airline#themes#spring_night#palette = {{}}

let g:airline#themes#spring_night#palette.accents = {{
\   'red': ['{guifg}', '', {ctermfg}, '', ''],
\ }}

"#,
            source = file!(),
            guifg = red.gui.normal(),
            ctermfg = red.cterm.normal(),
        )
    }

    fn build_airline_one_palette_color(&self, fgbg: (&'a str, &'a str)) -> String {
        let fg = &self.table[fgbg.0];
        let bg = &self.table[fgbg.1];
        format!(
            "['{}', '{}', {}, {}, '']",
            fg.gui.normal(),
            bg.gui.normal(),
            fg.cterm.normal(),
            bg.cterm.normal()
        )
    }

    fn write_airline_palette(&mut self, name: &str, error: &str, warning: &str) -> io::Result<()> {
        let map = &self.airline_theme.mode[name];

        writeln!(
            self.out,
            "let g:airline#themes#spring_night#palette.{} = {{",
            name
        )?;

        let label = self.build_airline_one_palette_color(map.label);
        let info = self.build_airline_one_palette_color(map.info);
        let main = self.build_airline_one_palette_color(map.main);

        writeln!(self.out, "\\   'airline_a': {},", label)?;
        writeln!(self.out, "\\   'airline_b': {},", info)?;
        writeln!(self.out, "\\   'airline_c': {},", main)?;
        writeln!(self.out, "\\   'airline_x': {},", main)?;
        writeln!(self.out, "\\   'airline_y': {},", info)?;
        writeln!(self.out, "\\   'airline_z': {},", label)?;
        writeln!(self.out, "\\   'airline_error': {},", error)?;
        writeln!(self.out, "\\   'airline_warning': {},", warning)?;
        writeln!(self.out, "\\ }}")?;

        if let Some(modified) = map.modified {
            let label = self.build_airline_one_palette_color((map.label.0, modified));
            let info =
                self.build_airline_one_palette_color((modified, self.airline_theme.info_mod));
            let main_fg = if let Some(n) = map.modified_main {
                n
            } else {
                modified
            };
            let main = self.build_airline_one_palette_color((main_fg, map.main.1));
            writeln!(
                self.out,
                "let g:airline#themes#spring_night#palette.{}_modified = {{",
                name
            )?;
            writeln!(self.out, "\\   'airline_a': {},", label)?;
            writeln!(self.out, "\\   'airline_b': {},", info)?;
            writeln!(self.out, "\\   'airline_c': {},", main)?;
            writeln!(self.out, "\\   'airline_error': {},", error)?;
            writeln!(self.out, "\\   'airline_warning': {},", warning)?;
            writeln!(self.out, "\\ }}")?;
        }

        writeln!(self.out)
    }

    fn write_airline_theme(&mut self) -> io::Result<()> {
        self.write_airline_theme_header()?;

        let error = self.build_airline_one_palette_color(self.airline_theme.error);
        let warning = self.build_airline_one_palette_color(self.airline_theme.warning);

        for name in &["normal", "insert", "visual", "replace", "inactive"] {
            self.write_airline_palette(name, &error, &warning)?;
        }

        let normal_map = &self.airline_theme.mode["normal"];
        let insert_map = &self.airline_theme.mode["insert"];

        // Insert Paste
        writeln!(
            self.out,
            "let g:airline#themes#spring_night#palette.insert_paste = {{"
        )?;
        let label =
            self.build_airline_one_palette_color((insert_map.label.0, self.airline_theme.paste));
        let info = self.build_airline_one_palette_color((
            self.airline_theme.paste,
            self.airline_theme.info_mod,
        ));
        let main =
            self.build_airline_one_palette_color((self.airline_theme.paste, normal_map.main.1));
        writeln!(self.out, "\\   'airline_a': {},", label)?;
        writeln!(self.out, "\\   'airline_b': {},", info)?;
        writeln!(self.out, "\\   'airline_c': {},", main)?;
        writeln!(self.out, "\\   'airline_error': {},", error)?;
        writeln!(self.out, "\\   'airline_warning': {},", warning)?;
        writeln!(self.out, "\\ }}\n")?;

        // Inactive Modified is a special case
        writeln!(
            self.out,
            "let g:airline#themes#spring_night#palette.inactive_modified = {{"
        )?;
        let modified_color = &self.table[normal_map.modified.unwrap()];
        let guifg = modified_color.gui.normal();
        let ctermfg = modified_color.cterm.normal();
        writeln!(
            self.out,
            "\\   'airline_c': ['{}', '', {}, '', ''],",
            guifg, ctermfg
        )?;
        writeln!(self.out, "\\   'airline_error': {},", error)?;
        writeln!(self.out, "\\   'airline_warning': {},", warning)?;
        writeln!(self.out, "\\ }}")?;

        Ok(())
    }
}

#[rustfmt::skip::macros(color_name, fgbg, fgbgsp)]
fn spring_night_writer<'a, W: io::Write>(out: W) -> Writer<'a, W> {
    let mut table = HashMap::new();
    {
        fn normal<T: Display>(c: T) -> ColorCode<T> {
            ColorCode::Normal(c)
        }

        fn contrast<T: Display>(high: T, low: T) -> ColorCode<T> {
            ColorCode::Contrast(high, low)
        }

        macro_rules! color_name {
            ($name:ident, $gui:expr, $cterm:expr) => {
                assert_eq!(
                    table.insert(
                        stringify!($name),
                        Color {
                            gui: $gui,
                            cterm: $cterm
                        }
                    ),
                    None
                )
            };
        }

        color_name!(bg,         contrast("#132132", "#334152"), normal(233));
        color_name!(bgweaker,   contrast("#213243", "#3a4b5c"), normal(235));
        color_name!(bgemphasis, normal("#3a4b5c"),              normal(235));
        color_name!(bglight,    normal("#435060"),              normal(236));
        color_name!(bgstrong,   normal("#536273"),              normal(238));
        color_name!(fg,         normal("#fffeeb"),              contrast(231, 230));
        color_name!(hiddenfg,   normal("#607080"),              normal(60));
        color_name!(weakfg,     normal("#8d9eb2"),              normal(103));
        color_name!(weakerfg,   normal("#788898"),              normal(102));
        color_name!(palepink,   normal("#e7c6b7"),              normal(181));
        color_name!(yellow,     normal("#f0eaaa"),              normal(229));
        color_name!(white,      normal("#ffffff"),              normal(231));
        color_name!(purple,     normal("#e7d5ff"),              normal(189));
        color_name!(darkpurple, normal("#605779"),              normal(60));
        color_name!(gray,       normal("#545f6e"),              normal(59));
        color_name!(light,      normal("#646f7c"),              normal(60));
        color_name!(yaezakura,  normal("#70495d"),              normal(95));
        color_name!(sakura,     normal("#a9667a"),              normal(132));
        color_name!(orange,     normal("#f0aa8a"),              normal(216));
        color_name!(green,      normal("#a9dd9d"),              normal(150));
        color_name!(darkgreen,  normal("#5f8770"),              normal(65));
        color_name!(skyblue,    normal("#a8d2eb"),              normal(153));
        color_name!(gold,       normal("#fedf81"),              normal(222));
        color_name!(darkgold,   contrast("#484000", "#685800"), normal(58));
        color_name!(red,        normal("#fd8489"),              normal(210));
        color_name!(mildred,    normal("#ab6560"),              normal(167));
        color_name!(crimson,    normal("#ff6a6f"),              normal(203));
        color_name!(mikan,      normal("#fb8965"),              normal(209));
        color_name!(darkblue,   normal("#00091e"),              normal(235));
        color_name!(blue,       normal("#7098e6"),              normal(69));
        color_name!(paleblue,   normal("#98b8e6"),              normal(111));
        color_name!(lime,       normal("#c9fd88"),              normal(149));
        color_name!(palesakura, normal("#e996aa"),              normal(175));
        color_name!(whitepink,  normal("#ebeadb"),              normal(224));
        color_name!(whitegreen, normal("#eaf0aa"),              normal(194));
        color_name!(whiteblue,  normal("#d8e2f0"),              normal(195));
        color_name!(whitered,   normal("#ffbfaf"),              normal(217));
        color_name!(inu,        normal("#ddbc96"),              normal(180));
    }
    let table = table;

    let highlights = &[
        // Normal colors
        Always(fgbg!(Boolean,               red,        -,            Nothing)),
        Always(fgbg!(Character,             green,      -,            Nothing)),
        Always(fgbg!(ColorColumn,           -,          bgstrong,     Nothing)),
        Always(fgbg!(Comment,               weakfg,     -,            CommentItalic)),
        Always(fgbg!(Conceal,               mikan,      bg,           Nothing)),
        Always(fgbg!(Conditional,           skyblue,    -,            Nothing)),
        Always(fgbg!(Constant,              red,        -,            Nothing)),
        Always(fgbg!(Cursor,                bg,         fg,           Nothing)),
        Always(fgbg!(CursorColumn,          -,          bgemphasis,   Nothing)),
        Always(fgbg!(CursorLine,            -,          bgemphasis,   None)),
        Always(fgbg!(CursorLineNr,          purple,     bgstrong,     Nothing)),
        Always(fgbg!(Define,                orange,     -,            Nothing)),
        Always(fgbg!(Directory,             green,      -,            Nothing)),
        Always(fgbg!(EndOfBuffer,           bgstrong,   -,            Nothing)),
        Always(fgbg!(Error,                 red,        bgemphasis,   Bold)),
        Always(fgbg!(ErrorMsg,              red,        bg,           Bold)),
        Always(fgbg!(Float,                 red,        -,            Nothing)),
        Always(fgbg!(NormalFloat,           fg,         bgweaker,     Nothing)),
        Always(fgbg!(FloatBorder,           weakfg,     bgweaker,     Nothing)),
        Always(fgbg!(FoldColumn,            purple,     bgemphasis,   Nothing)),
        Always(fgbg!(Folded,                purple,     light,        Nothing)),
        Always(fgbg!(Function,              orange,     -,            Nothing)),
        Always(fgbg!(Identifier,            gold,       -,            Italic)),
        Always(fgbg!(IncSearch,             NONE,       sakura,       Underline)),
        Always(fgbg!(Keyword,               yellow,     -,            Bold)),
        Always(fgbg!(Label,                 skyblue,    -,            Nothing)),
        Always(fgbg!(LineNr,                weakerfg,   bgemphasis,   Nothing)),
        Always(fgbg!(MatchParen,            bg,         gold,         Bold)),
        Always(fgbg!(ModeMsg,               gold,       -,            Nothing)),
        Always(fgbg!(MoreMsg,               green,      -,            Nothing)),
        Always(fgbg!(NonText,               light,      -,            Nothing)),
        Always(fgbg!(Normal,                fg,         bg,           Nothing)),
        Always(fgbg!(Number,                red,        -,            Nothing)),
        Always(fgbg!(Operater,              orange,     -,            Nothing)),
        Always(fgbg!(Pmenu,                 purple,     bgemphasis,   Nothing)),
        Always(fgbg!(PmenuSbar,             gold,       bgstrong,     Nothing)),
        Always(fgbg!(PmenuSel,              gold,       bgstrong,     Nothing)),
        Always(fgbg!(PmenuThumb,            gold,       weakfg,       Nothing)),
        Always(fgbg!(PreProc,               orange,     -,            Nothing)),
        Always(fgbg!(Question,              skyblue,    -,            Nothing)),
        Always(fgbg!(Search,                NONE,       darkpurple,   Underline)),
        Always(fgbg!(SignColumn,            fg,         bgemphasis,   Nothing)),
        Always(fgbg!(Special,               yellow,     -,            Bold)),
        Always(fgbg!(SpecialKey,            hiddenfg,   -,            Nothing)),
        Always(fgbg!(SpecialComment,        palepink,   -,            Nothing)),
        Switch(
            fgbgsp!(SpellBad,               red,        -,    red,    Undercurl),
            fgbgsp!(SpellBad,               red,        NONE, red,    Undercurl),
        ),
        Switch(
            fgbgsp!(SpellCap,               purple,     -,    purple, Undercurl),
            fgbgsp!(SpellCap,               purple,     NONE, purple, Undercurl),
        ),
        Switch(
            fgbgsp!(SpellLocal,             red,        -,    red,    Undercurl),
            fgbgsp!(SpellLocal,             red,        NONE, red,    Undercurl),
        ),
        Switch(
            fgbgsp!(SpellRare,              yellow,     -,    yellow, Undercurl),
            fgbgsp!(SpellRare,              yellow,     NONE, yellow, Undercurl),
        ),
        Always(fgbg!(Statement,             skyblue,    -,            Nothing)),
        Always(fgbg!(StatusLine,            fg,         bgstrong,     Bold)),
        Always(fgbg!(StatusLineNC,          weakfg,     bgemphasis,   None)),
        Always(fgbg!(StatusLineTerm,        fg,         bgstrong,     Bold)),
        Always(fgbg!(StatusLineTermNC,      weakfg,     bgemphasis,   None)),
        Always(fgbg!(StorageClass,          gold,       -,            Italic)),
        Always(fgbg!(String,                green,      -,            Nothing)),
        Always(fgbg!(TabLine,               weakfg,     bgstrong,     Nothing)),
        Always(fgbg!(TabLineFill,           bgemphasis, -,            Nothing)),
        Always(fgbg!(TabLineSel,            gold,       bg,           Bold)),
        Always(fgbg!(Tag,                   orange,     -,            Nothing)),
        Always(fgbg!(Title,                 gold,       -,            Bold)),
        Always(fgbg!(Todo,                  bg,         red,          Bold)),
        Always(fgbg!(ToolbarButton,         gold,       bg,           Bold)),
        Always(fgbg!(ToolbarLine,           weakfg,     bgstrong,     Nothing)),
        Always(fgbg!(Type,                  gold,       -,            Nothing)),
        Always(fgbg!(Underlined,            skyblue,    -,            Underline)),
        Always(fgbg!(VertSplit,             bgemphasis, bg,           Nothing)),
        Always(fgbg!(Visual,                -,          yaezakura,    Nothing)),
        Always(fgbg!(WarningMsg,            mikan,      bgemphasis,   Nothing)),
        Always(fgbg!(WildMenu,              bg,         gold,         Nothing)),
        //
        // File type specific
        //
        // Markdown is highlighted with HTML highlights in gVim but link text doesn't
        // have a color. So define it here.
        Always(fgbg!(cmakeArguments,        yellow,     -,            Nothing)),
        Always(fgbg!(cmakeOperators,        red,        -,            Nothing)),
        Always(fgbg!(cStorageClass,         yellow,     -,            Nothing)),
        Always(fgbg!(cTypedef,              yellow,     -,            Nothing)),
        Always(fgbg!(DiffAdd,               -,          darkgreen,    Bold)),
        Always(fgbg!(DiffChange,            -,          darkgold,     Bold)),
        Always(fgbg!(DiffDelete,            fg,         mildred,      Bold)),
        Always(fgbg!(DiffText,              -,          bg,           Nothing)),
        Always(fgbg!(diffAdded,             green,      -,            Nothing)),
        Always(fgbg!(diffFile,              yellow,     -,            Nothing)),
        Always(fgbg!(diffIndexLine,         gold,       -,            Nothing)),
        Always(fgbg!(diffNewFile,           yellow,     -,            Nothing)),
        Always(fgbg!(diffRemoved,           red,        -,            Nothing)),
        Always(fgbg!(gitCommitOverflow,     -,          mildred,      Nothing)),
        Always(fgbg!(gitCommitSummary,      yellow,     -,            Nothing)),
        Always(fgbg!(gitCommitSelectedFile, skyblue,    -,            Nothing)),
        Always(fgbg!(gitconfigSection,      skyblue,    -,            Bold)),
        Always(fgbg!(goBuiltins,            red,        -,            Nothing)),
        Always(fgbg!(helpExample,           skyblue,    -,            Nothing)),
        Always(fgbg!(helpCommand,           purple,     -,            Nothing)),
        Always(fgbg!(htmlBold,              -,          bgemphasis,   Nothing)),
        Always(fgbg!(htmlLinkText,          skyblue,    -,            Nothing)),
        Always(fgbg!(htmlTagName,           orange,     -,            Nothing)),
        Always(fgbg!(javaScriptBraces,      fg,         -,            Nothing)),
        Always(fgbg!(makeCommands,          yellow,     -,            Nothing)),
        Always(fgbg!(markdownCode,          yellow,     -,            Nothing)),
        Always(fgbg!(markdownUrl,           weakfg,     -,            Nothing)),
        Always(fgbg!(ocamlConstructor,      gold,       -,            Nothing)),
        Always(fgbg!(ocamlKeyChar,          skyblue,    -,            Nothing)),
        Always(fgbg!(ocamlKeyword,          gold   ,    -,            Nothing)),
        Always(fgbg!(ocamlFunDef,           skyblue,    -,            Nothing)),
        Always(fgbg!(plantumlColonLine,     skyblue,    -,            Nothing)),
        Always(fgbg!(pythonBuiltin,         red,        -,            Nothing)),
        Always(fgbg!(qfFileName,            gold,       -,            Nothing)),
        Always(fgbg!(qfLineNr,              skyblue,    -,            Nothing)),
        Always(fgbg!(rstEmphasis,           -,          bgemphasis,   Italic)),
        Always(fgbg!(rstStrongEmphasis,     -,          bgstrong,     Bold)),
        Always(fgbg!(rubyFunction,          yellow,     -,            Nothing)),
        Always(fgbg!(rubyIdentifier,        yellow,     -,            Nothing)),
        Always(fgbg!(rustEnumVariant,       gold,       -,            Nothing)),
        Always(fgbg!(rustFuncCall,          fg,         -,            Nothing)),
        Always(fgbg!(rustCommentLineDoc,    palepink,   -,            Nothing)),
        Always(fgbg!(scalaInstanceDeclaration, gold,    -,            Nothing)),
        Always(fgbg!(scalaInstanceDeclaration, gold,    -,            Nothing)),
        Always(fgbg!(tomlTable,             skyblue,    -,            Nothing)),
        Always(fgbg!(tomlTableArray,        skyblue,    -,            Nothing)),
        Always(fgbg!(tomlKey,               gold,       -,            Nothing)),
        Always(fgbg!(typescriptBraces,      fg,         -,            Nothing)),
        Always(fgbg!(typescriptAsyncFuncKeyword, skyblue, -,          Nothing)),
        Always(fgbg!(typescriptKeywordOp,   yellow,     -,            Bold)),
        Always(fgbg!(vimfilerColumn__SizeLine, weakfg,  -,            Nothing)),
        Always(fgbg!(vimfilerClosedFile,    green,      -,            Nothing)),
        Always(fgbg!(vimCommand,            skyblue,    -,            Nothing)),
        Always(fgbg!(watListDelimiter,      fg,         -,            Nothing)),
        Always(fgbg!(watInstGeneral,        yellow,     -,            Nothing)),
        Always(fgbg!(watInstGetSet,         yellow,     -,            Nothing)),
        Always(fgbg!(watInstWithType,       yellow,     -,            Nothing)),
        Always(fgbg!(watUnnamedVar  ,       purple,     -,            Nothing)),
        Always(fgbg!(zshDelimiter,          skyblue,    -,            Nothing)),
        Always(fgbg!(zshPrecommand,         red,        -,            Nothing)),
        Always(fgbg!(debugPC,               bg,         skyblue,      Nothing)),
        Always(fgbg!(debugBreakPoint,       bg,         gold,         Nothing)),
        Always(fgbg!(zigMultilineStringDelimiter, yellow, -,          Nothing)),
        //
        // Plugin specific
        //
        // Some plugins introduce its own highlight definitions. Adjust them for
        // working fine with this colorscheme.
        Always(fgbg!(ALEWarningSign,        orange,     bgemphasis,   Bold)),
        Always(fgbg!(ALEErrorSign,          bgemphasis, mildred,      Bold)),
        Always(fgbg!(ALEInfoSign,           -,          light,        Nothing)),
        Always(fgbg!(ALEError,              -,          mildred,      Nothing)),
        Always(fgbg!(ALEWarning,            -,          darkgold,     Nothing)),
        Always(fgbg!(Flake8_Error,          red,        bgemphasis,   Nothing)),
        Always(fgbg!(Flake8_Warning,        yellow,     bgemphasis,   Nothing)),
        Always(fgbg!(Flake8_PyFlake,        skyblue,    bgemphasis,   Nothing)),
        Always(fgbg!(Flake8_Complexity,     skyblue,    bgemphasis,   Nothing)),
        Always(fgbg!(Flake8_Naming,         skyblue,    bgemphasis,   Nothing)),
        Always(fgbg!(SignifySignAdd,        green,      bgemphasis,   Nothing)),
        Always(fgbg!(SignifySignChange,     yellow,     bgemphasis,   Nothing)),
        Always(fgbg!(SignifySignChangeDelete,  gold,       bgemphasis,   Nothing)),
        Always(fgbg!(SignifySignDelete,     red,        bgemphasis,   Nothing)),
        Always(fgbg!(CleverFChar,           bg,         red,          Nothing)),
        Always(fgbg!(CleverFDirect,         bg,         red,          Nothing)),
        Always(fgbg!(DirvishArg,            yellow,     -,            Bold)),
        Always(fgbg!(EasyMotionTarget,      red,        -,            Bold)),
        Always(fgbg!(EasyMotionShade,       weakfg,     bg,           Nothing)),
        Always(fgbg!(GitGutterAdd,          green,      bgemphasis,   Nothing)),
        Always(fgbg!(GitGutterChange,       yellow,     bgemphasis,   Nothing)),
        Always(fgbg!(GitGutterChangeDelete, gold,       bgemphasis,   Nothing)),
        Always(fgbg!(GitGutterDelete,       red,        bgemphasis,   Nothing)),
        Always(fgbg!(HighlightedyankRegion, -,          bgemphasis,   Nothing)),
        Switch(
            fgbg!(EasyMotionIncCursor,      bg,         fg,           Nothing),
            fgbg!(EasyMotionIncCursor,      -,          -,            Reverse),
        ),
        Always(fgbg!(plugDeleted,           weakfg,     -,            Nothing)),
        Always(fgbg!(ConflictMarker,        -,          mildred,      Nothing)),
        Always(fgbg!(IndentGuidesOdd,       -,          bgweaker,     Nothing)),
        Always(fgbg!(IndentGuidesEven,      -,          bgemphasis,   Nothing)),
    ];

    let term_colors = [
        "bg",
        "crimson",
        "green",
        "gold",
        "blue",
        "darkpurple",
        "skyblue",
        "fg",
        "weakfg",
        "red",
        "lime",
        "yellow",
        "paleblue",
        "purple",
        "skyblue",
        "white",
    ];

    //  Note: Pair strings are color names of (fg, bg)
    let airline_theme = AirlineThemeColors {
        mode: {
            let mut m = HashMap::new();

            macro_rules! theme_mode_colors {
                ($name:ident { $($n:ident: $e:expr,)+ }) => {
                    assert_eq!(m.insert(stringify!($name), ThemeModeColor { $($n: $e,)+ }), None)
                };
            }

            theme_mode_colors!(normal {
                label: ("bg", "gold"),
                info: ("gold", "hiddenfg"),
                main: ("yellow", "bglight"),
                modified: Some("green"),
                modified_main: Some("whitegreen"),
            });

            theme_mode_colors!(insert {
                label: ("bg", "skyblue"),
                info: ("skyblue", "hiddenfg"),
                main: ("whiteblue", "bglight"),
                modified: None,
                modified_main: None,
            });

            theme_mode_colors!(visual {
                label: ("bg", "palesakura"),
                info: ("palesakura", "hiddenfg"),
                main: ("whitepink", "bglight"),
                modified: Some("sakura"),
                modified_main: None,
            });

            theme_mode_colors!(replace {
                label: ("bg", "red"),
                info: ("red", "hiddenfg"),
                main: ("whitered", "bglight"),
                modified: Some("crimson"),
                modified_main: None,
            });

            theme_mode_colors!(inactive {
                label: ("weakfg", "bglight"),
                info: ("weakfg", "bglight"),
                main: ("weakfg", "bglight"),
                modified: None,
                modified_main: None,
            });

            m
        },
        paste: "mikan",
        info_mod: "hiddenfg",
        error: ("bg", "red"),
        warning: ("bg", "mikan"),
    };

    Writer {
        table,
        highlights,
        term_colors,
        airline_theme,
        out,
    }
}

fn main() -> Result<()> {
    let (program, args) = {
        let mut argv = env::args();
        (argv.next().unwrap(), argv.collect::<Vec<_>>())
    };

    let opts = {
        let mut o = Options::new();
        o.optopt(
            "d",
            "dir",
            "root directory of vim-color-spring-night repository",
            "PATH",
        );
        o.optflag("h", "help", "print this help");
        o
    };

    let matches = opts
        .parse(args)
        .with_context(|| "Please use --help option for more detail")?;

    if matches.opt_present("h") {
        let brief = &format!("Usage: {} [options]", program);
        eprintln!("{}", opts.usage(brief));
        return Ok(());
    }

    match matches.opt_str("d") {
        Some(dir) => {
            let path = PathBuf::from(&dir).join("colors").join("spring-night.vim");
            let out = io::BufWriter::new(
                File::create(&path)
                    .with_context(|| format!("Failed to read colorscheme file: {:?}", &path))?,
            );
            let mut writer = spring_night_writer(out);
            writer
                .write_color_scheme()
                .with_context(|| format!("While writing to colorscheme file {:?}", &path))?;
            let path = PathBuf::from(dir)
                .join("autoload")
                .join("airline")
                .join("themes")
                .join("spring_night.vim");
            writer.out = io::BufWriter::new(
                File::create(&path)
                    .with_context(|| format!("Could not make airline theme file {:?}", &path))?,
            );
            writer
                .write_airline_theme()
                .with_context(|| format!("While writing to airline theme file {:?}", &path))?;
        }
        None => {
            use std::io::Write;
            let out = io::BufWriter::new(io::stdout());
            let mut writer = spring_night_writer(out);
            writer
                .write_color_scheme()
                .with_context(|| "While writing colorscheme script to stdout")?;
            writeln!(writer.out)?;
            writer
                .write_airline_theme()
                .with_context(|| "While writing airline theme to stdout")?;
        }
    }

    Ok(())
}
