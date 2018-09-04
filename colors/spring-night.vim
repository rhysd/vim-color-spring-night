" spring-night : Calm-colored dark color scheme
"
" Author: rhysd <lin90162@yahoo.co.jp>
" License: MIT
"   Copyright (c) 2016 rhysd

" Optimization:
" `:set background=dark` has some side effects which takes a time.
" Avoid the side effects when the value is already 'dark'.
if &background !=# 'dark'
    set background=dark
endif

" Optimization:
" `:hi clear` takes a lot of time since it clears all highlights and set default
" highlights. This guard avoids `:hi clear` on loading vimrc since in almost
" all cases no additional highlight is set at start up. Almost all additional
" highlights are set by Vim plugins.
if !has('vim_starting')
    " Remove all existing highlighting and set the defaults.
    hi clear
endif

if exists('g:syntax_on')
    syntax reset
endif

let g:colors_name = 'spring-night'

if !exists('s:defs')
    let s:gui_running = has('gui_running')
    let s:true_colors = has('termguicolors') && &termguicolors
    let s:undercurl = s:gui_running ? 'undercurl' : 'underline'

    let g:spring_night_italic_comments = get(g:, 'spring_night_italic_comments', 0)
    let g:spring_night_kill_italic = get(g:, 'spring_night_kill_italic', 0)
    let g:spring_night_kill_bold = get(g:, 'spring_night_kill_bold', 0)
    let g:spring_night_high_contrast = get(g:, 'spring_night_high_contrast',
        \ !s:gui_running && s:true_colors ? ['cui'] : [])
    let g:spring_night_highlight_terminal = get(g:, 'spring_night_highlight_terminal', 1)
    let s:high_contrast =
        \ (s:gui_running && index(g:spring_night_high_contrast, 'gui') >= 0) ||
        \ (!s:gui_running && index(g:spring_night_high_contrast, 'cui') >= 0)

    " Define reusable color palette.
    let s:bg         = [s:high_contrast ? '#132132' : '#334152', 233]
    let s:bgemphasis = ['#3a4b5c', 235]
    let s:bgstrong   = ['#536273', 238]
    let s:fg         = ['#fffeeb', s:high_contrast ? 231 : 230]
    let s:hiddenfg   = ['#607080', 60]
    let s:weakfg     = ['#8d9eb2', 103]
    let s:weakerfg   = ['#788898', 102]
    let s:palepink   = ['#e7c6b7', 181]
    let s:yellow     = ['#f0eaaa', 229]
    let s:white      = ['#ffffff', 231]
    let s:purple     = ['#e7d5ff', 189]
    let s:gray       = ['#545f6e', 59]
    let s:light      = ['#646f7c', 60]
    let s:yaezakura  = ['#70495d', 95]
    let s:sakura     = ['#a9667a', 132]
    let s:orange     = ['#f0aa8a', 216]
    let s:green      = ['#a9dd9d', 150]
    let s:darkgreen  = ['#5f8770', 65]
    let s:skyblue    = ['#a8d2eb', 153]
    let s:gold       = ['#fedf81', 222]
    let s:darkgold   = ['#685800', 58]
    let s:red        = ['#fd8489', 210]
    let s:mildred    = ['#ab6560', 167]
    let s:crimson    = ['#ff6a6f', 203]
    let s:mikan      = ['#fb8965', 209]
    let s:darkblue   = ['#00091e', 235]
    let s:blue       = ['#7098e6', 69]
    let s:paleblue   = ['#98b8e6', 111]
    let s:lime       = ['#c9fd88', 149]
    let s:inu        = ['#ddbc96', 180]
    let s:NONE       = ['NONE', 'NONE']

    let s:NUMBER_TYPE = type(0)

    "      Name,                    Foreground,   Background,   Attribute(, Special)
    let s:defs = [
        \ ['Boolean',               s:red,        0,            0],
        \ ['Character',             s:green,      0,            0],
        \ ['ColorColumn',           0,            s:bgstrong,   0],
        \ ['Comment',               s:weakfg,     0,            g:spring_night_italic_comments ? 'italic' : 0],
        \ ['Conceal',               s:mikan,      s:bg,         0],
        \ ['Conditional',           s:skyblue,    0,            0],
        \ ['Constant',              s:red,        0,            0],
        \ ['Cursor',                s:bg,         s:fg,         0],
        \ ['CursorColumn',          0,            s:bgemphasis, 0],
        \ ['CursorLine',            0,            s:bgemphasis, 'NONE'],
        \ ['CursorLineNr',          s:purple,     s:bgstrong,   0],
        \ ['Define',                s:orange,     0,            0],
        \ ['Directory',             s:green,      0,            0],
        \ ['EndOfBuffer',           s:bgstrong,   0,            0],
        \ ['Error',                 s:red,        s:bgemphasis, 'bold'],
        \ ['ErrorMsg',              s:red,        s:bg,         'bold'],
        \ ['Float',                 s:red,        0,            0],
        \ ['FoldColumn',            s:purple,     s:bgemphasis, 0],
        \ ['Folded',                s:purple,     s:light,      0],
        \ ['Function',              s:orange,     0,            0],
        \ ['Identifier',            s:gold,       0,            'italic'],
        \ ['IncSearch',             s:NONE,       s:sakura,     'underline'],
        \ ['Keyword',               s:yellow,     0,            'bold'],
        \ ['Label',                 s:skyblue,    0,            0],
        \ ['LineNr',                s:weakerfg,   s:bgemphasis, 0],
        \ ['MatchParen',            s:bg,         s:sakura,     'underline'],
        \ ['ModeMsg',               s:gold,       0,            0],
        \ ['MoreMsg',               s:green,      0,            0],
        \ ['NonText',               s:light,      0,            0],
        \ ['Normal',                s:fg,         s:bg,         0],
        \ ['Number',                s:red,        0,            0],
        \ ['Operater',              s:orange,     0,            0],
        \ ['Pmenu',                 s:purple,     s:bgemphasis, 0],
        \ ['PmenuSbar',             s:gold,       s:bgstrong,   0],
        \ ['PmenuSel',              s:gold,       s:bgstrong,   0],
        \ ['PmenuThumb',            s:gold,       s:weakfg,     0],
        \ ['PreProc',               s:orange,     0,            0],
        \ ['Question',              s:skyblue,    0,            0],
        \ ['Search',                s:NONE,       s:yaezakura,  'underline'],
        \ ['SignColumn',            0,            s:bgemphasis, 0],
        \ ['Special',               s:yellow,     0,            'bold'],
        \ ['SpecialKey',            s:hiddenfg,   0,            0],
        \ ['SpellBad',              s:red,        0,            s:undercurl, s:red],
        \ ['SpellCap',              s:purple,     0,            s:undercurl, s:purple],
        \ ['SpellLocal',            s:red,        0,            s:undercurl, s:red],
        \ ['SpellRare',             s:yellow,     0,            s:undercurl, s:yellow],
        \ ['Statement',             s:skyblue,    0,            0],
        \ ['StatusLine',            s:fg,         s:bgstrong,   'bold'],
        \ ['StatusLineNC',          s:weakfg,     s:bgemphasis, 'NONE'],
        \ ['StatusLineTerm',        s:fg,         s:bgstrong,   'bold'],
        \ ['StatusLineTermNC',      s:weakfg,     s:bgemphasis, 'NONE'],
        \ ['StorageClass',          s:gold,       0,            'italic'],
        \ ['String',                s:green,      0,            0],
        \ ['TabLine',               s:weakfg,     s:bgstrong,   0],
        \ ['TabLineFill',           s:bgemphasis, 0,            0],
        \ ['TabLineSel',            s:gold,       s:bg,         'bold'],
        \ ['Tag',                   s:orange,     0,            0],
        \ ['Title',                 s:gold,       0,            'bold'],
        \ ['Todo',                  s:bg,         s:red,        'bold'],
        \ ['ToolbarButton',         s:gold,       s:bg,         'bold'],
        \ ['ToolbarLine',           s:weakfg,     s:bgstrong,   0],
        \ ['Type',                  s:gold,       0,            0],
        \ ['Underlined',            s:skyblue,    0,            'underline'],
        \ ['VertSplit',             s:bgemphasis, s:bg,         0],
        \ ['Visual',                0,            s:yaezakura,  0],
        \ ['WarningMsg',            s:mikan,      s:bgemphasis, 0],
        \ ['WildMenu',              0,            s:gold,       0],
        \ ['Terminal',              s:fg,         s:darkblue,   0],
        \ ['ALEWarningSign',        s:orange,     s:bgemphasis, 'bold'],
        \ ['ALEErrorSign',          s:bgemphasis, s:mildred,    'bold'],
        \ ['ALEInfoSign',           0,            s:light,      0],
        \ ['ALEError',              0,            s:mildred,    0],
        \ ['ALEWarning',            0,            s:darkgold,   0],
        \ ['CleverFChar',           s:bg,         s:red,        0],
        \ ['DirvishArg',            s:yellow,     0,            'bold'],
        \ ['EasyMotionTarget',      s:red,        0,            'bold'],
        \ ['EasyMotionShade',       s:weakfg,     s:bg,         0],
        \ s:gui_running ?
        \   ['EasyMotionIncCursor', s:bg,         s:fg,         0] :
        \   ['EasyMotionIncCursor', 0,            0,            'reverse'],
        \ ['GitGutterAdd',          s:green,      s:bgemphasis, 0],
        \ ['GitGutterChange',       s:yellow,     s:bgemphasis, 0],
        \ ['GitGutterChangeDelete', s:gold,       s:bgemphasis, 0],
        \ ['GitGutterDelete',       s:red,        s:bgemphasis, 0],
        \ ['HighlightedyankRegion', 0,            s:bgemphasis, 0],
        \ ['cmakeArguments',        s:yellow,     0,            0],
        \ ['cmakeOperators',        s:red,        0,            0],
        \ ['DiffAdd',               0,            s:darkgreen,  'bold'],
        \ ['DiffChange',            0,            s:darkgold,   'bold'],
        \ ['DiffDelete',            s:fg,         s:mildred,    'bold'],
        \ ['DiffText',              0,            s:bg,         0],
        \ ['diffAdded',             s:green,      0,            0],
        \ ['diffFile',              s:yellow,     0,            0],
        \ ['diffIndexLine',         s:gold,       0,            0],
        \ ['diffNewFile',           s:yellow,     0,            0],
        \ ['diffRemoved',           s:red,        0,            0],
        \ ['gitCommitOverflow',     0,            s:red,        0],
        \ ['gitCommitSummary',      s:yellow,     0,            0],
        \ ['gitCommitSelectedFile', s:skyblue,    0,            0],
        \ ['gitconfigSection',      s:skyblue,    0,            'bold'],
        \ ['goBuiltins',            s:red,        0,            0],
        \ ['helpExample',           s:skyblue,    0,            0],
        \ ['htmlBold',              0,            s:bgemphasis, 0],
        \ ['htmlLinkText',          s:skyblue,    0,            0],
        \ ['htmlTagName',           s:orange,     0,            0],
        \ ['javaScriptBraces',      s:fg,         0,            0],
        \ ['makeCommands',          s:yellow,     0,            0],
        \ ['markdownCode',          s:yellow,     0,            0],
        \ ['markdownUrl',           s:weakfg,     0,            0],
        \ ['ocamlConstructor',      s:gold,       0,            0],
        \ ['ocamlKeyChar',          s:skyblue,    0,            0],
        \ ['ocamlKeyword',          s:gold   ,    0,            0],
        \ ['ocamlFunDef',           s:skyblue,    0,            0],
        \ ['plantumlColonLine',     s:skyblue,    0,            0],
        \ ['pythonBuiltin',         s:red,        0,            0],
        \ ['qfFileName',            s:gold,       0,            0],
        \ ['qfLineNr',              s:skyblue,    0,            0],
        \ ['rstEmphasis',           0,            s:bgemphasis, 'italic'],
        \ ['rstStrongEmphasis',     0,            s:bgstrong,   'bold'],
        \ ['rubyFunction',          s:yellow,     0,            0],
        \ ['rubyIdentifier',        s:yellow,     0,            0],
        \ ['rustEnumVariant',       s:gold,       0,            0],
        \ ['rustFuncCall',          s:fg,         0,            0],
        \ ['rustCommentLineDoc',    s:palepink,   0,            0],
        \ ['typescriptBraces',      s:fg,         0,            0],
        \ ['vimfilerColumn__SizeLine', s:weakfg,  0,            0],
        \ ['vimfilerClosedFile',    s:green,      0,            0],
        \ ['vimCommand',            s:skyblue,    0,            0],
        \ ['wastListDelimiter',     s:fg,         0,            0],
        \ ['wastInstGeneral',       s:yellow,     0,            0],
        \ ['wastInstWithType',      s:yellow,     0,            0],
        \ ['wastUnnamedVar'  ,      s:purple,     0,            0],
        \ ['zshDelimiter',          s:skyblue,    0,            0],
        \ ['zshPrecommand',         s:red,        0,            0],
    \ ]
    " TODO: ['QuickFixLine',     0,            0,       0],

    "    0: Black (black)
    "    1: Red (dark red)
    "    2: Green (dark green)
    "    3: Yellow (brown)
    "    4: Blue (dark blue)
    "    5: Magenta (dark magenta)
    "    6: Cyan (dark cyan)
    "    7: White (light grey)
    "    8: Bright Black (dark grey)
    "    9: Bright Red (red)
    "   10: Bright Green (green)
    "   11: Bright Yellow (yellow)
    "   12: Bright Blue (blue)
    "   13: Bright Magenta (magenta)
    "   14: Bright Cyan (cyan)
    "   15: Bright White (white)
    let s:term_16_colors = [
        \   s:bg,
        \   s:crimson,
        \   s:green,
        \   s:gold,
        \   s:blue,
        \   s:purple,
        \   s:skyblue,
        \   s:fg,
        \   s:bgemphasis,
        \   s:red,
        \   s:lime,
        \   s:yellow,
        \   s:paleblue,
        \   s:palepink,
        \   s:skyblue,
        \   s:white,
        \ ]
    let s:should_setup_terminal = has('nvim') || (s:gui_running || s:true_colors) && exists('*term_setansicolors')
    if !s:should_setup_terminal
        " On Terminal-Normal mode, foreground and background colors of the
        " colorscheme is used. But some colors (especially blue) are not working
        " well with this colorscheme. So specify Terminal highlight group to
        " improve the visibility.
        let s:defs += [['Terminal', s:fg, s:darkblue, 0]]
    endif
