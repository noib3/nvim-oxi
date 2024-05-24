use types::{String as NvimString, *};

/// Option passed to [`get_hl()`][crate::get_hl].
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct GetHighlightOpts {
    #[builder(mask)]
    mask: u64,

    /// Get a highlight definition by id.
    #[builder(argtype = "u32", inline = "{0} as Integer")]
    id: Integer,

    /// Get a highlight definition by name.
    #[builder(
        generics = "S: Into<types::String>",
        argtype = "S",
        inline = "{0}.into()"
    )]
    name: NvimString,

    /// Show linked group name instead of effective definition (default is
    /// `true`).
    #[builder(argtype = "bool")]
    link: Boolean,

    /// When highlight group doesn't exist create it (default is
    /// `true`).
    #[builder(argtype = "bool")]
    create: Boolean,
}
