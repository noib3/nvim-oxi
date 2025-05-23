/// Option passed to [`Window::text_height()`][crate::Window::text_height].
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct WinTextHeightOpts {
    #[builder(mask)]
    mask: u64,

    /// Starting line index, 0-based inclusive. When omitted start at the very
    /// top.
    #[builder(argtype = "usize", inline = "{0} as types::Integer")]
    start_row: types::Integer,

    /// Ending line index, 0-based inclusive. When omitted end at the very
    /// bottom.
    #[builder(argtype = "usize", inline = "{0} as types::Integer")]
    end_row: types::Integer,

    /// Starting virtual column index on `start_row`, 0-based inclusive,
    /// rounded down to full screen lines. When omitted include the whole line.
    #[builder(argtype = "usize", inline = "{0} as types::Integer")]
    start_vcol: types::Integer,

    /// Ending virtual column index on `end_row`, 0-based exclusive, rounded up
    /// to full screen lines. When omitted include the whole line.
    #[builder(argtype = "usize", inline = "{0} as types::Integer")]
    end_vcol: types::Integer,

    // Don't add the height of lines below the row for which this height is
    // reached. Useful to e.g. limit the height to the window height, avoiding
    // unnecessary work. Or to find out how many buffer lines beyond
    // [`start_row`](Self::start_row) take up a certain number of logical lines
    // (returned in `end_row` and `end_vcol`).
    #[cfg(feature = "neovim-nightly")] // Only on Nightly.
    #[builder(argtype = "usize", inline = "{0} as types::Integer")]
    max_height: types::Integer,
}
