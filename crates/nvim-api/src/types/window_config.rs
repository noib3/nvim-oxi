use derive_builder::Builder;
use nvim_types::{
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
#[derive(Clone, Debug, Default, PartialEq, Builder, Deserialize)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct WindowConfig {
    /// Decides which corner of the window to place at `(row, col)`.
    #[builder(setter(strip_option))]
    pub anchor: Option<WindowAnchor>,

    /// Style of the optional window border.
    #[builder(setter(strip_option))]
    pub border: Option<WindowBorder>,

    /// Places window relative to buffer text (only when
    /// [`relative`](WindowConfigBuilder::relative) is set to
    /// [`WindowRelativeTo::Window(win)`](WindowRelativeTo)). Takes a zero
    /// indexed `(line, column)` tuple, with `row` and `col` being placed
    /// relative to this position if specified.
    #[builder(setter(custom))]
    pub bufpos: Option<(usize, usize)>,

    /// Column position in units of screen cell width. May be fractional
    #[builder(setter(into, strip_option))]
    pub col: Option<Float>,

    /// Whether an attached GUI should display the window as an external
    /// top-level window.
    #[builder(setter(strip_option))]
    pub external: Option<bool>,

    /// Enable focus by user actions like mouse events. Non-focusable windows
    /// can be entered by [`set_current_win`](crate::set_current_win).
    #[builder(setter(strip_option))]
    pub focusable: Option<bool>,

    /// Window height in character cells. Minimum of 1.
    #[builder(setter(strip_option))]
    pub height: Option<u32>,

    /// If `true` then no buffer-related autocommand events such as `BufEnter`
    /// or `BufLeave` are fired when calling [`open_win`](crate::open_win).
    #[builder(setter(strip_option))]
    pub noautocmd: Option<bool>,

    /// What the window is positioned relative to.
    #[builder(setter(strip_option))]
    pub relative: Option<WindowRelativeTo>,

    /// Row position in units of screen cell height. May be fractional.
    #[builder(setter(into, strip_option))]
    pub row: Option<Float>,

    /// Configures the appearance of the window.
    #[builder(setter(strip_option))]
    pub style: Option<WindowStyle>,

    /// Window width in character cells. Minimum of 1.
    #[builder(setter(strip_option))]
    pub width: Option<u32>,

    /// Stacking order. Windows with higher `zindex` go in front of windows
    /// with lower indices.
    #[builder(setter(strip_option))]
    pub zindex: Option<u32>,

    /// TODO
    #[cfg(not(feature = "neovim-0-8"))]
    #[builder(setter(strip_option))]
    pub title: Option<super::WindowTitle>,

    /// TODO
    #[cfg(not(feature = "neovim-0-8"))]
    #[builder(setter(strip_option))]
    pub title_pos: Option<super::WindowTitlePosition>,
}

impl WindowConfig {
    #[inline(always)]
    /// Creates a new `WinConfigBuilder`.
    pub fn builder() -> WindowConfigBuilder {
        WindowConfigBuilder::default()
    }
}

impl WindowConfigBuilder {
    /// Places window relative to buffer text (only when
    /// [`relative`](WindowConfigBuilder::relative) is set to
    /// [`WindowRelativeTo::Window(win)`](WindowRelativeTo)). Takes a zero
    /// indexed `(line, column)` tuple, with `row` and `col` being placed
    /// relative to this position if specified.
    pub fn bufpos(&mut self, line: usize, column: usize) -> &mut Self {
        self.bufpos = Some(Some((line, column)));
        self
    }

    pub fn build(&mut self) -> WindowConfig {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl FromObject for WindowConfig {
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
    #[cfg(not(feature = "neovim-0-8"))]
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
    #[cfg(not(feature = "neovim-0-8"))]
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
    #[cfg(not(feature = "neovim-0-8"))]
    border: Object,
    title: Object,
    #[cfg(not(feature = "neovim-0-8"))]
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
            #[cfg(not(feature = "neovim-0-8"))]
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
            #[cfg(not(feature = "neovim-0-8"))]
            title_pos: config.title_pos.as_ref().into(),
        }
    }
}
