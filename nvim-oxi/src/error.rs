use std::fmt;

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    NvimError(#[from] nvim_types::error::Error),

    #[error(transparent)]
    FromObjectError(#[from] nvim_types::object::FromObjectError),

    #[error(transparent)]
    BadUtf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    NulByteStringError(#[from] std::ffi::NulError),

    #[error(transparent)]
    IntError(#[from] std::num::TryFromIntError),

    #[error("{0}")]
    SerializeError(String),

    #[error("{0}")]
    DeserializeError(String),
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
