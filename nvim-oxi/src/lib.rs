pub mod api;
mod error;
mod lua;
mod toplevel;

pub use api::Buffer;
pub use error::Error;
pub use toplevel::*;

pub type Result<T> = std::result::Result<T, Error>;

use lua::LUA;

#[no_mangle]
pub extern "C" fn test() -> *mut std::os::raw::c_char {
    api::get_mode()
        .get::<_, nvim_types::NvimString>("mode")
        .unwrap()
        .as_c_str()
        .to_owned()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn is_modified() -> bool {
    api::get_current_buf().get_option::<bool>("modified").unwrap()
}

#[no_mangle]
pub extern "C" fn set_lines() {
    api::create_buf(true, false)
        .unwrap()
        .set_lines(0, 0, false, ["ciaone"])
        .unwrap()
}

#[no_mangle]
pub extern "C" fn set_option() -> bool {
    let mut buf = api::get_current_buf();
    buf.set_option("modified", true).unwrap();
    buf.get_option::<bool>("modified").unwrap()
}

#[no_mangle]
pub extern "C" fn set_var() -> bool {
    let mut buf = api::get_current_buf();
    buf.set_var("foo", true).unwrap();
    buf.get_var::<bool>("foo").unwrap()
}

#[no_mangle]
pub extern "C" fn buf_call() -> bool {
    let buf = api::get_current_buf();

    buf.call(|| {
        let buf = api::get_current_buf();
        println!("{:?}", buf.get_option::<bool>("modifiable"))
    })
    .is_err()
}

#[no_mangle]
extern "C" fn luaopen_libnvim_oxi(lstate: *mut lua::lua_State) -> libc::c_int {
    LUA.with(|lua| lua.set(lstate).expect("couldn't initialize Lua state"));

    let buf = api::get_current_buf();

    toplevel::print!(
        "{:?}",
        buf.call(|| {
            let buf = api::get_current_buf();
            toplevel::print!("This is \"{}\"", buf.get_name());
        })
    );

    // toplevel::print!("Hello {planet}!", planet = "Mars");

    0
}
