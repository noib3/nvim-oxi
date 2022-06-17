use derive_builder::Builder;
use nvim_types::Object;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct SetHighlightOpts {
    #[builder(setter(into, strip_option))]
    fg: Option<String>,

    #[builder(setter(into, strip_option))]
    bg: Option<String>,

    #[builder(setter(into, strip_option))]
    special: Option<String>,

    #[builder(setter(into, strip_option))]
    blend: Option<u8>,

    #[builder(setter(into, strip_option))]
    bold: Option<bool>,

    #[builder(setter(into, strip_option))]
    standout: Option<bool>,

    #[builder(setter(into, strip_option))]
    underline: Option<bool>,

    #[builder(setter(into, strip_option))]
    underlineline: Option<bool>,

    #[builder(setter(into, strip_option))]
    undercurl: Option<bool>,

    #[builder(setter(into, strip_option))]
    underdot: Option<bool>,

    #[builder(setter(into, strip_option))]
    underdash: Option<bool>,

    #[builder(setter(into, strip_option))]
    strikethrough: Option<bool>,

    #[builder(setter(into, strip_option))]
    italic: Option<bool>,

    #[builder(setter(into, strip_option))]
    reverse: Option<bool>,

    #[builder(setter(into, strip_option))]
    nocombine: Option<bool>,

    #[builder(setter(into, strip_option))]
    link: Option<String>,

    #[builder(setter(into, strip_option))]
    default: Option<bool>,

    #[builder(setter(into, strip_option))]
    ctermfg: Option<String>,

    #[builder(setter(into, strip_option))]
    ctermbg: Option<String>,

    #[builder(setter(into, strip_option))]
    cterm: Option<String>,
}

impl SetHighlightOpts {
    #[inline(always)]
    pub fn builder() -> SetHighlightOptsBuilder {
        <SetHighlightOptsBuilder as Default>::default()
    }
}

impl SetHighlightOptsBuilder {
    pub fn build(&mut self) -> SetHighlightOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_highlight {
    background: Object,
    bg: Object,
    blend: Object,
    bold: Object,
    cterm: Object,
    ctermbg: Object,
    ctermfg: Object,
    default_: Object,
    fallback: Object,
    fg: Object,
    foreground: Object,
    global: Object,
    italic: Object,
    link: Object,
    nocombine: Object,
    reverse: Object,
    sp: Object,
    special: Object,
    standout: Object,
    strikethrough: Object,
    temp: Object,
    undercurl: Object,
    underdash: Object,
    underdot: Object,
    underline: Object,
    underlineline: Object,
}

impl From<&SetHighlightOpts> for KeyDict_highlight {
    fn from(opts: &SetHighlightOpts) -> Self {
        Self {
            background: Object::nil(),
            bg: opts.bg.clone().into(),
            blend: opts.blend.into(),
            bold: opts.bold.into(),
            cterm: opts.cterm.clone().into(),
            ctermbg: opts.ctermbg.clone().into(),
            ctermfg: opts.ctermfg.clone().into(),
            default_: opts.default.into(),
            fallback: Object::nil(),
            fg: opts.fg.clone().into(),
            foreground: Object::nil(),
            global: Object::nil(),
            italic: opts.italic.into(),
            link: opts.link.clone().into(),
            nocombine: opts.nocombine.into(),
            reverse: opts.reverse.into(),
            sp: Object::nil(),
            special: opts.special.clone().into(),
            standout: opts.standout.into(),
            strikethrough: opts.strikethrough.into(),
            temp: Object::nil(),
            undercurl: opts.undercurl.into(),
            underdash: opts.underdash.into(),
            underdot: opts.underdot.into(),
            underline: opts.underline.into(),
            underlineline: opts.underlineline.into(),
        }
    }
}
