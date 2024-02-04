use crate::Buffer;
use crate::ToFunction;

/// Arguments passed to the callback registered to
/// [`on_input`](OpenTermOptsBuilder::on_input). The `(a, b, c, d)` tuple
/// represents:
///
/// - `a`: the string literal `"input"`;
/// - `b`: channel id;
/// - `c`: the [`Buffer`] associated to the terminal instance;
/// - `d`: data input.
pub type OnInputArgs = (
    String,        // the string literal `"input"`
    u32,           // channel_id
    Buffer,        // buffer
    types::String, // data input
);

/// Options passed to [`open_term()`](crate::open_term).
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct OpenTermOpts {
    #[cfg(feature = "neovim-nightly")]
    #[builder(mask)]
    mask: u64,

    #[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
    #[builder(
        generics = "F: ToFunction<OnInputArgs, ()>",
        argtype = "F",
        inline = "types::Object::from_luaref({0}.into_luaref())"
    )]
    /// Callback invoked on data input (like keypresses in terminal mode).
    on_input: types::Object,

    #[cfg(feature = "neovim-nightly")]
    #[builder(
        generics = "F: ToFunction<OnInputArgs, ()>",
        argtype = "F",
        inline = "{0}.into_luaref()"
    )]
    /// Callback invoked on data input (like keypresses in terminal mode).
    on_input: types::LuaRef,
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl From<&OpenTermOpts> for types::Dictionary {
    fn from(opts: &OpenTermOpts) -> Self {
        Self::from_iter([("on_input", opts.on_input.clone())])
    }
}
