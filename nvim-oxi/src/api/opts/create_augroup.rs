use derive_builder::Builder;
use nvim_types::{Array, Object, String as NvimString};

use crate::api::Buffer;

/// Options passed to `crate::api::create_augroup`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct CreateAugroupOpts {
    ///
    #[builder(setter(strip_option))]
    clear: Option<bool>,
}

impl CreateAugroupOpts {
    #[inline(always)]
    pub fn builder() -> CreateAugroupOptsBuilder {
        CreateAugroupOptsBuilder::default()
    }
}

impl CreateAugroupOptsBuilder {
    pub fn build(&mut self) -> CreateAugroupOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_create_augroup {
    clear: Object,
}

impl From<CreateAugroupOpts> for KeyDict_create_augroup {
    fn from(opts: CreateAugroupOpts) -> Self {
        Self { clear: opts.clear.into() }
    }
}
