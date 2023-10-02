use thiserror::Error as ThisError;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Eq, PartialEq, ThisError)]
pub enum Error {
    #[error("Couldn't trigger async handle")]
    AsyncTrigger,

    #[error("Couldn't initialize handle")]
    HandleInit,

    #[error("Couldn't allocate memory for a new handle")]
    HandleMemAlloc,

    #[error("Couldn't start timer handle")]
    TimerStart,

    #[error("Couldn't stop timer handle")]
    TimerStop,
}
