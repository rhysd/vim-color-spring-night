#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;
use std::{env, fs, io};

#[derive(Debug)]
struct Color<T: Display>(T, Option<T>);

fn color<T: Display>(c: T) -> Color<T> {
    Color(c, None)
}

fn contrast_color<T: Display>(high: T, low: T) -> Color<T> {
    Color(high, Some(low))
}

struct Colors<T: Display> {
    defs: HashMap<&'static str, Color<T>>,
    gui: bool,
}

lazy_static! {
    static ref GUI_COLORS: Colors<&'static str> = {
        let mut m = HashMap::new();
        m.insert("bg", contrast_color("#132132", "#334152"));
        m.insert("bgemphasis", color("#3a4b5c"));
        m.insert("bgstrong", color("#536273"));
        m.insert("fg", color("#fffeeb"));
        m.insert("hiddenfg", color("#607080"));
        m.insert("weakfg", color("#8d9eb2"));
        m.insert("weakerfg", color("#788898"));
        m.insert("palepink", color("#e7c6b7"));
        m.insert("yellow", color("#f0eaaa"));
        m.insert("white", color("#ffffff"));
        m.insert("purple", color("#e7d5ff"));
        m.insert("gray", color("#545f6e"));
        m.insert("light", color("#646f7c"));
        m.insert("yaezakura", color("#70495d"));
        m.insert("sakura", color("#a9667a"));
        m.insert("orange", color("#f0aa8a"));
        m.insert("green", color("#a9dd9d"));
        m.insert("darkgreen", color("#5f8770"));
        m.insert("skyblue", color("#a8d2eb"));
        m.insert("gold", color("#fedf81"));
        m.insert("darkgold", color("#685800"));
        m.insert("red", color("#fd8489"));
        m.insert("mildred", color("#ab6560"));
        m.insert("crimson", color("#ff6a6f"));
        m.insert("mikan", color("#fb8965"));
        m.insert("darkblue", color("#00091e"));
        m.insert("blue", color("#7098e6"));
        m.insert("paleblue", color("#98b8e6"));
        m.insert("lime", color("#c9fd88"));
        m.insert("inu", color("#ddbc96"));
        Colors { defs: m, gui: true }
    };
    static ref TERM_COLORS: Colors<u8> = {
        let mut m = HashMap::new();
        m.insert("bg", color(233));
        m.insert("bgemphasis", color(235));
        m.insert("bgstrong", color(238));
        m.insert("fg", contrast_color(231, 230));
        m.insert("hiddenfg", color(60));
        m.insert("weakfg", color(103));
        m.insert("weakerfg", color(102));
        m.insert("palepink", color(181));
        m.insert("yellow", color(229));
        m.insert("white", color(231));
        m.insert("purple", color(189));
        m.insert("gray", color(59));
        m.insert("light", color(60));
        m.insert("yaezakura", color(95));
        m.insert("sakura", color(132));
        m.insert("orange", color(216));
        m.insert("green", color(150));
        m.insert("darkgreen", color(65));
        m.insert("skyblue", color(153));
        m.insert("gold", color(222));
        m.insert("darkgold", color(58));
        m.insert("red", color(210));
        m.insert("mildred", color(167));
        m.insert("crimson", color(203));
        m.insert("mikan", color(209));
        m.insert("darkblue", color(235));
        m.insert("blue", color(69));
        m.insert("paleblue", color(111));
        m.insert("lime", color(149));
        m.insert("inu", color(180));
        Colors {
            defs: m,
            gui: false,
        }
    };
}

type ColorName = Option<&'static str>;

#[derive(Debug)]
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

use HowToHighlight::Always;
use HowToHighlight::Switch;

