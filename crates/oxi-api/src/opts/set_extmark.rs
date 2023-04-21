use oxi_types::{self as nvim, Array, Integer, Object};

use crate::trait_utils::StringOrListOfStrings;
use crate::types::{ExtmarkHlMode, ExtmarkVirtTextPosition};

/// Options passed to [`Buffer::set_extmark()`](crate::Buffer::set_extmark).
#[cfg(not(feature = "neovim-nightly"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct SetExtmarkOpts {
    id: Object,
    spell: Object,
    hl_eol: Object,
    strict: Object,
    end_col: Object,
    conceal: Object,
    hl_mode: Object,
    end_row: Object,
    /// The docs don't mention this but it's there.
    end_line: Object,
    hl_group: Object,
    priority: Object,
    ephemeral: Object,
    sign_text: Object,
    virt_text: Object,
    ui_watched: Object,
    virt_lines: Object,
    line_hl_group: Object,
    right_gravity: Object,
    sign_hl_group: Object,
    virt_text_pos: Object,
    virt_text_hide: Object,
    number_hl_group: Object,
    virt_lines_above: Object,
    end_right_gravity: Object,
    virt_text_win_col: Object,
    virt_lines_leftcol: Object,
    cursorline_hl_group: Object,
}

/// Options passed to [`set_extmark()`](crate::set_extmark).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct SetExtmarkOpts {
    id: Object,
    /// The docs don't mention this but it's there.
    end_line: Object,
    end_row: Object,
    end_col: Object,
    hl_group: Object,
    virt_text: Object,
    virt_text_pos: Object,
    virt_text_win_col: Object,
    virt_text_hide: Object,
    hl_eol: Object,
    hl_mode: Object,
    ephemeral: Object,
    priority: Object,
    right_gravity: Object,
    end_right_gravity: Object,
    virt_lines: Object,
    virt_lines_above: Object,
    virt_lines_leftcol: Object,
    strict: Object,
    sign_text: Object,
    sign_hl_group: Object,
    number_hl_group: Object,
    line_hl_group: Object,
    cursorline_hl_group: Object,
    conceal: Object,
    spell: Object,
    ui_watched: Object,
}

#[derive(Clone, Default)]
pub struct SetExtmarkOptsBuilder(SetExtmarkOpts);

impl SetExtmarkOpts {
    #[inline(always)]
    pub fn builder() -> SetExtmarkOptsBuilder {
        SetExtmarkOptsBuilder::default()
    }
}

impl SetExtmarkOptsBuilder {
    /// Enable concealing symilar to `:syn-conceal`. If a character is supplied
    /// it is used as `:syn-cchar`.
    ///
    /// [`hl_group`](SetExtmarkOptsBuilder::hl_group) is used to highlight the
    /// character if provided, otherwise it defaults to `hl-Conceal`.
    #[inline]
    pub fn conceal(&mut self, conceal: Option<char>) -> &mut Self {
        let ch = conceal.map(nvim::String::from).unwrap_or_default();
        self.0.conceal = ch.into();
        self
    }

    /// Name of the highlight group used to highlight the line when the cursor
    /// is on the same line as the mark and `cursorline` is enabled.
    #[inline]
    pub fn cursorline_hl_group(
        &mut self,
        cursorline_hl_group: &str,
    ) -> &mut Self {
        self.0.cursorline_hl_group =
            nvim::String::from(cursorline_hl_group).into();
        self
    }

    /// Ending line of the mark. 0-indexed and exclusive.
    #[inline]
    pub fn end_col(&mut self, end_col: usize) -> &mut Self {
        self.0.end_col = (end_col as Integer).into();
        self
    }

    /// Indicates the direction the extmark's end position (if it exists) will
    /// be shifted in when new text is inserted (`true` for right, `false` for
    /// left). Defaults to left.
    #[inline]
    pub fn end_right_gravity(&mut self, end_right_gravity: bool) -> &mut Self {
        self.0.end_right_gravity = end_right_gravity.into();
        self
    }

    /// Ending line of the mark. 0-indexed and inclusive.
    #[inline]
    pub fn end_row(&mut self, end_row: usize) -> &mut Self {
        self.0.end_row = (end_row as Integer).into();
        self
    }

    /// For use with
    /// [`set_decoration_provider()`](crate::set_decoration_provider)
    /// callbacks. The mark will only be used for the current redraw cycle, and
    /// not be permanently stored in the buffer.
    #[inline]
    pub fn ephemeral(&mut self, ephemeral: bool) -> &mut Self {
        self.0.ephemeral = ephemeral.into();
        self
    }

    /// Whether to continue the highlight for the rest of the screen line for
    /// multiline highlights covering the EOL of a line.
    #[inline]
    pub fn hl_eol(&mut self, hl_eol: bool) -> &mut Self {
        self.0.hl_eol = hl_eol.into();
        self
    }

    /// Name of the highlight group used to highlight this mark.
    #[inline]
    pub fn hl_group(&mut self, hl_group: &str) -> &mut Self {
        self.0.hl_group = nvim::String::from(hl_group).into();
        self
    }

    /// Controls how highlights are combined with the highlights of the text.
    #[inline]
    pub fn hl_mode(&mut self, hl_mode: ExtmarkHlMode) -> &mut Self {
        self.0.hl_mode = nvim::String::from(hl_mode).into();
        self
    }

