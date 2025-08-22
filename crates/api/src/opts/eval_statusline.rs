use crate::Window;

/// Options passed to [`eval_statusline()`](crate::eval_statusline).
//
// https://github.com/neovim/neovim/blob/v0.11.3/src/nvim/api/keysets_defs.h#L145-L154
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct EvalStatuslineOpts {
    #[builder(mask)]
    mask: u64,

    /// Window to use as context for the statusline.
    #[builder(method = "window", argtype = "Window", inline = "{0}.0")]
    winid: types::WinHandle,

    /// Maximum width for the statusline.
    #[builder(argtype = "u32", inline = "{0}.into()")]
    maxwidth: types::Integer,

    /// Character used to fill blank spaces in the statusline.
    #[builder(argtype = "char", inline = "{0}.into()")]
    fillchar: types::String,

    /// Whether to return statuline informations from
    /// [`eval_statusline()`](crate::eval_statusline).
    #[builder(argtype = "bool")]
    highlights: types::Boolean,

    /// Evaluate the winbar instead of the statusline. Mutually exclusive with
    /// [`use_tabline`](EvalStatuslineOptsBuilder::use_tabline).
    #[builder(argtype = "bool")]
    use_winbar: types::Boolean,

    /// Evaluate the tabline instead of the statusline. When `true` the
    /// [`window`](EvalStatuslineOptsBuilder::window) field is ignored.
    #[builder(argtype = "bool")]
    use_tabline: types::Boolean,

    // Evaluate the statuscolumn for this line number instead of the
    // statusline.
    #[builder(argtype = "u32", inline = "{0}.into()")]
    use_statuscol_lnum: types::Integer,
}
