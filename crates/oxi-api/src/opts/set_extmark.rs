use oxi_types as types;
use types::{Array, Integer};

use crate::trait_utils::StringOrListOfStrings;
use crate::types::{ExtmarkHlMode, ExtmarkVirtTextPosition};

/// Options passed to [`Buffer::set_extmark()`](crate::Buffer::set_extmark).
#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct SetExtmarkOpts {
    id: types::Object,
    spell: types::Object,
    hl_eol: types::Object,
    strict: types::Object,
    end_col: types::Object,
    conceal: types::Object,
    hl_mode: types::Object,
    end_row: types::Object,
    /// The docs don't mention this but it's there.
    end_line: types::Object,
    hl_group: types::Object,
    priority: types::Object,
    ephemeral: types::Object,
    sign_text: types::Object,
    virt_text: types::Object,
    ui_watched: types::Object,
    virt_lines: types::Object,
    line_hl_group: types::Object,
    right_gravity: types::Object,
    sign_hl_group: types::Object,
    virt_text_pos: types::Object,
    virt_text_hide: types::Object,
    number_hl_group: types::Object,
    virt_lines_above: types::Object,
    end_right_gravity: types::Object,
    virt_text_win_col: types::Object,
    virt_lines_leftcol: types::Object,
    cursorline_hl_group: types::Object,
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
#[derive(Clone, Default)]
pub struct SetExtmarkOptsBuilder(SetExtmarkOpts);

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl SetExtmarkOpts {
    #[inline(always)]
    pub fn builder() -> SetExtmarkOptsBuilder {
        SetExtmarkOptsBuilder::default()
    }
}

#[cfg(any(feature = "neovim-0-8", feature = "neovim-0-9"))]
impl SetExtmarkOptsBuilder {
    /// Enable concealing symilar to `:syn-conceal`. If a character is supplied
    /// it is used as `:syn-cchar`.
    ///
    /// [`hl_group`](SetExtmarkOptsBuilder::hl_group) is used to highlight the
    /// character if provided, otherwise it defaults to `hl-Conceal`.
    #[inline]
    pub fn conceal(&mut self, conceal: Option<char>) -> &mut Self {
        let ch = conceal.map(types::String::from).unwrap_or_default();
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
            types::String::from(cursorline_hl_group).into();
        self
    }

    /// Ending line of the mark. 0-indexed and exclusive.
    #[inline]
    pub fn end_col(&mut self, end_col: usize) -> &mut Self {
        let end_col = end_col as Integer;
        self.0.end_col = end_col.into();
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
        let end_row = end_row as Integer;
        self.0.end_row = end_row.into();
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
        self.0.hl_group = types::String::from(hl_group).into();
        self
    }

    /// Controls how highlights are combined with the highlights of the text.
    #[inline]
    pub fn hl_mode(&mut self, hl_mode: ExtmarkHlMode) -> &mut Self {
        let hl_mode = types::String::from(hl_mode);
        self.0.hl_mode = hl_mode.into();
        self
    }

    /// Id of the extmark to edit.
    #[inline]
    pub fn id(&mut self, id: u32) -> &mut Self {
        let id = id as Integer;
        self.0.id = id.into();
        self
    }

    /// Name of the highlight group used to highlight the whole line.
    #[inline]
    pub fn line_hl_group(&mut self, line_hl_group: &str) -> &mut Self {
        self.0.line_hl_group = types::String::from(line_hl_group).into();
        self
    }

    /// Name of the highlight group used to highlight the number column.
    #[inline]
    pub fn number_hl_group(&mut self, number_hl_group: &str) -> &mut Self {
        self.0.number_hl_group = types::String::from(number_hl_group).into();
        self
    }

    /// A priority value for the highlight group. For example, treesitter
    /// highlights use a value of 100.
    #[inline]
    pub fn priority(&mut self, priority: u32) -> &mut Self {
        let priority = priority as Integer;
        self.0.priority = priority.into();
        self
    }

    /// Indicates the direction the extmark will be shifted in when new text is
    /// inserted (`true` for right, `false` for left). Defaults to right.
    #[inline]
    pub fn right_gravity(&mut self, right_gravity: bool) -> &mut Self {
        self.0.right_gravity = right_gravity.into();
        self
    }

    /// Name of the highlight group used to highlight the sign column text.
    #[inline]
    pub fn sign_hl_group(&mut self, sign_hl_group: &str) -> &mut Self {
        self.0.sign_hl_group = types::String::from(sign_hl_group).into();
        self
    }

