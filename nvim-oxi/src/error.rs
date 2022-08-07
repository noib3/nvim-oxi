use std::fmt;

use serde::{de, ser};

/// Alias for a `Result` with error type [`nvim_oxi::Error`](Error).
pub type Result<T> = std::result::Result<T, Error>;

/// Error returned by `nvim-oxi` functions.
#[derive(thiserror::Error, Debug)]
#[cfg_attr(not(feature = "mlua"), derive(Eq, PartialEq))]
pub enum Error {
    #[error(transparent)]
    NvimError(#[from] nvim_types::Error),

    #[error(transparent)]
    FromObjectError(#[from] nvim_types::FromObjectError),

    #[error(transparent)]
    BadUtf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    IntError(#[from] std::num::TryFromIntError),

    #[error("{0}")]
    SerializeError(String),

    #[error("{0}")]
    DeserializeError(String),

    #[error("FnMut called recursively")]
    LuaFunMutRecursiveCallback,

    #[error("FnOnce called more than once")]
    LuaFunOnceMoreThanOnce,

    #[error("Lua runtime error: {0}")]
    LuaRuntimeError(String),

    #[error("Lua memory error: {0}")]
    LuaMemoryError(String),

    #[error("{0}")]
    Other(String),

    #[cfg(feature = "mlua")]
    #[error(transparent)]
    MluaError(#[from] mlua::Error),
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::SerializeError(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::DeserializeError(msg.to_string())
    }
}

impl Error {
    pub(crate) fn custom(msg: impl fmt::Display) -> Self {
        Self::Other(msg.to_string())
    }
}
