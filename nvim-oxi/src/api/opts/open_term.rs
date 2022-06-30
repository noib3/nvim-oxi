use derive_builder::Builder;
use nvim_types::{Dictionary, Object, String as NvimString};

use crate::api::Buffer;
use crate::lua::LuaFun;

pub type OnInputArgs = (
    String,     // the string literal `"input"`
    u32,        // channel_id
    Buffer,     // buffer
    NvimString, // data input
);

/// Options passed to `crate::api::open_term`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct OpenTermOpts {
    #[builder(setter(custom))]
    on_input: Option<LuaFun<OnInputArgs, ()>>,
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
        self.on_input = Some(Some(LuaFun::from_fn_mut(fun)));
        self
    }

    pub fn build(&mut self) -> OpenTermOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl From<OpenTermOpts> for Dictionary {
    fn from(opts: OpenTermOpts) -> Self {
        Self::from_iter([("on_input", Object::from(opts.on_input))])
    }
}