    /// Text to display in the sign column. Should take up 1-2 display cells.
    #[inline]
    pub fn sign_text(&mut self, sign_text: &str) -> &mut Self {
        let sign_text = types::String::from(sign_text);
        self.0.sign_text = sign_text.into();
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
        Txt: Into<types::String>,
        Hl: StringOrListOfStrings,
    {
        let mut virt = types::Array::default();
        set_virt_lines(&mut virt, virt_lines);
        self.0.virt_lines = virt.into();
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
        Txt: Into<types::String>,
        Hl: StringOrListOfStrings,
    {
        let mut array = types::Array::default();
        set_virt_text(&mut array, virt_text);
        self.0.virt_text = array.into();
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
        let virt_text_pos = types::String::from(virt_text_pos);
        self.0.virt_text_pos = virt_text_pos.into();
        self
    }

    /// Position the virtual text at a fixed window column (starting from the
    /// first text column).
    #[inline]
    pub fn virt_text_win_col(&mut self, virt_text_win_col: u32) -> &mut Self {
        let virt_text_win_col = virt_text_win_col as Integer;
        self.0.virt_text_win_col = virt_text_win_col.into();
        self
    }

    #[inline]
    pub fn build(&mut self) -> SetExtmarkOpts {
        std::mem::take(&mut self.0)
    }
}

/// Options passed to [`Buffer::set_extmark()`](crate::Buffer::set_extmark).
#[cfg(feature = "neovim-nightly")]
#[derive(Clone, Debug, Default, oxi_macros::OptsBuilder)]
#[repr(C)]
pub struct SetExtmarkOpts {
    #[builder(mask)]
    mask: u64,

    /// Id of the extmark to edit.
    #[builder(argtype = "u32", inline = "{0} as types::Integer")]
    id: types::Integer,

    #[builder(argtype = "u32", inline = "{0} as types::Integer")]
    end_line: types::Integer,

    /// Ending line of the mark. 0-indexed and inclusive.
    #[builder(argtype = "usize", inline = "{0} as types::Integer")]
    end_row: types::Integer,

    /// Ending line of the mark. 0-indexed and exclusive.
    #[builder(argtype = "usize", inline = "{0} as types::Integer")]
    end_col: types::Integer,

    /// Name of the highlight group used to highlight this mark.
    #[builder(
        generics = "Hl: crate::HlGroup",
        argtype = "Hl",
        inline = r#"{ let Ok(hl_id) = {0}.to_hl_id() else { return self; }; hl_id }"#
    )]
    hl_group: types::HlGroupId,

