use types::conversion::ToObject;

use crate::types::{
    CommandAddr,
    CommandArgs,
    CommandComplete,
    CommandNArgs,
    CommandRange,
};
use crate::Buffer;

/// Options passed to [`create_user_command`](crate::create_user_command) and
/// [`Buffer::create_user_command()`](crate::Buffer::create_user_command).
#[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct CreateCommandOpts {
    #[builder(mask)]
    mask: u64,

    #[builder(argtype = "CommandAddr", inline = "{0}.to_object().unwrap()")]
    addr: types::Object,

    #[builder(argtype = "bool")]
    bang: types::Boolean,

    #[builder(argtype = "bool")]
    bar: types::Boolean,

    #[builder(
        argtype = "CommandComplete",
        inline = "{0}.to_object().unwrap()"
    )]
    complete: types::Object,

    // TODO: fix `builder(Into)`.
    #[builder(
        generics = "C: Into<types::Integer>",
        argtype = "C",
        inline = "{0}.into().into()"
    )]
    count: types::Object,

    /// Description for the command.
    #[builder(
        generics = "C: Into<types::String>",
        argtype = "C",
        inline = "{0}.into().into()"
    )]
    desc: types::Object,

    #[builder(argtype = "bool")]
    force: types::Boolean,

    #[builder(argtype = "bool")]
    keepscript: types::Boolean,

    #[builder(argtype = "CommandNArgs", inline = "{0}.to_object().unwrap()")]
    nargs: types::Object,

    #[builder(
        generics = r#"F: Into<types::Function<(CommandArgs, Option<u32>, Option<Buffer>), u8>>"#,
        argtype = "F",
        inline = "{0}.into().into()"
    )]
    preview: types::Object,

    #[builder(argtype = "CommandRange", inline = "{0}.to_object().unwrap()")]
    range: types::Object,

    #[builder(method = "register", argtype = "bool")]
    register_: types::Boolean,
}

/// Options passed to
/// [`Buffer::create_user_command()`](crate::Buffer::create_user_command).
#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CreateCommandOpts {
    bar: types::Object,
    addr: types::Object,
    bang: types::Object,
    desc: types::Object,
    count: types::Object,
    force: types::Object,
    nargs: types::Object,
    range: types::Object,
    preview: types::Object,
    complete: types::Object,
    register_: types::Object,
    keepscript: types::Object,
}

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
impl CreateCommandOpts {
    #[inline(always)]
    /// Creates a new [`CreateCommandOptsBuilder`].
    pub fn builder() -> CreateCommandOptsBuilder {
        CreateCommandOptsBuilder::default()
    }
}

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
#[derive(Clone, Default)]
pub struct CreateCommandOptsBuilder(CreateCommandOpts);

#[cfg(not(feature = "neovim-0-10"))] // 0nly on 0.9.
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
    pub fn count(&mut self, count: impl Into<types::Integer>) -> &mut Self {
        self.0.count = count.into().into();
        self
    }

    /// Description for the command.
    #[inline]
    pub fn desc<S: Into<types::String>>(&mut self, desc: S) -> &mut Self {
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
        F: Into<
            types::Function<(CommandArgs, Option<u32>, Option<Buffer>), u8>,
        >,
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
