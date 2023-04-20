use nvim_types::{self as nvim, Dictionary, Object};

use crate::Buffer;
use crate::ToFunction;

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

#[derive(Clone, Debug, Default)]
pub struct OpenTermOpts {
    on_input: Object,
}

impl OpenTermOpts {
    /// Creates a new [`OpenTermOptsBuilder`].
    #[inline]
    pub fn builder() -> OpenTermOptsBuilder {
        OpenTermOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct OpenTermOptsBuilder(OpenTermOpts);

impl OpenTermOptsBuilder {
    /// Callback invoked on data input (like keypresses in terminal mode).
    #[inline]
    pub fn on_input<F>(&mut self, fun: F) -> &mut Self
    where
        F: ToFunction<OnInputArgs, ()>,
    {
        self.0.on_input = fun.to_object();
        self
    }

    #[inline]
    pub fn build(&mut self) -> OpenTermOpts {
        std::mem::take(&mut self.0)
    }
}

impl From<&OpenTermOpts> for Dictionary {
    fn from(opts: &OpenTermOpts) -> Self {
        Self::from_iter([("on_input", opts.on_input.clone())])
    }
}
