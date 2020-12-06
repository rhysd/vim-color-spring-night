let s:highlights = filter(split(execute('highlight'), "\n"), {_, s -> s =~# '^\h'})
let s:count = 0
for s:hl in s:highlights
    if s:hl =~# 'guifg=\u\w*' || s:hl =~# 'guibg=\u\w*'
        echomsg s:hl
        let s:count += 1
    endif
endfor
if s:count != 0
    echomsg s:count . ' highlights are found'
else
    echomsg 'No highlight was found'
endif
