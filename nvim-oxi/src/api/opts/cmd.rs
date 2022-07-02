use derive_builder::Builder;
use nvim_types::Object;

/// Options passed to `api::cmd`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct CmdOpts {
    #[builder(setter(strip_option))]
    output: Option<bool>,
}

impl CmdOpts {
    #[inline(always)]
    pub fn builder() -> CmdOptsBuilder {
        CmdOptsBuilder::default()
    }
}

impl CmdOptsBuilder {
    pub fn build(&mut self) -> CmdOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Default, Debug)]
pub(crate) struct KeyDict_cmd_opts {
    output: Object,
}

impl From<&CmdOpts> for KeyDict_cmd_opts {
    fn from(opts: &CmdOpts) -> Self {
        Self { output: opts.output.into() }
    }
}
