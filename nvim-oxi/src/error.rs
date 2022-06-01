use std::fmt;
use std::string::String as StdString;

use serde::{de, ser};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    NvimError(#[from] nvim_types::error::Error),

    #[error(transparent)]
    FromObjectError(#[from] nvim_types::object::FromObjectError),

    #[error(transparent)]
    NulByteStringError(#[from] std::ffi::NulError),

    #[error(transparent)]
    IntError(#[from] std::num::TryFromIntError),

    #[error("{0}")]
    SerializeError(StdString),

    #[error("{0}")]
    DeserializeError(StdString),
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
