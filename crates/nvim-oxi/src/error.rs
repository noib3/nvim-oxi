use thiserror::Error as ThisError;

/// Alias for a `Result` with error type [`nvim_oxi::Error`](Error).
pub type Result<T> = std::result::Result<T, Error>;

/// Error returned by `nvim-oxi` functions.
#[derive(Clone, Debug, ThisError)]
#[cfg_attr(not(feature = "mlua"), derive(Eq, PartialEq))]
pub enum Error {
    #[error(transparent)]
    Lua(#[from] luajit_bindings::Error),

    #[error(transparent)]
    Api(#[from] nvim_api::Error),

    #[error(transparent)]
    Nvim(#[from] nvim_types::Error),

    #[error(transparent)]
    ObjectConversion(#[from] nvim_types::conversion::Error),

    #[cfg(feature = "libuv")]
    #[error(transparent)]
    Libuv(#[from] libuv_bindings::Error),

    #[cfg(feature = "mlua")]
    #[error(transparent)]
    Mlua(#[from] mlua::Error),
}
