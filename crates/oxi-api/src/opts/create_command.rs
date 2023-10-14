#[cfg(feature = "neovim-nightly")]
use oxi_types::Boolean;
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

/// Options passed to [`create_user_command`](crate::create_user_command) and
/// [`Buffer::create_user_command()`](crate::Buffer::create_user_command).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CreateCommandOpts {
    mask: u64,

    /// 2nd in the mask.
    addr: Object,

    /// 3rd in the mask.
    bang: Boolean,

    /// 1st in the mask.
    bar: Boolean,

    /// 10th in the mask.
    complete: Object,

    /// 5th in the mask.
    count: Object,

    /// 4th in the mask.
    desc: Object,

    /// 6th in the mask.
    force: Boolean,

    /// 12th in the mask.
    keepscript: Boolean,

    /// 7th in the mask.
    nargs: Object,

    /// 9th in the mask.
    preview: Object,

    /// 8th in the mask.
    range: Object,

    /// 11th in the mask.
    register_: Boolean,
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
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b101;
        }
        self
    }

    #[inline]
    pub fn bang(&mut self, bang: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.bang = bang.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.bang = bang;
            self.0.mask |= 0b1001;
        }
        self
    }

    #[inline]
    pub fn bar(&mut self, bar: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.bar = bar.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.bar = bar;
            self.0.mask |= 0b11;
        }
        self
    }

    #[inline]
    pub fn complete(&mut self, complete: CommandComplete) -> &mut Self {
        self.0.complete = complete.to_object().unwrap();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b10000000001;
        }
        self
    }

    #[inline]
    pub fn count(&mut self, count: impl Into<Integer>) -> &mut Self {
        self.0.count = count.into().into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b100001;
        }
        self
    }

    /// Description for the command.
    #[inline]
    pub fn desc(&mut self, desc: impl Into<nvim::String>) -> &mut Self {
        self.0.desc = desc.into().into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b10001;
        }
        self
    }

    #[inline]
    pub fn force(&mut self, force: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.force = force.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.force = force;
            self.0.mask |= 0b1000001;
        }
        self
    }

    #[inline]
    pub fn keepscript(&mut self, keepscript: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.keepscript = keepscript.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.keepscript = keepscript;
            self.0.mask |= 0b1000000000001;
        }
        self
    }

    #[inline]
    pub fn nargs(&mut self, nargs: CommandNArgs) -> &mut Self {
        self.0.nargs = nargs.to_object().unwrap();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b10000001;
        }
        self
    }

    #[inline]
    pub fn preview<F>(&mut self, fun: F) -> &mut Self
    where
        F: Into<Function<(CommandArgs, Option<u32>, Option<Buffer>), u8>>,
    {
        self.0.preview = fun.into().into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b1000000001;
        }
        self
    }

    #[inline]
    pub fn range(&mut self, range: CommandRange) -> &mut Self {
        self.0.range = range.to_object().unwrap();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b100000001;
        }
        self
    }

    #[inline]
    pub fn register(&mut self, register: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.register_ = register.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.register_ = register;
            self.0.mask |= 0b100000000001;
        }
        self
    }

    #[inline]
    pub fn build(&mut self) -> CreateCommandOpts {
        std::mem::take(&mut self.0)
    }
}
