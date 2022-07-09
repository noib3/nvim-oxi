use derive_builder::Builder;
use nvim_types::{self as nvim, Dictionary, Object};

use crate::api::Buffer;
use crate::lua::Function;

pub type OnInputArgs = (
    String,       // the string literal `"input"`
    u32,          // channel_id
    Buffer,       // buffer
    nvim::String, // data input
);

/// Options passed to `crate::api::open_term`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct OpenTermOpts {
    #[builder(setter(custom))]
    on_input: Object,
}

impl OpenTermOpts {
    #[inline(always)]
    pub fn builder() -> OpenTermOptsBuilder {
        OpenTermOptsBuilder::default()
    }
}

impl OpenTermOptsBuilder {
    pub fn on_input<F>(&mut self, fun: F) -> &mut Self
    where
        F: FnMut(OnInputArgs) -> crate::Result<()> + 'static,
    {
        self.on_input = Some(Function::from_fn_mut(fun).into());
        self
    }

    pub fn build(&mut self) -> OpenTermOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl From<&OpenTermOpts> for Dictionary {
    fn from(opts: &OpenTermOpts) -> Self {
        Self::from_iter([("on_input", opts.on_input.clone())])
    }
}
