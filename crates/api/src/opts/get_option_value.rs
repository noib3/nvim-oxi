use serde::Serialize;
use types::conversion::FromObject;

use crate::{Buffer, Window};

/// Options passed to
/// [`set_option_value()`](crate::set_option_value).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct OptionValueOpts {
    #[builder(mask)]
    mask: u64,

    #[builder(argtype = "OptionScope", inline = "{0}.into()")]
    scope: types::String,

    #[builder(argtype = "Window", inline = "{0}.0")]
    win: types::WinHandle,

    #[builder(method = "buffer", argtype = "Buffer", inline = "{0}.0")]
    buf: types::BufHandle,

    #[builder(
        generics = "F: Into<types::String>",
        argtype = "F",
        inline = "{0}.into()"
    )]
    filetype: types::String,
}

/// Options passed to
/// [`set_option_value()`](crate::set_option_value).
#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct OptionValueOpts {
    buf: types::Object,
    win: types::Object,
    scope: types::Object,
    filetype: types::Object,
}

#[cfg(not(feature = "neovim-nightly"))]
impl OptionValueOpts {
    #[inline(always)]
    pub fn builder() -> OptionValueOptsBuilder {
        OptionValueOptsBuilder::default()
    }
}

#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Default)]
pub struct OptionValueOptsBuilder(OptionValueOpts);

#[cfg(not(feature = "neovim-nightly"))]
impl OptionValueOptsBuilder {
    #[inline]
    pub fn buffer(&mut self, buffer: Buffer) -> &mut Self {
        self.0.buf = buffer.into();
        self
    }

    #[inline]
    pub fn filetype(&mut self, filetype: &str) -> &mut Self {
        let filetype = types::String::from(filetype);
        self.0.filetype = filetype.into();
        self
    }

    #[inline]
    pub fn scope(&mut self, scope: OptionScope) -> &mut Self {
        let scope = types::String::from(scope);
        self.0.scope = scope.into();
        self
    }

    #[inline]
    pub fn window(&mut self, window: Window) -> &mut Self {
        self.0.win = window.into();
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

impl From<OptionScope> for types::String {
    #[inline]
    fn from(ctx: OptionScope) -> Self {
        types::String::from_object(
            ctx.serialize(types::serde::Serializer::new())
                .expect("`OptionScope` is serializable"),
        )
        .expect("`OptionScope` is serialized into a string")
    }
}
