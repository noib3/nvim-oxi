use derive_builder::Builder;
use nvim_types::Object;

/// Options passed to `Buffer::get_commands`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
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

impl GetCommandsOptsBuilder {
    pub fn build(&mut self) -> GetCommandsOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
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
