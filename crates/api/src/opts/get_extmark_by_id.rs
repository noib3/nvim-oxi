/// Options passed to
/// [`Buffer::get_extmark_by_id()`](crate::Buffer::get_extmark_by_id).
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct GetExtmarkByIdOpts {
    #[builder(mask)]
    mask: u64,

    /// Whether to include the extmark's
    /// [`ExtmarkInfos`](crate::types::ExtmarkInfos) as the last element of the
    /// tuple returned by
    /// [`Buffer::get_extmark_by_id`](crate::Buffer::get_extmark_by_id).
    #[builder(argtype = "bool")]
    details: types::Boolean,

    #[builder(argtype = "bool")]
    hl_name: types::Boolean,
}
