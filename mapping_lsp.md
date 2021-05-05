# Operations required for the browser and corresponding lsp methods
## Use
* Data TextDocumentEdit
* Notification `textDocument/didChange` inputs form
* Request `textDocument/definition` clicks link
* Request workspace.symbol shows bookmarks and history, etc.
* Request `textDocument/documentSymbol` lists input or id or tabs
* Request `textDocument/codeAction` does dblclick, ctrl-click, etc.
* Request `document/hover` wih autocmd CursorHold emits mouse hover
* Request `textDoucment/formatting` re-rendering

* Request `web-browser-lsp/page/reload`
* Request `web-browser-lsp/page/goBack`
* Request `web-browser-lsp/page/goForward`

## Useful
* Notification '$/cancelRequest'
* Notification '$/progress'
* Data MarkupContent
* Notification `window/showMessage`
* Request `workspace/configuration`
* Request `workspace/executeCommand`
* Request `Completion`
* Request `textDocument/signatureHelp`
* Request `textDocument/declaration`
* Request `textDocument/typeDefinition` Goto Type Definition
* Request `textDocument/implementation`
* Request `textDocument/documentLink` I don't need this because I have my own way to open links in textbrowser.
