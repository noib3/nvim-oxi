#!/bin/bash
export lua="
local api = require'api'
api.open_window()
api.close_window()
local k = vim.api.nvim_replace_termcodes('iFrom insert mode: hi<esc>', true, false, true)
vim.api.nvim_feedkeys(k, 'x', true)
print(vim.api.nvim_buf_get_lines(0, 0, -1, false)[1])
vim.cmd [[ Greetings ]]
vim.cmd [[q!]]
"
bash ../run.sh