macro_rules! fgbg {
    ($name:expr,NONE,NONE, $attr:ident) => {
        Highlight {
            name: $name,
            fg: None,
            bg: None,
            sp: None,
            attr: HighlightAttr::$attr,
        }
    };
    ($name:expr,NONE, $bg:ident, $attr:ident) => {
        Highlight {
            name: $name,
            fg: None,
            bg: Some(stringify!($bg)),
            sp: None,
            attr: HighlightAttr::$attr,
        }
    };
    ($name:expr, $fg:ident,NONE, $attr:ident) => {
        Highlight {
            name: $name,
            fg: Some(stringify!($fg)),
            bg: None,
            sp: None,
            attr: HighlightAttr::$attr,
        }
    };
    ($name:expr, $fg:ident, $bg:ident, $attr:ident) => {
        Highlight {
            name: $name,
            fg: Some(stringify!($fg)),
            bg: Some(stringify!($bg)),
            sp: None,
            attr: HighlightAttr::$attr,
        }
    };
}

macro_rules! fgbgsp {
    ($name:expr, $fg:ident,NONE, $sp:ident, $attr:ident) => {
        Highlight {
            name: $name,
            fg: Some(stringify!($fg)),
            bg: None,
            sp: Some(stringify!($sp)),
            attr: HighlightAttr::$attr,
        }
    };
}

