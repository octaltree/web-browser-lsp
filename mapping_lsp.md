# Operations required for the browser and corresponding lsp methods
## Use
* Data TextDocumentEdit
* Notification `textDocument/didChange` inputs form
* Request `textDocument/definition` clicks link
* Request workspace.symbol shows bookmarks and history, etc.
* Request `textDocument/documentSymbol` lists input or id
* Request `textDocument/codeAction` does dblclick, ctrl-click, etc.
* Request `document/hover` wih autocmd CursorHold emits mouse hover
* Request `textDoucment/formatting` re-rendering

## Useful
* Notification '$/cancelRequest'
* Notification '$/progress'
* Data MarkupContent
* Notification `window/showMessage`
* Request `workspace/configuration`
* Request `workspace/executeCommand`
* Request `Completion`
* Request `textDocument/signatureHelp`
* Request `textDocument/declaration` Goto Declaration
* Request `textDocument/typeDefinition` Goto Type Definition
* Request `textDocument/declaration` Goto Declaration
* Request `textDocument/implementation`  Goto Implementation
* Request `textDocument/documentLink` I don't need this because I have my own way to open links in textbrowser.
