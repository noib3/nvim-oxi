use oxi_types as types;
#[cfg(not(feature = "neovim-nightly"))]
use oxi_types::Object;
#[cfg(feature = "neovim-nightly")]
use oxi_types::{Boolean, LuaRef};

use crate::ToFunction;

/// Options passed to [`Buffer::set_keymap()`](crate::Buffer::set_keymap)
/// and [`set_keymap()`](crate::set_keymap).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default, PartialEq, oxi_macros::OptsBuilder)]
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

/// Options passed to [`Buffer::set_keymap()`](crate::Buffer::set_keymap)
/// and [`set_keymap()`](crate::set_keymap).
#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
#[derive(Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct SetKeymapOpts {
    desc: Object,
    expr: Object,
    script: Object,
    silent: Object,
    unique: Object,
    nowait: Object,
    noremap: Object,
    callback: Object,
    replace_keycodes: Object,
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl SetKeymapOpts {
    #[inline(always)]
    /// Creates a new [`SetKeymapOptsBuilder`].
    pub fn builder() -> SetKeymapOptsBuilder {
        SetKeymapOptsBuilder::default()
    }
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
#[derive(Clone, Default)]
pub struct SetKeymapOptsBuilder(SetKeymapOpts);

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl SetKeymapOptsBuilder {
    /// A function to call when the mapping is executed.
    #[inline]
    pub fn callback<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<(), ()>,
    {
        let callback = fun.into_luaref();
        self.0.callback = Object::from_luaref(callback);
        self
    }

    /// A description for the keymap.
    #[inline]
    pub fn desc(&mut self, desc: &str) -> &mut Self {
        let desc = types::String::from(desc);
        self.0.desc = desc.into();
        self
    }

    /// Whether the keymap argument is an expression.
    #[inline]
    pub fn expr(&mut self, expr: bool) -> &mut Self {
        self.0.expr = expr.into();
        self
    }

    /// Whether the right-hand side of the mapping shouldn't be remappable.
    #[inline]
    pub fn noremap(&mut self, noremap: bool) -> &mut Self {
        self.0.noremap = noremap.into();
        self
    }

    /// For buffer-local mappings, whether Neovim should wait for more
    /// characters to be typed if there's a global mapping that could also
    /// match. See `:h map-nowait` for more details.
    #[inline]
    pub fn nowait(&mut self, nowait: bool) -> &mut Self {
        self.0.nowait = nowait.into();
        self
    }

    /// When [`expr`](SetKeymapOptsBuilder::expr) is `true`, this option can be
    /// used to replace the keycodes in the resulting string (see
    /// [replace_termcodes()](crate::replace_termcodes)).
    #[inline]
    pub fn replace_keycodes(&mut self, replace_keycodes: bool) -> &mut Self {
        self.0.replace_keycodes = replace_keycodes.into();
        self
    }

    /// Whether to remap characters in the right-hand side by expanding the
    /// `<sid>` script tag.
    #[inline]
    pub fn script(&mut self, script: bool) -> &mut Self {
        self.0.script = script.into();
        self
    }

    /// Whether the keymap should be silent.
    #[inline]
    pub fn silent(&mut self, silent: bool) -> &mut Self {
        self.0.silent = silent.into();
        self
    }

    /// If `true` setting the keymap fill fail if another keymap with the same
    /// left-hand side already exists.
    #[inline]
    pub fn unique(&mut self, unique: bool) -> &mut Self {
        self.0.unique = unique.into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> SetKeymapOpts {
        std::mem::take(&mut self.0)
    }
}
