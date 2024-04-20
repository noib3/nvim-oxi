use crate::Buffer;
use crate::ToFunction;

/// Arguments passed to the callback registered to
/// [`on_lines`](BufAttachOptsBuilder::on_lines). The `(a, b, c, d, e, f, g, h,
/// i)` tuple represents:
///
/// - `a`: the string literal `"lines"`;
/// - `b`: the [`Buffer`] that triggered the callback;
/// - `c`: the value of the buffer-local `b:changedtick` variable;
/// - `d`: first row that changed (0-indexed);
/// - `e`: last row that was changed;
/// - `f`: last row in the updated range;
/// - `g`: byte count of previous contents;
/// - `h`: deleted UTF-32 codepoints (if
/// [`utf_sizes`](BufAttachOptsBuilder::utf_sizes) was `true`);
/// - `i`: deleted UTF-16 codeunits (if
/// [`utf_sizes`](BufAttachOptsBuilder::utf_sizes) was `true`);
pub type OnLinesArgs = (
    String,
    Buffer,
    u32,
    usize,
    usize,
    usize,
    usize,
    Option<usize>,
    Option<usize>,
);

/// Arguments passed to the callback registered to [`on_bytes`](BufAttachOptsBuilder::on_bytes). The `(a, b, c, d, e, f, g, h, i, j, k, l)`
/// - `a`: the string literal `"bytes"`;
/// - `b`: the [`Buffer`] that triggered the callback;
/// - `c`: the value of the buffer-local `b:changedtick` variable;
/// - `d`: start row of the changed text (0-indexed);
/// - `e`: start column of the changed text;
/// - `f`: byte offset of the changed text from the start of the buffer;
/// - `g`: number of rows deleted;
/// - `h`: number of columns deleted;
/// - `i`: number of bytes deleted;
/// - `j`: number of rows added;
/// - `k`: number of columns added;
/// - `l`: number of bytes added;
pub type OnBytesArgs = (
    String,
    Buffer,
    u32,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
);

/// Arguments passed to the callback registered to
/// [`on_changedtick`](BufAttachOptsBuilder::on_changedtick). The first tuple
/// element is the string literal `"changedtick"`, the second is the [`Buffer`]
/// that triggered the callback and the third is current value of the
/// buffer-local
/// [`b:changedtick`](https://neovim.io/doc/user/eval.html#b:changedtick)
/// variable.
pub type OnChangedtickArgs = (String, Buffer, u32);

/// Arguments passed to the callback registered to
/// [`on_detach`](BufAttachOptsBuilder::on_detach). The first tuple element is
/// the string literal `"detach"`, the second is the [`Buffer`] that triggered
/// the callback.
pub type OnDetachArgs = (String, Buffer);

/// Arguments passed to the callback registered to
/// [`on_reload`](BufAttachOptsBuilder::on_reload). The first tuple element is
/// the string literal `"reload"`, the second is the [`Buffer`] that triggered
/// the callback.
pub type OnReloadArgs = (String, Buffer);

/// All the registered callbacks can detach by returning `true`, as described
/// in `:h api-lua-detach`.
pub type ShouldDetach = bool;

/// Options passed to [`Buffer::attach`](crate::Buffer::attach).
#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Debug, Default)]
pub struct BufAttachOpts {
    on_bytes: types::Object,
    on_changedtick: types::Object,
    on_detach: types::Object,
    on_lines: types::Object,
    on_reload: types::Object,
    preview: types::Object,
    utf_sizes: types::Object,
}

#[cfg(not(feature = "neovim-nightly"))]
impl BufAttachOpts {
    #[inline(always)]
    /// Creates a new [`BufAttachOptsBuilder`].
    pub fn builder() -> BufAttachOptsBuilder {
        BufAttachOptsBuilder::default()
    }
}

#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Default)]
pub struct BufAttachOptsBuilder(BufAttachOpts);

#[cfg(not(feature = "neovim-nightly"))]
impl BufAttachOptsBuilder {
    /// Callback invoked on change. It receives more granular information about
    /// the change compared to [`on_lines`](BufAttachOptsBuilder::on_lines).
    #[inline]
    pub fn on_bytes<F>(&mut self, on_bytes: F) -> &mut Self
    where
        F: ToFunction<OnBytesArgs, ShouldDetach>,
    {
        self.0.on_bytes = types::Object::from_luaref(on_bytes.into_luaref());
        self
    }

