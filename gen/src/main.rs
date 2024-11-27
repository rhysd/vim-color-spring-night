#[cfg(test)]
mod test;

use anyhow::{Context, Result};
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
enum HiAttr {
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
struct HiCommand {
    name: &'static str,
    fg: ColorName,
    bg: ColorName,
    sp: ColorName,
    attr: HiAttr,
}

#[derive(Debug)]
enum Highlight {
    Fixed(HiCommand),
    Dynamic { gui: HiCommand, term: HiCommand }, // Use different highlights for GUI and CUI
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
        let mut color = |name, gui, cterm| {
            assert_eq!(table.insert(name, Color { gui, cterm }), None);
        };

        color("bg",         contrast("#132132", "#334152"), normal(233));
        color("bgweaker",   contrast("#213243", "#3a4b5c"), normal(235));
        color("bgemphasis", normal("#3a4b5c"),              normal(235));
        color("bglight",    normal("#435060"),              normal(236));
        color("bgstrong",   normal("#536273"),              normal(238));
        color("light",      normal("#646f7c"),              normal(60));
        color("fg",         normal("#fffeeb"),              contrast(231, 230));
        color("hiddenfg",   normal("#607080"),              normal(60));
        color("weakfg",     normal("#8d9eb2"),              normal(103));
        color("weakerfg",   normal("#788898"),              normal(102));
        color("black",      normal("#111e25"),              normal(233));
        color("gray",       normal("#545f6e"),              normal(59));
        color("white",      normal("#ffffff"),              normal(231));
        color("nasu",       normal("#605779"),              normal(61));
        color("fuchsia",    normal("#b9a5cf"),              normal(183));
        color("purple",     normal("#e7d5ff"),              normal(189));
        color("yaezakura",  normal("#70495d"),              normal(95));
        color("sakura",     normal("#a9667a"),              normal(132));
        color("kakezakura", normal("#e996aa"),              normal(175));
        color("palepink",   normal("#e7c6b7"),              normal(181));
        color("mikan",      normal("#fb8965"),              normal(209));
        color("orange",     normal("#f0aa8a"),              normal(216));
        color("darkgreen",  normal("#5f8770"),              normal(65));
        color("green",      normal("#a9dd9d"),              normal(150));
        color("lime",       normal("#c9fd88"),              normal(149));
        color("blue",       normal("#7098e6"),              normal(69));
        color("paleblue",   normal("#98b8e6"),              normal(111));
        color("cloudy",     normal("#90aecb"),              normal(75));
        color("skyblue",    normal("#a8d2eb"),              normal(153));
        color("sunny",      normal("#b8e2fb"),              normal(195));
        color("yellow",     normal("#f0eaaa"),              normal(229));
        color("gold",       normal("#fedf81"),              normal(222));
        color("dullgold",   normal("#b6955b"),              normal(221));
        color("darkgold",   contrast("#484000", "#685800"), normal(58));
        color("mildred",    normal("#ab6560"),              normal(167));
        color("red",        normal("#fd8489"),              normal(210));
        color("crimson",    normal("#ff6a6f"),              normal(203));
        color("darkblue",   normal("#00091e"),              normal(235));
        color("whitepink",  normal("#ebeadb"),              normal(224));
        color("whitegreen", normal("#eaf0aa"),              normal(194));
        color("whiteblue",  normal("#d8e2f0"),              normal(195));
        color("whitered",   normal("#ffbfaf"),              normal(217));
        color("inu",        normal("#ddbc96"),              normal(180));

        Self(table)
    }
}

fn indent(level: u8) -> &'static str {
    &"                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                "[..level as usize * 4]
}

#[derive(Debug)]
struct Colorscheme<'a> {
    palette: &'a Palette,
    highlights: &'a [Highlight],
    term_colors: [&'static str; 16],
}

