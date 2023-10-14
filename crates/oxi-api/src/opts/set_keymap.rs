use oxi_types as nvim;
#[cfg(not(feature = "neovim-nightly"))]
use oxi_types::Object;
#[cfg(feature = "neovim-nightly")]
use oxi_types::{Boolean, LuaRef};

use crate::ToFunction;

/// Options passed to [`Buffer::set_keymap()`](crate::Buffer::set_keymap)
/// and [`set_keymap()`](crate::set_keymap).
#[cfg(not(feature = "neovim-nightly"))]
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

/// Options passed to [`Buffer::set_keymap()`](crate::Buffer::set_keymap)
/// and [`set_keymap()`](crate::set_keymap).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct SetKeymapOpts {
    mask: u64,

    /// 7th in the mask.
    noremap: Boolean,

    /// 6th in the mask.
    nowait: Boolean,

    /// 4th in the mask.
    silent: Boolean,

    /// 3rd in the mask.
    script: Boolean,

    /// 2nd in the mask.
    expr: Boolean,

    /// 5th in the mask.
    unique: Boolean,

    /// 8th in the mask.
    callback: LuaRef,

    /// 1st in the mask.
    desc: nvim::String,

    /// 9th in the mask.
    replace_keycodes: Boolean,
}

impl SetKeymapOpts {
    #[inline(always)]
    /// Creates a new [`SetKeymapOptsBuilder`].
    pub fn builder() -> SetKeymapOptsBuilder {
        SetKeymapOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct SetKeymapOptsBuilder(SetKeymapOpts);

impl SetKeymapOptsBuilder {
    /// A function to call when the mapping is executed.
    #[inline]
    pub fn callback<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<(), ()>,
    {
        let callback = fun.into_luaref();

        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.callback = Object::from_luaref(callback);
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.callback = callback;
            self.0.mask |= 0b100000001;
        }

        self
    }

    /// A description for the keymap.
    #[inline]
    pub fn desc(&mut self, desc: &str) -> &mut Self {
        let desc = nvim::String::from(desc);

        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.desc = desc.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.desc = desc;
            self.0.mask |= 0b11;
        }

        self
    }

    /// Whether the keymap argument is an expression.
    #[inline]
    pub fn expr(&mut self, expr: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.expr = expr.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.expr = expr;
            self.0.mask |= 0b101;
        }
        self
    }

    /// Whether the right-hand side of the mapping shouldn't be remappable.
    #[inline]
    pub fn noremap(&mut self, noremap: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.noremap = noremap.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.noremap = noremap;
            self.0.mask |= 0b10000001;
        }
        self
    }

    /// For buffer-local mappings, whether Neovim should wait for more
    /// characters to be typed if there's a global mapping that could also
    /// match. See `:h map-nowait` for more details.
    #[inline]
    pub fn nowait(&mut self, nowait: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.nowait = nowait.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.nowait = nowait;
            self.0.mask |= 0b1000001;
        }
        self
    }

    /// When [`expr`](SetKeymapOptsBuilder::expr) is `true`, this option can be
    /// used to replace the keycodes in the resulting string (see
    /// [replace_termcodes()](crate::replace_termcodes)).
    #[inline]
    pub fn replace_keycodes(&mut self, replace_keycodes: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.replace_keycodes = replace_keycodes.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.replace_keycodes = replace_keycodes;
            self.0.mask |= 0b1000000001;
        }
        self
    }

    /// Whether to remap characters in the right-hand side by expanding the
    /// `<sid>` script tag.
    #[inline]
    pub fn script(&mut self, script: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.script = script.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.script = script;
            self.0.mask |= 0b1001;
        }
        self
    }

    /// Whether the keymap should be silent.
    #[inline]
    pub fn silent(&mut self, silent: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.silent = silent.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.silent = silent;
            self.0.mask |= 0b10001;
        }
        self
    }

    /// If `true` setting the keymap fill fail if another keymap with the same
    /// left-hand side already exists.
    #[inline]
    pub fn unique(&mut self, unique: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.unique = unique.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.unique = unique;
            self.0.mask |= 0b100001;
        }
        self
    }

    #[inline]
    pub fn build(&mut self) -> SetKeymapOpts {
        std::mem::take(&mut self.0)
    }
}
