use derive_builder::Builder;
use nvim_types::{self as nvim, conversion::FromObject, NonOwning, Object};
use serde::Serialize;

/// Options passed to
/// [`nvim_oxi::api::set_option_value`](crate::set_option_value).
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct OptionValueOpts {
    #[builder(setter(strip_option))]
    buffer: Option<crate::Buffer>,

    #[builder(setter(custom))]
    scope: Object,

    #[builder(setter(into, strip_option))]
    window: Option<crate::Window>,
}

impl OptionValueOpts {
    #[inline(always)]
    pub fn builder() -> OptionValueOptsBuilder {
        OptionValueOptsBuilder::default()
    }
}

impl OptionValueOptsBuilder {
    pub fn scope(&mut self, scope: OptionScope) -> &mut Self {
        self.scope = Some(nvim::String::from(scope).into());
        self
    }

    pub fn build(&mut self) -> OptionValueOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OptionScope {
    Global,
    Local,
}

impl From<OptionScope> for nvim::String {
    fn from(ctx: OptionScope) -> Self {
        nvim::String::from_object(
            ctx.serialize(nvim::serde::Serializer::new())
                .expect("`OptionScope` is serializable"),
        )
        .expect("`OptionScope` is serialized into a string")
    }
}

#[cfg(not(feature = "neovim-nightly"))]
#[derive(Default)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_option<'a> {
    buf: Object,
    win: Object,
    scope: NonOwning<'a, Object>,
}

#[cfg(feature = "neovim-nightly")]
#[derive(Default)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_option<'a> {
    scope: NonOwning<'a, Object>,
    win: Object,
    buf: Object,
    filetype: Object,
}

impl<'a> From<&'a OptionValueOpts> for KeyDict_option<'a> {
    fn from(opts: &'a OptionValueOpts) -> Self {
        Self {
            buf: opts.buffer.as_ref().into(),
            win: opts.window.as_ref().into(),
            scope: opts.scope.non_owning(),
            #[cfg(feature = "neovim-nightly")]
            filetype: Object::nil(), // TODO: update this
        }
    }
}
