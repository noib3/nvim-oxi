#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    NvimError(#[from] nvim_types::error::Error),

    #[error(transparent)]
    NulByteStringError(#[from] std::ffi::NulError),

    #[error(transparent)]
    IntError(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    FromObjectError(#[from] crate::object::FromObjectError),

    #[error(transparent)]
    ToObjectError(#[from] crate::object::ToObjectError),
}
