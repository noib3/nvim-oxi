use derive_builder::Builder;
use nvim_types::dictionary::Dictionary;

/// Options passed to `Buffer::get_commands`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default)]
pub struct GetCommandsOpts {
    builtin: bool,
}

impl GetCommandsOpts {
    #[inline(always)]
    pub fn builder() -> GetCommandsOptsBuilder {
        GetCommandsOptsBuilder::default()
    }
}

impl From<GetCommandsOpts> for Dictionary {
    fn from(opts: GetCommandsOpts) -> Self {
        Self::from_iter([("builtin", opts.builtin)])
    }
}

impl<'a> From<&'a GetCommandsOpts> for Dictionary {
    fn from(opts: &GetCommandsOpts) -> Self {
        opts.clone().into()
    }
}
