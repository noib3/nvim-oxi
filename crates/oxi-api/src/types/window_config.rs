use oxi_types::{
    conversion::{self, FromObject},
    serde::Deserializer,
    Array,
    Float,
    Integer,
    Object,
};
use serde::Deserialize;

use super::{WindowAnchor, WindowBorder, WindowRelativeTo, WindowStyle};

#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct WindowConfig {
    /// Decides which corner of the window to place at `(row, col)`.
    pub anchor: Option<WindowAnchor>,

    /// Style of the optional window border.
    pub border: Option<WindowBorder>,

    /// Places window relative to buffer text (only when
    /// [`relative`](WindowConfigBuilder::relative) is set to
    /// [`WindowRelativeTo::Window(win)`](WindowRelativeTo)). Takes a zero
    /// indexed `(line, column)` tuple, with `row` and `col` being placed
    /// relative to this position if specified.
    pub bufpos: Option<(usize, usize)>,

    /// Column position in units of screen cell width. May be fractional
    pub col: Option<Float>,

    /// Whether an attached GUI should display the window as an external
    /// top-level window.
    pub external: Option<bool>,

    /// Enable focus by user actions like mouse events. Non-focusable windows
    /// can be entered by [`set_current_win`](crate::set_current_win).
    pub focusable: Option<bool>,

    /// Window height in character cells. Minimum of 1.
    pub height: Option<u32>,

    /// If `true` then no buffer-related autocommand events such as `BufEnter`
    /// or `BufLeave` are fired when calling [`open_win`](crate::open_win).
    pub noautocmd: Option<bool>,

    /// What the window is positioned relative to.
    pub relative: Option<WindowRelativeTo>,

    /// Row position in units of screen cell height. May be fractional.
    pub row: Option<Float>,

    /// Configures the appearance of the window.
    pub style: Option<WindowStyle>,

    /// Window width in character cells. Minimum of 1.
    pub width: Option<u32>,

    /// Stacking order. Windows with higher `zindex` go in front of windows
    /// with lower indices.
    pub zindex: Option<u32>,

    /// TODO
    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "neovim-0-9", feature = "neovim-nightly")))
    )]
    pub title: Option<super::WindowTitle>,

    /// TODO
    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "neovim-0-9", feature = "neovim-nightly")))
    )]
    pub title_pos: Option<super::WindowTitlePosition>,
}

impl WindowConfig {
    #[inline(always)]
    /// Creates a new `WinConfigBuilder`.
    pub fn builder() -> WindowConfigBuilder {
        WindowConfigBuilder::default()
    }
}

#[derive(Clone, Default)]
pub struct WindowConfigBuilder(WindowConfig);

impl WindowConfigBuilder {
    /// Decides which corner of the window to place at `(row, col)`.
    #[inline]
    pub fn anchor(&mut self, anchor: WindowAnchor) -> &mut Self {
        self.0.anchor = Some(anchor);
        self
    }

    /// Style of the optional window border.
    #[inline]
    pub fn border(&mut self, border: WindowBorder) -> &mut Self {
        self.0.border = Some(border);
        self
    }

    /// Places window relative to buffer text (only when
    /// [`relative`](WindowConfigBuilder::relative) is set to
    /// [`WindowRelativeTo::Window(win)`](WindowRelativeTo)). Takes a zero
    /// indexed `(line, column)` tuple, with `row` and `col` being placed
    /// relative to this position if specified.
    #[inline]
    pub fn bufpos(&mut self, line: usize, column: usize) -> &mut Self {
        self.0.bufpos = Some((line, column));
        self
    }

    /// Column position in units of screen cell width. May be fractional
    #[inline]
    pub fn col(&mut self, col: impl Into<Float>) -> &mut Self {
        self.0.col = Some(col.into());
        self
    }

    /// Whether an attached GUI should display the window as an external
    /// top-level window.
    #[inline]
    pub fn external(&mut self, external: bool) -> &mut Self {
        self.0.external = Some(external);
        self
    }

    /// Enable focus by user actions like mouse events. Non-focusable windows
    /// can be entered by [`set_current_win`](crate::set_current_win).
    #[inline]
    pub fn focusable(&mut self, focusable: bool) -> &mut Self {
        self.0.focusable = Some(focusable);
        self
    }

