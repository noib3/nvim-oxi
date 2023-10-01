use oxi_types::{
    conversion::{self, FromObject},
    serde::Deserializer,
    Array,
    Float,
    Integer,
    Object,
};
#[cfg(feature = "neovim-nightly")]
use oxi_types::{Boolean, String as NvimString, WinHandle};
use serde::Deserialize;

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
    mask: u64,

    /// 2nd in the mask.
    row: Float,

    /// 1st in the mask.
    col: Float,

    /// 8th in the mask.
    width: Integer,

    /// 11th in the mask.
    height: Integer,

    /// 9th in the mask.
    anchor: NvimString,

    /// 16th in the mask.
    relative: NvimString,

    /// 3rd in the mask.
    win: WinHandle,

    /// 10th in the mask.
    bufpos: Array,

    /// 15th in the mask.
    external: Boolean,

    /// 17th in the mask.
    focusable: Boolean,

    /// 12th in the mask.
    zindex: Integer,

    /// 14th in the mask.
    border: Object,

    /// 7th in the mask.
    title: Object,

    /// 19th in the mask.
    title_pos: NvimString,

    /// 13th in the mask.
    footer: Object,

    /// 20th in the mask.
    footer_pos: NvimString,

    /// 6th in the mask.
    style: NvimString,

    /// 18th in the mask.
    noautocmd: Boolean,

    /// 5th in the mask.
    fixed: Boolean,

    /// 4th in the mask.
    hide: Boolean,
}

impl From<&WindowConfig> for KeyDict_float_config {
    fn from(config: &WindowConfig) -> Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            let bufpos = match config.bufpos {
                Some((line, column)) => {
                    Array::from_iter([line as Integer, column as Integer])
                        .into()
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
                #[cfg(any(
                    feature = "neovim-0-9",
                    feature = "neovim-nightly"
                ))]
                title: config.title.as_ref().into(),
                #[cfg(any(
                    feature = "neovim-0-9",
                    feature = "neovim-nightly"
                ))]
                title_pos: config.title_pos.as_ref().into(),
                width: config.width.into(),
                win: config.win.as_ref().into(),
                zindex: config.zindex.into(),
            }
        }

        #[cfg(feature = "neovim-nightly")]
        {
            let mut mask = 0;

            let row = if let Some(row) = config.row {
                mask |= 0b101;
                row
            } else {
                Float::default()
            };

            let col = if let Some(col) = config.col {
                mask |= 0b11;
                col
            } else {
                Float::default()
            };

            let width = if let Some(width) = config.width {
                mask |= 0b100000001;
                width as Integer
            } else {
                Integer::default()
            };

            let height = if let Some(height) = config.height {
                mask |= 0b100000000001;
                height as Integer
            } else {
                Integer::default()
            };

            let anchor = if let Some(anchor) = config.anchor {
                mask |= 0b1000000001;
                NvimString::from(anchor)
            } else {
                NvimString::default()
            };

            let relative = if let Some(relative) = &config.relative {
                mask |= 0b10000000000000001;
                NvimString::from(relative.clone())
            } else {
                NvimString::default()
            };

            let win = if let Some(win) = &config.win {
                mask |= 0b1001;
                win.0
            } else {
                WinHandle::default()
            };

            let bufpos = if let Some((line, column)) = config.bufpos {
                mask |= 0b10000000001;
                Array::from_iter([line as Integer, column as Integer])
            } else {
                Array::default()
            };

            let external = if let Some(external) = config.external {
                mask |= 0b1000000000000001;
                external
            } else {
                Boolean::default()
            };

            let focusable = if let Some(focusable) = config.focusable {
                mask |= 0b100000000000000001;
                focusable
            } else {
                Boolean::default()
            };

            let zindex = if let Some(zindex) = config.zindex {
                mask |= 0b1000000000001;
                zindex as Integer
            } else {
                Integer::default()
            };

            let border = if let Some(border) = &config.border {
                mask |= 0b100000000000001;
                border.clone().into()
            } else {
                Object::default()
            };

            let title = if let Some(title) = config.title.as_ref() {
                mask |= 0b10000001;
                title.into()
            } else {
                Object::default()
            };

            let title_pos = if let Some(title_pos) = config.title_pos {
                mask |= 0b10000000000000000001;
                title_pos.into()
            } else {
                NvimString::default()
            };

            let footer = if let Some(footer) = config.footer.as_ref() {
                mask |= 0b10000000000001;
                footer.into()
            } else {
                Object::default()
            };

            let footer_pos = if let Some(footer_pos) = config.footer_pos {
                mask |= 0b100000000000000000001;
                footer_pos.into()
            } else {
                NvimString::default()
            };

            let style = if let Some(style) = config.style {
                mask |= 0b1000001;
                style.into()
            } else {
                NvimString::default()
            };

            let noautocmd = if let Some(noautocmd) = config.noautocmd {
                mask |= 0b1000000000000000001;
                noautocmd
            } else {
                Boolean::default()
            };

            let fixed = if let Some(fixed) = config.fixed {
                mask |= 0b100001;
                fixed
            } else {
                Boolean::default()
            };

            let hide = if let Some(hide) = config.hide {
                mask |= 0b10001;
                hide
            } else {
                Boolean::default()
            };

            Self {
                mask,
                row,
                col,
                width,
                height,
                anchor,
                relative,
                win,
                bufpos,
                external,
                focusable,
                zindex,
                border,
                title,
                title_pos,
                footer,
                footer_pos,
                style,
                noautocmd,
                fixed,
                hide,
            }
        }
    }
}