endif " if !exists('s:defs')

function! s:hi(defs) abort
    for def in a:defs
        let name = def[0]
        let fg = def[1]
        let bg = def[2]
        let attr = def[3]

        let has_fg = type(fg) != s:NUMBER_TYPE
        let has_bg = type(bg) != s:NUMBER_TYPE

        let guifg   = has_fg ? ('guifg=' . fg[0]) : ''
        let guibg   = has_bg ? ('guibg=' . bg[0]) : ''
        let ctermfg = has_fg ? ('ctermfg=' . fg[1]) : ''
        let ctermbg = has_bg ? ('ctermbg=' . bg[1]) : ''

        let is_italic = attr ==# 'italic'
        let is_bold = attr ==# 'bold'
        if type(attr) != s:NUMBER_TYPE &&
            \ !(g:spring_night_kill_italic && is_italic) &&
            \ !(g:spring_night_kill_bold && is_bold)
            let deco =  'gui=' . attr
            if !is_italic
                let deco .= ' cterm=' . attr
            endif
        else
            let deco = ''
        endif

        if len(def) > 4
            let guisp = 'guisp=' . def[4][0]
        else
            let guisp = ''
        endif

        " XXX: term=NONE is a workaround for unintentional default values
        exe 'hi' name 'term=NONE' guifg guibg ctermfg ctermbg deco guisp

        unlet fg
        unlet bg
        unlet attr
    endfor
endfunction

function! s:setup_term_ansi_colors() abort
    " Neovim or recent Vim terminal colors configuration
    " See :help terminal-configuration or :help g:terminal_ansi_colors
    if has('nvim')
        let gui_or_term = s:gui_running || s:true_colors ? 0 : 1
        for i in range(len(s:term_16_colors))
            let g:terminal_color_{i} = s:term_16_colors[i][gui_or_term]
        endfor
        " TODO: Maybe TerminalCursor and TerminalCursorNC need to be optimized
    else
        " if vim
        let g:terminal_ansi_colors = map(s:term_16_colors, 'v:val[0]')
    endif
endfunction

call s:hi(s:defs)

" Terminal color configuration
if g:spring_night_highlight_terminal && s:should_setup_terminal
    call s:setup_term_ansi_colors()
endif
