/// Options passed to [`get_mark()`](crate::get_mark). Currently unused.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct GetMarkOpts {
    #[builder(mask)]
    mask: u64,
}
