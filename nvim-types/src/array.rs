use std::ptr::NonNull;

use libc::size_t;

use super::object::Object;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L95
#[repr(C)]
pub struct Array {
    items: NonNull<Object>,
    size: size_t,
    capacity: size_t,
}
