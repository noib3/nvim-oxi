use thiserror::Error as ThisError;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Eq, PartialEq, ThisError)]
pub enum Error {
    #[error("Couldn't create async handle")]
    CouldntCreateAsyncHandle,

    #[error("Couldn't trigger async handle")]
    CouldntTriggerAsyncHandle,
}
