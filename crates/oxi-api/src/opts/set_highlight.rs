use oxi_types::{self as nvim, Object};

/// Options passed to [`set_hl()`](crate::set_hl).
#[cfg(not(feature = "neovim-nightly"))]
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
    default_: Object,
    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    altfont: Object,
    reverse: Object,
    fallback: Object,
    standout: Object,
    nocombine: Object,
    undercurl: Object,
    underline: Object,
    background: Object,
    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    bg_indexed: Object,
    foreground: Object,
    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    fg_indexed: Object,
    global_link: Object,
    underdashed: Object,
    underdotted: Object,
    underdouble: Object,
    strikethrough: Object,
}

#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct SetHighlightOpts {
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
    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    altfont: Object,
    nocombine: Object,
    default_: Object,
    cterm: Object,
    foreground: Object,
    fg: Object,
    background: Object,
    bg: Object,
    ctermfg: Object,
    ctermbg: Object,
    special: Object,
    sp: Object,
    link: Object,
    global_link: Object,
    fallback: Object,
    blend: Object,
    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    fg_indexed: Object,
    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    bg_indexed: Object,
}

impl SetHighlightOpts {
    /// Creates a new [`SetHighlightOptsBuilder`].
    #[inline]
    pub fn builder() -> SetHighlightOptsBuilder {
        <SetHighlightOptsBuilder as Default>::default()
    }
}

#[derive(Clone, Default)]
pub struct SetHighlightOptsBuilder(SetHighlightOpts);

impl SetHighlightOptsBuilder {
    #[inline]
    pub fn background(&mut self, background: &str) -> &mut Self {
        self.0.background = nvim::String::from(background).into();
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
        self.0.cterm = nvim::String::from(cterm).into();
        self
    }

    #[inline]
    pub fn ctermbg(&mut self, ctermbg: &str) -> &mut Self {
        self.0.ctermbg = nvim::String::from(ctermbg).into();
        self
    }

    #[inline]
    pub fn ctermfg(&mut self, ctermfg: &str) -> &mut Self {
        self.0.ctermfg = nvim::String::from(ctermfg).into();
        self
    }

    #[inline]
    pub fn default(&mut self, default: bool) -> &mut Self {
        self.0.default_ = default.into();
        self
    }

    #[inline]
    pub fn foreground(&mut self, foreground: &str) -> &mut Self {
        self.0.foreground = nvim::String::from(foreground).into();
        self
    }

    #[inline]
    pub fn italic(&mut self, italic: bool) -> &mut Self {
        self.0.italic = italic.into();
        self
    }

    #[inline]
    pub fn link(&mut self, link: &str) -> &mut Self {
        self.0.link = nvim::String::from(link).into();
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
        self.0.special = nvim::String::from(special).into();
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

    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    #[inline]
    pub fn altfont(&mut self, altfont: bool) -> &mut Self {
        self.0.altfont = altfont.into();
        self
    }

    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    #[inline]
    pub fn bg_indexed(&mut self, bg_indexed: bool) -> &mut Self {
        self.0.bg_indexed = bg_indexed.into();
        self
    }

    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
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
