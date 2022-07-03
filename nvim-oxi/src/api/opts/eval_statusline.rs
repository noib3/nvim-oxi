use derive_builder::Builder;
use nvim_types::Object;

use crate::api::Window;

/// Options passed to `crate::api::eval_statusline`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct EvalStatuslineOpts {
    /// Character used to fill blank spaces in the statusline.
    #[builder(setter(strip_option))]
    fillchar: Option<char>,

    /// Return statuline informations from `api::eval_statusline`.
    #[builder(setter(strip_option))]
    highlights: Option<bool>,

    /// Maximum width for the statusline.
    #[builder(setter(into, strip_option))]
    maxwidth: Option<u32>,

    /// Evaluate the tabline instead of the statusline. When `true` the `winid`
    /// field is ignored. Mutually exclusive with `use_winbar`.
    #[builder(setter(strip_option))]
    use_tabline: Option<bool>,

    /// Evaluate the winbar instead of the statusline. Mutually exclusive with
    /// `use_tabline`.
    #[builder(setter(strip_option))]
    use_winbar: Option<bool>,

    /// Window id of the window to use as context for the statusline.
    #[builder(setter(into, strip_option))]
    winid: Option<Window>,
}

impl EvalStatuslineOpts {
    #[inline(always)]
    pub fn builder() -> EvalStatuslineOptsBuilder {
        EvalStatuslineOptsBuilder::default()
    }
}

impl EvalStatuslineOptsBuilder {
    pub fn build(&mut self) -> EvalStatuslineOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_eval_statusline {
    winid: Object,
    fillchar: Object,
    maxwidth: Object,
    highlights: Object,
    use_winbar: Object,
    use_tabline: Object,
}

impl From<&EvalStatuslineOpts> for KeyDict_eval_statusline {
    fn from(opts: &EvalStatuslineOpts) -> Self {
        Self {
            winid: opts.winid.into(),
            fillchar: opts.fillchar.into(),
            maxwidth: opts.maxwidth.into(),
            highlights: opts.highlights.into(),
            use_winbar: opts.use_winbar.into(),
            use_tabline: opts.use_tabline.into(),
        }
    }
}
