" spring-night: Calm-colored dark color scheme
"
" Author: rhysd <lin90162@yahoo.co.jp>
" License: MIT
"   Copyright (c) 2016 rhysd
"
" PLEASE DO NOT MODIFY THIS FILE DIRECTLY!
" Generated by script vim-color-spring-night/gen/src/main.rs

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

let s:bg_gui = g:spring_night_high_contrast ? '#132132' : '#334152'
let s:bgweaker_gui = g:spring_night_high_contrast ? '#213243' : '#3a4b5c'
let s:darkgold_gui = g:spring_night_high_contrast ? '#484000' : '#685800'
let s:fg_cterm = g:spring_night_high_contrast ? 231 : 230

hi Boolean term=NONE guifg=#fd8489 ctermfg=210
hi Character term=NONE guifg=#a9dd9d ctermfg=150
hi ColorColumn term=NONE guibg=#536273 ctermbg=238
exe 'hi' 'Comment' 'term=NONE' 'guifg=#8d9eb2' 'ctermfg=103' g:spring_night_italic_comments ? s:italic_attr : ''
exe 'hi' 'Conceal' 'term=NONE' 'guifg=#fb8965' 'ctermfg=209' 'guibg='.s:bg_gui 'ctermbg=233'
hi Conditional term=NONE guifg=#a8d2eb ctermfg=153
hi Constant term=NONE guifg=#fd8489 ctermfg=210
exe 'hi' 'Cursor' 'term=NONE' 'guifg='.s:bg_gui 'ctermfg=233' 'guibg=#fffeeb' 'ctermbg='.s:fg_cterm
hi CursorColumn term=NONE guibg=#3a4b5c ctermbg=235
hi CursorLine term=NONE guibg=#3a4b5c ctermbg=235 gui=NONE cterm=NONE
hi CursorLineNr term=NONE guifg=#e7d5ff ctermfg=189 guibg=#536273 ctermbg=238
hi Define term=NONE guifg=#f0aa8a ctermfg=216
hi Directory term=NONE guifg=#a9dd9d ctermfg=150
hi EndOfBuffer term=NONE guifg=#536273 ctermfg=238
exe 'hi' 'Error' 'term=NONE' 'guifg=#fd8489' 'ctermfg=210' 'guibg=#3a4b5c' 'ctermbg=235' s:bold_attr
exe 'hi' 'ErrorMsg' 'term=NONE' 'guifg=#fd8489' 'ctermfg=210' 'guibg='.s:bg_gui 'ctermbg=233' s:bold_attr
hi Float term=NONE guifg=#fd8489 ctermfg=210
exe 'hi' 'NormalFloat' 'term=NONE' 'guifg=#fffeeb' 'ctermfg='.s:fg_cterm 'guibg='.s:bgweaker_gui 'ctermbg=235'
exe 'hi' 'FloatBorder' 'term=NONE' 'guifg=#8d9eb2' 'ctermfg=103' 'guibg='.s:bgweaker_gui 'ctermbg=235'
hi FoldColumn term=NONE guifg=#e7d5ff ctermfg=189 guibg=#3a4b5c ctermbg=235
hi Folded term=NONE guifg=#e7d5ff ctermfg=189 guibg=#646f7c ctermbg=60
hi Function term=NONE guifg=#f0aa8a ctermfg=216
exe 'hi' 'Identifier' 'term=NONE' 'guifg=#fedf81' 'ctermfg=222' s:italic_attr
hi IncSearch term=NONE guifg=NONE ctermfg=NONE guibg=#a9667a ctermbg=132 gui=underline cterm=underline
exe 'hi' 'Keyword' 'term=NONE' 'guifg=#f0eaaa' 'ctermfg=229' s:bold_attr
hi Label term=NONE guifg=#a8d2eb ctermfg=153
hi LineNr term=NONE guifg=#788898 ctermfg=102 guibg=#3a4b5c ctermbg=235
exe 'hi' 'MatchParen' 'term=NONE' 'guifg='.s:bg_gui 'ctermfg=233' 'guibg=#fedf81' 'ctermbg=222' s:bold_attr
hi ModeMsg term=NONE guifg=#fedf81 ctermfg=222
hi MoreMsg term=NONE guifg=#a9dd9d ctermfg=150
hi NonText term=NONE guifg=#646f7c ctermfg=60
exe 'hi' 'Normal' 'term=NONE' 'guifg=#fffeeb' 'ctermfg='.s:fg_cterm 'guibg='.s:bg_gui 'ctermbg=233'
hi Number term=NONE guifg=#fd8489 ctermfg=210
hi Operater term=NONE guifg=#f0aa8a ctermfg=216
hi Pmenu term=NONE guifg=#e7d5ff ctermfg=189 guibg=#3a4b5c ctermbg=235
hi PmenuSbar term=NONE guifg=#fedf81 ctermfg=222 guibg=#536273 ctermbg=238
hi PmenuSel term=NONE guifg=#fedf81 ctermfg=222 guibg=#536273 ctermbg=238
hi PmenuThumb term=NONE guifg=#fedf81 ctermfg=222 guibg=#8d9eb2 ctermbg=103
hi PreProc term=NONE guifg=#f0aa8a ctermfg=216
hi Question term=NONE guifg=#a8d2eb ctermfg=153
hi Search term=NONE guifg=NONE ctermfg=NONE guibg=#605779 ctermbg=60 gui=underline cterm=underline
exe 'hi' 'SignColumn' 'term=NONE' 'guifg=#fffeeb' 'ctermfg='.s:fg_cterm 'guibg=#3a4b5c' 'ctermbg=235'
exe 'hi' 'Special' 'term=NONE' 'guifg=#f0eaaa' 'ctermfg=229' s:bold_attr
hi SpecialKey term=NONE guifg=#607080 ctermfg=60
hi SpecialComment term=NONE guifg=#e7c6b7 ctermfg=181
if s:gui_running
    exe 'hi' 'SpellBad' 'term=NONE' 'guifg=#fd8489' 'ctermfg=210' 'guisp=#fd8489' s:undercurl_attr
