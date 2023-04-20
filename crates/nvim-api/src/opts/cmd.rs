use nvim_types::Object;

/// Options passed to [cmd](crate::cmd).
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CmdOpts {
    output: Object,
}

impl CmdOpts {
    #[inline(always)]
    pub fn builder() -> CmdOptsBuilder {
        CmdOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct CmdOptsBuilder(CmdOpts);

impl CmdOptsBuilder {
    #[inline]
    pub fn output(&mut self, output: bool) -> &mut Self {
        self.0.output = output.into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> CmdOpts {
        std::mem::take(&mut self.0)
    }
}
