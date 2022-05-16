// https://github.com/neovim/neovim/blob/master/src/nvim/types.h#L18
#[allow(non_camel_case_types)]
type handle_T = libc::c_int;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L82
pub type BufHandle = handle_T;
