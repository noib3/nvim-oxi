use std::ops::{Deref, DerefMut};

use nvim_types::{self as nvim, Array, Integer, Object};

use crate::types::{ExtmarkHlMode, ExtmarkVirtTextPosition};

#[derive(Clone, Debug, Default)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_set_extmark {
    id: Object,
    #[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
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
    #[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
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

#[derive(Clone, Debug, Default)]
pub struct SetExtmarkOpts(pub(crate) KeyDict_set_extmark);

impl SetExtmarkOpts {
    #[inline(always)]
    pub fn builder() -> SetExtmarkOptsBuilder {
        SetExtmarkOptsBuilder::default()
    }

    #[inline(always)]
    pub fn set_conceal(&mut self, conceal: Option<char>) {
        let ch = conceal.map(nvim::String::from).unwrap_or_default();
        self.0.conceal = ch.into();
    }

    #[inline(always)]
    pub fn set_cursorline_hl_group(&mut self, cursorline_hl_group: &str) {
        self.0.cursorline_hl_group =
            nvim::String::from(cursorline_hl_group).into();
    }

    #[inline(always)]
    pub fn set_end_col(&mut self, end_col: usize) {
        self.0.end_col = (end_col as Integer).into();
    }

    #[inline(always)]
    pub fn set_end_right_gravity(&mut self, end_right_gravity: bool) {
        self.0.end_right_gravity = end_right_gravity.into();
    }

    #[inline(always)]
    pub fn set_end_row(&mut self, end_row: usize) {
        self.0.end_row = (end_row as Integer).into();
    }

    #[inline(always)]
    pub fn set_ephemeral(&mut self, ephemeral: bool) {
        self.0.ephemeral = ephemeral.into();
    }

    #[inline(always)]
    pub fn set_hl_eol(&mut self, hl_eol: bool) {
        self.0.hl_eol = hl_eol.into();
    }

    #[inline(always)]
    pub fn set_hl_group(&mut self, hl_group: &str) {
        self.0.hl_group = nvim::String::from(hl_group).into();
    }

    #[inline(always)]
    pub fn set_hl_mode(&mut self, hl_mode: ExtmarkHlMode) {
        self.0.hl_mode = nvim::String::from(hl_mode).into();
    }

    #[inline(always)]
    pub fn set_id(&mut self, id: u32) {
        self.0.id = id.into();
    }

    #[inline(always)]
    pub fn set_line_hl_group(&mut self, line_hl_group: &str) {
        self.0.line_hl_group = nvim::String::from(line_hl_group).into();
    }

    #[inline(always)]
    pub fn set_number_hl_group(&mut self, number_hl_group: &str) {
        self.0.number_hl_group = nvim::String::from(number_hl_group).into();
    }

    #[inline(always)]
    pub fn set_priority(&mut self, priority: u32) {
        self.0.priority = priority.into();
    }

    #[inline(always)]
    pub fn set_right_gravity(&mut self, right_gravity: u32) {
        self.0.right_gravity = right_gravity.into();
    }

    #[inline(always)]
    pub fn set_sign_hl_group(&mut self, sign_hl_group: &str) {
        self.0.sign_hl_group = nvim::String::from(sign_hl_group).into();
    }

    #[inline(always)]
    pub fn set_sign_text(&mut self, sign_text: &str) {
        self.0.sign_text = nvim::String::from(sign_text).into();
    }

    #[inline(always)]
    pub fn set_strict(&mut self, strict: bool) {
        self.0.strict = strict.into();
    }

    #[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "neovim-0-8", feature = "neovim-nightly")))
    )]
    #[inline(always)]
    pub fn set_ui_watched(&mut self, ui_watched: bool) {
        self.0.ui_watched = ui_watched.into();
    }

    #[inline(always)]
    pub fn set_virt_lines<Txt, Hl, Cnk, ChunkyCnk>(
        &mut self,
        virt_lines: ChunkyCnk,
    ) where
        ChunkyCnk: IntoIterator<Item = Cnk>,
        Cnk: IntoIterator<Item = (Txt, Hl)>,
        Txt: Into<nvim::String>,
        Hl: Into<Object>,
    {
        self.0.virt_lines = virt_lines
            .into_iter()
            .map(|chnky| {
                Array::from_iter(chnky.into_iter().map(|(txt, hl)| {
                    Array::from_iter([Object::from(txt.into()), hl.into()])
                }))
            })
            .collect::<Array>()
            .into();
    }

    #[inline(always)]
    pub fn set_virt_lines_above(&mut self, virt_lines_above: bool) {
        self.0.virt_lines_above = virt_lines_above.into();
    }

    #[inline(always)]
    pub fn set_virt_lines_leftcol(&mut self, virt_lines_leftcol: bool) {
        self.0.virt_lines_leftcol = virt_lines_leftcol.into();
    }

    #[inline(always)]
    pub fn set_virt_text<Txt, Hl, Hls, Cnk>(&mut self, virt_text: Cnk)
    where
        Cnk: IntoIterator<Item = (Txt, Hls)>,
        Txt: Into<nvim::String>,
        Hls: IntoIterator<Item = Hl>,
        Hl: Into<Object>,
    {
        self.0.virt_text = virt_text
            .into_iter()
            .map(|(txt, hls)| {
                let hls = Array::from_iter(hls);
                Array::from_iter([Object::from(txt.into()), hls.into()])
            })
            .collect::<Array>()
            .into();
    }

    #[inline(always)]
    pub fn set_virt_text_hide(&mut self, virt_text_hide: bool) {
        self.0.virt_text_hide = virt_text_hide.into();
    }

    pub fn set_virt_text_pos(
        &mut self,
        virt_text_pos: ExtmarkVirtTextPosition,
    ) {
        self.0.virt_text_pos = nvim::String::from(virt_text_pos).into();
    }

    #[inline(always)]
    pub fn set_virt_text_win_col(&mut self, virt_text_win_col: u32) {
        self.0.virt_text_win_col = virt_text_win_col.into();
    }
}

