#[cfg(test)]
mod test;

use anyhow::{Context, Result};
use getopts::Options;
use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::ops::Deref;
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
enum Highlighting {
    Fixed(Highlight),
    Dynamic { gui: Highlight, term: Highlight }, // Use different highlights for GUI and CUI
}

use Highlighting::{Dynamic, Fixed};

#[derive(Debug, PartialEq, Default)]
struct AirlineModeColors<'a> {
    label: (&'a str, &'a str),
    info: (&'a str, &'a str),
    main: (&'a str, &'a str),
    modified: Option<&'a str>,
    modified_main: Option<&'a str>,
}

#[derive(Debug, Default)]
struct AirlineThemeColors<'a> {
    modes: HashMap<&'a str, AirlineModeColors<'a>>,
    paste: &'a str,
    info_mod: &'a str,
    error: (&'a str, &'a str),
    warning: (&'a str, &'a str),
}

#[derive(Debug, Default, Clone)]
struct AlacrittyFgColors<'a> {
    foreground: &'a str,
    black: &'a str,
    red: &'a str,
    green: &'a str,
    yellow: &'a str,
    blue: &'a str,
    magenta: &'a str,
    cyan: &'a str,
    white: &'a str,
}

#[derive(Debug, Default)]
struct AlacrittyTheme<'a> {
    background: &'a str,
    dim: AlacrittyFgColors<'a>,
    normal: AlacrittyFgColors<'a>,
    bright: AlacrittyFgColors<'a>,
}

