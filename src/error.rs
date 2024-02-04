use thiserror::Error as ThisError;

/// `nvim-oxi`'s result type.
pub type Result<T> = std::result::Result<T, Error>;

/// `nvim-oxi`'s error type.
#[derive(Clone, Debug, ThisError)]
#[cfg_attr(not(feature = "mlua"), derive(Eq, PartialEq))]
pub enum Error {
    #[error(transparent)]
    Lua(#[from] luajit::Error),

    #[error(transparent)]
    Api(#[from] api::Error),

    #[error(transparent)]
    Nvim(#[from] types::Error),

    #[error(transparent)]
    ObjectConversion(#[from] types::conversion::Error),

    #[error(transparent)]
    Serde(#[from] types::serde::Error),

    #[cfg(feature = "libuv")]
    #[error(transparent)]
    Libuv(#[from] libuv::Error),

    #[cfg(feature = "mlua")]
    #[error(transparent)]
    Mlua(#[from] mlua::Error),
}