else
    exe 'hi' 'SpellBad' 'term=NONE' 'guifg=#fd8489' 'ctermfg=210' 'guibg=NONE' 'ctermbg=NONE' 'guisp=#fd8489' s:undercurl_attr
endif
if s:gui_running
    exe 'hi' 'SpellCap' 'term=NONE' 'guifg=#e7d5ff' 'ctermfg=189' 'guisp=#e7d5ff' s:undercurl_attr
else
    exe 'hi' 'SpellCap' 'term=NONE' 'guifg=#e7d5ff' 'ctermfg=189' 'guibg=NONE' 'ctermbg=NONE' 'guisp=#e7d5ff' s:undercurl_attr
endif
if s:gui_running
    exe 'hi' 'SpellLocal' 'term=NONE' 'guifg=#fd8489' 'ctermfg=210' 'guisp=#fd8489' s:undercurl_attr
else
    exe 'hi' 'SpellLocal' 'term=NONE' 'guifg=#fd8489' 'ctermfg=210' 'guibg=NONE' 'ctermbg=NONE' 'guisp=#fd8489' s:undercurl_attr
endif
if s:gui_running
    exe 'hi' 'SpellRare' 'term=NONE' 'guifg=#f0eaaa' 'ctermfg=229' 'guisp=#f0eaaa' s:undercurl_attr
else
    exe 'hi' 'SpellRare' 'term=NONE' 'guifg=#f0eaaa' 'ctermfg=229' 'guibg=NONE' 'ctermbg=NONE' 'guisp=#f0eaaa' s:undercurl_attr
