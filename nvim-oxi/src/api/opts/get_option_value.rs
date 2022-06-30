use derive_builder::Builder;
use nvim_types::{Object, String as NvimString, WinHandle};
use serde::Serialize;

use crate::api::Buffer;
use crate::object;

/// Options passed to `crate::api::create_user_command`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct OptionValueOpts {
    #[builder(setter(into, strip_option))]
    scope: Option<OptionScope>,

    #[builder(setter(into, strip_option))]
    window: Option<WinHandle>,

    #[builder(setter(into, strip_option))]
    buffer: Option<Buffer>,
}

impl OptionValueOpts {
    #[inline(always)]
    pub fn builder() -> OptionValueOptsBuilder {
        OptionValueOptsBuilder::default()
    }
}

impl OptionValueOptsBuilder {
    pub fn build(&mut self) -> OptionValueOpts {
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

impl From<OptionValueOpts> for KeyDict_option {
    fn from(opts: OptionValueOpts) -> Self {
        Self {
            buf: opts.buffer.into(),
            win: opts.window.map(WinHandle::from).into(),
            scope: opts.scope.map(NvimString::from).into(),
        }
    }
}
