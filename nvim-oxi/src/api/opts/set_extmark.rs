use derive_builder::Builder;
use nvim_types::{self as nvim, Array, Integer, NonOwning, Object};

use crate::api::types::{ExtmarkHlMode, ExtmarkVirtTextPosition};

/// Options passed to [`Buffer::set_extmark`](crate::api::Buffer::set_extmark).
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct SetExtmarkOpts {
    #[builder(setter(custom))]
    conceal: Object,

    #[builder(setter(custom))]
    cursorline_hl_group: Object,

    /// Ending line of the mark. 0-indexed and exclusive.
    #[builder(setter(strip_option))]
    end_col: Option<usize>,

    /// Indicates the direction the extmark's end position (if it exists) will
    /// be shifted in when new text is inserted (`true` for right, `false` for
    /// left). Defaults to left.
    #[builder(setter(strip_option))]
    end_right_gravity: Option<bool>,

    /// Ending line of the mark. 0-indexed and inclusive.
    #[builder(setter(strip_option))]
    end_row: Option<usize>,

    /// For use with
    /// [`api::set_decoration_provider`](crate::api::set_decoration_provider)
    /// callbacks. The mark will only be used for the current redraw cycle, and
    /// not be permanently stored in the buffer.
    #[builder(setter(strip_option))]
    ephemeral: Option<bool>,

    /// Whether to continue the highlight for the rest of the screen line for
    /// multiline highlights covering the EOL of a line.
    #[builder(setter(strip_option))]
    hl_eol: Option<bool>,

    #[builder(setter(custom))]
    hl_group: Object,

    #[builder(setter(custom))]
    hl_mode: Object,

    /// Id of the extmark to edit.
    #[builder(setter(strip_option))]
    id: Option<u32>,

    #[builder(setter(custom))]
    line_hl_group: Object,

    #[builder(setter(custom))]
    number_hl_group: Object,

    /// A priority value for the highlight group. For example, treesitter
    /// highlights use a value of 100.
    #[builder(setter(strip_option))]
    priority: Option<u32>,

    /// Indicates the direction the extmark will be shifted in when new text is
    /// inserted (`true` for right, `false` for left). Defaults to right.
    #[builder(setter(strip_option))]
    right_gravity: Option<bool>,

    #[builder(setter(custom))]
    sign_hl_group: Object,

    #[builder(setter(custom))]
    sign_text: Object,

    /// Whether the extmark should not be placed if the line or column value is
    /// past the end of the buffer or end of the line, respectively. Defaults
    /// to `true`.
    #[builder(setter(strip_option))]
    strict: Option<bool>,

    /// Whether the mark should be drawn by an external UI. When `true` the UI
    /// will receive `win_extmark` events.
    #[cfg(feature = "nightly")]
    #[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
    #[builder(setter(strip_option))]
    ui_watched: Option<bool>,

    #[builder(setter(custom))]
    virt_lines: Object,

    /// Whether to place virtual lines above the buffer line containing the
    /// mark.
    #[builder(setter(strip_option))]
    virt_lines_above: Option<bool>,

    /// Whether to place extmarks in the leftmost column of the ewindow,
    /// bypassing sign and number columns.
    #[builder(setter(strip_option))]
    virt_lines_leftcol: Option<bool>,

    #[builder(setter(custom))]
    virt_text: Object,

    /// Whether to hide the virtual text when the background text is selected
    /// or hidden due to horizontal scroll.
    #[builder(setter(strip_option))]
    virt_text_hide: Option<bool>,

    #[builder(setter(custom))]
    virt_text_pos: Object,

    /// Position the virtual text at a fixed window column (starting from the
    /// first text column).
    #[builder(setter(into, strip_option))]
    virt_text_win_col: Option<u32>,
}

impl SetExtmarkOpts {
    #[inline(always)]
    /// Creates a new `SetExtmarkOptsBuilder`.
    pub fn builder() -> SetExtmarkOptsBuilder {
        SetExtmarkOptsBuilder::default()
    }
}

impl SetExtmarkOptsBuilder {
    /// Enable concealing symilar to `:syn-conceal`. If a character is supplied
    /// it is used as `:syn-cchar`.
    /// [`hl_group`](SetExtmarkOptsBuilder::hl_group) is used to highlight the
    /// character if provided, otherwise it defaults to `hl-Conceal`.
    pub fn conceal(&mut self, char: Option<char>) -> &mut Self {
        self.conceal =
            Some(char.map(nvim::String::from).unwrap_or_default().into());
        self
    }

    /// Name of the highlight group used to highlight the line when the cursor
    /// is on the same line as the mark and `cursorline` is enabled.
    pub fn cursorline_hl_group(&mut self, hl_group: &str) -> &mut Self {
        self.cursorline_hl_group = Some(nvim::String::from(hl_group).into());
        self
    }

    /// Name of the highlight group used to highlight this mark.
    pub fn hl_group(&mut self, hl_group: &str) -> &mut Self {
        self.hl_group = Some(nvim::String::from(hl_group).into());
        self
    }

    /// Controls how highlights are combined with the highlights of the text.
    pub fn hl_mode(&mut self, hl_mode: ExtmarkHlMode) -> &mut Self {
        self.hl_mode = Some(nvim::String::from(hl_mode).into());
        self
    }

    /// Name of the highlight group used to highlight the whole line.
    pub fn line_hl_group(&mut self, hl_group: &str) -> &mut Self {
        self.line_hl_group = Some(nvim::String::from(hl_group).into());
        self
    }

