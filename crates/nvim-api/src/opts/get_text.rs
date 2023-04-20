use nvim_types::Dictionary;

/// Options passed to [`Buffer::get_text()`](crate::Buffer::get_text).
/// Currently unused.
#[derive(Clone, Debug, Default)]
pub struct GetTextOpts {}

impl GetTextOpts {
    #[inline]
    pub fn builder() -> GetTextOptsBuilder {
        GetTextOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct GetTextOptsBuilder(GetTextOpts);

impl GetTextOptsBuilder {
    #[inline]
    pub fn build(&mut self) -> GetTextOpts {
        std::mem::take(&mut self.0)
    }
}

impl From<&GetTextOpts> for Dictionary {
    #[inline]
    fn from(_: &GetTextOpts) -> Self {
        Dictionary::new()
    }
}