#[cfg_attr(rustfmt, rustfmt_skip)]
static HIGHLIGHTS: &'static [HowToHighlight] = &[
    // Normal colors
    Always(fgbg!("Boolean",               red,        NONE,         Nothing)),
    Always(fgbg!("Character",             green,      NONE,         Nothing)),
    Always(fgbg!("ColorColumn",           NONE,       bgstrong,     Nothing)),
    Always(fgbg!("Comment",               weakfg,     NONE,         CommentItalic)),
    Always(fgbg!("Conceal",               mikan,      bg,           Nothing)),
    Always(fgbg!("Conditional",           skyblue,    NONE,         Nothing)),
    Always(fgbg!("Constant",              red,        NONE,         Nothing)),
    Always(fgbg!("Cursor",                bg,         fg,           Nothing)),
    Always(fgbg!("CursorColumn",          NONE,       bgemphasis,   Nothing)),
    Always(fgbg!("CursorLine",            NONE,       bgemphasis,   None)),
    Always(fgbg!("CursorLineNr",          purple,     bgstrong,     Nothing)),
    Always(fgbg!("Define",                orange,     NONE,         Nothing)),
    Always(fgbg!("Directory",             green,      NONE,         Nothing)),
    Always(fgbg!("EndOfBuffer",           bgstrong,   NONE,         Nothing)),
    Always(fgbg!("Error",                 red,        bgemphasis,   Bold)),
    Always(fgbg!("ErrorMsg",              red,        bg,           Bold)),
    Always(fgbg!("Float",                 red,        NONE,         Nothing)),
    Always(fgbg!("FoldColumn",            purple,     bgemphasis,   Nothing)),
    Always(fgbg!("Folded",                purple,     light,        Nothing)),
    Always(fgbg!("Function",              orange,     NONE,         Nothing)),
    Always(fgbg!("Identifier",            gold,       NONE,         Italic)),
    Always(fgbg!("IncSearch",             NONE,       sakura,       Underline)),
    Always(fgbg!("Keyword",               yellow,     NONE,         Bold)),
    Always(fgbg!("Label",                 skyblue,    NONE,         Nothing)),
    Always(fgbg!("LineNr",                weakerfg,   bgemphasis,   Nothing)),
    Always(fgbg!("MatchParen",            bg,         sakura,       Underline)),
    Always(fgbg!("ModeMsg",               gold,       NONE,         Nothing)),
    Always(fgbg!("MoreMsg",               green,      NONE,         Nothing)),
    Always(fgbg!("NonText",               light,      NONE,         Nothing)),
    Always(fgbg!("Normal",                fg,         bg,           Nothing)),
    Always(fgbg!("Number",                red,        NONE,         Nothing)),
    Always(fgbg!("Operater",              orange,     NONE,         Nothing)),
    Always(fgbg!("Pmenu",                 purple,     bgemphasis,   Nothing)),
    Always(fgbg!("PmenuSbar",             gold,       bgstrong,     Nothing)),
    Always(fgbg!("PmenuSel",              gold,       bgstrong,     Nothing)),
    Always(fgbg!("PmenuThumb",            gold,       weakfg,       Nothing)),
    Always(fgbg!("PreProc",               orange,     NONE,         Nothing)),
    Always(fgbg!("Question",              skyblue,    NONE,         Nothing)),
    Always(fgbg!("Search",                NONE,       yaezakura,    Underline)),
    Always(fgbg!("SignColumn",            NONE,       bgemphasis,   Nothing)),
    Always(fgbg!("Special",               yellow,     NONE,         Bold)),
    Always(fgbg!("SpecialKey",            hiddenfg,   NONE,         Nothing)),
    Always(fgbgsp!("SpellBad",            red,        NONE, red,    Undercurl)),
    Always(fgbgsp!("SpellCap",            purple,     NONE, purple, Undercurl)),
    Always(fgbgsp!("SpellLocal",          red,        NONE, red,    Undercurl)),
    Always(fgbgsp!("SpellRare",           yellow,     NONE, yellow, Undercurl)),
    Always(fgbg!("Statement",             skyblue,    NONE,         Nothing)),
    Always(fgbg!("StatusLine",            fg,         bgstrong,     Bold)),
    Always(fgbg!("StatusLineNC",          weakfg,     bgemphasis,   None)),
    Always(fgbg!("StatusLineTerm",        fg,         bgstrong,     Bold)),
    Always(fgbg!("StatusLineTermNC",      weakfg,     bgemphasis,   None)),
    Always(fgbg!("StorageClass",          gold,       NONE,         Italic)),
    Always(fgbg!("String",                green,      NONE,         Nothing)),
    Always(fgbg!("TabLine",               weakfg,     bgstrong,     Nothing)),
    Always(fgbg!("TabLineFill",           bgemphasis, NONE,         Nothing)),
    Always(fgbg!("TabLineSel",            gold,       bg,           Bold)),
    Always(fgbg!("Tag",                   orange,     NONE,         Nothing)),
    Always(fgbg!("Title",                 gold,       NONE,         Bold)),
    Always(fgbg!("Todo",                  bg,         red,          Bold)),
    Always(fgbg!("ToolbarButton",         gold,       bg,           Bold)),
    Always(fgbg!("ToolbarLine",           weakfg,     bgstrong,     Nothing)),
    Always(fgbg!("Type",                  gold,       NONE,         Nothing)),
    Always(fgbg!("Underlined",            skyblue,    NONE,         Underline)),
    Always(fgbg!("VertSplit",             bgemphasis, bg,           Nothing)),
    Always(fgbg!("Visual",                NONE,       yaezakura,    Nothing)),
    Always(fgbg!("WarningMsg",            mikan,      bgemphasis,   Nothing)),
    Always(fgbg!("WildMenu",              NONE,       gold,         Nothing)),

    // File type specific
    //
    // Markdown is highlighted with HTML highlights in gVim but link text doesn't
    // have a color. So define it here.
    Always(fgbg!("cmakeArguments",        yellow,     NONE,         Nothing)),
    Always(fgbg!("cmakeOperators",        red,        NONE,         Nothing)),
    Always(fgbg!("DiffAdd",               NONE,       darkgreen,    Bold)),
    Always(fgbg!("DiffChange",            NONE,       darkgold,     Bold)),
    Always(fgbg!("DiffDelete",            fg,         mildred,      Bold)),
    Always(fgbg!("DiffText",              NONE,       bg,           Nothing)),
    Always(fgbg!("diffAdded",             green,      NONE,         Nothing)),
    Always(fgbg!("diffFile",              yellow,     NONE,         Nothing)),
    Always(fgbg!("diffIndexLine",         gold,       NONE,         Nothing)),
    Always(fgbg!("diffNewFile",           yellow,     NONE,         Nothing)),
    Always(fgbg!("diffRemoved",           red,        NONE,         Nothing)),
    Always(fgbg!("gitCommitOverflow",     NONE,       red,          Nothing)),
    Always(fgbg!("gitCommitSummary",      yellow,     NONE,         Nothing)),
    Always(fgbg!("gitCommitSelectedFile", skyblue,    NONE,         Nothing)),
    Always(fgbg!("gitconfigSection",      skyblue,    NONE,         Bold)),
    Always(fgbg!("goBuiltins",            red,        NONE,         Nothing)),
    Always(fgbg!("helpExample",           skyblue,    NONE,         Nothing)),
    Always(fgbg!("htmlBold",              NONE,       bgemphasis,   Nothing)),
    Always(fgbg!("htmlLinkText",          skyblue,    NONE,         Nothing)),
    Always(fgbg!("htmlTagName",           orange,     NONE,         Nothing)),
    Always(fgbg!("javaScriptBraces",      fg,         NONE,         Nothing)),
    Always(fgbg!("makeCommands",          yellow,     NONE,         Nothing)),
    Always(fgbg!("markdownCode",          yellow,     NONE,         Nothing)),
    Always(fgbg!("markdownUrl",           weakfg,     NONE,         Nothing)),
    Always(fgbg!("ocamlConstructor",      gold,       NONE,         Nothing)),
    Always(fgbg!("ocamlKeyChar",          skyblue,    NONE,         Nothing)),
    Always(fgbg!("ocamlKeyword",          gold   ,    NONE,         Nothing)),
    Always(fgbg!("ocamlFunDef",           skyblue,    NONE,         Nothing)),
    Always(fgbg!("plantumlColonLine",     skyblue,    NONE,         Nothing)),
    Always(fgbg!("pythonBuiltin",         red,        NONE,         Nothing)),
    Always(fgbg!("qfFileName",            gold,       NONE,         Nothing)),
    Always(fgbg!("qfLineNr",              skyblue,    NONE,         Nothing)),
    Always(fgbg!("rstEmphasis",           NONE,       bgemphasis,   Italic)),
    Always(fgbg!("rstStrongEmphasis",     NONE,       bgstrong,     Bold)),
    Always(fgbg!("rubyFunction",          yellow,     NONE,         Nothing)),
    Always(fgbg!("rubyIdentifier",        yellow,     NONE,         Nothing)),
    Always(fgbg!("rustEnumVariant",       gold,       NONE,         Nothing)),
    Always(fgbg!("rustFuncCall",          fg,         NONE,         Nothing)),
    Always(fgbg!("rustCommentLineDoc",    palepink,   NONE,         Nothing)),
    Always(fgbg!("typescriptBraces",      fg,         NONE,         Nothing)),
    Always(fgbg!("vimfilerColumn__SizeLine", weakfg,  NONE,         Nothing)),
    Always(fgbg!("vimfilerClosedFile",    green,      NONE,         Nothing)),
    Always(fgbg!("vimCommand",            skyblue,    NONE,         Nothing)),
    Always(fgbg!("wastListDelimiter",     fg,         NONE,         Nothing)),
    Always(fgbg!("wastInstGeneral",       yellow,     NONE,         Nothing)),
    Always(fgbg!("wastInstWithType",      yellow,     NONE,         Nothing)),
    Always(fgbg!("wastUnnamedVar"  ,      purple,     NONE,         Nothing)),
    Always(fgbg!("zshDelimiter",          skyblue,    NONE,         Nothing)),
    Always(fgbg!("zshPrecommand",         red,        NONE,         Nothing)),

    // Plugin specific
    //
    // Some plugins introduce its own highlight definitions. Adjust them for
    // working fine with this colorscheme.
    Always(fgbg!("ALEWarningSign",        orange,     bgemphasis,   Bold)),
    Always(fgbg!("ALEErrorSign",          bgemphasis, mildred,      Bold)),
    Always(fgbg!("ALEInfoSign",           NONE,       light,        Nothing)),
    Always(fgbg!("ALEError",              NONE,       mildred,      Nothing)),
    Always(fgbg!("ALEWarning",            NONE,       darkgold,     Nothing)),
    Always(fgbg!("CleverFChar",           bg,         red,          Nothing)),
    Always(fgbg!("DirvishArg",            yellow,     NONE,         Bold)),
    Always(fgbg!("EasyMotionTarget",      red,        NONE,         Bold)),
    Always(fgbg!("EasyMotionShade",       weakfg,     bg,           Nothing)),
    Always(fgbg!("GitGutterAdd",          green,      bgemphasis,   Nothing)),
    Always(fgbg!("GitGutterChange",       yellow,     bgemphasis,   Nothing)),
    Always(fgbg!("GitGutterChangeDelete", gold,       bgemphasis,   Nothing)),
    Always(fgbg!("GitGutterDelete",       red,        bgemphasis,   Nothing)),
    Always(fgbg!("HighlightedyankRegion", NONE,       bgemphasis,   Nothing)),
    Switch(
        fgbg!("EasyMotionIncCursor",      bg,         fg,           Nothing),
        fgbg!("EasyMotionIncCursor",      NONE,       NONE,         Reverse),
    ),
];

