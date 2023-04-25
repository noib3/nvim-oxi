use oxi_types::{self as nvim, conversion::FromObject, Object};
use serde::Serialize;

use crate::{Buffer, Window};

/// Options passed to
/// [`set_option_value()`](crate::set_option_value).
#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct OptionValueOpts {
    buffer: Object,
    window: Object,
    scope: Object,
}

/// Options passed to
/// [`set_option_value()`](crate::set_option_value).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct OptionValueOpts {
    scope: Object,
    window: Object,
    buffer: Object,
    filetype: Object,
}

impl OptionValueOpts {
    #[inline(always)]
    pub fn builder() -> OptionValueOptsBuilder {
        OptionValueOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct OptionValueOptsBuilder(OptionValueOpts);

impl OptionValueOptsBuilder {
    #[inline]
    pub fn buffer(&mut self, buffer: Buffer) -> &mut Self {
        self.0.buffer = buffer.into();
        self
    }

    #[cfg(feature = "neovim-nightly")]
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
    #[inline]
    pub fn filetype(&mut self, filetype: &str) -> &mut Self {
        self.0.filetype = nvim::String::from(filetype).into();
        self
    }

    #[inline]
    pub fn scope(&mut self, scope: OptionScope) -> &mut Self {
        self.0.scope = nvim::String::from(scope).into();
        self
    }

    #[inline]
    pub fn window(&mut self, window: Window) -> &mut Self {
        self.0.window = window.into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> OptionValueOpts {
        std::mem::take(&mut self.0)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OptionScope {
    Global,
    Local,
}

impl From<OptionScope> for nvim::String {
    #[inline]
    fn from(ctx: OptionScope) -> Self {
        nvim::String::from_object(
            ctx.serialize(nvim::serde::Serializer::new())
                .expect("`OptionScope` is serializable"),
        )
        .expect("`OptionScope` is serialized into a string")
    }
}
