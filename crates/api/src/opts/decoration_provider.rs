use types::LuaRef;

use crate::ToFunction;
use crate::{Buffer, Window};

/// Arguments passed to the function registered to
/// [`on_start`](DecorationProviderOptsBuilder::on_start).
pub type OnStartArgs = (
    String, // the string literal "start"
    u32,    // changedtick
);

// NOTE: docs say a third argument of changedtick is passed. I don't see it.
/// Arguments passed to the function registered to
/// [`on_buf`](DecorationProviderOptsBuilder::on_buf).
pub type OnBufArgs = (
    String, // the string literal "buf"
    Buffer, // buffer
    u32,    // changedtick
);

/// Arguments passed to the function registered to
/// [`on_win`](DecorationProviderOptsBuilder::on_win).
pub type OnWinArgs = (
    String, // the string literal "win"
    Window, // window
    Buffer, // buffer
    u32,    // topline
    u32,    // botline
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
/// [`on_end`](DecorationProviderOptsBuilder::on_end).
pub type OnEndArgs = (
    String, // the string literal "end"
    u32,    // changedtick
);

/// The `on_start` callback can return `false` to disable the provider until
/// the next redraw.
pub type DontSkipRedrawCycle = bool;

/// The `on_win` callback can return `false` to skip the `on_line` callback for
/// that window.
pub type DontSkipOnLines = bool;

/// Options passed to
/// [`set_decoration_provider()`](crate::set_decoration_provider).
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct DecorationProviderOpts {
    #[builder(mask)]
    mask: u64,

    #[builder(
        generics = "F: ToFunction<OnStartArgs, DontSkipRedrawCycle>",
        argtype = "F",
        inline = "{0}.into_luaref()"
    )]
    on_start: LuaRef,

    #[builder(
        generics = "F: ToFunction<OnBufArgs, ()>",
        argtype = "F",
        inline = "{0}.into_luaref()"
    )]
    on_buf: LuaRef,

    #[builder(
        generics = "F: ToFunction<OnWinArgs, DontSkipOnLines>",
        argtype = "F",
        inline = "{0}.into_luaref()"
    )]
    on_win: LuaRef,

    #[builder(
        generics = "F: ToFunction<OnLineArgs, ()>",
        argtype = "F",
        inline = "{0}.into_luaref()"
    )]
    on_line: LuaRef,

    #[builder(
        generics = "F: ToFunction<OnEndArgs, ()>",
        argtype = "F",
        inline = "{0}.into_luaref()"
    )]
    on_end: LuaRef,

    #[builder(skip)]
    _on_hl_def: LuaRef,
    #[builder(skip)]
    _on_spell_nav: LuaRef,
}
