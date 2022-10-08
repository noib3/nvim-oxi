use std::fmt::Display;

use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, ThisError, Eq, PartialEq)]
pub enum Error {
    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    Nvim(#[from] nvim_types::Error),

    #[error(transparent)]
    ObjectConversion(#[from] nvim_types::conversion::Error),

    #[error("{0}")]
    Other(String),
}

impl Error {
    pub(crate) fn custom<M: Display>(msg: M) -> Self {
        Self::Other(msg.to_string())
    }
}
