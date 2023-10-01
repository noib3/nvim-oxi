use oxi_types::{self as nvim, Array, Function, Object};
#[cfg(feature = "neovim-nightly")]
use oxi_types::{Boolean, BufHandle, String as NvimString};

use crate::types::AutocmdCallbackArgs;
use crate::Buffer;
use crate::StringOrInt;

pub type ShouldDeleteAutocmd = bool;

/// Options passed to [`create_autocmd()`](crate::create_autocmd).
#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CreateAutocmdOpts {
    desc: Object,
    once: Object,
    group: Object,
    buffer: Object,
    nested: Object,
    command: Object,
    pattern: Object,
    callback: Object,
}

/// Options passed to [`create_autocmd()`](crate::create_autocmd).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CreateAutocmdOpts {
    /// <callback><pattern><command><nested><buffer><group><once><desc>1
    mask: u64,

    /// 4th in the mask.
    buffer: BufHandle,

    /// 8th in the mask.
    callback: Object,

    /// 6th in the mask.
    command: NvimString,

    /// 1st in the mask.
    desc: NvimString,

    /// 3rd in the mask.
    group: Object,

    /// 5th in the mask.
    nested: Boolean,

    /// 2nd in the mask.
    once: Boolean,

    /// 7th in the mask.
    pattern: Object,
}

impl CreateAutocmdOpts {
    #[inline(always)]
    pub fn builder() -> CreateAutocmdOptsBuilder {
        CreateAutocmdOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct CreateAutocmdOptsBuilder(CreateAutocmdOpts);

impl CreateAutocmdOptsBuilder {
    /// A specific `Buffer` for buffer-local autocommands.
    #[inline]
    pub fn buffer(&mut self, buffer: Buffer) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.buffer = buffer.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.buffer = buffer.0;
            self.0.mask |= 0b10001;
        }
        self
    }

    /// Callback to execute when the autocommand is triggered. Cannot be used
    /// together with `command`.
    #[inline]
    pub fn callback<F>(&mut self, callback: F) -> &mut Self
    where
        F: Into<Function<AutocmdCallbackArgs, ShouldDeleteAutocmd>>,
    {
        self.0.callback = callback.into().into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b100000001;
        }
        self
    }

    /// Vim command to execute when the autocommand is triggered. Cannot be
    /// used together with `callback`.
    #[inline]
    pub fn command<S>(&mut self, command: S) -> &mut Self
    where
        S: Into<nvim::String>,
    {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.command = command.into().into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.command = command.into();
            self.0.mask |= 0b1000001;
        }
        self
    }

    /// Description of the autocommand.
    #[inline]
    pub fn desc<S>(&mut self, desc: S) -> &mut Self
    where
        S: Into<nvim::String>,
    {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.desc = desc.into().into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.desc = desc.into();
            self.0.mask |= 0b11;
        }
        self
    }

    /// The autocommand group name or id to match against.
    #[inline]
    pub fn group<Grp>(&mut self, group: Grp) -> &mut Self
    where
        Grp: StringOrInt,
    {
        self.0.group = group.to_object();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b1001;
        }
        self
    }

    /// Run nested autocommands.
    #[inline]
    pub fn nested(&mut self, nested: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.nested = nested.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.nested = nested;
            self.0.mask |= 0b100001;
        }
        self
    }

    /// Only run the autocommand once.
    #[inline]
    pub fn once(&mut self, once: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.once = once.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.once = once;
            self.0.mask |= 0b101;
        }
        self
    }

    /// Patterns to match against.
    #[inline]
    pub fn patterns<'a, I>(&mut self, patterns: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.0.pattern = Array::from_iter(patterns).into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b10000001;
        }
        self
    }

    #[inline]
    pub fn build(&mut self) -> CreateAutocmdOpts {
        std::mem::take(&mut self.0)
    }
}
