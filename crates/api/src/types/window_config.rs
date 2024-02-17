use serde::Deserialize;
use types::{
    conversion::{self, FromObject},
    serde::Deserializer,
    Array,
    Float,
    Integer,
    Object,
};
#[cfg(feature = "neovim-nightly")]
use types::{Boolean, String as NvimString, WinHandle};

use super::{WindowAnchor, WindowBorder, WindowRelativeTo, WindowStyle};
use crate::Window;

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

    #[cfg(feature = "neovim-nightly")]
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
    pub fixed: Option<bool>,

    /// Enable focus by user actions like mouse events. Non-focusable windows
    /// can be entered by [`set_current_win`](crate::set_current_win).
    pub focusable: Option<bool>,

    #[cfg(feature = "neovim-nightly")]
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
    pub footer: Option<super::WindowTitle>,

    #[cfg(feature = "neovim-nightly")]
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
    pub footer_pos: Option<super::WindowTitlePosition>,

    /// Window height in character cells. Minimum of 1.
    pub height: Option<u32>,

    #[cfg(feature = "neovim-nightly")]
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
    pub hide: Option<bool>,

    /// If `true` then no buffer-related autocommand events such as `BufEnter`
    /// or `BufLeave` are fired when calling [`open_win`](crate::open_win).
    pub noautocmd: Option<bool>,

    /// What the window is positioned relative to.
    pub relative: Option<WindowRelativeTo>,

    /// Row position in units of screen cell height. May be fractional.
    pub row: Option<Float>,

    /// Configures the appearance of the window.
    pub style: Option<WindowStyle>,

    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "neovim-0-9", feature = "neovim-nightly")))
    )]
    pub title: Option<super::WindowTitle>,

    #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "neovim-0-9", feature = "neovim-nightly")))
    )]
    pub title_pos: Option<super::WindowTitlePosition>,

    /// Window width in character cells. Minimum of 1.
    pub width: Option<u32>,

    /// For relative positioned windows, the [`Window`] that the window is
    /// positioned relative to.
    pub win: Option<Window>,

    /// Stacking order. Windows with higher `zindex` go in front of windows
    /// with lower indices.
    pub zindex: Option<u32>,
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

    #[cfg(feature = "neovim-nightly")]
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
    #[inline]
    pub fn fixed(&mut self, fixed: bool) -> &mut Self {
        self.0.fixed = Some(fixed);
        self
    }

    /// Enable focus by user actions like mouse events. Non-focusable windows
    /// can be entered by [`set_current_win`](crate::set_current_win).
    #[inline]
    pub fn focusable(&mut self, focusable: bool) -> &mut Self {
        self.0.focusable = Some(focusable);
        self
    }

    #[cfg(feature = "neovim-nightly")]
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
    #[inline]
    pub fn footer(&mut self, footer: super::WindowTitle) -> &mut Self {
        self.0.footer = Some(footer);
        self
    }

    #[cfg(feature = "neovim-nightly")]
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
    #[inline]
    pub fn footer_pos(
        &mut self,
        footer_pos: super::WindowTitlePosition,
    ) -> &mut Self {
        self.0.footer_pos = Some(footer_pos);
        self
    }

    #[cfg(feature = "neovim-nightly")]
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
    #[inline]
    pub fn hide(&mut self, hide: bool) -> &mut Self {
        self.0.hide = Some(hide);
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
        if let WindowRelativeTo::Window(win) = &relative {
            self.0.win = Some(win.clone());
        }
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
pub struct WindowOpts {
    col: Object,
    row: Object,
    win: Object,
    style: Object,
    #[cfg(feature = "neovim-0-9")]
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
    #[cfg(feature = "neovim-0-9")]
    title_pos: Object,
}

#[cfg(not(feature = "neovim-nightly"))]
impl From<&WindowConfig> for WindowOpts {
    fn from(config: &WindowConfig) -> Self {
        let bufpos = match config.bufpos {
            Some((line, column)) => {
                Array::from_iter([line as Integer, column as Integer]).into()
            },
            _ => Object::nil(),
        };

        Self {
            anchor: config.anchor.into(),
            border: config.border.clone().into(),
            bufpos,
            col: config.col.into(),
            external: config.external.into(),
            focusable: config.focusable.into(),
            height: config.height.into(),
            noautocmd: config.noautocmd.into(),
            relative: config.relative.as_ref().into(),
            row: config.row.into(),
            style: config.style.into(),
            #[cfg(feature = "neovim-0-9")]
            title: config.title.as_ref().into(),
            #[cfg(feature = "neovim-0-9")]
            title_pos: config.title_pos.as_ref().into(),
            width: config.width.into(),
            win: config.win.as_ref().into(),
            zindex: config.zindex.into(),
        }
    }
}

#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Default, Debug, macros::OptsBuilder)]
#[repr(C)]
pub struct WindowOpts {
    #[builder(mask)]
    mask: u64,
    row: Float,
    col: Float,
    width: Integer,
    height: Integer,
    anchor: NvimString,
    relative: NvimString,
    split: NvimString,
    win: WinHandle,
    bufpos: Array,
    external: Boolean,
    focusable: Boolean,
    vertical: Boolean,
    zindex: Integer,
    border: Object,
    title: Object,
    title_pos: NvimString,
    footer: Object,
    footer_pos: NvimString,
    style: NvimString,
    noautocmd: Boolean,
    fixed: Boolean,
    hide: Boolean,
}

#[cfg(feature = "neovim-nightly")]
impl From<&WindowConfig> for WindowOpts {
    fn from(config: &WindowConfig) -> Self {
        let mut builder = WindowOptsBuilder::default();

        if let Some(row) = config.row {
            builder.row(row);
        }

        if let Some(col) = config.col {
            builder.col(col);
        }

        if let Some(width) = config.width {
            builder.width(width as Integer);
        }

        if let Some(height) = config.height {
            builder.height(height as Integer);
        }

        if let Some(anchor) = config.anchor {
            builder.anchor(anchor.into());
        }

        if let Some(relative) = &config.relative {
            builder.relative(relative.clone().into());
        }

        if let Some(win) = &config.win {
            builder.win(win.0);
        }

        if let Some((line, column)) = config.bufpos {
            builder.bufpos(Array::from_iter([
                line as Integer,
                column as Integer,
            ]));
        }

        if let Some(external) = config.external {
            builder.external(external);
        }

        if let Some(focusable) = config.focusable {
            builder.focusable(focusable);
        }

        if let Some(zindex) = config.zindex {
            builder.zindex(zindex as Integer);
        }

        if let Some(border) = &config.border {
            builder.border(border.clone().into());
        }

        if let Some(title) = config.title.as_ref() {
            builder.title(title.into());
        }

        if let Some(title_pos) = config.title_pos {
            builder.title_pos(title_pos.into());
        }

        if let Some(footer) = config.footer.as_ref() {
            builder.footer(footer.into());
        }

        if let Some(footer_pos) = config.footer_pos {
            builder.footer_pos(footer_pos.into());
        }

        if let Some(style) = config.style {
            builder.style(style.into());
        }

        if let Some(noautocmd) = config.noautocmd {
            builder.noautocmd(noautocmd);
        }

        if let Some(fixed) = config.fixed {
            builder.fixed(fixed);
        }

        if let Some(hide) = config.hide {
            builder.hide(hide);
        }

        builder.build()
    }
}
