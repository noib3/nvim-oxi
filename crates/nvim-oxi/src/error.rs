use thiserror::Error as ThisError;

/// `nvim-oxi`'s result type.
pub type Result<T> = std::result::Result<T, Error>;

/// `nvim-oxi`'s error type.
#[derive(Clone, Debug, ThisError)]
#[cfg_attr(not(feature = "mlua"), derive(Eq, PartialEq))]
pub enum Error {
    #[error(transparent)]
    Lua(#[from] oxi_luajit::Error),

    #[error(transparent)]
    Api(#[from] oxi_api::Error),

    #[error(transparent)]
    Nvim(#[from] oxi_types::Error),

    #[error(transparent)]
    ObjectConversion(#[from] oxi_types::conversion::Error),

    #[error(transparent)]
    Serde(#[from] oxi_types::serde::Error),

    #[cfg(feature = "libuv")]
    #[error(transparent)]
    Libuv(#[from] oxi_libuv::Error),

    #[cfg(feature = "mlua")]
    #[error(transparent)]
    Mlua(#[from] mlua::Error),
}
