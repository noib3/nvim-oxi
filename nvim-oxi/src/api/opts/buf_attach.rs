use derive_builder::Builder;
use nvim_types::{Dictionary, Object};

use crate::api::buffer::Buffer;
use crate::lua::LuaFnMut;

/// Arguments passed to the function registered to `on_lines`.
pub type OnLinesArgs = (
    String,        // the string literal "lines"
    Buffer,        // buffer
    u32,           // b:changedtick
    usize,         // first row changed (0-indexed)
    usize,         // last row changed
    usize,         // last line in updated range
    usize,         // byte count of previous contents
    Option<usize>, // deleted utf32 codepoints (if `utf_sizes` was `true`)
    Option<usize>, // deleted utf16 codeunits (if `utf_sizes` was `true`)
);

/// Arguments passed to the function registered to `on_bytes`.
pub type OnBytesArgs = (
    String, // the string literal "bytes"
    Buffer, // buffer
    u32,    // b:changedtick
    usize,  //
    usize,  //
    usize,  //
    usize,  //
    usize,  //
    usize,  //
    usize,  //
    usize,  //
    usize,  //
);

/// Arguments passed to the function registered to `on_changedtick`.
pub type OnChangedtickArgs = (
    String, // the string literal "changedtick"
    Buffer, // buffer
    u32,    // b:changedtick
);

/// Arguments passed to the function registered to `on_detach`.
pub type OnDetachArgs = (
    String, // the string literal "detach"
    Buffer, // buffer
);

/// Arguments passed to the function registered to `on_reload`.
pub type OnReloadArgs = (
    String, // the string literal "reload"
    Buffer, // buffer
);

/// All the registered functions can detach by returning `true`, as described
/// in `:h api-lua-detach`.
pub type ShouldDetach = bool;

/// Options passed to `Buffer::attach`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default)]
pub struct BufAttachOpts {
    #[builder(setter(custom))]
    on_lines: Option<LuaFnMut<OnLinesArgs, ShouldDetach>>,

    #[builder(setter(custom))]
    on_bytes: Option<LuaFnMut<OnBytesArgs, ShouldDetach>>,

    #[builder(setter(custom))]
    on_changedtick: Option<LuaFnMut<OnChangedtickArgs, ShouldDetach>>,

    #[builder(setter(custom))]
    on_detach: Option<LuaFnMut<OnDetachArgs, ShouldDetach>>,

    #[builder(setter(custom))]
    on_reload: Option<LuaFnMut<OnReloadArgs, ShouldDetach>>,

    utf_sizes: bool,
    preview: bool,
}

impl BufAttachOpts {
    #[inline(always)]
    pub fn builder() -> BufAttachOptsBuilder {
        BufAttachOptsBuilder::default()
    }
}

macro_rules! lua_fn_setter {
    ($name:ident, $args:ident) => {
        pub fn $name<F>(&mut self, fun: F) -> &mut Self
        where
            F: FnMut($args) -> crate::Result<ShouldDetach> + 'static,
        {
            self.$name = Some(Some(fun.into()));
            self
        }
    };
}

impl BufAttachOptsBuilder {
    lua_fn_setter!(on_lines, OnLinesArgs);
    lua_fn_setter!(on_bytes, OnBytesArgs);
    lua_fn_setter!(on_changedtick, OnChangedtickArgs);
    lua_fn_setter!(on_detach, OnDetachArgs);
    lua_fn_setter!(on_reload, OnReloadArgs);
}

impl From<BufAttachOpts> for Dictionary {
    fn from(opts: BufAttachOpts) -> Self {
        Self::from_iter([
            ("on_lines", Object::from(opts.on_lines)),
            ("on_bytes", opts.on_bytes.into()),
            ("on_changedtick", opts.on_changedtick.into()),
            ("on_detach", opts.on_detach.into()),
            ("on_reload", opts.on_reload.into()),
            ("utf_sizes", opts.utf_sizes.into()),
            ("preview", opts.preview.into()),
        ])
    }
}