endif
hi Statement term=NONE guifg=#a8d2eb ctermfg=153
exe 'hi' 'StatusLine' 'term=NONE' 'guifg=#fffeeb' 'ctermfg='.s:fg_cterm 'guibg=#536273' 'ctermbg=238' s:bold_attr
hi StatusLineNC term=NONE guifg=#8d9eb2 ctermfg=103 guibg=#3a4b5c ctermbg=235 gui=NONE cterm=NONE
exe 'hi' 'StatusLineTerm' 'term=NONE' 'guifg=#fffeeb' 'ctermfg='.s:fg_cterm 'guibg=#536273' 'ctermbg=238' s:bold_attr
hi StatusLineTermNC term=NONE guifg=#8d9eb2 ctermfg=103 guibg=#3a4b5c ctermbg=235 gui=NONE cterm=NONE
exe 'hi' 'StorageClass' 'term=NONE' 'guifg=#fedf81' 'ctermfg=222' s:italic_attr
hi String term=NONE guifg=#a9dd9d ctermfg=150
hi TabLine term=NONE guifg=#8d9eb2 ctermfg=103 guibg=#536273 ctermbg=238
hi TabLineFill term=NONE guifg=#3a4b5c ctermfg=235
exe 'hi' 'TabLineSel' 'term=NONE' 'guifg=#fedf81' 'ctermfg=222' 'guibg='.s:bg_gui 'ctermbg=233' s:bold_attr
hi Tag term=NONE guifg=#f0aa8a ctermfg=216
exe 'hi' 'Title' 'term=NONE' 'guifg=#fedf81' 'ctermfg=222' s:bold_attr
exe 'hi' 'Todo' 'term=NONE' 'guifg='.s:bg_gui 'ctermfg=233' 'guibg=#fd8489' 'ctermbg=210' s:bold_attr
exe 'hi' 'ToolbarButton' 'term=NONE' 'guifg=#fedf81' 'ctermfg=222' 'guibg='.s:bg_gui 'ctermbg=233' s:bold_attr
hi ToolbarLine term=NONE guifg=#8d9eb2 ctermfg=103 guibg=#536273 ctermbg=238
hi Type term=NONE guifg=#fedf81 ctermfg=222
hi Underlined term=NONE guifg=#a8d2eb ctermfg=153 gui=underline cterm=underline
exe 'hi' 'VertSplit' 'term=NONE' 'guifg=#3a4b5c' 'ctermfg=235' 'guibg='.s:bg_gui 'ctermbg=233'
hi Visual term=NONE guibg=#70495d ctermbg=95
hi WarningMsg term=NONE guifg=#fb8965 ctermfg=209 guibg=#3a4b5c ctermbg=235
exe 'hi' 'WildMenu' 'term=NONE' 'guifg='.s:bg_gui 'ctermfg=233' 'guibg=#fedf81' 'ctermbg=222'
hi cmakeArguments term=NONE guifg=#f0eaaa ctermfg=229
hi cmakeOperators term=NONE guifg=#fd8489 ctermfg=210
hi cStorageClass term=NONE guifg=#f0eaaa ctermfg=229
hi cTypedef term=NONE guifg=#f0eaaa ctermfg=229
exe 'hi' 'DiffAdd' 'term=NONE' 'guibg=#5f8770' 'ctermbg=65' s:bold_attr
exe 'hi' 'DiffChange' 'term=NONE' 'guibg='.s:darkgold_gui 'ctermbg=58' s:bold_attr
exe 'hi' 'DiffDelete' 'term=NONE' 'guifg=#fffeeb' 'ctermfg='.s:fg_cterm 'guibg=#ab6560' 'ctermbg=167' s:bold_attr
exe 'hi' 'DiffText' 'term=NONE' 'guibg='.s:bg_gui 'ctermbg=233'
hi diffAdded term=NONE guifg=#a9dd9d ctermfg=150
hi diffFile term=NONE guifg=#f0eaaa ctermfg=229
hi diffIndexLine term=NONE guifg=#fedf81 ctermfg=222
hi diffNewFile term=NONE guifg=#f0eaaa ctermfg=229
hi diffRemoved term=NONE guifg=#fd8489 ctermfg=210
hi gitCommitOverflow term=NONE guibg=#ab6560 ctermbg=167
hi gitCommitSummary term=NONE guifg=#f0eaaa ctermfg=229
hi gitCommitSelectedFile term=NONE guifg=#a8d2eb ctermfg=153
exe 'hi' 'gitconfigSection' 'term=NONE' 'guifg=#a8d2eb' 'ctermfg=153' s:bold_attr
hi goBuiltins term=NONE guifg=#fd8489 ctermfg=210
hi helpExample term=NONE guifg=#a8d2eb ctermfg=153
hi helpCommand term=NONE guifg=#e7d5ff ctermfg=189
hi htmlBold term=NONE guibg=#3a4b5c ctermbg=235
hi htmlLinkText term=NONE guifg=#a8d2eb ctermfg=153
hi htmlTagName term=NONE guifg=#f0aa8a ctermfg=216
exe 'hi' 'javaScriptBraces' 'term=NONE' 'guifg=#fffeeb' 'ctermfg='.s:fg_cterm
hi makeCommands term=NONE guifg=#f0eaaa ctermfg=229
hi markdownCode term=NONE guifg=#f0eaaa ctermfg=229
hi markdownUrl term=NONE guifg=#8d9eb2 ctermfg=103
hi ocamlConstructor term=NONE guifg=#fedf81 ctermfg=222
hi ocamlKeyChar term=NONE guifg=#a8d2eb ctermfg=153
hi ocamlKeyword term=NONE guifg=#fedf81 ctermfg=222
hi ocamlFunDef term=NONE guifg=#a8d2eb ctermfg=153
hi plantumlColonLine term=NONE guifg=#a8d2eb ctermfg=153
hi pythonBuiltin term=NONE guifg=#fd8489 ctermfg=210
hi qfFileName term=NONE guifg=#fedf81 ctermfg=222
hi qfLineNr term=NONE guifg=#a8d2eb ctermfg=153
exe 'hi' 'rstEmphasis' 'term=NONE' 'guibg=#3a4b5c' 'ctermbg=235' s:italic_attr
exe 'hi' 'rstStrongEmphasis' 'term=NONE' 'guibg=#536273' 'ctermbg=238' s:bold_attr
hi rubyFunction term=NONE guifg=#f0eaaa ctermfg=229
hi rubyIdentifier term=NONE guifg=#f0eaaa ctermfg=229
hi rustEnumVariant term=NONE guifg=#fedf81 ctermfg=222
exe 'hi' 'rustFuncCall' 'term=NONE' 'guifg=#fffeeb' 'ctermfg='.s:fg_cterm
hi rustCommentLineDoc term=NONE guifg=#e7c6b7 ctermfg=181
hi scalaInstanceDeclaration term=NONE guifg=#fedf81 ctermfg=222
hi scalaInstanceDeclaration term=NONE guifg=#fedf81 ctermfg=222
hi tomlTable term=NONE guifg=#a8d2eb ctermfg=153
hi tomlTableArray term=NONE guifg=#a8d2eb ctermfg=153
hi tomlKey term=NONE guifg=#fedf81 ctermfg=222
exe 'hi' 'typescriptBraces' 'term=NONE' 'guifg=#fffeeb' 'ctermfg='.s:fg_cterm
hi typescriptAsyncFuncKeyword term=NONE guifg=#a8d2eb ctermfg=153
exe 'hi' 'typescriptKeywordOp' 'term=NONE' 'guifg=#f0eaaa' 'ctermfg=229' s:bold_attr
hi vimfilerColumn__SizeLine term=NONE guifg=#8d9eb2 ctermfg=103
hi vimfilerClosedFile term=NONE guifg=#a9dd9d ctermfg=150
hi vimCommand term=NONE guifg=#a8d2eb ctermfg=153
exe 'hi' 'wastListDelimiter' 'term=NONE' 'guifg=#fffeeb' 'ctermfg='.s:fg_cterm
hi wastInstGeneral term=NONE guifg=#f0eaaa ctermfg=229
hi wastInstGetSet term=NONE guifg=#f0eaaa ctermfg=229
hi wastInstWithType term=NONE guifg=#f0eaaa ctermfg=229
hi wastUnnamedVar term=NONE guifg=#e7d5ff ctermfg=189
hi zshDelimiter term=NONE guifg=#a8d2eb ctermfg=153
hi zshPrecommand term=NONE guifg=#fd8489 ctermfg=210
hi ghaworkflowAttrName term=NONE guifg=#f0eaaa ctermfg=229
exe 'hi' 'debugPC' 'term=NONE' 'guifg='.s:bg_gui 'ctermfg=233' 'guibg=#a8d2eb' 'ctermbg=153'
exe 'hi' 'debugBreakPoint' 'term=NONE' 'guifg='.s:bg_gui 'ctermfg=233' 'guibg=#fedf81' 'ctermbg=222'
hi zigMultilineStringDelimiter term=NONE guifg=#f0eaaa ctermfg=229
exe 'hi' 'ALEWarningSign' 'term=NONE' 'guifg=#f0aa8a' 'ctermfg=216' 'guibg=#3a4b5c' 'ctermbg=235' s:bold_attr
exe 'hi' 'ALEErrorSign' 'term=NONE' 'guifg=#3a4b5c' 'ctermfg=235' 'guibg=#ab6560' 'ctermbg=167' s:bold_attr
hi ALEInfoSign term=NONE guibg=#646f7c ctermbg=60
hi ALEError term=NONE guibg=#ab6560 ctermbg=167
exe 'hi' 'ALEWarning' 'term=NONE' 'guibg='.s:darkgold_gui 'ctermbg=58'
hi Flake8_Error term=NONE guifg=#fd8489 ctermfg=210 guibg=#3a4b5c ctermbg=235
hi Flake8_Warning term=NONE guifg=#f0eaaa ctermfg=229 guibg=#3a4b5c ctermbg=235
hi Flake8_PyFlake term=NONE guifg=#a8d2eb ctermfg=153 guibg=#3a4b5c ctermbg=235
hi Flake8_Complexity term=NONE guifg=#a8d2eb ctermfg=153 guibg=#3a4b5c ctermbg=235
hi Flake8_Naming term=NONE guifg=#a8d2eb ctermfg=153 guibg=#3a4b5c ctermbg=235
hi SignifySignAdd term=NONE guifg=#a9dd9d ctermfg=150 guibg=#3a4b5c ctermbg=235
hi SignifySignChange term=NONE guifg=#f0eaaa ctermfg=229 guibg=#3a4b5c ctermbg=235
hi SignifySignChangeDelete term=NONE guifg=#fedf81 ctermfg=222 guibg=#3a4b5c ctermbg=235
hi SignifySignDelete term=NONE guifg=#fd8489 ctermfg=210 guibg=#3a4b5c ctermbg=235
exe 'hi' 'CleverFChar' 'term=NONE' 'guifg='.s:bg_gui 'ctermfg=233' 'guibg=#fd8489' 'ctermbg=210'
exe 'hi' 'CleverFDirect' 'term=NONE' 'guifg='.s:bg_gui 'ctermfg=233' 'guibg=#fd8489' 'ctermbg=210'
exe 'hi' 'DirvishArg' 'term=NONE' 'guifg=#f0eaaa' 'ctermfg=229' s:bold_attr
exe 'hi' 'EasyMotionTarget' 'term=NONE' 'guifg=#fd8489' 'ctermfg=210' s:bold_attr
exe 'hi' 'EasyMotionShade' 'term=NONE' 'guifg=#8d9eb2' 'ctermfg=103' 'guibg='.s:bg_gui 'ctermbg=233'
hi GitGutterAdd term=NONE guifg=#a9dd9d ctermfg=150 guibg=#3a4b5c ctermbg=235
hi GitGutterChange term=NONE guifg=#f0eaaa ctermfg=229 guibg=#3a4b5c ctermbg=235
hi GitGutterChangeDelete term=NONE guifg=#fedf81 ctermfg=222 guibg=#3a4b5c ctermbg=235
hi GitGutterDelete term=NONE guifg=#fd8489 ctermfg=210 guibg=#3a4b5c ctermbg=235
hi HighlightedyankRegion term=NONE guibg=#3a4b5c ctermbg=235
if s:gui_running
    exe 'hi' 'EasyMotionIncCursor' 'term=NONE' 'guifg='.s:bg_gui 'ctermfg=233' 'guibg=#fffeeb' 'ctermbg='.s:fg_cterm
