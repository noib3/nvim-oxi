use std::marker::{PhantomData, PhantomPinned};

use once_cell::unsync::OnceCell;

thread_local! {
    pub(crate) static LUA: OnceCell<*mut lua_State> = OnceCell::new();
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct lua_State {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}
