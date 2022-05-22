use nvim_types::{BufHandle, NvimString};

#[derive(Default)]
pub struct BufAttachOpts {
    on_lines: Option<
        Box<
            dyn FnMut(
                NvimString,
                BufHandle,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> Option<bool>,
        >,
    >,

    on_bytes: Option<Box<dyn FnMut(NvimString, BufHandle) -> Option<bool>>>,

    on_changedtick:
        Option<Box<dyn FnMut(NvimString, BufHandle) -> Option<bool>>>,

    on_detach: Option<Box<dyn FnMut(NvimString, BufHandle) -> Option<bool>>>,

    on_reload: Option<Box<dyn FnMut(NvimString, BufHandle) -> Option<bool>>>,

    utf_sizes: bool,

    utf_preview: bool,
}
