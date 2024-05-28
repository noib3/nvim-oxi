#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "nvim.exe", kind = "raw-dylib", modifiers = "+verbatim")
)]
extern "C" {
    // https://github.com/neovim/neovim/blob/v0.10.0/src/nvim/api/private/helpers.c#L776
    #[cfg(feature = "neovim-0-10")] // On 0.10 and nightly.
    pub(crate) fn object_to_hl_id(
        obj: types::Object,
        what: *const core::ffi::c_char,
        err: *mut types::Error,
    ) -> types::Integer;
}
