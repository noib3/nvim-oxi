use oxi_types::Dictionary;

/// Options passed to [`notify()`](crate::notify). Currently unused.
#[derive(Clone, Debug, Default)]
pub struct NotifyOpts {}

impl NotifyOpts {
    #[inline(always)]
    pub fn builder() -> NotifyOptsBuilder {
        NotifyOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct NotifyOptsBuilder(NotifyOpts);

impl NotifyOptsBuilder {
    #[inline]
    pub fn build(&mut self) -> NotifyOpts {
        std::mem::take(&mut self.0)
    }
}

impl From<&NotifyOpts> for Dictionary {
    fn from(_: &NotifyOpts) -> Self {
        Dictionary::new()
    }
}
