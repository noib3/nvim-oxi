use oxi_types::{self as nvim, Object};
#[cfg(feature = "neovim-nightly")]
use oxi_types::{Boolean, Integer};

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

#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct SetHighlightOpts {
    mask: u64,

    /// 4th in the mask.
    bold: Boolean,

    /// 17th in the mask.
    standout: Boolean,

    /// 29th in the mask.
    strikethrough: Boolean,

    /// 20th in the mask.
    underline: Boolean,

    /// 19th in the mask.
    undercurl: Boolean,

    /// 28th in the mask.
    underdouble: Boolean,

    /// 27th in the mask.
    underdotted: Boolean,

    /// 26th in the mask.
    underdashed: Boolean,

    /// 9th in the mask.
    italic: Boolean,

    /// 15th in the mask.
    reverse: Boolean,

    /// 14th in the mask.
    altfont: Boolean,

    /// 18th in the mask.
    nocombine: Boolean,

    /// 13th in the mask.
    default: Boolean,

    /// 7th in the mask.
    cterm: Object,

    /// 23th in the mask.
    foreground: Object,

    /// 2nd in the mask.
    fg: Object,

    /// 21th in the mask.
    background: Object,

    /// 1st in the mask.
    bg: Object,

    /// 12th in the mask.
    ctermfg: Object,

    /// 11th in the mask.
    ctermbg: Object,

    /// 10th in the mask.
    special: Object,

    /// 3rd in the mask.
    sp: Object,

    /// 5th in the mask.
    link: Object,

    /// 25th in the mask.
    global_link: Object,

    /// 16th in the mask.
    fallback: Boolean,

    /// 6th in the mask.
    blend: Integer,

    /// 24th in the mask.
    fg_indexed: Boolean,

    /// 22th in the mask.
    bg_indexed: Boolean,

    /// 8th in the mask.
    force: Boolean,
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
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b1000000000000000000001;
        }
        self
    }

    #[inline]
    pub fn blend(&mut self, blend: u8) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.blend = blend.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.blend = blend as Integer;
            self.0.mask |= 0b1000001;
        }
        self
    }

    #[inline]
    pub fn bold(&mut self, bold: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.bold = bold.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.bold = bold;
            self.0.mask |= 0b10001;
        }
        self
    }

    #[inline]
    pub fn cterm(&mut self, cterm: &str) -> &mut Self {
        self.0.cterm = nvim::String::from(cterm).into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b10000001;
        }
        self
    }

    #[inline]
    pub fn ctermbg(&mut self, ctermbg: &str) -> &mut Self {
        self.0.ctermbg = nvim::String::from(ctermbg).into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b100000000001;
        }
        self
    }

    #[inline]
    pub fn ctermfg(&mut self, ctermfg: &str) -> &mut Self {
        self.0.ctermfg = nvim::String::from(ctermfg).into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b1000000000001;
        }
        self
    }

    #[inline]
    pub fn default(&mut self, default: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.default = default.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.default = default;
            self.0.mask |= 0b10000000000001;
        }
        self
    }

    #[inline]
    pub fn fallback(&mut self, fallback: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.fallback = fallback.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.fallback = fallback;
            self.0.mask |= 0b10000000000000001;
        }
        self
    }

    #[inline]
    pub fn foreground(&mut self, foreground: &str) -> &mut Self {
        self.0.foreground = nvim::String::from(foreground).into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b100000000000000000000001;
        }
        self
    }

    #[cfg(feature = "neovim-nightly")]
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
    #[inline]
    pub fn force(&mut self, force: bool) -> &mut Self {
        self.0.force = force;
        self.0.mask |= 0b100000001;
        self
    }

    #[inline]
    pub fn global_link(&mut self, global_link: &str) -> &mut Self {
        self.0.global_link = nvim::String::from(global_link).into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b10000000000000000000000001;
        }
        self
    }

    #[inline]
    pub fn italic(&mut self, italic: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.italic = italic.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.italic = italic;
            self.0.mask |= 0b1000000001;
        }
        self
    }

    #[inline]
    pub fn link(&mut self, link: &str) -> &mut Self {
        self.0.link = nvim::String::from(link).into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b100001;
        }
        self
    }

    #[inline]
    pub fn nocombine(&mut self, nocombine: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.nocombine = nocombine.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.nocombine = nocombine;
            self.0.mask |= 0b1000000000000000001;
        }
        self
    }

    #[inline]
    pub fn reverse(&mut self, reverse: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.reverse = reverse.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.reverse = reverse;
            self.0.mask |= 0b1000000000000001;
        }
        self
    }

    #[inline]
    pub fn special(&mut self, special: &str) -> &mut Self {
        self.0.special = nvim::String::from(special).into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b10000000001;
        }
        self
    }

    #[inline]
    pub fn standout(&mut self, standout: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.standout = standout.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.standout = standout;
            self.0.mask |= 0b100000000000000001;
        }
        self
    }

    #[inline]
    pub fn strikethrough(&mut self, strikethrough: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.strikethrough = strikethrough.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.strikethrough = strikethrough;
            self.0.mask |= 0b100000000000000000000000000001;
        }
        self
    }

    #[inline]
    pub fn undercurl(&mut self, undercurl: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.undercurl = undercurl.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.undercurl = undercurl;
            self.0.mask |= 0b10000000000000000001;
        }
        self
    }

    #[inline]
    pub fn underdashed(&mut self, underdashed: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.underdashed = underdashed.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.underdashed = underdashed;
            self.0.mask |= 0b100000000000000000000000001;
        }
        self
    }

    #[inline]
    pub fn underdotted(&mut self, underdotted: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.underdotted = underdotted.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.underdotted = underdotted;
            self.0.mask |= 0b1000000000000000000000000001;
        }
        self
    }

    #[inline]
    pub fn underdouble(&mut self, underdouble: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.underdouble = underdouble.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.underdouble = underdouble;
            self.0.mask |= 0b10000000000000000000000000001;
        }
        self
    }

    #[inline]
    pub fn underline(&mut self, underline: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.underline = underline.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.underline = underline;
            self.0.mask |= 0b100000000000000000001;
        }
        self
    }

    #[cfg(not(feature = "neovim-0-8"))]
    #[inline]
    pub fn altfont(&mut self, altfont: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.altfont = altfont.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.altfont = altfont;
            self.0.mask |= 0b100000000000001;
        }
        self
    }

    #[cfg(not(feature = "neovim-0-8"))]
    #[inline]
    pub fn bg_indexed(&mut self, bg_indexed: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.bg_indexed = bg_indexed.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.bg_indexed = bg_indexed;
            self.0.mask |= 0b10000000000000000000001;
        }
        self
    }

    #[cfg(not(feature = "neovim-0-8"))]
    #[inline]
    pub fn fg_indexed(&mut self, fg_indexed: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.fg_indexed = fg_indexed.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.fg_indexed = fg_indexed;
            self.0.mask |= 0b1000000000000000000000001;
        }
        self
    }

    #[inline]
    pub fn build(&mut self) -> SetHighlightOpts {
        std::mem::take(&mut self.0)
    }
}