else
    hi EasyMotionIncCursor term=NONE gui=reverse cterm=reverse
endif
hi plugDeleted term=NONE guifg=#8d9eb2 ctermfg=103
hi ConflictMarker term=NONE guibg=#ab6560 ctermbg=167
exe 'hi' 'IndentGuidesOdd' 'term=NONE' 'guibg='.s:bgweaker_gui 'ctermbg=235'
hi IndentGuidesEven term=NONE guibg=#3a4b5c ctermbg=235

if g:spring_night_highlight_terminal
    if has('nvim')
        if s:gui_running || s:true_colors
            let g:terminal_color_0 = '#132132'
            let g:terminal_color_1 = '#ff6a6f'
            let g:terminal_color_2 = '#a9dd9d'
            let g:terminal_color_3 = '#fedf81'
            let g:terminal_color_4 = '#7098e6'
            let g:terminal_color_5 = '#605779'
            let g:terminal_color_6 = '#a8d2eb'
            let g:terminal_color_7 = '#fffeeb'
            let g:terminal_color_8 = '#8d9eb2'
            let g:terminal_color_9 = '#fd8489'
            let g:terminal_color_10 = '#c9fd88'
            let g:terminal_color_11 = '#f0eaaa'
            let g:terminal_color_12 = '#98b8e6'
            let g:terminal_color_13 = '#e7d5ff'
            let g:terminal_color_14 = '#a8d2eb'
            let g:terminal_color_15 = '#ffffff'
        else
            let g:terminal_color_0 = 233
            let g:terminal_color_1 = 203
            let g:terminal_color_2 = 150
            let g:terminal_color_3 = 222
            let g:terminal_color_4 = 69
            let g:terminal_color_5 = 60
            let g:terminal_color_6 = 153
            let g:terminal_color_7 = 231
            let g:terminal_color_8 = 103
            let g:terminal_color_9 = 210
            let g:terminal_color_10 = 149
            let g:terminal_color_11 = 229
            let g:terminal_color_12 = 111
            let g:terminal_color_13 = 189
            let g:terminal_color_14 = 153
            let g:terminal_color_15 = 231
        endif
        let g:terminal_color_background = g:terminal_color_0
        let g:terminal_color_foreground = g:terminal_color_7
    elseif (s:gui_running || s:true_colors) && exists('*term_setansicolors')
        let g:terminal_ansi_colors = ['#132132', '#ff6a6f', '#a9dd9d', '#fedf81', '#7098e6', '#605779', '#a8d2eb', '#fffeeb', '#8d9eb2', '#fd8489', '#c9fd88', '#f0eaaa', '#98b8e6', '#e7d5ff', '#a8d2eb', '#ffffff']
    endif
endif
