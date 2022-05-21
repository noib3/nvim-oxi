pub mod api;
mod error;

pub use api::Buffer;
pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[no_mangle]
pub extern "C" fn test() -> *mut std::os::raw::c_char {
    // pub extern "C" fn test() {
    // api::create_buf(true, true).to_string().len().try_into().unwrap()

    // std::ffi::CString::new(api::get_current_buf().get_name())
    //     .unwrap()
    //     .into_raw()

    // api::replace_termcodes("<Cmd>q<CR>", true, true, true).unwrap().into_raw()

    // api::echo(
    //     [
    //         ("hey", Some("IncSearch")),
    //         (", this is some", None),
    //         ("Bullshiat", Some("DiffDelete")),
    //         ("Bullshiat", Some("ciaone")),
    //     ],
    //     true,
    // )
    // .unwrap()

    api::get_mode()
        .get::<_, nvim_types::NvimString>("mode")
        .unwrap()
        .as_c_str()
        .to_owned()
        .into_raw()
}
