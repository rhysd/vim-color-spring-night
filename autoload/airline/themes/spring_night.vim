" vim-airline companion theme of spring-night
"
"   https://github.com/vim-airline/vim-airline
"
" Usage:
"   let g:airline_theme = 'spring_night'

" Color Definitions {{{

" Normal mode
"          [ guifg, guibg, ctermfg, ctermbg, opts ]
let s:N1 = ['#334152', '#fedf81', 233, 222] " mode: gold
let s:N2 = ['#fedf81', '#40403c', 222, 238] " info: inverted gold
let s:N3 = ['#fffeee', '#435060', 230, 235] " statusline: normal
let s:N4 = ['#334152', '#a9de9c', 233, 150] " mode modified: green

" Insert mode
let s:I1 = ['#334152', '#a8d2eb', 233, 153] " skyblue
let s:I2 = ['#a8d2eb', '#40403c', 153, 238]
let s:I3 = ['#c8f2ff', '#435060', 189, 235] " paleskyblue

" Visual mode
let s:V1 = ['#334152', '#e996aa', 233, 175] " palesakura
let s:V2 = ['#e996aa', '#40403c', 175, 238]
let s:V3 = ['#ebeadb', '#435060', 224, 235]
let s:V4 = ['#d9869a', 132] " sakura

" Replace mode
let s:R1 = ['#334152', '#fd8489', 233, 210] " red
let s:R2 = ['#fd8489', '#40403c', 210, 238]
let s:R3 = ['#ffbfaf', '#435060', 217, 235] " palered
let s:R4 = ['#ff6f57', 203] " deeper red

" Paste mode
let s:PA = ['#fb8965', 209] " mikan

" Info modified
let s:IM = ['#40403c', 238]

" Inactive mode
let s:IA = ['#767676', s:N3[1], 243, s:N3[3], '']

" }}}

let g:airline#themes#spring_night#palette = {}

let g:airline#themes#spring_night#palette.accents = {
      \ 'red': ['#fd8489', '', 210, '', ''],
      \ }

let g:airline#themes#spring_night#palette.normal = airline#themes#generate_color_map(s:N1, s:N2, s:N3)
let g:airline#themes#spring_night#palette.normal_modified = {
    \ 'airline_a': [s:N1[0], s:N4[1], s:N1[2], s:N4[3], ''],
    \ 'airline_b': [s:N4[1], s:IM[0], s:N4[3], s:IM[1], ''],
    \ 'airline_c': [s:N4[1], s:N3[1], s:N4[3], s:N3[3], '']}

" Note:
" We don't prepare for 'modified' colors for insert mode because almost all
" time
" buffer is 'modified' while insert mode.
let g:airline#themes#spring_night#palette.insert = airline#themes#generate_color_map(s:I1, s:I2, s:I3)


let g:airline#themes#spring_night#palette.visual = airline#themes#generate_color_map(s:V1, s:V2, s:V3)
let g:airline#themes#spring_night#palette.visual_modified = {
    \ 'airline_a': [s:V1[0], s:V4[0], s:V1[2], s:V4[1], ''],
    \ 'airline_b': [s:V4[0], s:IM[0], s:V4[1], s:IM[1], ''],
    \ 'airline_c': [s:V4[0], s:N3[1], s:V4[1], s:N3[3], '']}


let g:airline#themes#spring_night#palette.replace = airline#themes#generate_color_map(s:R1, s:R2, s:R3)
let g:airline#themes#spring_night#palette.replace_modified = {
    \ 'airline_a': [s:R1[0], s:R4[0], s:R1[2], s:R4[1], ''],
    \ 'airline_b': [s:R4[0], s:IM[0], s:R4[1], s:IM[1], ''],
    \ 'airline_c': [s:R4[0], s:N3[1], s:R4[1], s:N3[3], '']}


let g:airline#themes#spring_night#palette.insert_paste = {
    \ 'airline_a': [s:I1[0], s:PA[0], s:I1[2], s:PA[1], ''],
    \ 'airline_b': [s:PA[0], s:IM[0], s:PA[1], s:IM[1], ''],
    \ 'airline_c': [s:PA[0], s:N3[1], s:PA[1], s:N3[3], '']}


let g:airline#themes#spring_night#palette.inactive = airline#themes#generate_color_map(s:IA, s:IA, s:IA)
let g:airline#themes#spring_night#palette.inactive_modified = {
    \ 'airline_c': [ s:N4[1] , ''      , s:N4[3] , ''      , ''     ] }

