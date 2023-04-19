use derive_builder::Builder;
use nvim_types::{self as nvim, NonOwning, Object};

/// Options passed to [`nvim_oxi::api::set_hl`](crate::set_hl).
#[derive(Clone, Debug, Default, PartialEq, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct SetHighlightOpts {
    #[builder(setter(custom))]
    background: Object,

    #[builder(setter(strip_option))]
    blend: Option<u8>,

    #[builder(setter(strip_option))]
    bold: Option<bool>,

    #[builder(setter(custom))]
    cterm: Object,

    #[builder(setter(custom))]
    ctermbg: Object,

    #[builder(setter(custom))]
    ctermfg: Object,

    #[builder(setter(strip_option))]
    default: Option<bool>,

    #[builder(setter(custom))]
    foreground: Object,

    #[builder(setter(strip_option))]
    italic: Option<bool>,

    #[builder(setter(custom))]
    link: Object,

    #[builder(setter(strip_option))]
    nocombine: Option<bool>,

    #[builder(setter(strip_option))]
    reverse: Option<bool>,

    #[builder(setter(custom))]
    special: Object,

    #[builder(setter(strip_option))]
    standout: Option<bool>,

    #[builder(setter(strip_option))]
    strikethrough: Option<bool>,

    #[builder(setter(strip_option))]
    undercurl: Option<bool>,

    #[builder(setter(strip_option))]
    underdashed: Option<bool>,

    #[builder(setter(strip_option))]
    underdotted: Option<bool>,

    #[builder(setter(strip_option))]
    underdouble: Option<bool>,

    #[builder(setter(strip_option))]
    underline: Option<bool>,

    #[builder(setter(strip_option))]
    altfont: Option<bool>,

    #[builder(setter(strip_option))]
    bg_indexed: Option<bool>,

    #[builder(setter(strip_option))]
    fg_indexed: Option<bool>,
}

impl SetHighlightOpts {
    #[inline(always)]
    /// Creates a new [`SetHighlightOptsBuilder`].
    pub fn builder() -> SetHighlightOptsBuilder {
        <SetHighlightOptsBuilder as Default>::default()
    }
}

impl SetHighlightOptsBuilder {
    pub fn background(&mut self, background: &str) -> &mut Self {
        self.background = Some(nvim::String::from(background).into());
        self
    }

    pub fn cterm(&mut self, cterm: &str) -> &mut Self {
        self.cterm = Some(nvim::String::from(cterm).into());
        self
    }

    pub fn ctermbg(&mut self, ctermbg: &str) -> &mut Self {
        self.ctermbg = Some(nvim::String::from(ctermbg).into());
        self
    }

    pub fn ctermfg(&mut self, ctermfg: &str) -> &mut Self {
        self.ctermfg = Some(nvim::String::from(ctermfg).into());
        self
    }

    pub fn foreground(&mut self, foreground: &str) -> &mut Self {
        self.foreground = Some(nvim::String::from(foreground).into());
        self
    }

    pub fn link(&mut self, link: &str) -> &mut Self {
        self.link = Some(nvim::String::from(link).into());
        self
    }

    pub fn special(&mut self, special: &str) -> &mut Self {
        self.special = Some(nvim::String::from(special).into());
        self
    }

    pub fn build(&mut self) -> SetHighlightOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[cfg(not(feature = "neovim-nightly"))]
#[derive(Default)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_highlight<'a> {
    bg: Object,
    fg: Object,
    sp: Object,
    bold: Object,
    link: NonOwning<'a, Object>,
    blend: Object,
    cterm: NonOwning<'a, Object>,
    italic: Object,
    special: NonOwning<'a, Object>,
    ctermbg: NonOwning<'a, Object>,
    ctermfg: NonOwning<'a, Object>,
    default_: Object,
    #[cfg(not(feature = "neovim-0-8"))]
    altfont: Object,
    reverse: Object,
    fallback: Object,
    standout: Object,
    nocombine: Object,
    undercurl: Object,
    underline: Object,
    background: NonOwning<'a, Object>,
    #[cfg(not(feature = "neovim-0-8"))]
    bg_indexed: Object,
    foreground: NonOwning<'a, Object>,
    #[cfg(not(feature = "neovim-0-8"))]
    fg_indexed: Object,
    global_link: Object,
    underdashed: Object,
    underdotted: Object,
    underdouble: Object,
    strikethrough: Object,
}

#[cfg(feature = "neovim-nightly")]
#[derive(Default)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_highlight<'a> {
    bold: Object,
    standout: Object,
    strikethrough: Object,
    underline: Object,
    undercurl: Object,
    underdouble: Object,
    underdotted: Object,
    underdashed: Object,
    italic: Object,
    reverse: Object,
    #[cfg(not(feature = "neovim-0-8"))]
    altfont: Object,
    nocombine: Object,
    default_: Object,
    cterm: NonOwning<'a, Object>,
    foreground: NonOwning<'a, Object>,
    fg: Object,
    background: NonOwning<'a, Object>,
    bg: Object,
    ctermfg: NonOwning<'a, Object>,
    ctermbg: NonOwning<'a, Object>,
    special: NonOwning<'a, Object>,
    sp: Object,
    link: NonOwning<'a, Object>,
    global_link: Object,
    fallback: Object,
    blend: Object,
    #[cfg(not(feature = "neovim-0-8"))]
    fg_indexed: Object,
    #[cfg(not(feature = "neovim-0-8"))]
    bg_indexed: Object,
}

impl<'a> From<&'a SetHighlightOpts> for KeyDict_highlight<'a> {
    fn from(opts: &'a SetHighlightOpts) -> Self {
        Self {
            bg: Object::nil(),
            fg: Object::nil(),
            sp: Object::nil(),
            bold: opts.bold.into(),
            link: opts.link.non_owning(),
            blend: opts.blend.into(),
            cterm: opts.cterm.non_owning(),
            italic: opts.italic.into(),
            special: opts.special.non_owning(),
            ctermbg: opts.ctermbg.non_owning(),
            ctermfg: opts.ctermfg.non_owning(),
            default_: opts.default.into(),
            #[cfg(not(feature = "neovim-0-8"))]
            altfont: opts.altfont.into(),
            reverse: opts.reverse.into(),
            fallback: Object::nil(),
            standout: opts.standout.into(),
            nocombine: opts.nocombine.into(),
            undercurl: opts.undercurl.into(),
            underline: opts.underline.into(),
            background: opts.background.non_owning(),
            #[cfg(not(feature = "neovim-0-8"))]
            bg_indexed: opts.bg_indexed.into(),
            foreground: opts.foreground.non_owning(),
            #[cfg(not(feature = "neovim-0-8"))]
            fg_indexed: opts.fg_indexed.into(),
            global_link: Object::nil(),
            underdashed: opts.underdashed.into(),
            underdotted: opts.underdotted.into(),
            underdouble: opts.underdouble.into(),
            strikethrough: opts.strikethrough.into(),
        }
    }
}
