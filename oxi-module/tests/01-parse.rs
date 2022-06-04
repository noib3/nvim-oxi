#![allow(dead_code)]

use oxi_derive::ToObject;

#[derive(ToObject)]
pub struct UserCommandOpts {
    desc: Option<String>,
    bang: bool,
    bar: bool,
    keepscript: bool,
    register: bool,
}

// impl ::nvim_oxi::ToObject for UserCommandOpts {
//     fn to_object(self) -> ::nvim_oxi::Object {
//         ::nvim_oxi::Object {
//             r#type: ::nvim_oxi::object::
//         }
//     }
// }

fn main() {}
