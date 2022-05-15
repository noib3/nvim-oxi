pub mod api;
mod types;

pub use api::Buffer;
pub use types::{BufHandle, Error};

pub type Result<T> = std::result::Result<T, Error>;

#[no_mangle]
pub extern "C" fn test() -> *mut libc::c_char {
    // api::create_buf(true, true).to_string().len().try_into().unwrap()

    // std::ffi::CString::new(api::get_current_buf().get_name())
    //     .unwrap()
    //     .into_raw()

    api::replace_termcodes("<Cmd>q<CR>", true, true, true).unwrap().into_raw()
}