#[derive(Clone, Debug)]
pub struct SetExtmarkOptsBuilder(Option<SetExtmarkOpts>);

impl SetExtmarkOptsBuilder {
    /// Enable concealing symilar to `:syn-conceal`. If a character is supplied
    /// it is used as `:syn-cchar`.
    ///
    /// [`hl_group`](SetExtmarkOptsBuilder::hl_group) is used to highlight the
    /// character if provided, otherwise it defaults to `hl-Conceal`.
    #[inline(always)]
    pub fn conceal(&mut self, ch: Option<char>) -> &mut Self {
        self.set_conceal(ch);
        self
    }

    /// Name of the highlight group used to highlight the line when the cursor
    /// is on the same line as the mark and `cursorline` is enabled.
    #[inline(always)]
    pub fn cursorline_hl_group(
        &mut self,
        cursorline_hl_group: &str,
    ) -> &mut Self {
        self.set_cursorline_hl_group(cursorline_hl_group);
        self
    }

    /// Ending line of the mark. 0-indexed and exclusive.
    #[inline(always)]
    pub fn end_col(&mut self, end_col: usize) -> &mut Self {
        self.set_end_col(end_col);
        self
    }

    /// Indicates the direction the extmark's end position (if it exists) will
    /// be shifted in when new text is inserted (`true` for right, `false` for
    /// left). Defaults to left.
    #[inline(always)]
    pub fn end_right_gravity(&mut self, end_right_gravity: bool) -> &mut Self {
        self.set_end_right_gravity(end_right_gravity);
        self
    }

    /// Ending line of the mark. 0-indexed and inclusive.
    #[inline(always)]
    pub fn end_row(&mut self, end_row: usize) -> &mut Self {
        self.set_end_row(end_row);
        self
    }

    /// For use with
    /// [`api::set_decoration_provider`](crate::set_decoration_provider)
    /// callbacks. The mark will only be used for the current redraw cycle, and
    /// not be permanently stored in the buffer.
    #[inline(always)]
    pub fn ephemeral(&mut self, ephemeral: bool) -> &mut Self {
        self.set_ephemeral(ephemeral);
        self
    }

    /// Whether to continue the highlight for the rest of the screen line for
    /// multiline highlights covering the EOL of a line.
    #[inline(always)]
    pub fn hl_eol(&mut self, hl_eol: bool) -> &mut Self {
        self.set_hl_eol(hl_eol);
        self
    }

    /// Name of the highlight group used to highlight this mark.
    #[inline(always)]
    pub fn hl_group(&mut self, hl_group: &str) -> &mut Self {
        self.set_hl_group(hl_group);
        self
    }

    /// Controls how highlights are combined with the highlights of the text.
    #[inline(always)]
    pub fn hl_mode(&mut self, hl_mode: ExtmarkHlMode) -> &mut Self {
        self.set_hl_mode(hl_mode);
        self
    }

    /// Id of the extmark to edit.
    #[inline(always)]
    pub fn id(&mut self, id: u32) -> &mut Self {
        self.set_id(id);
        self
    }

    /// Name of the highlight group used to highlight the whole line.
    #[inline(always)]
    pub fn line_hl_group(&mut self, line_hl_group: &str) -> &mut Self {
        self.set_line_hl_group(line_hl_group);
        self
    }

    /// Name of the highlight group used to highlight the number column.
    #[inline(always)]
    pub fn number_hl_group(&mut self, number_hl_group: &str) -> &mut Self {
        self.set_number_hl_group(number_hl_group);
        self
    }

