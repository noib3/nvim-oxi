#[cfg(not(feature = "neovim-nightly"))]
use oxi_types::Object;
use oxi_types::{self as nvim, conversion::FromObject};
#[cfg(feature = "neovim-nightly")]
use oxi_types::{BufHandle, String as NvimString, WinHandle};
use serde::Serialize;

use crate::{Buffer, Window};

/// Options passed to
/// [`set_option_value()`](crate::set_option_value).
#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct OptionValueOpts {
    buf: Object,
    win: Object,
    scope: Object,
    filetype: Object,
}

/// Options passed to
/// [`set_option_value()`](crate::set_option_value).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct OptionValueOpts {
    /// <filetype><scope><win><buf>1
    mask: u64,

    /// 3rd in the mask.
    scope: NvimString,

    /// 2nd in the mask.
    win: WinHandle,

    /// 1st in the mask.
    buf: BufHandle,

    /// 4th in the mask.
    filetype: NvimString,
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
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.buf = buffer.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.buf = buffer.0;
            self.0.mask |= 0b11;
        }
        self
    }

    #[inline]
    pub fn filetype(&mut self, filetype: &str) -> &mut Self {
        let filetype = nvim::String::from(filetype);

        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.filetype = filetype.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.filetype = filetype;
            self.0.mask |= 0b10001;
        }

        self
    }

    #[inline]
    pub fn scope(&mut self, scope: OptionScope) -> &mut Self {
        let scope = nvim::String::from(scope);

        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.scope = scope.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.scope = scope;
            self.0.mask |= 0b1001;
        }

        self
    }

    #[inline]
    pub fn window(&mut self, window: Window) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.win = window.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.win = window.0;
            self.0.mask |= 0b101;
        }
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
