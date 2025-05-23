/// Options passed to [`echo()`](crate::echo).
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct EchoOpts {
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
        self.0.verbose = verbose;
        self
    }

    #[inline]
    pub fn build(&mut self) -> EchoOpts {
        core::mem::take(&mut self.0)
    }
}
