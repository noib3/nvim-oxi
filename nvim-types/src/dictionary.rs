use std::ptr::NonNull;

use libc::size_t;

use super::object::Object;
use super::string::String;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L95
#[repr(C)]
pub struct Dictionary {
    items: NonNull<KeyValuePair>,
    size: size_t,
    capacity: size_t,
}

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L128
#[repr(C)]
struct KeyValuePair {
    key: String,
    data: Object,
}
