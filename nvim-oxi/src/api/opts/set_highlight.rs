use derive_builder::Builder;
use nvim_types::{self as nvim, NonOwning, Object};

#[derive(Clone, Debug, Default, PartialEq, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct SetHighlightOpts {
    #[builder(setter(custom))]
    fg: Object,

    #[builder(setter(custom))]
    bg: Object,

    #[builder(setter(custom))]
    special: Object,

    #[builder(setter(strip_option))]
    blend: Option<u8>,

    #[builder(setter(strip_option))]
    bold: Option<bool>,

    #[builder(setter(strip_option))]
    standout: Option<bool>,

    #[builder(setter(strip_option))]
    underline: Option<bool>,

    #[builder(setter(strip_option))]
    undercurl: Option<bool>,

    #[builder(setter(strip_option))]
    underdashed: Option<bool>,

    #[builder(setter(strip_option))]
    underdotted: Option<bool>,

    #[builder(setter(strip_option))]
    underdouble: Option<bool>,

    #[builder(setter(strip_option))]
    strikethrough: Option<bool>,

    #[builder(setter(strip_option))]
    italic: Option<bool>,

    #[builder(setter(strip_option))]
    reverse: Option<bool>,

    #[builder(setter(strip_option))]
    nocombine: Option<bool>,

    #[builder(setter(custom))]
    link: Object,

    #[builder(setter(strip_option))]
    default: Option<bool>,

    #[builder(setter(custom))]
    ctermfg: Object,

    #[builder(setter(custom))]
    ctermbg: Object,

    #[builder(setter(custom))]
    cterm: Object,
}

impl SetHighlightOpts {
    #[inline(always)]
    pub fn builder() -> SetHighlightOptsBuilder {
        <SetHighlightOptsBuilder as Default>::default()
    }
}

impl SetHighlightOptsBuilder {
    pub fn fg(&mut self, fg: impl Into<nvim::String>) -> &mut Self {
        self.fg = Some(fg.into().into());
        self
    }

    pub fn bg(&mut self, bg: impl Into<nvim::String>) -> &mut Self {
        self.bg = Some(bg.into().into());
        self
    }

    pub fn special(&mut self, special: impl Into<nvim::String>) -> &mut Self {
        self.special = Some(special.into().into());
        self
    }

    pub fn link(&mut self, link: impl Into<nvim::String>) -> &mut Self {
        self.link = Some(link.into().into());
        self
    }

    pub fn ctermbg(&mut self, ctermbg: impl Into<nvim::String>) -> &mut Self {
        self.ctermbg = Some(ctermbg.into().into());
        self
    }

    pub fn ctermfg(&mut self, ctermfg: impl Into<nvim::String>) -> &mut Self {
        self.ctermfg = Some(ctermfg.into().into());
        self
    }

    pub fn cterm(&mut self, cterm: impl Into<nvim::String>) -> &mut Self {
        self.cterm = Some(cterm.into().into());
        self
    }

    pub fn build(&mut self) -> SetHighlightOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

// Diff between 0.7.2 and master is:
//
// underdot -> underdotted
// underdash -> underdashed
// underlineline -> underdouble
//
// also `nocombine` and `undercurl` don't make it to the final definition of
// `KeyDict_highlight` in nightly builds, but are still mentioned in the docs
// and are present in `keysets.lua` so idk, I'll leave them in for now.
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_highlight<'a> {
    bg: NonOwning<'a, Object>,
    fg: NonOwning<'a, Object>,
    sp: Object,
    bold: Object,
    link: NonOwning<'a, Object>,
    temp: Object,
    blend: Object,
    cterm: NonOwning<'a, Object>,
    global: Object,
    italic: Object,
    special: NonOwning<'a, Object>,
    ctermbg: NonOwning<'a, Object>,
    ctermfg: NonOwning<'a, Object>,
    default_: Object,
    reverse: Object,
    fallback: Object,
    standout: Object,
    #[cfg(not(feature = "nightly"))]
    underdot: Object,
    nocombine: Object,
    undercurl: Object,
    #[cfg(not(feature = "nightly"))]
    underdash: Object,
    underline: Object,
    background: Object,
    foreground: Object,
    #[cfg(feature = "nightly")]
    underdashed: Object,
    #[cfg(feature = "nightly")]
    underdotted: Object,
    #[cfg(feature = "nightly")]
    underdouble: Object,
    strikethrough: Object,
    #[cfg(not(feature = "nightly"))]
    underlineline: Object,
}

impl<'a> From<&'a SetHighlightOpts> for KeyDict_highlight<'a> {
    fn from(opts: &'a SetHighlightOpts) -> Self {
        Self {
            bg: opts.bg.non_owning(),
            fg: opts.fg.non_owning(),
            sp: Object::nil(),
            bold: opts.bold.into(),
            link: opts.link.non_owning(),
            temp: Object::nil(),
            blend: opts.blend.into(),
            cterm: opts.cterm.non_owning(),
            global: Object::nil(),
            italic: opts.italic.into(),
            special: opts.special.non_owning(),
            ctermbg: opts.ctermbg.non_owning(),
            ctermfg: opts.ctermfg.non_owning(),
            default_: opts.default.into(),
            reverse: opts.reverse.into(),
            fallback: Object::nil(),
            standout: opts.standout.into(),
            #[cfg(not(feature = "nightly"))]
            underdot: opts.underdotted.into(),
            nocombine: opts.nocombine.into(),
            undercurl: opts.undercurl.into(),
            #[cfg(not(feature = "nightly"))]
            underdash: opts.underdashed.into(),
            underline: opts.underline.into(),
            background: Object::nil(),
            foreground: Object::nil(),
            #[cfg(feature = "nightly")]
            underdashed: opts.underdashed.into(),
            #[cfg(feature = "nightly")]
            underdotted: opts.underdotted.into(),
            #[cfg(feature = "nightly")]
            underdouble: opts.underdouble.into(),
            strikethrough: opts.strikethrough.into(),
            #[cfg(not(feature = "nightly"))]
            underlineline: opts.underdouble.into(),
        }
    }
}
