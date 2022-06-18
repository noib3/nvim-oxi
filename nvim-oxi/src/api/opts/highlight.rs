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
    bg: Object,
    fg: Object,
    sp: Object,
    bold: Object,
    link: Object,
    temp: Object,
    blend: Object,
    cterm: Object,
    global: Object,
    italic: Object,
    special: Object,
    ctermbg: Object,
    ctermfg: Object,
    default_: Object,
    reverse: Object,
    fallback: Object,
    standout: Object,
    underdot: Object,
    nocombine: Object,
    undercurl: Object,
    underdash: Object,
    underline: Object,
    background: Object,
    foreground: Object,
    strikethrough: Object,
    underlineline: Object,
}

impl From<&SetHighlightOpts> for KeyDict_highlight {
    fn from(opts: &SetHighlightOpts) -> Self {
        Self {
            bg: opts.bg.clone().into(),
            fg: opts.fg.clone().into(),
            sp: Object::nil(),
            bold: opts.bold.into(),
            link: opts.link.clone().into(),
            temp: Object::nil(),
            blend: opts.blend.into(),
            cterm: opts.cterm.clone().into(),
            global: Object::nil(),
            italic: opts.italic.into(),
            special: opts.special.clone().into(),
            ctermbg: opts.ctermbg.clone().into(),
            ctermfg: opts.ctermfg.clone().into(),
            default_: opts.default.into(),
            reverse: opts.reverse.into(),
            fallback: Object::nil(),
            standout: opts.standout.into(),
            underdot: opts.underdot.into(),
            nocombine: opts.nocombine.into(),
            undercurl: opts.undercurl.into(),
            underdash: opts.underdash.into(),
            underline: opts.underline.into(),
            background: Object::nil(),
            foreground: Object::nil(),
            strikethrough: opts.strikethrough.into(),
            underlineline: opts.underlineline.into(),
        }
    }
}