impl<'a> Colorscheme<'a> {
    fn new(palette: &'a Palette) -> Self {
        macro_rules! color {
            (-) => {
                None // '-' means don't care
            };
            ($name:ident) => {
                Some(stringify!($name))
            };
        }

        macro_rules! hi {
            ($name:ident, $fg:tt, $bg:tt , $sp:tt, $attr:ident) => {
                HiCommand {
                    name: stringify!($name),
                    fg: color!($fg),
                    bg: color!($bg),
                    sp: color!($sp),
                    attr: HiAttr::$attr,
                }
            };
        }

        use Highlight::{Dynamic, Fixed};

        #[rustfmt::skip]
        let highlights = &[
            //        NAME                         FG          BG            SP      ATTRIBUTES
            //---------------------------------------------------------------------------------
            // Normal colors
            Fixed(hi!(Boolean,                     red,        -,            -,      Nothing)),
            Fixed(hi!(Character,                   green,      -,            -,      Nothing)),
            Fixed(hi!(ColorColumn,                 -,          bgstrong,     -,      Nothing)),
            Fixed(hi!(Comment,                     weakfg,     -,            -,      CommentItalic)),
            Fixed(hi!(Conceal,                     mikan,      bg,           -,      Nothing)),
            Fixed(hi!(Conditional,                 skyblue,    -,            -,      Nothing)),
            Fixed(hi!(Constant,                    red,        -,            -,      Nothing)),
            Fixed(hi!(Cursor,                      bg,         fg,           -,      Nothing)),
            Fixed(hi!(lCursor,                     bg,         fg,           -,      Nothing)),
            Fixed(hi!(CursorColumn,                -,          bgemphasis,   -,      Nothing)),
            Fixed(hi!(CursorLine,                  -,          bgemphasis,   -,      None)),
            Fixed(hi!(CursorLineNr,                purple,     bgstrong,     -,      Nothing)),
            Fixed(hi!(Define,                      orange,     -,            -,      Nothing)),
            Fixed(hi!(Directory,                   green,      -,            -,      Nothing)),
            Fixed(hi!(EndOfBuffer,                 bgstrong,   -,            -,      Nothing)),
            Fixed(hi!(Error,                       red,        bgemphasis,   -,      Bold)),
            Fixed(hi!(ErrorMsg,                    red,        bg,           -,      Bold)),
            Fixed(hi!(Float,                       red,        -,            -,      Nothing)),
            Fixed(hi!(NormalFloat,                 fg,         bgweaker,     -,      Nothing)),
            Fixed(hi!(FloatBorder,                 weakfg,     bgweaker,     -,      Nothing)),
            Fixed(hi!(FoldColumn,                  purple,     bgemphasis,   -,      Nothing)),
            Fixed(hi!(Folded,                      purple,     light,        -,      Nothing)),
            Fixed(hi!(Function,                    orange,     -,            -,      Nothing)),
            Fixed(hi!(Identifier,                  gold,       -,            -,      Italic)),
            Fixed(hi!(IncSearch,                   NONE,       sakura,       -,      Underline)),
            Fixed(hi!(Keyword,                     yellow,     -,            -,      Bold)),
            Fixed(hi!(Label,                       skyblue,    -,            -,      Nothing)),
            Fixed(hi!(LineNr,                      weakerfg,   bgemphasis,   -,      Nothing)),
            Fixed(hi!(MatchParen,                  bg,         gold,         -,      Bold)),
            Fixed(hi!(ModeMsg,                     gold,       -,            -,      Nothing)),
            Fixed(hi!(MoreMsg,                     green,      -,            -,      Nothing)),
            Fixed(hi!(NonText,                     light,      -,            -,      Nothing)),
            Fixed(hi!(Normal,                      fg,         bg,           -,      Nothing)),
            Fixed(hi!(Number,                      red,        -,            -,      Nothing)),
            Fixed(hi!(Operater,                    orange,     -,            -,      Nothing)),
            Fixed(hi!(Pmenu,                       purple,     bgemphasis,   -,      Nothing)),
            Fixed(hi!(PmenuSbar,                   gold,       bgstrong,     -,      Nothing)),
            Fixed(hi!(PmenuSel,                    gold,       bgstrong,     -,      Nothing)),
            Fixed(hi!(PmenuThumb,                  gold,       weakfg,       -,      Nothing)),
            Fixed(hi!(PreProc,                     orange,     -,            -,      Nothing)),
            Fixed(hi!(Question,                    skyblue,    -,            -,      Nothing)),
            Fixed(hi!(Search,                      NONE,       nasu,         -,      Underline)),
            Fixed(hi!(SignColumn,                  fg,         bgemphasis,   -,      Nothing)),
            Fixed(hi!(Special,                     yellow,     -,            -,      Bold)),
            Fixed(hi!(SpecialKey,                  hiddenfg,   -,            -,      Nothing)),
            Fixed(hi!(SpecialComment,              palepink,   -,            -,      Nothing)),
            Dynamic {
                gui:  hi!(SpellBad,                red,        -,            red,    Undercurl),
                term: hi!(SpellBad,                red,        NONE,         red,    Undercurl),
            },
            Dynamic {
                gui:  hi!(SpellCap,                purple,     -,            purple, Undercurl),
                term: hi!(SpellCap,                purple,     NONE,         purple, Undercurl),
            },
            Dynamic {
                gui:  hi!(SpellLocal,              red,        -,            red,    Undercurl),
                term: hi!(SpellLocal,              red,        NONE,         red,    Undercurl),
            },
            Dynamic {
                gui:  hi!(SpellRare,               yellow,     -,            yellow, Undercurl),
                term: hi!(SpellRare,               yellow,     NONE,         yellow, Undercurl),
            },
            Fixed(hi!(Statement,                   skyblue,    -,            -,      Nothing)),
            Fixed(hi!(StatusLine,                  fg,         bgstrong,     -,      Bold)),
            Fixed(hi!(StatusLineNC,                weakfg,     bgemphasis,   -,      None)),
            Fixed(hi!(StatusLineTerm,              fg,         bgstrong,     -,      Bold)),
            Fixed(hi!(StatusLineTermNC,            weakfg,     bgemphasis,   -,      None)),
            Fixed(hi!(StorageClass,                gold,       -,            -,      Italic)),
            Fixed(hi!(String,                      green,      -,            -,      Nothing)),
            Fixed(hi!(TabLine,                     weakfg,     bgstrong,     -,      Nothing)),
            Fixed(hi!(TabLineFill,                 bgemphasis, -,            -,      Nothing)),
            Fixed(hi!(TabLineSel,                  gold,       bg,           -,      Bold)),
            Fixed(hi!(Tag,                         orange,     -,            -,      Nothing)),
            Fixed(hi!(Title,                       gold,       -,            -,      Bold)),
            Fixed(hi!(Todo,                        bg,         red,          -,      Bold)),
            Fixed(hi!(ToolbarButton,               gold,       bg,           -,      Bold)),
            Fixed(hi!(ToolbarLine,                 weakfg,     bgstrong,     -,      Nothing)),
            Fixed(hi!(Type,                        gold,       -,            -,      Nothing)),
            Fixed(hi!(Underlined,                  skyblue,    -,            -,      Underline)),
            Fixed(hi!(VertSplit,                   bgemphasis, bg,           -,      Nothing)),
            Fixed(hi!(Visual,                      -,          yaezakura,    -,      Nothing)),
            Fixed(hi!(WarningMsg,                  mikan,      bgemphasis,   -,      Nothing)),
            Fixed(hi!(WildMenu,                    bg,         gold,         -,      Nothing)),
            //
            // File type specific
            //
            // Markdown is highlighted with H      TML highlights in gVim but link text doesn't
            // have a color. So define it her      e.
            Fixed(hi!(cmakeArguments,              yellow,     -,            -,      Nothing)),
            Fixed(hi!(cmakeOperators,              red,        -,            -,      Nothing)),
            Fixed(hi!(cStorageClass,               yellow,     -,            -,      Nothing)),
            Fixed(hi!(cTypedef,                    yellow,     -,            -,      Nothing)),
            Fixed(hi!(DiffAdd,                     -,          darkgreen,    -,      Bold)),
            Fixed(hi!(DiffChange,                  -,          darkgold,     -,      Bold)),
            Fixed(hi!(DiffDelete,                  fg,         mildred,      -,      Bold)),
            Fixed(hi!(DiffText,                    -,          bg,           -,      Nothing)),
            Fixed(hi!(diffAdded,                   green,      -,            -,      Nothing)),
            Fixed(hi!(diffFile,                    yellow,     -,            -,      Nothing)),
            Fixed(hi!(diffIndexLine,               gold,       -,            -,      Nothing)),
            Fixed(hi!(diffNewFile,                 yellow,     -,            -,      Nothing)),
            Fixed(hi!(diffRemoved,                 red,        -,            -,      Nothing)),
            Fixed(hi!(gitCommitOverflow,           -,          mildred,      -,      Nothing)),
            Fixed(hi!(gitCommitSummary,            yellow,     -,            -,      Nothing)),
            Fixed(hi!(gitCommitSelectedFile,       skyblue,    -,            -,      Nothing)),
            Fixed(hi!(gitconfigSection,            skyblue,    -,            -,      Bold)),
            Fixed(hi!(goBuiltins,                  red,        -,            -,      Nothing)),
            Fixed(hi!(helpExample,                 skyblue,    -,            -,      Nothing)),
            Fixed(hi!(helpCommand,                 purple,     -,            -,      Nothing)),
            Fixed(hi!(htmlBold,                    -,          bgemphasis,   -,      Nothing)),
            Fixed(hi!(htmlLinkText,                skyblue,    -,            -,      Nothing)),
            Fixed(hi!(htmlTagName,                 orange,     -,            -,      Nothing)),
            Fixed(hi!(javaScriptBraces,            fg,         -,            -,      Nothing)),
            Fixed(hi!(makeCommands,                yellow,     -,            -,      Nothing)),
            Fixed(hi!(markdownCode,                yellow,     -,            -,      Nothing)),
            Fixed(hi!(markdownUrl,                 weakfg,     -,            -,      Nothing)),
            Fixed(hi!(ocamlConstructor,            gold,       -,            -,      Nothing)),
            Fixed(hi!(ocamlKeyChar,                skyblue,    -,            -,      Nothing)),
            Fixed(hi!(ocamlKeyword,                gold   ,    -,            -,      Nothing)),
            Fixed(hi!(ocamlFunDef,                 skyblue,    -,            -,      Nothing)),
            Fixed(hi!(plantumlColonLine,           skyblue,    -,            -,      Nothing)),
            Fixed(hi!(pythonBuiltin,               red,        -,            -,      Nothing)),
            Fixed(hi!(qfFileName,                  gold,       -,            -,      Nothing)),
            Fixed(hi!(qfLineNr,                    skyblue,    -,            -,      Nothing)),
            Fixed(hi!(rstEmphasis,                 -,          bgemphasis,   -,      Italic)),
            Fixed(hi!(rstStrongEmphasis,           -,          bgstrong,     -,      Bold)),
            Fixed(hi!(rubyFunction,                yellow,     -,            -,      Nothing)),
            Fixed(hi!(rubyIdentifier,              yellow,     -,            -,      Nothing)),
            Fixed(hi!(rustEnumVariant,             gold,       -,            -,      Nothing)),
            Fixed(hi!(rustFuncCall,                fg,         -,            -,      Nothing)),
            Fixed(hi!(rustCommentLineDoc,          palepink,   -,            -,      Nothing)),
            Fixed(hi!(scalaInstanceDeclaration,    gold,       -,            -,      Nothing)),
            Fixed(hi!(tomlTable,                   skyblue,    -,            -,      Nothing)),
            Fixed(hi!(tomlTableArray,              skyblue,    -,            -,      Nothing)),
            Fixed(hi!(tomlKey,                     gold,       -,            -,      Nothing)),
            Fixed(hi!(tmuxCommands,                skyblue,    -,            -,      Nothing)),
            Fixed(hi!(tmuxFlags,                   gold,       -,            -,      Nothing)),
            Fixed(hi!(tmuxFormatString,            yellow,     -,            -,      Nothing)),
            Fixed(hi!(typescriptBraces,            fg,         -,            -,      Nothing)),
            Fixed(hi!(typescriptAsyncFuncKeyword,  skyblue,    -,            -,      Nothing)),
            Fixed(hi!(typescriptKeywordOp,         yellow,     -,            -,      Bold)),
            Fixed(hi!(vimfilerColumn__SizeLine,    weakfg,     -,            -,      Nothing)),
            Fixed(hi!(vimfilerClosedFile,          green,      -,            -,      Nothing)),
            Fixed(hi!(vimCommand,                  skyblue,    -,            -,      Nothing)),
            Fixed(hi!(watListDelimiter,            fg,         -,            -,      Nothing)),
            Fixed(hi!(watInstGeneral,              yellow,     -,            -,      Nothing)),
            Fixed(hi!(watInstGetSet,               yellow,     -,            -,      Nothing)),
            Fixed(hi!(watInstWithType,             yellow,     -,            -,      Nothing)),
            Fixed(hi!(watUnnamedVar,               purple,     -,            -,      Nothing)),
            Fixed(hi!(zshDelimiter,                skyblue,    -,            -,      Nothing)),
            Fixed(hi!(zshPrecommand,               red,        -,            -,      Nothing)),
            Fixed(hi!(debugPC,                     bg,         skyblue,      -,      Nothing)),
            Fixed(hi!(debugBreakPoint,             bg,         gold,         -,      Nothing)),
            Fixed(hi!(zigMultilineStringDelimiter, yellow,     -,            -,      Nothing)),
            //
            // Plugin specific
            //
            // Some plugins introduce its own highlight definitions. Adjust them for
            // working fine with this colorscheme.
            Fixed(hi!(ALEWarningSign,              orange,     bgemphasis,   -,      Bold)),
            Fixed(hi!(ALEErrorSign,                bgemphasis, mildred,      -,      Bold)),
            Fixed(hi!(ALEInfoSign,                 -,          light,        -,      Nothing)),
            Fixed(hi!(ALEError,                    -,          mildred,      -,      Nothing)),
            Fixed(hi!(ALEWarning,                  -,          darkgold,     -,      Nothing)),
            Fixed(hi!(Flake8_Error,                red,        bgemphasis,   -,      Nothing)),
            Fixed(hi!(Flake8_Warning,              yellow,     bgemphasis,   -,      Nothing)),
            Fixed(hi!(Flake8_PyFlake,              skyblue,    bgemphasis,   -,      Nothing)),
            Fixed(hi!(Flake8_Complexity,           skyblue,    bgemphasis,   -,      Nothing)),
            Fixed(hi!(Flake8_Naming,               skyblue,    bgemphasis,   -,      Nothing)),
            Fixed(hi!(SignifySignAdd,              green,      bgemphasis,   -,      Nothing)),
            Fixed(hi!(SignifySignChange,           yellow,     bgemphasis,   -,      Nothing)),
            Fixed(hi!(SignifySignChangeDelete,     gold,       bgemphasis,   -,      Nothing)),
            Fixed(hi!(SignifySignDelete,           red,        bgemphasis,   -,      Nothing)),
            Fixed(hi!(CleverFChar,                 bg,         red,          -,      Nothing)),
            Fixed(hi!(CleverFDirect,               bg,         red,          -,      Nothing)),
            Fixed(hi!(DirvishArg,                  yellow,     -,            -,      Bold)),
            Fixed(hi!(EasyMotionTarget,            red,        -,            -,      Bold)),
            Fixed(hi!(EasyMotionShade,             weakfg,     bg,           -,      Nothing)),
            Fixed(hi!(GitGutterAdd,                green,      bgemphasis,   -,      Nothing)),
            Fixed(hi!(GitGutterChange,             yellow,     bgemphasis,   -,      Nothing)),
            Fixed(hi!(GitGutterChangeDelete,       gold,       bgemphasis,   -,      Nothing)),
            Fixed(hi!(GitGutterDelete,             red,        bgemphasis,   -,      Nothing)),
            Fixed(hi!(HighlightedyankRegion,       -,          bgemphasis,   -,      Nothing)),
            Dynamic {
                gui:  hi!(EasyMotionIncCursor,     bg,         fg,           -,      Nothing),
                term: hi!(EasyMotionIncCursor,     -,          -,            -,      Reverse),
            },
            Fixed(hi!(plugDeleted,                 weakfg,     -,            -,      Nothing)),
            Fixed(hi!(ConflictMarker,              -,          mildred,      -,      Nothing)),
            Fixed(hi!(IndentGuidesOdd,             -,          bgweaker,     -,      Nothing)),
            Fixed(hi!(IndentGuidesEven,            -,          bgemphasis,   -,      Nothing)),
        ];

        let term_colors = [
            "bg",       //  0: black
            "crimson",  //  1: red
            "green",    //  2: green
            "gold",     //  3: yellow
            "blue",     //  4: blue
            "purple",   //  5: magenta
            "skyblue",  //  6: cyan
            "fg",       //  7: white
            "weakerfg", //  8: bright black (gray)
            "red",      //  9: bright red
            "lime",     // 10: bright green
            "yellow",   // 11: bright yellow
            "paleblue", // 12: bright blue
            "purple",   // 13: bright magenta
            "sunny",    // 14: bright cyan
            "white",    // 15: bright white
        ];

        Self {
            palette,
            highlights,
            term_colors,
        }
    }

