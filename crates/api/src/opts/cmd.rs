#[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
use types::Boolean;
#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
use types::Object;

/// Options passed to [cmd](crate::cmd).
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CmdOpts {
    #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
    output: Object,
    #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
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
        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        {
            self.0.output = output.into();
        }
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
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
