use derive_builder::Builder;
use nvim_types::{Object, String as NvimString};
use serde::Serialize;

use crate::object;

/// Options passed to `crate::api::get_option_value`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct GetOptionValueOpts {
    scope: Option<OptionScope>,
}

impl GetOptionValueOpts {
    #[inline(always)]
    pub fn builder() -> GetOptionValueOptsBuilder {
        GetOptionValueOptsBuilder::default()
    }
}

impl GetOptionValueOptsBuilder {
    pub fn build(&mut self) -> GetOptionValueOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OptionScope {
    Global,
    Local,
}

impl From<OptionScope> for NvimString {
    fn from(ctx: OptionScope) -> Self {
        ctx.serialize(object::Serializer)
            .expect("`OptionScope` is serializable")
            .try_into()
            .expect("`OptionScope` is serialized into a string")
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_option {
    buf: Object,
    win: Object,
    scope: Object,
}

impl From<GetOptionValueOpts> for KeyDict_option {
    fn from(opts: GetOptionValueOpts) -> Self {
        Self {
            buf: Object::nil(),
            win: Object::nil(),
            scope: opts.scope.map(NvimString::from).into(),
        }
    }
}