    fn write_header(&self, w: &mut impl Write) -> io::Result<()> {
        write!(
            w,
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

    fn write_contrast_color_variables(&self, w: &mut impl Write) -> io::Result<()> {
        for (name, color) in {
            let mut v = self.palette.iter().collect::<Vec<_>>();
            v.sort_by_key(|(&k, _)| k); // Sort by color name to avoid random order
            v
        } {
            if let ColorCode::Contrast(high, low) = color.gui {
                writeln!(
                    w,
                    "let s:{name}_gui = g:spring_night_high_contrast ? '{high}' : '{low}'",
                )?;
            }
            if let ColorCode::Contrast(high, low) = color.cterm {
                writeln!(
                    w,
                    "let s:{name}_cterm = g:spring_night_high_contrast ? {high} : {low}",
                )?;
            }
        }
        writeln!(w)
    }

    fn write_hi_command(&self, w: &mut impl Write, cmd: &HiCommand, indents: u8) -> io::Result<()> {
        fn arg(name: &str, item: &str, color: &ColorCode<impl Display>) -> String {
            match color {
                ColorCode::Normal(c) => format!("{item}={c}"),
                ColorCode::Contrast(..) if item.starts_with("gui") => {
                    format!("'{item}='.s:{name}_gui")
                }
                ColorCode::Contrast(..) => format!("'{item}='.s:{name}_cterm"),
            }
        }

        let mut args = vec![format!("{} term=NONE", cmd.name)];

        for (color_name, gui, cterm) in
            [(&cmd.fg, "guifg", "ctermfg"), (&cmd.bg, "guibg", "ctermbg")]
        {
            if let Some(name) = color_name {
                if name != &"NONE" {
                    let color = &self.palette[name];
                    args.push(arg(name, gui, &color.gui));
                    args.push(arg(name, cterm, &color.cterm));
                } else {
                    args.push(arg(name, gui, &NONE_COLOR));
                    args.push(arg(name, cterm, &NONE_COLOR));
                }
            }
        }

        if let Some(name) = cmd.sp {
            // Note: ctermsp does not exist
            let color = &self.palette[name].gui; // Currently guisp must not be NONE
            args.push(arg(name, "guisp", color));
        }

        let attr_item = match cmd.attr {
            HiAttr::Nothing => "",
            HiAttr::Bold => "s:bold_attr",
            HiAttr::Italic => "s:italic_attr",
            HiAttr::Underline => "gui=underline cterm=underline",
            HiAttr::Reverse => "gui=reverse cterm=reverse",
            HiAttr::None => "gui=NONE cterm=NONE",
            HiAttr::CommentItalic => "g:spring_night_italic_comments ? s:italic_attr : ''",
            HiAttr::Undercurl => "s:undercurl_attr",
        };
        if !attr_item.is_empty() {
            args.push(attr_item.into());
        }

        let is_execute = args.iter().any(|a| a.contains("s:") || a.contains("g:"));
        if is_execute {
            write!(w, "{}exe 'hi'", indent(indents))?;
        } else {
            write!(w, "{}hi", indent(indents))?;
        }

        for arg in &args {
            if is_execute && !arg.contains("s:") && !arg.contains("g:") {
                write!(w, " '{}'", arg)?;
            } else {
                write!(w, " {}", arg)?;
            }
        }

        writeln!(w)
    }

    fn write_highlights(&self, w: &mut impl Write) -> io::Result<()> {
        for hl in self.highlights {
            match hl {
                Highlight::Fixed(hl) => self.write_hi_command(w, hl, 0)?,
                Highlight::Dynamic { gui, term } => {
                    writeln!(w, "if s:gui_running")?;
                    self.write_hi_command(w, gui, 1)?;
                    writeln!(w, "else")?;
                    self.write_hi_command(w, term, 1)?;
                    writeln!(w, "endif")?;
                }
            }
        }
        writeln!(w)
    }

    fn write_nvim_term_colors(&self, w: &mut impl Write, indents: u8) -> io::Result<()> {
        writeln!(w, "{}if s:gui_running || s:true_colors", indent(indents))?;
        for (index, name) in self.term_colors.iter().enumerate() {
            writeln!(
                w,
                "{indent}let g:terminal_color_{index} = '{color}'",
                indent = indent(indents + 1),
                color = self.palette[name].gui.normal(),
            )?;
        }
        writeln!(w, "{}else", indent(indents))?;
        for (index, name) in self.term_colors.iter().enumerate() {
            writeln!(
                w,
                "{indent}let g:terminal_color_{index} = {color}",
                indent = indent(indents + 1),
                color = self.palette[name].cterm.normal(),
            )?;
        }
        writeln!(w, "{}endif", indent(indents))?;
        writeln!(
            w,
            "{indent}let g:terminal_color_background = g:terminal_color_0",
            indent = indent(indents),
        )?;
        writeln!(
            w,
            "{indent}let g:terminal_color_foreground = g:terminal_color_7",
            indent = indent(indents),
        )
    }

    fn write_vim_term_colors(&self, w: &mut impl Write, indents: u8) -> io::Result<()> {
        write!(w, "{}let g:terminal_ansi_colors = [", indent(indents))?;
        for (index, name) in self.term_colors.iter().enumerate() {
            if index > 0 {
                write!(w, ",")?;
            }
            write!(w, "'{}'", self.palette[name].gui.normal())?;
        }
        writeln!(w, "]")
    }

    fn write_term_colors(&self, w: &mut impl Write) -> io::Result<()> {
        writeln!(w, "if g:spring_night_highlight_terminal")?;
        writeln!(w, "{}if has('nvim')", indent(1))?;
        self.write_nvim_term_colors(w, 2)?;
        writeln!(
            w,
            "{indent}elseif (s:gui_running || s:true_colors) && exists('*term_setansicolors')",
            indent = indent(1),
        )?;
        self.write_vim_term_colors(w, 2)?;
        writeln!(w, "{}endif", indent(1))?;
        writeln!(w, "endif")
    }

    fn write_to(&mut self, w: &mut impl Write) -> io::Result<()> {
        self.write_header(w)?;
        self.write_contrast_color_variables(w)?;
        self.write_highlights(w)?;
        self.write_term_colors(w)
    }
}

#[derive(Debug, PartialEq, Default)]
struct AirlineModeColors<'a> {
    label: (&'a str, &'a str),
    info: (&'a str, &'a str),
    main: (&'a str, &'a str),
    modified: Option<&'a str>,
    modified_main: Option<&'a str>,
}

#[derive(Debug)]
struct AirlineTheme<'a> {
    palette: &'a Palette,
    modes: HashMap<&'a str, AirlineModeColors<'a>>,
    paste: &'a str,
    info_mod: &'a str,
    error: (&'a str, &'a str),
    warning: (&'a str, &'a str),
}

