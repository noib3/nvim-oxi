use nvim_types::Dictionary;

/// Options passed to [`get_mark()`](crate::get_mark). Currently unused.
#[derive(Clone, Debug, Default)]
pub struct GetMarkOpts {}

impl GetMarkOpts {
    #[inline]
    pub fn builder() -> GetMarkOptsBuilder {
        GetMarkOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct GetMarkOptsBuilder(GetMarkOpts);

impl GetMarkOptsBuilder {
    #[inline]
    pub fn build(&mut self) -> GetMarkOpts {
        std::mem::take(&mut self.0)
    }
}

impl From<&GetMarkOpts> for Dictionary {
    fn from(_: &GetMarkOpts) -> Self {
        Dictionary::new()
    }
}
