use nvim_types::{self as nvim, Array, Object};

use crate::types::ContextType;

/// Options passed to [`get_context()`](crate::get_context).
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct GetContextOpts {
    types: Object,
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
        self.0.types = types
            .into_iter()
            .map(nvim::String::from)
            .collect::<Array>()
            .into();

        self
    }

    #[inline]
    pub fn build(&mut self) -> GetContextOpts {
        std::mem::take(&mut self.0)
    }
}