impl<'a> AirlineTheme<'a> {
    fn new(palette: &'a Palette) -> Self {
        //  Note: Pairs of strings are color names of (fg, bg)
        Self {
            palette,
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
        }
    }

    fn write_header(&self, w: &mut impl Write) -> io::Result<()> {
        let red = &self.palette["red"];
        // Header
        write!(
            w,
            r#"" vim-airline theme for spring-night colorscheme
"
" Author: rhysd <lin90162@yahoo.co.jp>
" License: MIT
"   Copyright (c) 2016 rhysd
"
" PLEASE DO NOT MODIFY THIS FILE DIRECTLY!
" Generated by script vim-color-spring-night/gen/{source}

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

    fn write_section_color(
        &self,
        w: &mut impl Write,
        name: &str,
        fgbg: (&'a str, &'a str),
    ) -> io::Result<()> {
        let fg = &self.palette[fgbg.0];
        let bg = &self.palette[fgbg.1];
        writeln!(
            w,
            "\\   'airline_{name}': ['{gui_fg}', '{gui_bg}', {cterm_fg}, {cterm_bg}, ''],",
            gui_fg = fg.gui.normal(),
            gui_bg = bg.gui.normal(),
            cterm_fg = fg.cterm.normal(),
            cterm_bg = bg.cterm.normal(),
        )
    }

    fn write_error_warning(&self, w: &mut impl Write) -> io::Result<()> {
        self.write_section_color(w, "error", self.error)?;
        self.write_section_color(w, "warning", self.warning)
    }

    fn write_mode_colors(&self, w: &mut impl Write, name: &str) -> io::Result<()> {
        let mode = &self.modes[name];

        writeln!(w, "let g:airline#themes#spring_night#palette.{name} = {{")?;
        self.write_section_color(w, "a", mode.label)?;
        self.write_section_color(w, "b", mode.info)?;
        self.write_section_color(w, "c", mode.main)?;
        self.write_section_color(w, "x", mode.main)?;
        self.write_section_color(w, "y", mode.info)?;
        self.write_section_color(w, "z", mode.label)?;
        self.write_error_warning(w)?;
        writeln!(w, "\\ }}")?;

        if let Some(modified) = mode.modified {
            let main_fg = mode.modified_main.unwrap_or(modified);
            writeln!(
                w,
                "let g:airline#themes#spring_night#palette.{name}_modified = {{",
            )?;
            self.write_section_color(w, "a", (mode.label.0, modified))?;
            self.write_section_color(w, "b", (modified, self.info_mod))?;
            self.write_section_color(w, "c", (main_fg, mode.main.1))?;
            self.write_error_warning(w)?;
            writeln!(w, "\\ }}")?;
        }

        writeln!(w)
    }

    fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        self.write_header(w)?;

        for mode in &["normal", "insert", "visual", "replace", "inactive"] {
            self.write_mode_colors(w, mode)?;
        }

        let normal_map = &self.modes["normal"];
        let insert_map = &self.modes["insert"];

        // Insert Paste
        writeln!(
            w,
            "let g:airline#themes#spring_night#palette.insert_paste = {{",
        )?;
        self.write_section_color(w, "a", (insert_map.label.0, self.paste))?;
        self.write_section_color(w, "b", (self.paste, self.info_mod))?;
        self.write_section_color(w, "c", (self.paste, normal_map.main.1))?;
        self.write_error_warning(w)?;
        writeln!(w, "\\ }}\n")?;

        // Inactive Modified is a special case
        writeln!(
            w,
            "let g:airline#themes#spring_night#palette.inactive_modified = {{",
        )?;
        let modified = &self.palette[normal_map.modified.unwrap()];
        let guifg = modified.gui.normal();
        let ctermfg = modified.cterm.normal();
        writeln!(w, "\\   'airline_c': ['{guifg}', '', {ctermfg}, '', ''],")?;
        self.write_error_warning(w)?;
        writeln!(w, "\\ }}")
    }
}

#[derive(Debug, Default, Clone)]
struct AlacrittyFgColors<'a> {
    name: &'static str,
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

#[derive(Debug)]
struct AlacrittyTheme<'a> {
    palette: &'a Palette,
    background: &'a str,
    search_background: &'a str,
    search_focus_background: &'a str,
    dim: AlacrittyFgColors<'a>,
    normal: AlacrittyFgColors<'a>,
    bright: AlacrittyFgColors<'a>,
}