    /// A priority value for the highlight group. For example, treesitter
    /// highlights use a value of 100.
    #[inline(always)]
    pub fn priority(&mut self, priority: u32) -> &mut Self {
        self.set_priority(priority);
        self
    }

    /// Indicates the direction the extmark will be shifted in when new text is
    /// inserted (`true` for right, `false` for left). Defaults to right.
    #[inline(always)]
    pub fn right_gravity(&mut self, right_gravity: u32) -> &mut Self {
        self.set_right_gravity(right_gravity);
        self
    }

    /// Name of the highlight group used to highlight the sign column text.
    #[inline(always)]
    pub fn sign_hl_group(&mut self, sign_hl_group: &str) -> &mut Self {
        self.set_sign_hl_group(sign_hl_group);
        self
    }

    /// Text to display in the sign column. Should take up 1-2 display cells.
    #[inline(always)]
    pub fn sign_text(&mut self, sign_text: &str) -> &mut Self {
        self.set_sign_text(sign_text);
        self
    }

    /// Whether the extmark should not be placed if the line or column value is
    /// past the end of the buffer or end of the line, respectively. Defaults
    /// to `true`.
    #[inline(always)]
    pub fn strict(&mut self, strict: bool) -> &mut Self {
        self.set_strict(strict);
        self
    }

    /// Whether the mark should be drawn by an external UI. When `true` the UI
    /// will receive `win_extmark` events.
    #[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "neovim-0-8", feature = "neovim-nightly")))
    )]
    #[inline(always)]
    pub fn ui_watched(&mut self, ui_watched: bool) -> &mut Self {
        self.set_ui_watched(ui_watched);
        self
    }

    /// Virtual lines to add next to the mark.
    #[inline(always)]
    pub fn virt_lines<Txt, Hl, Cnk, ChunkyCnk>(
        &mut self,
        virt_lines: ChunkyCnk,
    ) -> &mut Self
    where
        ChunkyCnk: IntoIterator<Item = Cnk>,
        Cnk: IntoIterator<Item = (Txt, Hl)>,
        Txt: Into<nvim::String>,
        Hl: Into<Object>,
    {
        self.set_virt_lines(virt_lines);
        self
    }

    /// Whether to place virtual lines above the buffer line containing the
    /// mark.
    #[inline(always)]
    pub fn virt_lines_above(&mut self, virt_lines_above: bool) -> &mut Self {
        self.set_virt_lines_above(virt_lines_above);
        self
    }

    /// Whether to place extmarks in the leftmost column of the ewindow,
    /// bypassing sign and number columns.
    #[inline(always)]
    pub fn virt_lines_leftcol(
        &mut self,
        virt_lines_leftcol: bool,
    ) -> &mut Self {
        self.set_virt_lines_leftcol(virt_lines_leftcol);
        self
    }

    /// Virtual text to link to this mark. Every `(text, highlights)` tuple
    /// represents a text chunk with a specified highlight. The highlights
    /// specified in `highlights` will be combined together, with the highest
    /// priority highlight beign applied last. Each highlight group can either
    /// be a string or an integer, the latter obtained using
    /// [`api::get_hl_id_by_name`](crate::get_hl_id_by_name).
    #[inline(always)]
    pub fn virt_text<Txt, Hl, Hls, Cnk>(&mut self, virt_text: Cnk) -> &mut Self
    where
        Cnk: IntoIterator<Item = (Txt, Hls)>,
        Txt: Into<nvim::String>,
        Hls: IntoIterator<Item = Hl>,
        Hl: Into<Object>,
    {
        self.set_virt_text(virt_text);
        self
    }

    /// Whether to hide the virtual text when the background text is selected
    /// or hidden due to horizontal scroll.
    #[inline(always)]
    pub fn virt_text_hide(&mut self, virt_text_hide: bool) -> &mut Self {
        self.set_virt_text_hide(virt_text_hide);
        self
    }

    /// Position of the virtual text.
    #[inline(always)]
    pub fn virt_text_pos(
        &mut self,
        virt_text_pos: ExtmarkVirtTextPosition,
    ) -> &mut Self {
        self.set_virt_text_pos(virt_text_pos);
        self
    }

    /// Position the virtual text at a fixed window column (starting from the
    /// first text column).
    #[inline(always)]
    pub fn virt_text_win_col(&mut self, col: u32) -> &mut Self {
        self.set_virt_text_win_col(col);
        self
    }

    /// Builds the options.
    #[inline(always)]
    pub fn build(&mut self) -> SetExtmarkOpts {
        self.0.take().unwrap()
    }
}

impl Default for SetExtmarkOptsBuilder {
    #[inline]
    fn default() -> Self {
        Self(Some(SetExtmarkOpts::default()))
    }
}

impl Deref for SetExtmarkOptsBuilder {
    type Target = SetExtmarkOpts;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl DerefMut for SetExtmarkOptsBuilder {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        self.0.as_mut().unwrap()
    }
}
