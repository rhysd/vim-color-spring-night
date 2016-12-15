" spring-night : Calm-colored dark color scheme
"
" Author: rhysd <lin90162@yahoo.co.jp>
" License: MIT

set background=dark
if version > 580
    " no guarantees for version 5.8 and below, but this makes it stop
    " complaining
    hi clear
    if exists("syntax_on")
        syntax reset
    endif
endif
let g:colors_name = "spring-night"

let g:spring_night_kill_italic = get(g:, 'spring_night_kill_italic', 0)

" Define reusable colorvariables.
let s:bg         = ['#334152', 233]
let s:fg         = ['#fffeee', 230]
let s:palepink   = ['#ebeadb', 224]
let s:yellow     = ['#fffaaa', 229]
let s:purple     = ['#c2c1b5', 145]
let s:bgemphasis = ['#435060', 235]
let s:gray       = ['#545f6e', 59]
let s:light      = ['#646f7c', 60]
let s:sakura     = ['#a9667a', 132]
let s:palesakura = ['#b9768a', 175]
let s:orange     = ['#fda08b', 216]
let s:lightgreen = ['#a9de9c', 150]
let s:weakfg     = ['#8090a0', 103]
let s:green      = ['#a9dd9d', 150]
let s:skyblue    = ['#a8d2eb', 153]
let s:gold       = ['#fedf81', 222]
let s:red        = ['#fd8489', 210]
let s:mikan      = ['#fb8965', 209]

let s:NUMBER = type(0)

" function! s:hi(name, fg, bg, ...)
function! s:hi(name, fg, bg, attr) abort
    let fg = type(a:fg) != s:NUMBER
    let bg = type(a:bg) != s:NUMBER

    let guifg   = fg ? ('guifg=' . a:fg[0]) : ''
    let guibg   = bg ? ('guibg=' . a:bg[0]) : ''
    let ctermfg = fg ? ('ctermfg=' . a:fg[1]) : ''
    let ctermbg = bg ? ('ctermbg=' . a:bg[1]) : ''

    if type(a:attr) != s:NUMBER && !(g:spring_night_kill_italic && a:attr ==# 'italic')
        let attr =  'gui=' . a:attr

        if a:attr !=# 'italic'
            let attr .= ' cterm=' . a:attr
        endif
    else
        let attr = ''
    endif

    " XXX: term=NONE is a workaround for unintentional default values
    exe 'hi' a:name 'term=NONE' guifg guibg ctermfg ctermbg attr
endfunction

call s:hi('Normal',       s:fg,         s:bg,         0)
call s:hi('Cursor',       s:bg,         s:fg,         0)
call s:hi('Cursorline',   0,            s:bgemphasis, 0)
call s:hi('CursorColumn', 0,            s:bgemphasis, 0)
call s:hi('ColorColumn',  0,            s:bgemphasis, 0)
call s:hi('LineNr',       s:purple,     s:bgemphasis, 0)
call s:hi('VertSplit',    s:gray,       0,            0)
call s:hi('MatchParen',   s:bg,         s:palesakura, 'underline')
call s:hi('StatusLine',   s:palepink,   s:gray,       'bold')
call s:hi('Pmenu',        s:fg,         s:bgemphasis, 0)
call s:hi('PmenuSel',     0,            s:gray,       0)
call s:hi('Directory',    s:lightgreen, 0,            0)
call s:hi('Boolean',      s:red,        0,            0)
call s:hi('Character',    s:lightgreen, 0,            0)
call s:hi('Comment',      s:weakfg,     0,            0)
call s:hi('Conditional',  s:orange,     0,            0)
call s:hi('Constant',     s:red,        0,            0)
call s:hi('Define',       s:orange,     0,            0)
call s:hi('DiffAdd',      s:bg,         s:green,      'bold')
call s:hi('DiffDelete',   s:bg,         s:red,        'bold')
call s:hi('DiffChange',   s:bg,         s:gold,       'bold')
call s:hi('DiffText',     s:fg,         s:bg,         0)
call s:hi('ErrorMsg',     s:bg,         s:red,        'bold')
call s:hi('Error',        s:red,        s:bg,         'bold')
call s:hi('WarningMsg',   s:bg,         s:mikan,      0)
call s:hi('Float',        s:red,        0,            0)
call s:hi('Function',     s:orange,     0,            0)
call s:hi('Identifier',   s:gold,       0,            'italic')
call s:hi('Keyword',      s:yellow,     0,            'bold')
call s:hi('Label',        s:gold,       0,            0)
call s:hi('NonText',      s:light,      0,            0)
call s:hi('Number',       s:red,        0,            0)
call s:hi('Operater',     s:orange,     0,            0)
call s:hi('PreProc',      s:orange,     0,            0)
call s:hi('Special',      s:yellow,     s:bgemphasis, 0)
call s:hi('SpecialKey',   s:palepink,   s:bgemphasis, 0)
call s:hi('Statement',    s:skyblue,    0,            0)
call s:hi('StorageClass', s:skyblue,    0,            'italic')
call s:hi('String',       s:green,      0,            0)
call s:hi('Tag',          s:orange,     0,            0)
call s:hi('Title',        s:fg,         0,            'bold')
call s:hi('Todo',         s:bg,         s:red,        'bold')
call s:hi('Type',         s:skyblue,    0,            0)
call s:hi('Underlined',   s:skyblue,    0,            'underline')
call s:hi('Visual',       0,            s:sakura,     0)
call s:hi('Search',       0,            s:palesakura, 'underline')
call s:hi('SignColumn',   0,            s:bgemphasis, 0)
call s:hi('FoldColumn',   0,            s:bgemphasis, 0)
call s:hi('Folded',       s:fg,         s:light,      0)
call s:hi('SpellBad',     0,            s:red,        0)
call s:hi('SpellCap',     0,            s:light,      0)
call s:hi('SpellLocal',   0,            s:bgemphasis, 0)
call s:hi('SpellRare',    0,            s:sakura,     0)

" Filetype specific
call s:hi('diffAdded',             s:green,   0, 0)
call s:hi('diffRemoved',           s:red,     0, 0)
call s:hi('javaScriptBraces',      s:orange,  0, 0)
call s:hi('GitGutterAdd',          s:green,   0, 0)
call s:hi('GitGutterChange',       s:gold,    0, 0)
call s:hi('GitGutterChangeDelete', s:gold,    0, 0)
call s:hi('GitGutterDelete',       s:red,     0, 0)
call s:hi('goBuiltins',            s:red,     0, 0)
call s:hi('pythonBuiltin',         s:red,     0, 0)
call s:hi('vimCommand',            s:skyblue, 0, 0)

