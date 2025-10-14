/// Options passed to [`echo()`](crate::echo).
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct EchoOpts {
    #[cfg(feature = "neovim-0-11")] // Only on 0.11.
    err: bool,
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
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-0-11")))]
    #[cfg(feature = "neovim-0-11")] // Only on 0.11.
    #[inline]
    pub fn err(&mut self, err: bool) -> &mut Self {
        self.0.err = err;
        self
    }

    #[inline]
    pub fn verbose(&mut self, verbose: bool) -> &mut Self {
        self.0.verbose = verbose;
        self
    }

    #[inline]
    pub fn build(&mut self) -> EchoOpts {
        core::mem::take(&mut self.0)
    }
}
