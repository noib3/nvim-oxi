pub mod api;
mod error;

pub use api::Buffer;
pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[no_mangle]
pub extern "C" fn test() -> *mut std::os::raw::c_char {
    // api::create_buf(true, true).to_string().len().try_into().unwrap()

    // std::ffi::CString::new(api::get_current_buf().get_name())
    //     .unwrap()
    //     .into_raw()

    api::replace_termcodes("<Cmd>q<CR>", true, true, true).unwrap().into_raw()
}
