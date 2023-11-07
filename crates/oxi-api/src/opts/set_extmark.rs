use oxi_types::{self as nvim, Array, Integer, Object};
#[cfg(feature = "neovim-nightly")]
use oxi_types::{Boolean, String as NvimString};

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
    mask: u64,

    /// 1st in the mask.
    id: Integer,

    /// The docs don't mention this but it's there.
    /// 9th in the mask.
    end_line: Integer,

    /// 8th in the mask.
    end_row: Integer,

    /// 5th in the mask.
    end_col: Integer,

    /// 10th in the mask.
    hl_group: Object,

    /// 14th in the mask.
    virt_text: Array,

    /// 21st in the mask.
    virt_text_pos: NvimString,

    /// 26th in the mask.
    virt_text_win_col: Integer,

    /// 22nd in the mask.
    virt_text_hide: Boolean,

    /// 3rd in the mask.
    hl_eol: Boolean,

    /// 7th in the mask.
    hl_mode: NvimString,

    /// 12th in the mask.
    ephemeral: Boolean,

    /// 11th in the mask.
    priority: Integer,

    /// 19th in the mask.
    right_gravity: Boolean,

    /// 25th in the mask.
    end_right_gravity: Boolean,

    /// 16th in the mask.
    virt_lines: Array,

    /// 24th in the mask.
    virt_lines_above: Boolean,

    /// 27th in the mask.
    virt_lines_leftcol: Boolean,

    /// 4th in the mask.
    strict: Boolean,

    /// 13th in the mask.
    sign_text: NvimString,

    /// 20th in the mask.
    sign_hl_group: Object,

    /// 23rd in the mask.
    number_hl_group: Object,

    /// 18th in the mask.
    line_hl_group: Object,

    /// 28th in the mask.
    cursorline_hl_group: Object,

    /// 6th in the mask.
    conceal: NvimString,

    /// 2nd in the mask.
    spell: Boolean,

    /// 15th in the mask.
    ui_watched: Boolean,

    /// 17th in the mask.
    undo_restore: Boolean,
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

        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.conceal = ch.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.conceal = ch;
            self.0.mask |= 0b1000001;
        }

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
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b10000000000000000000000000001;
        }
        self
    }

    /// Ending line of the mark. 0-indexed and exclusive.
    #[inline]
    pub fn end_col(&mut self, end_col: usize) -> &mut Self {
        let end_col = end_col as Integer;

        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.end_col = end_col.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.end_col = end_col;
            self.0.mask |= 0b100001;
        }

        self
    }

    /// Indicates the direction the extmark's end position (if it exists) will
    /// be shifted in when new text is inserted (`true` for right, `false` for
    /// left). Defaults to left.
    #[inline]
    pub fn end_right_gravity(&mut self, end_right_gravity: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.end_right_gravity = end_right_gravity.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.end_right_gravity = end_right_gravity;
            self.0.mask |= 0b10000000000000000000000001;
        }
        self
    }

    /// Ending line of the mark. 0-indexed and inclusive.
    #[inline]
    pub fn end_row(&mut self, end_row: usize) -> &mut Self {
        let end_row = end_row as Integer;

        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.end_row = end_row.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.end_row = end_row;
            self.0.mask |= 0b100000001;
        }

        self
    }

    /// For use with
    /// [`set_decoration_provider()`](crate::set_decoration_provider)
    /// callbacks. The mark will only be used for the current redraw cycle, and
    /// not be permanently stored in the buffer.
    #[inline]
    pub fn ephemeral(&mut self, ephemeral: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.ephemeral = ephemeral.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.ephemeral = ephemeral;
            self.0.mask |= 0b1000000000001;
        }
        self
    }

    /// Whether to continue the highlight for the rest of the screen line for
    /// multiline highlights covering the EOL of a line.
    #[inline]
    pub fn hl_eol(&mut self, hl_eol: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.hl_eol = hl_eol.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.hl_eol = hl_eol;
            self.0.mask |= 0b1001;
        }
        self
    }

    /// Name of the highlight group used to highlight this mark.
    #[inline]
    pub fn hl_group(&mut self, hl_group: &str) -> &mut Self {
        self.0.hl_group = nvim::String::from(hl_group).into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b10000000001;
        }
        self
    }

    /// Controls how highlights are combined with the highlights of the text.
    #[inline]
    pub fn hl_mode(&mut self, hl_mode: ExtmarkHlMode) -> &mut Self {
        let hl_mode = nvim::String::from(hl_mode);

        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.hl_mode = hl_mode.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.hl_mode = hl_mode;
            self.0.mask |= 0b10000001;
        }

        self
    }

    /// Id of the extmark to edit.
    #[inline]
    pub fn id(&mut self, id: u32) -> &mut Self {
        let id = id as Integer;
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.id = id.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.id = id;
            self.0.mask |= 0b11;
        }
        self
    }

    /// Name of the highlight group used to highlight the whole line.
    #[inline]
    pub fn line_hl_group(&mut self, line_hl_group: &str) -> &mut Self {
        self.0.line_hl_group = nvim::String::from(line_hl_group).into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b1000000000000000001;
        }
        self
    }

    /// Name of the highlight group used to highlight the number column.
    #[inline]
    pub fn number_hl_group(&mut self, number_hl_group: &str) -> &mut Self {
        self.0.number_hl_group = nvim::String::from(number_hl_group).into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b100000000000000000000001;
        }
        self
    }

    /// A priority value for the highlight group. For example, treesitter
    /// highlights use a value of 100.
    #[inline]
    pub fn priority(&mut self, priority: u32) -> &mut Self {
        let priority = priority as Integer;
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.priority = priority.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.priority = priority;
            self.0.mask |= 0b100000000001;
        }
        self
    }

    /// Indicates the direction the extmark will be shifted in when new text is
    /// inserted (`true` for right, `false` for left). Defaults to right.
    #[inline]
    pub fn right_gravity(&mut self, right_gravity: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.right_gravity = right_gravity.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.right_gravity = right_gravity;
            self.0.mask |= 0b10000000000000000001;
        }
        self
    }

    /// Name of the highlight group used to highlight the sign column text.
    #[inline]
    pub fn sign_hl_group(&mut self, sign_hl_group: &str) -> &mut Self {
        self.0.sign_hl_group = nvim::String::from(sign_hl_group).into();
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.mask |= 0b100000000000000000001;
        }
        self
    }

    /// Text to display in the sign column. Should take up 1-2 display cells.
    #[inline]
    pub fn sign_text(&mut self, sign_text: &str) -> &mut Self {
        let sign_text = nvim::String::from(sign_text);

        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.sign_text = sign_text.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.sign_text = sign_text;
            self.0.mask |= 0b10000000000001;
        }

        self
    }

    /// Whether the extmark should not be placed if the line or column value is
    /// past the end of the buffer or end of the line, respectively. Defaults
    /// to `true`.
    #[inline]
    pub fn strict(&mut self, strict: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.strict = strict.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.strict = strict;
            self.0.mask |= 0b10001;
        }
        self
    }

    /// Whether the mark should be drawn by an external UI. When `true` the UI
    /// will receive `win_extmark` events.
    #[inline]
    pub fn ui_watched(&mut self, ui_watched: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.ui_watched = ui_watched.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.ui_watched = ui_watched;
            self.0.mask |= 0b1000000000000001;
        }
        self
    }

    /// Whether to restore the exact position of the mark if text around the
    /// mark was deleted and then restored by undo.
    ///
    /// Defaults to `true`.
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
    #[cfg(feature = "neovim-nightly")]
    #[inline]
    pub fn undo_restore(&mut self, undo_restore: bool) -> &mut Self {
        self.0.undo_restore = undo_restore;
        self.0.mask |= 0b100000000000000001;
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
        let virt_lines = virt_lines
            .into_iter()
            .map(|chnky| {
                Array::from_iter(chnky.into_iter().map(|(txt, hl)| {
                    Array::from_iter([txt.into().into(), hl.to_object()])
                }))
            })
            .collect::<Array>();

        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.virt_lines = virt_lines.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.virt_lines = virt_lines;
            self.0.mask |= 0b10000000000000001;
        }

        self
    }

    /// Whether to place virtual lines above the buffer line containing the
    /// mark.
    #[inline]
    pub fn virt_lines_above(&mut self, virt_lines_above: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.virt_lines_above = virt_lines_above.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.virt_lines_above = virt_lines_above;
            self.0.mask |= 0b1000000000000000000000001;
        }
        self
    }

    /// Whether to place extmarks in the leftmost column of the ewindow,
    /// bypassing sign and number columns.
    #[inline]
    pub fn virt_lines_leftcol(
        &mut self,
        virt_lines_leftcol: bool,
    ) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.virt_lines_leftcol = virt_lines_leftcol.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.virt_lines_leftcol = virt_lines_leftcol;
            self.0.mask |= 0b1000000000000000000000000001;
        }
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
        let virt_text = virt_text
            .into_iter()
            .map(|(txt, hl)| {
                Array::from_iter([txt.into().into(), hl.to_object()])
            })
            .collect::<Array>();

        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.virt_text = virt_text.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.virt_text = virt_text;
            self.0.mask |= 0b100000000000001;
        }

        self
    }

    /// Whether to hide the virtual text when the background text is selected
    /// or hidden due to horizontal scroll.
    #[inline]
    pub fn virt_text_hide(&mut self, virt_text_hide: bool) -> &mut Self {
        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.virt_text_hide = virt_text_hide.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.virt_text_hide = virt_text_hide;
            self.0.mask |= 0b10000000000000000000001;
        }
        self
    }

    /// Position of the virtual text.
    #[inline]
    pub fn virt_text_pos(
        &mut self,
        virt_text_pos: ExtmarkVirtTextPosition,
    ) -> &mut Self {
        let virt_text_pos = nvim::String::from(virt_text_pos);

        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.virt_text_pos = virt_text_pos.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.virt_text_pos = virt_text_pos;
            self.0.mask |= 0b1000000000000000000001;
        }

        self
    }

    /// Position the virtual text at a fixed window column (starting from the
    /// first text column).
    #[inline]
    pub fn virt_text_win_col(&mut self, virt_text_win_col: u32) -> &mut Self {
        let virt_text_win_col = virt_text_win_col as Integer;

        #[cfg(not(feature = "neovim-nightly"))]
        {
            self.0.virt_text_win_col = virt_text_win_col.into();
        }
        #[cfg(feature = "neovim-nightly")]
        {
            self.0.virt_text_win_col = virt_text_win_col;
            self.0.mask |= 0b100000000000000000000000001;
        }

        self
    }

    #[inline]
    pub fn build(&mut self) -> SetExtmarkOpts {
        std::mem::take(&mut self.0)
    }
}
