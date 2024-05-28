use crate::types::ContextType;

/// Options passed to [`get_context()`](crate::get_context).
#[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
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
#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct GetContextOpts {
    types: types::Object,
}

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
impl GetContextOpts {
    /// Creates a new [`GetContextOptsBuilder`].
    #[inline]
    pub fn builder() -> GetContextOptsBuilder {
        GetContextOptsBuilder::default()
    }
}

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
#[derive(Clone, Default)]
pub struct GetContextOptsBuilder(GetContextOpts);

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
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