fn write_header<O: io::Write>(out: &mut O, name: &'static str) -> io::Result<()> {
    write!(
        out,
        r#"" {name}: Calm-colored dark color scheme
"
" Author: rhysd <lin90162@yahoo.co.jp>
" License: MIT
"   Copyright (c) 2016 rhysd
"
" THIS FILE WAS GENERATED BY SCRIPT. PLEASE DO NOT MODIFY DIRECTLY!

set background=dark
if v:version > 580
    " no guarantees for version 5.8 and below, but this makes it stop
    " complaining
    hi clear
    if exists('g:syntax_on')
        syntax reset
    endif
endif
let g:colors_name = '{name}'

let s:gui_running = has('gui_running')
let s:true_colors = has('termguicolors') && &termguicolors
let s:undercurl = s:gui_running ? 'undercurl' : 'underline'

let g:spring_night_italic_comments = get(g:, 'spring_night_italic_comments', 0)
let g:spring_night_kill_italic = get(g:, 'spring_night_kill_italic', 0)
let g:spring_night_kill_bold = get(g:, 'spring_night_kill_bold', 0)
let g:spring_night_high_contrast = get(g:, 'spring_night_high_contrast', !s:gui_running && s:true_colors)
let g:spring_night_highlight_terminal = get(g:, 'spring_night_highlight_terminal', 1)

"#,
        name = name
    )
}

