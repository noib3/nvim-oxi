use oxi_types::{
    self as nvim,
    conversion::ToObject,
    Function,
    Integer,
    Object,
};

use crate::types::{
    CommandAddr,
    CommandArgs,
    CommandComplete,
    CommandNArgs,
    CommandRange,
};
use crate::Buffer;

/// Options passed to
/// [`Buffer::create_user_command()`](crate::Buffer::create_user_command).
#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CreateCommandOpts {
    bar: Object,
    addr: Object,
    bang: Object,
    desc: Object,
    count: Object,
    force: Object,
    nargs: Object,
    range: Object,
    preview: Object,
    complete: Object,
    register_: Object,
    keepscript: Object,
}

/// Options passed to
/// [`Buffer::create_user_command()`](crate::Buffer::create_user_command).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CreateCommandOpts {
    addr: Object,
    bang: Object,
    bar: Object,
    complete: Object,
    count: Object,
    desc: Object,
    force: Object,
    keepscript: Object,
    nargs: Object,
    preview: Object,
    range: Object,
    register_: Object,
}

impl CreateCommandOpts {
    #[inline(always)]
    /// Creates a new [`CreateCommandOptsBuilder`].
    pub fn builder() -> CreateCommandOptsBuilder {
        CreateCommandOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct CreateCommandOptsBuilder(CreateCommandOpts);

impl CreateCommandOptsBuilder {
    #[inline]
    pub fn addr(&mut self, addr: CommandAddr) -> &mut Self {
        self.0.addr = addr.to_object().unwrap();
        self
    }

    #[inline]
    pub fn bang(&mut self, bang: bool) -> &mut Self {
        self.0.bang = bang.into();
        self
    }

    #[inline]
    pub fn bar(&mut self, bar: bool) -> &mut Self {
        self.0.bar = bar.into();
        self
    }

    #[inline]
    pub fn complete(&mut self, complete: CommandComplete) -> &mut Self {
        self.0.complete = complete.to_object().unwrap();
        self
    }

    #[inline]
    pub fn count(&mut self, count: impl Into<Integer>) -> &mut Self {
        self.0.count = count.into().into();
        self
    }

    /// Description for the command.
    #[inline]
    pub fn desc(&mut self, desc: impl Into<nvim::String>) -> &mut Self {
        self.0.desc = desc.into().into();
        self
    }

    #[inline]
    pub fn force(&mut self, force: bool) -> &mut Self {
        self.0.force = force.into();
        self
    }

    #[inline]
    pub fn keepscript(&mut self, keepscript: bool) -> &mut Self {
        self.0.keepscript = keepscript.into();
        self
    }

    #[inline]
    pub fn nargs(&mut self, nargs: CommandNArgs) -> &mut Self {
        self.0.nargs = nargs.to_object().unwrap();
        self
    }

    #[inline]
    pub fn preview<F>(&mut self, fun: F) -> &mut Self
    where
        F: Into<Function<(CommandArgs, Option<u32>, Option<Buffer>), u8>>,
    {
        self.0.preview = fun.into().into();
        self
    }

    #[inline]
    pub fn range(&mut self, range: CommandRange) -> &mut Self {
        self.0.range = range.to_object().unwrap();
        self
    }

    #[inline]
    pub fn register(&mut self, register: bool) -> &mut Self {
        self.0.register_ = register.into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> CreateCommandOpts {
        std::mem::take(&mut self.0)
    }
}
