/// Options passed to [`Buffer::get_text()`](crate::Buffer::get_text).
/// Currently unused.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct GetTextOpts {
    #[builder(mask)]
    mask: u64,
}
