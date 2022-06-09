use derive_builder::Builder;
use nvim_types::object::Object;

/// Options passed to `Buffer::get_commands`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default)]
pub struct GetCommandsOpts {
    #[builder(setter(strip_option))]
    builtin: Option<bool>,
}

impl GetCommandsOpts {
    #[inline(always)]
    pub fn builder() -> GetCommandsOptsBuilder {
        GetCommandsOptsBuilder::default()
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Default, Debug)]
pub(crate) struct KeyDict_get_commands {
    builtin: Object,
}

impl<'a> From<&'a GetCommandsOpts> for KeyDict_get_commands {
    fn from(opts: &GetCommandsOpts) -> Self {
        Self { builtin: opts.builtin.into() }
    }
}
