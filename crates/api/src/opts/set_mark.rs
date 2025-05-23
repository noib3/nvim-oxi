/// Options passed to [`Buffer::set_mark()`](crate::Buffer::get_mark).
/// Currently unused.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct SetMarkOpts {
    #[builder(mask)]
    mask: u64,
}
