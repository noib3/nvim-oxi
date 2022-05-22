use nvim_types::error::{ConversionError, Error as NvimError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    NvimError(#[from] NvimError),

    #[error(transparent)]
    ConversionError(#[from] ConversionError),
}

// impl TryFrom<NvimError> for Error {
//     type Error = &'static str;

//     fn try_from(err: NvimError) -> Result<Self, Self::Error> {
//         use nvim_types::ErrorType::*;
//         match err.r#type {
//             kErrorTypeNone => Err("not an error!"),
//             kErrorTypeException => Ok(Self::Exception(err.to_string())),
//             kErrorTypeValidation => Ok(Self::Validation(err.to_string())),
//         }
//     }
// }
