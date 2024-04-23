/// Options passed to [`echo()`](crate::echo).
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct EchoOpts {
    #[cfg(all(feature = "neovim-0-9", not(feature = "neovim-nightly")))]
    verbose: types::Object,

    #[cfg(feature = "neovim-nightly")]
    verbose: bool,
}

#[cfg(not(any(feature = "neovim-0-9", feature = "neovim-nightly")))]
impl From<&EchoOpts> for types::Dictionary {
    #[inline]
    fn from(_: &EchoOpts) -> Self {
        Self::default()
    }
}

impl EchoOpts {
    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "neovim-0-9", feature = "neovim-nightly")))
    )]
    #[inline(always)]
    pub fn builder() -> EchoOptsBuilder {
        EchoOptsBuilder::default()
    }
}

#[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "neovim-0-9", feature = "neovim-nightly")))
)]
#[derive(Clone, Default)]
pub struct EchoOptsBuilder(EchoOpts);

#[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
impl EchoOptsBuilder {
    #[inline]
    pub fn verbose(&mut self, verbose: bool) -> &mut Self {
        #[cfg(all(feature = "neovim-0-9", not(feature = "neovim-nightly")))]
        {
            self.0.verbose = verbose.into();
        }
        #[cfg(feature = "neovim-nightly")]
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
