use crate::Buffer;
use crate::StringOrInt;

/// Options passed to [`clear_autocmds()`](crate::clear_autocmds).
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct ClearAutocmdsOpts {
    #[builder(mask)]
    mask: u64,

    /// Only clear the autocommands local to a specific `Buffer`. Cannot be
    /// used together with [`patterns`](ClearAutocmdsOptsBuilder::patterns).
    #[builder(argtype = "Buffer", inline = "{0}.0")]
    buffer: types::BufHandle,

    /// Clear all the autocommands triggered by one or more of the specified
    /// events.
    #[builder(
        generics = "'a, I: IntoIterator<Item = &'a str>",
        method = "events",
        argtype = "I",
        inline = "types::Array::from_iter({0}).into()"
    )]
    event: types::Object,

    /// Only clear the autocommands matching specific patterns. For example, if
    /// you have `"*.py"` as a pattern for a particular autocommand, you must
    /// pass that exact pattern to clear it. Cannot be used together with
    /// [`buffer`](ClearAutocmdsOptsBuilder::buffer).
    #[builder(
        generics = "G: StringOrInt",
        argtype = "G",
        inline = "{0}.to_object()"
    )]
    group: types::Object,

    /// Only clear the autocommands belonging to a specific augroup. The
    /// augroup can be specified by both id and name.
    #[builder(
        generics = "'a, I: IntoIterator<Item = &'a str>",
        method = "patterns",
        argtype = "I",
        inline = "types::Array::from_iter({0}).into()"
    )]
    pattern: types::Object,
}