    /// Virtual text to link to this mark. Every `(text, highlights)` tuple
    /// represents a text chunk with a specified highlight. The highlights
    /// specified in `highlights` will be combined together, with the highest
    /// priority highlight beign applied last. Each highlight group can either
    /// be a string or an integer, the latter obtained using
    /// [`get_hl_id_by_name()`](crate::get_hl_id_by_name).
    #[builder(
        generics = r#"Txt: Into<types::String>, Hl: StringOrListOfStrings, Cnk: IntoIterator<Item = (Txt, Hl)>"#,
        argtype = "Cnk",
        setter = "set_virt_text"
    )]
    virt_text: types::Array,

    /// Position of the virtual text.
    #[builder(
        argtype = "ExtmarkVirtTextPosition",
        inline = "types::String::from({0})"
    )]
    virt_text_pos: types::String,

    /// Position the virtual text at a fixed window column (starting from the
    /// first text column).
    #[builder(argtype = "u32", inline = "{0} as types::Integer")]
    virt_text_win_col: Integer,

    /// Whether to hide the virtual text when the background text is selected
    /// or hidden due to horizontal scroll.
    #[builder(argtype = "bool")]
    virt_text_hide: types::Boolean,

    /// Whether to repeat the virtual text on wrapped lines.
    #[builder(argtype = "bool")]
    virt_text_repeat_linebreak: types::Boolean,

    /// Whether to continue the highlight for the rest of the screen line for
    /// multiline highlights covering the EOL of a line.
    #[builder(argtype = "bool")]
    hl_eol: types::Boolean,

    /// Controls how highlights are combined with the highlights of the text.
    #[builder(argtype = "ExtmarkHlMode", inline = "types::String::from({0})")]
    hl_mode: types::String,

    #[builder(argtype = "bool")]
    invalidate: types::Boolean,

    /// For use with
    /// [`set_decoration_provider()`](crate::set_decoration_provider)
    /// callbacks. The mark will only be used for the current redraw cycle, and
    /// not be permanently stored in the buffer.
    #[builder(argtype = "bool")]
    ephemeral: types::Boolean,

    /// A priority value for the highlight group. For example, treesitter
    /// highlights use a value of 100.
    #[builder(argtype = "u32", inline = "{0} as types::Integer")]
    priority: Integer,

    /// Indicates the direction the extmark will be shifted in when new text is
    /// inserted (`true` for right, `false` for left). Defaults to right.
    #[builder(argtype = "bool")]
    right_gravity: types::Boolean,

    /// Indicates the direction the extmark's end position (if it exists) will
    /// be shifted in when new text is inserted (`true` for right, `false` for
    /// left). Defaults to left.
    #[builder(argtype = "bool")]
    end_right_gravity: types::Boolean,

    /// Virtual lines to add next to the mark.
    #[builder(
        generics = r#"Txt: Into<types::String>, Hl: StringOrListOfStrings, Cnk: IntoIterator<Item = (Txt, Hl)>, ChunkyCnk: IntoIterator<Item = Cnk>"#,
        argtype = "ChunkyCnk",
        setter = "set_virt_lines"
    )]
    virt_lines: types::Array,

    /// Whether to place virtual lines above the buffer line containing the
    /// mark.
    #[builder(argtype = "bool")]
    virt_lines_above: types::Boolean,

    /// Whether to place extmarks in the leftmost column of the ewindow,
    /// bypassing sign and number columns.
    #[builder(argtype = "bool")]
    virt_lines_leftcol: types::Boolean,

    /// Whether the extmark should not be placed if the line or column value is
    /// past the end of the buffer or end of the line, respectively. Defaults
    /// to `true`.
    #[builder(argtype = "bool")]
    strict: types::Boolean,

    /// Text to display in the sign column. Should take up 1-2 display cells.
    #[builder(argtype = "&str", inline = "types::String::from({0})")]
    sign_text: types::String,

    /// Name of the highlight group used to highlight the sign column text.
    #[builder(
        generics = "Hl: crate::HlGroup",
        argtype = "Hl",
        inline = r#"{ let Ok(hl_id) = {0}.to_hl_id() else { return self; }; hl_id }"#
    )]
    sign_hl_group: types::HlGroupId,

    /// Name of the highlight group used to highlight the number column.
    #[builder(
        generics = "Hl: crate::HlGroup",
        argtype = "Hl",
        inline = r#"{ let Ok(hl_id) = {0}.to_hl_id() else { return self; }; hl_id }"#
    )]
    number_hl_group: types::HlGroupId,

    /// Name of the highlight group used to highlight the whole line.
    #[builder(
        generics = "Hl: crate::HlGroup",
        argtype = "Hl",
        inline = r#"{ let Ok(hl_id) = {0}.to_hl_id() else { return self; }; hl_id }"#
    )]
    line_hl_group: types::HlGroupId,

    /// Name of the highlight group used to highlight the line when the cursor
    /// is on the same line as the mark and `cursorline` is enabled.
    #[builder(
        generics = "Hl: crate::HlGroup",
        argtype = "Hl",
        inline = r#"{ let Ok(hl_id) = {0}.to_hl_id() else { return self; }; hl_id }"#
    )]
    cursorline_hl_group: types::HlGroupId,

    /// Enable concealing symilar to `:syn-conceal`. If a character is supplied
    /// it is used as `:syn-cchar`.
    ///
    /// [`hl_group`](SetExtmarkOptsBuilder::hl_group) is used to highlight the
    /// character if provided, otherwise it defaults to `hl-Conceal`.
    #[builder(
        argtype = "Option<char>",
        inline = "{0}.map(types::String::from).unwrap_or_default()"
    )]
    conceal: types::String,

    #[builder(argtype = "bool")]
    spell: types::Boolean,

    /// Whether the mark should be drawn by an external UI. When `true` the UI
    /// will receive `win_extmark` events.
    #[builder(argtype = "bool")]
    ui_watched: types::Boolean,

    /// Whether to restore the exact position of the mark if text around the
    /// mark was deleted and then restored by undo.
    ///
    /// Defaults to `true`.
    #[builder(argtype = "bool")]
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-nightly")))]
    undo_restore: types::Boolean,

    /// A URL to associate with this extmark.
    ///
    /// In the TUI, the OSC 8 control sequence is used to generate a clickable
    /// hyperlink to this URL.
    #[builder(argtype = "&str", inline = "types::String::from({0})")]
    url: types::String,
}

#[inline]
fn set_virt_lines<Txt, Hl, Cnk, ChunkyCnk>(
    field: &mut Array,
    virt_lines: ChunkyCnk,
) where
    ChunkyCnk: IntoIterator<Item = Cnk>,
    Cnk: IntoIterator<Item = (Txt, Hl)>,
    Txt: Into<types::String>,
    Hl: StringOrListOfStrings,
{
    *field = virt_lines
        .into_iter()
        .map(|chnky| {
            Array::from_iter(chnky.into_iter().map(|(txt, hl)| {
                Array::from_iter([txt.into().into(), hl.to_object()])
            }))
        })
        .collect::<Array>();
}

#[inline]
fn set_virt_text<Txt, Hl, Cnk>(field: &mut Array, virt_text: Cnk)
where
    Cnk: IntoIterator<Item = (Txt, Hl)>,
    Txt: Into<types::String>,
    Hl: StringOrListOfStrings,
{
    *field = virt_text
        .into_iter()
        .map(|(txt, hl)| Array::from_iter([txt.into().into(), hl.to_object()]))
        .collect::<Array>();
}
