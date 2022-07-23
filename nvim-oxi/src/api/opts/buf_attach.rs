use derive_builder::Builder;
use nvim_types::{Dictionary, Object};

use crate::api::Buffer;
use crate::trait_utils::ToFunction;

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

/// Options passed to [`Buffer::attach`](crate::api::Buffer::attach).
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct BufAttachOpts {
    #[builder(setter(custom))]
    on_bytes: Object,

    #[builder(setter(custom))]
    on_changedtick: Object,

    #[builder(setter(custom))]
    on_detach: Object,

    #[builder(setter(custom))]
    on_lines: Object,

    #[builder(setter(custom))]
    on_reload: Object,

    /// Whether to also attach to command preview (i.e.
    /// [`inccommand`](https://neovim.io/doc/user/options.html#'inccommand'))
    /// events.
    preview: bool,

    /// Whether to include the UTF-32 and UTF-16 sizes of the replaced region
    /// as the last arguments of the
    /// [`on_lines`](BufAttachOptsBuilder::on_lines) callback.
    utf_sizes: bool,
}

impl BufAttachOpts {
    #[inline(always)]
    /// Creates a new [`BufAttachOptsBuilder`].
    pub fn builder() -> BufAttachOptsBuilder {
        BufAttachOptsBuilder::default()
    }
}

impl BufAttachOptsBuilder {
    /// Callback invoked on change. It receives more granular information about
    /// the change compared to [`on_lines`](BufAttachOptsBuilder::on_lines).
    pub fn on_bytes<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnBytesArgs, ShouldDetach>,
    {
        self.on_bytes = Some(fun.to_obj());
        self
    }

    /// Callback invoked on changedtick increment without text change.
    pub fn on_changedtick<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnChangedtickArgs, ShouldDetach>,
    {
        self.on_changedtick = Some(fun.to_obj());
        self
    }

    /// Callback invoked on detach.
    pub fn on_detach<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnDetachArgs, ShouldDetach>,
    {
        self.on_detach = Some(fun.to_obj());
        self
    }

    /// Callback invoked on change.
    pub fn on_lines<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnLinesArgs, ShouldDetach>,
    {
        self.on_lines = Some(fun.to_obj());
        self
    }

    /// Callback invoked on reload. The entire buffer content should be
    /// considered changed.
    pub fn on_reload<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnReloadArgs, ShouldDetach>,
    {
        self.on_reload = Some(fun.to_obj());
        self
    }

    pub fn build(&mut self) -> BufAttachOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl From<&BufAttachOpts> for Dictionary {
    fn from(opts: &BufAttachOpts) -> Self {
        // TODO: don't clone by making non-owning version of Dictionary
        Self::from_iter([
            ("on_bytes", opts.on_bytes.clone()),
            ("on_changedtick", opts.on_changedtick.clone()),
            ("on_detach", opts.on_detach.clone()),
            ("on_lines", opts.on_lines.clone()),
            ("on_reload", opts.on_reload.clone()),
            ("preview", opts.preview.into()),
            ("utf_sizes", opts.utf_sizes.into()),
        ])
    }
}
