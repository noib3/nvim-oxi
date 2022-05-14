pub mod api;

#[no_mangle]
pub extern "C" fn test() -> i32 {
    api::get_current_buf().into()
}
