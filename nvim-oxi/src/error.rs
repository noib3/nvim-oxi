use nvim_types::error::{ConversionError, Error as NvimError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    NvimError(#[from] NvimError),

    #[error(transparent)]
    ConversionError(#[from] ConversionError),

    #[error(transparent)]
    NulByteStringError(#[from] std::ffi::NulError),

    #[error(transparent)]
    IntError(#[from] std::num::TryFromIntError),
}
