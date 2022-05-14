pub mod api;
mod types;

#[no_mangle]
pub extern "C" fn test() -> i32 {
    // std::ffi::CString::from_vec_with_nul(
    //     api::get_current_buf().get_name().into_bytes(),
    // )
    // .unwrap()

    api::create_buf(true, true).to_string().len().try_into().unwrap()
}
