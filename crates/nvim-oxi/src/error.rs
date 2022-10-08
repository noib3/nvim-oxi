use thiserror::Error as ThisError;

/// `nvim-oxi`'s result type.
pub type Result<T> = std::result::Result<T, Error>;

/// `nvim-oxi`'s error type.
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

    #[error(transparent)]
    Serde(#[from] nvim_types::serde::Error),

    #[cfg(feature = "libuv")]
    #[error(transparent)]
    Libuv(#[from] libuv_bindings::Error),

    #[cfg(feature = "mlua")]
    #[error(transparent)]
    Mlua(#[from] mlua::Error),
}
