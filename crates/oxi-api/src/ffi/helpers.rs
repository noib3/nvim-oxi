#[cfg(feature = "neovim-nightly")]
use oxi_types as types;

extern "C" {
    // https://github.com/neovim/neovim/blob/4f788f78f8b2d59a76b1a54a40af0c478eb3f929/src/nvim/api/private/helpers.c#L841
    #[cfg(feature = "neovim-nightly")]
    pub(crate) fn object_to_hl_id(
        obj: types::Object,
        what: *const core::ffi::c_char,
        err: *mut types::Error,
    ) -> types::Integer;
}
