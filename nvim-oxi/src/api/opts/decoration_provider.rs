use derive_builder::Builder;
use nvim_types::{Dictionary, Object};

use crate::api::{Buffer, Window};
use crate::lua::LuaFun;

/// Arguments passed to the function registered to `on_buf`.
// TODO: docs say a third argument of changedtick is passed. I don't see it.
pub type OnBufArgs = (
    String, // the string literal "buf"
    Buffer, // buffer
    u32,    // changedtick, dubious */
);

/// Arguments passed to the function registered to `on_reload`.
pub type OnEndArgs = (
    String, // the string literal "end"
    u32,    // changedtick
);

/// Arguments passed to the function registered to `on_line`.
pub type OnLineArgs = (
    String, // the string literal "win"
    Window, // window
    Buffer, // buffer
    usize,  // row
);

/// Arguments passed to the function registered to `on_start`.
pub type OnStartArgs = (
    String, // the string literal "start"
    u32,    // changedtick
    u32, /* `type`, undocumented? (https://github.com/neovim/neovim/blob/master/src/nvim/decoration_provider.c#L68) */
);

/// Arguments passed to the function registered to `on_win`.
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

/// Options passed to `Buffer::attach`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct DecorationProviderOpts {
    #[builder(setter(custom))]
    on_buf: Object,

    #[builder(setter(custom))]
    on_end: Object,

    #[builder(setter(custom))]
    on_line: Object,

    #[builder(setter(custom))]
    on_start: Object,

    #[builder(setter(custom))]
    on_win: Object,
}

impl DecorationProviderOpts {
    #[inline(always)]
    pub fn builder() -> DecorationProviderOptsBuilder {
        DecorationProviderOptsBuilder::default()
    }
}

macro_rules! lua_fn_setter {
    ($name:ident, $args:ty, $ret:ty) => {
        pub fn $name<F>(&mut self, fun: F) -> &mut Self
        where
            F: FnMut($args) -> crate::Result<$ret> + 'static,
        {
            self.$name = Some(LuaFun::from_fn_mut(fun).into());
            self
        }
    };
}

impl DecorationProviderOptsBuilder {
    lua_fn_setter!(on_buf, OnBufArgs, ());
    lua_fn_setter!(on_end, OnEndArgs, ());
    lua_fn_setter!(on_line, OnLineArgs, ());
    lua_fn_setter!(on_start, OnStartArgs, DontSkipRedrawCycle);
    lua_fn_setter!(on_win, OnWinArgs, DontSkipOnLines);

    pub fn build(&mut self) -> DecorationProviderOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl From<&DecorationProviderOpts> for Dictionary {
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
