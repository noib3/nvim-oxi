use serde::Deserialize;
use types::{
    conversion::{self, FromObject},
    serde::Deserializer,
    Array,
    Float,
    Integer,
    Object,
};
use types::{Boolean, String as NvimString, WinHandle};

use super::{WindowAnchor, WindowBorder, WindowRelativeTo, WindowStyle};
use crate::serde_utils as utils;
use crate::Window;

#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct WindowConfig {
    /// Decides which corner of the window to place at `(row, col)`.
    #[serde(default, deserialize_with = "utils::empty_string_is_none")]
    pub anchor: Option<WindowAnchor>,

    /// Style of the optional window border.
    pub border: Option<WindowBorder>,

    /// Places window relative to buffer text (only when
    /// [`relative`](WindowConfigBuilder::relative) is set to
    /// [`WindowRelativeTo::Window(win)`](WindowRelativeTo)). Takes a zero
    /// indexed `(line, column)` tuple, with `row` and `col` being placed
    /// relative to this position if specified.
    #[serde(default, deserialize_with = "utils::empty_array_is_none")]
    pub bufpos: Option<(usize, usize)>,

    /// Column position in units of screen cell width. May be fractional
    pub col: Option<Float>,

    /// Whether an attached GUI should display the window as an external
    /// top-level window.
    pub external: Option<bool>,

    pub fixed: Option<bool>,

    /// Enable focus by user actions like mouse events. Non-focusable windows
    /// can be entered by [`set_current_win`](crate::set_current_win).
    pub focusable: Option<bool>,

    pub footer: Option<super::WindowTitle>,

    pub footer_pos: Option<super::WindowTitlePosition>,

    /// Window height in character cells. Minimum of 1.
    pub height: Option<u32>,

    pub hide: Option<bool>,

    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
    #[cfg(feature = "neovim-nightly")] // Only on Nightly.
    #[serde(default)]
    pub mouse: bool,

    /// If `true` then no buffer-related autocommand events such as `BufEnter`
    /// or `BufLeave` are fired when calling [`open_win`](crate::open_win).
    pub noautocmd: Option<bool>,

    /// What the window is positioned relative to.
    pub relative: Option<WindowRelativeTo>,

    /// Row position in units of screen cell height. May be fractional.
    pub row: Option<Float>,

    #[serde(default, deserialize_with = "utils::empty_string_is_none")]
    pub split: Option<super::SplitDirection>,

    /// Configures the appearance of the window.
    #[serde(default, deserialize_with = "utils::empty_string_is_none")]
    pub style: Option<WindowStyle>,

    pub title: Option<super::WindowTitle>,

    #[serde(default, deserialize_with = "utils::empty_string_is_none")]
    pub title_pos: Option<super::WindowTitlePosition>,

    pub vertical: Option<bool>,

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

    #[inline]
    pub fn footer(&mut self, footer: super::WindowTitle) -> &mut Self {
        self.0.footer = Some(footer);
        self
    }

    #[inline]
    pub fn footer_pos(
        &mut self,
        footer_pos: super::WindowTitlePosition,
    ) -> &mut Self {
        self.0.footer_pos = Some(footer_pos);
        self
    }

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

    /// Sets the window layout to `floating`
    /// Decides what the window is positioned relatively to.
    #[inline]
    pub fn relative(&mut self, relative: WindowRelativeTo) -> &mut Self {
        if let WindowRelativeTo::Window(win) = &relative {
            self.0.win = Some(win.clone());
        }
        self.0.relative = Some(relative);
        self
    }

    /// Configures where a split window is opened.
    #[inline]
    pub fn split(&mut self, direction: super::SplitDirection) -> &mut Self {
        self.0.split = Some(direction);
        self
    }

    /// Should the split window be opened as vertical.
    #[inline]
    pub fn vertical(&mut self, vertical: bool) -> &mut Self {
        self.0.vertical = Some(vertical);
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

    #[inline]
    pub fn title(&mut self, title: super::WindowTitle) -> &mut Self {
        self.0.title = Some(title);
        self
    }

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
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
    #[cfg(feature = "neovim-nightly")]
    mouse: Boolean,
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

        if let Some(split) = config.split {
            builder.split(split.into());
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

        #[cfg(feature = "neovim-nightly")]
        builder.mouse(config.mouse);

        if let Some(vertical) = config.vertical {
            builder.vertical(vertical);
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

impl TryFrom<WindowOpts> for WindowConfig {
    type Error = conversion::Error;

    #[inline]
    fn try_from(opts: WindowOpts) -> Result<Self, Self::Error> {
        let WindowOpts {
            anchor,
            border,
            bufpos,
            col,
            external,
            fixed,
            focusable,
            footer,
            footer_pos,
            height,
            hide,
            #[cfg(feature = "neovim-nightly")]
            mouse,
            noautocmd,
            relative,
            row,
            split,
            style,
            title,
            title_pos,
            vertical,
            width,
            win,
            zindex,
            ..
        } = opts;

        #[inline]
        fn deserialize<T>(
            obj: impl Into<Object>,
        ) -> Result<T, conversion::Error>
        where
            T: serde::de::DeserializeOwned,
        {
            T::deserialize(Deserializer::new(obj.into())).map_err(Into::into)
        }

        #[derive(serde::Deserialize)]
        #[serde(rename_all = "lowercase")]
        enum WindowRelative {
            Editor,
            Win,
            Cursor,
            Mouse,
        }

        let relative = match utils::empty_string_is_none(Deserializer::new(
            relative.into(),
        ))? {
            Some(relative) => match relative {
                WindowRelative::Editor => Some(WindowRelativeTo::Editor),
                WindowRelative::Win => {
                    let win = deserialize(win)?;
                    Some(WindowRelativeTo::Window(win))
                },
                WindowRelative::Cursor => Some(WindowRelativeTo::Cursor),
                WindowRelative::Mouse => Some(WindowRelativeTo::Mouse),
            },
            None => None,
        };

        let win = if let Some(WindowRelativeTo::Window(win)) = &relative {
            Some(win.clone())
        } else {
            None
        };

        Ok(Self {
            anchor: utils::empty_string_is_none(Deserializer::new(
                anchor.into(),
            ))?,
            border: deserialize(border)?,
            bufpos: utils::empty_array_is_none(Deserializer::new(
                bufpos.into(),
            ))?,
            col: deserialize(col)?,
            external: deserialize(external)?,
            fixed: deserialize(fixed)?,
            focusable: deserialize(focusable)?,
            footer: deserialize(footer)?,
            footer_pos: utils::empty_string_is_none(Deserializer::new(
                footer_pos.into(),
            ))?,
            height: deserialize(height)?,
            hide: deserialize(hide)?,
            #[cfg(feature = "neovim-nightly")]
            mouse,
            noautocmd: deserialize(noautocmd)?,
            relative,
            row: deserialize(row)?,
            split: utils::empty_string_is_none(Deserializer::new(
                split.into(),
            ))?,
            style: utils::empty_string_is_none(Deserializer::new(
                style.into(),
            ))?,
            title: deserialize(title)?,
            title_pos: utils::empty_string_is_none(Deserializer::new(
                title_pos.into(),
            ))?,
            vertical: deserialize(vertical)?,
            width: deserialize(width)?,
            win,
            zindex: deserialize(zindex)?,
        })
    }
}
