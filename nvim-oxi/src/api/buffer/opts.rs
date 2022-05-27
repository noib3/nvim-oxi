use nvim_types::{BufHandle, Object};

use crate::lua;
/// The arguments passed to the function registered to `on_lines`.
pub type OnLinesArgs = (
    String,        // the string literal "lines"
    BufHandle,     // buffer handle
    u32,           // b:changedtick
    usize,         // first row changed (0-indexed)
    usize,         // last row changed
    usize,         // last line in updated range
    usize,         // byte count of previous contents
    Option<usize>, // deleted utf32 codepoints (if `utf_sizes` was `true`)
    Option<usize>, // deleted utf16 codeunits (if `utf_sizes` was `true`)
);

/// The arguments passed to the function registered to `on_bytes`.
pub type OnBytesArgs = (
    String,    // the string literal "bytes"
    BufHandle, // buffer handle
    u32,       // b:changedtick
    usize,     //
    usize,     //
    usize,     //
    usize,     //
    usize,     //
    usize,     //
    usize,     //
    usize,     //
    usize,     //
);

/// The arguments passed to the function registered to `on_changedtick`.
pub type OnChangedtickArgs = (
    String,    // the string literal "changedtick"
    BufHandle, // buffer handle
    u32,       // b:changedtick
);

/// The arguments passed to the function registered to `on_detach`.
pub type OnDetachArgs = (
    String,    // the string literal "detach"
    BufHandle, // buffer handle
);

/// The arguments passed to the function registered to `on_reload`.
pub type OnReloadArgs = (
    String,    // the string literal "reload"
    BufHandle, // buffer handle
);

/// All callbacks can detach by returning `true`, as described in `:h
/// api-lua-detach`.
pub type ShouldDetach = bool;

type CbMut<A, R> = Box<dyn FnMut(A) -> crate::Result<R> + 'static>;
type AttachCb<A> = Option<CbMut<A, ShouldDetach>>;

#[derive(Default)]
pub struct BufAttachOpts {
    pub(crate) on_lines: AttachCb<OnLinesArgs>,
    pub(crate) on_bytes: AttachCb<OnBytesArgs>,
    pub(crate) on_changedtick: AttachCb<OnChangedtickArgs>,
    pub(crate) on_detach: AttachCb<OnDetachArgs>,
    pub(crate) on_reload: AttachCb<OnReloadArgs>,
    pub(crate) utf_sizes: bool,
    pub(crate) preview: bool,
}

impl From<BufAttachOpts> for nvim_types::Dictionary {
    fn from(opts: BufAttachOpts) -> Self {
        Self::from_iter([
            // ("on_lines", Object::from(opts.on_lines.map(lua::mut_to_luaref))),
            ("on_bytes", opts.on_bytes.map(lua::mut_to_luaref).into()),
            // (
            //     "on_changedtick",
            //     opts.on_changedtick.map(lua::mut_to_luaref).into(),
            // ),
            // ("on_detach", opts.on_detach.map(lua::mut_to_luaref).into()),
            // ("on_reload", opts.on_reload.map(lua::mut_to_luaref).into()),
            ("utf_sizes", opts.utf_sizes.into()),
            ("preview", Object::from(opts.preview)),
        ])
    }
}