fn build_highlight_item<T: Display>(
    name: &'static str,
    colors: &Colors<T>,
    gui: &'static str,
    cterm: &'static str,
) -> String {
    let item_name = if colors.gui { gui } else { cterm };
    match colors.defs.get(name).unwrap() {
        Color(c, None) => format!("{}={}", item_name, c),
        Color(high, Some(low)) => format!(
            "'{}=' . g:spring_night_high_contrast ? '{}' : '{}'",
            item_name, high, low
        ),
    }
}

fn write_highlight<T: Display, O: io::Write>(
    out: &mut O,
    highlight: &Highlight,
    colors: &Colors<T>,
    indent: u32,
) -> io::Result<()> {
    let mut use_execute = false;
    let mut args = vec![highlight.name.to_string(), "term=NONE".to_string()];

    if let Some(ref name) = highlight.fg {
        let item = build_highlight_item(name, colors, "guifg", "ctermfg");
        if item.starts_with('\'') {
            use_execute = true;
        }
        args.push(item);
    }

    if let Some(ref name) = highlight.bg {
        let item = build_highlight_item(name, colors, "guibg", "ctermbg");
        if item.starts_with('\'') {
            use_execute = true;
        }
        args.push(item);
    }

    if let Some(ref name) = highlight.sp {
        if colors.gui {
            if let Color(c, None) = colors.defs.get(name).unwrap() {
                args.push(format!("guisp={}", c));
            } else {
                unreachable!();
            }
        }
        // Note: ctermsp does not exist
    }

    {
        let item_name = if colors.gui { "gui" } else { "cterm" };
        match highlight.attr {
            HighlightAttr::Nothing => { /* Do nothing */ }
            HighlightAttr::Bold => {
                use_execute = true;
                args.push(format!(
                    "!g:spring_night_kill_bold ? '{}=bold' : ''",
                    item_name
                ));
            }
            HighlightAttr::Italic => if colors.gui {
                use_execute = true;
                args.push("!g:spring_night_kill_italic ? 'gui=italic' : ''".to_string());
            },
            HighlightAttr::Underline => {
                args.push(format!("{}=underline", item_name));
            }
            HighlightAttr::Reverse => {
                args.push(format!("{}=reverse", item_name));
            }
            HighlightAttr::None => {
                args.push(format!("{}=NONE", item_name));
            }
            HighlightAttr::CommentItalic => {
                use_execute = true;
                args.push(format!("g:spring_night_italic_comments && !g:spring_night_kill_italic ? '{}=italic' : ''", item_name));
            }
            HighlightAttr::Undercurl => {
                use_execute = true;
                args.push(format!("'{}='.s:undercurl", item_name));
            }
        }
    }

    let indent = match indent {
        0u32 => "",
        1u32 => "    ",
        _ => unreachable!(),
    };

    if use_execute {
        for arg in args.iter_mut() {
            if !arg.starts_with('\'') && !arg.ends_with('\'') {
                *arg = format!("'{}'", arg);
            }
        }
        writeln!(out, "{}exe 'hi' {}", indent, args.join(" "))
    } else {
        writeln!(out, "{}hi {}", indent, args.join(" "))
    }
}

