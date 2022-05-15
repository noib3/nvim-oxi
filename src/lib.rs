pub mod api;
mod types;

#[no_mangle]
pub extern "C" fn test() -> *mut libc::c_char {
    // std::ffi::CString::from_vec_with_nul(
    //     api::get_current_buf().get_name().into_bytes(),
    // )
    // .unwrap()

    // api::create_buf(true, true).to_string().len().try_into().unwrap()

    std::ffi::CString::new(api::get_current_buf().get_name())
        .unwrap()
        .into_raw()
}
