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

// Diff between 0.7.2 and master is:
//
// `temp` -> removed
// `global` -> renamed to `global_link`
// `underdot` -> renamed to `underdotted`
// `underdash` -> renamed to `underdashed`
// `underlineline` -> renamed to `underdouble`
//
// also `nocombine` and `undercurl` don't make it to the final definition of
// `KeyDict_highlight` in nightly builds, but are still mentioned in the docs
// and are present in `keysets.lua` so idk, I'll leave them in for now.
#[derive(Default)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_highlight<'a> {
    bg: Object,
    fg: Object,
    sp: Object,
    bold: Object,
    link: NonOwning<'a, Object>,
    #[cfg(feature = "neovim-0-7")]
    temp: Object,
    blend: Object,
    cterm: NonOwning<'a, Object>,
    #[cfg(feature = "neovim-0-7")]
    global: Object,
    italic: Object,
    special: NonOwning<'a, Object>,
    ctermbg: NonOwning<'a, Object>,
    ctermfg: NonOwning<'a, Object>,
    default_: Object,
    #[cfg(feature = "neovim-nightly")]
    altfont: Object,
    reverse: Object,
    fallback: Object,
    standout: Object,
    #[cfg(feature = "neovim-0-7")]
    underdot: Object,
    nocombine: Object,
    undercurl: Object,
    #[cfg(feature = "neovim-0-7")]
    underdash: Object,
    underline: Object,
    background: NonOwning<'a, Object>,
    #[cfg(feature = "neovim-nightly")]
    bg_indexed: Object,
    foreground: NonOwning<'a, Object>,
    #[cfg(feature = "neovim-nightly")]
    fg_indexed: Object,
    #[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
    global_link: Object,
    #[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
    underdashed: Object,
    #[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
    underdotted: Object,
    #[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
    underdouble: Object,
    strikethrough: Object,
    #[cfg(feature = "neovim-0-7")]
    underlineline: Object,
}

impl<'a> From<&'a SetHighlightOpts> for KeyDict_highlight<'a> {
    fn from(opts: &'a SetHighlightOpts) -> Self {
        Self {
            bg: Object::nil(),
            fg: Object::nil(),
            sp: Object::nil(),
            bold: opts.bold.into(),
            link: opts.link.non_owning(),
            #[cfg(feature = "neovim-0-7")]
            temp: Object::nil(),
            blend: opts.blend.into(),
            cterm: opts.cterm.non_owning(),
            #[cfg(feature = "neovim-0-7")]
            global: Object::nil(),
            italic: opts.italic.into(),
            special: opts.special.non_owning(),
            ctermbg: opts.ctermbg.non_owning(),
            ctermfg: opts.ctermfg.non_owning(),
            default_: opts.default.into(),
            #[cfg(feature = "neovim-nightly")]
            altfont: opts.altfont.into(),
            reverse: opts.reverse.into(),
            fallback: Object::nil(),
            standout: opts.standout.into(),
            #[cfg(feature = "neovim-0-7")]
            underdot: opts.underdotted.into(),
            nocombine: opts.nocombine.into(),
            undercurl: opts.undercurl.into(),
            #[cfg(feature = "neovim-0-7")]
            underdash: opts.underdashed.into(),
            underline: opts.underline.into(),
            background: opts.background.non_owning(),
            #[cfg(feature = "neovim-nightly")]
            bg_indexed: opts.bg_indexed.into(),
            foreground: opts.foreground.non_owning(),
            #[cfg(feature = "neovim-nightly")]
            fg_indexed: opts.fg_indexed.into(),
            #[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
            global_link: Object::nil(),
            #[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
            underdashed: opts.underdashed.into(),
            #[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
            underdotted: opts.underdotted.into(),
            #[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
            underdouble: opts.underdouble.into(),
            strikethrough: opts.strikethrough.into(),
            #[cfg(feature = "neovim-0-7")]
            underlineline: opts.underdouble.into(),
        }
    }
}
