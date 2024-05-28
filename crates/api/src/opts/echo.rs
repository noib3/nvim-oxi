/// Options passed to [`echo()`](crate::echo).
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct EchoOpts {
    #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
    verbose: types::Object,

    #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
    verbose: bool,
}

impl EchoOpts {
    #[inline(always)]
    pub fn builder() -> EchoOptsBuilder {
        EchoOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct EchoOptsBuilder(EchoOpts);

impl EchoOptsBuilder {
    #[inline]
    pub fn verbose(&mut self, verbose: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
        {
            self.0.verbose = verbose.into();
        }
        #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
        {
            self.0.verbose = verbose;
        }
        self
    }

    #[inline]
    pub fn build(&mut self) -> EchoOpts {
        core::mem::take(&mut self.0)
    }
}
