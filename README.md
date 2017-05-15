Calm Spring Color Scheme for Vim [![Build Status][]](https://travis-ci.org/rhysd/vim-color-spring-night)
================================

`spring-night` is a low contrast calm color scheme for Vim. It's colored with deep blue background, shiny yellow foreground and *sakura*:cherry_blossom: text selection.

- Both GUI 24bit colors and CUI 256 bit colors.
- Aware of running Vim on translucent window.
- Enable to tweak contrast.
- Optimized for many filetypes.
- Support [vim-gitgutter][] and other some plugins.


## On gVim (MacVim)

| Vim script                       | Go                       |
|----------------------------------|--------------------------|
| ![Vim script on gVim (MacVim)][] | ![Go on gVim (MacVim)][] |


## On Translucent Terminal (24bit color)

| Vim script                           | Go                           |
|--------------------------------------|------------------------------|
| ![Vim script on Vim (24bit color)][] | ![Go on Vim (24bit color)][] |


## On Translucent Terminal (8bit color)

| Vim script                          | Go                          |
|-------------------------------------|-----------------------------|
| ![Vim script on Vim (8bit color)][] | ![Go on Vim (8bit color)][] |

If you want to see more code, please visit [demo page site][].

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

- **`g:spring_night_kill_italic`**: If `1`, this colorscheme does not use italic attribute. Default value is `0`.
- **`g:spring_night_high_contrast`**: List of string. If it includes `'cui'`, it uses high contrast version on Vim in terminal. If it includes `'gui'`, it uses high contrast version on gVim. Default value depends on `termguicolors` option.


## Supported Filetypes

This colorscheme is especially optimized for below filetypes/plugins:

- `c`
- `cpp`
- `diff`
- `gitcommit`
- `go`
- `help`
- `html`
- `javascript`
- `markdown`
- `ocaml`
- `python`
- `ruby`
- `rust`
- `typescript`
- `vim`
- `zsh`
- [ctrlp][]
- [vim-gitgutter][]
- [vim-easymotion][]
- [clever-f.vim][]
- [vimfiler.vim][]
- ... and more

## Contribution

If you find some odd highlight, I'll be happy if you report it to [issues](https://github.com/rhysd/vim-color-spring-night/issues/new) with how to reproduce.
And pull requests are of course welcome :smile:

For colors palette, please see [the source code directly](https://github.com/rhysd/vim-color-spring-night/blob/master/colors/spring-night.vim).
The variable definitions are color table of this colorscheme.

## Bonus: Slack Theme

```
#132132,#3a4b5c,#fedf81,#132132,#8090a0,#fffeeb,#a9dd9d,#a9667a
```

## License

Licensed under the MIT license.

```
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
```

[demo page site]: https://rhysd.github.io/vim-color-spring-night
[vim-airline]: https://github.com/vim-airline/vim-airline
[vim-gitgutter]: https://github.com/airblade/vim-gitgutter
[ctrlp]: https://github.com/ctrlpvim/ctrlp.vim
[vim-easymotion]: https://github.com/easymotion/vim-easymotion
[Vim script on gVim (MacVim)]: https://github.com/rhysd/ss/blob/master/vim-color-spring-night/gui-vim.jpg?raw=true
[Go on gVim (MacVim)]: https://github.com/rhysd/ss/blob/master/vim-color-spring-night/gui-go.jpg?raw=true
[Vim script on Vim (24bit color)]: https://github.com/rhysd/ss/blob/master/vim-color-spring-night/terminal-64bit-vim.jpg?raw=true
[Go on Vim (24bit color)]: https://github.com/rhysd/ss/blob/master/vim-color-spring-night/terminal-64bit-go.jpg?raw=true
[Vim script on Vim (8bit color)]: https://github.com/rhysd/ss/blob/master/vim-color-spring-night/terminal-8bit-vim.jpg?raw=true
[Go on Vim (8bit color)]: https://github.com/rhysd/ss/blob/master/vim-color-spring-night/terminal-8bit-go.jpg?raw=true
[Build Status]: https://travis-ci.org/rhysd/vim-color-spring-night.svg?branch=travis
[clever-f.vim]: https://github.com/rhysd/clever-f.vim
[vimfiler.vim]: https://github.com/Shougo/vimfiler.vim
