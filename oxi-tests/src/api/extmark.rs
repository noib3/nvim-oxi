use all_asserts::*;
use nvim_oxi::api::opts::*;
use nvim_oxi::api::{self, Buffer};

pub fn add_highlight() {
    let mut buf = Buffer::current();
    let id = api::create_namespace("Foo");
    let res = buf.add_highlight(id, "Normal", 0, 0, 1);
    assert!(res.is_ok(), "{res:?}");
}

pub fn clear_namespace() {
    let mut buf = Buffer::current();
    let id = api::create_namespace("Foo");
    let res = buf.clear_namespace(id, 0, -1);
    assert_eq!(Ok(()), res);
}

pub fn get_namespaces() {
    let id = api::create_namespace("Foo");

    let out = api::get_namespaces()
        .find_map(|(name, id)| (name == "Foo").then(|| id))
        .unwrap();

    assert_eq!(id, out);
}
