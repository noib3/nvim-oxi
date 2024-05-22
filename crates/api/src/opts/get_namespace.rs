use crate::Window;

/// Option passed to [`get_hl_ns()`][crate::get_hl_ns].
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct GetNamespaceOpts {
    #[builder(mask)]
    mask: u64,

    /// Used to retrieve a window's highlight namespace. A value of `-1` is
    /// returned by [`get_hl_ns()`][crate::get_hl_ns] when
    /// `nvim_win_set_hl_ns()` has not been called for the window (or was
    /// called with a namespace of `-1`).
    #[builder(argtype = "Window", inline = "{0}.0")]
    winid: types::WinHandle,
}
