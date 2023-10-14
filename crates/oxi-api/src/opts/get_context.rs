use oxi_types::{self as nvim, Array};

use crate::types::ContextType;

/// Options passed to [`get_context()`](crate::get_context).
#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct GetContextOpts {
    types: oxi_types::Object,
}

/// Options passed to [`get_context()`](crate::get_context).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct GetContextOpts {
    mask: u64,

    /// 1st in the mask.
    types: Array,
}

impl GetContextOpts {
    /// Creates a new [`GetContextOptsBuilder`].
    #[inline]
    pub fn builder() -> GetContextOptsBuilder {
        GetContextOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct GetContextOptsBuilder(GetContextOpts);

impl GetContextOptsBuilder {
    /// List of [`ContextType`]s to gather, or empty for all.
    #[inline]
    pub fn types<T>(&mut self, types: T) -> &mut Self
    where
        T: IntoIterator<Item = ContextType>,
    {
        let types =
            types.into_iter().map(nvim::String::from).collect::<Array>();

        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.types = types.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.types = types;
            self.0.mask |= 0b11;
        }

        self
    }

    #[inline]
    pub fn build(&mut self) -> GetContextOpts {
        std::mem::take(&mut self.0)
    }
}
