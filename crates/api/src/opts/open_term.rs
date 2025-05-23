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
    #[builder(mask)]
    mask: u64,

    #[builder(
        generics = "F: ToFunction<OnInputArgs, ()>",
        argtype = "F",
        inline = "{0}.into_luaref()"
    )]
    /// Callback invoked on data input (like keypresses in terminal mode).
    on_input: types::LuaRef,
}
