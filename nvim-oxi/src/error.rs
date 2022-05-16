use nvim_types::Error as NvimError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Exception(String),

    #[error("{0}")]
    Validation(String),
}

impl TryFrom<NvimError> for Error {
    type Error = &'static str;

    fn try_from(err: NvimError) -> Result<Self, Self::Error> {
        use nvim_types::ErrorType::*;
        match err.r#type {
            kErrorTypeNone => Err("not an error!"),
            kErrorTypeException => Ok(Self::Exception(err.to_string())),
            kErrorTypeValidation => Ok(Self::Validation(err.to_string())),
        }
    }
}
