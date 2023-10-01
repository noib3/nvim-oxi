#[cfg(feature = "neovim-nightly")]
use oxi_types::Boolean;
#[cfg(not(feature = "neovim-nightly"))]
use oxi_types::Object;

/// Options passed to [cmd](crate::cmd).
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CmdOpts {
    #[cfg(not(feature = "neovim-nightly"))]
    output: Object,
    #[cfg(feature = "neovim-nightly")]
    output: Boolean,
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
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.output = output.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.output = output;
        }
        self
    }

    #[inline]
    pub fn build(&mut self) -> CmdOpts {
        std::mem::take(&mut self.0)
    }
}
