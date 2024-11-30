use crate::palette::{ColorCode, Palette};
use std::fmt::Display;
use std::io::{Result, Write};

const NONE_COLOR: ColorCode<&'static str> = ColorCode::Normal("NONE");

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

fn indent(level: u8) -> &'static str {
    &"                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                "[..level as usize * 4]
}

#[derive(Debug)]
pub struct Colorscheme<'a> {
    palette: &'a Palette,
    highlights: &'a [Highlight],
    term_colors: [&'static str; 16],
}

impl<'a> Colorscheme<'a> {
    pub fn new(palette: &'a Palette) -> Self {
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

    fn write_header(&self, w: &mut impl Write) -> Result<()> {
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

    fn write_contrast_color_variables(&self, w: &mut impl Write) -> Result<()> {
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

    fn write_hi_command(&self, w: &mut impl Write, cmd: &HiCommand, indents: u8) -> Result<()> {
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

    fn write_highlights(&self, w: &mut impl Write) -> Result<()> {
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

    fn write_nvim_term_colors(&self, w: &mut impl Write, indents: u8) -> Result<()> {
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

    fn write_vim_term_colors(&self, w: &mut impl Write, indents: u8) -> Result<()> {
        write!(w, "{}let g:terminal_ansi_colors = [", indent(indents))?;
        for (index, name) in self.term_colors.iter().enumerate() {
            if index > 0 {
                write!(w, ",")?;
            }
            write!(w, "'{}'", self.palette[name].gui.normal())?;
        }
        writeln!(w, "]")
    }

    fn write_term_colors(&self, w: &mut impl Write) -> Result<()> {
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

    pub fn write_to(&mut self, w: &mut impl Write) -> Result<()> {
        self.write_header(w)?;
        self.write_contrast_color_variables(w)?;
        self.write_highlights(w)?;
        self.write_term_colors(w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palette::Color;
    use std::collections::{HashMap, HashSet};
    use std::str;

    #[test]
    fn test_write_header() {
        let palette = Palette::from(HashMap::new());
        let w = Colorscheme::new(&palette);
        let mut out = vec![];
        w.write_header(&mut out).unwrap();
        let rendered = str::from_utf8(&out).unwrap();
        assert!(rendered.starts_with(r#"" spring-night: Calm-colored dark color scheme"#));
        assert!(rendered.contains("let g:colors_name = 'spring-night'"));
    }

    #[test]
    fn test_write_contrast_color_variables() {
        let palette = Palette::from(HashMap::new());
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
        let palette = Palette::from(m);
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
        #[rustfmt::skip]
        let testcases = vec![
            ((None, None, None, HiAttr::Nothing),             0, "hi HL term=NONE"),
            ((Some("n"), None, None, HiAttr::Nothing),        0, "hi HL term=NONE guifg=#123456 ctermfg=123"),
            ((None, Some("n"), None, HiAttr::Nothing),        0, "hi HL term=NONE guibg=#123456 ctermbg=123"),
            ((Some("n"), Some("n"), None, HiAttr::Nothing),   0, "hi HL term=NONE guifg=#123456 ctermfg=123 guibg=#123456 ctermbg=123"),
            ((None, None, None, HiAttr::Bold),                0, "exe 'hi' 'HL term=NONE' s:bold_attr"),
            ((None, None, None, HiAttr::Italic),              0, "exe 'hi' 'HL term=NONE' s:italic_attr"),
            ((None, None, None, HiAttr::Underline),           0, "hi HL term=NONE gui=underline cterm=underline"),
            ((None, None, None, HiAttr::CommentItalic),       0, "exe 'hi' 'HL term=NONE' g:spring_night_italic_comments ? s:italic_attr : ''"),
            ((None, None, None, HiAttr::Undercurl),           0, "exe 'hi' 'HL term=NONE' s:undercurl_attr"),
            ((Some("c"), None, None, HiAttr::Nothing),        0, "exe 'hi' 'HL term=NONE' 'guifg='.s:c_gui 'ctermfg='.s:c_cterm"),
            ((None, Some("c"), None, HiAttr::Nothing),        0, "exe 'hi' 'HL term=NONE' 'guibg='.s:c_gui 'ctermbg='.s:c_cterm"),
            ((Some("c"), Some("c"), None, HiAttr::Underline), 0, "exe 'hi' 'HL term=NONE' 'guifg='.s:c_gui 'ctermfg='.s:c_cterm 'guibg='.s:c_gui 'ctermbg='.s:c_cterm 'gui=underline cterm=underline'"),
            ((None, None, Some("n"), HiAttr::Nothing),        0, "hi HL term=NONE guisp=#123456"),
            ((None, None, Some("n"), HiAttr::Undercurl),      0, "exe 'hi' 'HL term=NONE' 'guisp=#123456' s:undercurl_attr"),
            ((None, None, None, HiAttr::Nothing),             1, "    hi HL term=NONE"),
            ((None, None, None, HiAttr::Undercurl),           1, "    exe 'hi' 'HL term=NONE' s:undercurl_attr"),
        ];

        for ((fg, bg, sp, attr), indent, expected) in testcases {
            let cmd = HiCommand {
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
            let palette = Palette::from(m);
            let w = Colorscheme::new(&palette);
            let mut out = vec![];
            w.write_hi_command(&mut out, &cmd, indent).unwrap();
            assert_eq!(str::from_utf8(&out).unwrap(), format!("{}\n", expected));
        }

        // Edge case
        let palette = Palette::from(HashMap::new());
        let mut w = Colorscheme::new(&palette);
        w.highlights = &[];
        let mut out = vec![];
        w.write_highlights(&mut out).unwrap();
        assert_eq!(str::from_utf8(&out).unwrap(), "\n");
    }

    #[test]
    fn test_write_highlights() {
        const fn cmd() -> HiCommand {
            HiCommand {
                name: "HL",
                fg: None,
                bg: None,
                sp: None,
                attr: HiAttr::Nothing,
            }
        }

        let palette = Palette::from(HashMap::new());
        let mut w = Colorscheme::new(&palette);
        let fixed = &[Highlight::Fixed(cmd())];
        w.highlights = fixed;
        let mut out = vec![];
        w.write_highlights(&mut out).unwrap();
        assert_eq!(str::from_utf8(&out).unwrap(), "hi HL term=NONE\n\n");

        let dynamic = &[Highlight::Dynamic {
            gui: cmd(),
            term: cmd(),
        }];
        let palette = Palette::from(HashMap::new());
        let mut w = Colorscheme::new(&palette);
        w.highlights = dynamic;
        let mut out = vec![];
        w.write_highlights(&mut out).unwrap();
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
        let palette = Palette::from(m);
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
    fn test_highlight_uniqueness() {
        let palette = Palette::default();
        let w = Colorscheme::new(&palette);
        let mut seen = HashSet::new();
        for hl in w.highlights {
            let name = match hl {
                Highlight::Fixed(h) => h.name,
                Highlight::Dynamic { gui, term } => {
                    assert_eq!(gui.name, term.name);
                    gui.name
                }
            };
            assert!(seen.insert(name), "Duplicate highlight '{}'", name);
        }
    }

    #[test]
    fn test_term_colors() {
        let palette = Palette::default();
        let w = Colorscheme::new(&palette);
        for tc in &w.term_colors {
            assert!(
                palette.contains_key(tc),
                "Terminal color '{tc}' is not present in color names",
            );
        }
    }
}
