use crate::Buffer;
use crate::{StringOrInt, StringOrListOfStrings};

/// Options passed to [`exec_autocmds()`](crate::exec_autocmds).
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct ExecAutocmdsOpts {
    #[builder(mask)]
    mask: u64,

    /// A specific [`Buffer`] for buffer-local autocommands. Cannot be used
    /// together with [`patterns`](ExecAutocmdsOptsBuilder::patterns).
    #[builder(argtype = "Buffer", inline = "{0}.0")]
    buffer: types::BufHandle,

    /// The autocommand group name or id to match against.
    #[builder(
        generics = "G: StringOrInt",
        argtype = "G",
        inline = "{0}.to_object()"
    )]
    group: types::Object,

    /// Whether to process the modeline after the autocommands.
    #[builder(argtype = "bool")]
    modeline: types::Boolean,

    /// Patterns to match against. Cannot be used together with
    /// [`buffer`](ExecAutocmdsOptsBuilder::buffer).
    #[builder(
        generics = "P: StringOrListOfStrings",
        method = "patterns",
        argtype = "P",
        inline = "{0}.to_object()"
    )]
    pattern: types::Object,

    #[builder(
        generics = "D: Into<types::Object>",
        argtype = "D",
        inline = "{0}.into()"
    )]
    data: types::Object,
}
