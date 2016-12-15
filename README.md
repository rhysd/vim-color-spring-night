Calm Spring Color Scheme for Vim
================================

`spring-night` is a low contrast calm color scheme for Vim.

It supports:
- Both GUI 24bit colors and CUI 256 bit colors.
- Aware of running Vim on translucent window
- Enable to tweak contrast


## On gVim or Terminal with True Color Support (24bit color)

![screenshot for 24bit color](https://github.com/rhysd/ss/blob/master/vim-color-spring-night/gui.png?raw=true)


## On Terminal (8bit color)

![screenshot for 8bit color](https://github.com/rhysd/ss/blob/master/vim-color-spring-night/cui.png?raw=true)


## Installation

Copy `colors` directory into your `~/.vim` (or `~/vimfiles` on Windows) or use `:packadd` (Vim8). Or use your favorite plugin manager.


## Usage

Write below in your `.vimrc`.

```vim
colorscheme spring-night
```

This package provides a theme for [vim-airline][].

```vim
let g:airline_theme = 'spring_night'
```


## Customize

You can customize some behaviors of this colorscheme.

- **`g:spring_night_kill_itralic`**: If `1`, this colorscheme does not use italic attribute. Default value is `0`.
- **`g:spring_night_high_contrast`**: List of string. If it includes `'cui'`, it uses high contrast version on Vim in terminal. If it includes `'gui'`, it uses high contrast version on gVim. Default value depends on `termguicolors` option.


## License

Licensed under the MIT license.

    MIT License

    Copyright (c) 2016 rhysd

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
    of the Software, and to permit persons to whom the Software is furnished to do so,
    subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
    INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
    PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
    LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
    TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR
    THE USE OR OTHER DEALINGS IN THE SOFTWARE.

[vim-airline]: https://github.com/vim-airline/vim-airline
