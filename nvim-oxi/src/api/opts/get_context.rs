use derive_builder::Builder;
use nvim_types::{self as nvim, Array, NonOwning, Object};

use crate::api::types::ContextType;

/// Options passed to [`api::get_context`](crate::api::get_context).
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct GetContextOpts {
    #[builder(setter(custom))]
    types: Object,
}

impl GetContextOpts {
    #[inline(always)]
    /// Creates a new [`GetContextOptsBuilder`].
    pub fn builder() -> GetContextOptsBuilder {
        GetContextOptsBuilder::default()
    }
}

impl GetContextOptsBuilder {
    /// List of [`ContextType`]s to gather, or empty for all.
    pub fn types<T: IntoIterator<Item = ContextType>>(
        &mut self,
        types: T,
    ) -> &mut Self {
        self.types = Some(
            types
                .into_iter()
                .map(nvim::String::from)
                .collect::<Array>()
                .into(),
        );
        self
    }

    pub fn build(&mut self) -> GetContextOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[derive(Default, Debug)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_context<'a> {
    types: NonOwning<'a, Object>,
}

impl<'a> From<&'a GetContextOpts> for KeyDict_context<'a> {
    fn from(opts: &'a GetContextOpts) -> Self {
        Self { types: opts.types.non_owning() }
    }
}