impl<'a> AlacrittyTheme<'a> {
    fn new(palette: &'a Palette) -> Self {
        Self {
            palette,
            background: "bg",
            dim: AlacrittyFgColors {
                name: "dim",
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
                name: "normal",
                foreground: "fg",
                black: "black",
                red: "crimson",
                green: "green",
                yellow: "gold",
                blue: "blue",
                magenta: "purple",
                cyan: "skyblue",
                white: "white",
            },
            bright: AlacrittyFgColors {
                name: "bright",
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
            search_background: "sakura",
            search_focus_background: "kakezakura",
        }
    }

    fn write_header_comment(&self, w: &mut impl Write) -> io::Result<()> {
        write!(
            w,
            r#"# Alacritty theme for spring-night colorscheme
#
# Author: rhysd <lin90162@yahoo.co.jp>
# License: MIT
#   Copyright (c) 2016 rhysd
#
# PLEASE DO NOT MODIFY THIS FILE DIRECTLY!
# Generated by script vim-color-spring-night/gen/{}
"#,
            file!(),
        )
    }

    #[rustfmt::skip]
    fn write_primary_section(&self, w: &mut impl Write) -> io::Result<()> {
        writeln!(w)?;
        writeln!(w, "[colors.primary]")?;
        writeln!(w, "background = \"{}\"",        &self.palette[self.background].gui.normal())?;
        writeln!(w, "foreground = \"{}\"",        &self.palette[self.normal.foreground].gui.normal())?;
        writeln!(w, "dim_foreground = \"{}\"",    &self.palette[self.dim.foreground].gui.normal())?;
        writeln!(w, "bright_foreground = \"{}\"", &self.palette[self.bright.foreground].gui.normal())
    }

    #[rustfmt::skip]
    fn write_colors_section(&self, w: &mut impl Write, colors: &AlacrittyFgColors<'a>) -> io::Result<()> {
        writeln!(w)?;
        writeln!(w, "[colors.{}]", colors.name)?;
        writeln!(w, "black = \"{}\"",   &self.palette[colors.black].gui.normal())?;
        writeln!(w, "red = \"{}\"",     &self.palette[colors.red].gui.normal())?;
        writeln!(w, "green = \"{}\"",   &self.palette[colors.green].gui.normal())?;
        writeln!(w, "yellow = \"{}\"",  &self.palette[colors.yellow].gui.normal())?;
        writeln!(w, "blue = \"{}\"",    &self.palette[colors.blue].gui.normal())?;
        writeln!(w, "magenta = \"{}\"", &self.palette[colors.magenta].gui.normal())?;
        writeln!(w, "cyan = \"{}\"",    &self.palette[colors.cyan].gui.normal())?;
        writeln!(w, "white = \"{}\"",   &self.palette[colors.white].gui.normal())
    }

    fn write_search_section(&self, w: &mut impl Write) -> io::Result<()> {
        writeln!(w)?;
        writeln!(w, "[colors.search]")?;
        writeln!(
            w,
            r#"matches = {{ foreground = "{fg}", background = "{bg}" }}"#,
            fg = &self.palette[self.normal.foreground].gui.normal(),
            bg = &self.palette[self.search_background].gui.normal(),
        )?;
        writeln!(
            w,
            r#"focused_match = {{ foreground = "{fg}", background = "{bg}" }}"#,
            fg = &self.palette[self.bright.foreground].gui.normal(),
            bg = &self.palette[self.search_focus_background].gui.normal(),
        )
    }

    #[rustfmt::skip]
    fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        self.write_header_comment(w)?;
        self.write_primary_section(w)?;
        for colors in [&self.dim, &self.normal, &self.bright] {
            self.write_colors_section(w, colors)?;
        }
        self.write_search_section(w)
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
        .with_context(|| format!("Could not create colorscheme file: {:?}", &path))?;
    Colorscheme::new(&palette)
        .write_to(&mut BufWriter::new(file))
        .with_context(|| format!("While generate colorscheme file {:?}", &path))?;

    let path = join(&[dir, "autoload", "airline", "themes", "spring_night.vim"]);
    let file = File::create(&path)
        .with_context(|| format!("Could not create airline theme file {:?}", &path))?;
    AirlineTheme::new(&palette)
        .write_to(&mut BufWriter::new(file))
        .with_context(|| format!("Could not generate airline theme file {:?}", &path))?;

    let path = join(&[dir, "alacritty", "spring_night.toml"]);
    let file = File::create(&path)
        .with_context(|| format!("Could not create alacritty theme file {:?}", &path))?;
    AlacrittyTheme::new(&palette)
        .write_to(&mut BufWriter::new(file))
        .with_context(|| format!("Could not generate alacritty theme file {:?}", &path))
}

fn write_to_stdout() -> Result<()> {
    let palette = Palette::default();
    let mut stdout = io::stdout().lock();

    Colorscheme::new(&palette)
        .write_to(&mut stdout)
        .context("While writing colorscheme to stdout")?;
    writeln!(stdout)?;
    AirlineTheme::new(&palette)
        .write_to(&mut stdout)
        .context("While writing airline theme to stdout")?;
    writeln!(stdout)?;
    AlacrittyTheme::new(&palette)
        .write_to(&mut stdout)
        .context("While writing alacritty theme to stdout")
}

fn main() -> Result<()> {
    let (program, args) = {
        let mut argv = env::args();
        (argv.next().unwrap(), argv)
    };

    let mut opts = getopts::Options::new();
    opts.optopt("d", "dir", "repository root directory", "PATH");
    opts.optflag("h", "help", "print this help");
    let opts = opts;

    let matches = opts
        .parse(args)
        .context("Please use --help option for more detail")?;

    if matches.opt_present("h") {
        let brief = &format!("Usage: {} [options]", program);
        eprintln!("{}", opts.usage(brief));
        return Ok(());
    }

    if let Some(dir) = matches.opt_str("d") {
        write_to_files(&dir)
    } else {
        write_to_stdout()
    }
}
