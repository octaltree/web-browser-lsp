local configs = require 'lspconfig/configs'
local util = require 'lspconfig/util'
local lsp = vim.lsp

configs.web_browser_lsp = {
  default_config = {
    cmd = {"web-browser-lsp"},
    filetypes = {"textbrowser"},
    root_dir = function(fname)
      return vim.fn.getcwd()
    end,
    settings = {
      ["web-browser-lsp"] = {}
    }
  },
  commands = {},
  docs = {
    package_json = "",
    description = "",
    default_config = {}
  }
}
