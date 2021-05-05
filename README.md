# web-browser-lsp
A toy program that implements a text-based web browser as a language server.

## Motivation
My favorite progrmming tools are neovim, tmux on a fast terminal emulator and firefox. R.I.P. vimperator.
I've tried w3m, lynx, browsh, libsixel, vim-bind webextension, keyboard-driven browser, etc.
but I can't find anything that goes beyond vimerator.

## Features
There is no implementation
* initialize, shutdown, exit
* `web-browser-lsp/tab` creates first tab but not connect
* `textDocument/formatting` shows tab contents in text editor

![demo](https://user-images.githubusercontent.com/7942952/117168466-21cab600-ae03-11eb-9f8c-5b4736bc60d9.gif)

## Wish and Concept
* text-based web browser
* Using a full-featured browser
  - Connecting via Chrome DevTools Protocol
  - It's impossible to develop a full-featured browser by myself
* and you can switch to the GUI
  - Non-textual rich contents may be required
* Human-readable text mode and Raw (html) mode
  - Web developer friendly
  - Nested [onclick] can be clicked precisely
* zoom in/out html tags
  - Focus on the main content of a page with multiple columns

I'm considering about [operations required for the browser and corresponding lsp methods](./mapping_lsp.md).

## Credits
* [rust-analyzer/rust-analyzer](https://github.com/rust-analyzer/rust-analyzer) [MIT](https://github.com/rust-analyzer/rust-analyzer/blob/master/LICENSE-MIT)