    /// Id of the extmark to edit.
    #[inline]
    pub fn id(&mut self, id: u32) -> &mut Self {
        self.0.id = id.into();
        self
    }

    /// Name of the highlight group used to highlight the whole line.
    #[inline]
    pub fn line_hl_group(&mut self, line_hl_group: &str) -> &mut Self {
        self.0.line_hl_group = nvim::String::from(line_hl_group).into();
        self
    }

    /// Name of the highlight group used to highlight the number column.
    #[inline]
    pub fn number_hl_group(&mut self, number_hl_group: &str) -> &mut Self {
        self.0.number_hl_group = nvim::String::from(number_hl_group).into();
        self
    }

    /// A priority value for the highlight group. For example, treesitter
    /// highlights use a value of 100.
    #[inline]
    pub fn priority(&mut self, priority: u32) -> &mut Self {
        self.0.priority = priority.into();
        self
    }

    /// Indicates the direction the extmark will be shifted in when new text is
    /// inserted (`true` for right, `false` for left). Defaults to right.
    #[inline]
    pub fn right_gravity(&mut self, right_gravity: u32) -> &mut Self {
        self.0.right_gravity = right_gravity.into();
        self
    }

    /// Name of the highlight group used to highlight the sign column text.
    #[inline]
    pub fn sign_hl_group(&mut self, sign_hl_group: &str) -> &mut Self {
        self.0.sign_hl_group = nvim::String::from(sign_hl_group).into();
        self
    }

    /// Text to display in the sign column. Should take up 1-2 display cells.
    #[inline]
    pub fn sign_text(&mut self, sign_text: &str) -> &mut Self {
        self.0.sign_text = nvim::String::from(sign_text).into();
        self
    }

    /// Whether the extmark should not be placed if the line or column value is
    /// past the end of the buffer or end of the line, respectively. Defaults
    /// to `true`.
    #[inline]
    pub fn strict(&mut self, strict: bool) -> &mut Self {
        self.0.strict = strict.into();
        self
    }

    /// Whether the mark should be drawn by an external UI. When `true` the UI
    /// will receive `win_extmark` events.
    #[inline]
    pub fn ui_watched(&mut self, ui_watched: bool) -> &mut Self {
        self.0.ui_watched = ui_watched.into();
        self
    }

    /// Virtual lines to add next to the mark.
    #[inline]
    pub fn virt_lines<Txt, Hl, Cnk, ChunkyCnk>(
        &mut self,
        virt_lines: ChunkyCnk,
    ) -> &mut Self
    where
        ChunkyCnk: IntoIterator<Item = Cnk>,
        Cnk: IntoIterator<Item = (Txt, Hl)>,
        Txt: Into<nvim::String>,
        Hl: StringOrListOfStrings,
    {
        self.0.virt_lines = virt_lines
            .into_iter()
            .map(|chnky| {
                Array::from_iter(chnky.into_iter().map(|(txt, hl)| {
                    Array::from_iter([txt.into().into(), hl.to_object()])
                }))
            })
            .collect::<Array>()
            .into();
        self
    }

    /// Whether to place virtual lines above the buffer line containing the
    /// mark.
    #[inline]
    pub fn virt_lines_above(&mut self, virt_lines_above: bool) -> &mut Self {
        self.0.virt_lines_above = virt_lines_above.into();
        self
    }

    /// Whether to place extmarks in the leftmost column of the ewindow,
    /// bypassing sign and number columns.
    #[inline]
    pub fn virt_lines_leftcol(
        &mut self,
        virt_lines_leftcol: bool,
    ) -> &mut Self {
        self.0.virt_lines_leftcol = virt_lines_leftcol.into();
        self
    }

    /// Virtual text to link to this mark. Every `(text, highlights)` tuple
    /// represents a text chunk with a specified highlight. The highlights
    /// specified in `highlights` will be combined together, with the highest
    /// priority highlight beign applied last. Each highlight group can either
    /// be a string or an integer, the latter obtained using
    /// [`get_hl_id_by_name()`](crate::get_hl_id_by_name).
    #[inline]
    pub fn virt_text<Txt, Hl, Cnk>(&mut self, virt_text: Cnk) -> &mut Self
    where
        Cnk: IntoIterator<Item = (Txt, Hl)>,
        Txt: Into<nvim::String>,
        Hl: StringOrListOfStrings,
    {
        self.0.virt_text = virt_text
            .into_iter()
            .map(|(txt, hl)| {
                Array::from_iter([txt.into().into(), hl.to_object()])
            })
            .collect::<Array>()
            .into();
        self
    }

    /// Whether to hide the virtual text when the background text is selected
    /// or hidden due to horizontal scroll.
    #[inline]
    pub fn virt_text_hide(&mut self, virt_text_hide: bool) -> &mut Self {
        self.0.virt_text_hide = virt_text_hide.into();
        self
    }

    /// Position of the virtual text.
    #[inline]
    pub fn virt_text_pos(
        &mut self,
        virt_text_pos: ExtmarkVirtTextPosition,
    ) -> &mut Self {
        self.0.virt_text_pos = nvim::String::from(virt_text_pos).into();
        self
    }

    /// Position the virtual text at a fixed window column (starting from the
    /// first text column).
    #[inline]
    pub fn virt_text_win_col(&mut self, virt_text_win_col: u32) -> &mut Self {
        self.0.virt_text_win_col = virt_text_win_col.into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> SetExtmarkOpts {
        std::mem::take(&mut self.0)
    }
}