    /// Name of the highlight group used to highlight the number column.
    pub fn number_hl_group(&mut self, hl_group: &str) -> &mut Self {
        self.number_hl_group = Some(nvim::String::from(hl_group).into());
        self
    }

    /// Name of the highlight group used to highlight the sign column text.
    pub fn sign_hl_group(&mut self, hl_group: &str) -> &mut Self {
        self.sign_hl_group = Some(nvim::String::from(hl_group).into());
        self
    }

    /// Text to display in the sign column. Should take up 1-2 display cells.
    pub fn sign_text(&mut self, text: &str) -> &mut Self {
        self.sign_text = Some(nvim::String::from(text).into());
        self
    }

    /// Virtual lines to add next to the mark.
    pub fn virt_lines<Txt, Hl, Cnk>(&mut self, chunks: Cnk) -> &mut Self
    where
        Cnk: IntoIterator<Item = (Txt, Hl)>,
        Txt: Into<nvim::String>,
        Hl: Into<Object>,
    {
        self.virt_lines = Some(
            chunks
                .into_iter()
                .map(|(txt, hl)| {
                    Array::from_iter([Object::from(txt.into()), hl.into()])
                })
                .collect::<Array>()
                .into(),
        );
        self
    }

    /// Virtual text to link to this mark. Every `(text, highlights)` tuple
    /// represents a text chunk with a specified highlight. The highlights
    /// specified in `highlights` will be combined together, with the highest
    /// priority highlight beign applied last. Each highlight group can either
    /// be a string or an integer, the latter obtained using
    /// [`api::get_hl_id_by_name`](crate::api::get_hl_id_by_name).
    pub fn virt_text<Txt, Hl, Hls, Cnk>(&mut self, chunks: Cnk) -> &mut Self
    where
        Cnk: IntoIterator<Item = (Txt, Hls)>,
        Txt: Into<nvim::String>,
        Hls: IntoIterator<Item = Hl>,
        Hl: Into<Object>,
    {
        self.virt_text = Some(
            chunks
                .into_iter()
                .map(|(txt, hls)| {
                    let hls = Array::from_iter(hls);
                    Array::from_iter([Object::from(txt.into()), hls.into()])
                })
                .collect::<Array>()
                .into(),
        );
        self
    }

    /// Position of the virtual text.
    pub fn virt_text_pos(
        &mut self,
        pos: ExtmarkVirtTextPosition,
    ) -> &mut Self {
        self.virt_text_pos = Some(nvim::String::from(pos).into());
        self
    }

    /// Builds the options.
    pub fn build(&mut self) -> SetExtmarkOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_set_extmark<'a> {
    id: Object,
    hl_eol: Object,
    strict: Object,
    end_col: Object,
    conceal: NonOwning<'a, Object>,
    hl_mode: NonOwning<'a, Object>,
    end_row: Object,
    end_line: Object,
    hl_group: NonOwning<'a, Object>,
    priority: Object,
    ephemeral: Object,
    sign_text: NonOwning<'a, Object>,
    virt_text: NonOwning<'a, Object>,
    #[cfg(feature = "nightly")]
    ui_watched: Object,
    virt_lines: NonOwning<'a, Object>,
    line_hl_group: NonOwning<'a, Object>,
    right_gravity: Object,
    sign_hl_group: NonOwning<'a, Object>,
    virt_text_pos: NonOwning<'a, Object>,
    virt_text_hide: Object,
    number_hl_group: NonOwning<'a, Object>,
    virt_lines_above: Object,
    end_right_gravity: Object,
    virt_text_win_col: Object,
    virt_lines_leftcol: Object,
    cursorline_hl_group: NonOwning<'a, Object>,
}

impl<'a> From<&'a SetExtmarkOpts> for KeyDict_set_extmark<'a> {
    fn from(opts: &'a SetExtmarkOpts) -> Self {
        Self {
            id: opts.id.into(),
            hl_eol: opts.hl_eol.into(),
            strict: opts.strict.into(),
            end_col: opts.end_col.map(|n| n as Integer).into(),
            conceal: opts.conceal.non_owning(),
            hl_mode: opts.hl_mode.non_owning(),
            end_row: opts.end_row.map(|n| n as Integer).into(),
            end_line: Object::nil(),
            hl_group: opts.hl_group.non_owning(),
            priority: opts.priority.into(),
            ephemeral: opts.ephemeral.into(),
            sign_text: opts.sign_text.non_owning(),
            virt_text: opts.virt_text.non_owning(),
            #[cfg(feature = "nightly")]
            ui_watched: opts.ui_watched.into(),
            virt_lines: opts.virt_lines.non_owning(),
            line_hl_group: opts.line_hl_group.non_owning(),
            right_gravity: opts.right_gravity.into(),
            sign_hl_group: opts.sign_hl_group.non_owning(),
            virt_text_pos: opts.virt_text_pos.non_owning(),
            virt_text_hide: opts.virt_text_hide.into(),
            number_hl_group: opts.number_hl_group.non_owning(),
            virt_lines_above: opts.virt_lines_above.into(),
            end_right_gravity: opts.end_right_gravity.into(),
            virt_text_win_col: opts.virt_text_win_col.into(),
            virt_lines_leftcol: opts.virt_lines_leftcol.into(),
            cursorline_hl_group: opts.cursorline_hl_group.non_owning(),
        }
    }
}
