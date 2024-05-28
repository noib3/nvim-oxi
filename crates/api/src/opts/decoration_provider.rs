use types::Object;

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
/// [`set_decoration_provider()`](crate::set_decoration_provider).
#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct DecorationProviderOpts {
    on_buf: Object,
    on_end: Object,
    on_win: Object,
    on_line: Object,
    on_start: Object,
    _on_hl_def: Object,
    _on_spell_nav: Object,
}

/// Options passed to
/// [`set_decoration_provider()`](crate::set_decoration_provider).
#[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct DecorationProviderOpts {
    on_start: Object,
    on_buf: Object,
    on_win: Object,
    on_line: Object,
    on_end: Object,
    _on_hl_def: Object,
    _on_spell_nav: Object,
}

impl DecorationProviderOpts {
    #[inline(always)]
    /// Creates a new [`DecorationProviderOptsBuilder`].
    pub fn builder() -> DecorationProviderOptsBuilder {
        DecorationProviderOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct DecorationProviderOptsBuilder(DecorationProviderOpts);

impl DecorationProviderOptsBuilder {
    #[inline]
    pub fn on_buf<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnBufArgs, ()>,
    {
        self.0.on_buf = Object::from_luaref(fun.into_luaref());
        self
    }

    #[inline]
    pub fn on_end<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnEndArgs, ()>,
    {
        self.0.on_end = Object::from_luaref(fun.into_luaref());
        self
    }

    #[inline]
    pub fn on_line<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnLineArgs, ()>,
    {
        self.0.on_line = Object::from_luaref(fun.into_luaref());
        self
    }

    #[inline]
    pub fn on_start<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnStartArgs, DontSkipRedrawCycle>,
    {
        self.0.on_start = Object::from_luaref(fun.into_luaref());
        self
    }

    #[inline]
    pub fn on_win<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnWinArgs, DontSkipOnLines>,
    {
        self.0.on_win = Object::from_luaref(fun.into_luaref());
        self
    }

    #[inline]
    pub fn build(&mut self) -> DecorationProviderOpts {
        std::mem::take(&mut self.0)
    }
}
