/// Options passed to [`echo()`](crate::echo).
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct EchoOpts {
    #[cfg(feature = "neovim-0-9")]
    verbose: types::Object,

    #[cfg(feature = "neovim-nightly")]
    verbose: bool,
}

#[cfg(feature = "neovim-0-8")]
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
        #[cfg(feature = "neovim-0-9")]
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