    /// Callback invoked on changedtick increment without text change.
    #[inline]
    pub fn on_changedtick<F>(&mut self, on_changedtick: F) -> &mut Self
    where
        F: ToFunction<OnChangedtickArgs, ShouldDetach>,
    {
        self.0.on_changedtick =
            types::Object::from_luaref(on_changedtick.into_luaref());
        self
    }

    /// Callback invoked on detach.
    #[inline]
    pub fn on_detach<F>(&mut self, on_detach: F) -> &mut Self
    where
        F: ToFunction<OnDetachArgs, ShouldDetach>,
    {
        self.0.on_detach = types::Object::from_luaref(on_detach.into_luaref());
        self
    }

    /// Callback invoked on change.
    #[inline]
    pub fn on_lines<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnLinesArgs, ShouldDetach>,
    {
        self.0.on_lines = types::Object::from_luaref(fun.into_luaref());
        self
    }

    /// Callback invoked on reload. The entire buffer content should be
    /// considered changed.
    #[inline]
    pub fn on_reload<F>(&mut self, on_reload: F) -> &mut Self
    where
        F: ToFunction<OnReloadArgs, ShouldDetach>,
    {
        self.0.on_reload = types::Object::from_luaref(on_reload.into_luaref());
        self
    }

    /// Whether to also attach to command preview (i.e.
    /// [`inccommand`](https://neovim.io/doc/user/options.html#'inccommand'))
    /// events.
    #[inline]
    pub fn preview(&mut self, preview: bool) -> &mut Self {
        self.0.preview = preview.into();
        self
    }

    /// Whether to include the UTF-32 and UTF-16 sizes of the replaced region
    /// as the last arguments of the
    /// [`on_lines`](BufAttachOptsBuilder::on_lines) callback.
    #[inline]
    pub fn utf_sizes(&mut self, utf_sizes: bool) -> &mut Self {
        self.0.utf_sizes = utf_sizes.into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> BufAttachOpts {
        std::mem::take(&mut self.0)
    }
}

#[cfg(not(feature = "neovim-nightly"))]
impl From<&BufAttachOpts> for types::Dictionary {
    #[inline]
    fn from(opts: &BufAttachOpts) -> Self {
        Self::from_iter([
            ("on_bytes", opts.on_bytes.clone()),
            ("on_changedtick", opts.on_changedtick.clone()),
            ("on_detach", opts.on_detach.clone()),
            ("on_lines", opts.on_lines.clone()),
            ("on_reload", opts.on_reload.clone()),
            ("preview", opts.preview.clone()),
            ("utf_sizes", opts.utf_sizes.clone()),
        ])
    }
}

/// Options passed to [`Buffer::attach`](crate::Buffer::attach).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct BufAttachOpts {
    #[builder(mask)]
    mask: u64,

    /// Callback invoked on change.
    #[builder(
        generics = "F: ToFunction<OnLinesArgs, ShouldDetach>",
        argtype = "F",
        inline = "{0}.into_luaref()"
    )]
    on_lines: types::LuaRef,

    /// Callback invoked on change. It receives more granular information about
    /// the change compared to [`on_lines`](BufAttachOptsBuilder::on_lines).
    #[builder(
        generics = "F: ToFunction<OnBytesArgs, ShouldDetach>",
        argtype = "F",
        inline = "{0}.into_luaref()"
    )]
    on_bytes: types::LuaRef,

    /// Callback invoked on changedtick increment without text change.
    #[builder(
        generics = "F: ToFunction<OnChangedtickArgs, ShouldDetach>",
        argtype = "F",
        inline = "{0}.into_luaref()"
    )]
    on_changedtick: types::LuaRef,

    /// Callback invoked on detach.
    #[builder(
        generics = "F: ToFunction<OnDetachArgs, ShouldDetach>",
        argtype = "F",
        inline = "{0}.into_luaref()"
    )]
    on_detach: types::LuaRef,

    /// Callback invoked on reload. The entire buffer content should be
    /// considered changed.
    #[builder(
        generics = "F: ToFunction<OnReloadArgs, ShouldDetach>",
        argtype = "F",
        inline = "{0}.into_luaref()"
    )]
    on_reload: types::LuaRef,

    /// Whether to include the UTF-32 and UTF-16 sizes of the replaced region
    /// as the last arguments of the
    /// [`on_lines`](BufAttachOptsBuilder::on_lines) callback.
    #[builder(argtype = "bool")]
    utf_sizes: types::Boolean,

    /// Whether to also attach to command preview (i.e.
    /// [`inccommand`](https://neovim.io/doc/user/options.html#'inccommand'))
    /// events.
    #[builder(argtype = "bool")]
    preview: types::Boolean,
}
