use oxi_types::Dictionary;

/// Options passed to [`parse_cmd()`](crate::parse_cmd). Currently unused.
#[derive(Clone, Debug, Default)]
pub struct ParseCmdOpts {}

impl ParseCmdOpts {
    #[inline]
    pub fn builder() -> ParseCmdOptsBuilder {
        ParseCmdOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct ParseCmdOptsBuilder(ParseCmdOpts);

impl ParseCmdOptsBuilder {
    #[inline]
    pub fn build(&mut self) -> ParseCmdOpts {
        std::mem::take(&mut self.0)
    }
}

impl From<&ParseCmdOpts> for Dictionary {
    fn from(_: &ParseCmdOpts) -> Self {
        Dictionary::new()
    }
}
