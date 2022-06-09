use derive_builder::Builder;
use nvim_types::dictionary::Dictionary;

/// Options passed to `Buffer::get_commands`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default)]
pub struct BufDeleteOpts {
    force: bool,
    unload: bool,
}

impl BufDeleteOpts {
    #[inline(always)]
    pub fn builder() -> BufDeleteOptsBuilder {
        BufDeleteOptsBuilder::default()
    }
}

impl From<BufDeleteOpts> for Dictionary {
    fn from(opts: BufDeleteOpts) -> Self {
        Self::from_iter([("force", opts.force), ("unload", opts.unload)])
    }
}
