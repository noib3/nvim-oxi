use derive_builder::Builder;
use nvim_types::Object;

/// Options passed to `crate::api::eval_statusline`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct EvalStatuslineOpts {
    #[builder(setter(strip_option))]
    fillchar: Option<char>,

    #[builder(setter(strip_option))]
    highlights: Option<bool>,

    #[builder(setter(into, strip_option))]
    maxwidth: Option<u32>,

    #[builder(setter(strip_option))]
    use_tabline: Option<bool>,

    #[builder(setter(strip_option))]
    use_winbar: Option<bool>,

    // TODO: accept an `api::Window` here once that's implemented.
    #[builder(setter(into, strip_option))]
    winid: Option<u32>,
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
    fillchar: Object,
    highlights: Object,
    maxwidth: Object,
    use_tabline: Object,
    use_winbar: Object,
    winid: Object,
}

impl From<EvalStatuslineOpts> for KeyDict_eval_statusline {
    fn from(opts: EvalStatuslineOpts) -> Self {
        Self {
            fillchar: opts.fillchar.into(),
            highlights: opts.highlights.into(),
            maxwidth: opts.maxwidth.into(),
            use_tabline: opts.use_tabline.into(),
            use_winbar: opts.use_winbar.into(),
            winid: opts.winid.into(),
        }
    }
}
