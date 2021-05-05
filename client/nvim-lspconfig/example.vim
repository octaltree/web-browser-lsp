
function! s:start_lsp() abort
  lua <<EOF
  local capabilities = lsp.protocol.make_client_capabilities()
  require'lspconfig'.web_browser_lsp.setup{
    capabilities = capabilities,
    settings = {
      ["web-browser-lsp"] = {
      }
    }
  }
EOF

  augroup auto_hover
    autocmd!
    autocmd CursorHold,CursorHoldI * :lua vim.lsp.buf.hover()
  augroup END
endfunction

" unnamed buffer or with extension
autocmd BufNewFile,BufRead *.textbrowser setlocal filetype=textbrowser
autocmd FileType textbrowser call start_lsp()
