use derive_builder::Builder;
#[cfg(not(feature = "neovim-0-7"))]
use nvim_types::NonOwning;
use nvim_types::Object;

use crate::ToFunction;
use crate::{Buffer, Window};

// NOTE: docs say a third argument of changedtick is passed. I don't see it.
/// Arguments passed to the function registered to
/// [`on_buf`](DecorationProviderOptsBuilder::on_buf).
pub type OnBufArgs = (
    String, // the string literal "buf"
    Buffer, // buffer
);

/// Arguments passed to the function registered to
/// [`on_end`](DecorationProviderOptsBuilder::on_end).
pub type OnEndArgs = (
    String, // the string literal "end"
    u32,    // changedtick
);

/// Arguments passed to the function registered to
/// [`on_line`](DecorationProviderOptsBuilder::on_line).
pub type OnLineArgs = (
    String, // the string literal "win"
    Window, // window
    Buffer, // buffer
    usize,  // row
);

/// Arguments passed to the function registered to
/// [`on_start`](DecorationProviderOptsBuilder::on_start).
pub type OnStartArgs = (
    String, // the string literal "start"
    u32,    // changedtick
    u32, /* `type`, undocumented? (https://github.com/neovim/neovim/blob/master/src/nvim/decoration_provider.c#L68) */
);

/// Arguments passed to the function registered to
/// [`on_win`](DecorationProviderOptsBuilder::on_win).
pub type OnWinArgs = (
    String, // the string literal "win"
    Window, // window
    Buffer, // buffer
    u32,    // topline
    u32,    // botline guess
);

/// The `on_start` callback can return `false` to disable the provider until
/// the next redraw.
pub type DontSkipRedrawCycle = bool;

/// The `on_win` callback can return `false` to skip the `on_line` callback for
/// that window.
pub type DontSkipOnLines = bool;

/// Options passed to
/// [`nvim_oxi::api::set_decoration_provider`](crate::set_decoration_provider).
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct DecorationProviderOpts {
    #[builder(setter(custom))]
    on_buf: Object,

    #[builder(setter(custom))]
    on_end: Object,

    #[cfg(not(feature = "neovim-0-7"))]
    #[builder(setter(skip))]
    on_hl_def: Object,

    #[builder(setter(custom))]
    on_line: Object,

    #[cfg(not(feature = "neovim-0-7"))]
    #[builder(setter(skip))]
    on_spell_nav: Object,

    #[builder(setter(custom))]
    on_start: Object,

    #[builder(setter(custom))]
    on_win: Object,
}

impl DecorationProviderOpts {
    #[inline(always)]
    /// Creates a new [`DecorationProviderOptsBuilder`].
    pub fn builder() -> DecorationProviderOptsBuilder {
        DecorationProviderOptsBuilder::default()
    }
}

impl DecorationProviderOptsBuilder {
    pub fn on_buf<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnBufArgs, ()>,
    {
        self.on_buf = Some(fun.to_object());
        self
    }

    pub fn on_end<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnEndArgs, ()>,
    {
        self.on_end = Some(fun.to_object());
        self
    }

    pub fn on_line<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnLineArgs, ()>,
    {
        self.on_line = Some(fun.to_object());
        self
    }

    pub fn on_start<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnStartArgs, DontSkipRedrawCycle>,
    {
        self.on_start = Some(fun.to_object());
        self
    }

    pub fn on_win<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnWinArgs, DontSkipOnLines>,
    {
        self.on_win = Some(fun.to_object());
        self
    }

    pub fn build(&mut self) -> DecorationProviderOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[cfg(feature = "neovim-0-7")]
impl From<&DecorationProviderOpts> for nvim_types::Dictionary {
    fn from(opts: &DecorationProviderOpts) -> Self {
        Self::from_iter([
            ("on_buf", opts.on_buf.clone()),
            ("on_end", opts.on_end.clone()),
            ("on_line", opts.on_line.clone()),
            ("on_start", opts.on_start.clone()),
            ("on_win", opts.on_win.clone()),
        ])
    }
}

#[cfg(not(feature = "neovim-0-7"))]
#[derive(Default)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_set_decoration_provider<'a> {
    on_buf: NonOwning<'a, Object>,
    on_end: NonOwning<'a, Object>,
    on_win: NonOwning<'a, Object>,
    on_line: NonOwning<'a, Object>,
    on_start: NonOwning<'a, Object>,
    _on_hl_def: NonOwning<'a, Object>,
    _on_spell_nav: NonOwning<'a, Object>,
}

#[cfg(not(feature = "neovim-0-7"))]
impl<'a> From<&'a DecorationProviderOpts>
    for KeyDict_set_decoration_provider<'a>
{
    #[inline(always)]
    fn from(opts: &'a DecorationProviderOpts) -> Self {
        Self {
            on_buf: opts.on_buf.non_owning(),
            on_end: opts.on_end.non_owning(),
            on_win: opts.on_win.non_owning(),
            on_line: opts.on_line.non_owning(),
            on_start: opts.on_start.non_owning(),
            _on_hl_def: opts.on_hl_def.non_owning(),
            _on_spell_nav: opts.on_spell_nav.non_owning(),
        }
    }
}