#[derive(Debug)]
struct Palette(HashMap<&'static str, Color>);

impl Deref for Palette {
    type Target = HashMap<&'static str, Color>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Palette {
    #[rustfmt::skip]
    fn default() -> Self {
        fn normal<T: Display>(c: T) -> ColorCode<T> {
            ColorCode::Normal(c)
        }

        fn contrast<T: Display>(high: T, low: T) -> ColorCode<T> {
            ColorCode::Contrast(high, low)
        }

        let mut table = HashMap::new();

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
        color_name!(light,      normal("#646f7c"),              normal(60));
        color_name!(fg,         normal("#fffeeb"),              contrast(231, 230));
        color_name!(hiddenfg,   normal("#607080"),              normal(60));
        color_name!(weakfg,     normal("#8d9eb2"),              normal(103));
        color_name!(weakerfg,   normal("#788898"),              normal(102));
        color_name!(black,      normal("#111e25"),              normal(233));
        color_name!(gray,       normal("#545f6e"),              normal(59));
        color_name!(white,      normal("#ffffff"),              normal(231));
        color_name!(nasu,       normal("#605779"),              normal(61));
        color_name!(fuchsia,    normal("#b9a5cf"),              normal(183));
        color_name!(purple,     normal("#e7d5ff"),              normal(189));
        color_name!(yaezakura,  normal("#70495d"),              normal(95));
        color_name!(sakura,     normal("#a9667a"),              normal(132));
        color_name!(kakezakura, normal("#e996aa"),              normal(175));
        color_name!(palepink,   normal("#e7c6b7"),              normal(181));
        color_name!(mikan,      normal("#fb8965"),              normal(209));
        color_name!(orange,     normal("#f0aa8a"),              normal(216));
        color_name!(darkgreen,  normal("#5f8770"),              normal(65));
        color_name!(green,      normal("#a9dd9d"),              normal(150));
        color_name!(lime,       normal("#c9fd88"),              normal(149));
        color_name!(blue,       normal("#7098e6"),              normal(69));
        color_name!(paleblue,   normal("#98b8e6"),              normal(111));
        color_name!(cloudy,     normal("#90aecb"),              normal(75));
        color_name!(skyblue,    normal("#a8d2eb"),              normal(153));
        color_name!(sunny,      normal("#b8e2fb"),              normal(195));
        color_name!(yellow,     normal("#f0eaaa"),              normal(229));
        color_name!(gold,       normal("#fedf81"),              normal(222));
        color_name!(dullgold,   normal("#b6955b"),              normal(221));
        color_name!(darkgold,   contrast("#484000", "#685800"), normal(58));
        color_name!(mildred,    normal("#ab6560"),              normal(167));
        color_name!(red,        normal("#fd8489"),              normal(210));
        color_name!(crimson,    normal("#ff6a6f"),              normal(203));
        color_name!(darkblue,   normal("#00091e"),              normal(235));
        color_name!(whitepink,  normal("#ebeadb"),              normal(224));
        color_name!(whitegreen, normal("#eaf0aa"),              normal(194));
        color_name!(whiteblue,  normal("#d8e2f0"),              normal(195));
        color_name!(whitered,   normal("#ffbfaf"),              normal(217));
        color_name!(inu,        normal("#ddbc96"),              normal(180));

        Self(table)
    }
}

#[derive(Debug)]
struct ColorschemeWriter<'a, W> {
    palette: &'a Palette,
    highlightings: &'a [Highlighting],
    term_colors: [&'static str; 16],
    out: W,
}

impl<'a, W: Write> ColorschemeWriter<'a, W> {
    #[rustfmt::skip::macros(fgbg, fgbgsp)]
    fn new(out: W, palette: &'a Palette) -> Self {
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

        let highlightings = &[
            // Normal colors
            Fixed(fgbg!(Boolean,               red,        -,            Nothing)),
            Fixed(fgbg!(Character,             green,      -,            Nothing)),
            Fixed(fgbg!(ColorColumn,           -,          bgstrong,     Nothing)),
            Fixed(fgbg!(Comment,               weakfg,     -,            CommentItalic)),
            Fixed(fgbg!(Conceal, mikan, bg, Nothing)),
            Fixed(fgbg!(Conditional,           skyblue,    -,            Nothing)),
            Fixed(fgbg!(Constant,              red,        -,            Nothing)),
            Fixed(fgbg!(Cursor, bg, fg, Nothing)),
            Fixed(fgbg!(lCursor, bg, fg, Nothing)),
            Fixed(fgbg!(CursorColumn,          -,          bgemphasis,   Nothing)),
            Fixed(fgbg!(CursorLine,            -,          bgemphasis,   None)),
            Fixed(fgbg!(CursorLineNr, purple, bgstrong, Nothing)),
            Fixed(fgbg!(Define,                orange,     -,            Nothing)),
            Fixed(fgbg!(Directory,             green,      -,            Nothing)),
            Fixed(fgbg!(EndOfBuffer,           bgstrong,   -,            Nothing)),
            Fixed(fgbg!(Error, red, bgemphasis, Bold)),
            Fixed(fgbg!(ErrorMsg, red, bg, Bold)),
            Fixed(fgbg!(Float,                 red,        -,            Nothing)),
            Fixed(fgbg!(NormalFloat, fg, bgweaker, Nothing)),
            Fixed(fgbg!(FloatBorder, weakfg, bgweaker, Nothing)),
            Fixed(fgbg!(FoldColumn, purple, bgemphasis, Nothing)),
            Fixed(fgbg!(Folded, purple, light, Nothing)),
            Fixed(fgbg!(Function,              orange,     -,            Nothing)),
            Fixed(fgbg!(Identifier,            gold,       -,            Italic)),
            Fixed(fgbg!(IncSearch, NONE, sakura, Underline)),
            Fixed(fgbg!(Keyword,               yellow,     -,            Bold)),
            Fixed(fgbg!(Label,                 skyblue,    -,            Nothing)),
            Fixed(fgbg!(LineNr, weakerfg, bgemphasis, Nothing)),
            Fixed(fgbg!(MatchParen, bg, gold, Bold)),
            Fixed(fgbg!(ModeMsg,               gold,       -,            Nothing)),
            Fixed(fgbg!(MoreMsg,               green,      -,            Nothing)),
            Fixed(fgbg!(NonText,               light,      -,            Nothing)),
            Fixed(fgbg!(Normal, fg, bg, Nothing)),
            Fixed(fgbg!(Number,                red,        -,            Nothing)),
            Fixed(fgbg!(Operater,              orange,     -,            Nothing)),
            Fixed(fgbg!(Pmenu, purple, bgemphasis, Nothing)),
            Fixed(fgbg!(PmenuSbar, gold, bgstrong, Nothing)),
            Fixed(fgbg!(PmenuSel, gold, bgstrong, Nothing)),
            Fixed(fgbg!(PmenuThumb, gold, weakfg, Nothing)),
            Fixed(fgbg!(PreProc,               orange,     -,            Nothing)),
            Fixed(fgbg!(Question,              skyblue,    -,            Nothing)),
            Fixed(fgbg!(Search, NONE, nasu, Underline)),
            Fixed(fgbg!(SignColumn, fg, bgemphasis, Nothing)),
            Fixed(fgbg!(Special,               yellow,     -,            Bold)),
            Fixed(fgbg!(SpecialKey,            hiddenfg,   -,            Nothing)),
            Fixed(fgbg!(SpecialComment,        palepink,   -,            Nothing)),
            Dynamic {
                gui: fgbgsp!(SpellBad,         red,        -,    red,    Undercurl),
                term: fgbgsp!(SpellBad, red, NONE, red, Undercurl),
            },
            Dynamic {
                gui: fgbgsp!(SpellCap,         purple,     -,    purple, Undercurl),
                term: fgbgsp!(SpellCap, purple, NONE, purple, Undercurl),
            },
            Dynamic {
                gui: fgbgsp!(SpellLocal,       red,        -,    red,    Undercurl),
                term: fgbgsp!(SpellLocal, red, NONE, red, Undercurl),
            },
            Dynamic {
                gui: fgbgsp!(SpellRare,        yellow,     -,    yellow, Undercurl),
                term: fgbgsp!(SpellRare, yellow, NONE, yellow, Undercurl),
            },
            Fixed(fgbg!(Statement,             skyblue,    -,            Nothing)),
            Fixed(fgbg!(StatusLine, fg, bgstrong, Bold)),
            Fixed(fgbg!(StatusLineNC, weakfg, bgemphasis, None)),
            Fixed(fgbg!(StatusLineTerm, fg, bgstrong, Bold)),
            Fixed(fgbg!(StatusLineTermNC, weakfg, bgemphasis, None)),
            Fixed(fgbg!(StorageClass,          gold,       -,            Italic)),
            Fixed(fgbg!(String,                green,      -,            Nothing)),
            Fixed(fgbg!(TabLine, weakfg, bgstrong, Nothing)),
            Fixed(fgbg!(TabLineFill,           bgemphasis, -,            Nothing)),
            Fixed(fgbg!(TabLineSel, gold, bg, Bold)),
            Fixed(fgbg!(Tag,                   orange,     -,            Nothing)),
            Fixed(fgbg!(Title,                 gold,       -,            Bold)),
            Fixed(fgbg!(Todo, bg, red, Bold)),
            Fixed(fgbg!(ToolbarButton, gold, bg, Bold)),
            Fixed(fgbg!(ToolbarLine, weakfg, bgstrong, Nothing)),
            Fixed(fgbg!(Type,                  gold,       -,            Nothing)),
            Fixed(fgbg!(Underlined,            skyblue,    -,            Underline)),
            Fixed(fgbg!(VertSplit, bgemphasis, bg, Nothing)),
            Fixed(fgbg!(Visual,                -,          yaezakura,    Nothing)),
            Fixed(fgbg!(WarningMsg, mikan, bgemphasis, Nothing)),
            Fixed(fgbg!(WildMenu, bg, gold, Nothing)),
            //
            // File type specific
            //
            // Markdown is highlighted with HTML highlights in gVim but link text doesn't
            // have a color. So define it here.
            Fixed(fgbg!(cmakeArguments,        yellow,     -,            Nothing)),
            Fixed(fgbg!(cmakeOperators,        red,        -,            Nothing)),
            Fixed(fgbg!(cStorageClass,         yellow,     -,            Nothing)),
            Fixed(fgbg!(cTypedef,              yellow,     -,            Nothing)),
            Fixed(fgbg!(DiffAdd,               -,          darkgreen,    Bold)),
            Fixed(fgbg!(DiffChange,            -,          darkgold,     Bold)),
            Fixed(fgbg!(DiffDelete, fg, mildred, Bold)),
            Fixed(fgbg!(DiffText,              -,          bg,           Nothing)),
            Fixed(fgbg!(diffAdded,             green,      -,            Nothing)),
            Fixed(fgbg!(diffFile,              yellow,     -,            Nothing)),
            Fixed(fgbg!(diffIndexLine,         gold,       -,            Nothing)),
            Fixed(fgbg!(diffNewFile,           yellow,     -,            Nothing)),
            Fixed(fgbg!(diffRemoved,           red,        -,            Nothing)),
            Fixed(fgbg!(gitCommitOverflow,     -,          mildred,      Nothing)),
            Fixed(fgbg!(gitCommitSummary,      yellow,     -,            Nothing)),
            Fixed(fgbg!(gitCommitSelectedFile, skyblue,    -,            Nothing)),
            Fixed(fgbg!(gitconfigSection,      skyblue,    -,            Bold)),
            Fixed(fgbg!(goBuiltins,            red,        -,            Nothing)),
            Fixed(fgbg!(helpExample,           skyblue,    -,            Nothing)),
            Fixed(fgbg!(helpCommand,           purple,     -,            Nothing)),
            Fixed(fgbg!(htmlBold,              -,          bgemphasis,   Nothing)),
            Fixed(fgbg!(htmlLinkText,          skyblue,    -,            Nothing)),
            Fixed(fgbg!(htmlTagName,           orange,     -,            Nothing)),
            Fixed(fgbg!(javaScriptBraces,      fg,         -,            Nothing)),
            Fixed(fgbg!(makeCommands,          yellow,     -,            Nothing)),
            Fixed(fgbg!(markdownCode,          yellow,     -,            Nothing)),
            Fixed(fgbg!(markdownUrl,           weakfg,     -,            Nothing)),
            Fixed(fgbg!(ocamlConstructor,      gold,       -,            Nothing)),
            Fixed(fgbg!(ocamlKeyChar,          skyblue,    -,            Nothing)),
            Fixed(fgbg!(ocamlKeyword,          gold   ,    -,            Nothing)),
            Fixed(fgbg!(ocamlFunDef,           skyblue,    -,            Nothing)),
            Fixed(fgbg!(plantumlColonLine,     skyblue,    -,            Nothing)),
            Fixed(fgbg!(pythonBuiltin,         red,        -,            Nothing)),
            Fixed(fgbg!(qfFileName,            gold,       -,            Nothing)),
            Fixed(fgbg!(qfLineNr,              skyblue,    -,            Nothing)),
            Fixed(fgbg!(rstEmphasis,           -,          bgemphasis,   Italic)),
            Fixed(fgbg!(rstStrongEmphasis,     -,          bgstrong,     Bold)),
            Fixed(fgbg!(rubyFunction,          yellow,     -,            Nothing)),
            Fixed(fgbg!(rubyIdentifier,        yellow,     -,            Nothing)),
            Fixed(fgbg!(rustEnumVariant,       gold,       -,            Nothing)),
            Fixed(fgbg!(rustFuncCall,          fg,         -,            Nothing)),
            Fixed(fgbg!(rustCommentLineDoc,    palepink,   -,            Nothing)),
            Fixed(fgbg!(scalaInstanceDeclaration, gold,    -,            Nothing)),
            Fixed(fgbg!(tomlTable,             skyblue,    -,            Nothing)),
            Fixed(fgbg!(tomlTableArray,        skyblue,    -,            Nothing)),
            Fixed(fgbg!(tomlKey,               gold,       -,            Nothing)),
            Fixed(fgbg!(tmuxCommands,          skyblue,    -,            Nothing)),
            Fixed(fgbg!(tmuxFlags,             gold,       -,            Nothing)),
            Fixed(fgbg!(tmuxFormatString,      yellow,     -,            Nothing)),
            Fixed(fgbg!(typescriptBraces,      fg,         -,            Nothing)),
            Fixed(fgbg!(typescriptAsyncFuncKeyword, skyblue, -,          Nothing)),
            Fixed(fgbg!(typescriptKeywordOp,   yellow,     -,            Bold)),
            Fixed(fgbg!(vimfilerColumn__SizeLine, weakfg,  -,            Nothing)),
            Fixed(fgbg!(vimfilerClosedFile,    green,      -,            Nothing)),
            Fixed(fgbg!(vimCommand,            skyblue,    -,            Nothing)),
            Fixed(fgbg!(watListDelimiter,      fg,         -,            Nothing)),
            Fixed(fgbg!(watInstGeneral,        yellow,     -,            Nothing)),
            Fixed(fgbg!(watInstGetSet,         yellow,     -,            Nothing)),
            Fixed(fgbg!(watInstWithType,       yellow,     -,            Nothing)),
            Fixed(fgbg!(watUnnamedVar  ,       purple,     -,            Nothing)),
            Fixed(fgbg!(zshDelimiter,          skyblue,    -,            Nothing)),
            Fixed(fgbg!(zshPrecommand,         red,        -,            Nothing)),
            Fixed(fgbg!(debugPC, bg, skyblue, Nothing)),
            Fixed(fgbg!(debugBreakPoint, bg, gold, Nothing)),
            Fixed(fgbg!(zigMultilineStringDelimiter, yellow, -,          Nothing)),
            //
            // Plugin specific
            //
            // Some plugins introduce its own highlight definitions. Adjust them for
            // working fine with this colorscheme.
            Fixed(fgbg!(ALEWarningSign, orange, bgemphasis, Bold)),
            Fixed(fgbg!(ALEErrorSign, bgemphasis, mildred, Bold)),
            Fixed(fgbg!(ALEInfoSign,           -,          light,        Nothing)),
            Fixed(fgbg!(ALEError,              -,          mildred,      Nothing)),
            Fixed(fgbg!(ALEWarning,            -,          darkgold,     Nothing)),
            Fixed(fgbg!(Flake8_Error, red, bgemphasis, Nothing)),
            Fixed(fgbg!(Flake8_Warning, yellow, bgemphasis, Nothing)),
            Fixed(fgbg!(Flake8_PyFlake, skyblue, bgemphasis, Nothing)),
            Fixed(fgbg!(Flake8_Complexity, skyblue, bgemphasis, Nothing)),
            Fixed(fgbg!(Flake8_Naming, skyblue, bgemphasis, Nothing)),
            Fixed(fgbg!(SignifySignAdd, green, bgemphasis, Nothing)),
            Fixed(fgbg!(SignifySignChange, yellow, bgemphasis, Nothing)),
            Fixed(fgbg!(SignifySignChangeDelete, gold, bgemphasis, Nothing)),
            Fixed(fgbg!(SignifySignDelete, red, bgemphasis, Nothing)),
            Fixed(fgbg!(CleverFChar, bg, red, Nothing)),
            Fixed(fgbg!(CleverFDirect, bg, red, Nothing)),
            Fixed(fgbg!(DirvishArg,            yellow,     -,            Bold)),
            Fixed(fgbg!(EasyMotionTarget,      red,        -,            Bold)),
            Fixed(fgbg!(EasyMotionShade, weakfg, bg, Nothing)),
            Fixed(fgbg!(GitGutterAdd, green, bgemphasis, Nothing)),
            Fixed(fgbg!(GitGutterChange, yellow, bgemphasis, Nothing)),
            Fixed(fgbg!(GitGutterChangeDelete, gold, bgemphasis, Nothing)),
            Fixed(fgbg!(GitGutterDelete, red, bgemphasis, Nothing)),
            Fixed(fgbg!(HighlightedyankRegion, -,          bgemphasis,   Nothing)),
            Dynamic {
                gui: fgbg!(EasyMotionIncCursor, bg, fg, Nothing),
                term: fgbg!(EasyMotionIncCursor, -,        -,            Reverse),
            },
            Fixed(fgbg!(plugDeleted,           weakfg,     -,            Nothing)),
            Fixed(fgbg!(ConflictMarker,        -,          mildred,      Nothing)),
            Fixed(fgbg!(IndentGuidesOdd,       -,          bgweaker,     Nothing)),
            Fixed(fgbg!(IndentGuidesEven,      -,          bgemphasis,   Nothing)),
        ];

        let term_colors = [
            "bg", "crimson", "green", "gold", "blue", "purple", "skyblue", "fg", "weakerfg", "red",
            "lime", "yellow", "paleblue", "purple", "sunny", "white",
        ];

        Self {
            palette,
            highlightings,
            term_colors,
            out,
        }
    }

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
            let mut v = self.palette.iter().collect::<Vec<_>>();
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
        name: &'static str,
        item: &'static str,
        color: &ColorCode<T>,
    ) -> String {
        match color {
            ColorCode::Normal(c) => format!("{}={}", item, c),
            ColorCode::Contrast(..) if item.starts_with("gui") => {
                format!("'{}='.s:{}_gui", item, name)
            }
            ColorCode::Contrast(..) => format!("'{}='.s:{}_cterm", item, name),
        }
    }

    fn write_highlight(&mut self, highlight: &Highlight, indent: u32) -> io::Result<()> {
        let mut args = vec![highlight.name.to_string(), "term=NONE".to_string()];

        for (color_name, gui, cterm) in [
            (&highlight.fg, "guifg", "ctermfg"),
            (&highlight.bg, "guibg", "ctermbg"),
        ] {
            if let Some(name) = color_name {
                if name != &"NONE" {
                    let color = &self.palette[name];
                    args.push(self.build_highlight_item(name, gui, &color.gui));
                    args.push(self.build_highlight_item(name, cterm, &color.cterm));
                } else {
                    args.push(self.build_highlight_item(name, gui, &NONE_COLOR));
                    args.push(self.build_highlight_item(name, cterm, &NONE_COLOR));
                }
            }
        }

        if let Some(name) = highlight.sp {
            // Note: ctermsp does not exist
            args.push(self.build_highlight_item(
                name,
                "guisp",
                &self.palette[name].gui, // Currently guisp must not be NONE
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

    fn write_highlightings(&mut self) -> io::Result<()> {
        for highlighting in self.highlightings {
            match highlighting {
                Fixed(hl) => self.write_highlight(hl, 0u32)?,
                Dynamic { gui, term } => {
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
                self.palette[name].gui.normal()
            )?;
        }
        writeln!(self.out, "        else")?;
        for (index, name) in self.term_colors.iter().enumerate() {
            writeln!(
                self.out,
                "            let g:terminal_color_{} = {}",
                index,
                self.palette[name].cterm.normal()
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
            .map(|name| format!("'{}'", self.palette[name].gui.normal()))
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

    fn write(&mut self) -> io::Result<()> {
        self.write_header()?;
        self.write_contrast_color_variables()?;
        self.write_highlightings()?;
        self.write_term_colors()
    }
}

#[derive(Debug)]
struct AirlineThemeWriter<'a, W> {
    palette: &'a Palette,
    theme: AirlineThemeColors<'a>,
    out: W,
}

impl<'a, W: Write> AirlineThemeWriter<'a, W> {
    fn new(out: W, palette: &'a Palette) -> Self {
        //  Note: Pair strings are color names of (fg, bg)
        let theme = AirlineThemeColors {
            modes: {
                let mut m = HashMap::new();

                macro_rules! theme_mode_colors {
                    ($name:ident { $($n:ident: $e:expr,)+ }) => {
                        assert_eq!(m.insert(stringify!($name), AirlineModeColors { $($n: $e,)+ }), None)
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
                    label: ("bg", "kakezakura"),
                    info: ("kakezakura", "hiddenfg"),
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

        Self {
            palette,
            theme,
            out,
        }
    }

    fn write_header(&mut self) -> io::Result<()> {
        let red = &self.palette["red"];
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

    fn build_one_palette_color(&self, fgbg: (&'a str, &'a str)) -> String {
        let fg = &self.palette[fgbg.0];
        let bg = &self.palette[fgbg.1];
        format!(
            "['{}', '{}', {}, {}, '']",
            fg.gui.normal(),
            bg.gui.normal(),
            fg.cterm.normal(),
            bg.cterm.normal()
        )
    }

    fn write_mode_palette(&mut self, name: &str, error: &str, warning: &str) -> io::Result<()> {
        let mode = &self.theme.modes[name];

        writeln!(
            self.out,
            "let g:airline#themes#spring_night#palette.{} = {{",
            name,
        )?;

        let label = self.build_one_palette_color(mode.label);
        let info = self.build_one_palette_color(mode.info);
        let main = self.build_one_palette_color(mode.main);

        writeln!(self.out, "\\   'airline_a': {},", label)?;
        writeln!(self.out, "\\   'airline_b': {},", info)?;
        writeln!(self.out, "\\   'airline_c': {},", main)?;
        writeln!(self.out, "\\   'airline_x': {},", main)?;
        writeln!(self.out, "\\   'airline_y': {},", info)?;
        writeln!(self.out, "\\   'airline_z': {},", label)?;
        writeln!(self.out, "\\   'airline_error': {},", error)?;
        writeln!(self.out, "\\   'airline_warning': {},", warning)?;
        writeln!(self.out, "\\ }}")?;

        if let Some(modified) = mode.modified {
            let label = self.build_one_palette_color((mode.label.0, modified));
            let info = self.build_one_palette_color((modified, self.theme.info_mod));
            let main_fg = if let Some(n) = mode.modified_main {
                n
            } else {
                modified
            };
            let main = self.build_one_palette_color((main_fg, mode.main.1));
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

    fn write(&mut self) -> io::Result<()> {
        self.write_header()?;

        let error = self.build_one_palette_color(self.theme.error);
        let warning = self.build_one_palette_color(self.theme.warning);

        for mode in &["normal", "insert", "visual", "replace", "inactive"] {
            self.write_mode_palette(mode, &error, &warning)?;
        }

        let normal_map = &self.theme.modes["normal"];
        let insert_map = &self.theme.modes["insert"];

        // Insert Paste
        writeln!(
            self.out,
            "let g:airline#themes#spring_night#palette.insert_paste = {{"
        )?;
        let label = self.build_one_palette_color((insert_map.label.0, self.theme.paste));
        let info = self.build_one_palette_color((self.theme.paste, self.theme.info_mod));
        let main = self.build_one_palette_color((self.theme.paste, normal_map.main.1));
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
        let modified_color = &self.palette[normal_map.modified.unwrap()];
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

#[derive(Debug)]
struct AlacrittyThemeWriter<'a, W> {
    palette: &'a Palette,
    theme: AlacrittyTheme<'a>,
    out: W,
}

impl<'a, W: Write> AlacrittyThemeWriter<'a, W> {
    fn new(out: W, palette: &'a Palette) -> Self {
        let theme = AlacrittyTheme {
            background: "bg",
            dim: AlacrittyFgColors {
                foreground: "yellow",
                black: "black",
                red: "mildred",
                green: "darkgreen",
                yellow: "dullgold",
                blue: "blue",
                magenta: "fuchsia",
                cyan: "cloudy",
                white: "gray",
            },
            normal: AlacrittyFgColors {
                foreground: "fg",
                black: "black",
                red: "crimson",
                green: "green",
                yellow: "dullgold",
                blue: "blue",
                magenta: "purple",
                cyan: "skyblue",
                white: "white",
            },
            bright: AlacrittyFgColors {
                foreground: "fg",
                black: "gray",
                red: "red",
                green: "lime",
                yellow: "yellow",
                blue: "paleblue",
                magenta: "purple",
                cyan: "sunny",
                white: "white",
            },
        };

        Self {
            palette,
            theme,
            out,
        }
    }

    fn write_header(&mut self) -> io::Result<()> {
        write!(
            self.out,
            r#"# Alacritty theme for spring-night colorscheme
#
# Author: rhysd <lin90162@yahoo.co.jp>
# License: MIT
#   Copyright (c) 2016 rhysd
#
# PLEASE DO NOT MODIFY THIS FILE DIRECTLY!
# Generated by script vim-color-spring-night/gen/{source}

"#,
            source = file!(),
        )
    }

    #[rustfmt::skip]
    fn write(&mut self) -> io::Result<()> {
        self.write_header()?;

        writeln!(self.out, "[colors.primary]")?;
        writeln!(self.out, "background = \"{}\"", &self.palette[self.theme.background].gui.normal())?;
        writeln!(self.out, "foreground = \"{}\"", &self.palette[self.theme.normal.foreground].gui.normal())?;
        writeln!(self.out, "dim_foreground = \"{}\"", &self.palette[self.theme.dim.foreground].gui.normal())?;
        writeln!(self.out, "bright_foreground = \"{}\"", &self.palette[self.theme.bright.foreground].gui.normal())?;

        for (ty, colors) in [
            ("dim", &self.theme.dim),
            ("normal", &self.theme.normal),
            ("bright", &self.theme.bright),
        ] {
            writeln!(self.out)?;
            writeln!(self.out, "[colors.{}]", ty)?;
            writeln!(self.out, "black = \"{}\"", &self.palette[colors.black].gui.normal())?;
            writeln!(self.out, "red = \"{}\"", &self.palette[colors.red].gui.normal())?;
            writeln!(self.out, "green = \"{}\"", &self.palette[colors.green].gui.normal())?;
            writeln!(self.out, "yellow = \"{}\"", &self.palette[colors.yellow].gui.normal())?;
            writeln!(self.out, "blue = \"{}\"", &self.palette[colors.blue].gui.normal())?;
            writeln!(self.out, "magenta = \"{}\"", &self.palette[colors.magenta].gui.normal())?;
            writeln!(self.out, "cyan = \"{}\"", &self.palette[colors.cyan].gui.normal())?;
            writeln!(self.out, "white = \"{}\"", &self.palette[colors.white].gui.normal())?;
        }

        Ok(())
    }
}

fn write_to_files(dir: &str) -> Result<()> {
    let palette = Palette::default();

    fn join(entries: &[&str]) -> PathBuf {
        let mut it = entries.iter();
        let mut path = PathBuf::from(it.next().unwrap());
        for entry in it {
            path.push(entry);
        }
        path
    }

    let path = join(&[dir, "colors", "spring-night.vim"]);
    let file = File::create(&path)
        .with_context(|| format!("Failed to read colorscheme file: {:?}", &path))?;
    ColorschemeWriter::new(BufWriter::new(file), &palette)
        .write()
        .with_context(|| format!("While writing to colorscheme file {:?}", &path))?;

    let path = join(&[dir, "autoload", "airline", "themes", "spring_night.vim"]);
    let file = File::create(&path)
        .with_context(|| format!("Could not make airline theme file {:?}", &path))?;
    AirlineThemeWriter::new(BufWriter::new(file), &palette)
        .write()
        .with_context(|| format!("While writing to airline theme file {:?}", &path))?;

    let path = join(&[dir, "alacritty", "spring_night.toml"]);
    let file = File::create(&path)
        .with_context(|| format!("Could not make alacritty theme file {:?}", &path))?;
    AlacrittyThemeWriter::new(BufWriter::new(file), &palette)
        .write()
        .with_context(|| format!("While writing to alacritty theme file {:?}", &path))?;

    Ok(())
}

fn write_to_stdout() -> Result<()> {
    let palette = Palette::default();
    let mut stdout = io::stdout().lock();

    ColorschemeWriter::new(&mut stdout, &palette)
        .write()
        .with_context(|| "While writing colorscheme to stdout")?;
    writeln!(stdout)?;
    AirlineThemeWriter::new(&mut stdout, &palette)
        .write()
        .with_context(|| "While writing airline theme to stdout")?;
    writeln!(stdout)?;
    AlacrittyThemeWriter::new(&mut stdout, &palette)
        .write()
        .with_context(|| "While writing alacritty theme to stdout")?;
    Ok(())
}

fn main() -> Result<()> {
    let (program, args) = {
        let mut argv = env::args();
        (argv.next().unwrap(), argv)
    };

    let opts = {
        let mut o = Options::new();
        o.optopt("d", "dir", "repository root directory", "PATH");
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

    if let Some(dir) = matches.opt_str("d") {
        write_to_files(&dir)?;
    } else {
        write_to_stdout()?;
    }

    Ok(())
}
