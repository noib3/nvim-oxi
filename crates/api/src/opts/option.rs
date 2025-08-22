use serde::Serialize;
use types::conversion::FromObject;

use crate::{Buffer, Window};

/// Options passed to [`get_option_value()`](crate::get_option_value) and
/// [`set_option_value()`](crate::set_option_value).
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct OptionOpts {
    #[builder(mask)]
    mask: u64,

    #[builder(argtype = "OptionScope", inline = "{0}.into()")]
    scope: types::String,

    #[builder(argtype = "Window", inline = "{0}.0")]
    win: types::WinHandle,

    #[builder(argtype = "Buffer", inline = "{0}.0")]
    buf: types::BufHandle,

    #[builder(
        generics = "F: Into<types::String>",
        argtype = "F",
        inline = "{0}.into()"
    )]
    filetype: types::String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OptionScope {
    Global,
    Local,
}

impl From<OptionScope> for types::String {
    #[inline]
    fn from(ctx: OptionScope) -> Self {
        types::String::from_object(
            ctx.serialize(types::serde::Serializer::new())
                .expect("`OptionScope` is serializable"),
        )
        .expect("`OptionScope` is serialized into a string")
    }
}
