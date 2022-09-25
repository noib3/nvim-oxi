use std::fmt;

use serde::{de, ser};
use thiserror::Error as ThisError;

pub(super) type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Eq, PartialEq, ThisError)]
pub enum Error {
    #[error("{0}")]
    Serialize(String),

    #[error("{0}")]
    Deserialize(String),

    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::Serialize(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::Deserialize(msg.to_string())
    }
}
