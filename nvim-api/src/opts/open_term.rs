use derive_builder::Builder;
use nvim_types::{self as nvim, Dictionary, Object};

use crate::trait_utils::ToFunction;
use crate::Buffer;

/// Arguments passed to the callback registered to
/// [`on_input`](OpenTermOptsBuilder::on_input). The `(a, b, c, d)` tuple
/// represents:
///
/// - `a`: the string literal `"input"`;
/// - `b`: channel id;
/// - `c`: the [`Buffer`] associated to the terminal instance;
/// - `d`: data input.
pub type OnInputArgs = (
    String,       // the string literal `"input"`
    u32,          // channel_id
    Buffer,       // buffer
    nvim::String, // data input
);

#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct OpenTermOpts {
    #[builder(setter(custom))]
    on_input: Object,
}

impl OpenTermOpts {
    #[inline(always)]
    /// Creates a new [`OpenTermOptsBuilder`].
    pub fn builder() -> OpenTermOptsBuilder {
        OpenTermOptsBuilder::default()
    }
}

impl OpenTermOptsBuilder {
    /// Callback invoked on data input (like keypresses in terminal mode).
    pub fn on_input<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnInputArgs, ()>,
    {
        self.on_input = Some(fun.to_obj());
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
