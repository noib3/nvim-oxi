pub mod api;
mod error;
mod lua;
mod toplevel;

pub use api::Buffer;
pub use error::Error;
pub use toplevel::*;

pub type Result<T> = std::result::Result<T, Error>;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L41
pub(crate) const INTERNAL_CALL_MASK: u64 =
    1u64 << (std::mem::size_of::<u64>() * 8 - 1);

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L44
pub(crate) const VIML_INTERNAL_CALL: u64 = INTERNAL_CALL_MASK;

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L47
pub(crate) const LUA_INTERNAL_CALL: u64 = VIML_INTERNAL_CALL + 1;

// #[no_mangle]
// pub extern "C" fn test() -> *mut std::os::raw::c_char {
//     api::get_mode()
//         .get::<_, nvim_types::NvimString>("mode")
//         .unwrap()
//         .as_c_str()
//         .to_owned()
//         .into_raw()
// }

#[no_mangle]
pub extern "C" fn is_modified() -> bool {
    api::get_current_buf().get_option::<bool>("modified").unwrap()
}

#[no_mangle]
pub extern "C" fn set_lines() {
    api::create_buf(true, false)
        .unwrap()
        .set_lines(0, 0, false, ["foo", "bar", "baz"])
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

// #[no_mangle]
// pub extern "C" fn buf_call() -> bool {
//     let buf = api::get_current_buf();

//     buf.call(|| {
//         let buf = api::get_current_buf();
//         println!("{:?}", buf.get_option::<bool>("modifiable"));
//         Ok(())
//     })
//     .is_err()
// }

#[no_mangle]
extern "C" fn luaopen_libnvim_oxi(
    lstate: *mut lua::ffi::lua_State,
) -> libc::c_int {
    lua::init_state(lstate);

    // let mut buf = api::create_buf(true, false).unwrap();
    // buf.set_option("modified", true).unwrap();

    // let ciao = String::from("nope");
    // let is_modified =
    //     buf.call(move |_| Buffer::from(0).get_option::<bool>(&ciao));

    // toplevel::print!("{buf:?} is modified? {is_modified:?} YEAAAAAAA");

    // let _ = Buffer::from(0).call(|_| {
    //     let buf = api::get_current_buf();
    //     toplevel::print!("This is \"{}\", uhuhuh!!", buf.get_name().unwrap());
    //     Ok(())
    // });

    // use crate::api::buffer::opts::BufAttachOptsBuilder;

    // let on_bytes = |args| {
    //     crate::print!("on_bytes: {:?}", args);
    //     Ok(false)
    // };

    // let on_lines = |args| {
    //     crate::print!("on_lines: {:?}", args);
    //     Ok(false)
    // };

    // let opts = BufAttachOptsBuilder::default()
    //     .on_bytes(on_bytes)
    //     .on_lines(on_lines)
    //     .build()
    //     .unwrap();

    // let has_attached = Buffer::from(0).attach(false, opts);
    // crate::print!("{has_attached:?}");

    let foo = String::from("foo");

    crate::schedule(move |()| {
        crate::print!("{foo}");
        Ok(())
    });

    0
}

// use nvim_oxi as nvim;

// fn cool_shit() -> nvim::Result<()> {
//     let mut i = 0;
//     let timer = nvim::r#loop::new_timer();
//     timer.start(0, 1000, move || {
//         let msg =
//             if i % 2 == 0 { "Try doing this with RPC" } else { "Good luck!" };
//         nvim::print!("{msg}");
//         i += 1;
//         (i == 10).then(|| true)
//     })
// }
