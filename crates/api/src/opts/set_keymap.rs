use types::{Boolean, LuaRef};

use crate::ToFunction;

/// Options passed to [`Buffer::set_keymap()`](crate::Buffer::set_keymap)
/// and [`set_keymap()`](crate::set_keymap).
#[derive(Clone, Debug, Default, PartialEq, macros::OptsBuilder)]
#[repr(C)]
pub struct SetKeymapOpts {
    #[builder(mask)]
    mask: u64,

    /// Whether the right-hand side of the mapping shouldn't be remappable.
    #[builder(argtype = "bool")]
    noremap: Boolean,

    /// For buffer-local mappings, whether Neovim should wait for more
    /// characters to be typed if there's a global mapping that could also
    /// match. See `:h map-nowait` for more details.
    #[builder(argtype = "bool")]
    nowait: Boolean,

    /// Whether the keymap should be silent.
    #[builder(argtype = "bool")]
    silent: Boolean,

    /// Whether to remap characters in the right-hand side by expanding the
    /// `<sid>` script tag.
    #[builder(argtype = "bool")]
    script: Boolean,

    /// Whether the keymap argument is an expression.
    #[builder(argtype = "bool")]
    expr: Boolean,

    /// If `true` setting the keymap fill fail if another keymap with the same
    /// left-hand side already exists.
    #[builder(argtype = "bool")]
    unique: Boolean,

    /// A function to call when the mapping is executed.
    #[builder(
        generics = "F: ToFunction<(), ()>",
        argtype = "F",
        inline = "{0}.into_luaref()"
    )]
    callback: LuaRef,

    /// A description for the keymap.
    #[builder(
        generics = "D: Into<types::String>",
        argtype = "D",
        inline = "{0}.into()"
    )]
    desc: types::String,

    /// When [`expr`](SetKeymapOptsBuilder::expr) is `true`, this option can be
    /// used to replace the keycodes in the resulting string (see
    /// [replace_termcodes()](crate::replace_termcodes)).
    #[builder(argtype = "bool")]
    replace_keycodes: Boolean,
}
