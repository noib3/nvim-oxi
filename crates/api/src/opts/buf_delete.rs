/// Options passed to [`Buffer::attach`](crate::Buffer::attach).
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct BufDeleteOpts {
    #[builder(mask)]
    mask: u64,

    /// Force deletion ignoring unsaved changes.
    #[builder(argtype = "bool")]
    force: types::Boolean,

    /// If `true` the buffer will only be unloaded, not deleted.
    #[builder(argtype = "bool")]
    unload: types::Boolean,
}
