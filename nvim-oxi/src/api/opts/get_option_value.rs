use derive_builder::Builder;
use nvim_types::{self as nvim, NonOwning, Object};
use serde::Serialize;

use crate::object;

/// Options passed to
/// [`nvim_oxi::api::set_option_value`](crate::api::set_option_value).
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct OptionValueOpts {
    #[cfg(feature = "nightly")]
    #[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
    #[builder(setter(strip_option))]
    buffer: Option<crate::api::Buffer>,

    #[builder(setter(custom))]
    scope: Object,

    #[cfg(feature = "nightly")]
    #[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
    #[builder(setter(into, strip_option))]
    window: Option<crate::api::Window>,
}

impl OptionValueOpts {
    #[inline(always)]
    pub fn builder() -> OptionValueOptsBuilder {
        OptionValueOptsBuilder::default()
    }
}

impl OptionValueOptsBuilder {
    pub fn scope(&mut self, scope: OptionScope) -> &mut Self {
        self.scope = Some(nvim::String::from(scope).into());
        self
    }

    pub fn build(&mut self) -> OptionValueOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OptionScope {
    Global,
    Local,
}

impl From<OptionScope> for nvim::String {
    fn from(ctx: OptionScope) -> Self {
        ctx.serialize(object::Serializer)
            .expect("`OptionScope` is serializable")
            .try_into()
            .expect("`OptionScope` is serialized into a string")
    }
}

#[derive(Default)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_option<'a> {
    #[cfg(feature = "nightly")]
    buf: Object,
    #[cfg(feature = "nightly")]
    win: Object,
    scope: NonOwning<'a, Object>,
}

impl<'a> From<&'a OptionValueOpts> for KeyDict_option<'a> {
    fn from(opts: &'a OptionValueOpts) -> Self {
        Self {
            #[cfg(feature = "nightly")]
            buf: opts.buffer.into(),
            #[cfg(feature = "nightly")]
            win: opts.window.into(),
            scope: opts.scope.non_owning(),
        }
    }
}
