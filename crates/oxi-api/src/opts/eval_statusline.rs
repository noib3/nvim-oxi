use oxi_types::Object;

use crate::Window;

/// Options passed to [`eval_statusline()`](crate::eval_statusline).
#[cfg(feature = "neovim-0-8")]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct EvalStatuslineOpts {
    winid: Object,
    fillchar: Object,
    maxwidth: Object,
    highlights: Object,
    use_winbar: Object,
    use_tabline: Object,
}

/// Options passed to [`eval_statusline()`](crate::eval_statusline).
#[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct EvalStatuslineOpts {
    winid: Object,
    maxwidth: Object,
    fillchar: Object,
    highlights: Object,
    use_tabline: Object,
    use_winbar: Object,
    use_statuscol_lnum: Object,
}

impl EvalStatuslineOpts {
    #[inline(always)]
    /// Creates a new [`EvalStatuslineOptsBuilder`].
    pub fn builder() -> EvalStatuslineOptsBuilder {
        EvalStatuslineOptsBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct EvalStatuslineOptsBuilder(EvalStatuslineOpts);

impl EvalStatuslineOptsBuilder {
    /// Character used to fill blank spaces in the statusline.
    #[inline]
    pub fn fillchar(&mut self, fillchar: char) -> &mut Self {
        self.0.fillchar = fillchar.into();
        self
    }

    /// Return statuline informations from
    /// [`eval_statusline()`](crate::eval_statusline).
    #[inline]
    pub fn highlights(&mut self, highlights: bool) -> &mut Self {
        self.0.highlights = highlights.into();
        self
    }

    /// Maximum width for the statusline.
    #[inline]
    pub fn maxwidth(&mut self, maxwidth: u32) -> &mut Self {
        self.0.maxwidth = maxwidth.into();
        self
    }

    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "neovim-0-9", feature = "neovim-nightly")))
    )]
    #[inline]
    pub fn use_statuscol_lnum(
        &mut self,
        use_statuscol_lnum: bool,
    ) -> &mut Self {
        self.0.use_statuscol_lnum = use_statuscol_lnum.into();
        self
    }

    /// Evaluate the tabline instead of the statusline. When `true` the
    /// [`window`](EvalStatuslineOptsBuilder::window) field is ignored.
    #[inline]
    pub fn use_tabline(&mut self, use_tabline: bool) -> &mut Self {
        self.0.use_tabline = use_tabline.into();
        self
    }

    /// Evaluate the winbar instead of the statusline. Mutually exclusive with
    /// [`use_tabline`](EvalStatuslineOptsBuilder::use_tabline).
    #[inline]
    pub fn use_winbar(&mut self, use_winbar: bool) -> &mut Self {
        self.0.use_winbar = use_winbar.into();
        self
    }

    /// Window to use as context for the statusline.
    #[inline]
    pub fn window(&mut self, window: Window) -> &mut Self {
        self.0.winid = window.into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> EvalStatuslineOpts {
        std::mem::take(&mut self.0)
    }
}
