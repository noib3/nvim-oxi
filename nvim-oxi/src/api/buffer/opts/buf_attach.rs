use derive_builder::Builder;
use nvim_types::{Dictionary, LuaRef, Object};

use crate::api::buffer::Buffer;
use crate::lua;
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

#[derive(Default, Builder)]
#[builder(default)]
pub struct BufAttachOpts {
    #[builder(setter(custom))]
    on_lines: Option<LuaRef>,

    #[builder(setter(custom))]
    on_bytes: Option<LuaRef>,

    #[builder(setter(custom))]
    on_changedtick: Option<LuaRef>,

    #[builder(setter(custom))]
    on_detach: Option<LuaRef>,

    #[builder(setter(custom))]
    on_reload: Option<LuaRef>,

    utf_sizes: bool,
    preview: bool,
}

macro_rules! luaref_setter {
    ($name:ident, $args:ident) => {
        pub fn $name<F>(&mut self, fun: F) -> &mut Self
        where
            F: FnMut($args) -> crate::Result<ShouldDetach> + 'static,
        {
            self.$name = Some(Some(lua::mut_to_luaref(fun)));
            self
        }
    };
}

impl BufAttachOptsBuilder {
    luaref_setter!(on_lines, OnLinesArgs);

    luaref_setter!(on_bytes, OnBytesArgs);

    luaref_setter!(on_changedtick, OnChangedtickArgs);

    luaref_setter!(on_detach, OnDetachArgs);

    luaref_setter!(on_reload, OnReloadArgs);
}

impl From<BufAttachOpts> for Dictionary {
    fn from(opts: BufAttachOpts) -> Self {
        Self::from_iter::<[(_, Object); 7]>([
            ("on_lines", opts.on_lines.into()),
            ("on_bytes", opts.on_bytes.into()),
            ("on_changedtick", opts.on_changedtick.into()),
            ("on_detach", opts.on_detach.into()),
            ("on_reload", opts.on_reload.into()),
            ("utf_sizes", opts.utf_sizes.into()),
            ("preview", opts.preview.into()),
        ])
    }
}
