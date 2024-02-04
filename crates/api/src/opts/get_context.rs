use crate::types::ContextType;

/// Options passed to [`get_context()`](crate::get_context).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct GetContextOpts {
    #[builder(mask)]
    mask: u64,

    /// List of [`ContextType`]s to gather, or empty for all.
    #[builder(
        generics = "T: IntoIterator<Item = ContextType>",
        argtype = "T",
        inline = "{0}.into_iter().map(types::String::from).collect()"
    )]
    types: types::Array,
}

/// Options passed to [`get_context()`](crate::get_context).
#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct GetContextOpts {
    types: types::Object,
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl GetContextOpts {
    /// Creates a new [`GetContextOptsBuilder`].
    #[inline]
    pub fn builder() -> GetContextOptsBuilder {
        GetContextOptsBuilder::default()
    }
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
#[derive(Clone, Default)]
pub struct GetContextOptsBuilder(GetContextOpts);

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl GetContextOptsBuilder {
    /// List of [`ContextType`]s to gather, or empty for all.
    #[inline]
    pub fn types<T>(&mut self, types: T) -> &mut Self
    where
        T: IntoIterator<Item = ContextType>,
    {
        self.0.types = types
            .into_iter()
            .map(types::String::from)
            .collect::<types::Array>()
            .into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> GetContextOpts {
        std::mem::take(&mut self.0)
    }
}