    /// Window height in character cells. Minimum of 1.
    #[inline]
    pub fn height(&mut self, height: u32) -> &mut Self {
        self.0.height = Some(height);
        self
    }

    /// If `true` then no buffer-related autocommand events such as `BufEnter`
    /// or `BufLeave` are fired when calling [`open_win`](crate::open_win).
    #[inline]
    pub fn noautocmd(&mut self, noautocmd: bool) -> &mut Self {
        self.0.noautocmd = Some(noautocmd);
        self
    }

    /// What the window is positioned relative to.
    #[inline]
    pub fn relative(&mut self, relative: WindowRelativeTo) -> &mut Self {
        self.0.relative = Some(relative);
        self
    }

    /// Row position in units of screen cell height. May be fractional.
    #[inline]
    pub fn row(&mut self, row: impl Into<Float>) -> &mut Self {
        self.0.row = Some(row.into());
        self
    }

    /// Configures the appearance of the window.
    #[inline]
    pub fn style(&mut self, style: WindowStyle) -> &mut Self {
        self.0.style = Some(style);
        self
    }

    /// Window width in character cells. Minimum of 1.
    #[inline]
    pub fn width(&mut self, width: u32) -> &mut Self {
        self.0.width = Some(width);
        self
    }

    /// Stacking order. Windows with higher `zindex` go in front of windows
    /// with lower indices.
    #[inline]
    pub fn zindex(&mut self, zindex: u32) -> &mut Self {
        self.0.zindex = Some(zindex);
        self
    }

    /// TODO
    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "neovim-0-9", feature = "neovim-nightly")))
    )]
    #[inline]
    pub fn title(&mut self, title: super::WindowTitle) -> &mut Self {
        self.0.title = Some(title);
        self
    }

    /// TODO
    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "neovim-0-9", feature = "neovim-nightly")))
    )]
    #[inline]
    pub fn title_pos(
        &mut self,
        title_pos: super::WindowTitlePosition,
    ) -> &mut Self {
        self.0.title_pos = Some(title_pos);
        self
    }

    #[inline]
    pub fn build(&mut self) -> WindowConfig {
        std::mem::take(&mut self.0)
    }
}

impl FromObject for WindowConfig {
    #[inline]
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

#[cfg(not(feature = "neovim-nightly"))]
#[derive(Default, Debug)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_float_config {
    col: Object,
    row: Object,
    win: Object,
    style: Object,
    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    title: Object,
    width: Object,
    height: Object,
    zindex: Object,
    anchor: Object,
    border: Object,
    bufpos: Object,
    external: Object,
    relative: Object,
    focusable: Object,
    noautocmd: Object,
    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    title_pos: Object,
}

#[cfg(feature = "neovim-nightly")]
#[derive(Default, Debug)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_float_config {
    row: Object,
    col: Object,
    width: Object,
    height: Object,
    anchor: Object,
    relative: Object,
    win: Object,
    bufpos: Object,
    external: Object,
    focusable: Object,
    zindex: Object,
    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    border: Object,
    title: Object,
    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    title_pos: Object,
    style: Object,
    noautocmd: Object,
}

impl From<&WindowConfig> for KeyDict_float_config {
    fn from(config: &WindowConfig) -> Self {
        let win = match &config.relative {
            Some(WindowRelativeTo::Window(win)) => win.0.into(),
            _ => Object::nil(),
        };

        let bufpos = match config.bufpos {
            Some((line, column)) => {
                Array::from_iter([line as Integer, column as Integer]).into()
            },
            _ => Object::nil(),
        };

        Self {
            col: config.col.into(),
            row: config.row.into(),
            win,
            style: config.style.into(),
            #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
            title: config.title.as_ref().into(),
            width: config.width.into(),
            height: config.height.into(),
            zindex: config.zindex.into(),
            anchor: config.anchor.into(),
            border: config.border.clone().into(),
            bufpos,
            external: config.external.into(),
            relative: config.relative.as_ref().into(),
            focusable: config.focusable.into(),
            noautocmd: config.noautocmd.into(),
            #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
            title_pos: config.title_pos.as_ref().into(),
        }
    }
}
