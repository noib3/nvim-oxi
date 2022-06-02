pub mod api;
mod error;
mod lua;
mod macros;
mod object;
mod toplevel;

pub use error::{Error, Result};
pub use toplevel::*;

// #[no_mangle]
// pub extern "C" fn test() -> *mut std::os::raw::c_char {
//     api::get_mode()
//         .get::<_, nvim_types::NvimString>("mode")
//         .unwrap()
//         .as_c_str()
//         .to_owned()
//         .into_raw()
// }

// #[no_mangle]
// pub extern "C" fn is_modified() -> bool {
//     api::get_current_buf().get_option::<bool>("modified").unwrap()
// }

// #[no_mangle]
// pub extern "C" fn set_lines() {
//     api::create_buf(true, false)
//         .unwrap()
//         .set_lines(0, 0, false, ["foo", "bar", "baz"])
//         .unwrap()
// }

// #[no_mangle]
// pub extern "C" fn set_option() -> bool {
//     let mut buf = api::get_current_buf();
//     buf.set_option("modified", true).unwrap();
//     buf.get_option::<bool>("modified").unwrap()
// }

// #[no_mangle]
// pub extern "C" fn set_var() -> bool {
//     let mut buf = api::get_current_buf();
//     buf.set_var("foo", true).unwrap();
//     buf.get_var::<bool>("foo").unwrap()
// }

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
use crate::api::Buffer;

#[no_mangle]
extern "C" fn luaopen_libnvim_oxi(lstate: *mut lua::lua_State) -> libc::c_int {
    lua::init_state(lstate);

    // let buf = api::create_buf(true, false).unwrap();
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

    // let foo = String::from("foo");

    // crate::schedule(move |_| {
    //     crate::print!("{foo}");
    //     Ok(())
    // });

    // let lines = Buffer::from(0).get_lines(0, 4, false).unwrap();
    // for line in lines {
    //     crate::print!("{}", line.to_string_lossy());
    // }

    // let mut buf = api::get_current_buf();

    // buf.set_mark("f", 18, 154).unwrap();
    // crate::print!("{:?}", buf.get_mark("f"));

    // let path = Buffer::current().get_name().unwrap();
    // crate::print!("{:?}", path);
    // crate::print!("{:?}", path.components());

    // let opts =
    //     api::global::opts::UserCommandOptsBuilder::default().build().unwrap();

    // crate::print!("{:?}", nvim_types::dictionary::Dictionary::from(opts));

    // let opts =
    //     api::global::opts::CreateCommandOpts::builder().build().unwrap();

    // crate::print!(
    //     "{:?}",
    //     Buffer::current()
    //         .get_keymap(api::Mode::Normal)
    //         .map(|iter| iter.collect::<Vec<api::types::KeymapInfos>>())
    // );

    use nvim_types::object::Object;

    use crate::object::{FromObject, ToObject};

    let obj = nvim_types::array::Array::from_iter([
        Object::from(true),
        Object::from(true),
        Object::from(true),
        // Object::from("hey"),
        // Object::from(17),
    ]);

    // let des = <(bool, bool, bool)>::from_obj(obj.into());
    // let des = <[bool; 3]>::from_obj(obj.into());

    // #[derive(serde::Deserialize, Debug)]
    // struct Test {
    //     foo: String,
    //     bar: Option<u32>,
    //     foor: Foo,
    // }

    // #[derive(serde::Deserialize, Debug)]
    // struct Foo {
    //     ciao: bool,
    // }

    // let dict = Object::from_iter([
    //     ("foo", Object::from("Ciaone")),
    //     ("bar", 32.into()),
    //     ("foor", Object::from_iter([("ciao", true)])),
    // ]);

    #[derive(serde::Deserialize, Debug, serde::Serialize)]
    #[serde(rename_all = "lowercase")]
    enum Foo {
        Foo,
        Bar,
        Baz,
    }

    let des = Foo::from_obj("foo".into());

    let foo = Foo::Foo;
    // let ser = foo.to_obj();

    crate::print!("{des:?}");
    // crate::print!("{ser:?}");

    0
}

// use nvim_oxi as nvim;

// fn cool_shit() -> nvim::Result<()> {
//     let mut i = 0;
//     let timer = nvim::r#loop::Timer::new();
//     timer.start(0, 1000, move || {
//         let msg =
//             if i % 2 == 0 { "Try doing this with RPC" } else { "Good luck!" };
//         nvim::print!("{msg}");
//         i += 1;
//         (i == 10).then(|| true)
//     })
// }
