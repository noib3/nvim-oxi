use std::error::Error as StdError;
use std::fmt::Display;
use std::ptr;
use std::sync::Arc;

use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    Nvim(#[from] nvim_types::Error),

    #[error(transparent)]
    ObjectConversion(#[from] nvim_types::conversion::Error),

    // Using an `Arc` to be able to implement `Clone` w/o putting a `Clone`
    // boundary on the type.
    #[error("{0}")]
    External(Arc<dyn StdError + Send + Sync>),

    #[error("{0}")]
    Other(String),
}

impl Error {
    pub(crate) fn custom<M: Display>(msg: M) -> Self {
        Self::Other(msg.to_string())
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        use Error::*;
        match (self, other) {
            (FromInt(a), FromInt(b)) => a.eq(b),
            (FromUtf8(a), FromUtf8(b)) => a.eq(b),
            (Nvim(a), Nvim(b)) => a.eq(b),
            (ObjectConversion(a), ObjectConversion(b)) => a.eq(b),
            (Other(a), Other(b)) => a.eq(b),
            (External(a), External(b)) => ptr::eq(&**a, &**b),
            _ => false,
        }
    }
}

impl Eq for Error {}
