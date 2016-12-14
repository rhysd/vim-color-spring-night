" Vim color scheme
" spring-night
" Aughr: rhysd <lin90162@yahoo.co.jp>

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

let g:calmnight_kill_italic = get(g:, 'calmnight_kill_italic', 0)

" Define reusable colorvariables.
let s:bg       = ['#334152', 233]
let s:fg       = ['#fffeee', 230]
let s:fg2      = ['#ebeadb', 224]
let s:fg3      = ['#d6d5c8', 188]
let s:fg4      = ['#c2c1b5', 145]
let s:bg2      = ['#435060', 235]
let s:bg3      = ['#545f6e', 59]
let s:bg4      = ['#646f7c', 60]
let s:bg5      = ['#a9667a', 132]
let s:bg6      = ['#b9768a', 175]
let s:keyword  = ['#fda08b', 216]
let s:builtin  = ['#fd8489', 210]
let s:const    = ['#a9de9c', 150]
let s:comment  = ['#8090a0', 103]
let s:func     = ['#fd8489', 210]
let s:str      = ['#a9dd9d', 150]
let s:type     = ['#a8d2eb', 153]
let s:var      = ['#fedf81', 222]
let s:error    = ['#fc5554', 203]
let s:warning  = ['#fb8965', 209]
let s:diff_fg  = ['#f8f8f8', 231]
let s:diff_bg  = ['#46830c', 64]

let s:NUMBER = type(0)

" function! s:hi(name, fg, bg, ...)
function! s:hi(name, fg, bg, attr) abort
    let fg = type(a:fg) != s:NUMBER
    let bg = type(a:bg) != s:NUMBER

    let guifg   = fg ? ('guifg=' . a:fg[0]) : ''
    let guibg   = bg ? ('guibg=' . a:bg[0]) : ''
    let ctermfg = fg ? ('ctermfg=' . a:fg[1]) : ''
    let ctermbg = bg ? ('ctermbg=' . a:bg[1]) : ''

    if type(a:attr) != s:NUMBER && !(g:calmnight_kill_italic && a:attr ==# 'italic')
        let attr =  'gui=' . a:attr

        if a:attr !=# 'italic'
            let attr .= ' cterm=' . a:attr
        endif
    else
        let attr = ''
    endif

    exe 'hi' a:name guifg guibg ctermfg ctermbg attr
endfunction

call s:hi('Normal',       s:fg,       s:bg,       0)
call s:hi('Cursor',       s:bg,       s:fg,       0)
call s:hi('Cursorline',   0,          s:bg2,      0)
call s:hi('CursorColumn', 0,          s:bg2,      0)
call s:hi('ColorColumn',  0,          s:bg2,      0)
call s:hi('LineNr',       s:fg2,      s:bg2,      0)
call s:hi('VertSplit',    s:fg3,      s:fg,       0)
call s:hi('MatchParen',   s:warning,  0,          'underline')
call s:hi('StatusLine',   s:fg2,      s:bg3,      'bold')
call s:hi('Pmenu',        s:fg,       s:bg2,      0)
call s:hi('PmenuSel',     0,          s:bg3,      0)
call s:hi('Directory',    s:const,    0,          0)
call s:hi('Folded',       s:fg4,      s:bg,       0)
call s:hi('Boolean',      s:const,    0,          0)
call s:hi('Character',    s:const,    0,          0)
call s:hi('Comment',      s:comment,  0,          0)
call s:hi('Conditional',  s:keyword,  0,          0)
call s:hi('Constant',     s:const,    0,          0)
call s:hi('Define',       s:keyword,  0,          0)
call s:hi('DiffAdd',      s:diff_fg,  s:diff_bg,  'bold')
call s:hi('DiffDelete',   s:fg,       s:error,    0)
call s:hi('DiffChange',   s:fg,       s:var,      0)
call s:hi('DiffText',     s:fg,       s:builtin,  'bold')
call s:hi('ErrorMsg',     s:bg,       s:error,    'bold')
call s:hi('WarningMsg',   s:bg,       s:warning,  0)
call s:hi('Float',        s:const,    0,          0)
call s:hi('Function',     s:func,     0,          0)
call s:hi('Identifier',   s:var,      0,          'italic')
call s:hi('Keyword',      s:keyword,  0,          'bold')
call s:hi('Label',        s:var,      0,          0)
call s:hi('NonText',      s:bg4,      0,          0)
call s:hi('Number',       s:const,    0,          0)
call s:hi('Operater',     s:keyword,  0,          0)
call s:hi('PreProc',      s:keyword,  0,          0)
call s:hi('Special',      s:fg,       0,          0)
call s:hi('SpecialKey',   s:fg2,      s:bg2,      0)
call s:hi('Statement',    s:keyword,  0,          0)
call s:hi('StorageClass', s:type,     0,          'italic')
call s:hi('String',       s:str,      0,          0)
call s:hi('Tag',          s:keyword,  0,          0)
call s:hi('Title',        s:fg,       0,          'bold')
call s:hi('Todo',         s:fg2,      0,          'inverse,bold')
call s:hi('Type',         s:type,     0,          0)
call s:hi('Underlined',   0,          0,          'underline')
call s:hi('Visual',       0,          s:bg5,      0)
call s:hi('Search',       0,          s:bg6,      'underline')
call s:hi('Todo',         s:var,      s:bg,       'bold')
call s:hi('SignColumn',   0,          s:bg2,     0)

" Filetype specific
call s:hi('rubyAttribute',             s:builtin, 0, 0)
call s:hi('rubyLocalVariableOrMethod', s:var,     0, 0)
call s:hi('rubyGlobalVariable',        s:var,     0, 'italic')
call s:hi('rubyInstanceVariable',      s:var,     0, 0)
call s:hi('rubyKeyword',               s:keyword, 0, 0)
call s:hi('rubyKeywordAsMethod',       s:keyword, 0, 'bold')
call s:hi('rubyClassDeclaration',      s:keyword, 0, 'bold')
call s:hi('rubyClass',                 s:keyword, 0, 'bold')
call s:hi('rubyNumber',                s:const,   0, 0)
call s:hi('pythonBuiltinFunc',         s:builtin, 0, 0)
call s:hi('goBuiltins',                s:builtin, 0, 0)
call s:hi('javaScriptBraces',          s:keyword, 0, 'bold')
call s:hi('htmlLink',                  s:var,     0, 'underline')
call s:hi('htmlStatement',             s:keyword, 0, 0)
call s:hi('htmlSpecialTagName',        s:keyword, 0, 0)
call s:hi('mkdCode',                   s:builtin, 0, 0)
