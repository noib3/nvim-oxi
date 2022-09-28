use thiserror::Error as ThisError;

/// Alias for a `Result` with error type [`nvim_oxi::Error`](Error).
pub type Result<T> = std::result::Result<T, Error>;

/// Error returned by `nvim-oxi` functions.
#[derive(Clone, Debug, ThisError)]
#[cfg_attr(not(feature = "mlua"), derive(Eq, PartialEq))]
pub enum Error {
    #[error(transparent)]
    NvimError(#[from] nvim_types::Error),

    #[error(transparent)]
    FromObjectError(#[from] nvim_types::FromObjectError),

    #[error(transparent)]
    ToObjectError(#[from] nvim_types::ToObjectError),

    #[error(transparent)]
    ApiError(#[from] nvim_api::Error),

    #[cfg(feature = "libuv")]
    #[error(transparent)]
    LibuvError(#[from] libuv_bindings::Error),

    #[cfg(feature = "mlua")]
    #[error(transparent)]
    MluaError(#[from] mlua::Error),
}
