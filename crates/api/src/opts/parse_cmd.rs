/// Options passed to [`parse_cmd()`](crate::parse_cmd). Currently unused.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct ParseCmdOpts {
    #[builder(mask)]
    mask: u64,
}
