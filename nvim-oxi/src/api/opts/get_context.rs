use derive_builder::Builder;
use nvim_types::{Array, Object, String as NvimString};

use crate::api::types::ContextType;

/// Options passed to `crate::api::get_context`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct GetContextOpts {
    #[builder(setter(custom))]
    types: Option<Vec<NvimString>>,
}

impl GetContextOpts {
    #[inline(always)]
    pub fn builder() -> GetContextOptsBuilder {
        GetContextOptsBuilder::default()
    }
}

impl GetContextOptsBuilder {
    pub fn types<T: IntoIterator<Item = ContextType>>(
        &mut self,
        types: T,
    ) -> &mut Self {
        self.types = Some(Some(types.into_iter().map(Into::into).collect()));
        self
    }

    pub fn build(&mut self) -> GetContextOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Default, Debug)]
pub(crate) struct KeyDict_context {
    types: Object,
}

impl From<GetContextOpts> for KeyDict_context {
    fn from(opts: GetContextOpts) -> Self {
        Self { types: opts.types.map(Array::from_iter).into() }
    }
}
