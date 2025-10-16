use types::{Array, Integer};

use crate::trait_utils::StringOrListOfStrings;
#[cfg(feature = "neovim-0-11")] // On 0.11 and Nightly.
use crate::types::VirtLinesOverflow;
use crate::types::{ExtmarkHlMode, ExtmarkVirtTextPosition};

/// Options passed to [`Buffer::set_extmark()`](crate::Buffer::set_extmark).
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
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
    #[cfg(not(feature = "neovim-0-11"))] // Only on 0.10.
    #[cfg_attr(docsrs, doc(cfg(not(feature = "neovim-0-11"))))]
    #[builder(
        generics = "Hl: crate::HlGroup",
        argtype = "Hl",
        inline = r#"{ let Ok(hl_id) = {0}.to_hl_id() else { return self; }; hl_id }"#
    )]
    hl_group: types::HlGroupId,

    /// Name of the highlight group used to highlight this mark.
    #[cfg(feature = "neovim-0-11")] // On 0.11 and Nightly.
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-0-11")))]
    #[builder(
        generics = "Hl: crate::SetExtmarkHlGroup",
        argtype = "Hl",
        inline = r#"{0}.into_object()"#
    )]
    hl_group: types::Object,

    /// Virtual text to link to this mark. Every `(text, highlights)` tuple
    /// represents a text chunk with a specified highlight. The highlights
    /// specified in `highlights` will be combined together, with the highest
    /// priority highlight beign applied last. Each highlight group can either
    /// be a string or an integer, the latter obtained using
    /// [`get_hl_id_by_name()`](crate::get_hl_id_by_name).
    #[builder(
        generics = r#"Text: Into<types::String>, Hl: StringOrListOfStrings, Chunks: IntoIterator<Item = (Text, Hl)>"#,
        argtype = "Chunks",
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
        generics = r#"Text: Into<types::String>, Hl: StringOrListOfStrings, Chunks: IntoIterator<Item = (Text, Hl)>, Lines: IntoIterator<Item = Chunks>"#,
        argtype = "Lines",
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

    /// Controls how to handle virtual lines wider than the window.
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-0-11")))]
    #[cfg(feature = "neovim-0-11")] // On 0.11 and Nightly.
    #[builder(argtype = "VirtLinesOverflow", inline = "{0}.into()")]
    virt_lines_overflow: types::String,

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

    /// When called, lines in the range are not drawn at all (according to
    /// `conceallevel`); the next unconcealed line is drawn instead.
    #[cfg_attr(docsrs, doc(cfg(feature = "neovim-0-11")))]
    #[cfg(feature = "neovim-0-11")] // On 0.11 and Nightly.
    #[builder(argtype = "()", inline = "{let _ = {0}; types::String::new()}")]
    conceal_lines: types::String,

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
    undo_restore: types::Boolean,

    /// A URL to associate with this extmark.
    ///
    /// In the TUI, the OSC 8 control sequence is used to generate a clickable
    /// hyperlink to this URL.
    #[builder(argtype = "&str", inline = "types::String::from({0})")]
    url: types::String,

    // This was an experimental option in Neovim 0.10 but has been removed from
    // the public API on nightly, even though it's still included in the opts.
    scoped: types::Boolean,

    #[cfg(feature = "neovim-nightly")] // Only on Nightly.
    #[builder(skip)]
    _subpriority: types::Integer,
}

#[inline]
fn set_virt_lines<Text, Hl, Chunks, Lines>(
    field: &mut Array,
    virt_lines: Lines,
) where
    Lines: IntoIterator<Item = Chunks>,
    Chunks: IntoIterator<Item = (Text, Hl)>,
    Text: Into<types::String>,
    Hl: StringOrListOfStrings,
{
    *field = virt_lines
        .into_iter()
        .map(|chunks| {
            Array::from_iter(chunks.into_iter().map(|(txt, hl)| {
                Array::from_iter([txt.into().into(), hl.to_object()])
            }))
        })
        .collect::<Array>();
}

#[inline]
fn set_virt_text<Text, Hl, Chunks>(field: &mut Array, virt_text: Chunks)
where
    Chunks: IntoIterator<Item = (Text, Hl)>,
    Text: Into<types::String>,
    Hl: StringOrListOfStrings,
{
    *field = virt_text
        .into_iter()
        .map(|(txt, hl)| Array::from_iter([txt.into().into(), hl.to_object()]))
        .collect::<Array>();
}
