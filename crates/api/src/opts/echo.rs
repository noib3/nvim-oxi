#[cfg(feature = "neovim-nightly")] // Only on Nightly.
use crate::StringOrInt;
#[cfg(feature = "neovim-nightly")] // Only on Nightly.
use crate::types::ProgressMessageStatus;

/// Options passed to [`echo()`](crate::echo).
#[cfg(feature = "neovim-nightly")] // Only on Nightly.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct EchoOpts {
    #[builder(mask)]
    mask: u64,

    /// Treat the message like `:echoerr`.
    #[builder(argtype = "bool")]
    err: types::Boolean,

    /// Message is controlled by the `verbose` option. Nvim invoked with
    /// `-V3log` will write the message to the "log" file instead of standard
    /// output.
    #[builder(argtype = "bool")]
    verbose: types::Boolean,

    /// Set the `ui-messages` kind with which this message will be emitted.
    #[builder(
        generics = "S: Into<types::String>",
        argtype = "S",
        inline = "{0}.into()"
    )]
    kind: types::String,

    /// Message id for updating existing message.
    #[builder(
        generics = "Id: StringOrInt",
        argtype = "Id",
        inline = "{0}.to_object()"
    )]
    id: types::Object,

    /// The title of the progress message.
    #[builder(
        generics = "S: Into<types::String>",
        argtype = "S",
        inline = "{0}.into()"
    )]
    title: types::String,

    /// The current status of the progress message.
    #[builder(
        argtype = "ProgressMessageStatus",
        inline = "{0}.as_str().into()"
    )]
    status: types::String,

    /// How much progress is done on the progress message.
    #[builder(argtype = "u8", inline = "{0}.into()")]
    percent: types::Integer,

    /// Dictionary containing additional information.
    data: types::Dictionary,
}

/// Options passed to [`echo()`](crate::echo).
#[derive(Clone, Debug, Default)]
#[cfg(not(feature = "neovim-nightly"))] // On 0.10 and 0.11
#[repr(C)]
pub struct EchoOpts {
    #[cfg(feature = "neovim-0-11")] // Only on 0.11.
    err: bool,
    verbose: bool,
}

#[cfg(not(feature = "neovim-nightly"))] // On 0.10 and 0.11
impl EchoOpts {
    #[inline(always)]
    pub fn builder() -> EchoOptsBuilder {
        EchoOptsBuilder::default()
    }
}

#[cfg(not(feature = "neovim-nightly"))] // On 0.10 and 0.11
#[derive(Clone, Default)]
pub struct EchoOptsBuilder(EchoOpts);

#[cfg(not(feature = "neovim-nightly"))] // On 0.10 and 0.11
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
