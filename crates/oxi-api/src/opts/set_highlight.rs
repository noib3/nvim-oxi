use oxi_types::{self as types, Object};
#[cfg(feature = "neovim-nightly")]
use oxi_types::{Boolean, Integer};

#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default, PartialEq, oxi_macros::OptsBuilder)]
#[repr(C)]
pub struct SetHighlightOpts {
    #[builder(mask)]
    mask: u64,

    #[builder(argtype = "bool")]
    bold: Boolean,

    #[builder(argtype = "bool")]
    standout: Boolean,

    #[builder(argtype = "bool")]
    strikethrough: Boolean,

    #[builder(argtype = "bool")]
    underline: Boolean,

    #[builder(argtype = "bool")]
    undercurl: Boolean,

    #[builder(argtype = "bool")]
    underdouble: Boolean,

    #[builder(argtype = "bool")]
    underdotted: Boolean,

    #[builder(argtype = "bool")]
    underdashed: Boolean,

    #[builder(argtype = "bool")]
    italic: Boolean,

    #[builder(argtype = "bool")]
    reverse: Boolean,

    #[builder(argtype = "bool")]
    altfont: Boolean,

    #[builder(argtype = "bool")]
    nocombine: Boolean,

    #[builder(method = "builder", argtype = "bool")]
    default_: Boolean,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    cterm: Object,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    foreground: Object,

    #[builder(skip)]
    fg: Object,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    background: Object,

    #[builder(skip)]
    bg: Object,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    ctermfg: Object,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    ctermbg: Object,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    special: Object,

    #[builder(skip)]
    sp: Object,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    link: Object,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    global_link: Object,

    #[builder(argtype = "bool")]
    fallback: Boolean,

    #[builder(argtype = "u8", inline = "{0} as Integer")]
    blend: Integer,

    #[builder(argtype = "bool")]
    fg_indexed: Boolean,

    #[builder(argtype = "bool")]
    bg_indexed: Boolean,

    #[builder(argtype = "bool")]
    force: Boolean,
}

/// Options passed to [`set_hl()`](crate::set_hl).
#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
#[derive(Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct SetHighlightOpts {
    bg: Object,
    fg: Object,
    sp: Object,
    bold: Object,
    link: Object,
    blend: Object,
    cterm: Object,
    italic: Object,
    special: Object,
    ctermbg: Object,
    ctermfg: Object,
    default: Object,
    #[cfg(not(feature = "neovim-0-8"))]
    altfont: Object,
    reverse: Object,
    fallback: Object,
    standout: Object,
    nocombine: Object,
    undercurl: Object,
    underline: Object,
    background: Object,
    #[cfg(not(feature = "neovim-0-8"))]
    bg_indexed: Object,
    foreground: Object,
    #[cfg(not(feature = "neovim-0-8"))]
    fg_indexed: Object,
    global_link: Object,
    underdashed: Object,
    underdotted: Object,
    underdouble: Object,
    strikethrough: Object,
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl SetHighlightOpts {
    /// Creates a new [`SetHighlightOptsBuilder`].
    #[inline]
    pub fn builder() -> SetHighlightOptsBuilder {
        <SetHighlightOptsBuilder as Default>::default()
    }
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
#[derive(Clone, Default)]
pub struct SetHighlightOptsBuilder(SetHighlightOpts);

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl SetHighlightOptsBuilder {
    #[inline]
    pub fn background(&mut self, background: &str) -> &mut Self {
        self.0.background = types::String::from(background).into();
        self
    }

    #[inline]
    pub fn blend(&mut self, blend: u8) -> &mut Self {
        self.0.blend = blend.into();
        self
    }

    #[inline]
    pub fn bold(&mut self, bold: bool) -> &mut Self {
        self.0.bold = bold.into();
        self
    }

    #[inline]
    pub fn cterm(&mut self, cterm: &str) -> &mut Self {
        self.0.cterm = types::String::from(cterm).into();
        self
    }

    #[inline]
    pub fn ctermbg(&mut self, ctermbg: &str) -> &mut Self {
        self.0.ctermbg = types::String::from(ctermbg).into();
        self
    }

    #[inline]
    pub fn ctermfg(&mut self, ctermfg: &str) -> &mut Self {
        self.0.ctermfg = types::String::from(ctermfg).into();
        self
    }

    #[inline]
    pub fn default(&mut self, default: bool) -> &mut Self {
        self.0.default = default.into();
        self
    }

    #[inline]
    pub fn fallback(&mut self, fallback: bool) -> &mut Self {
        self.0.fallback = fallback.into();
        self
    }

    #[inline]
    pub fn foreground(&mut self, foreground: &str) -> &mut Self {
        self.0.foreground = types::String::from(foreground).into();
        self
    }

    #[inline]
    pub fn global_link(&mut self, global_link: &str) -> &mut Self {
        self.0.global_link = types::String::from(global_link).into();
        self
    }

    #[inline]
    pub fn italic(&mut self, italic: bool) -> &mut Self {
        self.0.italic = italic.into();
        self
    }

    #[inline]
    pub fn link(&mut self, link: &str) -> &mut Self {
        self.0.link = types::String::from(link).into();
        self
    }

    #[inline]
    pub fn nocombine(&mut self, nocombine: bool) -> &mut Self {
        self.0.nocombine = nocombine.into();
        self
    }

    #[inline]
    pub fn reverse(&mut self, reverse: bool) -> &mut Self {
        self.0.reverse = reverse.into();
        self
    }

    #[inline]
    pub fn special(&mut self, special: &str) -> &mut Self {
        self.0.special = types::String::from(special).into();
        self
    }

    #[inline]
    pub fn standout(&mut self, standout: bool) -> &mut Self {
        self.0.standout = standout.into();
        self
    }

    #[inline]
    pub fn strikethrough(&mut self, strikethrough: bool) -> &mut Self {
        self.0.strikethrough = strikethrough.into();
        self
    }

    #[inline]
    pub fn undercurl(&mut self, undercurl: bool) -> &mut Self {
        self.0.undercurl = undercurl.into();
        self
    }

    #[inline]
    pub fn underdashed(&mut self, underdashed: bool) -> &mut Self {
        self.0.underdashed = underdashed.into();
        self
    }

    #[inline]
    pub fn underdotted(&mut self, underdotted: bool) -> &mut Self {
        self.0.underdotted = underdotted.into();
        self
    }

    #[inline]
    pub fn underdouble(&mut self, underdouble: bool) -> &mut Self {
        self.0.underdouble = underdouble.into();
        self
    }

    #[inline]
    pub fn underline(&mut self, underline: bool) -> &mut Self {
        self.0.underline = underline.into();
        self
    }

    #[cfg(not(feature = "neovim-0-8"))]
    #[inline]
    pub fn altfont(&mut self, altfont: bool) -> &mut Self {
        self.0.altfont = altfont.into();
        self
    }

    #[cfg(not(feature = "neovim-0-8"))]
    #[inline]
    pub fn bg_indexed(&mut self, bg_indexed: bool) -> &mut Self {
        self.0.bg_indexed = bg_indexed.into();
        self
    }

    #[cfg(not(feature = "neovim-0-8"))]
    #[inline]
    pub fn fg_indexed(&mut self, fg_indexed: bool) -> &mut Self {
        self.0.fg_indexed = fg_indexed.into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> SetHighlightOpts {
        std::mem::take(&mut self.0)
    }
}
