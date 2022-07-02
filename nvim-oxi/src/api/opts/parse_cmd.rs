use derive_builder::Builder;
use nvim_types::Dictionary;

/// Options passed to `crate::api::get_mark`. It's currently reserved for
/// future use and doesn't have any methods.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct ParseCmdOpts {}

impl ParseCmdOpts {
    #[inline(always)]
    pub fn builder() -> ParseCmdOptsBuilder {
        ParseCmdOptsBuilder::default()
    }
}

impl ParseCmdOptsBuilder {
    pub fn build(&mut self) -> ParseCmdOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl From<&ParseCmdOpts> for Dictionary {
    fn from(_: &ParseCmdOpts) -> Self {
        Dictionary::new()
    }
}
