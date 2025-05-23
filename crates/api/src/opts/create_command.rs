use types::conversion::ToObject;

use crate::types::{
    CommandAddr,
    CommandArgs,
    CommandComplete,
    CommandNArgs,
    CommandRange,
};
use crate::Buffer;

/// Options passed to [`create_user_command`](crate::create_user_command) and
/// [`Buffer::create_user_command()`](crate::Buffer::create_user_command).
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct CreateCommandOpts {
    #[builder(mask)]
    mask: u64,

    #[builder(argtype = "CommandAddr", inline = "{0}.to_object().unwrap()")]
    addr: types::Object,

    #[builder(argtype = "bool")]
    bang: types::Boolean,

    #[builder(argtype = "bool")]
    bar: types::Boolean,

    #[builder(
        argtype = "CommandComplete",
        inline = "{0}.to_object().unwrap()"
    )]
    complete: types::Object,

    // TODO: fix `builder(Into)`.
    #[builder(
        generics = "C: Into<types::Integer>",
        argtype = "C",
        inline = "{0}.into().into()"
    )]
    count: types::Object,

    /// Description for the command.
    #[builder(
        generics = "C: Into<types::String>",
        argtype = "C",
        inline = "{0}.into().into()"
    )]
    desc: types::Object,

    #[builder(argtype = "bool")]
    force: types::Boolean,

    #[builder(argtype = "bool")]
    keepscript: types::Boolean,

    #[builder(argtype = "CommandNArgs", inline = "{0}.to_object().unwrap()")]
    nargs: types::Object,

    #[builder(
        generics = r#"F: Into<types::Function<(CommandArgs, Option<u32>, Option<Buffer>), u8>>"#,
        argtype = "F",
        inline = "{0}.into().into()"
    )]
    preview: types::Object,

    #[builder(argtype = "CommandRange", inline = "{0}.to_object().unwrap()")]
    range: types::Object,

    #[builder(method = "register", argtype = "bool")]
    register_: types::Boolean,
}