fn write_highlights<T: Display, O: io::Write>(out: &mut O, colors: &Colors<T>) -> io::Result<()> {
    for highlight in HIGHLIGHTS {
        match highlight {
            Always(ref hl) => write_highlight(out, hl, colors, 0u32)?,
            Switch(ref gui, ref term) => {
                writeln!(out, "if s:gui_running")?;
                write_highlight(out, gui, colors, 1u32)?;
                writeln!(out, "else")?;
                write_highlight(out, term, colors, 1u32)?;
                writeln!(out, "endif")?;
            }
        }
    }
    writeln!(out, "")
}

fn write_term_colors<T: Display, O: io::Write>(out: &mut O, colors: &Colors<T>) -> io::Result<()> {
    let term_color_names = [
        "bg",
        "crimson",
        "green",
        "gold",
        "blue",
        "purple",
        "skyblue",
        "fg",
        "bgemphasis",
        "red",
        "lime",
        "yellow",
        "paleblue",
        "palepink",
        "skyblue",
        "white",
    ];
    writeln!(out, "if has('nvim')")?;
    for (index, name) in term_color_names.iter().enumerate() {
        if colors.gui {
            writeln!(
                out,
                "    let g:terminal_color_{} = '{}'",
                index,
                colors.defs.get(name).unwrap().0
            )?;
        } else {
            writeln!(
                out,
                "    let g:terminal_color_{} = {}",
                index,
                colors.defs.get(name).unwrap().0
            )?;
        }
    }
    writeln!(out, "else")?;
    writeln!(out, "    let g:terminal_ansi_colors = [")?;
    for name in term_color_names.iter() {
        writeln!(out, "\\       '{}',", colors.defs.get(name).unwrap().0)?;
    }
    writeln!(out, "\\   ]")?;
    writeln!(out, "endif")
}

fn write_one<T: Display>(
    mut dir: PathBuf,
    name: &'static str,
    colors: &Colors<T>,
) -> io::Result<()> {
    dir.push(format!("{}.vim", name));
    let mut file = fs::File::create(dir.as_path())?;
    write_header(&mut file, name)?;
    write_highlights(&mut file, colors)?;
    write_term_colors(&mut file, colors)
}

fn write_all(dir: String) -> io::Result<()> {
    let dir = fs::canonicalize(&dir)?;
    write_one(dir.clone(), "spring-night", &GUI_COLORS)?;
    write_one(dir, "spring-night-256", &TERM_COLORS)
}

fn main() -> io::Result<()> {
    let dir = match env::args().skip(1).next() {
        Some(ref arg) if arg.as_str() == "--help" => {
            eprintln!("Usage: gen-color-sprint-night [directory]");
            std::process::exit(0);
        }
        Some(arg) => arg,
        None => env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(),
    };
    write_all(dir)
}
