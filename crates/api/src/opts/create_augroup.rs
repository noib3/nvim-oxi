/// Options passed to [`create_augroup()`](crate::create_augroup).
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct CreateAugroupOpts {
    #[builder(mask)]
    mask: u64,

    /// Whether to clear existing commands if the group already exists.
    #[builder(argtype = "bool")]
    clear: types::Boolean,
}
